#!/usr/bin/env python3
"""Check that simulator swipe markers appear inside the inner scroll border."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "spec/internal/evidence/c13-3-touch-action-simulator-matrix-2026-10-15"
OUTPUT = EVIDENCE / "pixel-analysis.json"
INNER_BORDER = (99, 169, 237)
SCROLL_MARKER = (227, 84, 78)
OUTER_MARKER = (237, 141, 63)
ROOT_MARKER = (177, 132, 226)
PAIRS = [
    ("ios-simulator", "horizontal", "ios-touch-action-pan-x-horizontal-left"),
    ("ios-simulator", "vertical", "ios-touch-action-pan-y-vertical-up"),
    ("android-emulator", "horizontal", "android-touch-action-pan-x-horizontal-left"),
    ("android-emulator", "vertical", "android-touch-action-pan-y-vertical-up"),
]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def color_bounds(image: Image.Image, color: tuple[int, int, int]):
    mask = Image.new("1", image.size)
    pixels = image.load()
    mask_pixels = mask.load()
    coordinates = []
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y] == color:
                mask_pixels[x, y] = 1
                coordinates.append((x, y))
    if not coordinates:
        return None, 0
    bounds = (
        min(x for x, _ in coordinates),
        min(y for _, y in coordinates),
        max(x for x, _ in coordinates) + 1,
        max(y for _, y in coordinates) + 1,
    )
    return bounds, len(coordinates)


def within(inner: tuple[int, int, int, int], outer: tuple[int, int, int, int]) -> bool:
    return outer[0] <= inner[0] and outer[1] <= inner[1] and inner[2] <= outer[2] and inner[3] <= outer[3]


def analyze(platform: str, axis: str, stem: str):
    before_path = EVIDENCE / f"{stem}-before.png"
    after_path = EVIDENCE / f"{stem}-after.png"
    before = Image.open(before_path).convert("RGB")
    after = Image.open(after_path).convert("RGB")
    if before.size != after.size:
        raise AssertionError(f"screenshot dimensions changed for {stem}: {before.size} -> {after.size}")

    inner_bounds, inner_border_pixels = color_bounds(after, INNER_BORDER)
    _, marker_before_pixels = color_bounds(before, SCROLL_MARKER)
    marker_after_bounds, marker_after_pixels = color_bounds(after, SCROLL_MARKER)
    _, outer_marker_pixels = color_bounds(after, OUTER_MARKER)
    _, root_marker_pixels = color_bounds(after, ROOT_MARKER)
    if inner_bounds is None or marker_after_bounds is None:
        raise AssertionError(f"missing inner border or moved marker for {stem}")
    if marker_before_pixels != 0:
        raise AssertionError(f"scroll marker was visible before swipe for {stem}: {marker_before_pixels}")
    if not within(marker_after_bounds, inner_bounds):
        raise AssertionError(
            f"moved marker escaped the inner border for {stem}: {marker_after_bounds} outside {inner_bounds}"
        )
    if outer_marker_pixels or root_marker_pixels:
        raise AssertionError(
            f"ancestor marker unexpectedly entered the capture for {stem}: outer={outer_marker_pixels} root={root_marker_pixels}"
        )

    before_marker_region = before.crop(marker_after_bounds)
    after_marker_region = after.crop(marker_after_bounds)
    marker_changed_pixels = sum(
        old != new
        for old, new in zip(before_marker_region.getdata(), after_marker_region.getdata())
    )
    if marker_changed_pixels != marker_after_pixels:
        raise AssertionError(
            f"marker region changed unexpectedly for {stem}: changed={marker_changed_pixels} marker={marker_after_pixels}"
        )

    return {
        "platform": platform,
        "axis": axis,
        "resolution": list(after.size),
        "innerBorderRgb": list(INNER_BORDER),
        "innerBorderBoundsPx": list(inner_bounds),
        "innerBorderPixels": inner_border_pixels,
        "markerRgb": list(SCROLL_MARKER),
        "markerBeforePixels": marker_before_pixels,
        "markerAfterBoundsPx": list(marker_after_bounds),
        "markerAfterPixels": marker_after_pixels,
        "markerChangedPixels": marker_changed_pixels,
        "markerWithinInnerBorder": True,
        "outerMarkerPixelsAfter": outer_marker_pixels,
        "rootMarkerPixelsAfter": root_marker_pixels,
        "beforeSha256": sha256(before_path),
        "afterSha256": sha256(after_path),
    }


def main():
    results = [analyze(*pair) for pair in PAIRS]
    OUTPUT.write_text(
        json.dumps(
            {
                "schema": "spinon-c13-3-touch-action-simulator-pixels/v1",
                "tool": "Pillow 11.3.0 exact RGB scan",
                "coordinateConvention": "zero-based, half-open pixel bounds",
                "results": results,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(OUTPUT.relative_to(ROOT))


if __name__ == "__main__":
    main()
