#!/usr/bin/env python3
"""Turn a FinalRep browser capture into canonical entries.csv rows.

A HAR saved from app.final-rep.com holds one `/events/{id}/statistics` response per
event visited. Each of those is a whole meet: groups, athletes, every attempt. This
reads them out and writes the competition directory the importer expects.

    python3 scripts/har-to-entries.py capture.har --list
    python3 scripts/har-to-entries.py capture.har --event "Costa Rican" --out backend/data/competitions/finalrep/2026/costa-rican-open-2026

Nothing here decides anything a human should: bodyweight is absent from the source,
countries that are not ISO alpha-2 are left empty, and every name that could split more
than one way is reported for review. Run `import fmt` afterwards to fill the Best*
columns. See docs/finalrep-api.md.
"""

import argparse
import csv
import json
import re
import sys
from pathlib import Path

# The API keys attempts by the movement's display name.
# api display name -> (entries.csv column, event letter, prose name for StatusReason)
MOVEMENTS = {
    "Muscle Up": ("MuscleUp", "M", "muscle-up"),
    "Pull/Chin Up": ("PullUp", "P", "pull-up"),
    "Dip": ("Dips", "D", "dips"),
    "Squat": ("Squat", "S", "squat"),
}
EVENT_ORDER = "MPDS"

CSV_HEADER = (
    ["Sex", "WeightClassKg", "FirstName", "LastName", "Disambiguation", "Country",
     "BodyweightKg", "Ris", "Status", "StatusReason"]
    # Best* is derived by `import fmt`; written empty so the two can never disagree.
    + [f"{c}{i}Kg" for c in ("MuscleUp", "PullUp", "Dips", "Squat")
       for i in ("1", "2", "3")]
)
for _column in ("MuscleUp", "PullUp", "Dips", "Squat"):
    _at = CSV_HEADER.index(f"{_column}3Kg")
    CSV_HEADER.insert(_at + 1, f"Best{_column}Kg")

GROUP_RE = re.compile(r"^\s*(male|female|mixed)\s*([+-])\s*(\d+(?:\.\d+)?)\s*kg\s*$", re.I)
SEX = {"male": "M", "female": "F", "mixed": "MX"}


def statistics_payloads(har_path):
    """Yield (event_id, payload) for every statistics response in the capture."""
    har = json.loads(Path(har_path).read_text())
    for entry in har.get("log", {}).get("entries", []):
        req = entry.get("request", {})
        if req.get("method") != "GET" or "/statistics" not in req.get("url", ""):
            continue
        body = entry.get("response", {}).get("content", {}).get("text")
        if not body:
            continue
        try:
            payload = json.loads(body)["_embedded"]
        except (ValueError, KeyError):
            continue
        if "group_stats" not in payload:
            continue
        event_id = req["url"].split("/events/")[1].split("/")[0]
        yield event_id, payload


def parse_group(name):
    """'Male -66kg' -> ('M', '66'); 'Male +101kg' -> ('M', '101+')."""
    m = GROUP_RE.match(name)
    if not m:
        raise ValueError(f"cannot read sex and weight class from group name {name!r}")
    sex_word, sign, bound = m.group(1).lower(), m.group(2), m.group(3)
    bound = bound.rstrip("0").rstrip(".") if "." in bound else bound
    return SEX[sex_word], (f"{bound}+" if sign == "+" else bound)


def split_name(full):
    """Best-effort first/last split. Ambiguous ones are reported, never silently trusted."""
    parts = full.split()
    if not parts:
        return "", "", True
    if len(parts) == 1:
        return "", parts[0], False
    # An initial belongs with the given names: 'Lianneth M. Ocanto'.
    first = [parts[0]]
    rest = parts[1:]
    while len(rest) > 1 and len(rest[0]) <= 2 and rest[0].endswith("."):
        first.append(rest.pop(0))
    if len(rest) > 2:
        first += rest[:-2]
        rest = rest[-2:]
    ambiguous = len(parts) > 2
    return " ".join(first), " ".join(rest), ambiguous


def cell(attempt):
    weight = attempt["weight"]
    text = f"{weight:g}"
    return text if attempt["success"] else text + "x"


def contested_movements(payload):
    """Movements where at least one athlete lifted a real weight.

    A movement the meet did not run still comes back with three 0 kg attempts per
    athlete, all failed. Old Dominion Classic 2026 is scored that way on the squat.
    Those are placeholders, not misses, so the movement is left out of `event`
    entirely rather than importing a competition nobody squatted in.
    """
    contested = set()
    for group_stats in payload["group_stats"]:
        for stat in group_stats["athlete_stats"]:
            for api_name, attempts in stat.get("attempts", {}).items():
                if any(a["weight"] for a in attempts):
                    contested.add(api_name)
    return contested


def build_rows(payload, default_country=None):
    rows, notes, movements_seen = [], [], set()
    contested = contested_movements(payload)
    for skipped in sorted(set(payload["group_stats"][0]["athlete_stats"][0]["attempts"]) - contested):
        notes.append(f"event   {skipped!r} was not contested, every attempt is a 0 kg placeholder")
    for group_stats in payload["group_stats"]:
        sex, weight_class = parse_group(group_stats["group"]["name"])
        for stat in group_stats["athlete_stats"]:
            user = stat["user"]
            name = user.get("name", "").strip()
            first, last, ambiguous = split_name(name)
            if ambiguous:
                notes.append(f"name    {name!r} split as first={first!r} last={last!r}")

            country = (user.get("country") or "").strip()
            if re.fullmatch(r"[A-Za-z]{2}", country):
                country = country.upper()
            else:
                notes.append(
                    f"country {name!r} has country={country!r}, not an ISO alpha-2 code"
                    + (f", defaulted to {default_country}" if default_country else ", left empty")
                )
                country = default_country or ""

            row = {c: "" for c in CSV_HEADER}
            row.update({
                "Sex": sex, "WeightClassKg": weight_class,
                "FirstName": first, "LastName": last, "Country": country,
                "Ris": f"{stat['ris']:.2f}",
            })

            bombed, lifted = [], False
            real = {k: v for k, v in stat.get("attempts", {}).items() if k in contested}
            if real and not any(a["weight"] for v in real.values() for a in v):
                # Entered but never lifted: 0 kg everywhere while the rest of the
                # field lifted real weights. That is a no_show, not a bombed meet.
                row["Status"] = "no_show"
                notes.append(f"no_show {name!r} has only 0 kg placeholder attempts")
                rows.append(row)
                continue
            for api_name, attempts in real.items():
                if api_name not in MOVEMENTS:
                    raise ValueError(
                        f"unknown movement {api_name!r}; known: {sorted(MOVEMENTS)}"
                    )
                column, letter, prose = MOVEMENTS[api_name]
                movements_seen.add(letter)
                ordered = sorted(attempts, key=lambda a: a["attempt"])
                if len(ordered) > 3:
                    raise ValueError(f"{name}: {api_name} has {len(ordered)} attempts")
                for i, attempt in enumerate(ordered, 1):
                    row[f"{column}{i}Kg"] = cell(attempt)
                if any(a["success"] for a in ordered):
                    lifted = True
                elif ordered:
                    bombed.append(prose)

            # A bombed movement ends the competition; an entrant who never lifted at
            # all is a no_show, which is a different thing. See the extract-competition
            # skill for why these are not interchangeable.
            if not lifted and not bombed:
                row["Status"] = "no_show"
            elif bombed:
                row["Status"] = "disqualified"
                joined = " and ".join(bombed)
                row["StatusReason"] = f"Bombed the {joined}"
            else:
                row["Status"] = "competed"

            if stat.get("disqualified") and row["Status"] == "competed":
                notes.append(f"status  {name!r} is flagged disqualified but made every movement")
            if not stat.get("disqualified") and row["Status"] == "disqualified":
                notes.append(f"status  {name!r} bombed {bombed} but is not flagged disqualified")

            rows.append(row)
    event_letters = "".join(c for c in EVENT_ORDER if c in movements_seen)
    return rows, notes, event_letters


def write_competition(out_dir, rows, event_letters, event_name):
    out = Path(out_dir)
    out.mkdir(parents=True, exist_ok=True)

    entries = out / "entries.csv"
    with entries.open("w", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=CSV_HEADER)
        writer.writeheader()
        writer.writerows(rows)

    toml = out / "competition.toml"
    if not toml.exists():
        toml.write_text(
            f'event = "{event_letters}"\n'
            'sources = [\n'
            '  "FinalRep app, /events-api/v1/events/{id}/statistics",\n'
            ']\n\n'
            "[competition]\n"
            f'name = "{event_name}"\n'
            '# start_date, end_date, city, region and country are not in the API.\n'
            '# Fill them from the organiser or the FinalRep recap before importing.\n'
            'start_date = ""\n'
            'end_date = ""\n'
            'city = ""\n'
            'country = ""\n'
            'status = "completed"\n\n'
            "[federation]\n"
            'name = "FinalRep"\n'
        )
    return entries, toml


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("har")
    ap.add_argument("--list", action="store_true", help="show the events in the capture")
    ap.add_argument("--event", help="event id, or part of the event name")
    ap.add_argument("--out", help="competition directory to write")
    ap.add_argument("--default-country", metavar="XX",
                    help="ISO alpha-2 to use where the source gives no usable country")
    args = ap.parse_args()

    found = list(statistics_payloads(args.har))
    if not found:
        sys.exit("no /statistics responses in this capture; was the results view opened?")

    if args.list or not args.event:
        print(f"{len(found)} event(s) in {args.har}:\n")
        for event_id, payload in found:
            athletes = sum(len(g["athlete_stats"]) for g in payload["group_stats"])
            classes = ", ".join(g["group"]["name"] for g in payload["group_stats"])
            print(f"  {payload['event_name']}")
            print(f"    id={event_id}  type={payload['event_type']}")
            print(f"    {len(payload['group_stats'])} classes, {athletes} athletes: {classes}\n")
        if not args.event:
            print("pass --event <id or name> --out <dir> to write one out")
        return

    matches = [
        (eid, p) for eid, p in found
        if eid == args.event or args.event.lower() in p["event_name"].lower()
    ]
    if not matches:
        sys.exit(f"no event matching {args.event!r}")
    if len(matches) > 1:
        sys.exit("ambiguous: " + ", ".join(p["event_name"] for _, p in matches))

    event_id, payload = matches[0]
    rows, notes, event_letters = build_rows(payload, args.default_country)
    if not args.out:
        sys.exit("--out is required to write a competition")

    entries, toml = write_competition(args.out, rows, event_letters, payload["event_name"])
    print(f"{payload['event_name']}  ({event_id})")
    print(f"  {len(rows)} rows -> {entries}")
    print(f"  event = {event_letters!r}")
    print(f"  {toml}: fill start_date, end_date, city and country before running fmt,")
    print("    they are not in the API. fmt will refuse the file until you do.")

    if notes:
        print(f"\nreview these {len(notes)} item(s) before committing:")
        for note in sorted(set(notes)):
            print(f"  {note}")
    print("\nbodyweight is not in the API, so every row is Ris-only.")
    print("next: cargo run -p osl_importer --bin import -- fmt <dir>")


if __name__ == "__main__":
    main()
