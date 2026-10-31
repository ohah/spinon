#!/usr/bin/env python3
"""저장된 Chrome 관측을 독립 CSS 기대값과 비교한다."""

import json
from collections import defaultdict
from decimal import Decimal, ROUND_CEILING
from pathlib import Path


ROOT = Path(__file__).parent
expected = json.loads((ROOT / "expected-values.json").read_text())
observed = json.loads((ROOT / "chrome-observation.json").read_text())


def close(actual: float, wanted: Decimal) -> bool:
    return abs(Decimal(str(actual)) - wanted) <= Decimal("0.000001")


def blink_bound(value: str) -> Decimal:
    """이상적인 양수 폭에 관측된 Blink 1/64px 바깥 경계를 적용한다."""
    css_px = Decimal(value)
    return (css_px * 64).to_integral_value(rounding=ROUND_CEILING) / 64


cases = {
    case["id"]: (value, case)
    for value in expected["values"]
    for case in value["cases"]
}
rows = {row["id"]: row for row in observed["cases"]}
errors = []
checked_lines = 0
ignored_rows = 0

if len(cases) != 154 or len(rows) != 154 or set(cases) != set(rows):
    errors.append(f"case IDs/count differ: expected={len(cases)} observed={len(rows)}")
if expected["value_count"] != 14 or expected["cases_per_value"] != 11:
    errors.append("expected matrix metadata is not 14 values × 11 cases")
if not observed["fontLoaded"] or observed["chWidth"] != 10 or observed["spaceAdvance"] != 10:
    errors.append("Ahem/font metrics do not match the fixed fixture basis")

for case_id, (value, case) in cases.items():
    row = rows.get(case_id)
    if row is None:
        continue
    if not row["supported"]:
        errors.append(f"{case_id}: CSS.supports returned false")
    if row["computed"] != value["expected_computed"]:
        errors.append(
            f"{case_id}: computed={row['computed']!r}, "
            f"expected={value['expected_computed']!r}"
        )

    if case["layout"] == "ignored-whitespace-only-anonymous-grid-text":
        ignored_rows += 1
        if row["textRects"] or row["frame"]["width"] != 0 or row["frame"]["height"] != 0:
            errors.append(f"{case_id}: whitespace-only Grid text was not suppressed")
        continue

    frame_expected = Decimal(case["expected_frame_width_px"])
    if case["layout"] != "fixed-overflow":
        frame_expected = blink_bound(case["expected_frame_width_px"])
    if not close(row["frame"]["width"], frame_expected):
        errors.append(
            f"{case_id}: frame width={row['frame']['width']}, "
            f"expected Blink bound={frame_expected}"
        )
    expected_height = Decimal(10 * len(case["line_advances_px"]))
    if not close(row["frame"]["height"], expected_height):
        errors.append(
            f"{case_id}: frame height={row['frame']['height']}, "
            f"expected={expected_height}"
        )

    line_widths = defaultdict(float)
    for rect in row["textRects"]:
        line_widths[rect["y"]] += rect["width"]
    actual_lines = [width for _, width in sorted(line_widths.items())]
    expected_lines = [blink_bound(width) for width in case["line_advances_px"]]
    if len(actual_lines) != len(expected_lines) or any(
        not close(actual, wanted)
        for actual, wanted in zip(actual_lines, expected_lines)
    ):
        errors.append(
            f"{case_id}: line widths={actual_lines}, "
            f"expected Blink bounds={list(map(str, expected_lines))}"
        )
    checked_lines += len(expected_lines)

summary = {
    "observed_rows": len(rows),
    "positive_text_cases": len(cases) - ignored_rows,
    "ignored_whitespace_controls": ignored_rows,
    "checked_line_advances": checked_lines,
    "font_loaded": observed["fontLoaded"],
    "ch_advance_px": observed["chWidth"],
    "space_advance_px": observed["spaceAdvance"],
    "mismatch_count": len(errors),
    "mismatches": errors,
}
print(json.dumps(summary, indent=2))
raise SystemExit(bool(errors))
