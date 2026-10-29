#!/usr/bin/env python3
"""Check pinned Agent Browser geometry and screenshot pixels against CSS expectations."""

import hashlib
import json
import struct
import zlib
from pathlib import Path


HERE = Path(__file__).resolve().parent
OBSERVATION = json.loads((HERE / "chrome-agent-browser-observation.json").read_text())
EXPECTED = json.loads((HERE / "expected.json").read_text())
FIXTURE = HERE.parents[3] / "tests/fixtures/css/c11/grid-visibility-collapse-c114-precomparison.html"
SCREENSHOT = HERE / "chrome-agent-browser-precomparison.png"
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_tracks(value: str) -> list[float]:
    return [float(part.removesuffix("px")) for part in value.split()]


def expect_close(actual: float, expected: float, message: str) -> None:
    if abs(actual - expected) > 1e-6:
        raise SystemExit(f"{message}: expected {expected}, got {actual}")


def read_png_samples(path: Path, points: set[tuple[int, int]]) -> dict[tuple[int, int], list[int]]:
    data = path.read_bytes()
    if not data.startswith(PNG_SIGNATURE):
        raise SystemExit("Agent Browser screenshot is not a PNG")

    width = height = bit_depth = color_type = interlace = None
    compressed = bytearray()
    cursor = len(PNG_SIGNATURE)
    while cursor < len(data):
        size = struct.unpack(">I", data[cursor : cursor + 4])[0]
        kind = data[cursor + 4 : cursor + 8]
        chunk = data[cursor + 8 : cursor + 8 + size]
        cursor += 12 + size
        if kind == b"IHDR":
            width, height, bit_depth, color_type, _, _, interlace = struct.unpack(">IIBBBBB", chunk)
        elif kind == b"IDAT":
            compressed.extend(chunk)
        elif kind == b"IEND":
            break

    if width is None or bit_depth != 8 or color_type not in (2, 6) or interlace != 0:
        raise SystemExit("screenshot PNG format changed; expected non-interlaced 8-bit RGB/RGBA")
    channels = 3 if color_type == 2 else 4
    stride = width * channels
    decoded = zlib.decompress(compressed)
    previous = bytearray(stride)
    wanted_rows = {y for _, y in points}
    result: dict[tuple[int, int], list[int]] = {}
    offset = 0

    def paeth(a: int, b: int, c: int) -> int:
        estimate = a + b - c
        da, db, dc = abs(estimate - a), abs(estimate - b), abs(estimate - c)
        if da <= db and da <= dc:
            return a
        if db <= dc:
            return b
        return c

    for y in range(height):
        filter_kind = decoded[offset]
        row = bytearray(decoded[offset + 1 : offset + 1 + stride])
        offset += stride + 1
        if filter_kind > 4:
            raise SystemExit(f"unsupported PNG filter {filter_kind}")
        for i in range(stride):
            left = row[i - channels] if i >= channels else 0
            above = previous[i]
            upper_left = previous[i - channels] if i >= channels else 0
            if filter_kind == 1:
                predictor = left
            elif filter_kind == 2:
                predictor = above
            elif filter_kind == 3:
                predictor = (left + above) // 2
            elif filter_kind == 4:
                predictor = paeth(left, above, upper_left)
            else:
                predictor = 0
            row[i] = (row[i] + predictor) & 0xFF
        if y in wanted_rows:
            for x, point_y in points:
                if point_y == y:
                    if x < 0 or x >= width:
                        raise SystemExit(f"pixel sample x={x} is outside screenshot width {width}")
                    start = x * channels
                    result[(x, y)] = list(row[start : start + 3])
        previous = row

    if len(result) != len(points):
        raise SystemExit("one or more pixel samples are outside the screenshot")
    return result


reference = EXPECTED["reference"]
if reference["userAgent"] not in OBSERVATION["userAgent"]:
    raise SystemExit("Agent Browser observation is not from the pinned Chrome version")
if OBSERVATION["viewport"] != reference["viewport"]:
    raise SystemExit("Agent Browser viewport or DPR changed")
if sha256(FIXTURE) != reference["sourceSha256"]:
    raise SystemExit("CSS fixture source hash differs from the pinned precomparison input")
if sha256(SCREENSHOT) != reference["screenshotSha256"]:
    raise SystemExit("Agent Browser screenshot hash differs from the pinned capture")

expected_cases = {case["id"]: case for case in EXPECTED["cases"]}
observed_cases = {case["id"]: case for case in OBSERVATION["cases"]}
fixed_ids = [
    "collapse-column",
    "hidden-column",
    "display-none-column",
    "collapse-visible-descendant-column",
    "collapse-row",
    "hidden-row",
    "display-none-row",
    "collapse-visible-descendant-row",
    "visible-column",
    "visible-row",
]
if list(expected_cases) != fixed_ids:
    raise SystemExit("fixed CSS Grid visibility scene IDs or order changed")
if len(observed_cases) != len(OBSERVATION["cases"]):
    raise SystemExit("Agent Browser returned duplicate scene IDs")
if list(observed_cases) != fixed_ids:
    raise SystemExit("Agent Browser scene IDs differ from the fixed matrix")

for scene_id, expected in expected_cases.items():
    actual = observed_cases[scene_id]
    if actual["axis"] != expected["axis"]:
        raise SystemExit(f"{scene_id}: axis differs")
    if parse_tracks(actual["gridTemplateColumns"]) != expected["columns"]:
        raise SystemExit(f"{scene_id}: columns differ")
    if parse_tracks(actual["gridTemplateRows"]) != expected["rows"]:
        raise SystemExit(f"{scene_id}: rows differ")
    if actual["overflow"] != "visible":
        raise SystemExit(f"{scene_id}: root overflow is not visible")
    if actual["itemDisplay"] != expected["display"]:
        raise SystemExit(f"{scene_id}: display differs")
    if actual["itemVisibility"] != expected["visibility"]:
        raise SystemExit(f"{scene_id}: computed visibility differs")
    for key in ("width", "height"):
        expect_close(actual["root"][key], expected["root"][key], f"{scene_id} root {key}")
    expect_close(actual["root"]["x"], 0, f"{scene_id} root x")
    for key in ("x", "y", "width", "height"):
        if key in ("x", "y") and expected["display"] != "none":
            actual_position = actual["item"][key] - actual["root"][key]
        else:
            actual_position = actual["item"][key]
        expect_close(actual_position, expected["item"][key], f"{scene_id} item {key}")
    for key in ("width", "height"):
        expect_close(actual["child"][key], expected["child"][key], f"{scene_id} child {key}")
    for key in ("x", "y"):
        if expected["display"] != "none":
            actual_position = actual["child"][key] - actual["item"][key]
        else:
            actual_position = actual["child"][key]
        expect_close(actual_position, expected["child"].get(key, 0), f"{scene_id} child {key}")
    if actual["childVisibility"] != expected["child"]["visibility"]:
        raise SystemExit(f"{scene_id}: child visibility differs")
    if actual["hitAtItem"] != expected["hitAtItem"]:
        raise SystemExit(f"{scene_id}: item hit-test target differs")
    if actual["hitAtOverflow"] != expected["hitAtOverflow"]:
        raise SystemExit(f"{scene_id}: root-overflow hit-test target differs")

    point = actual["overflowPoint"]
    offset = {"x": point["x"] - actual["root"]["x"], "y": point["y"] - actual["root"]["y"]}
    expected_offset = {"x": 81, "y": 10} if expected["axis"] == "columns" else {"x": 10, "y": 81}
    for key in ("x", "y"):
        expect_close(offset[key], expected_offset[key], f"{scene_id} overflow sample {key}")

dpr = reference["viewport"]["dpr"]
pixel_expectations: dict[tuple[int, int], list[int]] = {}
for sample in EXPECTED["surfaceSamples"]:
    root = observed_cases[sample["scene"]]["root"]
    x = round((root["x"] + sample["offset"]["x"]) * dpr)
    y = round((root["y"] + sample["offset"]["y"]) * dpr)
    point = (x, y)
    if point in pixel_expectations and pixel_expectations[point] != sample["rgb"]:
        raise SystemExit(f"conflicting screenshot expectations at {point}")
    pixel_expectations[point] = sample["rgb"]
actual_pixels = read_png_samples(SCREENSHOT, set(pixel_expectations))
for point, expected_rgb in pixel_expectations.items():
    if actual_pixels[point] != expected_rgb:
        raise SystemExit(f"screenshot pixel {point}: expected {expected_rgb}, got {actual_pixels[point]}")

print(
    f"PASS {len(expected_cases)}/{len(expected_cases)} fixed scenes; "
    f"{len(pixel_expectations)} exact screenshot samples"
)
