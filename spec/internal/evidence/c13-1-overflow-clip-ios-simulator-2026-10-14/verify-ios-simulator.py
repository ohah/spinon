#!/usr/bin/env python3
"""Compare C13.1 iOS Simulator logs with the pinned Chrome 155 oracle."""

import json
import re
import sys
from pathlib import Path


HERE = Path(__file__).resolve().parent
LOG_PATH = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "ios-simulator.log"
ORACLE_PATH = (
    HERE.parent / "c13-1-overflow-clip-android-avd-2026-10-14"
    / "chrome155-runtime-observation.json"
)
NUMBER = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
FRAME = re.compile(
    rf"SPINON_C131_NODE_FRAME (\d+):node=(\d+),x=({NUMBER}),y=({NUMBER}),"
    rf"width=({NUMBER}),height=({NUMBER})"
)
HIT = re.compile(
    r"SPINON_C131_HIT_TEST id=(\S+) x=(\S+) y=(\S+) "
    r"element=(\S+) status=(\S+)"
)


def fail(message):
    raise SystemExit(f"FAIL: {message}")


log = LOG_PATH.read_text()
oracle = json.loads(ORACLE_PATH.read_text())
if oracle["browser"]["version"] != "155.0.8059.40":
    fail(f"unexpected Chrome oracle version: {oracle['browser']['version']}")

summary = re.search(
    r"SPINON_C131_FIXTURE_SUMMARY status=(\w+) boxes=(\d+) frames=(\d+) "
    r"hit_samples=(\d+) hit_matches=(\d+)",
    log,
)
if not summary:
    fail("fixture summary marker is missing")
status, boxes, frame_count, hit_count, hit_matches = summary.groups()
if (status, boxes, frame_count, hit_count, hit_matches) != ("ok", "13", "13", "15", "15"):
    fail(f"unexpected fixture summary: {summary.group(0)}")

actual_frames = [
    {
        "index": int(match[1]),
        "node": int(match[2]),
        "x": float(match[3]),
        "y": float(match[4]),
        "width": float(match[5]),
        "height": float(match[6]),
    }
    for match in FRAME.finditer(log)
]
expected_frames = oracle["frames"]
if len(actual_frames) != len(expected_frames) or len(actual_frames) != 13:
    fail(f"frame count mismatch: iOS={len(actual_frames)} Chrome={len(expected_frames)}")

max_error = 0.0
for index, (actual, expected) in enumerate(zip(actual_frames, expected_frames)):
    if actual["index"] != index or actual["node"] != index + 1:
        fail(f"unexpected iOS frame ordering at index {index}: {actual}")
    for field in ("x", "y", "width", "height"):
        error = abs(actual[field] - float(expected[field]))
        max_error = max(max_error, error)
        if error > 0.5:
            fail(
                f"frame {index} {expected['id']}.{field}: "
                f"iOS={actual[field]} Chrome={expected[field]} error={error}"
            )

actual_hits = [
    {
        "id": match[1],
        "xCssPx": float(match[2]),
        "yCssPx": float(match[3]),
        "elementId": None if match[4] == "null" else match[4],
        "status": match[5],
    }
    for match in HIT.finditer(log)
]
expected_hits = oracle["hitTestSamples"]
if len(actual_hits) != len(expected_hits) or len(actual_hits) != 15:
    fail(f"hit sample count mismatch: iOS={len(actual_hits)} Chrome={len(expected_hits)}")
for index, (actual, expected) in enumerate(zip(actual_hits, expected_hits)):
    for field in ("id", "elementId"):
        if actual[field] != expected[field]:
            fail(f"hit sample {index} {field}: iOS={actual[field]} Chrome={expected[field]}")
    for field in ("xCssPx", "yCssPx"):
        if actual[field] != float(expected[field]):
            fail(f"hit sample {index} {field}: iOS={actual[field]} Chrome={expected[field]}")
    if actual["status"] != "matched":
        fail(f"hit sample {index} was not matched: {actual}")

print(f"PASS: iPhone 17 Pro Simulator; Chrome {oracle['browser']['version']} oracle")
print("Runtime: status=0 layout=ready boxes=13")
print(f"Geometry: {len(actual_frames)}/13 frames; max absolute error={max_error:.3f} CSS px (limit 0.500)")
print("Hit testing: 15/15 fixed samples match IDs, coordinates, and elements")
print("Negative controls: viewport left/top/right/bottom outside points return null")
for actual, expected in zip(actual_frames, expected_frames):
    print(
        f"frame {actual['index']:02} {expected['id']}: "
        f"x={actual['x']:g} y={actual['y']:g} "
        f"w={actual['width']:g} h={actual['height']:g}"
    )
