#!/usr/bin/env python3
"""Run the pinned C11.4 auto-fit-columns scene list on the existing Android AVD."""

from __future__ import annotations

import hashlib
import json
import math
import re
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
SERIAL = "emulator-5554"
PACKAGE = "dev.spinon.bootstrap"
ACTIVITY = f"{PACKAGE}/.MainActivity"
APK = ROOT / "platforms/android/app/build/outputs/apk/debug/app-debug.apk"
SCREENSHOT_SCENES = {1, 2, 7, 8, 11, 14, 19, 31}
TERMINAL_MARKERS = (
    "SPINON_C114_GRID_SUMMARY",
    "SPINON_C114_GRID_ASSERTION_ERROR",
    "SPINON_C114_GRID_EVAL_DETAIL",
)
DIAGNOSTICS_MARKER = "grid_track_sizing_diagnostics="
EXPECTED_MARKER = "c11_4_grid_auto_fit_columns_wpt_expected="
EPSILON = 0.02
SOURCE_FILES = (
    "crates/spinon-ffi/src/runtime_gpu.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_fit_columns_wpt.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_fit_columns_wpt_cases.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs",
    "crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs",
    "crates/spinon-layout/src/calc_tree.rs",
    "crates/spinon-layout/src/grid.rs",
    "crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs",
    "platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java",
    "platforms/android/app/src/main/java/dev/spinon/bootstrap/C114GridAutoFitColumnsWpt.java",
    "platforms/ios/Sources/C0410RuntimeGpuDemo.swift",
    "platforms/ios/Sources/C114GridAutoFitColumnsWpt.swift",
    "vendor/taffy/src/compute/grid/mod.rs",
    "spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/chrome-observation.json",
    "spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/derive_runtime_cases.py",
    "spec/internal/evidence/c11-4-grid-auto-fit-columns-runtime-2026-10-27/run_android_wpt_matrix.py",
)


def run(arguments: list[str], *, timeout: float = 30) -> str:
    result = subprocess.run(
        arguments,
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    return result.stdout


def matrix_log(run_id: str) -> str:
    return run(["adb", "-s", SERIAL, "logcat", "-d", "-v", "threadtime"])


def android_log_pid(line: str) -> str | None:
    fields = line.split(maxsplit=5)
    return fields[2] if len(fields) >= 5 and fields[2].isdigit() else None


def close(left: float, right: float) -> bool:
    return abs(left - right) <= EPSILON


def parse_track_list(value: str) -> tuple[list[tuple[frozenset[str], float]], frozenset[str]] | None:
    token_pattern = re.compile(r"\[([^\]]*)\]|((?:\d+(?:\.\d+)?|\.\d+)px)")
    cursor = 0
    pending_names: set[str] = set()
    tracks: list[tuple[frozenset[str], float]] = []
    for match in token_pattern.finditer(value):
        if value[cursor : match.start()].strip():
            return None
        if match.group(1) is not None:
            pending_names.update(match.group(1).split())
        else:
            assert match.group(2) is not None
            tracks.append((frozenset(pending_names), float(match.group(2)[:-2])))
            pending_names.clear()
        cursor = match.end()
    if value[cursor:].strip() or not tracks:
        return None
    return tracks, frozenset(pending_names)


def track_lists_match(actual_text: str, expected_text: str) -> bool:
    actual = parse_track_list(actual_text)
    expected = parse_track_list(expected_text)
    if actual is None or expected is None:
        return False
    actual_tracks, actual_trailing = actual
    expected_tracks, expected_trailing = expected
    return (
        actual_trailing == expected_trailing
        and len(actual_tracks) == len(expected_tracks)
        and all(
            actual_names == expected_names and close(actual_size, expected_size)
            for (actual_names, actual_size), (expected_names, expected_size)
            in zip(actual_tracks, expected_tracks)
        )
    )


def extract_trace(scoped: str) -> tuple[dict[str, object], dict[str, object]] | None:
    chunks: dict[int, str] = {}
    total = None
    for line in scoped.splitlines():
        if "SPINON_C114_GRID_TRACE_CHUNK" not in line:
            continue
        match = re.search(r"part=(\d+)/(\d+) (.*)$", line)
        if not match:
            return None
        part, count, payload = match.groups()
        total = int(count)
        if int(part) in chunks:
            return None
        chunks[int(part)] = payload
    if total is None or set(chunks) != set(range(1, total + 1)):
        return None
    combined = "".join(chunks[index] for index in range(1, total + 1))
    diagnostic_start = combined.find(DIAGNOSTICS_MARKER)
    if diagnostic_start < 0:
        return None
    decoder = json.JSONDecoder()
    diagnostics, end = decoder.raw_decode(
        combined[diagnostic_start + len(DIAGNOSTICS_MARKER) :]
    )
    expected_start = combined.find(
        EXPECTED_MARKER,
        diagnostic_start + len(DIAGNOSTICS_MARKER) + end,
    )
    if expected_start < 0:
        return None
    expected, _ = decoder.raw_decode(combined[expected_start + len(EXPECTED_MARKER) :])
    if not isinstance(diagnostics, dict) or not isinstance(expected, dict):
        return None
    return diagnostics, expected


def resolved_axis_matches(
    actual: dict[str, object], observation: dict[str, object], axis: str,
    explicit_count: int, repeat_mode: str,
) -> bool:
    suffix = "Columns" if axis == "columns" else "Rows"
    word = "Column" if axis == "columns" else "Row"
    expected_list = observation.get(f"gridTemplate{suffix}")
    if not isinstance(expected_list, str):
        return False
    parsed = parse_track_list(expected_list)
    if parsed is None or not track_lists_match(actual.get("usedTrackList", ""), expected_list):
        return False
    tracks, _ = parsed
    expected_explicit = observation.get(f"explicit{word}TrackCount")
    expected_negative = observation.get(f"negativeImplicit{word}TrackCount")
    expected_positive = observation.get(f"positiveImplicit{word}TrackCount")
    if any(not isinstance(value, int) for value in (
        expected_explicit, expected_negative, expected_positive
    )):
        return False
    if (
        expected_explicit != explicit_count
        or actual.get("explicitTrackCount") != expected_explicit
        or actual.get("negativeImplicitTrackCount") != expected_negative
        or actual.get("positiveImplicitTrackCount") != expected_positive
        or expected_explicit + expected_negative + expected_positive != len(tracks)
    ):
        return False
    collapsed = actual.get("collapsedTrackIndices")
    expected_collapsed = (
        [index for index, (_, size) in enumerate(tracks) if close(size, 0)]
        if repeat_mode == "auto-fit" else []
    )
    return collapsed == expected_collapsed


def pass_matches(
    passed: dict[str, object], expected_list: str, gap_value: str, repeat_mode: str,
) -> bool:
    parsed = parse_track_list(expected_list)
    stages = passed.get("stages")
    if parsed is None or not isinstance(stages, list) or len(stages) < 2:
        return False
    expected_tracks, _ = parsed
    if gap_value == "normal":
        gap = 0.0
    elif gap_value.endswith("px"):
        gap = float(gap_value[:-2])
    else:
        return False
    collapsed = [repeat_mode == "auto-fit" and close(size, 0) for _, size in expected_tracks]
    trailing_collapsed_start = len(collapsed)
    while trailing_collapsed_start > 0 and collapsed[trailing_collapsed_start - 1]:
        trailing_collapsed_start -= 1
    expected_internal_count = len(expected_tracks) * 2 + 1
    for stage_index, stage in enumerate(stages):
        final_stage = stage_index == len(stages) - 1
        if not isinstance(stage, dict):
            return False
        tracks = stage.get("tracks")
        if not isinstance(tracks, list) or len(tracks) != expected_internal_count:
            return False
        for index, track in enumerate(tracks):
            if not isinstance(track, dict):
                return False
            base = track.get("baseSize")
            growth = track.get("growthLimit")
            infinitely = track.get("infinitelyGrowable")
            if (
                track.get("internalIndex") != index
                or track.get("gutter") != (index % 2 == 0)
                or not isinstance(base, (int, float))
                or float(base) < -EPSILON
                or not isinstance(track.get("collapsed"), bool)
                or not isinstance(infinitely, bool)
            ):
                return False
            if isinstance(growth, (int, float)):
                if not math.isfinite(float(growth)) or float(growth) < -EPSILON:
                    return False
            elif growth is not None:
                return False
            if index % 2:
                track_index = index // 2
                expected_collapsed = collapsed[track_index]
                if track["collapsed"] != expected_collapsed or (
                    final_stage and not close(float(base), expected_tracks[track_index][1])
                ):
                    return False
                if expected_collapsed and (
                    not isinstance(growth, (int, float)) or not close(float(growth), 0)
                ):
                    return False
            else:
                line_index = index // 2
                edge = line_index == 0 or line_index == len(expected_tracks)
                previous_track_collapsed = not edge and collapsed[line_index - 1]
                starts_trailing_collapsed_run = (
                    line_index == trailing_collapsed_start
                    and trailing_collapsed_start < len(expected_tracks)
                )
                expected_collapsed = (
                    edge or previous_track_collapsed or starts_trailing_collapsed_run
                )
                if track["collapsed"] != expected_collapsed or not close(
                    float(base), 0 if expected_collapsed else gap
                ):
                    return False
    return stages[0].get("stage") == "Initialize"


def contributions_match(contributions: object, scene_id: str) -> bool:
    if not isinstance(contributions, list):
        return False
    expected_span = {
        "grid-auto-fit-columns-15": 4,
        "grid-auto-fit-columns-18": 3,
        "grid-auto-fit-columns-19": 2,
    }.get(scene_id)
    if expected_span is None:
        return contributions == []
    if len(contributions) != 3:
        return False
    kinds: set[str] = set()
    for contribution in contributions:
        if not isinstance(contribution, dict):
            return False
        kind = contribution.get("kind")
        value = contribution.get("value")
        if (
            kind not in {"Minimum", "MinContent", "MaxContent"}
            or kind in kinds
            or contribution.get("item") != 2
            or contribution.get("span") != expected_span
            or contribution.get("crossesFlexibleTrack") is not False
            or not isinstance(value, (int, float))
            or not math.isfinite(float(value))
            or not close(float(value), 0)
        ):
            return False
        minimum_source = contribution.get("minimumSource")
        if kind == "Minimum":
            if not isinstance(minimum_source, str):
                return False
        elif minimum_source is not None:
            return False
        kinds.add(kind)
    return kinds == {"Minimum", "MinContent", "MaxContent"}


def validate_runtime(scoped: str, scene: dict[str, object]) -> dict[str, object]:
    result: dict[str, object] = {"nodeFramesMatch": False, "traceMatches": False}
    root = scene.get("expectedRootFrame")
    items = scene.get("items")
    if isinstance(root, dict) and isinstance(items, list):
        frames = [
            f"0:node=1,x={root['x']},y={root['y']},width={root['width']},height={root['height']}"
        ]
        for index, item in enumerate(items, start=1):
            if not isinstance(item, dict) or not isinstance(item.get("expected"), dict):
                return result
            frame = item["expected"]
            frames.append(
                f"{index}:node={index + 1},x={frame['x'] + 2},y={frame['y'] + 2},"
                f"width={frame['width']},height={frame['height']}"
            )
        result["nodeFramesMatch"] = (
            "node_frames_css_px=[" + ";".join(frames) + "]"
        ) in scoped

    extracted = extract_trace(scoped)
    if extracted is None:
        return result
    diagnostics, expected = extracted
    repeat = expected.get("repeat")
    observation = expected.get("usedTrackObservation")
    trace_list = diagnostics.get("trackSizingTraces")
    if (
        not isinstance(repeat, dict)
        or not isinstance(observation, dict)
        or not isinstance(trace_list, list)
        or len(trace_list) != 1
        or not isinstance(trace_list[0], dict)
    ):
        return result
    trace = trace_list[0]
    passes = trace.get("passes")
    if (
        not isinstance(passes, list) or len(passes) != 2
        or not isinstance(passes[0], dict) or not isinstance(passes[1], dict)
    ):
        return result
    columns, rows = passes
    repeat_count = repeat.get("expectedCount")
    repeat_mode = repeat.get("mode")
    explicit_count = (
        repeat.get("prefixTrackCount", 0)
        + repeat.get("patternTrackCount", 0) * repeat.get("expectedCount", 0)
        + repeat.get("suffixTrackCount", 0)
    )
    column_list = observation.get("gridTemplateColumns")
    row_list = observation.get("gridTemplateRows")
    column_gap = observation.get("columnGap")
    row_gap = observation.get("rowGap")
    result["traceMatches"] = (
        trace.get("grid") == 1
        and columns.get("axis") == "Columns" and rows.get("axis") == "Rows"
        and columns.get("sequence") == 0 and rows.get("sequence") == 1
        and columns.get("autoRepeatCount") == repeat_count
        and rows.get("autoRepeatCount") is None
        and contributions_match(columns.get("contributions"), scene.get("sceneId", ""))
        and rows.get("contributions") == []
        and isinstance(column_list, str) and isinstance(row_list, str)
        and isinstance(column_gap, str) and isinstance(row_gap, str)
        and resolved_axis_matches(trace.get("usedColumns", {}), observation,
                                  "columns", explicit_count, repeat_mode)
        and resolved_axis_matches(trace.get("usedRows", {}), observation, "rows", 0, "none")
        and pass_matches(columns, column_list, column_gap, repeat_mode)
        and pass_matches(rows, row_list, row_gap, "none")
    )
    result["observedRepeatCount"] = columns.get("autoRepeatCount")
    result["expectedRepeatCount"] = repeat_count
    return result


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--scenes", nargs="*", type=int, default=list(range(1, 36)))
    args = parser.parse_args()
    state = run(["adb", "-s", SERIAL, "get-state"]).strip()
    sdk = run(["adb", "-s", SERIAL, "shell", "getprop", "ro.build.version.sdk"]).strip()
    abi = run(["adb", "-s", SERIAL, "shell", "getprop", "ro.product.cpu.abi"]).strip()
    if state != "device" or sdk != "37" or abi != "arm64-v8a":
        raise SystemExit(f"AVD 불일치: state={state} sdk={sdk} abi={abi}")
    if not APK.is_file():
        raise SystemExit(f"APK 없음: {APK}")

    apk_sha = hashlib.sha256(APK.read_bytes()).hexdigest()
    run(["adb", "-s", SERIAL, "install", "-r", str(APK)], timeout=120)
    installed_paths = run(["adb", "-s", SERIAL, "shell", "pm", "path", PACKAGE]).splitlines()
    if len(installed_paths) != 1 or not installed_paths[0].startswith("package:"):
        raise SystemExit(f"설치된 APK 경로가 단일 base.apk가 아닙니다: {installed_paths}")
    installed_path = installed_paths[0].removeprefix("package:").strip()
    installed_sha = run(
        ["adb", "-s", SERIAL, "shell", "sha256sum", installed_path]
    ).split()[0]
    if installed_sha != apk_sha:
        raise SystemExit(f"설치 APK hash 불일치: built={apk_sha} installed={installed_sha}")
    source_hash = hashlib.sha256()
    for relative in SOURCE_FILES:
        source_hash.update(relative.encode("utf-8"))
        source_hash.update((ROOT / relative).read_bytes())
    case_table = json.loads(
        (ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_fit_columns_wpt_cases.json")
        .read_text(encoding="utf-8")
    )
    cases = {scene["wptIndex"]: scene for scene in case_table["scenes"]}
    batch_id = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    results: list[dict[str, object]] = []
    for index in args.scenes:
        if index not in cases:
            raise SystemExit(f"미등록 WPT scene index: {index}")
        case = cases[index]
        scene = f"grid-auto-fit-columns-{index:02d}"
        run_id = f"c11-4-wpt-{index:02d}-android-{batch_id}"
        run(["adb", "-s", SERIAL, "shell", "am", "force-stop", PACKAGE])
        run(
            [
                "adb",
                "-s",
                SERIAL,
                "shell",
                "am",
                "start",
                "-n",
                ACTIVITY,
                "--ez",
                "spinon_c114_grid_track_sizing",
                "true",
                "--es",
                "spinon_c114_grid_scene",
                scene,
                "--es",
                "spinon_c114_grid_run_id",
                run_id,
            ]
        )

        observed = ""
        draw_line = ""
        run_pid = None
        expected_boxes = 1 + len(case["items"])
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            observed = matrix_log(run_id)
            observed_lines = observed.splitlines()
            scoped = "\n".join(line for line in observed_lines if run_id in line)
            if any(marker in scoped for marker in TERMINAL_MARKERS[1:]):
                break
            summary_index = next(
                (i for i, line in enumerate(observed_lines)
                 if f"SPINON_C114_GRID_SUMMARY run_id={run_id}" in line),
                -1,
            )
            if summary_index >= 0:
                run_pid = android_log_pid(observed_lines[summary_index])
                draw_line = next(
                    (
                        line for line in observed_lines[summary_index + 1:]
                        if android_log_pid(line) == run_pid
                        and "SPINON_C0410_DRAW generation=" in line
                        and f"status=0 presented boxes={expected_boxes} " in line
                    ),
                    "",
                )
                if draw_line:
                    break
            time.sleep(0.25)
        observed_lines = observed.splitlines()
        scoped = "\n".join(
            line for line in observed_lines
            if run_id in line
            or (
                run_pid is not None
                and android_log_pid(line) == run_pid
                and "SPINON_C0410_DRAW generation=" in line
            )
        )
        log_path = EVIDENCE / f"android-grid-auto-fit-{index:02d}-{batch_id}.log"
        log_path.write_text(scoped + ("\n" if scoped else ""), encoding="utf-8")

        summary = f"SPINON_C114_GRID_SUMMARY run_id={run_id}"
        assertion = f"SPINON_C114_GRID_ASSERTION_ERROR run_id={run_id}"
        eval_failure = f"SPINON_C114_GRID_EVAL_DETAIL run_id={run_id}"
        runtime_validation = validate_runtime(scoped, case)
        summary_line = next(
            (line for line in scoped.splitlines() if f"SPINON_C114_GRID_SUMMARY run_id={run_id}" in line),
            "",
        )
        runtime_ready = (
            f"status=0 layout=ready boxes={expected_boxes}" in summary_line
        )
        passed = (
            summary in scoped
            and assertion not in scoped
            and eval_failure not in scoped
            and runtime_ready
            and runtime_validation["nodeFramesMatch"] is True
            and runtime_validation["traceMatches"] is True
            and bool(draw_line)
        )
        repeat_match = re.search(r'"autoRepeatCount":(\d+|null)', scoped)
        expected_match = re.search(r'"expectedCount":(\d+)', scoped)
        frames_match = re.search(r"node_frames_css_px=(\[[^]]+\])", scoped)

        screenshot = None
        if passed and index in SCREENSHOT_SCENES:
            time.sleep(1.0)
            remote = f"/sdcard/{run_id}.png"
            local = EVIDENCE / f"android-grid-auto-fit-{index:02d}-{batch_id}.png"
            run(["adb", "-s", SERIAL, "shell", "screencap", "-p", remote])
            run(["adb", "-s", SERIAL, "pull", remote, str(local)])
            run(["adb", "-s", SERIAL, "shell", "rm", "-f", remote])
            screenshot = str(local.relative_to(ROOT))

        result = {
            "sceneId": scene,
            "runId": run_id,
            "passed": passed,
            "runtimeReady": runtime_ready,
            "drawPresented": bool(draw_line),
            "drawLine": draw_line,
            "runtimePid": run_pid,
            **runtime_validation,
            "expectedRepeatCount": int(expected_match.group(1)) if expected_match else None,
            "observedRepeatCount": int(repeat_match.group(1))
            if repeat_match and repeat_match.group(1) != "null"
            else None,
            "nodeFrames": frames_match.group(1) if frames_match else None,
            "log": str(log_path.relative_to(ROOT)),
            "screenshot": screenshot,
            "failure": next(
                (
                    line[-500:]
                    for marker in ("SPINON_C114_GRID_ASSERTION_ERROR", "SPINON_C114_GRID_EVAL_DETAIL")
                    for line in scoped.splitlines()
                    if marker in line
                ),
                None if passed else "no terminal result before timeout",
            ),
        }
        results.append(result)
        print(
            f"{index:02d} {'PASS' if passed else 'FAIL'} "
            f"repeat={result['observedRepeatCount']}/{result['expectedRepeatCount']} "
            f"draw={'presented' if result['drawPresented'] else 'missing'}"
        )

    summary_path = EVIDENCE / f"android-{batch_id}-summary.json"
    summary_path.write_text(
        json.dumps(
            {
                "device": {"serial": SERIAL, "api": sdk, "abi": abi},
                "apkSha256": apk_sha,
                "installedApkSha256": installed_sha,
                "installedApkPath": installed_path,
                "sourceFilesSha256": source_hash.hexdigest(),
                "sourceHead": run(["git", "rev-parse", "HEAD"]).strip(),
                "sceneCount": len(results),
                "requestedSceneCount": len(args.scenes),
                "passedCount": sum(bool(result["passed"]) for result in results),
                "drawPresentedCount": sum(bool(result["drawPresented"]) for result in results),
                "results": results,
            },
            ensure_ascii=False,
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"summary={summary_path.relative_to(ROOT)}")
    return 0 if all(bool(result["passed"]) for result in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
