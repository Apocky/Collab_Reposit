# Roadmap generator

Regenerates `spec/21_IMPLEMENTATION_ROADMAP.md` tables, critical path, schedule and Mermaid diagram from `wps.py` (work packages) and `template.md`.

```sh
python3 gen.py template.md ../../spec/21_IMPLEMENTATION_ROADMAP.md
python3 checks.py   # schedule and lattice feasibility checks
```

`headings.json` caches chapter section titles for citation checks; refresh it when chapters renumber.
