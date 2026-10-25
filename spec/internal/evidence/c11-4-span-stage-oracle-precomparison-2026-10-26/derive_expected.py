#!/usr/bin/env python3
"""Derive expected snapshots for the fixed C11.4 overlapping-span controls.

This is deliberately narrower than a general Grid sizing implementation. It
models §11.5.1 planned increases for the exact max-content track/item matrix
below, then §11.6 for the definite 220px axis. All arithmetic is Fraction.
"""

from fractions import Fraction
import json


TRACK_COUNT = 4
AVAILABLE = Fraction(220)
BASE = [Fraction(0) for _ in range(TRACK_COUNT)]
ITEMS = {
    "A": {"span": 2, "first_track": 0, "contribution": Fraction(80)},
    "B": {"span": 2, "first_track": 1, "contribution": Fraction(120)},
    "C": {"span": 3, "first_track": 0, "contribution": Fraction(200)},
}
CASES = [
    {"id": "competing-span2-forward", "source_order": ["A", "B"]},
    {"id": "competing-span2-reversed", "source_order": ["B", "A"]},
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


def apply_group(growth_limits, item_ids):
    # §11.5.1 keeps per-track planned increases for the batch. All affected
    # tracks are max-content maxima with no fit-content/fixed upper limit.
    planned = [Fraction(0) for _ in range(TRACK_COUNT)]
    item_calculations = []
    for item_id in item_ids:
        item = ITEMS[item_id]
        indexes = range(item["first_track"], item["first_track"] + item["span"])
        current_sizes = [
            BASE[index] if growth_limits[index] is None else growth_limits[index]
            for index in indexes
        ]
        space = max(Fraction(0), item["contribution"] - sum(current_sizes))
        share = space / item["span"]
        for index in range(item["first_track"], item["first_track"] + item["span"]):
            planned[index] = max(planned[index], share)
        item_calculations.append(
            {
                "item": item_id,
                "span": item["span"],
                "contribution_px": px(item["contribution"]),
                "current_spanned_sum_px": px(sum(current_sizes)),
                "space_to_distribute_px": px(space),
                "equal_increase_per_affected_track_px": px(share),
            }
        )

    for index, increase in enumerate(planned):
        if increase:
            current = BASE[index] if growth_limits[index] is None else growth_limits[index]
            growth_limits[index] = current + increase

    return {
        "items": item_calculations,
        "planned_increases_px": [px(value) for value in planned],
        "base_sizes_px": [px(value) for value in BASE],
        "growth_limits_px": [px(value) for value in growth_limits],
    }


def maximize(growth_limits):
    remaining = AVAILABLE - sum(BASE)
    sizes = list(BASE)
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
        "available_space_px": px(AVAILABLE),
        "base_sizes_px": [px(value) for value in sizes],
        "growth_limits_px": [px(value) for value in growth_limits],
        "remaining_free_space_px": px(remaining),
    }


def derive(case):
    growth_limits = [None for _ in range(TRACK_COUNT)]  # None means infinity.
    initialization = {
        "base_sizes_px": [px(value) for value in BASE],
        "growth_limits_px": [None for _ in range(TRACK_COUNT)],
    }
    groups = {}
    for item_id in case["source_order"]:
        groups.setdefault(ITEMS[item_id]["span"], []).append(item_id)

    stages = []
    for span in sorted(groups):
        calculation = apply_group(growth_limits, groups[span])
        stages.append(
            {
                "stage": f"IntrinsicItemBatch(span={span}, crosses_flexible_track=false)",
                **calculation,
            }
        )

    # §11.5 step 5: unconstrained infinite growth limits become base size.
    completed_growth_limits = [
        BASE[index] if value is None else value for index, value in enumerate(growth_limits)
    ]
    intrinsic_complete = {
        "base_sizes_px": [px(value) for value in BASE],
        "growth_limits_px": [px(value) for value in completed_growth_limits],
    }
    return {
        "id": case["id"],
        "source_order": case["source_order"],
        "normative_batch_order": [f"span{span}" for span in sorted(groups)],
        "initialize": initialization,
        "intrinsic_batch_stages": stages,
        "intrinsic_sizing_complete": intrinsic_complete,
        "maximize_tracks_stage": maximize(completed_growth_limits),
    }


results = [derive(case) for case in CASES]
snapshot_fields = ("stage", "planned_increases_px", "base_sizes_px", "growth_limits_px")
snapshots = lambda result: [
    {field: stage[field] for field in snapshot_fields}
    for stage in result["intrinsic_batch_stages"]
]
assert snapshots(results[0]) == snapshots(results[1])
assert snapshots(results[2]) == snapshots(results[3])
assert results[0]["initialize"] == results[1]["initialize"]
assert results[2]["initialize"] == results[3]["initialize"]
assert results[0]["intrinsic_sizing_complete"] == results[1]["intrinsic_sizing_complete"]
assert results[2]["intrinsic_sizing_complete"] == results[3]["intrinsic_sizing_complete"]
assert results[0]["maximize_tracks_stage"] == results[1]["maximize_tracks_stage"]
assert results[2]["maximize_tracks_stage"] == results[3]["maximize_tracks_stage"]
assert results[2]["normative_batch_order"] == ["span2", "span3"]
assert results[2]["intrinsic_batch_stages"][0]["growth_limits_px"] == [40, 60, 60, None]
assert results[2]["intrinsic_batch_stages"][1]["growth_limits_px"] == [
    "160/3",
    "220/3",
    "220/3",
    None,
]
assert results[2]["intrinsic_sizing_complete"]["growth_limits_px"] == [
    "160/3",
    "220/3",
    "220/3",
    0,
]
assert results[2]["maximize_tracks_stage"]["base_sizes_px"] == [
    "160/3",
    "220/3",
    "220/3",
    0,
]
assert results[2]["maximize_tracks_stage"]["remaining_free_space_px"] == 20
assert results[0]["maximize_tracks_stage"]["base_sizes_px"] == [40, 60, 60, 0]
assert results[0]["maximize_tracks_stage"]["remaining_free_space_px"] == 60

print(json.dumps({"standard": "CSS Grid Layout Module Level 1, 2025 CRD §§11.5.1, 11.6", "results": results}, indent=2))
