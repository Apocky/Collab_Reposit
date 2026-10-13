#!/usr/bin/env python3
"""Generate the WP tables, critical path, schedule check, coverage and Mermaid for doc 21.

Usage: gen.py <template.md> <out.md>
"""
import sys, json, re, datetime as dt, pathlib, collections
HERE = pathlib.Path(__file__).parent
sys.path.insert(0, str(HERE))
from wps import WPS, MILESTONES, RETIRED

SPEC = HERE.parents[1] / 'spec'
HEAD = json.load(open(HERE / 'headings.json', encoding='utf-8'))
BYID = {w['id']: w for w in WPS}
EA_GATE = 102
MS_ORDER = [m for m, _, _ in MILESTONES]
LANES = 8
START = dt.date(2026, 10, 12)

def wid(n): return f"WP-{n:03d}"

def ranges(nums):
    nums = sorted(set(nums)); out = []; i = 0
    while i < len(nums):
        j = i
        while j + 1 < len(nums) and nums[j + 1] == nums[j] + 1: j += 1
        out.append(str(nums[i]) if i == j else (f"{nums[i]}, {nums[j]}" if j == i + 1 else f"{nums[i]}–{nums[j]}"))
        i = j + 1
    return ", ".join(out)

def render_owner(owner):
    parts = []
    for doc, secs in owner:
        if doc in ("19", "20") and not secs:
            parts.append(f"{doc} (whole chapter)"); continue
        ss = []
        for s in secs:
            t = HEAD[doc].get(s)
            if t is None: raise SystemExit(f"missing heading {doc} §{s}")
            t = re.sub(r"\s*\(`[^`]*`[^)]*\)\s*$", "", t)  # drop trailing (`id`) for brevity
            t = t.replace('`', '')
            ss.append(f"§{s} {t}")
        parts.append(f"{doc} " + "; ".join(ss))
    return "<br>".join(parts)

def render_tests(t):
    parts = []
    if 'C' in t: parts.append("C " + ranges(t['C']))
    for k in sorted(k for k in t if k.isdigit()):
        parts.append(f"{k}: " + ranges(t[k]))
    if 'P' in t: parts.append(", ".join(f"`pillar.{p}`" for p in t['P']))
    if 'SM' in t: parts.append(f"SM-{min(t['SM']):02d}–SM-{max(t['SM']):02d}")
    if 'X' in t: parts.extend(t['X'])
    return "; ".join(parts)

def table(rows):
    hdr = "| WP | Title | Ms | Owner chapter and sections | Crate / module | Depends on | Deliverables | Acceptance tests | Size |\n|---|---|---|---|---|---|---|---|---|\n"
    out = []
    for w in rows:
        deps = ", ".join(wid(d) for d in sorted(w['deps'])) or "—"
        out.append(f"| {wid(w['id'])} | {w['title']} | {w['ms']} | {render_owner(w['owner'])} | {w['crate']} | {deps} | {w['deliv']} | {render_tests(w['tests'])} | {w['size']} |")
    return hdr + "\n".join(out)

# ---- checks ----
for w in WPS:
    for d in w['deps']:
        assert d in BYID, (w['id'], d)
        assert d < w['id'], ("not topological", w['id'], d)

# ---- longest path (critical path) ----
def longest_to(target):
    memo = {}
    def f(n):
        if n in memo: return memo[n]
        w = BYID[n]
        best = (0, [])
        for d in w['deps']:
            c = f(d)
            if c[0] > best[0] or (c[0] == best[0] and best[1] and c[1] < best[1]): best = c
        memo[n] = (best[0] + w['size'], best[1] + [n])
        return memo[n]
    return f(target)

# ---- working-day calendar ----
def add_wd(d, n):
    # d is a working day index 0 = START; returns date of working day n
    day = START; k = 0
    while k < n:
        day += dt.timedelta(days=1)
        if day.weekday() < 5: k += 1
    return day
def wd_between(a, b):
    k = 0; day = a
    while day < b:
        day += dt.timedelta(days=1)
        if day.weekday() < 5: k += 1
    return k

def asap():
    fin = {}
    for w in sorted(WPS, key=lambda x: x['id']):
        st = max([fin[d] for d in w['deps']], default=0)
        fin[w['id']] = st + w['size']
    return fin

def lane_schedule(ids):
    # list scheduling with LANES lanes, priority = longest remaining path to EA gate (or any sink)
    succ = collections.defaultdict(list)
    for w in WPS:
        for d in w['deps']: succ[d].append(w['id'])
    tail = {}
    for w in sorted(WPS, key=lambda x: -x['id']):
        tail[w['id']] = w['size'] + max([tail[s] for s in succ[w['id']]], default=0)
    done = {}; running = []; t = 0; pending = set(ids)
    lanes_free = LANES
    while pending or running:
        ready = [i for i in pending if all(d in done for d in BYID[i]['deps']) and all(done[d] <= t for d in BYID[i]['deps'])]
        ready.sort(key=lambda i: (MS_ORDER.index(BYID[i]['ms']), -tail[i], i))
        while lanes_free and ready:
            i = ready.pop(0); pending.discard(i); running.append((t + BYID[i]['size'], i)); lanes_free -= 1
        running.sort()
        if not running: raise SystemExit("deadlock")
        t = running[0][0]
        while running and running[0][0] == t:
            _, i = running.pop(0); done[i] = t; lanes_free += 1
    return done

def main():
    tpl, out = sys.argv[1], sys.argv[2]
    text = open(tpl, encoding='utf-8').read()
    ea = [w for w in WPS if w['tier'] == 'EA']
    alls = WPS
    later = [w for w in WPS if w['tier'] != 'EA']
    text = text.replace('{{WP_TABLE_EA}}', table(ea)).replace('{{WP_TABLE_LATER}}', table(later))
    L, path = longest_to(EA_GATE)
    text = text.replace('{{CRIT_PATH}}', " → ".join(f"{wid(p)} ({BYID[p]['size']})" for p in path))
    text = text.replace('{{CRIT_LEN}}', str(L)).replace('{{CRIT_N}}', str(len(path)))
    fin = asap()
    succ = collections.defaultdict(list)
    for w in WPS:
        for d in w['deps']: succ[d].append(w['id'])
    reach = {}
    def reaches(n):
        if n not in reach: reach[n] = n == EA_GATE or any(reaches(s) for s in succ[n])
        return reach[n]
    late = {}
    def lf(n):
        if n not in late:
            late[n] = fin[EA_GATE] if n == EA_GATE else min(lf(s) - BYID[s]['size'] for s in succ[n] if reaches(s))
        return late[n]
    near = [(lf(i) - fin[i], i) for i in BYID if reaches(i) and i not in path and lf(i) - fin[i] <= 10]
    near.sort()
    packs = [i for i in BYID if BYID[i]['tier'] == 'EA' and BYID[i]['title'].startswith(('Genre pack', 'Assets:'))]
    text = text.replace('{{PACK_SLACK}}', str(min(lf(i) - fin[i] for i in packs)))
    text = text.replace('{{NEAR_CRIT}}', "; ".join(f"{wid(i)} {BYID[i]['title']} ({s})" for s, i in near))
    ea_ids = [w['id'] for w in ea]
    ls = lane_schedule([w['id'] for w in WPS if w['ms'] != 'Post' and w['ms'] not in ('M12','M13')])
    tot_ea = sum(w['size'] for w in ea); tot_all = sum(w['size'] for w in WPS)
    text = text.replace('{{EA_DAYS}}', str(tot_ea)).replace('{{ALL_DAYS}}', str(tot_all))
    text = text.replace('{{N_EA}}', str(len(ea))).replace('{{N_ALL}}', str(len(WPS))).replace('{{N_LATER}}', str(len(later)))
    # per-milestone schedule check
    msdate = {m: d for m, _, d in MILESTONES}
    rows = ["| Milestone | Latest ASAP finish of its WPs (working day) | Latest finish with 8 lanes (working day) |", "|---|---|---|"]
    for m, name, d in MILESTONES[:8]:
        ids = [w['id'] for w in WPS if w['ms'] == m]
        if not ids: continue
        a = max(fin[i] for i in ids); b = max(ls[i] for i in ids)
        target = dt.date.fromisoformat(d)
        tw = wd_between(START, target)
        rows.append(f"| {m} {name} | day {a} | day {b} |")
    text = text.replace('{{SCHED_TABLE}}', "\n".join(rows))
    text = text.replace('{{EA_GATE_DAY}}', str(ls[EA_GATE]))
    text = text.replace('{{CRIT_END}}', str(L))
    # coverage
    cov = collections.defaultdict(set)
    for w in WPS:
        for k, v in w['tests'].items():
            if k.isdigit() or k == 'C':
                cov[k].update(v)
    report = []
    for f in sorted(SPEC.glob('[0-1][0-9]_*.md')):
        n = f.name[:2]
        if f.name.startswith('16_NETCODE') or n == '00' or int(n) > 18: continue
        sec = f.read_text(encoding='utf-8').split('## Acceptance tests', 1)[1].split('\n## ', 1)[0]
        nums = [int(m.group(1)) for m in re.finditer(r'^(\d+)\.\s', sec, re.M)]
        missing = sorted(set(nums) - cov[n] - set(RETIRED.get(n, [])))
        extra = sorted(cov[n] - set(nums))
        report.append((n, len(nums), missing, extra))
    canon = (SPEC / '00_CANON.md').read_text(encoding='utf-8').split('## 21. Acceptance tests', 1)[1].split('\n## ', 1)[0]
    cn = [int(m.group(1)) for m in re.finditer(r'^(\d+)\.\s', canon, re.M)]
    report.append(('00', len(cn), sorted(set(cn) - cov['C']), sorted(cov['C'] - set(cn))))
    cov_rows = ["| Chapter | Tests on disk | Covered by WPs | Retired |", "|---|---|---|---|"]
    for n, cnt, miss, extra in sorted(report):
        if miss or extra: print("COVERAGE", n, "missing", miss, "extra", extra, file=sys.stderr)
        ret = ", ".join(str(x) for x in RETIRED.get(n, [])) or "—"
        label = "Canon §21" if n == '00' else f"doc {n}"
        cov_rows.append(f"| {label} | {cnt} | {cnt - len(RETIRED.get(n, []))} | {ret} |")
    text = text.replace('{{COVERAGE}}', "\n".join(cov_rows))
    # mermaid with transitive reduction
    anc = {}
    for w in sorted(WPS, key=lambda x: x['id']):
        s = set()
        for d in w['deps']: s |= {d} | anc[d]
        anc[w['id']] = s
    edges = []
    for w in WPS:
        for d in w['deps']:
            if not any(d in anc[o] for o in w['deps'] if o != d):
                edges.append((d, w['id']))
    crit = set(path)
    crit_edges = set(zip(path, path[1:]))
    lines = ["```mermaid", "flowchart LR"]
    for m, name, d in MILESTONES:
        ids = [w['id'] for w in WPS if w['ms'] == m]
        if not ids: continue
        lines.append(f'  subgraph {m.replace("Post", "MP")}["{m} {name}"]')
        for i in ids:
            lines.append(f'    W{i:03d}["{wid(i)} {BYID[i]["title"].replace(chr(34), "").replace(chr(96), "")}"]')
        lines.append("  end")
    for a, b in sorted(edges, key=lambda e: (e[1], e[0])):
        lines.append(f"  W{a:03d} --> W{b:03d}")
    lines.append("  classDef crit stroke-width:4px")
    lines.append("  class " + ",".join(f"W{i:03d}" for i in path) + " crit")
    lines.append("```")
    text = text.replace('{{MERMAID}}', "\n".join(lines))
    text = text.replace('{{N_EDGES}}', str(len(edges)))
    open(out, 'w', encoding='utf-8').write(text)
    print("crit", L, [wid(p) for p in path], "working day", L, file=sys.stderr)
    print("EA days", tot_ea, "all", tot_all, "WPs", len(WPS), file=sys.stderr)

if __name__ == '__main__':
    main()
