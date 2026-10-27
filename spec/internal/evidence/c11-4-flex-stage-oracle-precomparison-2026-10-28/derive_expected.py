#!/usr/bin/env python3
"""CSS Grid §11.7 definite fr-space oracle using exact rational arithmetic."""

from __future__ import annotations

from fractions import Fraction
import json
from pathlib import Path


CASES = [
    {
        "id": "weighted-columns",
        "axis": "columns",
        "available_size_px": 200,
        "opposite_size_px": 120,
        "gap_px": 10,
        "tracks": [
            {"css": "minmax(0px, 1fr)", "base_size_px": 0, "flex_factor": "1"},
            {"css": "minmax(0px, 2fr)", "base_size_px": 0, "flex_factor": "2"},
        ],
    },
    {
        "id": "frozen-row",
        "axis": "rows",
        "available_size_px": 200,
        "opposite_size_px": 120,
        "gap_px": 10,
        "tracks": [
            {"css": "minmax(80px, 1fr)", "base_size_px": 80, "flex_factor": "1"},
            {"css": "minmax(0px, 2fr)", "base_size_px": 0, "flex_factor": "2"},
        ],
    },
    {
        "id": "subunit-columns",
        "axis": "columns",
        "available_size_px": 200,
        "opposite_size_px": 120,
        "gap_px": 10,
        "tracks": [
            {"css": "minmax(0px, 0.5fr)", "base_size_px": 0, "flex_factor": "1/2"},
            {"css": "20px", "base_size_px": 20, "flex_factor": None},
        ],
    },
    {
        "id": "zero-fr-row",
        "axis": "rows",
        "available_size_px": 200,
        "opposite_size_px": 120,
        "gap_px": 10,
        "tracks": [
            {"css": "minmax(24px, 0fr)", "base_size_px": 24, "flex_factor": "0"},
            {"css": "minmax(0px, 1fr)", "base_size_px": 0, "flex_factor": "1"},
        ],
    },
]


def fraction(value: str | int) -> Fraction:
    return Fraction(value) if isinstance(value, int) else Fraction(value)


def expand_flexible_tracks(case: dict) -> tuple[list[Fraction], list[Fraction], list[int]]:
    tracks = case["tracks"]
    available = Fraction(case["available_size_px"] - case["gap_px"])
    frozen = {index for index, track in enumerate(tracks) if track["flex_factor"] is None}
    fractions = []

    while True:
        leftover = available - sum(fraction(tracks[index]["base_size_px"]) for index in frozen)
        flex_sum = sum(
            (fraction(track["flex_factor"]) for index, track in enumerate(tracks) if index not in frozen),
            Fraction(0),
        )
        fr_size = leftover / max(Fraction(1), flex_sum)
        fractions.append(fr_size)
        newly_frozen = {
            index
            for index, track in enumerate(tracks)
            if index not in frozen
            and fraction(track["flex_factor"]) * fr_size < fraction(track["base_size_px"])
        }
        if not newly_frozen:
            sizes = [
                fraction(track["base_size_px"])
                if index in frozen
                else max(
                    fraction(track["base_size_px"]),
                    fraction(track["flex_factor"]) * fr_size,
                )
                for index, track in enumerate(tracks)
            ]
            return sizes, fractions, sorted(frozen)
        frozen.update(newly_frozen)


def main() -> None:
    output = {
        "standard": "CSS Grid Layout Module Level 1, 2025 CRD, §§11.7–11.7.1",
        "standard_url": "https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks",
        "stage": "ExpandFlexibleTracks",
        "method": "exact Fraction implementation of the §11.7 definite-space flex fraction and freeze loop",
        "cases": [],
    }
    for case in CASES:
        sizes, fractions, frozen = expand_flexible_tracks(case)
        leftover_after = Fraction(case["available_size_px"] - case["gap_px"]) - sum(sizes)
        output["cases"].append(
            {
                **case,
                "fr_fractions_px": [str(value) for value in fractions],
                "inflexible_track_indices": frozen,
                "expected_track_bases_px": [str(value) for value in sizes],
                "remaining_space_px": str(leftover_after),
            }
        )

    path = Path(__file__).with_name("expected.json")
    path.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
