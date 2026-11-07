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
    frames = expected["expectedFramesByGapCase"][gap_case]
    if scene["computedAlignContent"] != scene["inputAlignContent"]:
        errors.append(f"computed value mismatch for {gap_case}/{scene['inputAlignContent']}")
    expected_gap = "0px" if gap_case == "positive" else "calc(120% - 2px)"
    if scene["computedRowGap"] != expected_gap:
        errors.append(f"computed row-gap mismatch for {gap_case}/{scene['inputAlignContent']}")
    if scene["computedGridTemplateRows"] != "10px 10px 10px":
        errors.append(f"track sizing changed for {gap_case}/{scene['inputAlignContent']}: {scene['computedGridTemplateRows']}")
    if scene["container"] != {"width": 40, "height": 40}:
        errors.append(f"container mismatch for {gap_case}/{scene['inputAlignContent']}")
    if len(scene["itemFrames"]) != len(frames):
        errors.append(f"item count mismatch for {gap_case}/{scene['inputAlignContent']}")
        continue
    for actual, target in zip(scene["itemFrames"], frames):
        for key in ("x", "y", "width", "height"):
            error = abs(actual[key] - target[key])
            coordinate_count += 1
            max_error = max(max_error, error)
            if error > tolerance:
                errors.append(f"{gap_case}/{scene['inputAlignContent']} {key}: {actual[key]} != {target[key]}")

for gap_case in ("positive", "overflow"):
    same_gap = [scene for scene in observed["scenes"] if scene["gapCase"] == gap_case]
    baseline = next(scene for scene in same_gap if scene["inputAlignContent"] == "start")
    for alias in ("safe flex-start", "unsafe flex-start"):
        candidate = next(scene for scene in same_gap if scene["inputAlignContent"] == alias)
        if candidate["itemFrames"] != baseline["itemFrames"]:
            errors.append(f"{alias} differs from start in {gap_case}")

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
