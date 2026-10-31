#!/usr/bin/env python3
"""One-off OPE-450 backfill. Preview by default; pass --write to update TOML files.

Requires Python 3.11+. Relationships are explicitly curated below, never inferred
by the importer or API. Existing conflicting assignments stop the whole run.
After applying, run `import prepare --check` and re-import the competition files.
"""

import argparse
from pathlib import Path
import re
import tomllib


def annual(pattern, years):
    return [pattern.format(year=year) for year in years]


# Organizer prefixes make these identifiers unique across the archive. Missing
# years are intentional: only editions already present in OSL are listed.
SERIES = {
    "4lift": annual("4lift-{year}", (2024, 2025)),
    "ace-squad-ace-cup": annual("ace-cup-{year}", (2025, 2026)),
    "bodystrong-nationals": [
        "bodystrong-uk-nationals-2024",
        "bodystrong-uk-ire-nationals-2025",
        "bodystrong-uk-ire-nationals-2026",
    ],
    "dcsv-german-championship": annual("dcsv-german-championship-{year}", (2022, 2023, 2024, 2026)),
    "finalrep-worlds": annual("worlds-{year}", (2023, 2025)),
    "finalrep-euros": annual("euros-{year}", (2024, 2026)),
    "finalrep-german-open": annual("german-open-{year}", (2024, 2025)),
    "finalrep-portugal-open": ["portuguese-open-2025", "portugal-open-2026"],
    # CNSL and Elite remain separate pending confirmation of their relationship.
    "fnsl-cnsl": annual("fnsl-cnsl-{year}", (2021, 2022, 2023, 2024, 2025)),
    "fnsl-elite": annual("fnsl-elite-{year}", (2026, 2027)),
    "fnsl-dips-only": annual("fnsl-dip-only-{year}", (2022, 2023, 2024, 2025)) + ["fnsl-only-dips-2027"],
    "fnsl-pull-up-only": annual("fnsl-pull-only-{year}", (2022, 2023, 2024, 2025)) + ["fnsl-only-pull-up-2027"],
    "fnsl-muscle-up-only": annual("fnsl-muscle-up-only-{year}", (2023, 2024, 2025)) + ["fnsl-only-muscle-up-2027"],
    "fnsl-squat-only": ["fnsl-squat-only-2025", "fnsl-only-squat-2027"],
    "fnsl-premiers-pas": [
        "fnsl-premiers-pas-2023", "fnsl-premiers-pas-2024",
        "fnsl-premier-pas-2026", "fnsl-premiers-pas-2027",
    ],
    "fnsl-interregionale-grand-ouest": annual("fnsl-interregionale-grand-ouest-{year}", (2023, 2024, 2026, 2027)),
    "fnsl-interregionale-centre": annual("fnsl-interregionale-centre-{year}", (2026, 2027)),
    "fnsl-interregionale-sud-est": ["fnsl-inter-regions-sud-est-2025"] + annual("fnsl-interregionale-sud-est-{year}", (2026, 2027)),
    "fnsl-interregionale-nord-est": ["fnsl-inter-regions-nord-est-2026", "fnsl-interregionale-nord-est-2027"],
    "fnsl-interregionale-sud-ouest": annual("fnsl-interregionale-sud-ouest-{year}", (2026, 2027)),
    "fnsl-regionale-pays-de-la-loire": [
        "fnsl-pays-de-la-loire-2025", "fnsl-pays-de-loire-2026",
        "fnsl-regionale-pays-de-la-loire-2027",
    ],
    "fnsl-regionale-hauts-de-france": ["fnsl-hauts-de-france-2025", "fnsl-regionale-hauts-de-france-2027"],
    "munich-underground": [
        "munich-underground-2024", "munich-underground-reloaded-2024",
        "munich-underground-revolutions-2025", "munich-underground-resurrections-2025",
        "munich-underground-homecoming-2026",
    ],
    "nca-dutch-nationals": annual("nca-dutch-streetlifting-nationals-{year}", (2025, 2026)),
    "raw-workout-cwwb": annual("cwwb-{year}", (2015, 2016, 2017, 2019)),
    "sli-italian-open": annual("sli-italian-open-{year}", (2024, 2025)),
    "street-workout-slovenija-balkan-open": annual("balkan-open-championship-{year}", (2024, 2025)),
    "streetlifting-australia-national-finals": annual("australian-national-finals-{year}", (2025, 2026)),
    "streetlifting-australia-open": [
        "australian-open-2025-event-1", "australian-open-2026-event-2",
        "australian-open-2026-event-3", "australian-open-2026-event-4",
    ],
    "streetlifting-belgium-championship": annual("belgian-streetlifting-championship-{year}", (2025, 2026)),
    "streetlifting-nordic": annual("streetlifting-nordic-{year}", (2024, 2025, 2026)),
    "streetlifting-polska-max-weighted-nationals": [
        "i-max-weighted-pull-up-nationals-2016", "ii-max-weighted-pull-up-nationals-2017",
        "iii-mp-max-weighted-pull-up-2018", "iv-mp-max-weighted-chin-up-2019",
        "v-max-weighted-chin-up-nationals-2020", "vi-max-weighted-chin-up-nationals-2021",
        "vii-max-weighted-chin-up-nationals-2022", "viii-max-weighted-chin-up-nationals-2023",
    ],
    # Łomża has separate pull-up, muscle-up and Classic contests.
    "streetlifting-polska-lomza-street-power-pull-up": ["lomza-street-power-2018",
        "lomza-street-power-viii-2025", "lomza-street-power-ix-2026",
    ],
    "streetlifting-polska-lomza-street-power-classic": annual("lomza-street-power-{year}", (2019, 2021)),
    "streetlifting-polska-lomza-street-power-muscle-up": [
        "lomza-street-power-2024", "lomza-street-power-viii-2025-muscleup",
    ],
    "streetlifting-polska-glogow-nationals": annual("nationals-streetlifting-glogow-{year}", (2018, 2019, 2020, 2021, 2022)),
    "streetlifting-polska-na-wygodzie": annual("street-lifting-na-wygodzie-{year}", (2025, 2026)),
    "svenska-calisthenicsforbundet-nationals": annual("swedish-streetlifting-nationals-{year}", (2023, 2024, 2025, 2026)),
    "usastreetlifting-national-championships": annual("usastreetlifting-national-championships-{year}", (2024, 2025)) + ["usastreetlifting-nationals-3-2026"],
    "usastreetlifting-classic-nationals": ["usastreetlifting-classic-national-championships-2024", "usastreetlifting-classic-nationals-2025"],
    "usastreetlifting-empire-state-classic": annual("usastreetlifting-empire-state-classic-{year}", (2025, 2026)),
}

# Keep each format or single-lift contest in its own series, even when the
# organizer published them together or reused the same competition name.
for contest, years in {
    "arnold-classic": (2021, 2022, 2024),
    "gods-battles": (2021, 2022, 2023, 2024, 2025),
    "ragnarok-battles": (2022, 2023, 2024, 2025),
    "spanish-nationals": (2021, 2022, 2023, 2025),
}.items():
    for event in ("all4", "classic"):
        SERIES[f"aesl-{contest}-{event}"] = annual(f"aesl-{contest}-{event}-{{year}}", years)

for movement in ("dip", "pullup", "squat"):
    SERIES[f"streetlifting-polska-glogow-nationals-{movement}"] = annual(
        f"nationals-streetlifting-glogow-{{year}}-single-{movement}", (2021, 2022)
    )

for code, movement in (("m", "muscle-up"), ("p", "pull-up"), ("d", "dips")):
    SERIES[f"streetlifting-polska-lipka-krajenska-{movement}"] = [
        f"ii-turniej-lipka-krajenska-2023-{code}",
        f"iii-turniej-lipka-krajenska-2024-{code}",
    ]


def plan(root):
    files = {}
    for path in sorted(root.glob("*/*/*/competition.toml")):
        slug = path.parent.name
        if slug in files:
            raise ValueError(f"Duplicate competition slug: {slug}")
        files[slug] = path

    changes = []
    assigned = set()
    for series, slugs in sorted(SERIES.items()):
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", series):
            raise ValueError(f"Invalid series identifier: {series}")
        known_format = None
        for slug in slugs:
            if slug in assigned:
                raise ValueError(f"Competition assigned more than once: {slug}")
            assigned.add(slug)
            path = files[slug]
            text = path.read_text(encoding="utf-8")
            metadata = tomllib.loads(text)
            event = metadata.get("event")
            if event is not None:
                if known_format is not None and event != known_format:
                    raise ValueError(f"{series}: backfill mixes contest formats {known_format} and {event} ({slug})")
                known_format = event
            existing = metadata["competition"].get("series")
            if existing == series:
                continue
            if existing is not None:
                raise ValueError(f"{path}: existing series {existing!r} conflicts with {series!r}")
            output, count = re.subn(
                r"(?m)^(\[competition\]\nname = [^\n]+\n)",
                lambda match: match[1] + f'series = "{series}"\n',
                text,
                count=1,
            )
            if count != 1:
                raise ValueError(f"{path}: could not locate competition name")
            parsed = tomllib.loads(output)
            if parsed["competition"].pop("series") != series or parsed != metadata:
                raise ValueError(f"{path}: unexpected metadata change")
            changes.append((path, series, output))
    return changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="apply the backfill")
    mode.add_argument("--check", action="store_true", help="fail if any assignments are missing")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1] / "data" / "competitions"
    # Validate the complete plan before writing any file.
    changes = plan(root)
    for path, series, output in changes:
        print(f"{path.relative_to(root)} -> {series}")
        if args.write:
            path.write_text(output, encoding="utf-8")
    print(f"{len(changes)} {'updated' if args.write else 'pending'} competitions; {len(SERIES)} curated series")
    return 1 if args.check and changes else 0


if __name__ == "__main__":
    raise SystemExit(main())
