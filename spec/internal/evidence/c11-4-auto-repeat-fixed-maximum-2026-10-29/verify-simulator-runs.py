#!/usr/bin/env python3
"""Check the retained 49-scene Android and iOS simulator logs."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
FIXTURE = ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_maximum_scenes.json"

RUNS = {
    "android-avd": EVIDENCE / "android-avd-c114-auto-repeat-fixed-maximum-android-avd-20261029T064318832Z-6620d4a9",
    "ios-simulator": EVIDENCE / "ios-simulator-c114-auto-repeat-fixed-maximum-ios-simulator-20261029T064625210Z-bb79b8bb",
}
REPRESENTATIVE_SCREENSHOTS = {
    "android-avd": {
        "columns-auto-fill-auto-px40-130.png",
        "rows-overflow-auto-max10-basis10.png",
    },
    "ios-simulator": {
        "columns-auto-fill-auto-px40-130.png",
        "rows-overflow-auto-max10-basis10.png",
    },
}


def verify_run(target: str, run_dir: Path, expected_ids: set[str]) -> dict[str, object]:
    metadata = json.loads((run_dir / "simulator.json").read_text())
    result = json.loads((run_dir / "result.json").read_text())
    screenshot_manifest = json.loads((run_dir / "screenshot-manifest.json").read_text())
    observations = result["observations"]
    scene_ids = [entry["sceneId"] for entry in observations]
    assert result["sceneCount"] == 49
    assert len(scene_ids) == 49 and set(scene_ids) == expected_ids
    assert all(entry["drawPresented"] for entry in observations)
    retained_screenshots = REPRESENTATIVE_SCREENSHOTS[target]
    screenshots = screenshot_manifest["screenshots"]
    assert len(screenshots) == 49
    assert {entry["file"] for entry in screenshots} == {f"{scene_id}.png" for scene_id in expected_ids}
    screenshot_by_name = {entry["file"]: entry for entry in screenshots}
    for name in retained_screenshots:
        image_path = run_dir / name
        assert image_path.is_file()
        digest = hashlib.sha256(image_path.read_bytes()).hexdigest()
        assert digest == screenshot_by_name[name]["sha256"]

    if metadata["target"] == "android-avd":
        assert target == "android-avd"
        assert metadata["apiLevel"] == 37 and metadata["abi"] == "arm64-v8a"
        assert metadata["apkSha256"] == metadata["installedApkSha256"]
        summary_prefix = "SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched"
        sample_prefix = "SPINON_C114_GRID_PIXEL_MATRIX_SAMPLE"
        sample_pattern = re.compile(r"\bexact=true\b")
        count_pattern = re.compile(
            r"expected_samples=(\d+) observed_samples=(\d+) matched_samples=(\d+)"
        )
    else:
        assert target == "ios-simulator"
        assert metadata["modelIdentifier"] == "iPhone18,1"
        assert metadata["runtime"] == "com.apple.CoreSimulator.SimRuntime.iOS-26-2"
        assert metadata["appSha256"] == metadata["installedAppSha256"]
        summary_prefix = "SPINON_C114_METAL_PIXEL_SUMMARY status=matched"
        sample_prefix = "SPINON_C114_METAL_PIXEL_SAMPLE"
        sample_pattern = re.compile(r"\bexact=true\b")
        count_pattern = re.compile(
            r"expected_samples=(\d+) observed_samples=(\d+) matched_samples=(\d+)"
        )

    total_expected = 0
    total_observed = 0
    total_matched = 0
    exact_samples = 0
    completed_scenes: set[str] = set()
    for log_path in run_dir.glob("runtime-*.log"):
        log = log_path.read_text(errors="replace")
        assert "SPINON_C114_GRID_SUMMARY" in log and "status=0" in log, log_path
        if target == "android-avd":
            scene_match = re.search(r"SPINON_C114_GRID_SUMMARY .*?scene_id=([^ ]+)", log)
        else:
            scene_match = re.search(r"SPINON_C114_GRID_SUMMARY .*?scene_id=([^ ]+)", log)
        assert scene_match, log_path
        completed_scenes.add(scene_match.group(1))
        samples = [line for line in log.splitlines() if sample_prefix in line]
        exact_samples += sum(bool(sample_pattern.search(line)) for line in samples)
        summaries = [line for line in log.splitlines() if summary_prefix in line]
        assert len(summaries) == 1, (log_path, summaries)
        counts = count_pattern.search(summaries[0])
        assert counts, summaries[0]
        expected, observed, matched = map(int, counts.groups())
        assert expected == observed == matched == len(samples)
        total_expected += expected
        total_observed += observed
        total_matched += matched

    assert completed_scenes == expected_ids
    assert len(list(run_dir.glob("runtime-*.log"))) == 49
    assert total_expected == total_observed == total_matched == exact_samples == 104
    return {
        "target": target,
        "runPrefix": result["runPrefix"],
        "sceneCount": len(completed_scenes),
        "allDrawPresented": True,
        "pixelExpected": total_expected,
        "pixelObserved": total_observed,
        "pixelMatched": total_matched,
        "pixelExact": exact_samples,
        "pixelMismatches": 0,
        "sceneScreenshotsCaptured": len(screenshots),
        "representativeScreenshotsRetained": len(retained_screenshots),
        "sourceRevision": metadata["sourceRevision"],
        "installedBinarySha256": metadata.get("installedApkSha256", metadata.get("installedAppSha256")),
    }


def main() -> None:
    scenes = json.loads(FIXTURE.read_text())
    ids = {scene["id"] for scene in scenes}
    assert len(ids) == 49
    summary = {target: verify_run(target, run_dir, ids) for target, run_dir in RUNS.items()}
    print(json.dumps(summary, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
