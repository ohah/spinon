#!/usr/bin/env python3
"""Measure fixed-color Root HTML Grid item bounds in simulator screenshots."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys

from PIL import Image

ROOT = Path(__file__).resolve().parent
PASS_NAME = sys.argv[1] if len(sys.argv) > 1 else "pass01"
if not PASS_NAME.startswith("pass") or not PASS_NAME[4:].isdigit():
    raise SystemExit("pass name must use passNN format")
PASS_DIR = ROOT / PASS_NAME
CASES = {
    "android": {"scale": 2.625, "device": "Android API 37 ARM64 AVD"},
    "ios": {"scale": 3.0, "device": "iPhone 17 Pro / iOS 26.2 Simulator"},
}
SCENES = {
    "positive": {"alignment": "baseline", "expectedY": [0, 10, 20]},
    "overflow": {"alignment": "baseline", "expectedY": [0, 56, 112]},
}
GRID = (233, 237, 242)
FIXED = (216, 221, 227)
TEXT_A = (91, 159, 245)
TEXT_B = (244, 162, 97)
scene_name = (PASS_DIR / "environment.txt").read_text().split("scene=", 1)[1].splitlines()[0]
scene = SCENES[scene_name]


def bbox(image: Image.Image, color: tuple[int, int, int]) -> list[int]:
    width, height = image.size
    pixels = image.load()
    xs: list[int] = []
    ys: list[int] = []
    for y in range(min(height, 700)):
        for x in range(min(width, 240)):
            if pixels[x, y] == color:
                xs.append(x)
                ys.append(y)
    if not xs:
        raise AssertionError(f"missing exact screenshot color {color}")
    return [min(xs), min(ys), max(xs), max(ys)]


results: dict[str, object] = {
    "purpose": "Measure Grid and item bounds from fixed-color OS screenshot captures.",
    "expected": {
        "scene": scene_name,
        "alignment": scene["alignment"],
        "gridSizeCssPx": [40, 40],
        "expectedItemYRelativeCssPx": scene["expectedY"],
        "classification": "visual screenshot bounds only; not PixelCopy or Metal drawable readback",
    },
    "platforms": {},
}

for platform, details in CASES.items():
    path = PASS_DIR / platform / "simulator.png"
    image = Image.open(path).convert("RGB")
    positive_passes = [
        candidate
        for candidate in sorted(ROOT.glob("pass[0-9][0-9]"))
        if (candidate / "environment.txt").is_file()
        and "scene=positive\n" in (candidate / "environment.txt").read_text()
        and all((candidate / platform / "simulator.png").is_file() for platform in CASES)
    ]
    if not positive_passes:
        raise AssertionError("a successful positive simulator pass is required as origin")
    positive_pass = positive_passes[-1]
    positive_path = positive_pass / platform / "simulator.png"
    positive = Image.open(positive_path).convert("RGB")
    grid = bbox(positive, GRID)
    fixed_control = bbox(positive, FIXED)
    root_origin_y = fixed_control[1]  # The first item begins at the Grid start edge in both baseline scenes.
    visible_grid_width = grid[2] - grid[0] + 1
    visible_grid_background_height = grid[3] - grid[1] + 1
    item_bounds: list[list[int] | None] = []
    measured_y: list[float | None] = []
    scale = details["scale"]
    for index, color in enumerate((FIXED, TEXT_A, TEXT_B)):
        expected = scene["expectedY"][index]
        try:
            item = bbox(image, color)
        except AssertionError:
            if expected >= 0:
                raise
            item_bounds.append(None)
            measured_y.append(None)
            continue
        item_bounds.append(item)
        measured_y.append((item[1] - root_origin_y) / scale)
    expected_y = scene["expectedY"]
    if len(measured_y) != len(expected_y):
        raise AssertionError(f"{platform}: expected three item bounds, got {len(measured_y)}")
    for index, (actual, expected) in enumerate(zip(measured_y, expected_y)):
        if actual is None:
            continue
        if abs(actual - expected) > 1.0:
            raise AssertionError(
                f"{platform}: item {index} y offset {actual} CSS px != {expected} ±1"
            )
    expected_grid_width = 40 * scale
    if abs(visible_grid_width - expected_grid_width) > 2:
        raise AssertionError(
            f"{platform}: positive-control Grid background width is not 40 CSS px: {visible_grid_width}"
        )
    results["platforms"][platform] = {
        "device": details["device"],
        "png": str(path.relative_to(ROOT)),
        "pngSha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "imagePx": list(image.size),
        "deviceScale": scale,
        "exactRgb": {
            "grid": list(GRID),
            "fixed": list(FIXED),
            "textA": list(TEXT_A),
            "textB": list(TEXT_B),
        },
        "positiveControlGridBackgroundColorBoundsPx": grid,
        "positiveControlFixedItemBoundsPx": fixed_control,
        "rootOriginYFromFixedItemTopPx": root_origin_y,
        "positiveControlGridBackgroundColorVisibleSizePx": [
            visible_grid_width,
            visible_grid_background_height,
        ],
        "positiveControlGridVisibleBackgroundWidthCssPx": visible_grid_width / scale,
        "visibleColorBoundsPx": {"fixed": item_bounds[0], "textA": item_bounds[1], "textB": item_bounds[2]},
        "measuredItemYRelativeCssPx": measured_y,
        "rootOriginSource": f"{positive_path.relative_to(ROOT)} first fixed item top (CSS y=0)",
        "toleranceCssPx": 1.0,
    }

out = PASS_DIR / "screenshot-pixel-analysis.json"
out.write_text(json.dumps(results, indent=2) + "\n")
print(out)
