#!/usr/bin/env python3
"""Independently validate the signed calc oracle and Chrome CSSOM snapshot."""

import json
import math
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
FIXTURE = ROOT / "tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html"
OBSERVATION = EVIDENCE / "chrome156-observation.json"
TOLERANCE = 1 / 64


def lengths(value: str) -> list[float]:
    tokens = value.split()
    parsed = [re.fullmatch(r"(-?(?:\d+(?:\.\d*)?|\.\d+))px", token) for token in tokens]
    assert all(parsed), f"invalid CSSOM track list: {value!r}"
    return [float(match.group(1)) for match in parsed if match]


def expected_tracks(scene: dict) -> list[float]:
    box = scene["boxModel"]
    basis = box["outerAxisSize"] - box["paddingAxis"] - box["borderAxis"]
    assert basis == scene["basis"], f"{scene['id']}: content basis does not match box model"
    outer_size = scene["grid"]["width" if scene["axis"] == "columns" else "height"]
    assert outer_size == box["outerAxisSize"], f"{scene['id']}: observed outer axis differs from fixture"

    raw_neighbor = scene["terms"]["lengthPx"] + scene["terms"]["percentage"] * basis
    neighbor = max(0.0, raw_neighbor)
    repeat_count = max(1, math.floor((basis - neighbor) / 44 + 1e-9))
    if scene["repeatMode"] == "auto-fit":
        assert scene["occupiedRepeatIndices"] == [0], f"{scene['id']}: unexpected auto-fit occupancy"
        repeated = [max(40.0, basis - neighbor - 4.0)] + [0.0] * (repeat_count - 1)
    else:
        used_size = max(40.0, (basis - neighbor) / repeat_count - 4.0)
        repeated = [used_size] * repeat_count

    expected = [neighbor, *repeated] if scene["neighborSide"] == "prefix" else [*repeated, neighbor]
    assert len(expected) == len(scene["expectedTracks"]), f"{scene['id']}: stale expected track count"
    for index, (actual, recorded) in enumerate(zip(expected, scene["expectedTracks"])):
        assert abs(actual - recorded) <= 1e-9, f"{scene['id']} expected track {index} is not independently derived"
    return expected


def main() -> None:
    assert FIXTURE.is_file(), f"missing fixture: {FIXTURE}"
    observation = json.loads(OBSERVATION.read_text())
    assert observation["sceneCount"] == 9
    assert observation["viewport"] == {"width": 1280, "height": 633, "devicePixelRatio": 1}
    assert all(observation["supports"].values()), "Chrome rejected a CSS-valid calc declaration"

    ids = set()
    for scene in observation["scenes"]:
        scene_id = scene["id"]
        assert scene_id not in ids, f"duplicate scene: {scene_id}"
        ids.add(scene_id)
        axis = scene["axis"]
        actual = lengths(scene[axis])
        expected = expected_tracks(scene)
        assert len(actual) == len(expected), f"{scene_id}: track count {len(actual)} != {len(expected)}"
        for index, (got, want) in enumerate(zip(actual, expected)):
            assert math.isfinite(got) and abs(got - want) <= TOLERANCE, (
                f"{scene_id} track {index}: Chrome {got}px, independent expected {want}px"
            )
        assert scene["grid"]["width"] > 0 and scene["grid"]["height"] > 0
        assert scene["columnGap"] == "4px" and scene["rowGap"] == "4px"

    assert ids == {
        "col-prefix-positive",
        "col-suffix-positive",
        "row-prefix-positive",
        "row-suffix-positive",
        "col-prefix-negative-final",
        "row-suffix-negative-final",
        "col-border-box",
        "col-fit-prefix-item",
        "col-both-negative-final",
    }
    print(f"PASS: {len(ids)}/{len(ids)} Chrome scenes match the independent CSS Grid model within {TOLERANCE:.6f}px")


if __name__ == "__main__":
    main()
