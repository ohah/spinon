#!/usr/bin/env python3
"""시뮬레이터 PNG에서 고정 Grid 배경과 item 색상의 실제 경계를 잰다."""

import json
from pathlib import Path

import PIL
from PIL import Image


ROOT = Path(__file__).parent
EXPECTED_WIDTHS_CSS_PX = {
    "auto-fill-repeat": 24,
    "auto-fit-collapse": 96,
    "auto-fill-track-overflow": 120,
}
PIXEL_SCALE = {"android-avd": 2.625, "ios-simulator": 3.0}
GRID_BACKGROUND = (16, 24, 32)
ITEM_COLOR = (229, 72, 77)
ROOT_SIZE_CSS_PX = (100, 40)
ITEM_HEIGHT_CSS_PX = 16


def exact_color_bounds(image: Image.Image, color: tuple[int, int, int]):
    pixels = image.load()
    left, top = image.width, image.height
    right = bottom = -1
    count = 0
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y] == color:
                left = min(left, x)
                top = min(top, y)
                right = max(right, x)
                bottom = max(bottom, y)
                count += 1
    if count == 0:
        return None, 0
    return [left, top, right, bottom], count


report = {
    "method": "Exact RGB bounding boxes for #101820 Grid background and #e5484d item in device screenshots",
    "pillow_version": PIL.__version__,
    "expected_css_item_widths": EXPECTED_WIDTHS_CSS_PX,
    "runs": {},
}

for platform, scale in PIXEL_SCALE.items():
    report["runs"][platform] = {}
    for scene, expected_width in EXPECTED_WIDTHS_CSS_PX.items():
        path = ROOT / platform / f"{scene}.png"
        image = Image.open(path).convert("RGB")
        grid_bounds, grid_count = exact_color_bounds(image, GRID_BACKGROUND)
        item_bounds, item_count = exact_color_bounds(image, ITEM_COLOR)
        if grid_bounds is None or item_bounds is None:
            raise SystemExit(f"필수 색 표본이 없습니다: {path}")
        grid_width = grid_bounds[2] - grid_bounds[0] + 1
        grid_height = grid_bounds[3] - grid_bounds[1] + 1
        item_width = item_bounds[2] - item_bounds[0] + 1
        item_height = item_bounds[3] - item_bounds[1] + 1
        outset = item_bounds[2] - grid_bounds[2]
        if abs(grid_width - ROOT_SIZE_CSS_PX[0] * scale) > 1:
            raise SystemExit(f"Grid 픽셀 폭이 다릅니다: {path}: {grid_width}")
        if abs(grid_height - ROOT_SIZE_CSS_PX[1] * scale) > 1:
            raise SystemExit(f"Grid 픽셀 높이가 다릅니다: {path}: {grid_height}")
        if abs(item_width - expected_width * scale) > 1:
            raise SystemExit(f"item 픽셀 폭이 다릅니다: {path}: {item_width}")
        if abs(item_height - ITEM_HEIGHT_CSS_PX * scale) > 1:
            raise SystemExit(f"item 픽셀 높이가 다릅니다: {path}: {item_height}")
        if scene == "auto-fill-track-overflow" and outset <= 20 * scale:
            raise SystemExit(f"Grid 경계 밖 overflow가 없습니다: {path}: {outset}")
        if scene != "auto-fill-track-overflow" and item_bounds[2] >= grid_bounds[2]:
            raise SystemExit(f"예상 밖 item overflow입니다: {path}")
        report["runs"][platform][scene] = {
            "screenshot": str(path.relative_to(ROOT)),
            "image_size_px": [image.width, image.height],
            "grid_background_bbox_px": grid_bounds,
            "grid_background_pixel_count": grid_count,
            "grid_size_css_px_from_pixels": [
                round(grid_width / scale, 3),
                round(grid_height / scale, 3),
            ],
            "item_bbox_px": item_bounds,
            "item_pixel_count": item_count,
            "item_size_css_px_from_pixels": [
                round(item_width / scale, 3),
                round(item_height / scale, 3),
            ],
            "item_left_offset_css_px_from_pixels": round(
                (item_bounds[0] - grid_bounds[0]) / scale, 3
            ),
            "item_right_outset_px": outset,
            "item_right_outset_css_px": round(outset / scale, 3),
        }

print(json.dumps(report, ensure_ascii=False, indent=2))
