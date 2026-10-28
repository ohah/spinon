#!/usr/bin/env python3
"""Derive mixed-track hidden-item expectations from CSS Grid sizing rules."""

from __future__ import annotations

from fractions import Fraction
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
AXIS_SIZE = Fraction(300)
CROSS_SIZE = Fraction(120)
FIXED_TRACK = Fraction(40)
GAP = Fraction(10)
FLEX_FACTORS = (Fraction(1), Fraction(2))
VISIBLE_A_MIN_CONTENT = Fraction(100)
LARGE_SIBLING_MIN_CONTENT = Fraction(150)


def css_px(value: Fraction) -> int | float:
    return value.numerator if value.denominator == 1 else float(value)


def frame(x: Fraction, y: Fraction, width: Fraction, height: Fraction) -> dict[str, int | float]:
    return {
        "x": css_px(x),
        "y": css_px(y),
        "width": css_px(width),
        "height": css_px(height),
    }


def derive_scene(scene_id: str, axis: str, hidden: bool) -> dict[str, object]:
    flexible_space = AXIS_SIZE - FIXED_TRACK - 2 * GAP
    flex_fraction = flexible_space / sum(FLEX_FACTORS)
    first_base = max(
        VISIBLE_A_MIN_CONTENT,
        Fraction(0) if hidden else LARGE_SIBLING_MIN_CONTENT,
    )
    frozen = flex_fraction < first_base
    first_track = first_base if frozen else flex_fraction * FLEX_FACTORS[0]
    third_track = flexible_space - first_track
    tracks = [first_track, FIXED_TRACK, third_track]
    assert sum(tracks) + 2 * GAP == AXIS_SIZE
    assert third_track >= 0

    root = (
        frame(Fraction(0), Fraction(0), AXIS_SIZE, CROSS_SIZE)
        if axis == "Columns"
        else frame(Fraction(0), Fraction(0), CROSS_SIZE, AXIS_SIZE)
    )
    if axis == "Columns":
        a = frame(Fraction(0), Fraction(0), first_track, CROSS_SIZE)
        b = frame(first_track + GAP, Fraction(0), Fraction(24), Fraction(16))
        c = frame(first_track + GAP + FIXED_TRACK + GAP, Fraction(0), third_track, CROSS_SIZE)
        visible_probe = frame(Fraction(0), Fraction(0), first_track, CROSS_SIZE)
        track_axis = "columns"
    else:
        a = frame(Fraction(0), Fraction(0), CROSS_SIZE, first_track)
        b = frame(Fraction(0), first_track + GAP, Fraction(24), Fraction(16))
        c = frame(Fraction(0), first_track + GAP + FIXED_TRACK + GAP, CROSS_SIZE, third_track)
        visible_probe = frame(Fraction(0), Fraction(0), CROSS_SIZE, first_track)
        track_axis = "rows"

    return {
        "sceneId": scene_id,
        "axis": axis,
        "displayNoneChild": hidden,
        "intrinsicInputs": {
            "visibleAminContentCssPx": css_px(VISIBLE_A_MIN_CONTENT),
            "largeSiblingMinContentCssPx": css_px(LARGE_SIBLING_MIN_CONTENT),
            "largeSiblingParticipates": not hidden,
        },
        "trackSizing": {
            "definiteAxisCssPx": css_px(AXIS_SIZE),
            "fixedNeighborCssPx": css_px(FIXED_TRACK),
            "gapsCssPx": [css_px(GAP), css_px(GAP)],
            "flexibleSpaceBeforeExpansionCssPx": css_px(flexible_space),
            "initialFlexFractionCssPx": css_px(flex_fraction),
            "firstTrackBaseCssPx": css_px(first_base),
            "firstTrackFreezes": frozen,
            "usedTracksCssPx": [css_px(value) for value in tracks],
            "remainingFreeSpaceCssPx": 0,
            "trackAxis": track_axis,
        },
        "rootRelativeFramesCssPx": {
            "root": root,
            "a": a,
            "b": b,
            "c": c,
            "hidden": None if hidden else visible_probe,
        },
        "hiddenBox": {"display": "none", "generated": False, "clientRectCount": 0}
        if hidden
        else {"display": "block", "generated": True, "clientRectCount": 1},
    }


def main() -> None:
    scenes = [
        derive_scene("column-hidden", "Columns", True),
        derive_scene("column-visible-control", "Columns", False),
        derive_scene("row-hidden", "Rows", True),
        derive_scene("row-visible-control", "Rows", False),
    ]
    assert scenes[0]["trackSizing"]["usedTracksCssPx"] == [100, 40, 140]
    assert scenes[1]["trackSizing"]["usedTracksCssPx"] == [150, 40, 90]
    assert scenes[2]["trackSizing"]["usedTracksCssPx"] == [100, 40, 140]
    assert scenes[3]["trackSizing"]["usedTracksCssPx"] == [150, 40, 90]
    (ROOT / "expected.json").write_text(json.dumps({"scenes": scenes}, indent=2) + "\n")


if __name__ == "__main__":
    main()
