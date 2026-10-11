#!/usr/bin/env python3
"""Minimum core-genre count N for which the STRICT lattice rules of doc 12 §5.4/§5.5 always leave >= 3
eligible candidates, per run mode, over every reachable history. Prints the worst case for N-1.

Strict rules modeled (doc 12 §5.4): rule 2 no genre twice in the same Volume (Marginalia: same 5-Folio block);
rule 3 run cap of 3 appearances for m <= 25; rule 4 never the current genre; rule 7 the depth-25 Folio is forced
to genre.ur_text but its exit draw still needs 3 candidates (§5.5 'exactly 3 candidates every time, even before
forced Folios'). Edge weights between core genres are all >= 10 (§5.2) and freshness > 0, so every eligible
genre has positive weight. Secret Folios only remove pressure, so the worst case has none.
Symmetry: the rules never read genre identity, so states are reduced to the multiset of per-genre attributes.
"""
import sys
from itertools import combinations

def vol_std(f): return (f - 1) // 5          # Standard, Novella, Daily, Weekly, Second Edition (5 per Volume)
def vol_abr(f): return (f - 1) // 3          # Abridged
def block(f): return 1000 + (f - 26) // 5    # Marginalia blocks

MODES = {
    # name: (first draw m, last draw m, volume fn, cap applies to m <= 25, label)
    'Standard 25, draws m=2..24 (doc 12 feasibility scope)': (2, 24, vol_std, True),
    'Standard 25 (draws m=2..25)':   (2, 25, vol_std, True),
    'Standard + Marginalia (m=26..50)': (2, 50, None, True),
    'Abridged 15 (m=2..15)':         (2, 15, vol_abr, True),
    'Novella / Weekly 15 (m=2..15)': (2, 15, vol_std, True),
    'Daily 5 (m=1..5)':              (1, 5, vol_std, True),
}

def volume_of(mode, f):
    if mode.startswith('Standard + Marginalia'):
        return vol_std(f) if f <= 25 else block(f)
    return MODES[mode][2](f)

def solve(N, mode):
    lo, hi, _, _ = MODES[mode]
    marg = mode.startswith('Standard + Marginalia')
    # concrete state: history tuple of genres (ints); Folio 25 in Marginalia mode is Ur-Text (-1)
    def canon(hist, m):
        # attributes per genre for choosing Folio m
        cur = hist[-1] if hist else None
        vm = volume_of(mode, m)
        same = {hist[i - 1] for i in range(1, m) if volume_of(mode, i) == vm}
        sig = []
        for g in range(N):
            c = sum(1 for h in hist if h == g)
            sig.append((min(c, 3) if m <= 25 else 0, g in same, g == cur))
        return (m, tuple(sorted(sig)))
    def elig(hist, m):
        cur = hist[-1] if hist else None
        vm = volume_of(mode, m)
        same = {hist[i - 1] for i in range(1, m) if volume_of(mode, i) == vm}
        return [g for g in range(N) if g != cur and g not in same and (m > 25 or hist.count(g) < 3)]
    worst = None
    seen = set()
    if lo == 1:
        frontier = [()]
        m0 = 1
    else:
        frontier = [(0,)]          # Folio 1 = host Origin genre, any of the N (symmetric)
        m0 = 2
    m = m0
    while m <= hi:
        nxt = []
        for hist in frontier:
            e = elig(list(hist), m)
            if len(e) < 3:
                if worst is None or len(e) < worst[0]:
                    worst = (len(e), m, hist)
                continue  # the assert fires; this history cannot continue
            choices = e
            if marg and m == 25:
                choices = [-1]       # Folio 25 is genre.ur_text (doors replaced after the draw)
            for g in choices:
                h2 = hist + (g,)
                k = canon(list(h2), m + 1)
                if k in seen: continue
                seen.add(k); nxt.append(h2)
        frontier = nxt
        m += 1
    return worst

def solve_se(N, cap=True):
    """Second Edition: 15 Folios, 3 Volumes of 5, each Folio a pair of distinct genres. Strict rules applied to
    each component genre: no genre twice in a Volume, at most 3 appearances in the 15 Folios (cap=True), and
    never a genre of the current pair. 3 candidate pairs needed per exit, i.e. at least 3 eligible pairs."""
    def canon(hist, m):
        cur = set(hist[-1]) if hist else set()
        vm = vol_std(m)
        same = set().union(*[set(hist[i - 1]) for i in range(1, m) if vol_std(i) == vm]) if m > 1 else set()
        sig = tuple(sorted((min(sum(p.count(g) for p in hist), 3) if cap else 0, g in same, g in cur) for g in range(N)))
        return (m, sig)
    def elig(hist, m):
        cur = set(hist[-1]) if hist else set()
        vm = vol_std(m)
        same = set()
        for i in range(1, m):
            if vol_std(i) == vm: same |= set(hist[i - 1])
        ok = [g for g in range(N) if g not in cur and g not in same and (not cap or sum(p.count(g) for p in hist) < 3)]
        return list(combinations(ok, 2))
    worst = None; seen = set(); frontier = [()]
    for m in range(1, 16):
        nxt = []
        for hist in frontier:
            e = elig(list(hist), m)
            if len(e) < 3:
                if worst is None or len(e) < worst[0]: worst = (len(e), m, hist)
                continue
            for p in e:
                h2 = hist + (p,)
                k = canon(list(h2), m + 1)
                if k in seen: continue
                seen.add(k); nxt.append(h2)
        frontier = nxt
    return worst

if __name__ == '__main__':
    names = ['MF', 'GH', 'CP', 'WW', 'WX', 'SP'] + [f'G{i}' for i in range(7, 25)]
    for mode in MODES:
        res = {}
        for N in range(3, 21):
            res[N] = solve(N, mode)
            if res[N] is None: break
        nmin = min(N for N in res if res[N] is None)
        w = res[nmin - 1]
        hist = [('UT' if g == -1 else names[g]) for g in w[2]]
        print(f"{mode}: N_min = {nmin}; N-1 = {nmin-1} fails choosing Folio {w[1]} with {w[0]} eligible after {hist}")
    for cap in (True, False):
        res = {}
        for N in range(6, 17):
            res[N] = solve_se(N, cap)
            if res[N] is None: break
        nmin = min(N for N in res if res[N] is None)
        w = res[nmin - 1]
        print(f"Second Edition pairs (cap={'3 per genre' if cap else 'none'}): N_min = {nmin}; N-1 fails choosing Folio {w[1]} with {w[0]} eligible pairs after {[tuple(names[x] if x < 12 else f'G{x+1}' for x in p) for p in w[2]]}")
