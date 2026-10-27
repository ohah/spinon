#!/usr/bin/env python3
import json
from pathlib import Path

root = Path(__file__).parent
actual = json.loads((root / "chrome-observation.json").read_text())
expected = json.loads((root / "expected.json").read_text())
tol = expected["toleranceCssPx"]
states = actual["states"]
want = expected["states"]
assert "HeadlessChrome/156" in actual["userAgent"]
assert "Harness status: OK" in actual["wptHarness"]
assert "8 Pass" in actual["wptHarness"]
assert len(states) == len(want) == 8

def close(a, b, path):
    assert len(a) == len(b), f"{path}: length {len(a)} != {len(b)}"
    for index, (left, right) in enumerate(zip(a, b)):
        assert abs(float(left) - float(right)) <= tol, (
            f"{path}[{index}]: {left} != {right} within {tol}"
        )

for got, target in zip(states, want):
    label = target["id"]
    assert got["sizing"] == target["sizing"], f"{label}: sizing"
    assert got["itemWidth"] == f'{target["itemWidth"]}px', f"{label}: item width"
    assert got["grid"]["width"] == f'{target["rootContent"][0]}px', f"{label}: root width"
    assert got["grid"]["height"] == f'{target["rootContent"][1]}px', f"{label}: root height"
    columns = [float(value[:-2]) for value in got["grid"]["columns"].split()]
    close(columns, target["columns"], f"{label}: columns")
    assert len(columns) - 1 == target["repeatCount"], f"{label}: repeat count"
    close(
        [got["grid"]["frame"]["width"], got["grid"]["frame"]["height"]],
        target["rootBorderBox"],
        f"{label}: root border box",
    )
    close(
        [got["frames"]["item"][key] for key in ("x", "y", "width", "height")],
        target["item"],
        f"{label}: item frame",
    )
    for probe, frame in target["probes"].items():
        close(
            [got["frames"][probe][key] for key in ("x", "y", "width", "height")],
            frame,
            f"{label}: {probe}",
        )

print(json.dumps({
    "result": "pass",
    "browser": "HeadlessChrome/156",
    "wptHarness": "8/8",
    "states": len(states),
    "repeatCounts": [item["repeatCount"] for item in want],
    "maxAbsoluteCssPxError": 0,
    "scope": "WPT oracle and independent expected-value comparison only"
}, indent=2))
