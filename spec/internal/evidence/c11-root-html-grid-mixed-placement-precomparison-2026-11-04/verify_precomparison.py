#!/usr/bin/env python3
"""Compare the fixed mixed-placement browser observation with independent frames."""

import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
observation = json.loads((HERE / "chrome156-observation.json").read_text())
expected = json.loads((HERE / "expected.json").read_text())
scene_map = {scene["id"]: scene for scene in observation["scenes"]}
assert set(scene_map) == set(expected["scenes"])

frame_count = 0
for scene_id, contract in expected["scenes"].items():
    scene = scene_map[scene_id]
    assert scene["columns"] == "40px 50px 30px", (scene_id, scene["columns"])
    expected_rows = "20px 30px 40px 7px" if scene_id == "implicit-row-reject" else "20px 30px 40px"
    assert scene["rows"] == expected_rows, (scene_id, scene["rows"])
    children = {child["id"]: child for child in scene["children"]}
    assert set(children) == set(contract["frames"]), scene_id
    for item_id, expected_frame in contract["frames"].items():
        actual = children[item_id]["frame"]
        if expected_frame is None:
            assert actual is None, (scene_id, item_id, actual)
            continue
        assert actual is not None, (scene_id, item_id, actual)
        delta = max(abs(a - b) for a, b in zip(
            (actual["x"], actual["y"], actual["width"], actual["height"]),
            expected_frame,
        ))
        assert delta <= expected["toleranceCssPx"], (scene_id, item_id, delta)
        frame_count += 1

assert observation["viewport"] == {"width": 1280, "height": 633}
assert observation["devicePixelRatio"] == 1
assert "HeadlessChrome/156.0.0.0" in observation["userAgent"]
print(f"PASS scenes={len(scene_map)} frames={frame_count} tolerance_css_px={expected['toleranceCssPx']}")
