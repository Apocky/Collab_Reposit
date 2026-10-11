#!/usr/bin/env python3
"""Extra checks for doc 21: citations, cascade enumeration, Folio-1 map, pillars, ownership, hedges."""
import re, json, sys, itertools, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from wps import WPS
HEAD = json.load(open(pathlib.Path(__file__).parent / 'headings.json'))
doc = open(sys.argv[1]).read()
errs = 0
# 1. citations "doc NN §x (Title)" / "doc NN §x, Title" / "Canon §x (Title)"
norm = lambda s: re.sub(r'[`]', '', s).strip().lower()
for m in re.finditer(r'(?:doc (\d\d)|Canon) §(\d+(?:\.\d+)*[a-z]?)(?: \(([^()]*(?:\([^()]*\))?[^()]*)\)|, ([A-Z][^,;.)]*))?', doc):
    d = m.group(1) or '00'; s = m.group(2); title = m.group(3) or m.group(4)
    if d in ('19', '20'): continue
    h = HEAD.get(d, {}).get(s)
    if h is None:
        print('MISSING SECTION', m.group(0)); errs += 1; continue
    if title and not (norm(h).startswith(norm(title)) or norm(title).startswith(norm(h))):
        # titles in parentheses can also be descriptive notes; flag for review
        print('TITLE?', m.group(0), '| on disk:', h)
# 2. cascade enumeration over EA genres for Novella, Abridged, Daily
EA = ['MF', 'GH', 'CP', 'WW', 'WX', 'SP']
def vol(m, mode):
    return (m - 1) // (3 if mode == 'A' else 5)
def elig(m, hist, mode):
    cur = hist[m - 1]
    same = [hist[i] for i in range(1, m) if vol(i, mode) == vol(m, mode)]
    e0 = [g for g in EA if g != cur and g not in same and hist[1:m].count(g) < 3]
    if len(e0) >= 3: return e0, 0
    e1 = [g for g in EA if g != cur and g not in same]
    if len(e1) >= 3: return e1, 1
    prev = hist[m - 2] if m >= 3 else None
    e2 = [g for g in EA if g != cur and g != prev]
    return e2, 2
worst = 99; stages = [0, 0, 0]
from functools import lru_cache
for mode, limit in (('N', 15), ('A', 14), ('D', 5)):
    seen = set()
    frontier = set()
    for g in EA:
        cnt = tuple(1 if h == g else 0 for h in EA)
        frontier.add((2, g, None, frozenset([g]), cnt))  # about to choose Folio 2
    while frontier:
        nxt = set()
        for st in frontier:
            if st in seen: continue
            seen.add(st)
            m, cur, prev, same, cnt = st
            if m > limit: continue
            newvol = vol(m, mode) != vol(m - 1, mode)
            same_m = frozenset() if newvol else same
            e0 = [g for g in EA if g != cur and g not in same_m and cnt[EA.index(g)] < 3]
            if len(e0) >= 3: e, k = e0, 0
            else:
                e1 = [g for g in EA if g != cur and g not in same_m]
                if len(e1) >= 3: e, k = e1, 1
                else: e, k = [g for g in EA if g != cur and g != prev], 2
            stages[k] += 1; worst = min(worst, len(e))
            for g in e:
                c2 = list(cnt); c2[EA.index(g)] += 1
                nxt.add((m + 1, g, cur, same_m | {g}, tuple(c2)))
        frontier = nxt
print('cascade: min eligible', worst, 'stage counts', stages)
if worst < 3: errs += 1
e, st = elig(5, [None, 'MF', 'GH', 'CP', 'WW'], 'N'); print('example', e, st)
# 3. Folio-1 map from doc 12 §5.2
t12 = open(__import__('pathlib').Path(__file__).resolve().parents[2] / 'spec' / '12_WORLD_GENERATION_GENRES.md').read()
sec = t12.split('### 5.2 Core edge weights', 1)[1].split('###', 1)[0]
rows = [l for l in sec.splitlines() if re.match(r'^\| [A-Z]{2} \|', l)]
order = ['MF', 'GH', 'CH', 'SO', 'CP', 'PA', 'WW', 'HN', 'WX', 'SP', 'FT', 'MA']
W = {}
for l in rows:
    c = [x.strip() for x in l.strip('|').split('|')]
    W[c[0]] = {order[i]: (int(c[i + 1]) if c[i + 1] != '—' else 0) for i in range(12)}
mp = {}
for g in order:
    if g in EA: continue
    best = max(EA, key=lambda h: (W[g][h], -order.index(h)))
    mp[g] = (best, W[g][best])
print('folio1 map', mp)
# 4. pillars exactly once among EA WPs
from collections import Counter
pc = Counter(p for w in WPS if w['tier'] == 'EA' for p in w['tests'].get('P', []))
print('pillar tests', len(pc), 'max count', max(pc.values()))
if len(pc) != 18 or max(pc.values()) != 1: errs += 1
# 5. ownership coverage
own = set(d for w in WPS for d, _ in w['owner'])
miss = [f"{i:02d}" for i in range(1, 21) if f"{i:02d}" not in own]
print('chapters without WP:', miss); errs += bool(miss)
# 6. hedges and banned words
for word in ['might', 'could', 'consider', 'optionally', 'TBD', 'roughly', 'perhaps', 'maybe', 'probably', 'possibly', 'approximately', 'about']:
    n = len(re.findall(r'\b' + word + r'\b', doc, re.I))
    if n: print('HEDGE', word, n)
print('errors', errs)
