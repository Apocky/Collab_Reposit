#!/usr/bin/env python3
"""Apply audit/map_*.json ID renames across every spec chapter.

Only exact tokens are rewritten: `old` inside backticks and "old" as a JSON
string value. Chains (a->b, b->c) are resolved; cycles are an error.
Usage: python3 apply_id_maps.py [--dry-run]
"""
import json, re, sys, pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
maps = {}
for p in sorted((ROOT / "audit").glob("map_*.json")):
    for old, new in json.loads(p.read_text()).items():
        if old == new:
            continue
        if old in maps and maps[old] != new:
            sys.exit(f"conflict: {old} -> {maps[old]} and {new} ({p.name})")
        maps[old] = new

def resolve(i, seen=()):
    if i in seen:
        sys.exit(f"cycle at {i}")
    return resolve(maps[i], seen + (i,)) if i in maps else i

final = {o: resolve(o) for o in maps}
dry = "--dry-run" in sys.argv
total = 0
for p in sorted((ROOT / "spec").glob("[0-9][0-9]_*.md")):
    t = p.read_text()
    n = 0
    def tick(m):
        global n
        tok = m.group(1)
        if tok in final:
            n += 1
            return f"`{final[tok]}`"
        return m.group(0)
    def quote(m):
        global n
        tok = m.group(1)
        if tok in final:
            n += 1
            return f'"{final[tok]}"'
        return m.group(0)
    t2 = re.sub(r"`([a-z][a-z0-9_.]*)`", tick, t)
    t2 = re.sub(r'"([a-z][a-z0-9_.]*)"', quote, t2)
    if n:
        print(f"{p.name}: {n} replacements")
        total += n
        if not dry:
            p.write_text(t2)
print(f"mappings={len(final)} replacements={total} dry_run={dry}")
