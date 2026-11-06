#!/usr/bin/env python3
"""Derive Grid intrinsic-size expectations from the fixed letter-word inputs."""

import json
import math
import re
from pathlib import Path


HERE = Path(__file__).resolve().parent
OBSERVATION = HERE / "chrome-observation.json"
OUTPUT = HERE / "expected.json"
CSS_PX_QUANTUM = 1 / 64
CONTAINER_WIDTH = 40
LINE_HEIGHT = 10


def css_layout_width(font_width: float) -> float:
    return math.ceil(font_width / CSS_PX_QUANTUM) * CSS_PX_QUANTUM


def line_count(words: list[str], column_width: float, advances: dict[str, float], space: float) -> int:
    lines = 1
    current = 0.0
    for word in words:
        word_width = advances[word]
        if current == 0:
            current = word_width
        elif current + space + word_width <= column_width:
            current += space + word_width
        else:
            lines += 1
            current = word_width
    return lines


def main() -> None:
    observation = json.loads(OBSERVATION.read_text())
    font_advances = observation["canvasAdvanceCssPx"]
    glyph = css_layout_width(font_advances["A"])
    space = css_layout_width(font_advances["space"])
    if not all(css_layout_width(font_advances[name]) == glyph for name in ("B", "a")):
        raise SystemExit("The fixed generic monospace input did not have equal ASCII letter advances")
    advances = {
        word: css_layout_width(width)
        for word, width in observation["wordAdvancesCssPx"].items()
    }

    cases = []
    for observed in observation["cases"]:
        words = observed["source"].split()
        if not words or not all(re.fullmatch(r"[A-Za-z]+", word) for word in words):
            raise SystemExit(f"Out-of-profile text source: {observed['source']!r}")
        min_content = max(advances[word] for word in words)
        max_content = sum(advances[word] for word in words) + space * (len(words) - 1)
        expected_column = (
            min_content
            if observed["columnsMode"] == "min-content"
            else max(min_content, min(CONTAINER_WIDTH, max_content))
        )
        expected_lines = line_count(words, expected_column, advances, space)
        expected_rows = (
            [10, 10, expected_lines * LINE_HEIGHT, 10]
            if observed["sourceOrder"] == "reverse"
            else [10, expected_lines * LINE_HEIGHT, 10, 10]
        )
        used_column = float(observed["usedColumns"].split()[0].removesuffix("px"))
        used_rows = [float(part.removesuffix("px")) for part in observed["usedRows"].split()]
        actual_lines = len(observed["textLineRects"])
        if abs(used_column - expected_column) > CSS_PX_QUANTUM / 2:
            raise SystemExit(
                f"{observed['id']}: used column {used_column} != independent expected {expected_column}"
            )
        if used_rows != expected_rows or actual_lines != expected_lines:
            raise SystemExit(
                f"{observed['id']}: rows/line count {used_rows}/{actual_lines} != "
                f"{expected_rows}/{expected_lines}"
            )

        relative_items = []
        grid_x, grid_y, _, _ = observed["gridFrame"]
        for item in observed["itemFrames"]:
            relative_items.append(
                [item[0] - grid_x, item[1] - grid_y, item[2], item[3]]
            )
        cases.append(
            {
                "id": observed["id"],
                "source": observed["source"],
                "columnsMode": observed["columnsMode"],
                "sourceOrder": observed["sourceOrder"],
                "maxUnbreakableWordLetters": max(map(len, words)),
                "independentExpectedColumnsCssPx": expected_column,
                "independentExpectedMaxContentCssPx": max_content,
                "independentExpectedRowsCssPx": expected_rows,
                "independentExpectedLongTextLineCount": expected_lines,
                "chromeObservedColumns": observed["usedColumns"],
                "chromeObservedRows": observed["usedRows"],
                "chromeObservedLongTextLineCount": actual_lines,
                "chromeObservedRootRelativeItemFramesCssPx": relative_items,
            }
        )

    result = {
        "reference": {
            "source": "CSS Grid Layout Module Level 1 2025 CRD §§11.4–11.7",
            "text": "CSS Text Module Level 4 §§4.1, 4.3.1 and 4.3.3",
            "fontMetricInput": "HeadlessChrome 156 canvas advance for 10px generic monospace",
            "layoutQuantizationCssPx": CSS_PX_QUANTUM,
        },
        "method": {
            "minContent": "maximum advance of an ASCII-letter word separated by collapsible spaces",
            "autoColumn": "the max-content track fills the 40px definite container unless the automatic minimum is wider",
            "lineWrapping": "greedy normal-whitespace line breaks between words; a word does not break internally",
            "independence": "track and row expectations are derived from the source tokens and glyph advance before comparing Chrome used tracks",
        },
        "matrix": {
            "columns": ["min-content", "auto"],
            "sourceCases": len(cases),
            "acceptedTokenGrammarCandidate": "one or more ASCII letters per token, separated by normal collapsible ASCII whitespace",
            "limits": ["Unicode", "digits", "punctuation", "non-collapsible whitespace", "other font families"],
        },
        "monospaceGlyphAdvanceCssPx": glyph,
        "monospaceSpaceAdvanceCssPx": space,
        "independentlyMeasuredWordAdvanceCssPx": advances,
        "cases": cases,
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(f"PASS: independently derived {len(cases)} cases -> {OUTPUT}")


if __name__ == "__main__":
    main()
