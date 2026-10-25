#!/usr/bin/env python3
"""Capture the fixed C11.4 orthogonal-text scene matrix on Android/iOS simulators."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time


ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
RUN_ID_PREFIX = "c114text-" + dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
ANDROID_SERIAL = os.environ.get("SPINON_ANDROID_EMULATOR_SERIAL", "emulator-5560")
IOS_UDID = os.environ.get(
    "SPINON_IOS_SIMULATOR_UDID", "9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
)
ANDROID_APK = ROOT / "platforms/android/app/build/outputs/apk/debug/app-debug.apk"
IOS_APP = ROOT / "build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
IOS_EXECUTABLE = IOS_APP / "SpinonBootstrap"
BUNDLE_ID = "dev.spinon.bootstrap"

TARGET_ROWS = {
    "single-token-min-content": ("ABC", 30.0, 10.0, 30.0, 10.0, 30.0),
    "single-token-max-content": ("ABC", 30.0, 10.0, 30.0, 10.0, 30.0),
    "two-words-min-content": ("A BC", 20.0, 20.0, 40.0, 10.0, 20.0),
    "two-words-max-content": ("A BC", 20.0, 20.0, 40.0, 10.0, 40.0),
    "three-words-min-content": ("AB CDE F", 30.0, 30.0, 80.0, 10.0, 30.0),
    "three-words-max-content": ("AB CDE F", 30.0, 30.0, 80.0, 10.0, 80.0),
    "collapsed-whitespace-min-content": (
        "  A   9Z  ", 20.0, 20.0, 40.0, 10.0, 20.0
    ),
    "collapsed-whitespace-max-content": (
        "  A   9Z  ", 20.0, 20.0, 40.0, 10.0, 40.0
    ),
    "spaces-only-min-content": ("   ", 0.0, 0.0, 0.0, 0.0, 16.0),
    "spaces-only-max-content": ("   ", 0.0, 0.0, 0.0, 0.0, 16.0),
}
LEGACY_ROWS = {
    "direct-orthogonal-row-min-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 50.0),
    "direct-orthogonal-row-min-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 50.0),
    "direct-orthogonal-row-max-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 80.0),
    "direct-orthogonal-row-max-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 80.0),
    "direct-min-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 48.0),
    "direct-min-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 48.0),
    "direct-max-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 48.0),
    "direct-max-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 48.0),
    "direct-row-min-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 20.0),
    "direct-row-min-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 20.0),
    "direct-row-max-content-forward": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 20.0),
    "direct-row-max-content-reversed": ("XX XXXXX", 50.0, 20.0, 80.0, 10.0, 20.0),
}
SCENES = {**TARGET_ROWS, **LEGACY_ROWS}
assert len(TARGET_ROWS) == 10 and len(LEGACY_ROWS) == 12 and len(SCENES) == 22

SOURCE_FILES = [
    "crates/spinon-text-layout/src/engine/intrinsic.rs",
    "crates/spinon-text-layout/src/tests.rs",
    "crates/spinon-style-to-layout/src/projection.rs",
    "crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/orthogonal_writing_mode.rs",
    "crates/spinon-layout/src/intrinsic_measurement.rs",
    "crates/spinon-layout/src/calc_tree/text.rs",
    "crates/spinon-layout/src/calc_tree/intrinsic_measurements.rs",
    "crates/spinon-layout/src/grid_text_stretch_tests.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/prepare.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/scene.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/style.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/scene.rs",
    "crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/snapshot.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs",
    "platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java",
    "platforms/ios/Sources/C0410RuntimeGpuDemo.swift",
]


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def command(*args: str, timeout: int = 90) -> str:
    result = subprocess.run(args, cwd=ROOT, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=True, timeout=timeout)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {args!r}\n{result.stdout}")
    return result.stdout


def write_command(path: Path, *args: str, timeout: int = 90) -> str:
    output = command(*args, timeout=timeout)
    path.write_text(output, encoding="utf-8")
    return output


def source_snapshot() -> dict:
    files = {}
    for relative in SOURCE_FILES:
        path = ROOT / relative
        if not path.is_file():
            raise RuntimeError(f"missing implementation source: {relative}")
        files[relative] = sha256(path.read_bytes())
    implementation_diff = command(
        "git", "diff", "--binary", "HEAD", "--", *SOURCE_FILES
    ).encode()
    return {
        "head": command("git", "rev-parse", "HEAD").strip(),
        "implementationDiffSha256": sha256(implementation_diff),
        "implementationFiles": files,
        "implementationFilesSha256": sha256(json.dumps(files, sort_keys=True).encode()),
    }


def parse_device_context() -> dict:
    state = command("adb", "-s", ANDROID_SERIAL, "get-state").strip()
    api = command("adb", "-s", ANDROID_SERIAL, "shell", "getprop", "ro.build.version.sdk").strip()
    abi = command("adb", "-s", ANDROID_SERIAL, "shell", "getprop", "ro.product.cpu.abi").strip()
    page_size = command("adb", "-s", ANDROID_SERIAL, "shell", "getconf", "PAGESIZE").strip()
    avd = command("adb", "-s", ANDROID_SERIAL, "emu", "avd", "name").splitlines()[0].strip()
    assert state == "device" and api == "37" and abi == "arm64-v8a" and page_size == "16384"

    devices = json.loads(command("xcrun", "simctl", "list", "devices", "booted", "--json"))
    simulator = None
    for runtime, values in devices["devices"].items():
        for value in values:
            if value["udid"] == IOS_UDID and value["state"] == "Booted":
                simulator = {**value, "runtime": runtime}
    if simulator is None or not simulator["runtime"].endswith("iOS-26-2"):
        raise RuntimeError(f"requested iOS Simulator is not booted on iOS 26.2: {IOS_UDID}")
    return {
        "android": {"serial": ANDROID_SERIAL, "state": state, "api": int(api),
                    "abi": abi, "pageSize": int(page_size), "avd": avd},
        "ios": {"udid": IOS_UDID, "name": simulator["name"],
                "deviceTypeIdentifier": simulator["deviceTypeIdentifier"],
                "runtime": simulator["runtime"]},
    }


def frame_from_line(line: str, key: str) -> tuple[float, float] | None:
    match = re.search(rf"\b{re.escape(key)}=(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)", line)
    if not match:
        return None
    return float(match.group(1)), float(match.group(2))


def revision_from_line(line: str, key: str) -> int | None:
    match = re.search(rf"\b{re.escape(key)}=(\d+)", line)
    return int(match.group(1)) if match else None


def process_id_from_line(line: str) -> int | None:
    ios = re.search(r"\bSpinonBootstrap\[(\d+):", line)
    if ios:
        return int(ios.group(1))
    android = re.search(r"^\S+\s+\S+\s+(\d+)\s+\d+\s+[VDIWEF]\s+SpinonBootstrap:", line)
    return int(android.group(1)) if android else None


def scene_bound_draw(log: str, run_id: str, scene_id: str) -> dict | None:
    lines = log.splitlines()
    summary_index = next((index for index, line in enumerate(lines)
                          if f"SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene_id} " in line), None)
    if summary_index is None:
        return None
    next_summary = next((index for index in range(summary_index + 1, len(lines))
                         if "SPINON_C114_GRID_SUMMARY " in lines[index]), len(lines))
    summary = lines[summary_index]
    process_id = process_id_from_line(summary)
    if process_id is None:
        return None
    expected_match = re.search(
        r"expected_root_frame=width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)", summary
    )
    if not expected_match:
        return None
    expected_root = (float(expected_match.group(1)), float(expected_match.group(2)))
    eval_line = next((line for line in lines
                      if (f"SPINON_C114_GRID_EVAL run_id={run_id} scene_id={scene_id} " in line
                          or f"SPINON_C114_GRID_EVAL_RUN run_id={run_id} scene_id={scene_id} " in line)), None)
    if eval_line is None:
        return None
    document_revision = revision_from_line(eval_line, "document_revision")
    render_tree_revision = revision_from_line(eval_line, "render_tree_revision")
    summary_document_revision = revision_from_line(summary, "document_revision")
    if document_revision is None or summary_document_revision != document_revision:
        return None
    if render_tree_revision is None:
        for line in lines:
            if f"SPINON_C114_GRID_TRACE_CHUNK run_id={run_id} " not in line:
                continue
            match = re.search(r"render_tree:\s*RenderTreeRevision\((\d+)\)", line)
            if match:
                render_tree_revision = int(match.group(1))
                break
    if render_tree_revision is None:
        return None

    for draw_index in range(summary_index + 1, next_summary):
        line = lines[draw_index]
        if "SPINON_C0410_DRAW" not in line or "status=0 presented boxes=3" not in line:
            continue
        if (process_id_from_line(line) != process_id
                or frame_from_line(line, "root_css_px") != expected_root
                or revision_from_line(line, "document_revision") != document_revision
                or revision_from_line(line, "render_tree_revision") != render_tree_revision):
            continue
        return {
            "line": line.strip(),
            "summaryLineNumber": summary_index + 1,
            "evalLineNumber": lines.index(eval_line) + 1,
            "renderTreeRevisionSource": "scene run trace" if "render_tree_revision=" not in eval_line else "scene eval record",
            "processId": process_id,
            "drawLineNumber": draw_index + 1,
            "binding": "post-summary presented draw matching this run's document revision, render-tree revision, and expected root frame",
            "documentRevision": document_revision,
            "renderTreeRevision": render_tree_revision,
            "expectedRootFrameCssPx": {"width": expected_root[0], "height": expected_root[1]},
        }
    return None


def verify_log(log: str, run_id: str, scene_id: str, platform: str) -> dict:
    lines = log.splitlines()
    summary = next((line for line in lines
                    if f"SPINON_C114_GRID_SUMMARY run_id={run_id} scene_id={scene_id} " in line), None)
    if summary is None or "status=0 layout=ready boxes=3" not in summary:
        raise RuntimeError(f"{platform}/{scene_id}: successful native matcher summary missing\n{log[-4000:]}")
    summary_index = lines.index(summary)
    process_id = process_id_from_line(summary)
    if process_id is None:
        raise RuntimeError(f"{platform}/{scene_id}: native summary process identity missing")
    error_lines = [line for line in log.splitlines()
                   if run_id in line and ("SPINON_C114_GRID_ASSERTION_ERROR" in line
                                          or "SPINON_C114_GRID_ARGUMENT_ERROR" in line)]
    if error_lines:
        raise RuntimeError(f"{platform}/{scene_id}: native assertion/argument error\n"
                           + "\n".join(error_lines))
    draw = scene_bound_draw(log, run_id, scene_id)
    if draw is None:
        raise RuntimeError(
            f"{platform}/{scene_id}: no post-summary draw matches the run's document/render revisions and expected root frame\n{log[-4000:]}"
        )
    grid_frame_lines = [
        {"lineNumber": index + 1, "line": line.strip()}
        for index, line in enumerate(lines)
        if process_id_from_line(line) == process_id
        and "SPINON_C114_GRID_NODE_FRAME" in line
    ]
    legacy_frame_lines = [
        {"lineNumber": index + 1, "line": line.strip()}
        for index, line in enumerate(lines)
        if process_id_from_line(line) == process_id
        and "SPINON_C114_NODE_FRAME" in line
    ]
    frame_summaries = [
        {"lineNumber": index + 1, "line": line.strip()}
        for index, line in enumerate(lines)
        if process_id_from_line(line) == process_id
        and ((f"SPINON_C114_GRID_FRAME_SUMMARY run_id={run_id} " in line)
             or ("SPINON_C114_FRAME_SUMMARY" in line and f"state={scene_id}" in line))
        and ("count=3" in line or "frames=3" in line)
    ]
    if len(grid_frame_lines) != 3 or not frame_summaries:
        raise RuntimeError(
            f"{platform}/{scene_id}: expected three node frames and a matching frame summary from process {process_id}; "
            f"got {len(grid_frame_lines)} grid frames and {frame_summaries}"
        )
    chunks = [line for line in log.splitlines()
              if f"SPINON_C114_GRID_TRACE_CHUNK run_id={run_id} " in line]
    parts = sorted({int(match.group(1)) for line in chunks
                    if (match := re.search(r"part=(\d+)/(\d+)", line))})
    expected_parts = next((int(match.group(2)) for line in chunks
                           if (match := re.search(r"part=(\d+)/(\d+)", line))), 0)
    if expected_parts == 0 or parts != list(range(1, expected_parts + 1)):
        raise RuntimeError(f"{platform}/{scene_id}: trace chunks incomplete ({parts}/{expected_parts})")
    text, min_inline, min_block, max_inline, max_block, row = SCENES[scene_id]
    expected_size = min_inline if "min-content" in scene_id else max_inline
    if f"minContentInline={min_inline}" not in log or f"maxContentInline={max_inline}" not in log:
        # The source-bound values live in JSON trace chunks rather than the summary itself.
        if not chunks:
            raise RuntimeError(f"{platform}/{scene_id}: text intrinsic trace is absent")
    return {
        "sceneId": scene_id,
        "runId": run_id,
        "text": text,
        "expectedLogicalIntrinsic": {"minInline": min_inline, "minBlock": min_block,
                                     "maxInline": max_inline, "maxBlock": max_block},
        "expectedRow": row,
        "selectedTextContribution": expected_size,
        "nativeMatcher": "PASS",
        "drawPresent": "PASS",
        "drawBinding": draw,
        "frameBinding": {
            "runId": run_id,
            "processId": process_id,
            "gridNodeFrameCount": len(grid_frame_lines),
            "summaryLine": frame_summaries[0],
            "gridNodeFrameLines": grid_frame_lines,
            "legacyNodeFrameLines": legacy_frame_lines,
            "binding": "native summary process identity; exact run ID for grid summary where available",
        },
        "nodeFrameLineCount": len(grid_frame_lines),
        "traceParts": expected_parts,
        "summary": summary.strip(),
        "draw": draw["line"],
    }


def stable_screenshot(capture, platform: str, scene_id: str) -> tuple[bytes, dict]:
    settle_delay_ms = 1000
    sample_interval_ms = 400
    required_identical_samples = 2
    timeout_seconds = 15
    time.sleep(settle_delay_ms / 1000)
    deadline = time.monotonic() + timeout_seconds
    previous_hash = None
    identical_samples = 0
    samples = 0
    latest = b""
    while time.monotonic() < deadline:
        latest = capture()
        if not latest.startswith(b"\x89PNG\r\n\x1a\n"):
            raise RuntimeError(f"{platform}/{scene_id}: screenshot is not a PNG")
        samples += 1
        digest = sha256(latest)
        if digest == previous_hash:
            identical_samples += 1
        else:
            previous_hash = digest
            identical_samples = 1
        if identical_samples >= required_identical_samples:
            return latest, {
                "method": "two consecutive identical framebuffer captures after the scene-bound draw",
                "settleDelayMs": settle_delay_ms,
                "sampleIntervalMs": sample_interval_ms,
                "requiredIdenticalSamples": required_identical_samples,
                "identicalSamples": identical_samples,
                "samplesTaken": samples,
            }
        time.sleep(sample_interval_ms / 1000)
    raise RuntimeError(f"{platform}/{scene_id}: screenshot did not stabilize within {timeout_seconds}s")


def run_android(scene_id: str, index: int) -> dict:
    run_id = f"{RUN_ID_PREFIX}-android-{index:02d}"
    launch_path = OUT / "android" / f"{scene_id}-launch.log"
    runtime_path = OUT / "android" / f"{scene_id}-runtime.log"
    screenshot_path = OUT / "android" / f"{scene_id}.png"
    launch = command(
        "adb", "-s", ANDROID_SERIAL, "shell", "am", "start", "-S", "-f", "0x10008000",
        "-n", f"{BUNDLE_ID}/.MainActivity", "-a", "android.intent.action.MAIN",
        "-c", "android.intent.category.LAUNCHER", "--ez", "spinon_c114_grid_track_sizing", "true",
        "--es", "spinon_c114_grid_scene", scene_id,
        "--es", "spinon_c114_grid_run_id", run_id,
    )
    launch_path.write_text(launch, encoding="utf-8")
    deadline = time.monotonic() + 60
    log = ""
    result = None
    last_error = "scene-bound runtime evidence is not complete"
    while time.monotonic() < deadline:
        log = command("adb", "-s", ANDROID_SERIAL, "logcat", "-d", "-v", "threadtime",
                      "SpinonBootstrap:D", "*:S", timeout=30)
        try:
            result = verify_log(log, run_id, scene_id, "android")
            break
        except RuntimeError as error:
            last_error = str(error)
        if "SPINON_C114_GRID_ASSERTION_ERROR" in log and scene_id in log:
            break
        time.sleep(1)
    runtime_path.write_text(log, encoding="utf-8")
    if result is None:
        raise RuntimeError(last_error)
    screenshot, screenshot_capture = stable_screenshot(
        lambda: subprocess.run(
            ["adb", "-s", ANDROID_SERIAL, "exec-out", "screencap", "-p"],
            cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True,
        ).stdout,
        "android",
        scene_id,
    )
    screenshot_path.write_bytes(screenshot)
    result.update({"screenshotCapture": screenshot_capture,
                   "runtimeLogSha256": sha256(log.encode()),
                   "launchLogSha256": sha256(launch.encode()),
                   "screenshotSha256": sha256(screenshot),
                   "screenshotPath": str(screenshot_path.relative_to(OUT))})
    return result


def run_ios(scene_id: str, index: int) -> dict:
    run_id = f"{RUN_ID_PREFIX}-ios-{index:02d}"
    launch_path = OUT / "ios-simulator" / f"{scene_id}-launch.log"
    runtime_path = OUT / "ios-simulator" / f"{scene_id}-unified.log"
    screenshot_path = OUT / "ios-simulator" / f"{scene_id}.png"
    launch = command(
        "xcrun", "simctl", "launch", "--terminate-running-process", IOS_UDID, BUNDLE_ID,
        "--spinon-c114-grid-track-sizing", f"--spinon-c114-grid-scene={scene_id}",
        f"--spinon-c114-grid-run-id={run_id}",
    )
    launch_path.write_text(launch, encoding="utf-8")
    deadline = time.monotonic() + 60
    log = ""
    result = None
    last_error = "scene-bound runtime evidence is not complete"
    while time.monotonic() < deadline:
        log = command("xcrun", "simctl", "spawn", IOS_UDID, "log", "show", "--style", "compact",
                      "--last", "2m", "--predicate", 'process == "SpinonBootstrap"', timeout=30)
        try:
            result = verify_log(log, run_id, scene_id, "ios-simulator")
            break
        except RuntimeError as error:
            last_error = str(error)
        if "SPINON_C114_GRID_ASSERTION_ERROR" in log and scene_id in log:
            break
        time.sleep(1)
    runtime_path.write_text(log, encoding="utf-8")
    if result is None:
        raise RuntimeError(last_error)

    temporary_screenshot = screenshot_path.with_name(screenshot_path.stem + ".capture.png")

    def capture_ios() -> bytes:
        command("xcrun", "simctl", "io", IOS_UDID, "screenshot", str(temporary_screenshot))
        if not temporary_screenshot.is_file() or temporary_screenshot.stat().st_size == 0:
            raise RuntimeError(f"ios-simulator/{scene_id}: screenshot command produced no file")
        return temporary_screenshot.read_bytes()

    screenshot, screenshot_capture = stable_screenshot(capture_ios, "ios-simulator", scene_id)
    temporary_screenshot.unlink(missing_ok=True)
    screenshot_path.write_bytes(screenshot)
    result.update({"screenshotCapture": screenshot_capture,
                   "runtimeLogSha256": sha256(log.encode()),
                   "launchLogSha256": sha256(launch.encode()),
                   "screenshotSha256": sha256(screenshot),
                   "screenshotPath": str(screenshot_path.relative_to(OUT))})
    return result


def verify_existing_matrix() -> None:
    metadata_path = OUT / "matrix-metadata.json"
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    source = source_snapshot()
    if metadata.get("source") != source or metadata.get("finalSource") != source:
        raise RuntimeError("saved simulator matrix source snapshots do not match the current implementation")
    if not metadata.get("runIdPrefix"):
        raise RuntimeError("saved simulator matrix has no unique run ID prefix")
    if sha256(ANDROID_APK.read_bytes()) != metadata.get("androidApkSha256"):
        raise RuntimeError("saved Android matrix APK hash does not match the current APK")
    if sha256(IOS_EXECUTABLE.read_bytes()) != metadata.get("iosExecutableSha256"):
        raise RuntimeError("saved iOS matrix executable hash does not match the current executable")

    for platform_key, directory, run_suffix, log_suffix, platform_name in (
        ("android", "android", "android", "runtime.log", "android"),
        ("iosSimulator", "ios-simulator", "ios", "unified.log", "ios-simulator"),
    ):
        observations = metadata["observations"][platform_key]
        if len(observations) != len(SCENES):
            raise RuntimeError(f"{platform_key}: expected {len(SCENES)} saved scenes, got {len(observations)}")
        seen_run_ids = set()
        for row in observations:
            scene_id = row["sceneId"]
            run_id = row["runId"]
            if run_id in seen_run_ids:
                raise RuntimeError(f"{platform_key}/{scene_id}: duplicate run ID {run_id}")
            seen_run_ids.add(run_id)
            if not run_id.startswith(metadata["runIdPrefix"] + f"-{run_suffix}-"):
                raise RuntimeError(f"{platform_key}/{scene_id}: run ID is outside this capture prefix")
            log_path = OUT / directory / f"{scene_id}-{log_suffix}"
            launch_path = OUT / directory / f"{scene_id}-launch.log"
            screenshot_path = OUT / row["screenshotPath"]
            log_bytes = log_path.read_bytes()
            launch_bytes = launch_path.read_bytes()
            screenshot = screenshot_path.read_bytes()
            if sha256(screenshot) != row.get("screenshotSha256"):
                raise RuntimeError(f"{platform_key}/{scene_id}: saved screenshot hash does not match metadata")
            capture = row.get("screenshotCapture", {})
            if capture.get("identicalSamples", 0) < 2:
                raise RuntimeError(f"{platform_key}/{scene_id}: stable screenshot capture evidence missing")

            result = verify_log(log_bytes.decode("utf-8", errors="replace"), run_id, scene_id, platform_name)
            row.update(result)
            row.update({
                "screenshotCapture": capture,
                "runtimeLogSha256": sha256(log_bytes),
                "launchLogSha256": sha256(launch_bytes),
                "screenshotSha256": sha256(screenshot),
                "screenshotPath": str(screenshot_path.relative_to(OUT)),
            })
        print(f"PASS verified saved {platform_key} matrix: {len(observations)}/{len(SCENES)}")

    metadata["runtimeEvidenceReverifiedAtUtc"] = dt.datetime.now(dt.timezone.utc).isoformat()
    metadata_path.write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
    print("PASS source, unique run IDs, process-bound frames/draws, stable screenshot hashes")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify-existing", action="store_true",
                        help="revalidate saved logs, screenshots, source snapshot and update matrix evidence")
    args = parser.parse_args()
    if args.verify_existing:
        verify_existing_matrix()
        return
    (OUT / "android").mkdir(parents=True, exist_ok=True)
    (OUT / "ios-simulator").mkdir(parents=True, exist_ok=True)
    initial = source_snapshot()
    apk = ANDROID_APK.read_bytes()
    app = IOS_EXECUTABLE.read_bytes()
    context = parse_device_context()
    metadata = {
        "generatedAtUtc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "runIdPrefix": RUN_ID_PREFIX,
        "scope": "22 fixed scenes per platform: 10 targets + 12 regression scenes",
        "source": initial,
        "androidApkSha256": sha256(apk),
        "iosExecutableSha256": sha256(app),
        "devices": context,
        "observations": {"android": [], "iosSimulator": []},
    }
    (OUT / "matrix-metadata.partial.json").write_text(json.dumps(metadata, indent=2) + "\n")

    android_install = write_command(OUT / "android" / "install.log",
                                    "adb", "-s", ANDROID_SERIAL, "install", "-r", str(ANDROID_APK))
    metadata["androidInstallCommandOutputSha256"] = sha256(android_install.encode())
    for index, scene_id in enumerate(SCENES, start=1):
        result = run_android(scene_id, index)
        metadata["observations"]["android"].append(result)
        (OUT / "matrix-metadata.partial.json").write_text(json.dumps(metadata, indent=2) + "\n")
        print(f"PASS Android {index}/22 {scene_id}", flush=True)

    ios_install = write_command(OUT / "ios-simulator" / "install.log",
                                "xcrun", "simctl", "install", IOS_UDID, str(IOS_APP))
    metadata["iosInstallCommandOutputSha256"] = sha256(ios_install.encode())
    for index, scene_id in enumerate(SCENES, start=1):
        result = run_ios(scene_id, index)
        metadata["observations"]["iosSimulator"].append(result)
        (OUT / "matrix-metadata.partial.json").write_text(json.dumps(metadata, indent=2) + "\n")
        print(f"PASS iOS Simulator {index}/22 {scene_id}", flush=True)

    final = source_snapshot()
    if final != initial:
        raise RuntimeError("implementation sources changed during simulator capture")
    metadata["finalSource"] = final
    metadata["completedAtUtc"] = dt.datetime.now(dt.timezone.utc).isoformat()
    metadata["counts"] = {"androidPassed": len(metadata["observations"]["android"]),
                           "iosSimulatorPassed": len(metadata["observations"]["iosSimulator"])}
    (OUT / "matrix-metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    (OUT / "matrix-metadata.partial.json").unlink(missing_ok=True)
    command("adb", "-s", ANDROID_SERIAL, "shell", "am", "force-stop", BUNDLE_ID)
    command("xcrun", "simctl", "terminate", IOS_UDID, BUNDLE_ID)
    print("PASS complete: 22/22 Android API 37 AVD + 22/22 iOS 26.2 Simulator", flush=True)


if __name__ == "__main__":
    main()
