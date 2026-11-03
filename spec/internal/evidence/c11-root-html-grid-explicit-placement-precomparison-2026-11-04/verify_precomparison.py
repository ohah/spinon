#!/usr/bin/env python3
"""Compare the saved Agent Browser observation with independent CSS expectations."""

import json
import hashlib
import struct
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
observation = json.loads((HERE / "chrome156-observation.json").read_text())
expected = json.loads((HERE / "expected.json").read_text())
manifest = json.loads((HERE / "input.json").read_text())
actual_scenes = {scene["id"]: scene for scene in observation["scenes"]}
expected_scenes = expected["scenes"]
assert set(actual_scenes) == set(expected_scenes), "scene inventory changed"
assert observation["viewport"] == {"width": 1280, "height": 633}
assert observation["devicePixelRatio"] == 1
assert "HeadlessChrome/156." in observation["userAgent"]

for artifact in manifest["artifacts"].values():
    data = (REPO / artifact["path"]).read_bytes()
    assert hashlib.sha256(data).hexdigest() == artifact["sha256"], artifact["path"]
screenshot = (REPO / manifest["artifacts"]["screenshot"]["path"]).read_bytes()
assert screenshot[:8] == b"\x89PNG\r\n\x1a\n"
assert struct.unpack(">II", screenshot[16:24]) == (1280, 640)

checked_frames = 0
for scene_id, expected_scene in expected_scenes.items():
    actual_scene = actual_scenes[scene_id]
    children = {child["id"]: child for child in actual_scene["children"]}
    for child_id, properties in expected_scene.get("computed", {}).items():
        child = children[child_id]
        for property_name, expected_value in properties.items():
            assert child[property_name] == expected_value, (
                scene_id,
                child_id,
                property_name,
                expected_value,
                child[property_name],
            )

    for child_id, frame in {
        **expected_scene.get("frames", {}),
        **expected_scene.get("chromeFrames", {}),
    }.items():
        actual_frame = children[child_id]["frame"]
        actual_values = [
            actual_frame["x"],
            actual_frame["y"],
            actual_frame["width"],
            actual_frame["height"],
        ]
        assert len(actual_values) == len(frame)
        assert all(
            abs(actual - wanted) <= expected["toleranceCssPx"]
            for actual, wanted in zip(actual_values, frame)
        ), (scene_id, child_id, frame, actual_values)
        checked_frames += 1

    for child_id in expected_scene.get("excludedItems", []):
        assert children[child_id]["display"] == "none", (scene_id, child_id)
    if "anonymousTextItemCount" in expected_scene:
        assert len(actual_scene["textNodes"]) == expected_scene["anonymousTextItemCount"]
    if "topItemAtFirstCell" in expected_scene:
        assert actual_scene["topItemAtFirstCell"] == expected_scene["topItemAtFirstCell"]
    if "chromeRows" in expected_scene:
        assert actual_scene["rows"] == expected_scene["chromeRows"]

print(
    f"PASS: {len(actual_scenes)} scenes, {checked_frames} item frames, "
    "computed longhands, hidden/text controls, overlap hit-test, and SHA-256 manifest"
)
