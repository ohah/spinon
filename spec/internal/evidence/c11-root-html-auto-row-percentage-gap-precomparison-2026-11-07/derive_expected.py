#!/usr/bin/env python3
"""Derive RootHtmlAutoRowStretch percentage-gap frames from CSS Grid rules."""

from fractions import Fraction
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
OBSERVATION = HERE / "chrome-observation.json"
EXPECTED = HERE / "expected-values.json"
VERIFICATION = HERE / "precomparison-verification.json"

PERCENTAGES = [0, 6.25, 12.5, 18.75, 20, 100, 120]
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
ROOT_SIZE = Fraction(40)
BASE_ROWS = [Fraction(10), Fraction(10), Fraction(10)]
# Chrome's 1/64 CSS px serialization can round both the distributed offset
# and the track/gutter accumulation; the observed end-point delta is <= 1/32.
CSSOM_TOLERANCE = Fraction(1, 32)


def number(value: Fraction) -> float:
    return float(value)


def expected_case(percentage: float, alignment: str) -> dict:
    ratio = Fraction(str(percentage)) / 100
    gap = ROOT_SIZE * ratio
    free = ROOT_SIZE - sum(BASE_ROWS) - gap * 2
    rows = list(BASE_ROWS)
    offset = Fraction(0)
    extra_between = Fraction(0)

    if alignment in ("normal", "stretch"):
        if free > 0:
            rows[1] += free / 2
            rows[2] += free / 2
    elif alignment in ("end", "flex-end"):
        offset = free
    elif alignment == "center":
        offset = free / 2
    elif free > 0 and alignment == "space-between":
        extra_between = free / 2
    elif free > 0 and alignment == "space-around":
        extra_between = free / 3
        offset = free / 6
    elif free > 0 and alignment == "space-evenly":
        extra_between = free / 4
        offset = free / 4
    # Negative free space for distributed values falls back to start.

    starts = [offset]
    starts.append(starts[0] + rows[0] + gap + extra_between)
    starts.append(starts[1] + rows[1] + gap + extra_between)
    item_rects = [
        [Fraction(0), starts[0], Fraction(40), rows[0]],
        [Fraction(0), starts[1], Fraction(40), rows[1]],
        [Fraction(0), starts[2], Fraction(40), rows[2]],
    ]
    return {
        "percentage": percentage,
        "alignment": alignment,
        "gapPx": number(gap),
        "freeSpacePx": number(free),
        "tracksPx": [number(row) for row in rows],
        "itemRects": [[number(value) for value in rect] for rect in item_rects],
    }


def close(actual: float, expected: float) -> bool:
    return abs(Fraction(str(actual)) - Fraction(str(expected))) <= CSSOM_TOLERANCE


def main() -> None:
    expected_cases = [
        expected_case(percentage, alignment)
        for percentage in PERCENTAGES
        for alignment in ALIGNMENTS
    ]
    expected = {
        "basis": {
            "rootContentBoxPx": [40, 40],
            "explicitRowsPx": [10],
            "implicitAutoRowsPx": [10, 10],
            "columnGapPx": 0,
        },
        "percentages": PERCENTAGES,
        "alignments": ALIGNMENTS,
        "expectedCases": expected_cases,
        "comparisonToleranceCssPx": number(CSSOM_TOLERANCE),
    }
    EXPECTED.write_text(json.dumps(expected, indent=2) + "\n", encoding="utf-8")

    observed = json.loads(OBSERVATION.read_text(encoding="utf-8"))
    failures = []
    if len(observed) != len(PERCENTAGES):
        failures.append(f"percentage groups: got {len(observed)}, expected {len(PERCENTAGES)}")
    observation_count = sum(len(group.get("cases", [])) for group in observed)
    if observation_count != len(expected_cases):
        failures.append(f"scene count: got {observation_count}, expected {len(expected_cases)}")

    compared = 0
    max_abs_error = Fraction(0)
    for group_index, group in enumerate(observed[: len(PERCENTAGES)]):
        percentage = PERCENTAGES[group_index]
        if group.get("percentage") != percentage:
            failures.append(f"group {group_index}: unexpected percentage {group.get('percentage')}")
            continue
        if len(group.get("cases", [])) != len(ALIGNMENTS):
            failures.append(f"{percentage}%: alignment count is not 10")
            continue
        for alignment_index, alignment in enumerate(ALIGNMENTS):
            actual = group["cases"][alignment_index]
            wanted = expected_cases[group_index * len(ALIGNMENTS) + alignment_index]
            if actual.get("specified") != f"{percentage}%":
                failures.append(f"{percentage}%/{alignment}: specified value changed")
            if actual.get("computed") != f"{percentage}%":
                failures.append(f"{percentage}%/{alignment}: computed value changed")
            if actual.get("alignContent") != alignment:
                failures.append(f"{percentage}%: expected {alignment}, got {actual.get('alignContent')}")
            if actual.get("root") != [100, 100, 40, 40]:
                failures.append(f"{percentage}%/{alignment}: unexpected root rect {actual.get('root')}")
            for row_index, (got, want) in enumerate(zip(actual.get("usedRows", []), wanted["tracksPx"])):
                max_abs_error = max(max_abs_error, abs(Fraction(str(got)) - Fraction(str(want))))
                if not close(got, want):
                    failures.append(f"{percentage}%/{alignment}: track {row_index} got {got}, expected {want}")
            actual_rects = actual.get("itemRects", [])
            if len(actual_rects) != 3:
                failures.append(f"{percentage}%/{alignment}: expected three item rects")
            for item_index, (got_rect, want_rect) in enumerate(zip(actual_rects, wanted["itemRects"])):
                for axis, (got, want) in enumerate(zip(got_rect, want_rect)):
                    max_abs_error = max(max_abs_error, abs(Fraction(str(got)) - Fraction(str(want))))
                    if not close(got, want):
                        failures.append(
                            f"{percentage}%/{alignment}: item {item_index} axis {axis} got {got}, expected {want}"
                        )
            compared += 1

    result = {
        "result": "failed" if failures else "passed",
        "expectedSceneCount": len(expected_cases),
        "comparedSceneCount": compared,
        "toleranceCssPx": number(CSSOM_TOLERANCE),
        "maxObservedAbsoluteErrorCssPx": number(max_abs_error),
        "failures": failures,
        "note": "Chrome is recorded as a reference observation; expected values come from the independent CSS Grid free-space calculation.",
    }
    VERIFICATION.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if failures:
        raise SystemExit("\n".join(failures))


if __name__ == "__main__":
    main()
