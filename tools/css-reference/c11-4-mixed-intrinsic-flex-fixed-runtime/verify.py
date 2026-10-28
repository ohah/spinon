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
TRACE_TOLERANCE = 1e-4
FRAME_TOLERANCE = 1e-4
NUMBER = r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?"
FRAME_RE = re.compile(
    rf"^(\d+):node=(\d+),x=({NUMBER}),y=({NUMBER}),width=({NUMBER}),height=({NUMBER})$"
)
SCENE_IDS = {
    "column-freeze", "column-no-freeze", "column-equal-boundary",
    "column-freeze-reversed", "row-freeze", "row-no-freeze",
    "row-equal-boundary", "row-freeze-reversed",
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
    if not isinstance(inputs, list) or len(inputs) != 3:
        raise AssertionError(f"{case['id']}: direct Grid intrinsic input count 오류")
    order = ["c", "b", "a"] if case["id"].endswith("reversed") else ["a", "b", "c"]
    item_nodes = [2, 4, 5]
    target_dimension = "height" if case["axis"] == "rows" else "width"
    for index, (item, value) in enumerate(zip(order, inputs)):
        if value.get("sourceOrder") != index or value.get("node") != item_nodes[index]:
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
        expected = (case["minContentBasePx"] if item == "a" else 40)
        close(contribution[target_dimension].get("minContent"), expected,
              f"{case['id']}/{item}: min-content contribution")


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


def verify_scene_log(log: str, run_id: str, case: dict[str, Any]) -> dict[str, Any]:
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
    if "status=0 layout=ready boxes=4" not in line:
        raise AssertionError(f"{scene_id}: ready/box summary 불일치")
    if "SPINON_C0410_DRAW" not in log or "status=0 presented boxes=4" not in log:
        raise AssertionError(f"{scene_id}: presented marker 누락")
    frames = parse_node_frames(log, scene_id)
    verify_frames(frames, case)
    diagnostics, chunk_count = trace_diagnostics(log, run_id, scene_id)
    trace = validate_diagnostics(diagnostics, case)
    return {"sceneId": scene_id, "runID": run_id, "status": "passed", "boxCount": 4,
            "nodeFrames": frames, "traceChunks": chunk_count, **trace,
            "presented": True}


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
    for line in body.splitlines():
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
    if not {"unknown-scene", "conflicting-c1121-profile"}.issubset(rejected):
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
        result = verify_scene_log(log_path.read_text(), observation["runID"], expected_cases[scene_id])
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
    order = ["c", "b", "a"] if case["id"].endswith("reversed") else ["a", "b", "c"]
    item_nodes = [2, 4, 5]
    inputs = []
    for index, item in enumerate(order):
        if item == "b":
            source = {"kind": "replaced", "natural": {"width": 24, "height": 16}}
            contribution = {"width": {"minContent": 24}, "height": {"minContent": 16}}
        else:
            minimum = case["minContentBasePx"] if item == "a" else 40
            dimension = "height" if case["axis"] == "rows" else "width"
            source = {"kind": "orthogonalText" if case["axis"] == "rows" else "text"}
            contribution = {dimension: {"minContent": minimum}}
        inputs.append({"sourceOrder": index, "node": item_nodes[index],
                       "source": source, "contributionInput": contribution})
    passes = []
    for index, axis in enumerate(axes):
        if index == target_index:
            stages = [
                {"stage": "Initialize"}, {"stage": "MaximizeTracks"},
                {"stage": "ExpandFlexibleTracks", "tracks": flex_tracks},
                {"stage": "StretchAutoTracks { applied: false }"},
            ]
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
    return passed


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--android-run-dir", type=Path)
    parser.add_argument("--ios-run-dir", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    expected = load_json(EXPECTED_PATH)
    cases = validate_case_inventory(expected.get("cases"))
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
