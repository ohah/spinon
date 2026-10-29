#!/usr/bin/env python3
"""고정된 Chrome 관찰값을 독립 반복 수·배치 규칙과 대조한다."""

import json
from pathlib import Path


evidence_dir = Path(__file__).resolve().parent
observed = json.loads((evidence_dir / "chrome-156-observations.json").read_text())
assert "HeadlessChrome/156.0.0.0" in observed["userAgent"]
assert len(observed["scenes"]) == 24

seen = set()
for scene in observed["scenes"]:
    axis = scene["axis"]
    mode = scene["mode"]
    minimum = scene["minimum"]
    gap = scene["gap"]
    assert axis in {"columns", "rows"}
    assert mode in {"auto-fill", "auto-fit"}
    assert minimum in {"auto", "min-content", "max-content"}
    assert gap in {0, 4}
    assert scene["maximum"] == "0px"
    assert scene["basis"] == 16
    key = (axis, mode, minimum, gap)
    assert key not in seen, key
    seen.add(key)

    repeated_values = (
        scene["gridTemplateColumns"] if axis == "columns" else scene["gridTemplateRows"]
    ).split()
    expected_count = (scene["basis"] + gap) // (1 + gap)
    assert scene["repeatedTrackCount"] == expected_count, scene["id"]
    assert len(repeated_values) == expected_count, scene["id"]
    assert repeated_values[2] == "12px", scene["id"]
    assert all(value == "0px" for index, value in enumerate(repeated_values) if index != 2)

    root = scene["root"]
    item = scene["item"]
    assert root["width"] == (16 if axis == "columns" else 40), scene["id"]
    assert root["height"] == (16 if axis == "rows" else 40), scene["id"]
    assert item["width"] == item["height"] == 12, scene["id"]
    expected_offset = 2 * gap if mode == "auto-fill" else 0
    actual_offset = scene["itemOffset"]["x" if axis == "columns" else "y"]
    assert actual_offset == expected_offset, scene["id"]

assert len(seen) == 24
print("PASS: 24 Chrome scenes match the independent 1px repeat-count floor and offset rules")
