#!/usr/bin/env python3
"""Generate content/00/meta/namespaces.json from the canon (doc 19 §1.4).

The content validator never parses Markdown. This script reads two canon
sections once and writes them as ordinary content records:

- canon §19 "Namespace ownership": one ``prefix`` record per namespace prefix,
  with the owning documents;
- canon §17.7: one ``retired`` record per ID or ID prefix named in a sentence
  that declares them "retired IDs" (other IDs in that section, such as a
  replacement, are not retired).

It also adds the ``meta.`` namespace (owner 00), which holds these records and
which canon §19 does not list yet (doc 19 Canon gaps G1).

Owner-cell rules (doc 19 §1.4), applied to the second cell of each §19 row:

1. The cell is split at ``;`` into clauses.
2. A clause that starts with a two-digit document number is a record-owner
   clause. Every two-digit number in it that is not part of a section
   reference (``§11.1``, ``§12``) is an owner of every prefix in the row,
   except that a scoping phrase adds ``NN`` only to the prefix it names: the
   parenthetical ``(<word> shared with NN)`` and the phrase
   ``core <word> values in NN`` name the prefix whose namespace is ``<word>``
   or ``<word>`` minus a final ``s`` (``(blocks shared with 03)`` adds 03 to
   ``block.``; ``with core rank values in 00`` adds 00 to ``rank.``).
3. A clause that does not start with a number names the owner of rules about
   the namespace, not of its records (``Echo acquisition in 15``); its numbers
   go to ``rule_owners``, which the validator does not use for placement.
4. A row whose owner cell says ``not content IDs`` (``loc.``) is a text-key
   namespace: ``text_keys`` is true and no content record may use it.

Usage:
    python3 tools/gen_namespaces.py [--root <workspace>] [--canon <00_CANON.md>] [--check]

The canon is found at ``--canon``, else ``<workspace>/../docs/palimpsest/spec/00_CANON.md``
(the Collab_Reposit layout), else ``<workspace>/docs/spec/00_CANON.md`` (the
standalone export layout). ``--check`` exits 1 when the committed file differs
from what the canon generates. Python 3.12 or later, standard library only.
"""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

SCHEMA_ID = "00.namespace"
OUT_REL = pathlib.Path("content") / "00" / "meta" / "namespaces.json"
DOC_NUM = r"(?<![§\d.])\b(\d\d)\b(?![\d.])"
PREFIX_RE = re.compile(r"`([a-z][a-z0-9_]*\.(?:[a-z0-9_]+\.)*)`")
TOKEN_RE = re.compile(r"`([a-z][a-z0-9_]*(?:\.[a-z0-9_]+)+\.?)`")
SCOPE_RES = (re.compile(r"\(([a-z]+) shared with (\d\d)\)"),
             re.compile(r"\bcore ([a-z]+) values in (\d\d)\b"))
META_ROW = {
    "prefix": "meta.",
    "owners": ["00"],
    "rule_owners": [],
    "owner_text": "00 (tooling metadata generated from the canon; not yet in canon §19)",
    "text_keys": False,
    "source": "doc 19 §1.4 (Canon gaps G1)",
}


def find_canon(root: pathlib.Path, explicit: str | None) -> pathlib.Path:
    """Locate 00_CANON.md in the documented order."""
    if explicit:
        path = pathlib.Path(explicit)
        if not path.is_file():
            raise SystemExit(f"gen_namespaces: --canon {explicit} is not a file")
        return path
    for cand in (root.parent / "docs" / "palimpsest" / "spec" / "00_CANON.md",
                 root / "docs" / "spec" / "00_CANON.md"):
        if cand.is_file():
            return cand
    raise SystemExit("gen_namespaces: canon not found; pass --canon <path to 00_CANON.md>")


def section(text: str, heading_re: str, level: str) -> str:
    """Body of the first heading matching heading_re, up to the next heading of the same or a higher level."""
    m = re.search(heading_re, text, re.M)
    if not m:
        raise SystemExit(f"gen_namespaces: canon heading /{heading_re}/ not found")
    rest = text[m.end():]
    stop = re.compile(r"^#{1,%d} " % len(level), re.M)
    n = stop.search(rest)
    return rest[: n.start()] if n else rest


def parse_owner_cell(cell: str, prefixes: list[str]) -> tuple[dict[str, list[str]], list[str]]:
    """Apply the owner-cell rules of the module docstring. Returns (owners per prefix, rule owners)."""
    owners_all: list[str] = []
    scoped: dict[str, list[str]] = {}
    rule_owners: list[str] = []
    for clause in (c.strip() for c in cell.split(";")):
        if re.match(r"^\d\d\b", clause):
            rest = clause
            for scope_re in SCOPE_RES:
                for word, nn in scope_re.findall(rest):
                    scoped.setdefault(word, []).append(nn)
                rest = scope_re.sub("", rest)
            owners_all += re.findall(DOC_NUM, rest)
        else:
            rule_owners += re.findall(DOC_NUM, clause)
    out: dict[str, list[str]] = {}
    for p in prefixes:
        ns = p.rstrip(".")
        extra = [nn for word, nns in scoped.items() if word in (ns, ns + "s") for nn in nns]
        out[p] = sorted(set(owners_all + extra))
    return out, sorted(set(rule_owners))


def sentences(text: str) -> list[str]:
    """Split prose at '. ' boundaries that lie outside backtick spans."""
    out, cur, in_tick = [], [], False
    for i, ch in enumerate(text):
        cur.append(ch)
        if ch == "`":
            in_tick = not in_tick
        elif ch == "." and not in_tick and (i + 1 == len(text) or text[i + 1].isspace()):
            out.append("".join(cur).strip())
            cur = []
    if "".join(cur).strip():
        out.append("".join(cur).strip())
    return out


def meta_id(prefix: str) -> str:
    return "meta.ns." + prefix.rstrip(".")


def retired_id(token: str) -> str:
    base = token.rstrip(".").replace(".", "_")
    return "meta.retired." + (base + "_all" if token.endswith(".") else base)


def generate(canon_text: str) -> dict:
    """Build the namespaces.json document from the canon text."""
    records: list[dict] = []
    sec19 = section(canon_text, r"^## 19\. Namespace ownership\s*$", "##")
    row = 0
    seen: set[str] = set()
    for line in sec19.splitlines():
        if not line.startswith("| `"):
            continue
        row += 1
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) < 2:
            raise SystemExit(f"gen_namespaces: canon §19 row {row} has fewer than 2 cells")
        prefixes = PREFIX_RE.findall(cells[0])
        if not prefixes:
            raise SystemExit(f"gen_namespaces: canon §19 row {row} names no prefix")
        owners, rule_owners = parse_owner_cell(cells[1], prefixes)
        text_keys = "not content IDs" in cells[1]
        for p in prefixes:
            if not owners[p]:
                raise SystemExit(f"gen_namespaces: canon §19 row {row}: no owner for {p}")
            if p in seen:
                raise SystemExit(f"gen_namespaces: canon §19 lists {p} twice")
            seen.add(p)
            records.append({
                "id": meta_id(p), "kind": "prefix", "prefix": p, "owners": owners[p],
                "rule_owners": rule_owners, "owner_text": cells[1], "text_keys": text_keys,
                "row": row, "source": f"canon §19 row {row}",
            })
    if row == 0:
        raise SystemExit("gen_namespaces: canon §19 has no ownership rows")
    if META_ROW["prefix"] not in seen:
        records.append({"id": meta_id(META_ROW["prefix"]), "kind": "prefix", "prefix": META_ROW["prefix"],
                        "owners": META_ROW["owners"], "rule_owners": META_ROW["rule_owners"],
                        "owner_text": META_ROW["owner_text"], "text_keys": META_ROW["text_keys"],
                        "row": row + 1, "source": META_ROW["source"]})
    sec177 = section(canon_text, r"^### 17\.7 .*$", "###")
    retired = [t for sent in sentences(sec177) if "retired IDs" in sent for t in TOKEN_RE.findall(sent)]
    if not retired:
        raise SystemExit("gen_namespaces: canon §17.7 declares no retired IDs")
    for tok in dict.fromkeys(retired):
        records.append({"id": retired_id(tok), "kind": "retired", "retired": tok, "source": "canon §17.7"})
    records.sort(key=lambda r: r["id"].encode())
    return {"schema": SCHEMA_ID, "records": records}


def render(doc: dict) -> str:
    """Deterministic text: one record per line, keys in a fixed order, UTF-8, LF."""
    lines = ["{", f'  "schema": {json.dumps(doc["schema"])},', '  "records": [']
    recs = doc["records"]
    for i, r in enumerate(recs):
        body = json.dumps(r, ensure_ascii=False, separators=(", ", ": "))
        lines.append(f"    {body}{',' if i + 1 < len(recs) else ''}")
    lines += ["  ]", "}", ""]
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=str(pathlib.Path(__file__).resolve().parent.parent),
                    help="workspace root (default: the directory above tools/)")
    ap.add_argument("--canon", help="path to 00_CANON.md")
    ap.add_argument("--check", action="store_true", help="exit 1 if the committed file is out of date")
    args = ap.parse_args(argv)
    root = pathlib.Path(args.root).resolve()
    canon = find_canon(root, args.canon)
    text = render(generate(canon.read_text(encoding="utf-8")))
    out = root / OUT_REL
    if args.check:
        current = out.read_text(encoding="utf-8") if out.is_file() else None
        if current != text:
            print(f"gen_namespaces: {OUT_REL.as_posix()} is out of date with {canon}; "
                  f"run python3 tools/gen_namespaces.py", file=sys.stderr)
            return 1
        print(f"gen_namespaces: {OUT_REL.as_posix()} matches {canon.name}")
        return 0
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8", newline="\n")
    n = text.count('"kind": "prefix"'), text.count('"kind": "retired"')
    print(f"gen_namespaces: wrote {OUT_REL.as_posix()} ({n[0]} prefixes, {n[1]} retired) from {canon}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
