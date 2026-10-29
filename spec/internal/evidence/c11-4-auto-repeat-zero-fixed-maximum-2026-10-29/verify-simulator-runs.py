#!/usr/bin/env python3
"""Verify the fixed 24-scene Android/iOS C11.4 simulator captures."""

import hashlib
import json
import struct
from pathlib import Path


ROOT = Path(__file__).resolve().parent
REPOSITORY = ROOT.parents[3]
FIXTURE_PATH = REPOSITORY / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_fixed_maximum_scenes.json"
CHROME_PATH = REPOSITORY / "spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/chrome-156-observations.json"
SOURCE_PATHS = {
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_fixed_maximum_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_zero_fixed_maximum.rs",
    "crates/spinon-layout/src/grid_track_sizing_tests/auto_repeat_zero_fixed_maximum.rs",
}
RUN_PREFIXES = {
    "android-avd": "android-avd-c114-zero-fixed-maximum-android-avd-",
    "ios-simulator": "ios-simulator-c114-zero-fixed-maximum-ios-simulator-",
}
PIXEL_MARKERS = {
    "android-avd": "SPINON_C114_GRID_PIXEL_MATRIX_SAMPLE",
    "ios-simulator": "SPINON_C114_METAL_PIXEL_SAMPLE",
}


def read_json(path):
    return json.loads(path.read_text())


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def find_run(target, prefix):
    matches = sorted(path for path in ROOT.glob(f"{prefix}*") if path.is_dir())
    assert matches, f"{target}: no run directory found"
    return max(matches, key=lambda path: path.name)


def png_dimensions(path):
    data = path.read_bytes()
    assert data.startswith(b"\x89PNG\r\n\x1a\n"), f"invalid PNG: {path}"
    return struct.unpack(">II", data[16:24])


def parse_sample_line(line, target, scene_id):
    fields = {
        key: value
        for token in line.split()
        if "=" in token
        for key, value in [token.split("=", 1)]
    }
    rgba = [int(channel) for channel in fields["rgba"].split(",")]
    expected = [int(channel) for channel in fields["expected"].split(",")]
    assert len(rgba) == len(expected) == 4
    point = {
        key: float(fields[key]) if key.startswith("css_") else int(fields[key])
        for key in ("css_x", "css_y", "x", "y")
        if key in fields
    }
    return {
        "sceneId": scene_id,
        "sampleId": fields["sample"],
        "targetNodeId": fields.get("target_node_id"),
        "surfacePoint": point,
        "rgba": rgba,
        "expectedRgba": expected,
        "exact": fields.get("exact") == "true",
    }


fixtures = read_json(FIXTURE_PATH)
chrome = read_json(CHROME_PATH)
fixture_ids = [scene["id"] for scene in fixtures]
chrome_ids = [scene["id"] for scene in chrome["scenes"]]
assert len(fixtures) == len(chrome["scenes"]) == 24
assert fixture_ids == chrome_ids

verification = {
    "schemaVersion": 1,
    "chromeUserAgent": chrome["userAgent"],
    "sceneCount": len(fixtures),
    "targets": {},
}
receipts = {}
surface_samples_path = ROOT / "surface-samples.json"
existing_surface_samples = (
    read_json(surface_samples_path) if surface_samples_path.is_file() else None
)
surface_samples = {target: [] for target in RUN_PREFIXES}

for target, prefix in RUN_PREFIXES.items():
    run = find_run(target, prefix)
    result = read_json(run / "result.json")
    metadata_path = run / "simulator-sanitized.json"
    if not metadata_path.is_file():
        metadata_path = run / "simulator.json"
    metadata = read_json(metadata_path)
    assert result["target"] == target
    assert result["sceneCount"] == 24
    assert [item["sceneId"] for item in result["observations"]] == fixture_ids

    if target == "android-avd":
        assert metadata["qemu"] == "1"
        assert metadata["apiLevel"] == 37
        assert metadata["abi"] == "arm64-v8a"
        assert metadata["apkSha256"] == metadata["installedApkSha256"]
        install = read_json(run / "installed-apk-verification.json")
        assert install["matched"]
        pixel_summary = "SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched"
    else:
        assert metadata["name"] == "iPhone 17 Pro"
        assert metadata["modelIdentifier"] == "iPhone18,1"
        assert metadata["runtime"].endswith("iOS-26-2")
        assert metadata["appSha256"] == metadata["installedAppSha256"]
        pixel_summary = "SPINON_C114_METAL_PIXEL_SUMMARY status=matched"

    screenshot_manifest = []
    total_samples = 0
    total_trace_chunks = 0
    for scene, observation in zip(fixtures, result["observations"]):
        scene_id = scene["id"]
        assert observation["drawPresented"]
        assert observation["trackTraceChunks"] > 0
        total_trace_chunks += observation["trackTraceChunks"]
        expected_root = scene["expect"]["root"]
        expected_item = scene["expect"]["items"][0]["frame"]
        frames = observation["observedNodeFrames"]
        assert len(frames) == 2, f"{target}/{scene_id}: node count"
        assert [frames[0][key] for key in ("x", "y", "width", "height")] == [0, 0, *expected_root]
        assert [frames[1][key] for key in ("x", "y", "width", "height")] == expected_item

        for axis, axis_name in (("columns", "Columns"), ("rows", "Rows")):
            actual_tracks = observation["tracks"][axis_name]
            expected_tracks = scene["expect"]["tracks"][axis]
            expected_collapsed = scene["expect"]["collapsed"][axis]
            assert [track["baseSize"] for track in actual_tracks] == expected_tracks, f"{target}/{scene_id}/{axis}: tracks"
            assert [track["collapsed"] for track in actual_tracks] == expected_collapsed, f"{target}/{scene_id}/{axis}: collapse"

        expected_sample_ids = [f"{scene_id}-root-interior", f"{scene_id}-item-interior"]
        if scene["mode"] == "auto-fill" and scene["gap"] == 4:
            expected_sample_ids.append(f"{scene_id}-item-outside-root")
        log_path = run / f"runtime-{scene_id}.log"
        if log_path.is_file():
            log = log_path.read_text()
            assert pixel_summary in log, f"{target}/{scene_id}: missing matched summary"
            assert "status=mismatch" not in log and "status=error" not in log
            sample_lines = [line for line in log.splitlines() if PIXEL_MARKERS[target] in line]
            scene_samples = [parse_sample_line(line, target, scene_id) for line in sample_lines]
        else:
            assert existing_surface_samples is not None, f"{target}/{scene_id}: sample evidence missing"
            scene_samples = [
                sample for sample in existing_surface_samples["targets"][target]["samples"]
                if sample["sceneId"] == scene_id
            ]
        assert len(scene_samples) == len(expected_sample_ids), f"{target}/{scene_id}: sample count"
        assert {sample["sampleId"] for sample in scene_samples} == set(expected_sample_ids), (
            f"{target}/{scene_id}: sample IDs"
        )
        assert all(sample["exact"] for sample in scene_samples), f"{target}/{scene_id}: pixel mismatch"
        surface_samples[target].extend(scene_samples)
        total_samples += len(scene_samples)

        screenshot = run / f"{scene_id}.png"
        width, height = png_dimensions(screenshot)
        screenshot_manifest.append({
            "file": screenshot.name,
            "bytes": screenshot.stat().st_size,
            "width": width,
            "height": height,
            "sha256": sha256(screenshot),
        })

    assert len(screenshot_manifest) == 24
    assert total_samples == 54
    (run / "screenshot-manifest.json").write_text(json.dumps({
        "target": target,
        "sceneCount": 24,
        "screenshots": screenshot_manifest,
    }, indent=2) + "\n")

    source = metadata["sourceRevision"]
    relevant_untracked = [
        item for item in source["untrackedFiles"] if item["path"] in SOURCE_PATHS
    ]
    assert {item["path"] for item in relevant_untracked} == SOURCE_PATHS
    receipts[target] = {
        "device": {
            key: metadata[key]
            for key in ("serial", "model", "apiLevel", "abi", "udid", "name", "modelIdentifier", "runtime")
            if key in metadata
        },
        "appSha256": metadata.get("apkSha256", metadata.get("appSha256")),
        "installedAppSha256": metadata.get("installedApkSha256", metadata.get("installedAppSha256")),
        "sourceRevision": {
            "commit": source["commit"],
            "tree": source["tree"],
            "trackedDiffSha256": source["trackedDiffSha256"],
            "trackedDirtyPaths": source["trackedDirtyPaths"],
            "relevantUntrackedSources": relevant_untracked,
            "workspaceUntrackedSnapshotSha256": source["sourceSnapshotSha256"],
            "omittedUnrelatedUntrackedFileCount": source.get(
                "omittedUnrelatedUntrackedFileCount",
                len(source["untrackedFiles"]) - len(relevant_untracked),
            ),
        },
    }
    verification["targets"][target] = {
        "runDirectory": run.name,
        "sceneCount": 24,
        "drawPresented": 24,
        "traceChunks": total_trace_chunks,
        "pixelSamplesExact": total_samples,
        "screenshots": 24,
        "appInstalledHashMatches": True,
    }

(surface_samples_path).write_text(json.dumps({
    "schemaVersion": 1,
    "targets": {
        target: {
            "readback": "PixelCopy" if target == "android-avd" else "Metal drawable before compositor",
            "sampleCount": len(samples),
            "exactCount": sum(sample["exact"] for sample in samples),
            "samples": samples,
        }
        for target, samples in surface_samples.items()
    },
}, indent=2) + "\n")
(ROOT / "simulator-source-revisions.json").write_text(json.dumps(receipts, indent=2) + "\n")
(ROOT / "verification.json").write_text(json.dumps(verification, indent=2) + "\n")
print(json.dumps(verification, indent=2))
