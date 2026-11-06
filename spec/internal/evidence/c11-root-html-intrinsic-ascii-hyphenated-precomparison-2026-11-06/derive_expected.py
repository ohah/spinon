#!/usr/bin/env python3
"""Derive the fixed hyphenated-token Grid expectations from text advances."""

from __future__ import annotations

import json
import math
import re
from pathlib import Path


EVIDENCE = Path(__file__).resolve().parent
OBSERVATION = json.loads((EVIDENCE / "chrome-observation.json").read_text())
SCALE = 64
LINE_HEIGHT = 10.0
AVAILABLE_WIDTH = 40.0
FIXED_WIDTH = 20.0


def ceil_to_layout_unit(value: float) -> float:
    return math.ceil(value * SCALE - 1e-9) / SCALE


def parse_px_list(value: str) -> list[float]:
    return [float(token.removesuffix("px")) for token in value.split()]


def segments(token: str) -> list[str]:
    return re.findall(r"[A-Za-z0-9]+-?", token)


def line_count(text: str, widths: dict[str, float], available: float) -> int:
    tokens = text.split()
    current = 0.0
    count = 0
    previous_token: int | None = None
    space = widths[" "]

    for token_index, token in enumerate(tokens):
        for segment in segments(token):
            segment_width = widths[segment]
            separator = space if current and previous_token != token_index else 0.0
            candidate = current + separator + segment_width
            if current and candidate > available + 1e-9:
                count += 1
                current = segment_width
            else:
                current = candidate
            previous_token = token_index

    return count + int(current > 0.0)


def observed_close(actual: float, expected: float) -> bool:
    return abs(actual - expected) <= 1.0 / SCALE + 1e-9


derived = []
for case in OBSERVATION["cases"]:
    text = case["text"]
    tokens = text.split()
    break_segments = [part for token in tokens for part in segments(token)]
    measured = {
        entry["text"]: entry["advanceCssPx"]
        for entry in case["metrics"]["breakSegmentAdvancesCssPx"]
    }
    measured.update(
        {entry["text"]: entry["advanceCssPx"] for entry in case["metrics"]["tokenAdvancesCssPx"]}
    )
    measured[text] = case["metrics"]["fullTextAdvanceCssPx"]
    measured[" "] = case["metrics"]["spaceAdvanceCssPx"]

    min_content = ceil_to_layout_unit(max(measured[part] for part in break_segments))
    max_content = ceil_to_layout_unit(measured[text])
    mode = case["columnsMode"]
    if mode == "min-content":
        used_column = min_content
    elif mode == "max-content":
        used_column = max_content
    elif mode == "auto":
        used_column = max(min_content, min(AVAILABLE_WIDTH, max_content))
    elif mode == "fixed":
        used_column = FIXED_WIDTH
    else:
        raise ValueError(f"unsupported column mode: {mode}")

    expected_lines = line_count(text, measured, used_column)
    reverse = case["id"] == "hyphenated-reverse-order"
    text_row = 20.0 if reverse else 10.0
    expected_rows = (
        [LINE_HEIGHT, LINE_HEIGHT, expected_lines * LINE_HEIGHT, LINE_HEIGHT]
        if reverse
        else [LINE_HEIGHT, expected_lines * LINE_HEIGHT, LINE_HEIGHT, LINE_HEIGHT]
    )
    expected_word_frame = [0.0, text_row, used_column, expected_lines * LINE_HEIGHT]

    observed_column = parse_px_list(case["computedColumns"])
    observed_rows = parse_px_list(case["computedRows"])
    observed_word_frame = case["wordFrame"]
    observed_line_count = len(case["lines"])
    assert len(observed_column) == 1 and observed_close(observed_column[0], used_column), case["id"]
    assert len(observed_rows) == len(expected_rows), case["id"]
    assert all(observed_close(a, b) for a, b in zip(observed_rows, expected_rows)), case["id"]
    assert observed_line_count == expected_lines, case["id"]
    assert all(observed_close(a, b) for a, b in zip(observed_word_frame, expected_word_frame)), case["id"]

    derived.append(
        {
            "id": case["id"],
            "text": text,
            "breakSegments": break_segments,
            "independentMinContentCssPx": min_content,
            "independentMaxContentCssPx": max_content,
            "expectedColumnsCssPx": [used_column],
            "expectedRowsCssPx": expected_rows,
            "expectedTextLineCount": expected_lines,
            "expectedWordFrameRelativeToGridCssPx": expected_word_frame,
            "chromeComparison": "match",
        }
    )

output = {
    "schema": "spinon-css-reference-expectation-v1",
    "task": "C11.4 parsed Root HTML ASCII hyphenated word intrinsic sizing",
    "basis": [
        "CSS Text Module Level 3 §5.1: the hyphen-minus line-break opportunity occurs after the hyphen.",
        "CSS Grid Layout Module Level 1 2025 CRD §§11.4-11.7: min-content uses the widest unbreakable segment, and an auto track is bounded by its min/max contributions and definite available size.",
        "The fixed monospace advances are read from Canvas measureText, independent of CSSOM Grid used track results; each expected used size is rounded up to 1/64 CSS px.",
    ],
    "chrome": OBSERVATION["userAgent"],
    "devicePixelRatio": OBSERVATION["dpr"],
    "rounding": "ceil advance values to the next 1/64 CSS px layout unit",
    "cases": derived,
}
(EVIDENCE / "expected.json").write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n")
print(f"derived and compared {len(derived)} cases")
