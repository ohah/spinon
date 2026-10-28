#!/usr/bin/env python3
"""Check C11.4 mixed intrinsic/flexible/fixed app-run evidence."""

from __future__ import annotations

import argparse
import ast
import copy
import hashlib
import json
import math
import re
from fractions import Fraction
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
EXPECTED_PATH = ROOT / (
    "spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/expected.json"
)
HIDDEN_EXPECTED_PATH = ROOT / (
    "spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/expected.json"
)
VISIBILITY_EXPECTED_PATH = ROOT / (
    "spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/expected.json"
)
TRACE_TOLERANCE = 1e-4
FRAME_TOLERANCE = 1e-4
NUMBER = r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?"
FRAME_RE = re.compile(
    rf"^(\d+):node=(\d+),x=({NUMBER}),y=({NUMBER}),width=({NUMBER}),height=({NUMBER})$"
)
SCENE_IDS = {
    "column-freeze", "column-no-freeze", "column-equal-boundary",
    "column-freeze-reversed", "row-freeze", "row-no-freeze",
    "row-equal-boundary", "row-freeze-reversed", "column-hidden", "row-hidden",
    "column-visibility-hidden", "column-visibility-visible-control",
    "row-visibility-hidden", "row-visibility-visible-control",
}


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"중복 JSON key: {key}")
        result[key] = value
    return result


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(), object_pairs_hook=unique_object)


def fraction(value: Any) -> Fraction:
    if isinstance(value, dict):
        return Fraction(value["numerator"], value["denominator"])
    return Fraction(str(value))


def close(actual: Any, expected: float, label: str, tolerance: float = TRACE_TOLERANCE) -> None:
    if isinstance(actual, bool) or not isinstance(actual, (float, int)):
        raise AssertionError(f"{label}: 숫자가 아닙니다: {actual!r}")
    if not math.isfinite(float(actual)) or abs(float(actual) - expected) > tolerance:
        raise AssertionError(f"{label}: 기대 {expected}, 실제 {actual}, 허용 {tolerance}")


def validate_case_inventory(cases: Any) -> dict[str, dict[str, Any]]:
    if not isinstance(cases, dict) or set(cases) != SCENE_IDS:
        raise AssertionError(f"scene 집합 불일치: {sorted(cases) if isinstance(cases, dict) else cases!r}")
    indexed: dict[str, dict[str, Any]] = {}
    for scene_id, case in cases.items():
        if not isinstance(case, dict) or case.get("axis") not in {"columns", "rows"}:
            raise AssertionError(f"잘못된 scene record: {scene_id}")
        indexed[scene_id] = {"id": scene_id, **case}
    return indexed


def merge_hidden_cases(cases: dict[str, Any], hidden_scenes: Any) -> dict[str, Any]:
    if not isinstance(hidden_scenes, list):
        raise AssertionError("hidden scene 목록이 아닙니다.")
    merged = dict(cases)
    for scene in hidden_scenes:
        if not isinstance(scene, dict) or scene.get("displayNoneChild") is not True:
            continue
        scene_id = scene.get("sceneId")
        if scene_id not in {"column-hidden", "row-hidden"}:
            raise AssertionError(f"허용되지 않은 hidden scene: {scene_id!r}")
        frames = scene.get("rootRelativeFramesCssPx")
        sizing = scene.get("trackSizing")
        intrinsic = scene.get("intrinsicInputs")
        if not all(isinstance(value, dict) for value in (frames, sizing, intrinsic)):
            raise AssertionError(f"{scene_id}: hidden expected 값 누락")
        merged[scene_id] = {
            "axis": "rows" if scene["axis"] == "Rows" else "columns",
            "crossAxisPx": frames["root"]["width"] if scene["axis"] == "Rows"
                else frames["root"]["height"],
            "sizingAxisPx": sizing["definiteAxisCssPx"],
            "gapPx": sizing["gapsCssPx"][0],
            "tracksPx": sizing["usedTracksCssPx"],
            "remainingFreeSpacePx": sizing["remainingFreeSpaceCssPx"],
            "minContentBasePx": intrinsic["visibleAminContentCssPx"],
            "itemFramesRootRelativePx": {
                "hidden": {"x": 0, "y": 0, "width": 0, "height": 0},
                "a": frames["a"],
                "b": frames["b"],
                "c": frames["c"],
            },
            "hiddenItem": True,
        }
    return merged


def merge_visibility_cases(cases: dict[str, Any], visibility_scenes: Any) -> dict[str, Any]:
    if not isinstance(visibility_scenes, dict):
        raise AssertionError("visibility scene 목록이 아닙니다.")
    merged = dict(cases)
    scene_ids = {
        "column-hidden": "column-visibility-hidden",
        "column-visible-control": "column-visibility-visible-control",
        "row-hidden": "row-visibility-hidden",
        "row-visible-control": "row-visibility-visible-control",
    }
    if set(visibility_scenes) != set(scene_ids):
        raise AssertionError(f"visibility expected scene 집합 불일치: {sorted(visibility_scenes)}")
    for expected_id, scene in visibility_scenes.items():
        scene_id = scene_ids[expected_id]
        axis = "rows" if scene_id.startswith("row-") else "columns"
        root = scene.get("root")
        items = scene.get("items")
        if not isinstance(root, list) or len(root) != 4 or not isinstance(items, dict):
            raise AssertionError(f"{scene_id}: root/items expected 값 누락")
        hidden_frame = items.get("hidden")
        if not isinstance(hidden_frame, list) or len(hidden_frame) != 4:
            raise AssertionError(f"{scene_id}: visibility frame 누락")
        if set(items) != {"a", "hidden", "b", "c"}:
            raise AssertionError(f"{scene_id}: visibility item 집합 불일치")
        if any(not isinstance(values, list) or len(values) != 4 for values in items.values()):
            raise AssertionError(f"{scene_id}: visibility item frame 형식 오류")
        frames = {
            name: dict(zip(("x", "y", "width", "height"), values))
            for name, values in items.items()
        }
        samples = scene.get("surface_sample_root_css_px")
        rgb = scene.get("surface_sample_rgb")
        if not isinstance(samples, list) or len(samples) != 2 or not isinstance(rgb, list) or len(rgb) != 3:
            raise AssertionError(f"{scene_id}: 표면 pixel expected 값 누락")
        hidden = scene.get("hidden_visibility") == "hidden"
        if scene.get("hidden_visibility") not in {"hidden", "visible"}:
            raise AssertionError(f"{scene_id}: visibility 값 오류")
        prefix = f"novel-mixed-intrinsic-flex-fixed-{scene_id}"
        gap_sample = ([60, 155] if axis == "rows" else [155, 60])
        merged[scene_id] = {
            "id": scene_id,
            "axis": axis,
            "crossAxisPx": root[2] if axis == "rows" else root[3],
            "sizingAxisPx": root[3] if axis == "rows" else root[2],
            "gapPx": 10,
            "tracksPx": scene.get("axis_tracks_css_px"),
            "remainingFreeSpacePx": 0,
            "minContentBasePx": 100,
            "visibilityMinContentPx": 150,
            "itemFramesRootRelativePx": frames,
            "visibilityHidden": hidden,
            "boxCount": 4 if hidden else 5,
            "drawBoxCount": 4 if hidden else 5,
            "surfacePixelSamples": [
                {"id": f"{scene_id}-overlap", "nodeId": f"{prefix}-visibility",
                 "xCssPx": samples[0], "yCssPx": samples[1], "rgba": [*rgb, 255]},
                {"id": f"{scene_id}-fixed-track-gap", "nodeId": f"{prefix}-root",
                 "xCssPx": gap_sample[0], "yCssPx": gap_sample[1],
                 "rgba": [232, 237, 245, 255]},
            ],
        }
    return merged


def expected_track_bases(case: dict[str, Any]) -> list[float]:
    tracks = case.get("tracksPx")
    if not isinstance(tracks, list) or len(tracks) != 3:
        raise AssertionError(f"{case['id']}: CSS track 세 개가 필요합니다.")
    return [float(fraction(value)) for value in tracks]


def expected_frames(case: dict[str, Any]) -> list[dict[str, float | int]]:
    rows = case["axis"] == "rows"
    root = {
        "node": 1, "x": 0, "y": 0,
        "width": case["crossAxisPx"] if rows else case["sizingAxisPx"],
        "height": case["sizingAxisPx"] if rows else case["crossAxisPx"],
    }
    if case.get("visibilityHidden") is not None:
        return [
            root,
            {"node": 2, **case["itemFramesRootRelativePx"]["a"]},
            {"node": 4, **case["itemFramesRootRelativePx"]["hidden"]},
            {"node": 6, **case["itemFramesRootRelativePx"]["b"]},
            {"node": 7, **case["itemFramesRootRelativePx"]["c"]},
        ]
    if case.get("hiddenItem"):
        return [
            root,
            {"node": 2, **case["itemFramesRootRelativePx"]["hidden"]},
            {"node": 4, **case["itemFramesRootRelativePx"]["a"]},
            {"node": 6, **case["itemFramesRootRelativePx"]["b"]},
            {"node": 7, **case["itemFramesRootRelativePx"]["c"]},
        ]
    order = ["c", "b", "a"] if case["id"].endswith("reversed") else ["a", "b", "c"]
    item_nodes = [2, 4, 5]
    return [root, *[
        {"node": item_nodes[index], **case["itemFramesRootRelativePx"][item]}
        for index, item in enumerate(order)
    ]]


def verify_frames(actual: list[dict[str, Any]], case: dict[str, Any]) -> None:
    expected = expected_frames(case)
    if len(actual) != len(expected):
        raise AssertionError(f"{case['id']}: frame 수 {len(actual)} != {len(expected)}")
    for index, (found, wanted) in enumerate(zip(actual, expected)):
        if found.get("index") != index or found.get("node") != wanted["node"]:
            raise AssertionError(f"{case['id']}: frame/node 순서 오류 at {index}")
        for field in ("x", "y", "width", "height"):
            close(found.get(field), float(wanted[field]),
                  f"{case['id']} node {index} {field}", FRAME_TOLERANCE)


def validate_fixed_pass(pass_record: dict[str, Any], case: dict[str, Any]) -> None:
    stages = pass_record.get("stages")
    if [stage.get("stage") for stage in stages or []] != [
        "Initialize", "FixedTrackEarlyReturn"
    ]:
        raise AssertionError(f"{case['id']}: 고정 축 stage 순서 오류")
    if pass_record.get("contributions") != []:
        raise AssertionError(f"{case['id']}: 고정 축 contribution이 비어 있지 않음")
    opposite = float(case["crossAxisPx"])
    for stage in stages:
        tracks = stage.get("tracks")
        if not isinstance(tracks, list) or len(tracks) != 3:
            raise AssertionError(f"{case['id']}: 고정 축 topology 오류")
        for index, track in enumerate(tracks):
            gutter = index % 2 == 0
            boundary = index in (0, 2)
            if (track.get("internalIndex") != index or track.get("gutter") is not gutter
                    or track.get("collapsed") is not boundary):
                raise AssertionError(f"{case['id']}: 고정 축 flags 오류 at {index}")
            size = 0.0 if gutter else opposite
            close(track.get("baseSize"), size, f"{case['id']} fixed base {index}")
            close(track.get("growthLimit"), size, f"{case['id']} fixed growth {index}")


def validate_intrinsic_inputs(diagnostics: dict[str, Any], case: dict[str, Any]) -> None:
    inputs = diagnostics.get("gridItemIntrinsicInputs")
    visibility = case.get("visibilityHidden") is not None
    expected_count = 4 if visibility else 3
    if not isinstance(inputs, list) or len(inputs) != expected_count:
        raise AssertionError(f"{case['id']}: direct Grid intrinsic input count 오류")
    hidden = case.get("hiddenItem") is True
    order = (["a", "b", "c"] if hidden else
             (["a", "visibility", "b", "c"] if visibility else
              (["c", "b", "a"] if case["id"].endswith("reversed") else ["a", "b", "c"])))
    item_nodes = ([4, 6, 7] if hidden else
                  ([2, 4, 6, 7] if visibility else [2, 4, 5]))
    target_dimension = "height" if case["axis"] == "rows" else "width"
    for index, (item, value) in enumerate(zip(order, inputs)):
        expected_source_order = index + 1 if hidden else index
        if value.get("sourceOrder") != expected_source_order or value.get("node") != item_nodes[index]:
            raise AssertionError(f"{case['id']}: source order/node 불일치 at {index}")
        source = value.get("source")
        if not isinstance(source, dict):
            raise AssertionError(f"{case['id']}: intrinsic source 누락")
        if item == "b":
            if source.get("kind") != "replaced":
                raise AssertionError(f"{case['id']}: PNG source kind 오류")
            natural = source.get("natural")
            if not isinstance(natural, dict):
                raise AssertionError(f"{case['id']}: PNG natural size 누락")
            close(natural.get("width"), 24, f"{case['id']}: PNG natural width")
            close(natural.get("height"), 16, f"{case['id']}: PNG natural height")
            continue
        expected_kind = "orthogonalText" if case["axis"] == "rows" else "text"
        if source.get("kind") != expected_kind:
            raise AssertionError(f"{case['id']}/{item}: text source kind 오류")
        contribution = value.get("contributionInput")
        if not isinstance(contribution, dict) or not isinstance(contribution.get(target_dimension), dict):
            raise AssertionError(f"{case['id']}/{item}: target-axis contribution 누락")
        expected = (case["minContentBasePx"] if item == "a" else
                    case["visibilityMinContentPx"] if item == "visibility" else 40)
        close(contribution[target_dimension].get("minContent"), expected,
              f"{case['id']}/{item}: min-content contribution")


def validate_visibility_contributions(target: dict[str, Any], case: dict[str, Any]) -> None:
    contributions = target.get("contributions")
    automatic_minimum = (
        "AutomaticMinimum { spans_auto_min_track: false, spans_multiple_tracks: false, "
        "spans_flexible_max_track: true, content_based: false, "
        "is_compressible_replaced: false, fixed_track_sum: None, "
        "fixed_gutter_sum: None, fixed_track_limit: None }"
    )
    signature = lambda item: (
        item.get("item"), item.get("kind"), item.get("span"), item.get("value"),
        item.get("crossesFlexibleTrack"), item.get("minimumSource"),
    )
    actual = sorted(signature(item) for item in contributions or [])
    expected = sorted([
        (2, "Minimum", 1, 0.0, True, automatic_minimum),
        (4, "Minimum", 1, 0.0, True, automatic_minimum),
        (2, "MinContent", 1, 100.0, True, None),
        (4, "MinContent", 1, 150.0, True, None),
    ])
    if actual != expected:
        raise AssertionError(
            f"{case['id']}: visibility hidden span/visible sibling contributions 불일치: {actual!r}"
        )


def validate_diagnostics(diagnostics: dict[str, Any], case: dict[str, Any]) -> dict[str, Any]:
    traces = diagnostics.get("trackSizingTraces")
    if not isinstance(traces, list) or len(traces) != 1:
        raise AssertionError(f"{case['id']}: trace 개수는 하나여야 합니다.")
    trace = traces[0]
    if trace.get("grid") != 1:
        raise AssertionError(f"{case['id']}: grid node 불일치")
    passes = trace.get("passes")
    if not isinstance(passes, list) or len(passes) != 2:
        raise AssertionError(f"{case['id']}: Columns/Rows pass 누락")
    if [item.get("sequence") for item in passes] != [0, 1] or [
        item.get("axis") for item in passes
    ] != ["Columns", "Rows"]:
        raise AssertionError(f"{case['id']}: axis/sequence 불일치")

    target_axis = "Rows" if case["axis"] == "rows" else "Columns"
    target_index = 1 if target_axis == "Rows" else 0
    validate_fixed_pass(passes[1 - target_index], case)
    target = passes[target_index]
    if target.get("axis") != target_axis:
        raise AssertionError(f"{case['id']}: target axis 불일치")
    if case.get("visibilityHidden") is not None:
        validate_visibility_contributions(target, case)
    if case.get("hiddenItem") and any(
        contribution.get("item") == 2
        for axis_pass in passes
        for contribution in axis_pass.get("contributions", [])
    ):
        raise AssertionError(f"{case['id']}: display:none item track contribution 발견")
    stages = target.get("stages")
    if not isinstance(stages, list):
        raise AssertionError(f"{case['id']}: stage 목록 누락")
    names = [stage.get("stage") for stage in stages]
    required = ["Initialize", "MaximizeTracks", "ExpandFlexibleTracks",
                "StretchAutoTracks { applied: false }"]
    indexes: list[int] = []
    for name in required:
        found = [index for index, candidate in enumerate(names) if candidate == name]
        if len(found) != 1:
            raise AssertionError(f"{case['id']}: stage {name!r} 중복 또는 누락")
        indexes.append(found[0])
    if indexes != sorted(indexes):
        raise AssertionError(f"{case['id']}: flex stage 순서 오류")

    tracks = stages[indexes[2]].get("tracks")
    bases = expected_track_bases(case)
    if not isinstance(tracks, list) or len(tracks) != len(bases) * 2 + 1:
        raise AssertionError(f"{case['id']}: Expand track topology 오류")
    actual_bases: list[float] = []
    css_index = 0
    for index, track in enumerate(tracks):
        gutter = index % 2 == 0
        boundary = index in (0, len(tracks) - 1)
        if (track.get("internalIndex") != index or track.get("gutter") is not gutter
                or track.get("collapsed") is not boundary):
            raise AssertionError(f"{case['id']}: Expand track flags 오류 at {index}")
        if gutter:
            size = 0.0 if boundary else float(case["gapPx"])
        else:
            size = bases[css_index]
            css_index += 1
            actual_bases.append(float(track.get("baseSize")))
        close(track.get("baseSize"), size, f"{case['id']} Expand base {index}")
    if css_index != len(bases):
        raise AssertionError(f"{case['id']}: CSS track 수 불일치")
    remaining = float(case["sizingAxisPx"]) - sum(actual_bases) \
        - float(case["gapPx"]) * (len(bases) - 1)
    close(remaining, float(fraction(case["remainingFreeSpacePx"])),
          f"{case['id']} remaining space")
    validate_intrinsic_inputs(diagnostics, case)
    return {"targetAxis": target_axis, "trackBasesCssPx": actual_bases,
            "remainingSpaceCssPx": remaining}


def trace_diagnostics(log: str, run_id: str, scene_id: str) -> tuple[dict[str, Any], int]:
    marker = f"SPINON_C114_GRID_TRACE_CHUNK run_id={run_id} "
    parts: dict[int, str] = {}
    total: int | None = None
    for line in log.splitlines():
        offset = line.find(marker)
        if offset < 0:
            continue
        match = re.search(r"part=(\d+)/(\d+) (.*)$", line[offset:])
        if not match:
            continue
        index, found_total = int(match[1]), int(match[2])
        if total is None:
            total = found_total
        if total != found_total or index in parts or not 1 <= index <= total:
            raise AssertionError(f"{scene_id}: trace chunk 번호/총수 오류")
        parts[index] = match[3]
    if total is None or len(parts) != total or set(parts) != set(range(1, total + 1)):
        raise AssertionError(f"{scene_id}: trace chunk 누락/중복")
    raw = "".join(parts[index] for index in range(1, total + 1))
    raw = raw.removeprefix("grid_track_sizing_diagnostics=")
    return load_json_string(raw), total


def load_json_string(value: str) -> Any:
    return json.loads(value, object_pairs_hook=unique_object)


def parse_node_frames(log: str, scene_id: str) -> list[dict[str, Any]]:
    marker = "SPINON_C114_GRID_NODE_FRAME "
    frames: list[dict[str, Any]] = []
    for line in log.splitlines():
        offset = line.find(marker)
        if offset < 0:
            continue
        match = FRAME_RE.fullmatch(line[offset + len(marker):].strip())
        if not match:
            raise AssertionError(f"{scene_id}: node frame 형식 오류")
        frames.append({"index": int(match[1]), "node": int(match[2]),
                       "x": float(match[3]), "y": float(match[4]),
                       "width": float(match[5]), "height": float(match[6]),
                       "raw": line[offset + len(marker):].strip()})
    return frames


def verify_surface_pixels(log: str, run_id: str, case: dict[str, Any], target: str) -> list[dict[str, Any]]:
    samples = case.get("surfacePixelSamples")
    if not isinstance(samples, list) or not samples:
        return []
    marker = ("SPINON_C114_GRID_PIXEL_MATRIX" if target == "android-avd"
              else "SPINON_C114_METAL_PIXEL")
    summary_marker = f"{marker}_SUMMARY"
    summary = next((line for line in log.splitlines()
                    if summary_marker in line and f"run_id={run_id}" in line), None)
    if summary is None or "status=matched" not in summary:
        raise AssertionError(f"{case['id']}: 정확한 표면 표본 summary 누락/불일치")
    for field in ("expected_samples", "observed_samples", "matched_samples"):
        if f"{field}={len(samples)}" not in summary:
            raise AssertionError(f"{case['id']}: 표면 표본 {field} 불일치")
    begin_marker = f"{marker}_BEGIN"
    begin = next((line for line in log.splitlines()
                  if begin_marker in line and f"run_id={run_id}" in line), None)
    if begin is None:
        raise AssertionError(f"{case['id']}: 표면 표본 begin marker 누락")
    if target == "android-avd":
        drawable_width = re.search(r"\bwidth=(\d+)", begin)
        drawable_height = re.search(r"\bheight=(\d+)", begin)
        viewport_width = re.search(r"\bviewport_width=(\d+)", begin)
        viewport_height = re.search(r"\bviewport_height=(\d+)", begin)
    else:
        drawable = re.search(r"\bdrawable=(\d+)x(\d+)", begin)
        viewport = re.search(r"\bviewport=(\d+)x(\d+)", begin)
        drawable_width = re.match(r"(\d+)", drawable[1]) if drawable else None
        drawable_height = re.match(r"(\d+)", drawable[2]) if drawable else None
        viewport_width = re.match(r"(\d+)", viewport[1]) if viewport else None
        viewport_height = re.match(r"(\d+)", viewport[2]) if viewport else None
    if not all((drawable_width, drawable_height, viewport_width, viewport_height)):
        raise AssertionError(f"{case['id']}: 표면 drawable/viewport 크기 누락")
    scale_x = int(drawable_width[1]) / int(viewport_width[1])
    scale_y = int(drawable_height[1]) / int(viewport_height[1])
    if not math.isfinite(scale_x) or not math.isfinite(scale_y) or scale_x <= 0 or scale_y <= 0:
        raise AssertionError(f"{case['id']}: 표면 pixel scale 오류")
    if abs(scale_x - scale_y) >= 1e-9:
        raise AssertionError(f"{case['id']}: 표면 x/y pixel scale 불일치")
    sample_marker = f"{marker}_SAMPLE"
    for expected in samples:
        line = next((line for line in log.splitlines()
                     if sample_marker in line and f"run_id={run_id}" in line
                     and f"sample={expected['id']}" in line), None)
        if line is None:
            raise AssertionError(f"{case['id']}/{expected['id']}: 표면 표본 로그 누락")
        rgba = ",".join(str(channel) for channel in expected["rgba"])
        if f"rgba={rgba}" not in line or f"expected={rgba} exact=true" not in line:
            raise AssertionError(f"{case['id']}/{expected['id']}: 표면 RGBA mismatch")
        if target == "android-avd" and f"target_node_id={expected['nodeId']}" not in line:
            raise AssertionError(f"{case['id']}/{expected['id']}: PixelCopy target node 불일치")
        if target == "android-avd" and (
            f"css_x={float(expected['xCssPx'])}" not in line
            or f"css_y={float(expected['yCssPx'])}" not in line
        ):
            raise AssertionError(f"{case['id']}/{expected['id']}: PixelCopy CSS 좌표 불일치")
        x = math.floor((expected["xCssPx"] + 0.5) * scale_x)
        y = math.floor((expected["yCssPx"] + 0.5) * scale_y)
        if f"x={x} y={y}" not in line:
            raise AssertionError(f"{case['id']}/{expected['id']}: drawable pixel 좌표 불일치")
    return [{key: sample[key] for key in ("id", "nodeId", "rgba")} for sample in samples]


def verify_scene_log(log: str, run_id: str, case: dict[str, Any], target: str) -> dict[str, Any]:
    scene_id = case["id"]
    if f"SPINON_C114_GRID_ASSERTION_ERROR run_id={run_id}" in log:
        raise AssertionError(f"{scene_id}: native assertion 실패")
    summary_marker = f"SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene_id} "
    line = next((line for line in log.splitlines() if summary_marker in line), None)
    if line is None:
        raise AssertionError(f"{scene_id}: exact runtime summary 누락")
    frame = expected_frames(case)[0]
    if f"expected_root_frame=width={frame['width']},height={frame['height']}" not in line:
        raise AssertionError(f"{scene_id}: root frame summary 불일치")
    frame_count = len(expected_frames(case))
    box_count = case.get("boxCount", frame_count - (1 if case.get("hiddenItem") else 0))
    if f"status=0 layout=ready boxes={box_count}" not in line:
        raise AssertionError(f"{scene_id}: ready/box summary 불일치")
    draw_box_count = case.get("drawBoxCount", box_count)
    if ("SPINON_C0410_DRAW" not in log
            or f"status=0 presented boxes={draw_box_count}" not in log):
        raise AssertionError(f"{scene_id}: presented marker 누락")
    frames = parse_node_frames(log, scene_id)
    verify_frames(frames, case)
    diagnostics, chunk_count = trace_diagnostics(log, run_id, scene_id)
    trace = validate_diagnostics(diagnostics, case)
    pixel_samples = verify_surface_pixels(log, run_id, case, target)
    return {"sceneId": scene_id, "runID": run_id, "status": "passed",
            "boxCount": box_count, "frameCount": frame_count, "drawBoxCount": draw_box_count,
            "nodeFrames": frames, "traceChunks": chunk_count, **trace,
            "presented": True, **({"pixelSamples": pixel_samples} if pixel_samples else {})}


def extract_function_body(source: str, signature: str) -> str:
    start = source.find(signature)
    if start < 0:
        raise AssertionError(f"플랫폼 matcher 함수 누락: {signature}")
    opening = source.find("{", start)
    depth = 0
    for index in range(opening, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[opening + 1:index]
    raise AssertionError(f"플랫폼 matcher 함수 괄호 미완성: {signature}")


def evaluate_expression(expression: str) -> Fraction:
    tree = ast.parse(expression.strip(), mode="eval")

    def evaluate(node: ast.AST) -> Fraction:
        if isinstance(node, ast.Expression): return evaluate(node.body)
        if isinstance(node, ast.Constant) and type(node.value) in (int, float):
            return Fraction(str(node.value))
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, (ast.USub, ast.UAdd)):
            value = evaluate(node.operand)
            return -value if isinstance(node.op, ast.USub) else value
        if isinstance(node, ast.BinOp) and isinstance(node.op, (ast.Add, ast.Sub, ast.Mult, ast.Div)):
            left, right = evaluate(node.left), evaluate(node.right)
            if isinstance(node.op, ast.Add): return left + right
            if isinstance(node.op, ast.Sub): return left - right
            if isinstance(node.op, ast.Mult): return left * right
            return left / right
        raise AssertionError(f"지원하지 않는 수식: {expression}")

    return evaluate(tree)


def platform_track_map(path: Path, language: str) -> dict[str, list[float]]:
    source = path.read_text()
    signature = ("private static double[] expectedTrackBases"
                 if language == "java" else "private static func expectedTrackBases")
    body = extract_function_body(source, signature)
    case_pattern = re.compile(r"case\s+([^:]+):")
    result: dict[str, list[float]] = {}
    labels: list[str] = []
    array_pattern = (re.compile(r'return\s+new double\[\]\s*\{([^}]+)\}')
                     if language == "java" else re.compile(r'return\s+\[([^\]]+)\]'))
    pending_swift_case = ""
    for line in body.splitlines():
        if language == "swift":
            stripped = line.strip()
            if stripped.startswith("case "):
                pending_swift_case = stripped
                if ":" in stripped:
                    labels.extend(re.findall(r'"([^"]+)"', pending_swift_case.split(":", 1)[0]))
                    pending_swift_case = ""
            elif pending_swift_case:
                pending_swift_case += " " + stripped
                if ":" in stripped:
                    labels.extend(re.findall(r'"([^"]+)"', pending_swift_case.split(":", 1)[0]))
                    pending_swift_case = ""
        else:
            for case_labels in case_pattern.findall(line):
                labels.extend(re.findall(r'"([^"]+)"', case_labels))
        match = array_pattern.search(line)
        if not match:
            continue
        values = [float(evaluate_expression(item)) for item in match[1].split(",")]
        for scene_id in labels:
            if scene_id in result:
                raise AssertionError(f"{path}: duplicate matcher case {scene_id}")
            result[scene_id] = values
        labels = []
    return result


def compare_platform_tables(cases: dict[str, dict[str, Any]]) -> None:
    tables = [
        (ROOT / "platforms/android/app/src/main/java/dev/spinon/bootstrap/C114FlexStage.java", "java"),
        (ROOT / "platforms/ios/Sources/C114FlexStage.swift", "swift"),
    ]
    for path, language in tables:
        actual = platform_track_map(path, language)
        for scene_id, case in cases.items():
            if scene_id not in actual:
                raise AssertionError(f"{path}: matcher case 누락 {scene_id}")
            expected = expected_track_bases(case)
            if len(actual[scene_id]) != len(expected):
                raise AssertionError(f"{path}/{scene_id}: track 수 불일치")
            for index, (found, wanted) in enumerate(zip(actual[scene_id], expected)):
                close(found, wanted, f"{path.name}/{scene_id}/track-{index}")


def verify_manifest(run_dir: Path, expected_cases: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    manifest_name = "android-simulator.json" if "android-avd" in run_dir.name else "ios-simulator.json"
    manifest_path = run_dir / manifest_name
    manifest = load_json(manifest_path)
    if manifest.get("complete") is not True:
        raise AssertionError(f"{manifest_path}: complete run이 아닙니다.")
    if manifest.get("target") == "android-avd":
        device = manifest["device"]
        if (not str(device.get("serial", "")).startswith("emulator-")
                or device.get("qemu") != "1" or device.get("apiLevel") != 37
                or device.get("abi") != "arm64-v8a" or device.get("pageSizeBytes") != 16384):
            raise AssertionError("Android run이 지정 AVD profile과 일치하지 않습니다.")
        log_prefix = "android-runtime-"
    elif manifest.get("target") == "ios-simulator":
        if (manifest.get("name") != "iPhone 17 Pro"
                or not str(manifest.get("runtime", "")).endswith("iOS-26-2")):
            raise AssertionError("iOS run이 iPhone 17 Pro / iOS 26.2가 아닙니다.")
        log_prefix = "ios-runtime-"
    else:
        raise AssertionError("지원하지 않는 simulator target")
    rejected = {item.get("label") for item in manifest.get("rejectedRoutes", [])
                if item.get("rejected") is True}
    if not {"unknown-scene", "visible-control", "conflicting-c1121-profile"}.issubset(rejected):
        raise AssertionError(f"{manifest_path}: fail-closed route controls 누락")
    observations = manifest.get("observations")
    if not isinstance(observations, list) or len(observations) != len(expected_cases):
        raise AssertionError(f"{manifest_path}: scene 실행 수 불일치")
    found: dict[str, dict[str, Any]] = {}
    for observation in observations:
        scene_id = observation.get("sceneId")
        if scene_id not in expected_cases or scene_id in found:
            raise AssertionError(f"{manifest_path}: 누락/중복/미등록 scene {scene_id!r}")
        log_path = run_dir / f"{log_prefix}{scene_id}.log"
        result = verify_scene_log(log_path.read_text(), observation["runID"],
                                  expected_cases[scene_id], manifest["target"])
        if "pixelSamples" in result and observation.get("pixelSamples") != result["pixelSamples"]:
            raise AssertionError(f"{scene_id}: manifest Pixel sample과 runtime log 불일치")
        screenshot = observation.get("screenshot", {})
        screenshot_path = ROOT / screenshot.get("path", "")
        data = screenshot_path.read_bytes()
        if hashlib.sha256(data).hexdigest() != screenshot.get("sha256"):
            raise AssertionError(f"{scene_id}: screenshot SHA-256 불일치")
        if not data.startswith(bytes((137, 80, 78, 71, 13, 10, 26, 10))):
            raise AssertionError(f"{scene_id}: screenshot PNG signature 누락")
        result["screenshot"] = screenshot
        found[scene_id] = result
    if set(found) != set(expected_cases):
        raise AssertionError(f"{manifest_path}: scene 집합 불일치")
    return [found[scene_id] for scene_id in expected_cases]


def synthetic_diagnostics(case: dict[str, Any]) -> dict[str, Any]:
    axes = ["Columns", "Rows"]
    target_axis = "Rows" if case["axis"] == "rows" else "Columns"
    target_index = axes.index(target_axis)
    opposite = float(case["crossAxisPx"])
    fixed_tracks = [
        {"internalIndex": index, "gutter": index % 2 == 0,
         "collapsed": index in (0, 2),
         "baseSize": 0.0 if index % 2 == 0 else opposite,
         "growthLimit": 0.0 if index % 2 == 0 else opposite}
        for index in range(3)
    ]
    bases = expected_track_bases(case)
    flex_tracks = []
    for index in range(len(bases) * 2 + 1):
        gutter = index % 2 == 0
        boundary = index in (0, len(bases) * 2)
        size = 0.0 if boundary else float(case["gapPx"]) if gutter else bases[index // 2]
        flex_tracks.append({"internalIndex": index, "gutter": gutter,
                            "collapsed": boundary, "baseSize": size})
    hidden = case.get("hiddenItem") is True
    visibility = case.get("visibilityHidden") is not None
    order = (["a", "b", "c"] if hidden else
             (["a", "visibility", "b", "c"] if visibility else
              (["c", "b", "a"] if case["id"].endswith("reversed") else ["a", "b", "c"])))
    item_nodes = ([4, 6, 7] if hidden else
                  ([2, 4, 6, 7] if visibility else [2, 4, 5]))
    inputs = []
    for index, item in enumerate(order):
        source_order = index + 1 if hidden else index
        if item == "b":
            source = {"kind": "replaced", "natural": {"width": 24, "height": 16}}
            contribution = {"width": {"minContent": 24}, "height": {"minContent": 16}}
        else:
            minimum = (case["minContentBasePx"] if item == "a" else
                       case["visibilityMinContentPx"] if item == "visibility" else 40)
            dimension = "height" if case["axis"] == "rows" else "width"
            source = {"kind": "orthogonalText" if case["axis"] == "rows" else "text"}
            contribution = {dimension: {"minContent": minimum}}
        inputs.append({"sourceOrder": source_order, "node": item_nodes[index],
                       "source": source, "contributionInput": contribution})
    passes = []
    for index, axis in enumerate(axes):
        if index == target_index:
            stages = [
                {"stage": "Initialize"}, {"stage": "MaximizeTracks"},
                {"stage": "ExpandFlexibleTracks", "tracks": flex_tracks},
                {"stage": "StretchAutoTracks { applied: false }"},
            ]
            if visibility:
                contributions = [
                    {"item": 2, "kind": "Minimum", "span": 1, "value": 0.0,
                     "crossesFlexibleTrack": True,
                     "minimumSource": "AutomaticMinimum { spans_auto_min_track: false, spans_multiple_tracks: false, spans_flexible_max_track: true, content_based: false, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }"},
                    {"item": 4, "kind": "Minimum", "span": 1, "value": 0.0,
                     "crossesFlexibleTrack": True,
                     "minimumSource": "AutomaticMinimum { spans_auto_min_track: false, spans_multiple_tracks: false, spans_flexible_max_track: true, content_based: false, is_compressible_replaced: false, fixed_track_sum: None, fixed_gutter_sum: None, fixed_track_limit: None }"},
                    {"item": 2, "kind": "MinContent", "span": 1, "value": 100.0,
                     "crossesFlexibleTrack": True, "minimumSource": None},
                    {"item": 4, "kind": "MinContent", "span": 1, "value": 150.0,
                     "crossesFlexibleTrack": True, "minimumSource": None},
                ]
            else:
                contributions = []
        else:
            stages = [{"stage": "Initialize", "tracks": fixed_tracks},
                      {"stage": "FixedTrackEarlyReturn", "tracks": fixed_tracks}]
            contributions = []
        passes.append({"sequence": index, "axis": axis,
                       "stages": stages, "contributions": contributions})
    return {"gridItemIntrinsicInputs": inputs,
            "trackSizingTraces": [{"grid": 1, "passes": passes}]}


def expect_rejected(label: str, callback: Any) -> None:
    try:
        callback()
    except (AssertionError, ValueError, json.JSONDecodeError, KeyError, TypeError):
        return
    raise AssertionError(f"negative control이 통과했습니다: {label}")


def run_negative_controls(cases: dict[str, dict[str, Any]]) -> list[str]:
    case = cases["column-freeze"]
    passed: list[str] = []
    expect_rejected("duplicate JSON key",
                    lambda: load_json_string('{"id":1,"id":2}'))
    passed.append("duplicate JSON key")
    valid = synthetic_diagnostics(case)
    validate_diagnostics(valid, case)
    mutations = [
        ("missing axis", lambda data: data["trackSizingTraces"][0]["passes"][0].pop("axis")),
        ("missing Expand stage", lambda data: data["trackSizingTraces"][0]["passes"][0]["stages"].pop(2)),
        ("non-finite track", lambda data: data["trackSizingTraces"][0]["passes"][0]["stages"][2]["tracks"][1].update(baseSize=float("nan"))),
        ("wrong source order", lambda data: data["gridItemIntrinsicInputs"][0].update(sourceOrder=2)),
        ("wrong image natural size", lambda data: data["gridItemIntrinsicInputs"][1]["source"]["natural"].update(width=23)),
        ("wrong min-content input", lambda data: data["gridItemIntrinsicInputs"][0]["contributionInput"]["width"].update(minContent=99)),
    ]
    for label, mutate in mutations:
        changed = copy.deepcopy(valid)
        mutate(changed)
        expect_rejected(label, lambda changed=changed: validate_diagnostics(changed, case))
        passed.append(label)
    wrong_frames = expected_frames(case)
    wrong_frames[1]["width"] = 99
    expect_rejected("wrong frame", lambda: verify_frames(
        [{"index": index, **frame} for index, frame in enumerate(wrong_frames)], case))
    passed.append("wrong frame")
    hidden_case = cases["column-hidden"]
    hidden_diagnostics = synthetic_diagnostics(hidden_case)
    validate_diagnostics(hidden_diagnostics, hidden_case)
    wrong_hidden_source_order = copy.deepcopy(hidden_diagnostics)
    wrong_hidden_source_order["gridItemIntrinsicInputs"][0]["sourceOrder"] = 0
    expect_rejected("wrong hidden source order", lambda: validate_intrinsic_inputs(
        wrong_hidden_source_order, hidden_case))
    passed.append("wrong hidden source order")
    leaked_text = copy.deepcopy(hidden_diagnostics)
    leaked_text["gridItemIntrinsicInputs"].insert(0, {
        "sourceOrder": 0,
        "node": 2,
        "source": {"kind": "text"},
        "contributionInput": {"width": {"minContent": 150}},
    })
    expect_rejected("hidden text intrinsic input", lambda: validate_intrinsic_inputs(
        leaked_text, hidden_case))
    passed.append("hidden text intrinsic input")
    leaked_contribution = copy.deepcopy(hidden_diagnostics)
    leaked_contribution["trackSizingTraces"][0]["passes"][0]["contributions"].append(
        {"item": 2})
    expect_rejected("hidden item track contribution", lambda: validate_diagnostics(
        leaked_contribution, hidden_case))
    passed.append("hidden item track contribution")
    visibility_case = cases["row-visibility-hidden"]
    visibility_diagnostics = synthetic_diagnostics(visibility_case)
    validate_diagnostics(visibility_diagnostics, visibility_case)
    wrong_visibility_min_content = copy.deepcopy(visibility_diagnostics)
    wrong_visibility_min_content["gridItemIntrinsicInputs"][1]["contributionInput"]["height"]["minContent"] = 149
    expect_rejected("wrong visibility hidden min-content", lambda: validate_intrinsic_inputs(
        wrong_visibility_min_content, visibility_case))
    passed.append("wrong visibility hidden min-content")
    missing_visibility_contribution = copy.deepcopy(visibility_diagnostics)
    missing_visibility_contribution["trackSizingTraces"][0]["passes"][1]["contributions"] = [
        record for record in missing_visibility_contribution["trackSizingTraces"][0]["passes"][1]["contributions"]
        if not (record.get("item") == 4 and record.get("kind") == "MinContent")
    ]
    expect_rejected("missing visibility hidden track contribution", lambda: validate_diagnostics(
        missing_visibility_contribution, visibility_case))
    passed.append("missing visibility hidden track contribution")
    visible_case = cases["row-visibility-visible-control"]
    visible_diagnostics = synthetic_diagnostics(visible_case)
    validate_diagnostics(visible_diagnostics, visible_case)
    pixel_log = (
        "SPINON_C114_METAL_PIXEL_BEGIN run_id=test drawable=960x720 viewport=320x240\n"
        "SPINON_C114_METAL_PIXEL_SAMPLE run_id=test sample=row-visibility-hidden-overlap x=181 y=376 "
        "rgba=40,100,220,255 expected=40,100,220,255 exact=true\n"
        "SPINON_C114_METAL_PIXEL_SAMPLE run_id=test sample=row-visibility-hidden-fixed-track-gap x=181 y=466 "
        "rgba=232,237,245,255 expected=232,237,245,255 exact=true\n"
        "SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=test expected_samples=2 "
        "observed_samples=2 matched_samples=2"
    )
    expect_rejected("visible paint leaked through hidden visibility sample", lambda: verify_surface_pixels(
        pixel_log.replace("rgba=40,100,220,255 expected=",
                          "rgba=142,68,173,255 expected=", 1),
        "test", visibility_case, "ios-simulator"))
    passed.append("visible paint leaked through hidden visibility sample")
    expect_rejected("wrong visibility sample coordinate", lambda: verify_surface_pixels(
        pixel_log.replace("x=181 y=376", "x=182 y=376", 1),
        "test", visibility_case, "ios-simulator"))
    passed.append("wrong visibility sample coordinate")
    return passed


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--android-run-dir", type=Path)
    parser.add_argument("--ios-run-dir", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    expected = load_json(EXPECTED_PATH)
    hidden_expected = load_json(HIDDEN_EXPECTED_PATH)
    visibility_expected = load_json(VISIBILITY_EXPECTED_PATH)
    cases = validate_case_inventory(merge_visibility_cases(
        merge_hidden_cases(expected.get("cases"), hidden_expected.get("scenes")),
        visibility_expected.get("scenes")))
    compare_platform_tables(cases)
    summary: dict[str, Any] = {
        "expectedSceneIds": list(cases),
        "platformMatcherTables": "matched",
    }
    if args.self_test:
        summary["negativeControls"] = run_negative_controls(cases)
    for run_dir in (args.android_run_dir, args.ios_run_dir):
        if run_dir is None:
            continue
        summary[str(run_dir)] = verify_manifest(run_dir, cases)
    if not args.self_test and not args.android_run_dir and not args.ios_run_dir:
        parser.error("--self-test 또는 simulator run directory가 필요합니다.")
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
