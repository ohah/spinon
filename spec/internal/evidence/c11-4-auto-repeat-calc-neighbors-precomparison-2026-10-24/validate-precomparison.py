#!/usr/bin/env python3
"""고정 CSS 입력으로 auto-repeat calc neighbor 기대값을 독립 계산한다."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


HERE = Path(__file__).resolve().parent
OBSERVATION = HERE / "chrome156-observation.json"
EXPECTATIONS = HERE / "spec-derived-expectations.json"
GAP = 4.0
REPEAT_MINIMUM = 40.0
TOLERANCE = 1.0 / 64.0


CASES = {
    "col-fill-both": {
        "axis": "columns", "size": 240.0, "prefix": 22.0, "suffix": 10.0,
        "prefix_present": True, "suffix_present": True,
    },
    "row-fill-both": {
        "axis": "rows", "size": 240.0, "prefix": 22.0, "suffix": 10.0,
        "prefix_present": True, "suffix_present": True,
    },
    "col-fill-prefix": {
        "axis": "columns", "size": 240.0, "prefix": 22.0, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "row-fill-suffix": {
        "axis": "rows", "size": 240.0, "prefix": 0.0, "suffix": 10.0,
        "prefix_present": False, "suffix_present": True,
    },
    "col-fit-first": {
        "axis": "columns",
        "size": 240.0,
        "prefix": 22.0,
        "suffix": 10.0,
        "prefix_present": True,
        "suffix_present": True,
        "fit": "first",
    },
    "col-fit-empty": {
        "axis": "columns",
        "size": 240.0,
        "prefix": 22.0,
        "suffix": 10.0,
        "prefix_present": True,
        "suffix_present": True,
        "fit": "empty",
    },
    "col-fill-overflow": {
        "axis": "columns",
        "size": 240.0,
        "prefix": 196.0,
        "suffix": 74.0,
        "prefix_present": True,
        "suffix_present": True,
        "item_offset": (244.0, 0.0),
    },
    "col-fill-border-box": {
        "axis": "columns",
        "size": 116.0,
        "prefix": 9.8,
        "suffix": 5.9,
        "prefix_present": True,
        "suffix_present": True,
        "outer_size": 136.0,
    },
    "col-fill-extra-term": {
        "axis": "columns", "size": 240.0, "prefix": 22.0, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "col-fill-negative-term": {
        "axis": "columns", "size": 240.0, "prefix": 10.0, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "col-fill-negative-final": {
        "axis": "columns", "size": 240.0, "prefix": 0.0, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "col-fill-negative-percentage-term": {
        "axis": "columns", "size": 240.0, "prefix": 8.0, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "col-max-content": {
        "axis": "columns", "size": None, "prefix": None, "suffix": 0.0,
        "prefix_present": True, "suffix_present": False,
    },
    "col-fill-fixed-control": {
        "axis": "columns", "size": 240.0, "prefix": 22.0, "suffix": 10.0,
        "prefix_present": True, "suffix_present": True,
    },
}


def repeat_count(size: float, prefix: float, suffix: float, fixed_count: int) -> int:
    fitting = [
        count
        for count in range(1, 1001)
        if prefix
        + suffix
        + count * REPEAT_MINIMUM
        + max(0, count + fixed_count - 1) * GAP
        <= size
    ]
    return max(fitting) if fitting else 1


def expected_tracks(case: dict[str, object]) -> tuple[int, list[float] | None, float | None]:
    size = case["size"]
    if size is None:
        # Definite preferred/max size가 없으면 auto-repeat은 한 번이다.
        return 1, None, None

    prefix = float(case["prefix"] or 0.0)
    suffix = float(case["suffix"])
    prefix_present = bool(case["prefix_present"])
    suffix_present = bool(case["suffix_present"])
    fixed_count = int(prefix_present) + int(suffix_present)
    count = repeat_count(float(size), prefix, suffix, fixed_count)
    fit = case.get("fit")
    overflow: float | None = None

    if fit == "empty":
        repeated = [0.0] * count
        tracks = ([prefix] if prefix_present else []) + repeated + ([suffix] if suffix_present else [])
        return count, tracks, 0.0

    if fit == "first":
        # 비어 있지 않은 한 repeat와 양쪽 이웃 사이 gutter만 남는다.
        used_gaps = fixed_count
        flex = max(REPEAT_MINIMUM, float(size) - prefix - suffix - used_gaps * GAP)
        repeated = [flex] + [0.0] * (count - 1)
        tracks = ([prefix] if prefix_present else []) + repeated + ([suffix] if suffix_present else [])
        return count, tracks, 0.0

    gaps = max(0, count + fixed_count - 1)
    free = float(size) - prefix - suffix - count * REPEAT_MINIMUM - gaps * GAP
    track = REPEAT_MINIMUM + max(0.0, free) / count
    overflow = max(0.0, -free)
    tracks = ([prefix] if prefix_present else []) + [track] * count + ([suffix] if suffix_present else [])
    return count, tracks, overflow


def css_tracks(value: str) -> list[float]:
    return [float(item) for item in re.findall(r"([+-]?(?:\d+(?:\.\d*)?|\.\d+))px", value)]


def build_expectations() -> dict[str, object]:
    cases: dict[str, object] = {}
    for name, case in CASES.items():
        count, tracks, overflow = expected_tracks(case)
        cases[name] = {
            "axis": case["axis"],
            "definiteContentSize": case["size"],
            "repeatCount": count,
            "usedTracksCssPx": tracks,
            "overflowBeyondRepeatMinimumCssPx": overflow,
            "fixtureScope": (
                "out-of-scope negative control"
                if name in {"col-fill-extra-term", "col-max-content", "col-fill-negative-percentage-term"}
                else "in-scope oracle"
            ),
        }
    return {
        "source": "CSS Grid 2025 CRD §7.2.1 and §7.2.3.1-§7.2.3.2; values independently derived from fixture inputs",
        "gapCssPx": GAP,
        "repeatMinimumCssPx": REPEAT_MINIMUM,
        "trackValueToleranceCssPx": TOLERANCE,
        "cases": cases,
    }


def main() -> int:
    expectations = build_expectations()
    if len(sys.argv) == 2 and sys.argv[1] == "--write-expectations":
        EXPECTATIONS.write_text(json.dumps(expectations, indent=2) + "\n")
        print(f"wrote {EXPECTATIONS}")
        return 0

    observed = json.loads(OBSERVATION.read_text())
    observed_by_id = {scene["id"]: scene for scene in observed["scenes"]}
    if set(observed_by_id) != set(CASES):
        raise SystemExit(f"scene id mismatch: expected={sorted(CASES)} actual={sorted(observed_by_id)}")
    if len(observed["scenes"]) != 14:
        raise SystemExit("Chrome 156 fixed observation must contain 14 scenes")

    for name, case in CASES.items():
        scene = observed_by_id[name]
        expected = expectations["cases"][name]
        axis = case["axis"]
        actual = css_tracks(scene[axis])
        tracks = expected["usedTracksCssPx"]
        if tracks is None:
            # Cyclic max-content used values are recorded as a browser observation only.
            if len(actual) != expected["repeatCount"] + 1:
                raise SystemExit(f"{name}: expected one repeat plus its calc neighbor, got {actual}")
            continue
        if len(actual) != len(tracks):
            raise SystemExit(f"{name}: expected tracks {tracks}, got {actual}")
        for index, (wanted, got) in enumerate(zip(tracks, actual)):
            if abs(wanted - got) > TOLERANCE:
                raise SystemExit(f"{name}[{index}]: expected {wanted}px ± {TOLERANCE}px, got {got}px")
        if "item_offset" in case:
            wanted_x, wanted_y = case["item_offset"]
            grid = scene["grid"]
            item = scene["item"]
            if item is None:
                raise SystemExit(f"{name}: expected an overflow item frame")
            if abs(item["x"] - grid["x"] - wanted_x) > TOLERANCE:
                raise SystemExit(f"{name}: item x offset is {item['x'] - grid['x']}px, expected {wanted_x}px")
            if abs(item["y"] - grid["y"] - wanted_y) > TOLERANCE:
                raise SystemExit(f"{name}: item y offset is {item['y'] - grid['y']}px, expected {wanted_y}px")
            if item["x"] < grid["x"] + grid["width"] or item["x"] + item["width"] <= grid["x"] + grid["width"]:
                raise SystemExit(f"{name}: item does not cross the grid's right border-box edge")

    EXPECTATIONS.write_text(json.dumps(expectations, indent=2) + "\n")
    print(f"PASS: {len(CASES)} scenes, CSS track values within {TOLERANCE}px")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
