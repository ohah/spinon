#!/usr/bin/env python3
import json
from pathlib import Path

root = Path(__file__).resolve().parent
expected = json.loads((root / "expected.json").read_text())
observed = json.loads((root / "chrome156-observation.json").read_text())
assert "HeadlessChrome/156." in observed["userAgent"]
assert observed["devicePixelRatio"] == 1
assert observed["viewport"] == {"width": 1280, "height": 633}
assert {scene["id"] for scene in observed["scenes"]} == set(expected["scenes"])

max_error = 0.0
frame_count = 0
item_count = 0


def verify_grid(scene_id, actual_grid, expected_grid):
    global max_error, frame_count, item_count
    if "id" in expected_grid:
        assert actual_grid["id"] == expected_grid["id"], scene_id
    if "tracks" in expected_grid:
        tracks = expected_grid["tracks"]
    else:
        tracks = {"columns": expected_grid["columns"], "rows": expected_grid["rows"]}
    assert actual_grid["columns"] == tracks["columns"], scene_id
    assert actual_grid["rows"] == tracks["rows"], scene_id
    expected_items = expected_grid.get("items", expected_grid.get("children"))
    actual_items = {item["id"]: item for item in actual_grid["children"]}
    assert actual_items.keys() == expected_items.keys(), scene_id
    for item_id, wanted in expected_items.items():
        item_count += 1
        actual = actual_items[item_id]
        for prop, value in wanted["computed"].items():
            assert actual[prop] == value, (scene_id, item_id, prop, actual[prop], value)
        got = actual["frame"]
        if wanted["frame"] is None:
            assert got is None, (scene_id, item_id, got)
            continue
        assert got is not None, (scene_id, item_id)
        for field, value in zip(("x", "y", "width", "height"), wanted["frame"]):
            error = abs(got[field] - value)
            max_error = max(max_error, error)
            assert error <= expected["toleranceCssPx"], (scene_id, item_id, field, got, value)
        frame_count += 1


for scene in observed["scenes"]:
    case = expected["scenes"][scene["id"]]
    if "gridOwners" in case:
        actual_owners = {grid["id"]: grid for grid in scene["gridOwners"]}
        assert actual_owners.keys() == {grid["id"] for grid in case["gridOwners"]}, scene["id"]
        for owner in case["gridOwners"]:
            verify_grid(scene["id"], actual_owners[owner["id"]], owner)
    else:
        verify_grid(scene["id"], scene, case)

accepted = sum(v["runtimeDisposition"].startswith("accept") for v in expected["scenes"].values())
rejected = sum(v["runtimeDisposition"].startswith("reject") for v in expected["scenes"].values())
print(
    f"PASS scenes={len(expected['scenes'])} items={item_count} frames={frame_count} accepted={accepted} "
    f"expected_rejection={rejected} max_abs_error_css_px={max_error}"
)
