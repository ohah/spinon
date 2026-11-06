#!/usr/bin/env python3
"""Check the 120%/center Grid overflow in simulator screenshot pixels."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys

from PIL import Image

ROOT = Path(__file__).resolve().parent
PASS_NAME = sys.argv[1] if len(sys.argv) > 1 else "pass02"
if not PASS_NAME.startswith("pass") or not PASS_NAME[4:].isdigit():
    raise SystemExit("pass name must use passNN format")
PASS_DIR = ROOT / PASS_NAME
CASES = {
    "android": {"scale": 2.625, "device": "Android API 37 ARM64 AVD"},
    "ios": {"scale": 3.0, "device": "iPhone 17 Pro / iOS 26.2 Simulator"},
}
GRID = (233, 237, 242)
TEXT_A = (91, 159, 245)
TEXT_B = (244, 162, 97)


def bbox(image: Image.Image, color: tuple[int, int, int]) -> list[int]:
    width, height = image.size
    pixels = image.load()
    xs: list[int] = []
    ys: list[int] = []
    for y in range(min(height, 600)):
        for x in range(min(width, 220)):
            if pixels[x, y] == color:
                xs.append(x)
                ys.append(y)
    if not xs:
        raise AssertionError(f"missing exact screenshot color {color}")
    return [min(xs), min(ys), max(xs), max(ys)]


results: dict[str, object] = {
    "purpose": "Verify that the rendered last Grid item is below the 40px root in the simulator PNG.",
    "expected": {
        "gridSizeCssPx": [40, 40],
        "rowGapPercent": 120,
        "alignContent": "center",
        "lastItemTopRelativeToGridCssPx": 73,
        "classification": "simulator screenshot pixel observation; not PixelCopy or Metal drawable readback",
    },
    "platforms": {},
}

for platform, details in CASES.items():
    path = PASS_DIR / platform / "simulator.png"
    image = Image.open(path).convert("RGB")
    grid = bbox(image, GRID)
    text_a = bbox(image, TEXT_A)
    text_b = bbox(image, TEXT_B)
    scale = details["scale"]
    expected_root_bottom = grid[1] + 40 * scale
    text_a_offset = (text_a[1] - grid[1]) / scale
    text_b_offset = (text_b[1] - grid[1]) / scale
    text_b_is_outside = text_b[1] >= expected_root_bottom
    if not text_b_is_outside:
        raise AssertionError(f"{platform}: last item begins inside the Grid root")
    if abs(text_b_offset - 73.0) > 1.0:
        raise AssertionError(f"{platform}: last item offset {text_b_offset} CSS px != 73px ±1")

    results["platforms"][platform] = {
        "device": details["device"],
        "png": str(path.relative_to(ROOT)),
        "pngSha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "imagePx": list(image.size),
        "deviceScale": scale,
        "exactRgb": {"grid": list(GRID), "firstText": list(TEXT_A), "lastText": list(TEXT_B)},
        "visibleColorBoundsPx": {"grid": grid, "firstText": text_a, "lastText": text_b},
        "gridBottomExclusivePx": expected_root_bottom,
        "lastItemTopOffsetCssPx": text_b_offset,
        "firstTextTopOffsetCssPx": text_a_offset,
        "lastItemStartsOutsideGrid": text_b_is_outside,
    }

out = PASS_DIR / "screenshot-pixel-analysis.json"
out.write_text(json.dumps(results, indent=2) + "\n")
print(out)
