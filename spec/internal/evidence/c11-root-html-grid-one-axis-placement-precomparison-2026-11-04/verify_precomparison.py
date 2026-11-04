#!/usr/bin/env python3
import json
from pathlib import Path

root = Path(__file__).resolve().parent
expected = json.loads((root / "expected.json").read_text())
observed = json.loads((root / "chrome156-observation.json").read_text())
assert "HeadlessChrome/156." in observed["userAgent"]
assert observed["devicePixelRatio"] == 1
assert observed["viewport"] == {"width": 1280, "height": 633}
assert {s["id"] for s in observed["scenes"]} == set(expected["scenes"])
max_error = 0.0
frame_count = 0
for scene in observed["scenes"]:
    case = expected["scenes"][scene["id"]]
    assert scene["columns"] == case["tracks"]["columns"], scene["id"]
    assert scene["rows"] == case["tracks"]["rows"], scene["id"]
    actual_items = {item["id"]: item for item in scene["children"]}
    assert actual_items.keys() == case["items"].keys(), scene["id"]
    for item_id, wanted in case["items"].items():
        actual = actual_items[item_id]
        for prop, value in wanted["computed"].items():
            assert actual[prop] == value, (scene["id"], item_id, prop, actual[prop], value)
        got = actual["frame"]
        for field, value in zip(("x", "y", "width", "height"), wanted["frame"]):
            error = abs(got[field] - value)
            max_error = max(max_error, error)
            assert error <= expected["toleranceCssPx"], (scene["id"], item_id, field, got[field], value)
        frame_count += 1
accepted = sum(v["runtimeDisposition"].startswith("accept") for v in expected["scenes"].values())
rejected = sum(v["runtimeDisposition"].startswith("reject") for v in expected["scenes"].values())
print(f"PASS scenes={len(expected['scenes'])} frames={frame_count} accepted={accepted} expected_rejection={rejected} max_abs_error_css_px={max_error}")
