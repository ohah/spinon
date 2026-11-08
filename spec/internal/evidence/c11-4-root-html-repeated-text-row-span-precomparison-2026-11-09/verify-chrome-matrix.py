#!/usr/bin/env python3
"""Validate the saved independent Chrome observation for the C11.4 span matrix."""
import hashlib
import json
import math
from pathlib import Path

root = Path(__file__).resolve().parents[4]
evidence = Path(__file__).resolve().parent
records = [json.loads(line) for line in (evidence / "chrome157-matrix-observation.ndjson").read_text().splitlines()]
observed = records[0]
scenes = records[1:]
assert observed["kind"] == "meta"
expected_topologies = {
    "span2-forward": {"A": (2, 2), "B": (3, 2)},
    "span2-reversed": {"A": (2, 2), "B": (3, 2)},
    "span2-before-span3": {"A": (2, 2), "B": (3, 2), "C": (2, 3)},
    "span3-before-span2": {"A": (2, 2), "B": (3, 2), "C": (2, 3)},
}
assert observed["tailPermutationCount"] == 30
assert observed["caseCount"] == len(scenes) == 960
assert observed["dpr"] == 1
assert "HeadlessChrome/157.0.0.0" in observed["userAgent"]
assert len({(tuple(s["tail"]), s["cap"], s["align"], s["topology"]) for s in scenes}) == 960

by_case = {}
max_span_height_delta = 0.0
max_source_order_delta = 0.0
line_counts = {"A": set(), "B": set(), "C": set()}
for case in scenes:
    expected_items = expected_topologies[case["topology"]]
    items = {item["id"]: item for item in case["items"]}
    assert len(items) == len(expected_items) + 1
    assert len(case["rows"]) == 4
    assert all(math.isfinite(value) and value >= 0 for value in case["rows"])
    assert case["rows"][0] == 10.0
    assert case["align"] in ("start", "stretch")
    assert case["cap"] in ("20px", "20%", "0%", "120%")
    assert all(abs(item["frame"][0]) <= 1e-6 and abs(item["frame"][2] - 40.0) <= 1e-6 for item in items.values())
    for item_id, (row_start, span) in expected_items.items():
        item = items[item_id]
        assert item["rowStart"] == str(row_start)
        assert item["rowEnd"] == f"span {span}"
        assert len(item["lineYs"]) == {"A": 2, "B": 3, "C": 4}[item_id]
        line_counts[item_id].add(len(item["lineYs"]))
        row_start_index = row_start - 1
        expected_height = sum(case["rows"][row_start_index:row_start_index + span])
        delta = abs(expected_height - item["frame"][3])
        max_span_height_delta = max(max_span_height_delta, delta)
        assert delta <= 0.011, (case["topology"], item_id, expected_height, item["frame"][3])
        assert item["payload"] is not None
        assert item["payload"][2] == 40.0
    by_case[(case["topology"], tuple(case["tail"]), case["cap"], case["align"])] = case

for item_id, counts in line_counts.items():
    assert counts == ({2} if item_id == "A" else {3} if item_id == "B" else {4})

for key in {(tuple(case["tail"]), case["cap"], case["align"]) for case in scenes}:
    tail, cap, align = key
    for pair, semantic_ids in [(("span2-forward", "span2-reversed"), ("A", "B")), (("span2-before-span3", "span3-before-span2"), ("A", "B", "C"))]:
        left, right = (by_case[(topology, tail, cap, align)] for topology in pair)
        left_items = {item["id"]: item for item in left["items"]}
        right_items = {item["id"]: item for item in right["items"]}
        assert left["rows"] == right["rows"]
        for item_id in semantic_ids:
            for a, b in zip(left_items[item_id]["frame"], right_items[item_id]["frame"]):
                max_source_order_delta = max(max_source_order_delta, abs(a - b))
                assert abs(a - b) <= 1 / 64 + 1e-9

for case in scenes:
    start = by_case[(case["topology"], tuple(case["tail"]), case["cap"], "start")]
    stretch = by_case[(case["topology"], tuple(case["tail"]), case["cap"], "stretch")]
    assert abs(start["rows"][0] - stretch["rows"][0]) <= 1e-9
    assert abs(start["rows"][1] - stretch["rows"][1]) <= 1e-9
    for row_index in range(2, len(start["rows"])):
        track_kind = case["tail"][row_index - 2]
        if track_kind != "auto":
            assert abs(start["rows"][row_index] - stretch["rows"][row_index]) <= 1e-9

summary = {
    "userAgent": observed["userAgent"],
    "dpr": observed["dpr"],
    "tailPermutationCount": observed["tailPermutationCount"],
    "caseCount": observed["caseCount"],
    "byTopology": {topology: sum(case["topology"] == topology for case in scenes) for topology in expected_topologies},
    "caps": sorted({case["cap"] for case in scenes}),
    "alignments": sorted({case["align"] for case in scenes}),
    "payloadLineCounts": {item: sorted(counts) for item, counts in line_counts.items()},
    "maxSpanHeightDeltaCssPx": max_span_height_delta,
    "maxSourceOrderFrameDeltaCssPx": max_source_order_delta,
    "fixtureSha256": hashlib.sha256((root / "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-precomparison.html").read_bytes()).hexdigest(),
    "observationSha256": hashlib.sha256((evidence / "chrome157-matrix-observation.ndjson").read_bytes()).hexdigest(),
    "checks": [
        "all 30 unique tail permutations x 4 caps x 2 alignments x 4 topologies present",
        "explicit row spans map to expected track indices and do not create implicit columns",
        "item block size equals the sum of spanned used rows within 0.011 CSS px",
        "payload line counts are exact (A=2, B=3, C=4)",
        "source order reversal preserves each semantic item frame within 1/64 CSS px",
        "align-content:stretch changes only auto-track sizes",
    ],
}
(evidence / "matrix-verification.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
