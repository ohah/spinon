#!/usr/bin/env python3
"""Check fixed C11.5 samples in Android/iOS simulator display screenshots.

These are OS-composited screen screenshots. They are not Android PixelCopy or
pre-compositor Metal drawable readbacks; the native render/present evidence is
recorded separately in the corresponding logs.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

from PIL import Image


EVIDENCE = Path(__file__).resolve().parent
VIEWPORT_CSS = (980, 735)
GRID_ORIGIN_CSS = (24, 36)
GRID_BACKGROUND = (219, 234, 254)

SAMPLES = [
    ("centered-default-item", 44, 51, (249, 115, 22)),
    ("right-edge-item-interior", 63, 151, (249, 115, 22)),
    ("cell-right-boundary-outside", 64, 151, (255, 255, 255)),
    ("safe-overflow-outside-cell", 69, 556, (249, 115, 22)),
    ("unsafe-overflow-outside-cell", 20, 596, (249, 115, 22)),
    ("viewport-bottom-clip-visible-last-row", 49, 734, (249, 115, 22)),
    ("viewport-bottom-clip-outside-x", 90, 734, (255, 255, 255)),
]


def required_match(pattern: str, text: str, label: str) -> re.Match[str]:
    match = re.search(pattern, text)
    if match is None:
        raise ValueError(f"missing {label} in simulator log")
    return match


def verify(platform: str, screenshot_name: str, log_name: str, marker: str) -> dict:
    screenshot = EVIDENCE / screenshot_name
    log_path = EVIDENCE / log_name
    log_text = log_path.read_text()
    if marker not in log_text:
        raise ValueError(f"missing render/present marker {marker!r} in {log_name}")

    viewport_match = required_match(
        r"viewport_css_px=(\d+)x(\d+)", log_text, "viewport_css_px"
    )
    viewport = (int(viewport_match[1]), int(viewport_match[2]))
    if viewport != VIEWPORT_CSS:
        raise ValueError(f"unexpected {platform} CSS viewport {viewport}")

    if platform == "android-api37-avd":
        drawable_match = required_match(
            r"surface_texture=(\d+)x(\d+)", log_text, "surface_texture"
        )
        renderer = required_match(r"device=(.+?) format=", log_text, "renderer")[1]
    else:
        drawable_match = required_match(
            r"backend=Metal device=(.+?) format=\w+ size=(\d+)x(\d+)",
            log_text,
            "Metal drawable",
        )
        renderer = drawable_match[1]

    drawable = (int(drawable_match[1]), int(drawable_match[2])) if platform.startswith("android") else (
        int(drawable_match[2]), int(drawable_match[3])
    )
    scale_x = drawable[0] / viewport[0]
    scale_y = drawable[1] / viewport[1]
    if abs(scale_x - scale_y) > 1e-9:
        raise ValueError(f"nonuniform {platform} screenshot scale {scale_x}x{scale_y}")

    image = Image.open(screenshot).convert("RGB")
    blue_points = [
        (x, y)
        for y in range(image.height)
        for x in range(image.width)
        if image.getpixel((x, y)) == GRID_BACKGROUND
    ]
    if not blue_points:
        raise ValueError(f"Grid background color was not visible in {screenshot_name}")
    observed_grid_origin = (
        min(x for x, _ in blue_points),
        min(y for _, y in blue_points),
    )
    screen_origin = (
        observed_grid_origin[0] - GRID_ORIGIN_CSS[0] * scale_x,
        observed_grid_origin[1] - GRID_ORIGIN_CSS[1] * scale_y,
    )

    verified_samples = []
    for sample_id, css_x, css_y, expected in SAMPLES:
        pixel_x = round(screen_origin[0] + css_x * scale_x)
        pixel_y = round(screen_origin[1] + css_y * scale_y)
        actual = image.getpixel((pixel_x, pixel_y))
        verified_samples.append(
            {
                "id": sample_id,
                "css": [css_x, css_y],
                "pixel": [pixel_x, pixel_y],
                "rgba": [*actual, 255],
                "expected": [*expected, 255],
                "match": actual == expected,
            }
        )

    screenshot_hash = hashlib.sha256(screenshot.read_bytes()).hexdigest()
    return {
        "platform": platform,
        "screenshot": screenshot.name,
        "screenshotSha256": screenshot_hash,
        "evidenceClass": "OS-composited screen screenshot samples",
        "directPixelCopyOrMetalDrawableReadback": False,
        "renderer": renderer,
        "viewportCssPx": list(viewport),
        "drawablePx": list(drawable),
        "screenshotPx": list(image.size),
        "scale": [scale_x, scale_y],
        "gridBackgroundFirstPixel": list(observed_grid_origin),
        "screenOriginEstimate": list(screen_origin),
        "sampleCount": len(verified_samples),
        "matchedSamples": sum(sample["match"] for sample in verified_samples),
        "samples": verified_samples,
        "passed": all(sample["match"] for sample in verified_samples),
    }


def main() -> None:
    results = [
        verify(
            "android-api37-avd",
            "android-api37-avd.png",
            "android-api37-avd.log",
            "SPINON_C0410_DRAW status=0 presented",
        ),
        verify(
            "ios-iphone17pro-simulator",
            "ios-iphone17pro-simulator.png",
            "ios-simulator.log",
            "SPINON_C0410_DRAW generation=1 status=0 presented",
        ),
    ]
    output = {"schema": "spinon-c11-5-simulator-screen-samples-v1", "results": results}
    destination = EVIDENCE / "simulator-screen-samples.json"
    destination.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))
    if not all(result["passed"] for result in results):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
