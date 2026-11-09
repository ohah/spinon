#!/usr/bin/env python3
"""Agent Browser Chrome 157 fixed-input matrix의 독립 관계를 검증한다."""

from collections import Counter, defaultdict
import hashlib
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
capture_path = EVIDENCE / "chrome157-capture.json"
raw = json.loads(capture_path.read_text())
if isinstance(raw, str):
    raw = json.loads(raw)
meta = {key: value for key, value in raw.items() if key != "cases"}
scenes = raw["cases"]

assert "HeadlessChrome/157.0.0.0" in meta["userAgent"]
assert meta["platform"] == "MacIntel"
assert meta["dpr"] == 1
assert meta["fitContentPositions"] == [4, 5, 6]
assert meta["tailPermutationCount"] == 30
assert meta["caseCount"] == len(scenes) == 4320

topologies = {
    "span2-six-forward": {
        "A": (2, 2),
        "B": (3, 2),
        "C": (4, 2),
        "D": (5, 2),
        "E": (6, 2),
    },
    "span2-six-reversed": {
        "A": (2, 2),
        "B": (3, 2),
        "C": (4, 2),
        "D": (5, 2),
        "E": (6, 2),
    },
    "span2-span3-six-forward": {
        "A": (2, 2),
        "B": (3, 2),
        "C": (4, 3),
        "D": (5, 3),
    },
    "span2-span3-six-reversed": {
        "A": (2, 2),
        "B": (3, 2),
        "C": (4, 3),
        "D": (5, 3),
    },
}
expected_source_order = {
    "span2-six-forward": ["A", "B", "C", "D", "E"],
    "span2-six-reversed": ["E", "D", "C", "B", "A"],
    "span2-span3-six-forward": ["A", "B", "C", "D"],
    "span2-span3-six-reversed": ["D", "C", "B", "A"],
}
caps = {"0px", "0%", "10px", "20px", "20%", "120%"}
expected_line_counts = {"A": 6, "B": 5, "C": 4, "D": 4, "E": 3}
tails = {tuple(case["tail"]) for case in scenes}
seen = set()
by_case = {}
case_counts = Counter()
topology_counts = Counter()
line_counts = defaultdict(set)
max_span_delta = 0.0
max_source_order_delta = 0.0
max_line_height_delta = 0.0

for case in scenes:
    fit_position = case["fitPosition"]
    assert fit_position in (4, 5, 6)
    assert case["topology"] in topologies
    assert case["cap"] in caps
    assert case["align"] in ("start", "stretch")
    tail = tuple(case["tail"])
    assert len(tail) == 5
    assert tail.count("min-content") == 2
    assert tail.count("max-content") == 2
    assert tail.count("auto") == 1
    assert len(case["trackList"]) == 6
    expected_track_list = list(tail)
    expected_track_list.insert(fit_position - 1, f"fit-content({case['cap']})")
    assert case["trackList"] == expected_track_list
    assert case["computedAutoRows"].split() == expected_track_list

    key = (fit_position, tail, case["cap"], case["align"], case["topology"])
    assert key not in seen, f"duplicate scene key: {key}"
    seen.add(key)
    case_counts[fit_position] += 1
    topology_counts[case["topology"]] += 1

    rows = case["rows"]
    assert len(rows) == 7, (key, rows)
    assert all(math.isfinite(value) and value >= 0 for value in rows)
    assert rows[0] == 10
    items = {item["id"]: item for item in case["items"]}
    expected_items = topologies[case["topology"]]
    assert len(items) == len(expected_items) + 1
    assert [item["id"] for item in case["items"][1:]] == expected_source_order[case["topology"]]
    empty = items["empty-0"]
    assert empty["rowStart"] == "1" and empty["rowEnd"] == "2"
    assert empty["frame"] == [0, 0, 40, 10]

    for item_id, (row_start, span) in expected_items.items():
        item = items[item_id]
        assert item["rowStart"] == str(row_start)
        assert item["rowEnd"] == f"span {span}"
        assert len(item["lineYs"]) == expected_line_counts[item_id]
        line_counts[item_id].add(len(item["lineYs"]))
        assert all(
            math.isfinite(value)
            for value in item["frame"] + (item["payload"] or [])
        )
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
        expected_payload_height = 10 * expected_line_counts[item_id]
        assert abs(payload[3] - expected_payload_height) <= 1e-6
        line_deltas = [
            abs((right - left) - 10)
            for left, right in zip(item["lineYs"], item["lineYs"][1:])
        ]
        if line_deltas:
            max_line_height_delta = max(max_line_height_delta, max(line_deltas))
        assert all(delta <= 1 / 64 + 1e-9 for delta in line_deltas)

    by_case[key] = case

assert len(tails) == 30
assert case_counts == {4: 1440, 5: 1440, 6: 1440}
assert topology_counts == {topology: 1080 for topology in topologies}
assert seen == {
    (position, tail, cap, align, topology)
    for position in (4, 5, 6)
    for tail in tails
    for cap in caps
    for align in ("start", "stretch")
    for topology in topologies
}
assert line_counts == {
    "A": {6},
    "B": {5},
    "C": {4},
    "D": {4},
    "E": {3},
}

source_order_pairs = (
    (("span2-six-forward", "span2-six-reversed"), ("A", "B", "C", "D", "E")),
    (
        ("span2-span3-six-forward", "span2-span3-six-reversed"),
        ("A", "B", "C", "D"),
    ),
)
for position in (4, 5, 6):
    for tail in tails:
        for cap in caps:
            for align in ("start", "stretch"):
                for topology_pair, semantic_ids in source_order_pairs:
                    left, right = (
                        by_case[(position, tail, cap, align, topology)]
                        for topology in topology_pair
                    )
                    assert left["rows"] == right["rows"]
                    left_items = {item["id"]: item for item in left["items"]}
                    right_items = {item["id"]: item for item in right["items"]}
                    for item_id in semantic_ids:
                        for first, second in zip(
                            left_items[item_id]["frame"],
                            right_items[item_id]["frame"],
                        ):
                            delta = abs(first - second)
                            max_source_order_delta = max(
                                max_source_order_delta, delta
                            )
                            assert delta <= 1 / 64 + 1e-9

for position in (4, 5, 6):
    for tail in tails:
        for cap in caps:
            for topology in topologies:
                start = by_case[(position, tail, cap, "start", topology)]
                stretch = by_case[(position, tail, cap, "stretch", topology)]
                assert start["rows"][0] == stretch["rows"][0]
                for row_index in range(1, 7):
                    track_kind = start["trackList"][row_index - 1]
                    if track_kind != "auto":
                        assert start["rows"][row_index] == stretch["rows"][row_index]
                assert sum(stretch["rows"]) + 1e-7 >= sum(start["rows"])

fixture = ROOT / (
    "tests/fixtures/css/c11/"
    "root-html-grid-repeated-text-row-span-fit-content-positions-4-6-precomparison.html"
)
summary = {
    "userAgent": meta["userAgent"],
    "platform": meta["platform"],
    "dpr": meta["dpr"],
    "fitContentPositions": [4, 5, 6],
    "tailPermutationCount": len(tails),
    "caseCount": len(scenes),
    "byFitPosition": {str(position): case_counts[position] for position in (4, 5, 6)},
    "byTopology": dict(sorted(topology_counts.items())),
    "caps": sorted(caps),
    "alignments": ["start", "stretch"],
    "implicitRows": 6,
    "payloadLineCounts": {
        item: sorted(values) for item, values in sorted(line_counts.items())
    },
    "maxSpanFrameToTrackSumDeltaCssPx": max_span_delta,
    "maxSourceOrderFrameDeltaCssPx": max_source_order_delta,
    "maxTextLineHeightDeltaCssPx": max_line_height_delta,
    "fixtureSha256": hashlib.sha256(fixture.read_bytes()).hexdigest(),
    "captureSha256": hashlib.sha256(capture_path.read_bytes()).hexdigest(),
    "independentChecks": [
        "all 30 unique five-member tails at fit-content positions 4, 5, and 6",
        "six cap values, two alignments, and four overlapping-span/source-order topologies",
        "authored/computed six-member order and explicit-plus-six implicit rows",
        "each spanning item frame equals the sum of the used rows it crosses",
        "text line counts, line-height, and 40px payload widths remain fixed",
        "reversing DOM source order preserves row sizes and semantic frames",
        "align-content stretch changes only auto tracks among used implicit rows",
        "fit-content cap zero is retained as the explicit zero-cap control",
    ],
}
cap_differences = {}
for position in (4, 5, 6):
    changed_by_low_cap = 0
    for tail in tails:
        for align in ("start", "stretch"):
            for topology in topologies:
                zero_px = by_case[(position, tail, "0px", align, topology)]
                zero_percent = by_case[(position, tail, "0%", align, topology)]
                low = by_case[(position, tail, "10px", align, topology)]
                twenty_px = by_case[(position, tail, "20px", align, topology)]
                twenty_percent = by_case[(position, tail, "20%", align, topology)]
                large_percent = by_case[(position, tail, "120%", align, topology)]
                assert zero_px["rows"] == zero_percent["rows"]
                assert twenty_px["rows"] == twenty_percent["rows"] == large_percent["rows"]
                if low["rows"] != twenty_px["rows"]:
                    changed_by_low_cap += 1
                    assert low["rows"][position] != twenty_px["rows"][position]
    cap_differences[str(position)] = changed_by_low_cap
assert cap_differences == {"4": 48, "5": 48, "6": 24}

summary["capDifferential"] = {
    "blockSizeBasisCssPx": 220,
    "twentyPercentCssPx": 44,
    "oneHundredTwentyPercentCssPx": 264,
    "tenPxVsTwentyPxFitTrackDifferencesByPosition": cap_differences,
    "zeroPxAndZeroPercentEquivalent": True,
    "twentyPxTwentyPercentAndOneHundredTwentyPercentEquivalentForThisCorpus": True,
}
(EVIDENCE / "matrix-verification.json").write_text(
    json.dumps(summary, indent=2) + "\n"
)
print(json.dumps(summary, indent=2))
