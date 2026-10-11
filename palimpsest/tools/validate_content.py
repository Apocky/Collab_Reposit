#!/usr/bin/env python3
"""PALIMPSEST content validator (doc 19 §4; doc 02 §11.1; canon §1.1, §1.6, §19).

Usage:
    python3 tools/validate_content.py [--root <workspace>] [--json]
                                      [--allow-missing-prefix PREFIX ...]
                                      [--namespaces FILE] [--schemas DIR ...]
                                      [--self-test]

Validates every file under <root>/content against the schemas in
<root>/schemas, in the ten stages of doc 19 §4.1, and prints one line per
finding (``error|warning CODE file#pointer: message``) and a summary. Exit
status: 0 when no error was found (warnings allowed), 1 when at least one
error was found, 2 on a usage error. ``--self-test`` runs the built-in vectors
and every case under tools/validate_fixtures/cases and checks the exact error
and warning codes each case expects.

Needs Python 3.12 or later, jsonschema >= 4.18 (4.26 tested) and referencing.
The rules mirror crates/palimpsest_data (ids.rs, hex.rs, registry.rs,
content_check.rs); doc 19 §7 lists what the Rust loader must match.
"""
from __future__ import annotations

import argparse
import collections
import functools
import json
import os
import pathlib
import re
import sys
from dataclasses import dataclass, field

try:
    import referencing
    import referencing.exceptions
    from jsonschema import Draft202012Validator, ValidationError
    from jsonschema.exceptions import SchemaError, best_match
    from jsonschema.validators import extend
    from referencing.jsonschema import DRAFT202012
except ImportError:  # pragma: no cover - environment problem
    print("validate_content: needs jsonschema >= 4.18 and referencing "
          "(python3 -m pip install jsonschema)", file=sys.stderr)
    sys.exit(2)

WORKSPACE = pathlib.Path(__file__).resolve().parent.parent
FIXTURES = WORKSPACE / "tools" / "validate_fixtures"

MAX_SAFE = (1 << 53) - 1
META_SCHEMA = "https://json-schema.org/draft/2020-12/schema"
URN = "urn:palimpsest:schema:"
LIBRARY_SCHEMAS = ("common", "envelope")
RECORD_BASE_REF = URN + "common#/$defs/record_base"
RECORD_SCHEMA_RE = re.compile(r"^(0[0-9]|1[0-8])\.[a-z][a-z0-9_]{0,47}$")
ID_PATTERN = "^(?!loc\\.)[a-z][a-z0-9_]*(\\.[a-z0-9_]+){1,5}$"
CONTENT_FILE_RE = re.compile(r"^content/(0[0-9]|1[0-8])/([a-z][a-z0-9_]*)/([a-z0-9_]+)\.json$")
PREFIX_RE = re.compile(r"^[a-z][a-z0-9_]*\.([a-z0-9_]+\.){0,4}$")
OWNER_RE = re.compile(r"^(0[0-9]|1[0-8])$")
NAMESPACES_REL = "content/00/meta/namespaces.json"
DEPTH_LIMIT = 256
MSG_LIMIT = 400

# Every finding code, in pipeline stage order (doc 19 §4.2).
CODES = {
    "E_SCHEMA_DEF": (1, "a schema file breaks the schema rules of doc 19 §1.3 and §3"),
    "E_NAMESPACES": (2, "the ownership table content/00/meta/namespaces.json is missing or malformed"),
    "E_PATH": (3, "a file under content/ is not content/<NN>/<namespace>/<name>.json"),
    "E_JSON": (4, "the file is not well-formed UTF-8 JSON, or an object repeats a key"),
    "E_FLOAT_TOKEN": (4, "a number token has a fraction or exponent part, or is -0"),
    "E_INT_RANGE": (4, "an integer token lies outside ±(2^53 − 1)"),
    "E_ENVELOPE": (5, "the file is not {schema, records} or a record is not an object with a string id"),
    "E_ID_GRAMMAR": (5, "a record ID, alias or reference breaks the content-ID grammar"),
    "E_SCHEMA_UNKNOWN": (6, "the envelope names a schema_id with no schema file"),
    "E_SCHEMA_OWNER": (6, "the schema's x-owner differs from the file's owner directory"),
    "E_SCHEMA": (7, "a record fails its JSON Schema"),
    "E_REF_PREFIX": (7, "a reference is outside the namespaces its x-idref allows"),
    "E_SUM": (7, "an x-sum total is not exactly the required value"),
    "E_SUM_MAX": (7, "an x-sum-max total exceeds its maximum"),
    "E_UNIQUE": (7, "an x-unique-by key repeats within an array"),
    "E_NS_DIR": (8, "a record's namespace differs from its file's namespace directory"),
    "E_NS_UNKNOWN": (8, "a record's ID matches no namespace prefix of canon §19"),
    "E_OWNER": (8, "a record's namespace is not owned by the file's owner directory"),
    "E_MOD_ID": (8, "base content uses a second segment that starts with x_ (canon §1.1)"),
    "E_RETIRED": (8, "a record ID, alias or reference is a retired ID (canon §17.7)"),
    "E_DUP_ID": (9, "the same ID is defined twice (record or alias)"),
    "E_ID32_COLLISION": (9, "two different IDs share an id32"),
    "E_REF_MISSING": (10, "a reference names no record and no alias"),
    "W_REF_MISSING": (10, "as E_REF_MISSING, downgraded by --allow-missing-prefix"),
    "W_REF_ALIAS": (10, "a reference uses an alias's old ID instead of the current ID"),
}

# Schema keywords whose values hold subschemas (draft 2020-12).
SUB_MAP = ("properties", "patternProperties", "$defs", "dependentSchemas")
SUB_ONE = ("additionalProperties", "unevaluatedProperties", "items", "contains", "propertyNames",
           "not", "if", "then", "else", "unevaluatedItems", "contentSchema")
SUB_LIST = ("allOf", "anyOf", "oneOf", "prefixItems")
CUSTOM_KEYWORDS = ("x-idref", "x-sum", "x-sum-max", "x-unique-by", "x-owner", "x-patchable")


# --------------------------------------------------------------------------
# Rules shared with palimpsest_data
# --------------------------------------------------------------------------

def fnv1a32(data: bytes) -> int:
    """Canon §1.1 FNV-1a 32-bit: offset 0x811C9DC5, prime 0x01000193."""
    h = 0x811C9DC5
    for b in data:
        h ^= b
        h = (h * 0x01000193) & 0xFFFFFFFF
    return h


def id32(content_id: str) -> int:
    """Canon §1.1 id32 = fnv1a32(utf8(id))."""
    return fnv1a32(content_id.encode("utf-8"))


def id_error(s: str) -> str | None:
    """palimpsest_data::ids::validate_content_id, same order and messages; None when valid."""
    b = s.encode("utf-8")
    if not b:
        return "content id is empty"
    if len(b) > 64:
        return f"content id is {len(b)} bytes, more than 64"
    if not 0x61 <= b[0] <= 0x7A:
        return f"content id starts with byte 0x{b[0]:02x}, not a..=z"
    segments, seg_len = 1, 0
    for i, c in enumerate(b):
        if c == 0x2E:
            if seg_len == 0:
                return f"content id segment {segments - 1} is empty"
            segments += 1
            seg_len = 0
        elif 0x61 <= c <= 0x7A or 0x30 <= c <= 0x39 or c == 0x5F:
            seg_len += 1
        else:
            return f"content id has byte 0x{c:02x} at offset {i}, outside [a-z0-9_.]"
    if seg_len == 0:
        return f"content id segment {segments - 1} is empty"
    if segments < 2:
        return "content id has no segment after the namespace"
    if segments > 6:
        return f"content id has {segments} segments, more than 6"
    if b.startswith(b"loc."):
        return "namespace `loc` is reserved for text keys (canon §1.10)"
    return None


def id_grammar_message(s: str, reason: str) -> str:
    return f"{json.dumps(s)} is not a content ID: {reason}"


# --------------------------------------------------------------------------
# JSON parsing with the token pre-pass (stage 4)
# --------------------------------------------------------------------------

class FloatToken:
    """A number token that is not an integer; kept so the envelope can still be read."""
    __slots__ = ("text",)

    def __init__(self, text: str):
        self.text = text

    def __repr__(self) -> str:
        return self.text


class PStr(str):
    """A JSON string that remembers its JSON Pointer in the file."""

    def __new__(cls, value: str, pointer: str):
        obj = super().__new__(cls, value)
        obj.pointer = pointer
        return obj


class JsonSyntaxError(Exception):
    def __init__(self, message: str, pos: int):
        super().__init__(message)
        self.message = message
        self.pos = pos


def esc(seg) -> str:
    """RFC 6901 escape of one pointer segment."""
    return str(seg).replace("~", "~0").replace("/", "~1")


def pointer_of(path) -> str:
    return "".join("/" + esc(s) for s in path)


def pointer_key(ptr: str):
    return tuple((0, int(s), "") if s.isdigit() else (1, 0, s) for s in ptr.split("/")[1:])


class Parser:
    """RFC 8259 parser that records a pointer for every string and checks every number token.

    Number tokens with '.', 'e' or 'E', and the token -0, give E_FLOAT_TOKEN; integer tokens
    beyond ±(2^53 − 1) give E_INT_RANGE (canon §1.6, doc 02 §11.1). String contents are never
    scanned for numbers. Duplicate object keys are a syntax error.
    """
    WS = " \t\n\r"
    NUM_RE = re.compile(r"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?")
    PLAIN_RE = re.compile(r'[^"\\\x00-\x1f]+')
    HEX4_RE = re.compile(r"[0-9A-Fa-f]{4}")
    ESCAPES = {'"': '"', "\\": "\\", "/": "/", "b": "\b", "f": "\f", "n": "\n", "r": "\r", "t": "\t"}

    def __init__(self, text: str):
        self.t = text
        self.n = len(text)
        self.i = 0
        self.issues: list[tuple[str, str, str]] = []

    def fail(self, message: str, pos: int | None = None):
        raise JsonSyntaxError(message, self.i if pos is None else pos)

    def ws(self):
        t, n, i = self.t, self.n, self.i
        while i < n and t[i] in self.WS:
            i += 1
        self.i = i

    def parse(self):
        self.ws()
        value = self.value("", 0)
        self.ws()
        if self.i != self.n:
            self.fail("unexpected data after the JSON value")
        return value

    def value(self, ptr: str, depth: int):
        if depth > DEPTH_LIMIT:
            self.fail(f"nesting deeper than {DEPTH_LIMIT}")
        if self.i >= self.n:
            self.fail("unexpected end of input")
        c = self.t[self.i]
        if c == "{":
            return self.obj(ptr, depth)
        if c == "[":
            return self.arr(ptr, depth)
        if c == '"':
            return PStr(self.string(), ptr)
        if c == "-" or c.isdigit():
            return self.number(ptr)
        for lit, val in (("true", True), ("false", False), ("null", None)):
            if self.t.startswith(lit, self.i):
                self.i += len(lit)
                return val
        self.fail(f"unexpected character {c!r}")

    def obj(self, ptr: str, depth: int) -> dict:
        self.i += 1
        out: dict = {}
        self.ws()
        if self.i < self.n and self.t[self.i] == "}":
            self.i += 1
            return out
        while True:
            self.ws()
            if self.i >= self.n or self.t[self.i] != '"':
                self.fail("expected a string key")
            key_pos = self.i
            key = self.string()
            self.ws()
            if self.i >= self.n or self.t[self.i] != ":":
                self.fail("expected ':'")
            self.i += 1
            self.ws()
            if key in out:
                self.fail(f"duplicate object key {json.dumps(key)}", key_pos)
            out[key] = self.value(ptr + "/" + esc(key), depth + 1)
            self.ws()
            if self.i < self.n and self.t[self.i] == ",":
                self.i += 1
                continue
            if self.i < self.n and self.t[self.i] == "}":
                self.i += 1
                return out
            self.fail("expected ',' or '}'")

    def arr(self, ptr: str, depth: int) -> list:
        self.i += 1
        out: list = []
        self.ws()
        if self.i < self.n and self.t[self.i] == "]":
            self.i += 1
            return out
        while True:
            self.ws()
            out.append(self.value(f"{ptr}/{len(out)}", depth + 1))
            self.ws()
            if self.i < self.n and self.t[self.i] == ",":
                self.i += 1
                continue
            if self.i < self.n and self.t[self.i] == "]":
                self.i += 1
                return out
            self.fail("expected ',' or ']'")

    def string(self) -> str:
        t, n = self.t, self.n
        i = self.i + 1
        buf: list[str] = []
        while True:
            if i >= n:
                self.fail("unterminated string", self.i)
            m = self.PLAIN_RE.match(t, i)
            if m:
                buf.append(m.group(0))
                i = m.end()
                continue
            c = t[i]
            if c == '"':
                self.i = i + 1
                return "".join(buf)
            if c != "\\":
                self.fail("control character in string", i)
            e = t[i + 1] if i + 1 < n else ""
            if e in self.ESCAPES:
                buf.append(self.ESCAPES[e])
                i += 2
                continue
            if e != "u" or not self.HEX4_RE.fullmatch(t, i + 2, i + 6):
                self.fail("invalid escape", i)
            cp = int(t[i + 2:i + 6], 16)
            i += 6
            if 0xD800 <= cp <= 0xDBFF and t.startswith("\\u", i) and self.HEX4_RE.fullmatch(t, i + 2, i + 6):
                lo = int(t[i + 2:i + 6], 16)
                if 0xDC00 <= lo <= 0xDFFF:
                    cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00)
                    i += 6
            if 0xD800 <= cp <= 0xDFFF:
                self.fail("lone surrogate in \\u escape", i - 6)
            buf.append(chr(cp))

    def number(self, ptr: str):
        m = self.NUM_RE.match(self.t, self.i)
        if not m:
            self.fail("invalid number")
        tok = m.group(0)
        self.i = m.end()
        if any(ch in tok for ch in ".eE"):
            self.issues.append((ptr, "E_FLOAT_TOKEN", f"number {tok} has a fraction or exponent part (canon §1.6)"))
            return FloatToken(tok)
        if tok == "-0":
            self.issues.append((ptr, "E_FLOAT_TOKEN", "number -0 is negative zero, which only a float holds (canon §1.6)"))
            return FloatToken(tok)
        digits = tok.lstrip("-")
        if len(digits) > 16 or int(digits) > MAX_SAFE:
            self.issues.append((ptr, "E_INT_RANGE",
                                f"integer {tok} is outside ±(2^53 − 1); write it as a \"0x\" hex string (canon §1.6)"))
            return FloatToken(tok)
        return int(tok)


def parse_text(raw: bytes):
    """Decode and parse; returns (value, issues) or raises JsonSyntaxError."""
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as e:
        raise JsonSyntaxError(f"not UTF-8 (byte offset {e.start})", -1) from None
    if text.startswith("﻿"):
        raise JsonSyntaxError("byte order mark before the JSON value", 0)
    p = Parser(text)
    try:
        value = p.parse()
    except JsonSyntaxError as e:
        if e.pos >= 0:
            line = text.count("\n", 0, e.pos) + 1
            col = e.pos - (text.rfind("\n", 0, e.pos) + 1) + 1
            e.message = f"line {line} column {col}: {e.message}"
        raise
    return value, p.issues


# --------------------------------------------------------------------------
# Report
# --------------------------------------------------------------------------

@dataclass
class Finding:
    code: str
    file: str
    pointer: str
    message: str

    @property
    def level(self) -> str:
        return "warning" if self.code.startswith("W_") else "error"

    def line(self) -> str:
        return f"{self.level} {self.code} {self.file}#{self.pointer}: {self.message}"

    def as_dict(self) -> dict:
        return {"code": self.code, "file": self.file, "pointer": self.pointer, "message": self.message}


class Report:
    """Findings in emission order; an identical (code, file, pointer, message) is kept once."""

    def __init__(self):
        self.items: list[Finding] = []
        self._seen: set[tuple] = set()

    def add(self, code: str, file: str, pointer: str, message: str):
        assert code in CODES, code
        if len(message) > MSG_LIMIT:
            message = message[:MSG_LIMIT - 3] + "..."
        key = (code, file, pointer, message)
        if key not in self._seen:
            self._seen.add(key)
            self.items.append(Finding(code, file, pointer, message))

    @property
    def errors(self) -> list[Finding]:
        return [f for f in self.items if f.level == "error"]

    @property
    def warnings(self) -> list[Finding]:
        return [f for f in self.items if f.level == "warning"]


# --------------------------------------------------------------------------
# Custom keywords (doc 19 §3)
# --------------------------------------------------------------------------

class _Context:
    refs: list | None = None


CTX = _Context()


def _err(message: str, path, code: str) -> ValidationError:
    e = ValidationError(message, path=path)
    e.pal_code = code
    return e


def kw_idref(validator, prefixes, instance, schema):
    """x-idref: every string (or string element) is a content ID in an allowed namespace; existence is stage 10."""
    if isinstance(instance, str):
        items = [((), instance)]
    elif isinstance(instance, list):
        items = [((i,), x) for i, x in enumerate(instance) if isinstance(x, str)]
    else:
        return
    for path, s in items:
        reason = id_error(s)
        if reason:
            yield _err(id_grammar_message(s, reason), path, "E_ID_GRAMMAR")
            continue
        if "*" not in prefixes and not any(s.startswith(p) for p in prefixes):
            yield _err(f"{json.dumps(s)} is not in a namespace this field allows ({', '.join(prefixes)})",
                       path, "E_REF_PREFIX")
            continue
        if CTX.refs is not None and isinstance(s, PStr):
            CTX.refs.append((s.pointer, str(s), tuple(prefixes)))


def _sum(instance, fld):
    if isinstance(instance, list):
        values = instance
    elif isinstance(instance, dict):
        values = list(instance.values())
    else:
        return None
    total = 0
    for v in values:
        x = (v.get(fld) if isinstance(v, dict) else None) if fld is not None else v
        if isinstance(x, int) and not isinstance(x, bool):
            total += x
    return total


def _sum_label(fld) -> str:
    return f"field {json.dumps(fld)}" if fld is not None else "the integer values"


def kw_sum(validator, spec, instance, schema):
    """x-sum: the summed integers equal spec.total exactly."""
    s = _sum(instance, spec.get("field"))
    if s is not None and s != spec["total"]:
        yield _err(f"sum of {_sum_label(spec.get('field'))} is {s}, must equal {spec['total']}", (), "E_SUM")


def kw_sum_max(validator, spec, instance, schema):
    """x-sum-max: the summed integers are at most spec.max."""
    s = _sum(instance, spec.get("field"))
    if s is not None and s > spec["max"]:
        yield _err(f"sum of {_sum_label(spec.get('field'))} is {s}, more than {spec['max']}", (), "E_SUM_MAX")


def kw_unique_by(validator, spec, instance, schema):
    """x-unique-by: among object elements carrying every key field, the key tuple is unique."""
    if not isinstance(instance, list):
        return
    fields = [spec] if isinstance(spec, str) else list(spec)
    seen: dict[str, int] = {}
    for i, el in enumerate(instance):
        if not isinstance(el, dict) or any(f not in el for f in fields):
            continue
        key = json.dumps([el[f] for f in fields], sort_keys=True, separators=(",", ":"), default=repr)
        if key in seen:
            yield _err(f"duplicate {'+'.join(fields)} {key[1:-1]} (first at index {seen[key]})", (i,), "E_UNIQUE")
        else:
            seen[key] = i


@functools.lru_cache(maxsize=None)
def _py_pattern(pattern: str) -> re.Pattern:
    """ECMA-262 '$' means end of input; Python's '$' also matches before a final newline."""
    if pattern.endswith("$") and not pattern.endswith("\\$"):
        pattern = pattern[:-1] + r"\Z"
    return re.compile(pattern)


def kw_pattern(validator, pattern, instance, schema):
    if isinstance(instance, str) and not _py_pattern(pattern).search(instance):
        yield ValidationError(f"{instance!r} does not match {pattern!r}")


PalValidator = extend(Draft202012Validator, validators={
    "x-idref": kw_idref, "x-sum": kw_sum, "x-sum-max": kw_sum_max,
    "x-unique-by": kw_unique_by, "pattern": kw_pattern,
})


def classify(err: ValidationError) -> tuple[str, str]:
    code = getattr(err, "pal_code", None)
    if code:
        return code, err.message
    if err.validator == "x-sum":
        return "E_SUM", err.message
    if (err.validator in ("pattern", "maxLength") and isinstance(err.schema, dict)
            and err.schema.get("pattern") == ID_PATTERN and isinstance(err.instance, str)):
        reason = id_error(err.instance)
        if reason:
            return "E_ID_GRAMMAR", id_grammar_message(err.instance, reason)
    msg = err.message
    if err.context:
        bm = best_match(err.context)
        code_bm, msg_bm = classify(bm)
        msg += f"; closest branch: {msg_bm} at #{pointer_of(bm.relative_path) or '/'}"
    return "E_SCHEMA", msg


# --------------------------------------------------------------------------
# Schema set (stage 1)
# --------------------------------------------------------------------------

@dataclass
class SchemaEntry:
    schema_id: str
    file: str
    contents: object
    owner: str | None = None
    ok: bool = True
    validator: object = None


def walk_schema(schema, ptr: str = ""):
    """Yield (subschema, pointer) for every subschema position of a 2020-12 schema."""
    if not isinstance(schema, dict):
        return
    yield schema, ptr
    for k in SUB_MAP:
        if isinstance(schema.get(k), dict):
            for name, sub in schema[k].items():
                yield from walk_schema(sub, f"{ptr}/{esc(k)}/{esc(name)}")
    for k in SUB_ONE:
        if k in schema:
            yield from walk_schema(schema[k], f"{ptr}/{esc(k)}")
    for k in SUB_LIST:
        if isinstance(schema.get(k), list):
            for i, sub in enumerate(schema[k]):
                yield from walk_schema(sub, f"{ptr}/{esc(k)}/{i}")


def _is_int(v) -> bool:
    return isinstance(v, int) and not isinstance(v, bool)


def check_custom_keywords(sub: dict, ptr: str, is_root: bool, namespaces: set[str] | None) -> list[tuple[str, str]]:
    """Doc 19 §3 well-formedness of x- keyword values; returns (pointer, message) pairs."""
    out = []
    t = sub.get("type")
    if t == "number" or (isinstance(t, list) and "number" in t):
        out.append((ptr + "/type", "type number is forbidden; content holds integers only (canon §1.6)"))
    pat = sub.get("pattern")
    if isinstance(pat, str) and re.search(r"\\[dwsDWS]", pat):
        out.append((ptr + "/pattern", "pattern uses \\d, \\w or \\s, which differ between ECMA-262 and Python; "
                                      "write explicit classes such as [0-9]"))
    for k, v in sub.items():
        if not isinstance(k, str) or not k.startswith("x-"):
            continue
        kp = f"{ptr}/{esc(k)}"
        if k not in CUSTOM_KEYWORDS:
            out.append((kp, f"unknown custom keyword {k}; doc 19 §3 defines {', '.join(CUSTOM_KEYWORDS)}"))
        elif k == "x-idref":
            ok = (isinstance(v, list) and v and len(set(map(str, v))) == len(v)
                  and all(isinstance(p, str) and (p == "*" or PREFIX_RE.fullmatch(p)) for p in v)
                  and ("*" not in v or len(v) == 1))
            if not ok:
                out.append((kp, "x-idref must be a non-empty array of unique namespace prefixes "
                                "such as \"genre.\" or \"arch.melee.\", or exactly [\"*\"]"))
            elif namespaces is not None:
                for p in v:
                    if p != "*" and p.split(".")[0] not in namespaces:
                        out.append((kp, f"x-idref prefix {p} names no namespace of canon §19"))
        elif k in ("x-sum", "x-sum-max"):
            need = "total" if k == "x-sum" else "max"
            ok = (isinstance(v, dict) and need in v and _is_int(v[need]) and set(v) <= {need, "field"}
                  and (("field" not in v) or (isinstance(v["field"], str) and v["field"])))
            if not ok:
                out.append((kp, f"{k} must be {{\"{need}\": <integer>}} with an optional non-empty string \"field\""))
        elif k == "x-unique-by":
            ok = (isinstance(v, str) and v) or (
                isinstance(v, list) and v and len(set(map(str, v))) == len(v)
                and all(isinstance(f, str) and f for f in v))
            if not ok:
                out.append((kp, "x-unique-by must be a field name or a non-empty array of unique field names"))
        elif k == "x-owner":
            if not is_root:
                out.append((kp, "x-owner is allowed only at the schema root"))
            elif not (isinstance(v, str) and OWNER_RE.fullmatch(v)):
                out.append((kp, "x-owner must be a two-digit owner document \"00\" to \"18\""))
        elif k == "x-patchable":
            if not isinstance(v, bool) or sub.get("type") != "array":
                out.append((kp, "x-patchable must be a boolean on a schema with \"type\": \"array\""))
    return out


def load_schemas(dirs: list[pathlib.Path], root: pathlib.Path, report: Report,
                 namespaces: set[str] | None) -> dict[str, SchemaEntry]:
    """Stage 1: read <dir>/*.schema.json (earlier directories shadow later ones) and check each schema."""
    entries: dict[str, SchemaEntry] = {}
    for d in dirs:
        if not d.is_dir():
            continue
        for path in sorted(d.iterdir(), key=lambda p: p.name.encode()):
            if path.name.startswith(".") or not path.is_file():
                continue
            disp = rel(path, root)
            name = path.name
            sid = name[: -len(".schema.json")] if name.endswith(".schema.json") else None
            if sid is None or not (sid in LIBRARY_SCHEMAS or RECORD_SCHEMA_RE.fullmatch(sid)):
                report.add("E_SCHEMA_DEF", disp, "", "schema files are named <schema_id>.schema.json with schema_id "
                           "common, envelope or <NN>.<record_kind> (doc 19 §1.3)")
                continue
            if sid in entries:
                continue
            try:
                contents, issues = parse_text(path.read_bytes())
            except JsonSyntaxError as e:
                report.add("E_JSON", disp, "", e.message)
                continue
            for ptr, code, msg in issues:
                report.add(code, disp, ptr, msg)
            entry = SchemaEntry(sid, disp, contents, ok=not issues)
            entries[sid] = entry
            if issues:
                continue
            if not isinstance(contents, dict):
                report.add("E_SCHEMA_DEF", disp, "", "a schema file holds a JSON object")
                entry.ok = False
                continue
            problems: list[tuple[str, str]] = []
            if contents.get("$schema") != META_SCHEMA:
                problems.append(("/$schema", f"\"$schema\" must be \"{META_SCHEMA}\""))
            if contents.get("$id") != URN + sid:
                problems.append(("/$id", f"\"$id\" must be \"{URN}{sid}\""))
            for sub, ptr in walk_schema(contents):
                problems += check_custom_keywords(sub, ptr, ptr == "", namespaces)
            if sid in LIBRARY_SCHEMAS:
                if "x-owner" in contents:
                    problems.append(("/x-owner", "library schemas carry no x-owner"))
            else:
                if contents.get("x-owner") != sid[:2]:
                    problems.append(("/x-owner", f"a record schema carries \"x-owner\": \"{sid[:2]}\", "
                                                 "the owner part of its schema_id"))
                if contents.get("$ref") != RECORD_BASE_REF:
                    problems.append(("/$ref", f"a record schema root is \"$ref\": \"{RECORD_BASE_REF}\""))
                if contents.get("unevaluatedProperties") is not False:
                    problems.append(("/unevaluatedProperties",
                                     "a record schema root has \"unevaluatedProperties\": false"))
            try:
                PalValidator.check_schema(contents)
            except SchemaError as e:
                problems.append((pointer_of(e.path), f"not a valid 2020-12 schema: {e.message}"))
            for ptr, msg in problems:
                report.add("E_SCHEMA_DEF", disp, ptr, msg)
            if problems:
                entry.ok = False
    return entries


def build_registry(entries: dict[str, SchemaEntry], report: Report) -> referencing.Registry:
    """Register every schema whose $id is right, then check that every $ref resolves."""
    resources = [(URN + e.schema_id, DRAFT202012.create_resource(e.contents))
                 for e in entries.values() if isinstance(e.contents, dict) and e.contents.get("$id") == URN + e.schema_id]
    registry = referencing.Registry().with_resources(resources)
    for e in entries.values():
        if not isinstance(e.contents, dict):
            continue
        resolver = registry.resolver(base_uri=URN + e.schema_id)
        for sub, ptr in walk_schema(e.contents):
            ref = sub.get("$ref")
            if isinstance(ref, str):
                try:
                    resolver.lookup(ref)
                except referencing.exceptions.Unresolvable:
                    report.add("E_SCHEMA_DEF", e.file, ptr + "/$ref", f"$ref {ref} does not resolve")
                    e.ok = False
    for e in entries.values():
        if e.ok:
            e.owner = e.contents.get("x-owner")
            e.validator = PalValidator(e.contents, registry=registry)
    return registry


# --------------------------------------------------------------------------
# Ownership table (stage 2)
# --------------------------------------------------------------------------

@dataclass
class Ownership:
    prefixes: list[tuple[str, list[str], str]] = field(default_factory=list)  # (prefix, owners, source), longest first
    retired_ids: set[str] = field(default_factory=set)
    retired_prefixes: list[str] = field(default_factory=list)

    @property
    def namespaces(self) -> set[str]:
        return {p.split(".")[0] for p, _, _ in self.prefixes}

    def lookup(self, cid: str):
        for p, owners, src in self.prefixes:
            if cid.startswith(p):
                return p, owners, src
        return None

    def retired(self, cid: str) -> bool:
        return cid in self.retired_ids or any(cid.startswith(p) for p in self.retired_prefixes)


def load_ownership(path: pathlib.Path, root: pathlib.Path, report: Report) -> Ownership | None:
    disp = rel(path, root)
    if not path.is_file():
        report.add("E_NAMESPACES", disp, "", "ownership table missing; run python3 tools/gen_namespaces.py")
        return None
    try:
        doc, issues = parse_text(path.read_bytes())
    except JsonSyntaxError as e:
        report.add("E_NAMESPACES", disp, "", f"ownership table unreadable: {e.message}")
        return None
    own = Ownership()
    recs = doc.get("records") if isinstance(doc, dict) else None
    if issues or not isinstance(recs, list):
        report.add("E_NAMESPACES", disp, "", "ownership table is not a {schema, records} file of integers")
        return None
    for i, r in enumerate(recs):
        kind = r.get("kind") if isinstance(r, dict) else None
        if kind == "prefix" and isinstance(r.get("prefix"), str) and PREFIX_RE.fullmatch(r["prefix"]) \
                and isinstance(r.get("owners"), list) and r["owners"] \
                and all(isinstance(o, str) and OWNER_RE.fullmatch(o) for o in r["owners"]):
            if r.get("text_keys") is not True:
                own.prefixes.append((str(r["prefix"]), [str(o) for o in r["owners"]], str(r.get("source", ""))))
        elif kind == "retired" and isinstance(r.get("retired"), str):
            tok = str(r["retired"])
            (own.retired_prefixes.append(tok) if tok.endswith(".") else own.retired_ids.add(tok))
        else:
            report.add("E_NAMESPACES", disp, f"/records/{i}", "ownership record is neither a well-formed prefix "
                       "record nor a retired record (doc 19 §1.4)")
            return None
    own.prefixes.sort(key=lambda x: (-len(x[0]), x[0]))
    return own


# --------------------------------------------------------------------------
# Content validation (stages 3 to 10)
# --------------------------------------------------------------------------

@dataclass
class ContentFile:
    rel: str
    owner: str
    nsdir: str
    value: object = None
    token_ok: bool = True
    dead: bool = False
    bad_records: set[int] = field(default_factory=set)
    bad_aliases: set[tuple[int, int]] = field(default_factory=set)
    schema: SchemaEntry | None = None

    def records(self):
        if self.dead:
            return
        for i, r in enumerate(self.value["records"]):
            if i not in self.bad_records:
                yield i, r


@dataclass
class Stats:
    schemas: int = 0
    files: int = 0
    records: int = 0
    aliases: int = 0
    references: int = 0
    resolved: int = 0


def rel(path: pathlib.Path, root: pathlib.Path) -> str:
    return pathlib.PurePath(os.path.relpath(path, root)).as_posix()


def discover(root: pathlib.Path, report: Report) -> list[ContentFile]:
    """Stage 3: every non-hidden file under content/, in ascending relative-path byte order."""
    base = root / "content"
    found = []
    if not base.is_dir():
        return found
    for dirpath, dirnames, filenames in os.walk(base):
        dirnames[:] = [d for d in dirnames if not d.startswith(".")]
        for fn in filenames:
            if not fn.startswith("."):
                found.append(rel(pathlib.Path(dirpath) / fn, root))
    files = []
    for r in sorted(found, key=lambda s: s.encode()):
        m = CONTENT_FILE_RE.fullmatch(r)
        if not m:
            report.add("E_PATH", r, "", "content files live at content/<NN>/<namespace>/<name>.json with NN in 00..=18, "
                       "namespace [a-z][a-z0-9_]* and name [a-z0-9_]+ (doc 02 §11.1, doc 19 §1.1)")
            continue
        files.append(ContentFile(r, m.group(1), m.group(2)))
    return files


def validate(root: pathlib.Path, schema_dirs: list[pathlib.Path], namespaces_file: pathlib.Path | None = None,
             allow_missing: tuple[str, ...] = ()) -> tuple[Report, Stats]:
    report, stats = Report(), Stats()

    # Stage 2 runs before stage 1 so x-idref prefixes can be checked against canon §19.
    ns_path = namespaces_file if namespaces_file is not None else root / NAMESPACES_REL
    stage2 = Report()
    own = load_ownership(ns_path, root, stage2)

    # Stage 1: schemas.
    schemas = load_schemas(schema_dirs, root, report, own.namespaces if own else None)
    for lib in LIBRARY_SCHEMAS:
        if lib not in schemas:
            report.add("E_SCHEMA_DEF", f"schemas/{lib}.schema.json", "", "required library schema missing")
    build_registry(schemas, report)
    stats.schemas = len(schemas)
    for f in stage2.items:
        report.add(f.code, f.file, f.pointer, f.message)
    envelope = schemas.get("envelope")
    if envelope is None or not envelope.ok:
        return report, stats

    # Stage 3: discovery.
    files = discover(root, report)
    stats.files = len(files)

    # Stage 4: token pre-pass.
    for cf in files:
        try:
            cf.value, issues = parse_text((root / cf.rel).read_bytes())
        except JsonSyntaxError as e:
            report.add("E_JSON", cf.rel, "", e.message)
            cf.dead = True
            continue
        for ptr, code, msg in issues:
            report.add(code, cf.rel, ptr, msg)
        cf.token_ok = not issues

    # Stage 5: envelope, record ID and alias grammar.
    for cf in files:
        if cf.dead:
            continue
        errs = sorted(envelope.validator.iter_errors(cf.value), key=lambda e: pointer_key(pointer_of(e.absolute_path)))
        for err in errs:
            ptr = pointer_of(err.absolute_path)
            m = re.match(r"^/records/(\d+)(/|$)", ptr)
            if m:
                cf.bad_records.add(int(m.group(1)))
            else:
                cf.dead = True
            report.add("E_ENVELOPE", cf.rel, ptr, err.message)
        if cf.dead:
            continue
        for i, r in list(cf.records()):
            reason = id_error(r["id"])
            if reason:
                report.add("E_ID_GRAMMAR", cf.rel, f"/records/{i}/id", id_grammar_message(r["id"], reason))
                cf.bad_records.add(i)
                continue
            for j, a in enumerate(r.get("aliases", [])):
                reason = id_error(a)
                if reason:
                    report.add("E_ID_GRAMMAR", cf.rel, f"/records/{i}/aliases/{j}", id_grammar_message(a, reason))
                    cf.bad_aliases.add((i, j))

    # Stage 6: schema binding.
    for cf in files:
        if cf.dead:
            continue
        sid = cf.value["schema"]
        entry = schemas.get(sid)
        if entry is None:
            report.add("E_SCHEMA_UNKNOWN", cf.rel, "/schema", f"no schema file schemas/{sid}.schema.json")
            continue
        if not entry.ok:
            continue
        cf.schema = entry
        if entry.owner != cf.owner:
            report.add("E_SCHEMA_OWNER", cf.rel, "/schema", f"schema {sid} has x-owner {entry.owner}, so its files "
                       f"live under content/{entry.owner}/, not content/{cf.owner}/")

    # Stage 7: record schemas and custom keywords; collect references.
    refs: dict[tuple[str, str], list] = {}
    for cf in files:
        if cf.dead or cf.schema is None or not cf.token_ok:
            continue
        for i, r in cf.records():
            CTX.refs = []
            found = []
            for err in cf.schema.validator.iter_errors(r):
                code, msg = classify(err)
                found.append((f"/records/{i}{pointer_of(err.absolute_path)}", code, msg))
            for ptr, code, msg in sorted(found, key=lambda x: (pointer_key(x[0]), x[1], x[2])):
                report.add(code, cf.rel, ptr, msg)
            for ptr, cid, prefixes in CTX.refs:
                slot = refs.setdefault((cf.rel, ptr), [cid, set()])
                slot[1].update(prefixes)
            CTX.refs = None
    stats.references = len(refs)

    # Stage 8: placement and ownership.
    mod_named: set[tuple[str, str]] = set()  # (file, pointer) of names refused by the x_ rule
    for cf in files:
        for i, r in cf.records():
            cid = r["id"]
            ptr = f"/records/{i}/id"
            ns = cid.split(".")[0]
            if ns != cf.nsdir:
                report.add("E_NS_DIR", cf.rel, ptr, f"record namespace {ns} differs from the directory "
                           f"content/{cf.owner}/{cf.nsdir}/")
            if own is not None:
                hit = own.lookup(cid)
                if hit is None:
                    report.add("E_NS_UNKNOWN", cf.rel, ptr, f"{json.dumps(cid)} matches no namespace prefix of canon §19")
                elif cf.owner not in hit[1]:
                    report.add("E_OWNER", cf.rel, ptr, f"namespace prefix {hit[0]} is owned by "
                               f"{', '.join(hit[1])} ({hit[2]}), not by {cf.owner}")
            names = [(ptr, cid)] + [(f"/records/{i}/aliases/{j}", a) for j, a in enumerate(r.get("aliases", []))
                                    if (i, j) not in cf.bad_aliases]
            for p, name in names:
                if name.split(".")[1].startswith("x_"):
                    report.add("E_MOD_ID", cf.rel, p, f"{json.dumps(name)}: base content id has a second segment "
                               "starting with `x_` (canon §1.1)")
                    mod_named.add((cf.rel, p))
                if own is not None and own.retired(name):
                    report.add("E_RETIRED", cf.rel, p, f"{json.dumps(name)} is a retired ID (canon §17.7)")

    # Stage 9: registry.
    content, aliases = [], []
    for cf in files:
        for i, r in cf.records():
            content.append((r["id"], cf.rel, f"/records/{i}/id"))
            for j, a in enumerate(r.get("aliases", [])):
                if (i, j) not in cf.bad_aliases:
                    aliases.append((a, r["id"], cf.rel, f"/records/{i}/aliases/{j}"))
    taken: dict[int, tuple[str, str, str]] = {}
    ids: set[str] = set()
    alias_target: dict[str, str] = {}
    for kind, items in (("record", sorted(content, key=lambda x: (x[0].encode(), x[1], pointer_key(x[2])))),
                        ("alias", sorted(aliases, key=lambda x: (x[0].encode(), x[1].encode(), x[2], pointer_key(x[3]))))):
        for item in items:
            name, file, ptr = item[0], item[-2], item[-1]
            if (file, ptr) in mod_named:
                continue  # palimpsest_data::registry refuses it with mod_segment_invalid
            h = id32(name)
            if h in taken:
                prev = taken[h]
                if prev[0] == name:
                    report.add("E_DUP_ID", file, ptr, f"{json.dumps(name)} ({kind}) is already defined at {prev[1]}#{prev[2]}")
                else:
                    report.add("E_ID32_COLLISION", file, ptr, f"{json.dumps(name)} and {json.dumps(prev[0])} "
                               f"({prev[1]}#{prev[2]}) share id32 0x{h:08x}")
                continue
            taken[h] = (name, file, ptr)
            if kind == "record":
                ids.add(name)
            else:
                alias_target[name] = item[1]
    stats.records = len(ids)
    stats.aliases = len(alias_target)

    # Stage 10: references.
    for (file, ptr), (cid, prefixes) in refs.items():
        if own is not None and own.retired(cid):
            report.add("E_RETIRED", file, ptr, f"{json.dumps(cid)} is a retired ID (canon §17.7)")
            continue
        if cid in ids:
            stats.resolved += 1
            continue
        if cid in alias_target:
            target = alias_target[cid]
            stats.resolved += 1
            report.add("W_REF_ALIAS", file, ptr, f"{json.dumps(cid)} is an alias of {json.dumps(target)}; "
                       "write the current ID")
            if "*" not in prefixes and not any(target.startswith(p) for p in prefixes):
                report.add("E_REF_PREFIX", file, ptr, f"alias {json.dumps(cid)} resolves to {json.dumps(target)}, "
                           f"outside the namespaces this field allows ({', '.join(sorted(prefixes))})")
            continue
        if any(cid.startswith(p) for p in allow_missing):
            report.add("W_REF_MISSING", file, ptr, f"{json.dumps(cid)} names no record or alias (allowed by "
                       "--allow-missing-prefix)")
        else:
            report.add("E_REF_MISSING", file, ptr, f"{json.dumps(cid)} names no record or alias in the content set")
    return report, stats


def summary_line(report: Report, stats: Stats) -> str:
    return (f"validate_content: {stats.files} files, {stats.records} records, {stats.aliases} aliases, "
            f"{stats.references} references ({stats.resolved} resolved), {stats.schemas} schemas: "
            f"{len(report.errors)} errors, {len(report.warnings)} warnings")


# --------------------------------------------------------------------------
# Self-test (doc 19 §4.5)
# --------------------------------------------------------------------------

COLLISION_PAIRS = (  # palimpsest_data::test_support::COLLISION_PAIRS
    ("item.c93fb1c3b7c754cf3", "item.cd16e77dd95ebbaed", 0x1DC7EDB8),
    ("item.c0640759fbcdbd638", "item.cdc57e8cb2128c8ca", 0x5D0CA0C8),
    ("item.c8a651be5c267ee9e", "item.ca63bf2042bd07c1d", 0xC8EE4DCC),
    ("item.c2cda799b408e5506", "item.c56ceaf992bc17b30", 0x82A7FAE9),
)

ID_VECTORS = (  # palimpsest_data::ids::tests::content_id_rejects_each_rule, plus accepted examples
    ("", "content id is empty"),
    ("Genre.Cyberpunk", "content id starts with byte 0x47, not a..=z"),
    ("genre.Cyberpunk", "content id has byte 0x43 at offset 6, outside [a-z0-9_.]"),
    ("genre..x", "content id segment 1 is empty"),
    ("genre.x.", "content id segment 2 is empty"),
    (".genre.x", "content id starts with byte 0x2e, not a..=z"),
    ("1genre.x", "content id starts with byte 0x31, not a..=z"),
    ("_genre.x", "content id starts with byte 0x5f, not a..=z"),
    ("genre", "content id has no segment after the namespace"),
    ("a.b.c.d.e.f.g", "content id has 7 segments, more than 6"),
    ("genre.cyber-punk", "content id has byte 0x2d at offset 11, outside [a-z0-9_.]"),
    ("genre.café", "content id has byte 0xc3 at offset 9, outside [a-z0-9_.]"),
    ("genre.x y", "content id has byte 0x20 at offset 7, outside [a-z0-9_.]"),
    ("loc.item.sword", "namespace `loc` is reserved for text keys (canon §1.10)"),
    ("a." + "b" * 63, "content id is 65 bytes, more than 64"),
    ("A" * 65, "content id is 65 bytes, more than 64"),
    ("a." + "b" * 62, None), ("genre.cyberpunk", None), ("arch.melee.blade_long", None), ("crate.sim", None),
    ("item.x_frostpack.ice_blade", None), ("a.0", None), ("a1_.b.c.d.e.f", None), ("q._", None),
)

TOKEN_VECTORS = (  # doc 02 acceptance test 19, palimpsest_data::content_check tests
    ("1.5", "E_FLOAT_TOKEN"), ("1e3", "E_FLOAT_TOKEN"), ("-0.0", "E_FLOAT_TOKEN"), ("-0", "E_FLOAT_TOKEN"),
    ("1.0", "E_FLOAT_TOKEN"), ("2E0", "E_FLOAT_TOKEN"), ("2e+0", "E_FLOAT_TOKEN"), ("1E-3", "E_FLOAT_TOKEN"),
    ("-0e0", "E_FLOAT_TOKEN"), ("9007199254740992", "E_INT_RANGE"), ("-9007199254740992", "E_INT_RANGE"),
    ("18446744073709551615", "E_INT_RANGE"), ("123456789012345678901234", "E_INT_RANGE"),
    ("9007199254740991", None), ("-9007199254740991", None), ("0", None), ("-1", None), ('"1.5"', None),
    ('"1e3"', None),
)


def unit_checks(workspace: pathlib.Path) -> list[str]:
    """Built-in vectors that tie this file to the canon and to palimpsest_data."""
    fails = []
    for s, h in (("genre.cyberpunk", 0x99B54427), ("crate.sim", 0xDC634881)):
        if id32(s) != h:
            fails.append(f"fnv1a32({s}) = 0x{id32(s):08x}, expected 0x{h:08x} (canon §21 test 4)")
    for a, b, h in COLLISION_PAIRS:
        if not (id32(a) == id32(b) == h and id_error(a) is None and id_error(b) is None):
            fails.append(f"collision pair {a} / {b} does not collide at 0x{h:08x}")
    id_re = _py_pattern(ID_PATTERN)
    for s, want in ID_VECTORS:
        got = id_error(s)
        if got != want:
            fails.append(f"id_error({s!r}) = {got!r}, expected {want!r}")
        if bool(id_re.search(s)) != (want is None) and len(s) <= 64:
            fails.append(f"ID_PATTERN and id_error disagree on {s!r}")
    seed = 0x2545F4914F6CDD1D
    alphabet = "abz09_.Ax-"
    for _ in range(3000):  # differential check of the schema pattern against the byte scanner
        seed = (seed * 6364136223846793005 + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
        n = (seed >> 33) % 14
        s = "".join(alphabet[(seed >> (4 * k)) % len(alphabet)] for k in range(n))
        if len(s) <= 64 and bool(id_re.search(s)) != (id_error(s) is None):
            fails.append(f"ID_PATTERN and id_error disagree on {s!r}")
            break
    for tok, want in TOKEN_VECTORS:
        try:
            _, issues = parse_text(('{"x": ' + tok + "}").encode())
            got = issues[0][1] if issues else None
            where = issues[0][0] if issues else "/x"
        except JsonSyntaxError as e:
            got, where = "E_JSON", e.message
        if got != want or where != "/x":
            fails.append(f"token {tok}: got {got} at {where}, expected {want}")
    common = workspace / "schemas" / "common.schema.json"
    try:
        defs = json.loads(common.read_text(encoding="utf-8"))["$defs"]
        if defs["id"]["pattern"] != ID_PATTERN:
            fails.append("common.schema.json $defs/id pattern differs from ID_PATTERN")
        hex_re = _py_pattern(defs["hex_u64"]["pattern"])
        for s, ok in (("0x00000000000000FF", True), ("0x0123456789ABCDEF", True), ("0xff", True), ("ff", False),
                      ("0x", False), ("0x1G", False), ("0x10000000000000000", False), ("0X1", False),
                      ("0xFF\n", False)):
            if bool(hex_re.search(s)) != ok:
                fails.append(f"hex_u64 pattern on {s!r}: expected {'accept' if ok else 'reject'}")
    except (OSError, KeyError, ValueError) as e:
        fails.append(f"cannot read {common}: {e}")
    return fails


def run_case(case: pathlib.Path, workspace: pathlib.Path) -> tuple[bool, str]:
    exp = json.loads((case / "expect.json").read_text(encoding="utf-8"))
    mode = exp.get("namespaces", "auto")
    own_file = case / NAMESPACES_REL
    if mode == "none":
        ns = case / "no_such_namespaces.json"
    elif mode == "case" or (mode == "auto" and own_file.is_file()):
        ns = own_file
    else:
        ns = workspace / NAMESPACES_REL
    dirs = [case / "schemas", FIXTURES / "schemas", workspace / "schemas"]
    report, stats = validate(case, dirs, ns, tuple(exp.get("allow_missing_prefix", [])))
    got_e = collections.Counter(f.code for f in report.errors)
    got_w = collections.Counter(f.code for f in report.warnings)
    want_e = collections.Counter(exp.get("errors", {}))
    want_w = collections.Counter(exp.get("warnings", {}))
    if got_e == want_e and got_w == want_w:
        return True, f"ok   {case.name}: {dict(sorted(got_e.items())) or 'no errors'}" + \
            (f", warnings {dict(sorted(got_w.items()))}" if got_w else "")
    lines = [f"FAIL {case.name}: expected errors {dict(want_e)} warnings {dict(want_w)}, "
             f"got errors {dict(got_e)} warnings {dict(got_w)}"]
    lines += ["       " + f.line() for f in report.items]
    return False, "\n".join(lines)


def self_test(workspace: pathlib.Path) -> int:
    fails = unit_checks(workspace)
    for f in fails:
        print(f"FAIL unit: {f}")
    if not fails:
        print("ok   unit: fnv1a32 vectors, collision pairs, ID grammar, token pre-pass, common patterns")
    cases_dir = FIXTURES / "cases"
    cases = sorted((d for d in cases_dir.iterdir() if d.is_dir()), key=lambda p: p.name) if cases_dir.is_dir() else []
    covered: set[str] = set()
    n_ok = 0
    for case in cases:
        ok, text = run_case(case, workspace)
        print(text)
        n_ok += ok
        exp = json.loads((case / "expect.json").read_text(encoding="utf-8"))
        covered.update(exp.get("errors", {}))
        covered.update(exp.get("warnings", {}))
    missing = sorted(set(CODES) - covered)
    if missing:
        print(f"FAIL coverage: no fixture expects {', '.join(missing)}")
    if not cases:
        print(f"FAIL fixtures: no cases under {cases_dir}")
    good = not fails and n_ok == len(cases) and not missing and cases
    print(f"self-test: {n_ok}/{len(cases)} cases passed, {len(CODES) - len(missing)}/{len(CODES)} codes covered, "
          f"{'unit checks passed' if not fails else f'{len(fails)} unit checks failed'}")
    return 0 if good else 1


# --------------------------------------------------------------------------
# CLI
# --------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="validate_content.py",
                                 description="Validate PALIMPSEST content (doc 19 §4).")
    ap.add_argument("--root", default=str(WORKSPACE), help="workspace root holding content/ and schemas/")
    ap.add_argument("--json", action="store_true", help="print one JSON document instead of text lines")
    ap.add_argument("--allow-missing-prefix", nargs="+", action="extend", default=[], metavar="PREFIX",
                    help="report missing references under these namespace prefixes (\"genre.\") as warnings")
    ap.add_argument("--namespaces", metavar="FILE", help="ownership table to use instead of <root>/" + NAMESPACES_REL)
    ap.add_argument("--schemas", nargs="+", action="extend", default=[], metavar="DIR",
                    help="extra schema directories searched after <root>/schemas")
    ap.add_argument("--self-test", action="store_true", help="run the built-in vectors and every fixture case")
    args = ap.parse_args(argv)
    if args.self_test:
        return self_test(WORKSPACE)
    for p in args.allow_missing_prefix:
        if not PREFIX_RE.fullmatch(p):
            ap.error(f"--allow-missing-prefix {p!r}: give a namespace prefix ending in '.', such as genre.")
    root = pathlib.Path(args.root).resolve()
    if not root.is_dir() or not ((root / "content").is_dir() or (root / "schemas").is_dir()):
        ap.error(f"--root {root} holds neither content/ nor schemas/")
    dirs = [root / "schemas"] + [pathlib.Path(d).resolve() for d in args.schemas]
    ns = pathlib.Path(args.namespaces).resolve() if args.namespaces else None
    report, stats = validate(root, dirs, ns, tuple(args.allow_missing_prefix))
    if args.json:
        print(json.dumps({"errors": [f.as_dict() for f in report.errors],
                          "warnings": [f.as_dict() for f in report.warnings],
                          "summary": {"files": stats.files, "records": stats.records, "aliases": stats.aliases,
                                      "references": stats.references, "resolved": stats.resolved,
                                      "schemas": stats.schemas, "errors": len(report.errors),
                                      "warnings": len(report.warnings)}}, ensure_ascii=False, indent=1))
    else:
        for f in report.items:
            print(f.line())
        print(summary_line(report, stats))
    return 1 if report.errors else 0


if __name__ == "__main__":
    sys.exit(main())
