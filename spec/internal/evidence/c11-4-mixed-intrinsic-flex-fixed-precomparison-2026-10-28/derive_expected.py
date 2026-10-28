#!/usr/bin/env python3
"""Derive exact §11.7.1 track sizes for the four fixed precomparison scenes."""

from fractions import Fraction
import json


SCENES = (
    ("column-freeze", "columns", 300, 120, 100),
    ("column-no-freeze", "columns", 300, 120, 40),
    ("column-equal-boundary", "columns", 300, 120, 80),
    ("column-freeze-reversed", "columns", 300, 120, 100),
    ("row-freeze", "rows", 300, 120, 100),
    ("row-no-freeze", "rows", 300, 120, 40),
    ("row-equal-boundary", "rows", 300, 120, 80),
    ("row-freeze-reversed", "rows", 300, 120, 100),
)
GAP = Fraction(10)
FIXED_TRACK = Fraction(40)
FLEX_FACTORS = (Fraction(1), Fraction(2))


def rational(value: Fraction) -> dict[str, int]:
    return {"numerator": value.numerator, "denominator": value.denominator}


def integer_px(value: Fraction) -> int:
    if value.denominator != 1:
        raise ValueError(f"root-relative frame requires integer CSS px: {value}")
    return value.numerator


def derive_tracks(axis_size: int, min_content_base: int) -> list[Fraction]:
    # Two internal gaps and the fixed middle track are removed before §11.7.1
    # distributes the remaining definite free space among flexible tracks.
    flex_space = Fraction(axis_size) - 2 * GAP - FIXED_TRACK
    flex_fraction = flex_space / sum(FLEX_FACTORS)
    bases = (Fraction(min_content_base), Fraction(0))
    frozen = {
        index
        for index, (base, factor) in enumerate(zip(bases, FLEX_FACTORS))
        if base > flex_fraction * factor
    }
    if not frozen:
        return [flex_fraction * factor for factor in FLEX_FACTORS]

    frozen_space = sum((bases[index] for index in frozen), Fraction(0))
    unfrozen_factors = sum(
        (factor for index, factor in enumerate(FLEX_FACTORS) if index not in frozen),
        Fraction(0),
    )
    remaining_fraction = (flex_space - frozen_space) / unfrozen_factors
    return [
        bases[index] if index in frozen else remaining_fraction * factor
        for index, factor in enumerate(FLEX_FACTORS)
    ]


def item_frames(axis: str, tracks: list[Fraction], cross_size: int) -> dict[str, dict[str, int]]:
    first, third = tracks
    after_first = first + GAP
    after_fixed = after_first + FIXED_TRACK + GAP
    if axis == "columns":
        return {
            "a": {"x": 0, "y": 0, "width": integer_px(first), "height": cross_size},
            "b": {"x": integer_px(after_first), "y": 0, "width": 24, "height": 16},
            "c": {"x": integer_px(after_fixed), "y": 0, "width": integer_px(third), "height": cross_size},
        }
    return {
        "a": {"x": 0, "y": 0, "width": cross_size, "height": integer_px(first)},
        "b": {"x": 0, "y": integer_px(after_first), "width": 24, "height": 16},
        "c": {"x": 0, "y": integer_px(after_fixed), "width": cross_size, "height": integer_px(third)},
    }


def main() -> None:
    cases = {}
    for scene_id, axis, axis_size, cross_size, min_content_base in SCENES:
        flexible = derive_tracks(axis_size, min_content_base)
        tracks = [flexible[0], FIXED_TRACK, flexible[1]]
        cases[scene_id] = {
            "axis": axis,
            "sizingAxisPx": axis_size,
            "crossAxisPx": cross_size,
            "minContentBasePx": min_content_base,
            "flexFactors": [1, 2],
            "gapPx": 10,
            "tracksPx": [rational(value) for value in tracks],
            "remainingFreeSpacePx": rational(
                Fraction(axis_size) - 2 * GAP - sum(tracks)
            ),
            "itemFramesRootRelativePx": item_frames(axis, flexible, cross_size),
        }

    print(json.dumps({"calculation": "CSS Grid 2025 CRD §11.7.1 exact rational arithmetic", "cases": cases}, indent=2))


if __name__ == "__main__":
    main()
