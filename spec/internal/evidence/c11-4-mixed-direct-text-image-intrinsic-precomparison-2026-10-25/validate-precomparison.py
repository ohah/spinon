#!/usr/bin/env python3
"""Validate fixed CSS Grid expectations against the saved Chrome observation."""

import hashlib
import json
import math
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
OBSERVATION = EVIDENCE / "chrome156-observation.json"
FIXTURE = ROOT / "tests/fixtures/css/c11/grid-mixed-direct-text-image-intrinsic-c114-precomparison.html"
TOLERANCE = 1 / 64


def close(actual: float, expected: float) -> bool:
    return math.isfinite(actual) and abs(actual - expected) <= TOLERANCE


def css_px(value: str) -> float:
    match = re.fullmatch(r"([0-9]+(?:\.[0-9]+)?)px", value)
    if match is None:
        raise ValueError(f"unexpected computed track size: {value!r}")
    return float(match.group(1))


observation = json.loads(OBSERVATION.read_text(encoding="utf-8"))
assert FIXTURE.is_file(), f"missing fixture: {FIXTURE}"
assert observation["source_fixture"] == FIXTURE.relative_to(ROOT).as_posix()
assert hashlib.sha256(FIXTURE.read_bytes()).hexdigest() == observation["source_fixture_sha256"]
assert (EVIDENCE / observation["screenshot"]).is_file()
assert observation["input"] == {
    "text": "XX XXXXX",
    "font": "Ahem 10px/10px",
    "white_space": "normal",
    "word_break": "normal",
    "overflow_wrap": "normal",
    "grid_width_css_px": 120,
    "rows_css_px": [24, 24],
    "gap_css_px": 0,
    "item_span": 1,
}

# Independent Ahem model: two words of 2 and 5 full-width glyphs, plus one
# full-width collapsible space. The 24x16 PNG contributes its natural inline
# size to either intrinsic contribution.
glyph_advance = 10.0
word_glyph_counts = (2, 5)
space_glyph_count = 1
text_min_content = max(word_glyph_counts) * glyph_advance
text_max_content = (sum(word_glyph_counts) + space_glyph_count) * glyph_advance
image_inline_size = 24.0
expected = {
    "min-content": max(text_min_content, image_inline_size),
    "max-content": max(text_max_content, image_inline_size),
}

assert observation["browser"]["version"] == "156.0.8078.48"
assert observation["browser"]["viewport_css_px"] == [1280, 633]
assert observation["browser"]["device_pixel_ratio"] == 1
assert observation["readiness"] == {
    "ahem_font_ready": True,
    "all_images_decoded": True,
    "natural_image_css_px": [24, 16],
}
assert close(observation["independent_expectations"]["text_min_content_css_px"], text_min_content)
assert close(observation["independent_expectations"]["text_max_content_css_px"], text_max_content)
assert close(observation["independent_expectations"]["single_track_min_content_css_px"], expected["min-content"])
assert close(observation["independent_expectations"]["single_track_max_content_css_px"], expected["max-content"])

observed_cases = observation["observations"]
case_ids = [case["id"] for case in observed_cases]
assert len(observed_cases) == 4
assert len(set(case_ids)) == 4
cases = {case["id"]: case for case in observed_cases}
source_orders = {
    "min-content-forward": ["text", "image"],
    "max-content-forward": ["text", "image"],
    "min-content-reversed": ["image", "text"],
    "max-content-reversed": ["image", "text"],
}
assert set(cases) == {
    "min-content-forward",
    "max-content-forward",
    "min-content-reversed",
    "max-content-reversed",
}
for kind, expected_width in expected.items():
    forward = cases[f"{kind}-forward"]
    reversed_case = cases[f"{kind}-reversed"]
    for case in (forward, reversed_case):
        assert case["source_order"] == source_orders[case["id"]]
        assert close(css_px(case["columns_cssom"]), expected_width), case
        assert case["rows_cssom"] == "24px 24px"
        assert case["root_frame_css_px"][2:] == [120, 48]
        assert close(case["text_frame_css_px"][2], expected_width), case
        assert case["image_frame_css_px"][2:] == [24, 16]
    assert forward["columns_cssom"] == reversed_case["columns_cssom"]

print("C11.4 mixed direct text/image precomparison: 4/4 scenes PASS")
print(f"independent track expectations: min-content={expected['min-content']}px, max-content={expected['max-content']}px")
