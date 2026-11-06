#!/usr/bin/env python3
"""Derive ASCII alphanumeric Grid expectations independently from Chrome used tracks."""
import json
import math
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
OBSERVATION = HERE / "chrome-observation.json"
OUTPUT = HERE / "expected.json"
QUANTUM = 1 / 64
CONTAINER_WIDTH = 40
LINE_HEIGHT = 10
TOKEN = re.compile(r"[A-Za-z0-9]+\Z")


def css_advance(canvas_width: float) -> float:
    return math.ceil(canvas_width / QUANTUM) * QUANTUM


def line_count(tokens: list[str], width: float, advances: dict[str, float], space: float) -> int:
    count = 1
    current = 0.0
    for token in tokens:
        token_width = advances[token]
        if current == 0:
            current = token_width
        elif current + space + token_width <= width:
            current += space + token_width
        else:
            count += 1
            current = token_width
    return count


def main() -> None:
    observed = json.loads(OBSERVATION.read_text())
    canvas = observed["canvasAdvanceCssPx"]
    space = css_advance(canvas[" "])
    tokens_seen = sorted({token for case in observed["cases"] for token in case["source"].split()})
    if not all(TOKEN.fullmatch(token) for token in tokens_seen):
        raise SystemExit("fixture contains a token outside the ASCII alphanumeric profile")
    advances = {token: css_advance(observed["wordAdvancesCssPx"][token]) for token in tokens_seen}
    cases = []
    for case in observed["cases"]:
        tokens = case["source"].split()
        min_content = max(advances[token] for token in tokens)
        max_content = sum(advances[token] for token in tokens) + space * (len(tokens) - 1)
        expected_column = min_content if case["columnsMode"] == "min-content" else max(min_content, min(CONTAINER_WIDTH, max_content))
        expected_lines = line_count(tokens, expected_column, advances, space)
        expected_rows = ([10, 10, expected_lines * LINE_HEIGHT, 10] if case["sourceOrder"] == "reverse" else [10, expected_lines * LINE_HEIGHT, 10, 10])
        used_column = float(case["usedColumns"].split()[0].removesuffix("px"))
        used_rows = [float(value.removesuffix("px")) for value in case["usedRows"].split()]
        if abs(used_column - expected_column) > QUANTUM / 2:
            raise SystemExit(f"{case['id']}: Chrome track {used_column} != independent {expected_column}")
        if used_rows != expected_rows or len(case["textLineRects"]) != expected_lines:
            raise SystemExit(f"{case['id']}: Chrome rows/lines {used_rows}/{len(case['textLineRects'])} != independent {expected_rows}/{expected_lines}")
        grid_x, grid_y, _, _ = case["gridFrame"]
        item_frames = [[frame[0]-grid_x, frame[1]-grid_y, frame[2], frame[3]] for frame in case["itemFrames"]]
        cases.append({
            "id": case["id"], "source": case["source"], "columnsMode": case["columnsMode"], "sourceOrder": case["sourceOrder"],
            "maxUnbreakableTokenCharacters": max(map(len, tokens)),
            "independentExpectedColumnsCssPx": expected_column,
            "independentExpectedMaxContentCssPx": max_content,
            "independentExpectedRowsCssPx": expected_rows,
            "independentExpectedLongTextLineCount": expected_lines,
            "chromeObservedColumns": case["usedColumns"], "chromeObservedRows": case["usedRows"],
            "chromeObservedLongTextLineCount": len(case["textLineRects"]),
            "chromeObservedRootRelativeItemFramesCssPx": item_frames,
        })
    result = {
        "reference": {
            "grid": "CSS Grid Layout Module Level 1, 2025 CRD, §§11.4–11.7",
            "text": "CSS Text Module Level 4, Editor's Draft, §§3–6",
            "browserObservation": observed["userAgent"],
            "canvasFontMetric": "10px generic monospace canvas advance, quantized upward to 1/64 CSS px for independent geometry arithmetic",
        },
        "method": {
            "minContent": "maximum advance of a non-empty ASCII alphanumeric token separated by collapsible ASCII whitespace",
            "autoColumn": "max(min-content, min(40px, max-content)) for the definite 40px container",
            "lineWrapping": "greedy normal-whitespace opportunities between tokens; no internal break in the accepted token profile",
            "independence": "expected track, rows, and line counts are calculated from source tokens and Canvas advances before comparison to Chrome used tracks",
        },
        "matrix": {"columns": ["min-content", "auto"], "sceneCount": len(cases), "tokenGrammar": "[A-Za-z0-9]+", "separators": ["SPACE", "TAB", "LF", "CR"], "limits": ["Unicode", "punctuation", "non-collapsible whitespace", "other fonts"]},
        "monospaceGlyphAdvanceCssPx": css_advance(canvas["A"]),
        "monospaceSpaceAdvanceCssPx": space,
        "independentlyMeasuredTokenAdvanceCssPx": advances,
        "cases": cases,
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(f"PASS: independently derived {len(cases)} scenes -> {OUTPUT}")

if __name__ == "__main__":
    main()
