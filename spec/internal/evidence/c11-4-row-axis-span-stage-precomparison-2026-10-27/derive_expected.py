#!/usr/bin/env python3
"""Derive fixed row-axis snapshots for the C11.4 span-stage test slice."""

from fractions import Fraction
import json


TRACK_COUNT = 4
AVAILABLE_BLOCK_SIZE = Fraction(220)
BASE_SIZES = [Fraction(0) for _ in range(TRACK_COUNT)]
ITEMS = {
    "A": {"first_row": 0, "span": 2, "height": Fraction(80)},
    "B": {"first_row": 1, "span": 2, "height": Fraction(120)},
    "C": {"first_row": 0, "span": 3, "height": Fraction(200)},
}
CASES = [
    {"id": "span2-forward", "source_order": ["A", "B"]},
    {"id": "span2-reversed", "source_order": ["B", "A"]},
    {"id": "span2-before-span3", "source_order": ["A", "B", "C"]},
    {"id": "span3-before-span2", "source_order": ["C", "B", "A"]},
]


def px(value):
    if value is None:
        return None
    value = Fraction(value)
    if value.denominator == 1:
        return value.numerator
    return f"{value.numerator}/{value.denominator}"


def apply_span_group(growth_limits, item_ids):
    planned_increases = [Fraction(0) for _ in range(TRACK_COUNT)]
    calculations = []
    for item_id in item_ids:
        item = ITEMS[item_id]
        rows = range(item["first_row"], item["first_row"] + item["span"])
        current_sum = sum(
            BASE_SIZES[index] if growth_limits[index] is None else growth_limits[index]
            for index in rows
        )
        space = max(Fraction(0), item["height"] - current_sum)
        share = space / item["span"]
        for index in range(item["first_row"], item["first_row"] + item["span"]):
            planned_increases[index] = max(planned_increases[index], share)
        calculations.append(
            {
                "item": item_id,
                "span": item["span"],
                "height_contribution_px": px(item["height"]),
                "current_spanned_growth_sum_px": px(current_sum),
                "space_to_distribute_px": px(space),
                "equal_increase_per_affected_row_px": px(share),
            }
        )

    for index, increase in enumerate(planned_increases):
        if increase:
            current = BASE_SIZES[index] if growth_limits[index] is None else growth_limits[index]
            growth_limits[index] = current + increase
    return {
        "items": calculations,
        "planned_increases_px": [px(value) for value in planned_increases],
        "base_sizes_px": [px(value) for value in BASE_SIZES],
        "growth_limits_px": [px(value) for value in growth_limits],
    }


def maximize_tracks(growth_limits):
    sizes = list(BASE_SIZES)
    remaining = AVAILABLE_BLOCK_SIZE - sum(sizes)
    active = {index for index, limit in enumerate(growth_limits) if limit > sizes[index]}
    while remaining > 0 and active:
        share = remaining / len(active)
        progressed = Fraction(0)
        for index in list(active):
            increase = min(share, growth_limits[index] - sizes[index])
            sizes[index] += increase
            progressed += increase
            if sizes[index] == growth_limits[index]:
                active.remove(index)
        assert progressed > 0
        remaining -= progressed
    return {
        "available_block_size_px": px(AVAILABLE_BLOCK_SIZE),
        "row_sizes_px": [px(value) for value in sizes],
        "growth_limits_px": [px(value) for value in growth_limits],
        "remaining_free_space_px": px(remaining),
    }


def derive(case):
    growth_limits = [None for _ in range(TRACK_COUNT)]  # None represents +infinity.
    groups = {}
    for item_id in case["source_order"]:
        groups.setdefault(ITEMS[item_id]["span"], []).append(item_id)
    batch_stages = []
    for span in sorted(groups):
        batch_stages.append(
            {
                "stage": f"IntrinsicItemBatch(span={span}, crosses_flexible_track=false)",
                **apply_span_group(growth_limits, groups[span]),
            }
        )

    completed_growth_limits = [
        BASE_SIZES[index] if value is None else value
        for index, value in enumerate(growth_limits)
    ]
    return {
        "id": case["id"],
        "source_order": case["source_order"],
        "row_contributions_px": {key: px(ITEMS[key]["height"]) for key in case["source_order"]},
        "normative_batch_order": [f"span{span}" for span in sorted(groups)],
        "initialize": {
            "base_sizes_px": [px(value) for value in BASE_SIZES],
            "growth_limits_px": [None for _ in range(TRACK_COUNT)],
        },
        "intrinsic_batch_stages": batch_stages,
        "intrinsic_sizing_complete": {
            "base_sizes_px": [px(value) for value in BASE_SIZES],
            "growth_limits_px": [px(value) for value in completed_growth_limits],
        },
        "maximize_tracks_stage": maximize_tracks(completed_growth_limits),
    }


results = [derive(case) for case in CASES]
assert results[0]["maximize_tracks_stage"]["row_sizes_px"] == [40, 60, 60, 0]
assert results[0]["maximize_tracks_stage"]["remaining_free_space_px"] == 60
assert results[0]["intrinsic_batch_stages"][0]["growth_limits_px"] == [40, 60, 60, None]
assert results[2]["normative_batch_order"] == ["span2", "span3"]
assert results[2]["intrinsic_batch_stages"][0]["growth_limits_px"] == [40, 60, 60, None]
assert results[2]["intrinsic_batch_stages"][1]["growth_limits_px"] == [
    "160/3", "220/3", "220/3", None
]
assert results[2]["intrinsic_sizing_complete"]["growth_limits_px"] == [
    "160/3", "220/3", "220/3", 0
]
assert results[2]["maximize_tracks_stage"]["row_sizes_px"] == [
    "160/3", "220/3", "220/3", 0
]
assert results[2]["maximize_tracks_stage"]["remaining_free_space_px"] == 20
for forward, reversed_case in [(results[0], results[1]), (results[2], results[3])]:
    assert forward["normative_batch_order"] == reversed_case["normative_batch_order"]
    snapshot_fields = ("stage", "planned_increases_px", "base_sizes_px", "growth_limits_px")
    forward_snapshots = [
        {field: stage[field] for field in snapshot_fields}
        for stage in forward["intrinsic_batch_stages"]
    ]
    reversed_snapshots = [
        {field: stage[field] for field in snapshot_fields}
        for stage in reversed_case["intrinsic_batch_stages"]
    ]
    assert forward_snapshots == reversed_snapshots
    assert forward["intrinsic_sizing_complete"] == reversed_case["intrinsic_sizing_complete"]
    assert forward["maximize_tracks_stage"] == reversed_case["maximize_tracks_stage"]

print(json.dumps({
    "standard": "CSS Grid Layout Module Level 1, 2025 CRD §§11.5.1, 11.6",
    "axis": "Rows",
    "track_count": TRACK_COUNT,
    "available_block_size_px": px(AVAILABLE_BLOCK_SIZE),
    "results": results,
}, indent=2))
