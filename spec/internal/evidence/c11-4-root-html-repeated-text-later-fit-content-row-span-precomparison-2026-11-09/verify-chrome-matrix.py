#!/usr/bin/env python3
"""Agent Browser Chrome의 후속 fit-content 위치 matrix를 검증한다."""

from collections import defaultdict
import hashlib
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
records = [
    json.loads(line)
    for line in (EVIDENCE / "chrome157-matrix-observation.ndjson").read_text().splitlines()
]
meta, *scenes = records
assert meta["kind"] == "meta"
assert "HeadlessChrome/157.0.0.0" in meta["userAgent"]
assert meta["platform"] == "MacIntel"
assert meta["dpr"] == 1
assert meta["fitContentPositions"] == [2, 3]
assert meta["tailPermutationCount"] == 30
assert meta["caseCount"] == len(scenes) == 1920

topologies = {
    "span2-forward": {"A": (2, 2), "B": (3, 2)},
    "span2-reversed": {"A": (2, 2), "B": (3, 2)},
    "span2-before-span3": {"A": (2, 2), "B": (3, 2), "C": (2, 3)},
    "span3-before-span2": {"A": (2, 2), "B": (3, 2), "C": (2, 3)},
}
caps = {"20px", "20%", "0%", "120%"}
expected_line_counts = {"A": 2, "B": 3, "C": 4}
seen = set()
by_case = {}
max_span_delta = 0.0
max_source_order_delta = 0.0
line_counts = defaultdict(set)

for case in scenes:
    fit_position = case["fitPosition"]
    assert fit_position in (2, 3)
    assert case["topology"] in topologies
    assert case["cap"] in caps
    assert case["align"] in ("start", "stretch")
    tail = case["tail"]
    assert len(tail) == 5
    assert tail.count("min-content") == 2
    assert tail.count("max-content") == 2
    assert tail.count("auto") == 1
    assert len(case["trackList"]) == 6
    expected_track_list = list(tail)
    expected_track_list.insert(fit_position - 1, f"fit-content({case['cap']})")
    assert case["trackList"] == expected_track_list
    assert case["computedAutoRows"].split() == expected_track_list
    key = (fit_position, tuple(tail), case["cap"], case["align"], case["topology"])
    assert key not in seen, f"duplicate scene key: {key}"
    seen.add(key)

    rows = case["rows"]
    assert len(rows) == 4, (key, rows)
    assert all(math.isfinite(value) and value >= 0 for value in rows)
    assert rows[0] == 10
    items = {item["id"]: item for item in case["items"]}
    expected_items = topologies[case["topology"]]
    assert len(items) == len(expected_items) + 1
    empty = items["empty-0"]
    assert empty["rowStart"] == "1" and empty["rowEnd"] == "2"
    assert empty["frame"] == [0, 0, 40, 10]

    for item_id, (row_start, span) in expected_items.items():
        item = items[item_id]
        assert item["rowStart"] == str(row_start)
        assert item["rowEnd"] == f"span {span}"
        assert len(item["lineYs"]) == expected_line_counts[item_id]
        line_counts[item_id].add(len(item["lineYs"]))
        frame = item["frame"]
        assert abs(frame[0]) <= 1e-6 and abs(frame[2] - 40) <= 1e-6
        first_track = row_start - 1
        expected_y = sum(rows[:first_track])
        expected_height = sum(rows[first_track : first_track + span])
        delta = max(abs(frame[1] - expected_y), abs(frame[3] - expected_height))
        max_span_delta = max(max_span_delta, delta)
        assert delta <= 0.011, (key, item_id, frame, rows, expected_y, expected_height)
        payload = item["payload"]
        assert payload is not None
        assert abs(payload[0] - frame[0]) <= 1e-6
        assert abs(payload[1] - frame[1]) <= 1e-6
        assert abs(payload[2] - 40) <= 1e-6
        assert abs(payload[3] - 10 * expected_line_counts[item_id]) <= 1e-6

    by_case[key] = case

assert seen == {
    (position, tuple(case_tail), cap, align, topology)
    for position in (2, 3)
    for case_tail in {tuple(case["tail"]) for case in scenes}
    for cap in caps
    for align in ("start", "stretch")
    for topology in topologies
}
assert line_counts == {"A": {2}, "B": {3}, "C": {4}}

for position in (2, 3):
    for tail in {tuple(case["tail"]) for case in scenes}:
        for cap in caps:
            for align in ("start", "stretch"):
                for topology_pair, semantic_ids in (
                    (("span2-forward", "span2-reversed"), ("A", "B")),
                    (("span2-before-span3", "span3-before-span2"), ("A", "B", "C")),
                ):
                    left, right = (
                        by_case[(position, tail, cap, align, topology)]
                        for topology in topology_pair
                    )
                    assert left["rows"] == right["rows"]
                    left_items = {item["id"]: item for item in left["items"]}
                    right_items = {item["id"]: item for item in right["items"]}
                    for item_id in semantic_ids:
                        for first, second in zip(
                            left_items[item_id]["frame"], right_items[item_id]["frame"]
                        ):
                            delta = abs(first - second)
                            max_source_order_delta = max(max_source_order_delta, delta)
                            assert delta <= 1 / 64 + 1e-9

for position in (2, 3):
    for tail in {tuple(case["tail"]) for case in scenes}:
        for cap in caps:
            for topology in topologies:
                start = by_case[(position, tail, cap, "start", topology)]
                stretch = by_case[(position, tail, cap, "stretch", topology)]
                assert start["rows"][0] == stretch["rows"][0]
                for row_index in range(1, 4):
                    track_kind = start["trackList"][row_index - 1]
                    if track_kind != "auto":
                        assert start["rows"][row_index] == stretch["rows"][row_index]

fixture = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-later-fit-content-row-span-precomparison.html"
observation = EVIDENCE / "chrome157-matrix-observation.ndjson"
summary = {
    "userAgent": meta["userAgent"],
    "platform": meta["platform"],
    "dpr": meta["dpr"],
    "fitContentPositions": [2, 3],
    "tailPermutationCount": 30,
    "caseCount": len(scenes),
    "byFitPosition": {
        str(position): sum(case["fitPosition"] == position for case in scenes)
        for position in (2, 3)
    },
    "byTopology": {
        topology: sum(case["topology"] == topology for case in scenes)
        for topology in topologies
    },
    "caps": sorted(caps),
    "alignments": ["start", "stretch"],
    "payloadLineCounts": {item: sorted(values) for item, values in line_counts.items()},
    "maxSpanFrameToTrackSumDeltaCssPx": max_span_delta,
    "maxSourceOrderFrameDeltaCssPx": max_source_order_delta,
    "fixtureSha256": hashlib.sha256(fixture.read_bytes()).hexdigest(),
    "observationSha256": hashlib.sha256(observation.read_bytes()).hexdigest(),
    "independentChecks": [
        "all 30 unique five-member tail permutations at fit-content positions 2 and 3",
        "four cap forms, two alignments, and four overlapping-span/source-order topologies",
        "CSS line placements map to explicit-plus-three implicit used rows",
        "each span frame equals the independent sum of the rows it covers",
        "text line counts and 40px payload widths remain fixed",
        "source-order reversal preserves semantic frames and row sizes",
        "align-content stretch changes only auto tracks among used rows",
        "computed grid-auto-rows preserves the six-member authored order",
    ],
}
(EVIDENCE / "matrix-verification.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
