#!/usr/bin/env python3
"""Check the pinned Chrome capture against independently derived Grid frames."""
import json
from pathlib import Path

root = Path(__file__).parent
capture = json.loads((root / "capture.json").read_text())
expected = json.loads((root / "expected.json").read_text())
assert capture["userAgent"].find(expected["oracle"]["browser"]) >= 0
assert capture["viewport"] == expected["oracle"]["viewportCssPx"]
assert capture["dpr"] == expected["oracle"]["devicePixelRatio"]
scenes = {scene["id"]: {item["id"]: item for item in scene["items"]} for scene in capture["scenes"]}
for case in expected["independentExpectations"]:
    items = scenes[case["scene"]]
    observed_key = "framesCssPx" if case["product"] == "positive" else "browserFramesCssPx"
    for item_id, frame in case[observed_key].items():
        observed = items[item_id]
        actual = [observed["x"], observed["y"], observed["width"], observed["height"]]
        assert actual == frame, (case["scene"], item_id, frame, actual)
    for item_id in case.get("nonParticipating", []):
        assert items[item_id]["display"] == "none", (case["scene"], item_id)
assert len(expected["independentExpectations"]) == 6
print("PASS: 6 scenes; 22 positive frames and 2 implicit-boundary frames match independent expectations")
