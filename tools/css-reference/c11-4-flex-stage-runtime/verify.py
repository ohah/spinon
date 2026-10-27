#!/usr/bin/env python3
"""Verify C11.4 §11.7 app-runtime evidence against the independent exact oracle."""

from __future__ import annotations

import argparse
import ast
import hashlib
import json
import math
import re
from fractions import Fraction
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
EXPECTED_PATH = ROOT / "spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json"
TRACE_TOLERANCE = 1e-4
FRAME_TOLERANCE = 1e-4
NUMBER = r"[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?"
FRAME_RE = re.compile(
    rf"^(\d+):node=(\d+),x=({NUMBER}),y=({NUMBER}),width=({NUMBER}),height=({NUMBER})$"
)


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
    return Fraction(str(value))


def close(actual: Any, expected: float, label: str, tolerance: float = TRACE_TOLERANCE) -> None:
    if isinstance(actual, bool) or not isinstance(actual, (float, int)):
        raise AssertionError(f"{label}: 숫자가 아닙니다: {actual!r}")
    if not math.isfinite(float(actual)) or abs(float(actual) - expected) > tolerance:
        raise AssertionError(f"{label}: 기대 {expected}, 실제 {actual}, 허용 {tolerance}")


def validate_case_inventory(cases: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for case in cases:
        scene_id = case.get("id")
        if not isinstance(scene_id, str) or scene_id in indexed:
            raise AssertionError(f"누락/중복 scene ID: {scene_id!r}")
        indexed[scene_id] = case
    expected = {"weighted-columns", "frozen-row", "subunit-columns", "zero-fr-row"}
    if set(indexed) != expected:
        raise AssertionError(f"scene 집합 불일치: {sorted(indexed)}")
    return indexed


def expected_track_bases(case: dict[str, Any]) -> list[float]:
    tracks = case.get("tracks")
    if not isinstance(tracks, list) or len(tracks) != 2:
        raise AssertionError(f"{case['id']}: CSS track 2개가 필요합니다.")
    values = case.get("expected_track_bases_px")
    if not isinstance(values, list) or len(values) != len(tracks):
        raise AssertionError(f"{case['id']}: expected track base 수 불일치")
    for item in case.get("fr_fractions_px", []):
        parsed = fraction(item)
        if parsed.denominator <= 0:
            raise AssertionError(f"{case['id']}: 잘못된 fr fraction {item!r}")
    indices = case.get("inflexible_track_indices", [])
    if any(type(index) is not int or not 0 <= index < len(tracks) for index in indices):
        raise AssertionError(f"{case['id']}: 잘못된 inflexible track index")
    if len(set(indices)) != len(indices):
        raise AssertionError(f"{case['id']}: 중복 inflexible track index")
    return [float(fraction(value)) for value in values]


def expected_frames(case: dict[str, Any]) -> list[dict[str, float | int]]:
    axis = case["axis"]
    available = float(case["available_size_px"])
    opposite = float(case["opposite_size_px"])
    gap = float(case["gap_px"])
    first, second = expected_track_bases(case)
    if axis == "columns":
        return [
            {"node": 1, "x": 0, "y": 0, "width": available, "height": opposite},
            {"node": 2, "x": 0, "y": 0, "width": first, "height": opposite},
            {"node": 3, "x": first + gap, "y": 0, "width": second, "height": opposite},
        ]
    if axis == "rows":
        return [
            {"node": 1, "x": 0, "y": 0, "width": opposite, "height": available},
            {"node": 2, "x": 0, "y": 0, "width": opposite, "height": first},
            {"node": 3, "x": 0, "y": first + gap, "width": opposite, "height": second},
        ]
    raise AssertionError(f"{case['id']}: unknown axis {axis!r}")


def verify_frames(actual: list[dict[str, float | int]], case: dict[str, Any]) -> None:
    expected = expected_frames(case)
    if len(actual) != len(expected):
        raise AssertionError(f"{case['id']}: frame 수 {len(actual)} != {len(expected)}")
    for index, (found, wanted) in enumerate(zip(actual, expected)):
        if found.get("index") != index or found.get("node") != wanted["node"]:
            raise AssertionError(f"{case['id']}: frame/node 순서 오류 at {index}")
        for field in ("x", "y", "width", "height"):
            close(found.get(field), float(wanted[field]), f"{case['id']} node {index} {field}", FRAME_TOLERANCE)


def validate_fixed_pass(pass_record: dict[str, Any], case: dict[str, Any]) -> None:
    stages = pass_record.get("stages")
    if [stage.get("stage") for stage in stages or []] != ["Initialize", "FixedTrackEarlyReturn"]:
        raise AssertionError(f"{case['id']}: 고정 축 stage 순서 오류")
    if pass_record.get("contributions") != []:
        raise AssertionError(f"{case['id']}: 고정 축 contribution이 비어 있지 않음")
    opposite = float(case["opposite_size_px"])
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
            item.get("axis") for item in passes] != ["Columns", "Rows"]:
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

    expand_tracks = stages[indexes[2]].get("tracks")
    if not isinstance(expand_tracks, list) or len(expand_tracks) != 5:
        raise AssertionError(f"{case['id']}: Expand track topology 오류")
    bases = expected_track_bases(case)
    actual_bases: list[float] = []
    css_index = 0
    gap = float(case["gap_px"])
    for index, track in enumerate(expand_tracks):
        gutter = index % 2 == 0
        boundary = index in (0, 4)
        if (track.get("internalIndex") != index or track.get("gutter") is not gutter
                or track.get("collapsed") is not boundary):
            raise AssertionError(f"{case['id']}: Expand track flags 오류 at {index}")
        if gutter:
            size = gap if index == 2 else 0.0
        else:
            size = bases[css_index]
            css_index += 1
            actual_bases.append(float(track.get("baseSize")))
        close(track.get("baseSize"), size, f"{case['id']} Expand base {index}")
    if css_index != len(bases):
        raise AssertionError(f"{case['id']}: CSS track count 불일치")
    remaining = float(case["available_size_px"]) - sum(actual_bases) - gap
    close(remaining, float(fraction(case["remaining_space_px"])), f"{case['id']} remaining space")
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
        match = re.search(r"\bpart=(\d+)/(\d+)\s+(.*)$", line[offset + len(marker):])
        if not match:
            raise AssertionError(f"{scene_id}: malformed trace chunk")
        index, found_total = int(match[1]), int(match[2])
        if total is None:
            total = found_total
        if found_total != total or index < 1 or index > total or index in parts:
            raise AssertionError(f"{scene_id}: duplicate/malformed trace part")
        parts[index] = match[3]
    if total is None or set(parts) != set(range(1, total + 1)):
        raise AssertionError(f"{scene_id}: trace part missing/duplicate")
    raw = "".join(parts[index] for index in range(1, total + 1))
    prefix = "grid_track_sizing_diagnostics="
    if not raw.startswith(prefix):
        raise AssertionError(f"{scene_id}: diagnostics marker missing")
    try:
        diagnostics = json.loads(raw[len(prefix):], object_pairs_hook=unique_object)
    except (json.JSONDecodeError, ValueError) as error:
        raise AssertionError(f"{scene_id}: trace JSON invalid: {error}") from error
    return diagnostics, total


def parse_node_frames(log: str, scene_id: str) -> list[dict[str, Any]]:
    marker = "SPINON_C114_GRID_NODE_FRAME "
    frames: list[dict[str, Any]] = []
    for line in log.splitlines():
        offset = line.find(marker)
        if offset < 0:
            continue
        encoded = line[offset + len(marker):].strip()
        match = FRAME_RE.fullmatch(encoded)
        if not match:
            raise AssertionError(f"{scene_id}: malformed node frame: {encoded}")
        values = [float(match[index]) for index in range(3, 7)]
        if not all(math.isfinite(value) for value in values):
            raise AssertionError(f"{scene_id}: non-finite frame")
        frames.append({"index": int(match[1]), "node": int(match[2]),
                       "x": values[0], "y": values[1],
                       "width": values[2], "height": values[3]})
    return frames


def verify_scene_log(log: str, run_id: str, case: dict[str, Any]) -> dict[str, Any]:
    scene_id = case["id"]
    summary = f"SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene_id} "
    matching = [line for line in log.splitlines() if summary in line]
    if len(matching) != 1 or "status=0 layout=ready boxes=3" not in matching[0]:
        raise AssertionError(f"{scene_id}: successful summary missing/duplicate")
    if f"SPINON_C114_GRID_ASSERTION_ERROR run_id={run_id}" in log:
        raise AssertionError(f"{scene_id}: native matcher reported failure")
    if "SPINON_C114_GRID_ARGUMENT_ERROR" in log:
        raise AssertionError(f"{scene_id}: scene route rejected")
    if "SPINON_C0410_DRAW" not in log or "status=0 presented boxes=3" not in log:
        raise AssertionError(f"{scene_id}: presented marker missing")
    frames = parse_node_frames(log, scene_id)
    verify_frames(frames, case)
    diagnostics, chunk_count = trace_diagnostics(log, run_id, scene_id)
    trace = validate_diagnostics(diagnostics, case)
    return {"sceneId": scene_id, "runID": run_id, "status": "passed", "boxCount": 3,
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


def evaluate_fraction_expression(expression: str) -> Fraction:
    tree = ast.parse(expression.strip(), mode="eval")

    def evaluate(node: ast.AST) -> Fraction:
        if isinstance(node, ast.Expression):
            return evaluate(node.body)
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
        raise AssertionError(f"지원하지 않는 상수 수식: {expression}")

    return evaluate(tree)


def platform_track_map(path: Path, language: str) -> dict[str, list[float]]:
    source = path.read_text()
    signature = ("private static double[] expectedTrackBases"
                 if language == "java" else "private static func expectedTrackBases")
    body = extract_function_body(source, signature)
    result: dict[str, list[float]] = {}
    if language == "java":
        pattern = re.compile(r'case\s+"([^"]+)":\s*return\s+new double\[\]\s*\{([^}]+)\}')
    else:
        pattern = re.compile(r'case\s+"([^"]+)":\s*return\s+\[([^\]]+)\]')
    for match in pattern.finditer(body):
        scene_id = match[1]
        values = [evaluate_fraction_expression(item).numerator
                  / evaluate_fraction_expression(item).denominator
                  for item in match[2].split(",")]
        if scene_id in result:
            raise AssertionError(f"{path}: duplicate matcher case {scene_id}")
        result[scene_id] = values
    return result


def compare_platform_tables(cases: dict[str, dict[str, Any]]) -> None:
    tables = [
        (ROOT / "platforms/android/app/src/main/java/dev/spinon/bootstrap/C114FlexStage.java", "java"),
        (ROOT / "platforms/ios/Sources/C114FlexStage.swift", "swift"),
    ]
    expected_ids = set(cases)
    for path, language in tables:
        actual = platform_track_map(path, language)
        if set(actual) != expected_ids:
            raise AssertionError(f"{path}: matcher scene 집합 불일치 {sorted(actual)}")
        for scene_id, case in cases.items():
            expected = expected_track_bases(case)
            if len(actual[scene_id]) != len(expected):
                raise AssertionError(f"{path}/{scene_id}: track 수 불일치")
            for index, (found, wanted) in enumerate(zip(actual[scene_id], expected)):
                close(found, wanted, f"{path.name}/{scene_id}/track-{index}")


def verify_manifest(run_dir: Path, expected_cases: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    manifest_name = "android-simulator.json" if "android" in run_dir.name else "ios-simulator.json"
    manifest_path = run_dir / manifest_name
    manifest = load_json(manifest_path)
    if manifest.get("complete") is not True:
        raise AssertionError(f"{manifest_path}: complete run이 아닙니다.")
    if "android" in manifest["target"]:
        device = manifest["device"]
        if (not str(device.get("serial", "")).startswith("emulator-")
                or device.get("qemu") != "1" or device.get("apiLevel") != 37
                or device.get("abi") != "arm64-v8a" or device.get("pageSizeBytes") != 16384):
            raise AssertionError("Android run이 지정 AVD profile과 일치하지 않습니다.")
        log_prefix = "android-runtime-"
    else:
        if (manifest.get("name") != "iPhone 17 Pro"
                or not str(manifest.get("runtime", "")).endswith("iOS-26-2")):
            raise AssertionError("iOS run이 iPhone 17 Pro / iOS 26.2가 아닙니다.")
        log_prefix = "ios-runtime-"
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
    opposite = float(case["opposite_size_px"])
    fixed_tracks = [
        {"internalIndex": index, "gutter": index % 2 == 0,
         "collapsed": index in (0, 2),
         "baseSize": 0.0 if index % 2 == 0 else opposite,
         "growthLimit": 0.0 if index % 2 == 0 else opposite}
        for index in range(3)
    ]
    flex_tracks = []
    bases = expected_track_bases(case)
    for index in range(5):
        gutter = index % 2 == 0
        size = (float(case["gap_px"]) if index == 2 else 0.0) if gutter else bases[(index - 1) // 2]
        flex_tracks.append({"internalIndex": index, "gutter": gutter,
                            "collapsed": index in (0, 4), "baseSize": size})
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
    return {"trackSizingTraces": [{"grid": 1, "passes": passes}]}


def expect_rejected(label: str, callback: Any) -> None:
    try:
        callback()
    except (AssertionError, ValueError, json.JSONDecodeError):
        return
    raise AssertionError(f"negative control이 통과했습니다: {label}")


def run_negative_controls(cases: dict[str, dict[str, Any]]) -> list[str]:
    case = cases["weighted-columns"]
    passed: list[str] = []
    expect_rejected("duplicate scene", lambda: validate_case_inventory([case, case]))
    passed.append("duplicate scene ID")
    expect_rejected("duplicate JSON key", lambda: json.loads('{"id":1,"id":2}', object_pairs_hook=unique_object))
    passed.append("duplicate JSON key")
    valid = synthetic_diagnostics(case)
    validate_diagnostics(valid, case)

    def swap_initialize_and_maximize(data: dict[str, Any]) -> None:
        stages = data["trackSizingTraces"][0]["passes"][0]["stages"]
        stages[0], stages[1] = stages[1], stages[0]

    for label, mutate in [
        ("missing axis", lambda data: data["trackSizingTraces"][0]["passes"][0].pop("axis")),
        ("Initialize after MaximizeTracks", swap_initialize_and_maximize),
        ("missing Expand stage", lambda data: data["trackSizingTraces"][0]["passes"][0]["stages"].pop(2)),
        ("non-finite track", lambda data: data["trackSizingTraces"][0]["passes"][0]["stages"][2]["tracks"][1].update(baseSize=float("nan"))),
    ]:
        import copy
        changed = copy.deepcopy(valid)
        mutate(changed)
        expect_rejected(label, lambda changed=changed: validate_diagnostics(changed, case))
        passed.append(label)
    wrong_frames = expected_frames(case)
    wrong_frames[1]["width"] = 99
    expect_rejected("wrong frame", lambda: verify_frames(
        [{"index": i, **frame} for i, frame in enumerate(wrong_frames)], case))
    passed.append("wrong frame")
    return passed


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--android-run-dir", type=Path)
    parser.add_argument("--ios-run-dir", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    expected = load_json(EXPECTED_PATH)
    cases = validate_case_inventory(expected.get("cases", []))
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
        observations = verify_manifest(run_dir, cases)
        summary[str(run_dir)] = observations
    if not args.self_test and not args.android_run_dir and not args.ios_run_dir:
        parser.error("--self-test 또는 simulator run directory가 필요합니다.")
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
