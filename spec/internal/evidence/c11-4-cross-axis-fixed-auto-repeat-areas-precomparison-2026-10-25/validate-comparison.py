#!/usr/bin/env python3
"""Compare the stored CSS Grid CRD calculations with Agent Browser output."""

import json
import math
import re
from pathlib import Path


EVIDENCE_DIR = Path(__file__).resolve().parent
EXPECTED_PATH = EVIDENCE_DIR / "expected-values.json"
OBSERVED_PATH = EVIDENCE_DIR / "chrome-observation.json"
VALIDATION_PATH = EVIDENCE_DIR / "comparison-validation.json"
TOLERANCE = 1 / 64


def close_list(expected: list[float], observed: list[float], scene: str, field: str) -> None:
    assert len(expected) == len(observed), (scene, field, expected, observed)
    assert all(abs(left - right) <= TOLERANCE for left, right in zip(expected, observed)), (
        scene,
        field,
        expected,
        observed,
    )


def parse_area_matrix(value: str, scene: str) -> list[list[str]]:
    rows = list(re.finditer(r'"([^"]*)"', value))
    assert rows, (scene, "gridTemplateAreas has no quoted rows", value)
    assert re.sub(r'"[^"]*"', "", value).strip() == "", (
        scene,
        "unexpected content outside gridTemplateAreas rows",
        value,
    )
    matrix = [match.group(1).split() for match in rows]
    assert all(matrix) and all(len(row) == len(matrix[0]) for row in matrix), (
        scene,
        "gridTemplateAreas is not a rectangular matrix",
        value,
        matrix,
    )
    return matrix


def expected_child_frames(expected: dict, scene: str) -> dict[str, list[float]]:
    if "childFramesPx" in expected:
        frames = expected["childFramesPx"]
        assert isinstance(frames, dict), (
            scene,
            "childFramesPx must be an object",
            frames,
        )
        return frames

    # Backward compatibility for the original precomparison data. New scenes
    # should carry every child in childFramesPx, including an explicit empty map.
    hero_frame = expected.get("heroFramePx")
    return {"hero": hero_frame} if hero_frame is not None else {}


expected_doc = json.loads(EXPECTED_PATH.read_text())
observed_doc = json.loads(OBSERVED_PATH.read_text())
expected_scenes = expected_doc["scenes"]
observed_results = observed_doc["results"]
observed_scenes = {scene["id"]: scene for scene in observed_results}
assert expected_scenes, "expected-values.json has no scenes"
assert len(observed_scenes) == len(observed_results), "duplicate observed scene IDs"
assert set(expected_scenes) == set(observed_scenes), set(expected_scenes) ^ set(observed_scenes)

gap_px = expected_doc["derivation"]["gapPx"]
repeat_track_px = expected_doc["derivation"]["autoRepeatTrackPx"]
matched = []
for scene_id, expected in expected_scenes.items():
    observed = observed_scenes[scene_id]
    close_list(expected["columnsPx"], observed["columnsPx"], scene_id, "columnsPx")
    close_list(expected["rowsPx"], observed["rowsPx"], scene_id, "rowsPx")
    close_list(expected["rootSizePx"], observed["rootSize"], scene_id, "rootSizePx")
    close_list(expected["overflowPx"], observed["overflowPx"], scene_id, "overflowPx")

    axis = expected["autoRepeatAxis"]
    assert axis in ("columns", "rows"), (scene_id, "autoRepeatAxis", axis)
    axis_index = 0 if axis == "columns" else 1
    assert len(expected["rootSizePx"]) == 2, (
        scene_id,
        "rootSizePx must be [width, height]",
        expected["rootSizePx"],
    )
    close_list(
        [expected["availableSizePx"]],
        [expected["rootSizePx"][axis_index]],
        scene_id,
        "availableSizePx/autoRepeatAxis",
    )
    close_list(
        [expected["availableSizePx"]],
        [observed["rootSize"][axis_index]],
        scene_id,
        "observed availableSizePx/autoRepeatAxis",
    )

    observed_area_matrix = parse_area_matrix(observed["areas"], scene_id)
    expected_area_matrix = expected.get("areaMatrix")
    if expected_area_matrix is not None:
        assert (
            isinstance(expected_area_matrix, list)
            and expected_area_matrix
            and all(isinstance(row, list) and row for row in expected_area_matrix)
            and all(
                isinstance(cell, str)
                for row in expected_area_matrix
                for cell in row
            )
            and all(
                len(row) == len(expected_area_matrix[0])
                for row in expected_area_matrix
            )
        ), (
            scene_id,
            "areaMatrix must be a non-empty rectangular string matrix",
            expected_area_matrix,
        )
        assert observed_area_matrix == expected_area_matrix, (
            scene_id,
            "gridTemplateAreas matrix",
            observed_area_matrix,
            expected_area_matrix,
        )

    area_dimensions = [len(observed_area_matrix[0]), len(observed_area_matrix)]
    expected_area_dimensions = expected.get("areaDimensionsTracks")
    if expected_area_dimensions is None and expected_area_matrix is not None:
        expected_area_dimensions = [len(expected_area_matrix[0]), len(expected_area_matrix)]
    assert area_dimensions == expected_area_dimensions, (
        scene_id,
        area_dimensions,
        expected_area_dimensions,
    )
    fixed_axis_index = 1 - axis_index
    fixed_axis = "rows" if axis == "columns" else "columns"
    fixed_axis_tracks = expected[f"{fixed_axis}Px"]
    fixed_axis_area_dimension = expected["areaDimensionsTracks"][fixed_axis_index]
    assert len(fixed_axis_tracks) == fixed_axis_area_dimension, (
        scene_id,
        "fixed-axis area and expanded track dimension",
        fixed_axis,
        fixed_axis_tracks,
        fixed_axis_area_dimension,
    )

    expected_frames = expected_child_frames(expected, scene_id)
    actual_frames = observed["childFramesPx"]
    assert set(actual_frames) == set(expected_frames), (
        scene_id,
        "child key set",
        actual_frames,
        expected_frames,
    )
    for child, frame in expected_frames.items():
        close_list(frame, actual_frames[child], scene_id, f"{child}FramePx")

    if "heroFramePx" in expected:
        expected_hero = expected["heroFramePx"]
        observed_hero = observed.get("heroFramePx")
        close_list(expected_hero or [], observed_hero or [], scene_id, "heroFramePx")

    computed_count = math.floor(
        (expected["availableSizePx"] + gap_px) / (repeat_track_px + gap_px)
    )
    assert computed_count == expected["autoRepeatCount"], (
        scene_id,
        computed_count,
        expected["autoRepeatCount"],
    )
    auto_axis_tracks = expected[f"{axis}Px"]
    auto_track_indices = expected["autoRepeatTrackIndices"]
    assert auto_track_indices == list(range(computed_count)), (
        scene_id,
        "single auto-repeat group track indices",
        auto_track_indices,
        list(range(computed_count)),
    )
    assert all(index < len(auto_axis_tracks) for index in auto_track_indices), (
        scene_id,
        axis,
        auto_track_indices,
        auto_axis_tracks,
    )
    area_dimension_on_axis = expected["areaDimensionsTracks"][axis_index]
    expected_explicit_axis_tracks = max(computed_count, area_dimension_on_axis)
    assert len(auto_axis_tracks) == expected_explicit_axis_tracks, (
        scene_id,
        axis,
        auto_axis_tracks,
        "expanded auto-repeat and named-area dimensions",
        expected_explicit_axis_tracks,
    )
    if "childFramesPx" in expected:
        actual_child_frames = observed["childFramesPx"]
        assert set(actual_child_frames) == set(expected["childFramesPx"]), (
            scene_id,
            actual_child_frames,
            expected["childFramesPx"],
        )
        for child, frame in expected["childFramesPx"].items():
            close_list(frame, actual_child_frames[child], scene_id, f"{child}FramePx")
    matched.append(scene_id)

validation = {
    "browser": observed_doc["browser"],
    "sceneCount": len(matched),
    "matchedScenes": matched,
    "checks": [
        "resolved columns and rows",
        "root content size",
        "exact named-area matrix and cells"
        if all("areaMatrix" in scene for scene in expected_scenes.values())
        else "named-area dimensions (legacy expectations)",
        "fixed-axis area and expanded track dimension",
        "child key set and every child frame",
        "root overflow geometry",
        "auto-repeat axis mapping, count formula, indices, and area extension",
    ],
    "toleranceCssPx": TOLERANCE,
    "result": f"PASS: all {len(matched)} scenes agree with independent CRD calculations",
}
VALIDATION_PATH.write_text(json.dumps(validation, indent=2) + "\n")
print(json.dumps(validation, indent=2))
