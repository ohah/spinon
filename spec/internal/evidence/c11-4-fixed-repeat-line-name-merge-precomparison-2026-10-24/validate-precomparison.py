#!/usr/bin/env python3
"""Check the pinned Chrome observation against independently derived C11.4 expectations."""

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL: {message}")


def item_map(scene: dict) -> dict:
    return {item["id"]: item for item in scene["items"]}


def sizes(template: str) -> list[int]:
    return [int(value) for value in re.findall(r"\]\s+(\d+)px", template)]


def check_rect(scene: dict, item_id: str, expected: list[int]) -> None:
    item = item_map(scene).get(item_id)
    require(item is not None, f"missing item {item_id}")
    actual = [item[key] for key in ("x", "y", "width", "height")]
    require(actual == expected, f"{item_id}: expected {expected}, observed {actual}")


def main() -> None:
    observation = json.loads((EVIDENCE / "chrome156-observation.json").read_text())
    expected = json.loads((EVIDENCE / "spec-derived-expectations.json").read_text())

    require("HeadlessChrome/156.0.0.0" in observation["userAgent"], "unexpected Chrome oracle")
    require(observation["viewport"] == expected["viewport_css_px"], "viewport/DPR drift")
    scenes = {scene["id"]: scene for scene in observation["scenes"]}
    require(
        set(scenes) == {"col-auto-fill", "col-auto-fit", "col-ordinary-join", "row-auto-fill"},
        "scene set drift",
    )

    expected_tracks = expected["auto_repeat_sizing"]["expected_track_sizes_px"]
    for scene_id, axis in (("col-auto-fill", "columns"), ("row-auto-fill", "rows")):
        observed_tracks = sizes(scenes[scene_id][axis])
        require(observed_tracks == expected_tracks, f"{scene_id} track order: {observed_tracks}")
        require(observed_tracks.count(24) == 6, f"{scene_id} expected six auto-repeat tracks")

    column = scenes["col-auto-fill"]
    for item_id, rect in expected["expected_item_rects_relative_to_grid"]["columns_auto_fill"].items():
        check_rect(column, item_id, rect)
    row = scenes["row-auto-fill"]
    for item_id, rect in expected["expected_item_rects_relative_to_grid"]["rows_auto_fill"].items():
        check_rect(row, item_id, rect)

    fit = scenes["col-auto-fit"]
    fit_sizes = sizes(fit["columns"])
    prefix_track_count = 8
    auto_fit = fit_sizes[prefix_track_count : prefix_track_count + 6]
    require(auto_fit == [0, 0, 0, 24, 0, 0], f"auto-fit collapse: {auto_fit}")
    check_rect(
        fit,
        "col-fit-item",
        expected["expected_item_rects_relative_to_grid"]["columns_auto_fit"]["col-fit-item"],
    )

    ordinary = scenes["col-ordinary-join"]
    ordinary_expected = expected["ordinary_track_neighbor"]
    ordinary_tracks = sizes(ordinary["columns"])
    require(ordinary_tracks == ordinary_expected["expected_track_sizes_px"], "ordinary neighbor tracks")
    require(ordinary_tracks.count(12) == ordinary_expected["expected_repetition_count"], "ordinary neighbor auto-repeat count")
    for item_id, rect in expected["expected_item_rects_relative_to_grid"]["ordinary_track_neighbor"].items():
        check_rect(ordinary, item_id, rect)
    for merged_group in ordinary_expected["merged_name_groups"]:
        require(merged_group in ordinary["columns"], f"missing ordinary seam group {merged_group}")

    cssom = column["columns"]
    for merged_group in expected["merged_name_groups"].values():
        if isinstance(merged_group, str):
            require(merged_group in cssom, f"missing merged CSSOM line group {merged_group}")
    require("[mid-end seam seam]" in cssom, "boundary must preserve duplicate seam tokens")
    require("[repeat-end repeat-end join]" in cssom, "fixed/auto boundary names were not merged")

    print("PASS: Chrome 156 oracle matches independent sizing, merge, occurrence, and geometry expectations")
    print("LIMIT: Chrome oracle only; this does not verify Spinon runtime or full C11.4")


if __name__ == "__main__":
    main()
