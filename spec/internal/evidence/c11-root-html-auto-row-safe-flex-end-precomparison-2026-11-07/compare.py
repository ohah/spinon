#!/usr/bin/env python3
import json
from pathlib import Path

root = Path(__file__).parent
observed = json.loads((root / "chrome-observed.json").read_text())
expected = json.loads((root / "expected.json").read_text())
tolerance = expected["toleranceCssPx"]
errors = []
coordinate_count = 0
max_error = 0.0
if len(observed["scenes"]) != expected["sceneCount"]:
    errors.append(f"scene count {len(observed['scenes'])} != {expected['sceneCount']}")

for scene in observed["scenes"]:
    gap_case = scene["gapCase"]
    alignment = scene["inputAlignContent"]
    frames = expected["expectedFramesByAlignmentAndGapCase"][alignment][gap_case]
    if scene["computedAlignContent"] != alignment:
        errors.append(f"computed value mismatch for {gap_case}/{alignment}")
    expected_gap = "0px" if gap_case == "positive" else "calc(120% - 2px)"
    if scene["computedRowGap"] != expected_gap:
        errors.append(f"computed row-gap mismatch for {gap_case}/{alignment}")
    if scene["computedGridTemplateRows"] != "10px 10px 10px":
        errors.append(f"track sizing changed for {gap_case}/{alignment}: {scene['computedGridTemplateRows']}")
    if scene["container"] != {"width": 40, "height": 40}:
        errors.append(f"container mismatch for {gap_case}/{alignment}")
    if len(scene["itemFrames"]) != len(frames):
        errors.append(f"item count mismatch for {gap_case}/{alignment}")
        continue
    for actual, target in zip(scene["itemFrames"], frames):
        for key in ("x", "y", "width", "height"):
            error = abs(actual[key] - target[key])
            coordinate_count += 1
            max_error = max(max_error, error)
            if error > tolerance:
                errors.append(f"{gap_case}/{alignment} {key}: {actual[key]} != {target[key]}")

for gap_case in ("positive", "overflow"):
    scene_by_alignment = {scene["inputAlignContent"]: scene for scene in observed["scenes"] if scene["gapCase"] == gap_case}
    end = scene_by_alignment["end"]["itemFrames"]
    unsafe = scene_by_alignment["unsafe flex-end"]["itemFrames"]
    if unsafe != end:
        errors.append(f"unsafe flex-end differs from end in {gap_case}")
    safe = scene_by_alignment["safe flex-end"]["itemFrames"]
    if gap_case == "positive" and safe != end:
        errors.append("safe flex-end differs from end in positive free space")
    if gap_case == "overflow" and [frame["y"] for frame in safe] != [0, 56, 112]:
        errors.append("safe flex-end did not fall back to start in overflow")

result = {
    "passed": not errors,
    "sceneCount": len(observed["scenes"]),
    "coordinateCount": coordinate_count,
    "maximumAbsoluteErrorCssPx": max_error,
    "toleranceCssPx": tolerance,
    "errors": errors,
}
(root / "comparison.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
raise SystemExit(0 if not errors else 1)
