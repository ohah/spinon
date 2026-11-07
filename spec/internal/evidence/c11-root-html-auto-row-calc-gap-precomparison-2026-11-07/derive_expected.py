#!/usr/bin/env python3
"""Derive C11.4 calc(row-gap) expected geometry independently of Chrome output."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
FIXTURE = Path(__file__).resolve().parents[4] / "tests/fixtures/css/c11/root-html-auto-row-content-alignment-calc-row-gap-precomparison.html"
OBSERVATION = ROOT / "chrome-observation.json"
ENVIRONMENT = ROOT / "browser-environment.json"
EXPECTED = ROOT / "expected-values.json"
VERIFICATION = ROOT / "precomparison-verification.json"

GAPS = [
    ("calc(1px - 10%)", -0.1, 1.0),
    ("calc(6.25% + 2px)", 0.0625, 2.0),
    ("calc(2px + 6.25%)", 0.0625, 2.0),
    ("calc(25% - 2px)", 0.25, -2.0),
    ("calc(10% - 1px)", 0.1, -1.0),
    ("calc(120% - 2px)", 1.2, -2.0),
]
ALIGNMENTS = [
    "normal",
    "stretch",
    "start",
    "flex-start",
    "end",
    "flex-end",
    "center",
    "space-between",
    "space-around",
    "space-evenly",
]
GRID = 40.0
BASE_ROWS = [10.0, 10.0, 10.0]
TOLERANCE = 1.0 / 32.0


def expected_case(
    expression: str,
    percentage: float,
    length: float,
    alignment: str,
    item_count: int,
) -> dict:
    raw_gap = GRID * percentage + length
    gap = max(0.0, raw_gap)
    assert gap < float("inf"), (expression, gap)
    rows = BASE_ROWS[:item_count]
    free = GRID - sum(rows) - (item_count - 1) * gap
    rows = list(rows)
    if alignment in ("normal", "stretch") and free > 0.0:
        stretch_count = item_count - 1
        for index in range(1, item_count):
            rows[index] += free / stretch_count

    leading = 0.0
    distributed_gaps = [0.0] * (item_count - 1)
    if alignment in ("end", "flex-end"):
        leading = free
    elif alignment == "center":
        leading = free / 2.0
    elif free > 0.0 and alignment == "space-between":
        distributed_gaps = [free / (item_count - 1)] * (item_count - 1)
    elif free > 0.0 and alignment == "space-around":
        leading = free / (2.0 * item_count)
        distributed_gaps = [free / item_count] * (item_count - 1)
    elif free > 0.0 and alignment == "space-evenly":
        leading = free / (item_count + 1)
        distributed_gaps = [free / (item_count + 1)] * (item_count - 1)

    tops = [leading]
    for index in range(1, item_count):
        tops.append(tops[-1] + rows[index - 1] + gap + distributed_gaps[index - 1])
    return {
        "expression": expression,
        "itemCount": item_count,
        "rawCalculatedGapPx": raw_gap,
        "resolvedGapPx": gap,
        "freeSpaceBeforeAlignmentPx": free,
        "alignContent": alignment,
        "usedRowsPx": rows,
        "itemRects": [[0.0, top, 40.0, height] for top, height in zip(tops, rows)],
    }


def close(actual: float, expected: float) -> bool:
    return abs(actual - expected) <= TOLERANCE


def main() -> None:
    fixture = FIXTURE.read_text(encoding="utf-8")
    observation = json.loads(OBSERVATION.read_text(encoding="utf-8"))
    environment = json.loads(ENVIRONMENT.read_text(encoding="utf-8"))
    assert fixture.count("runCalcGapAlignmentMatrix") == 1
    assert "grid-template-rows: 10px" in fixture and "grid-auto-rows: auto" in fixture
    assert "align-items: normal" in fixture and "column-gap: 0" in fixture
    assert "HeadlessChrome/157.0.0.0" in environment["userAgent"]
    assert environment["dpr"] == 1

    observed_groups = observation["results"]
    assert len(observed_groups) == len(GAPS)
    expected_groups = []
    maximum_error = 0.0
    compared_coordinates = 0
    for group, (expression, percentage, length) in zip(observed_groups, GAPS):
        assert group["expression"] == expression
        assert len(group["itemCountCases"]) == 2
        expected_item_count_cases = []
        for observed_item_count_case, item_count in zip(group["itemCountCases"], (2, 3)):
            assert observed_item_count_case["itemCount"] == item_count
            assert len(observed_item_count_case["cases"]) == len(ALIGNMENTS)
            expected_cases = []
            for observed, alignment in zip(observed_item_count_case["cases"], ALIGNMENTS):
                expected = expected_case(expression, percentage, length, alignment, item_count)
                assert observed["rowGap"].startswith("calc(")
                assert observed["alignContent"] == alignment
                assert observed["root"] == [100, 100, 40, 40]
                assert len(observed["usedRows"]) == item_count
                assert len(observed["itemRects"]) == item_count
                pairs = list(zip(observed["usedRows"], expected["usedRowsPx"]))
                pairs.extend(
                    (actual_value, expected_value)
                    for actual_rect, expected_rect in zip(observed["itemRects"], expected["itemRects"])
                    for actual_value, expected_value in zip(actual_rect, expected_rect)
                )
                for actual_value, expected_value in pairs:
                    error = abs(actual_value - expected_value)
                    maximum_error = max(maximum_error, error)
                    compared_coordinates += 1
                    assert close(actual_value, expected_value), (
                        expression,
                        item_count,
                        alignment,
                        actual_value,
                        expected_value,
                        error,
                    )
                expected_cases.append(expected)
            expected_item_count_cases.append({"itemCount": item_count, "cases": expected_cases})
        expected_groups.append({"expression": expression, "itemCountCases": expected_item_count_cases})

    EXPECTED.write_text(json.dumps({"groups": expected_groups}, indent=2) + "\n", encoding="utf-8")
    VERIFICATION.write_text(
        json.dumps(
            {
                "result": "passed",
                "browser": environment,
                "matrixCases": len(GAPS) * 2 * len(ALIGNMENTS),
                "comparedCoordinates": compared_coordinates,
                "toleranceCssPx": TOLERANCE,
                "maximumObservedErrorCssPx": maximum_error,
                "expectedSource": "fixed formula in derive_expected.py; not copied from Chrome geometry",
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(
        f"passed: {len(GAPS) * 2 * len(ALIGNMENTS)} cases, {compared_coordinates} coordinates, "
        f"max error {maximum_error:.9f}px <= {TOLERANCE:.9f}px"
    )


if __name__ == "__main__":
    main()
