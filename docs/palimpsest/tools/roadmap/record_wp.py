"""Record one finished work package's measured elapsed time in velocity.csv.

Usage: python record_wp.py WP-NNN START_UTC FINISH_UTC AGENT_DAYS "note"
  START_UTC / FINISH_UTC: ISO 8601 with Z, e.g. 2026-10-12T18:00:00Z
Calibration needs at least three rows (see gen.py).
"""
import csv, sys, pathlib, datetime as dt
LEDGER = pathlib.Path(__file__).resolve().parent / "velocity.csv"
wp, start, finish, days, note = sys.argv[1:6]
s = dt.datetime.fromisoformat(start.replace("Z", "+00:00"))
f = dt.datetime.fromisoformat(finish.replace("Z", "+00:00"))
if f <= s:
    sys.exit("finish must be after start")
float(days)
with LEDGER.open("a", newline="", encoding="utf-8") as fh:
    csv.writer(fh).writerow([wp, start, finish, days, note])
print("recorded", wp, round((f - s).total_seconds() / 3600, 2), "hours")
