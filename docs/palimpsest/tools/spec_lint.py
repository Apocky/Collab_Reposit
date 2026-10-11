#!/usr/bin/env python3
"""PALIMPSEST spec linter.

Checks every chapter in docs/palimpsest/spec against the canon contract:
  1. required closing sections exist;
  2. every backticked content ID is defined somewhere (canon, an Exports
     section, the first cell of a definition table, or a JSON "id" field);
  3. IDs are defined only in chapters that own their namespace (canon §19);
  4. JSON code blocks parse and contain no floats (canon §1.6);
  5. hedging words are counted.
Usage: python3 spec_lint.py [--json] [--chapter NN]
"""
import json, re, sys, pathlib, collections

ROOT = pathlib.Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec"
ID_RE = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$")
TICK_RE = re.compile(r"`([^`\n]+)`")
REQUIRED = ["## Imports", "## Exports", "## Interface assumptions", "## Canon gaps",
            "## Acceptance tests", "## Invariants annex"]
RETIRED = ["cat.mount", "slot.mount", "st.mounted", "ev.mounted", "ev.dismounted", "room.no_mount",
           "skill.delving", "cat.block"]
RETIRED_PREFIX = ["arch.mount."]
HEDGES = ["might", "could", "consider", "optionally", "TBD", "roughly", "perhaps", "maybe"]
# Namespaces that look like content IDs. Others (e.g. file names, Rust paths) are ignored.
NAMESPACES = None

def chapters():
    return sorted(p for p in SPEC.glob("[0-9][0-9]_*.md"))

def chapter_num(p):
    return p.name[:2]

def load_ownership(canon_text):
    """Parse canon §19 table: namespace prefix -> set of owning chapter numbers."""
    own = {}
    sec = canon_text.split("## 19. Namespace ownership", 1)[1].split("\n## ", 1)[0]
    for line in sec.splitlines():
        if not line.startswith("| `"):
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        prefixes = re.findall(r"`([a-z_.]+)`", cells[0])
        owners = set(re.findall(r"\b(\d\d)\b", cells[1]))
        for pf in prefixes:
            own[pf] = owners
    return own

def owners_of(id_, own):
    best = None
    for pf in own:
        if id_.startswith(pf) and (best is None or len(pf) > len(best)):
            best = pf
    return (best, own.get(best, set())) if best else (None, set())

def ids_in(text):
    out = []
    for m in TICK_RE.finditer(text):
        tok = m.group(1).strip()
        if ID_RE.match(tok) and not tok.startswith("loc."):
            out.append(tok)
    return out

def defined_ids(text):
    """IDs this document defines: Exports section, first table cell, JSON "id" values."""
    d = set()
    if "## Exports" in text:
        exp = text.split("## Exports", 1)[1].split("\n## ", 1)[0]
        d.update(ids_in(exp))
    for line in text.splitlines():
        if line.startswith("|") and not line.startswith("|---"):
            first = line.strip("|").split("|")[0]
            d.update(ids_in(first))
    for m in re.finditer(r'"id"\s*:\s*"([a-z0-9_.]+)"', text):
        if ID_RE.match(m.group(1)):
            d.add(m.group(1))
    return d

def json_blocks(text):
    for m in re.finditer(r"```json\n(.*?)```", text, re.S):
        yield m.group(1)

def has_float(v):
    return isinstance(v, float) or (isinstance(v, dict) and any(has_float(x) for x in v.values())) \
        or (isinstance(v, list) and any(has_float(x) for x in v))

def main():
    as_json = "--json" in sys.argv
    only = sys.argv[sys.argv.index("--chapter") + 1] if "--chapter" in sys.argv else None
    canon = (SPEC / "00_CANON.md").read_text()
    own = load_ownership(canon)
    known_ns = set(p.split(".")[0] for p in own)
    docs = {chapter_num(p): p.read_text() for p in chapters()}
    defs = collections.defaultdict(set)  # id -> chapters
    for n, t in docs.items():
        for i in defined_ids(t) if n != "00" else ids_in(t):
            defs[i].add(n)
    all_defined = set(defs)
    report = {}
    for n, t in docs.items():
        if only and n != only:
            continue
        r = {}
        if n != "00":
            r["missing_sections"] = [s for s in REQUIRED if s not in t]
        used = [i for i in ids_in(t) if i.split(".")[0] in known_ns]
        r["undefined_ids"] = sorted(set(i for i in used if i not in all_defined and "<" not in i))
        bad_owner = []
        if n != "00":
            for i in defined_ids(t):
                pf, owners = owners_of(i, own)
                if pf and owners and n not in owners and "00" not in owners:
                    bad_owner.append(f"{i} (namespace {pf} owned by {','.join(sorted(owners))})")
        r["foreign_namespace_definitions"] = sorted(set(bad_owner))
        r["retired_ids"] = sorted(set(i for i in ids_in(t) if i in RETIRED or any(i.startswith(x) for x in RETIRED_PREFIX))) if n != "00" else []
        jerr = []
        for k, blk in enumerate(json_blocks(t)):
            try:
                v = json.loads(blk)
                if has_float(v):
                    jerr.append(f"json block {k}: contains float")
            except Exception as e:
                jerr.append(f"json block {k}: {e.__class__.__name__}: {str(e)[:80]}")
        r["json_issues"] = jerr
        words = re.findall(r"[A-Za-z]+", t)
        r["hedges"] = {h: sum(1 for w in words if w == h) for h in HEDGES if any(w == h for w in words)}
        r["words"] = len(t.split())
        report[n] = r
    dup = {i: sorted(c) for i, c in defs.items() if len(c - {"00"}) > 1}
    if as_json:
        print(json.dumps({"chapters": report, "multiply_defined": dup}, indent=1))
        return
    for n, r in report.items():
        print(f"== {n}  words={r['words']}")
        for k, v in r.items():
            if k == "words" or not v:
                continue
            if isinstance(v, list):
                print(f"  {k} ({len(v)}): {', '.join(v[:25])}{' ...' if len(v) > 25 else ''}")
            else:
                print(f"  {k}: {v}")
    print(f"== IDs defined in more than one chapter: {len(dup)}")

if __name__ == "__main__":
    main()
