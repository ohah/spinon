import json
from pathlib import Path

ROOT = Path(__file__).parent
EPSILON = 1 / 64

expectations = json.loads((ROOT / "spec-derived-expectations.json").read_text())
observation = json.loads((ROOT / "chrome156-observation.json").read_text())
scenes = {scene["id"]: scene for scene in observation["scenes"]}

assert len(expectations["cases"]) == 13
assert len(scenes) == 13
assert observation["devicePixelRatio"] == 1
assert "HeadlessChrome/156.0.0.0" in observation["userAgent"]

for case in expectations["cases"]:
    scene = scenes[case["id"]]
    axis_values = scene["columns"] if case["axis"] == "columns" else scene["rows"]
    tracks = [float(value.removesuffix("px")) for value in axis_values]
    observed_count = len(tracks) - case["prefixTrackCount"] - case["suffixTrackCount"]
    assert observed_count == case["repeatCount"], (
        case["id"],
        observed_count,
        case["repeatCount"],
    )

    expected_tracks = case.get("chromeUsedTracksPx", case["usedTracksPx"])
    assert len(tracks) == len(expected_tracks), (case["id"], tracks, expected_tracks)
    assert all(
        abs(actual - expected) <= EPSILON
        for actual, expected in zip(tracks, expected_tracks)
    ), (case["id"], tracks, expected_tracks)

    if "itemFrameOuterPx" in case:
        item = scene["item"]
        actual_frame = [item["x"], item["y"], item["width"], item["height"]]
        assert actual_frame == case["itemFrameOuterPx"], (
            case["id"],
            actual_frame,
            case["itemFrameOuterPx"],
        )

print("13/13 scene ids, repeat counts, used tracks and item frames matched")
