#!/usr/bin/env python3
"""Independently validate the stored Agent Browser Chrome 157 scene matrix."""
from collections import defaultdict
from itertools import permutations
import hashlib
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
OBSERVATION = EVIDENCE / "chrome157-observation.ndjson"
FIXTURE = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-positions-4-6-auto-minimum-precomparison.html"
CAPTURE = EVIDENCE / "capture-chrome.mjs"
TOLERANCE = 0.01
TOPOLOGIES = {
    "span2-six-forward": [("A", 2, 2), ("B", 3, 2), ("C", 4, 2), ("D", 5, 2), ("E", 6, 2)],
    "span2-six-reversed": [("E", 6, 2), ("D", 5, 2), ("C", 4, 2), ("B", 3, 2), ("A", 2, 2)],
    "span2-span3-six-forward": [("A", 2, 2), ("B", 3, 2), ("C", 4, 3), ("D", 5, 3)],
    "span2-span3-six-reversed": [("D", 5, 3), ("C", 4, 3), ("B", 3, 2), ("A", 2, 2)],
}
LINE_COUNTS = {"A": 6, "B": 5, "C": 4, "D": 4, "E": 3}
CAPS = {"0px", "0%", "10px", "20px", "20%", "120%"}
TAILS = set(permutations(("min-content", "min-content", "max-content", "max-content", "auto")))

def require(condition, message):
    if not condition:
        raise AssertionError(message)

def delta(left, right):
    return max(abs(a - b) for a, b in zip(left, right))

records = [json.loads(line) for line in OBSERVATION.read_text().splitlines() if line]
require(len(records) == 8640, f"expected 8640 records, got {len(records)}")
require([record["index"] for record in records] == list(range(8640)), "capture indexes are not contiguous")
require(all("HeadlessChrome/157." in record["userAgent"] for record in records), "unexpected browser version")
require(all(record["platform"] == "MacIntel" and record["devicePixelRatio"] == 1 for record in records), "browser environment changed")

pairs = defaultdict(dict)
source_pairs = defaultdict(dict)
alignment_pairs = defaultdict(dict)
seen = set()
coverage = defaultdict(int)
max_track_delta = 0.0
max_item_delta = 0.0
max_order_delta = 0.0
max_non_auto_stretch_delta = 0.0
max_auto_stretch_delta = 0.0
auto_track_growth_count = 0
changed_item_frames = 0
eligible_occurrences = 0
baseline_below_floor = 0
candidate_below_floor = 0
candidate_overflow = 0

for record in records:
    scene = record["scene"]
    position = scene["fitPosition"]
    cap = scene["cap"]
    align = scene["alignContent"]
    topology = scene["topology"]
    mode = scene["minHeight"]
    tail = tuple(scene["tail"])
    require(position in (4, 5, 6), f"bad fit position: {position}")
    require(cap in CAPS, f"bad cap: {cap}")
    require(tail in TAILS, f"bad tail: {tail}")
    require(align in ("start", "stretch"), f"bad alignment: {align}")
    require(topology in TOPOLOGIES, f"bad topology: {topology}")
    require(mode in ("0px", "auto"), f"bad minimum mode: {mode}")
    tail_list = list(tail)
    members = tail_list.copy()
    members.insert(position - 1, f"fit-content({cap})")
    track_list = " ".join(members)
    require(scene["trackMembers"] == members and scene["trackList"] == track_list, "authored track order/value changed")
    require(members.count("min-content") == 2 and members.count("max-content") == 2 and members.count("auto") == 1 and sum(member.startswith("fit-content(") for member in members) == 1, "track multiplicity changed")
    require(scene["key"] == "|".join((str(position), " ".join(tail), cap, align, topology)), "paired scene key changed")
    key = scene["key"]
    require((key, mode) not in seen, f"duplicate scene: {key} {mode}")
    seen.add((key, mode))
    coverage[(position, cap, align, topology, mode)] += 1
    require(record["gridFrame"] == [0, 0, 40, 220], "definite grid frame changed")
    require(record["computedAutoRows"] == track_list and record["computedAlignContent"] == align, "computed CSS differs from authored scene")
    rows = record["rowValues"]
    require(len(rows) == 7 and rows[0] == 10 and all(math.isfinite(value) and value >= 0 for value in rows), "expected explicit row and six finite nonnegative implicit rows")
    require(record["usedRows"].split() == [f"{value:g}px" for value in rows] or len(record["usedRows"].split()) == 7, "used track serialization missing")
    expected_items = TOPOLOGIES[topology]
    require(scene["items"] == [list(item) for item in expected_items], "scene item order/topology changed")
    require(len(record["items"]) == len(expected_items) + 1, "direct item count changed")
    empty = record["items"][0]
    require(empty["id"] == "empty" and empty["minHeight"] == "0px" and empty["frame"] == [0, 0, 40, 10], "explicit-row control changed")
    actual_by_id = {item["id"]: item for item in record["items"]}
    require(list(actual_by_id) == ["empty", *[item[0] for item in expected_items]], "DOM source order changed")
    for item_id, row_start, span in expected_items:
        item = actual_by_id[item_id]
        require(item["minHeight"] == mode, f"{item_id} computed min-height changed")
        require(item["rowStart"] == str(row_start) and item["rowEnd"] == f"span {span}", f"{item_id} placement changed")
        require(len(item["lineRects"]) == LINE_COUNTS[item_id], f"{item_id} line count changed")
        frame = item["frame"]
        payload = item["payloadFrame"]
        require(payload is not None and len(item["lineRects"]) > 0, f"{item_id} payload missing")
        require(abs(frame[0]) <= TOLERANCE and abs(frame[2] - 40) <= TOLERANCE, f"{item_id} column geometry changed")
        first = row_start - 1
        expected_y = sum(rows[:first])
        expected_height = sum(rows[first:first + span])
        require(abs(frame[1] - expected_y) <= TOLERANCE and abs(frame[3] - expected_height) <= TOLERANCE, f"{item_id} frame does not equal occupied used-row sum")
        require(item["contentOverflowBottom"] <= TOLERANCE, f"{item_id} visible content overflows its frame")
    pairs[key][mode] = record
    order_key = (position, tail, cap, align, mode, "span2" if "span2-six" in topology else "span2-span3")
    order_name = "forward" if topology.endswith("forward") else "reverse"
    source_pairs[order_key][order_name] = record
    alignment_pairs[(position, tail, cap, topology, mode)][align] = record

require(len(pairs) == 4320 and all(set(pair) == {"0px", "auto"} for pair in pairs.values()), "baseline/candidate pairs incomplete")
for key, pair in pairs.items():
    before, after = pair["0px"], pair["auto"]
    require(before["scene"]["trackList"] == after["scene"]["trackList"] and before["scene"]["items"] == after["scene"]["items"], f"paired authored inputs differ: {key}")
    for index, (left, right) in enumerate(zip(before["rowValues"], after["rowValues"])):
        max_track_delta = max(max_track_delta, abs(left - right))
    before_items = {item["id"]: item for item in before["items"]}
    after_items = {item["id"]: item for item in after["items"]}
    for item_id, row_start, span in before["scene"]["items"]:
        left, right = before_items[item_id], after_items[item_id]
        require(len(left["lineRects"]) == len(right["lineRects"]), f"paired text wrapping changed: {key} {item_id}")
        frame_delta = delta(left["frame"], right["frame"])
        max_item_delta = max(max_item_delta, frame_delta)
        if frame_delta > TOLERANCE:
            changed_item_frames += 1
        eligible = any(
            before["scene"]["trackMembers"][(grid_track - 1) % 6] == "auto"
            or before["scene"]["trackMembers"][(grid_track - 1) % 6].startswith("fit-content(")
            for grid_track in range(row_start - 1, row_start - 1 + span)
            if grid_track > 0
        )
        if eligible:
            eligible_occurrences += 1
            baseline_below_floor += left["frame"][3] + TOLERANCE < left["payloadFrame"][3]
            candidate_below_floor += right["frame"][3] + TOLERANCE < right["payloadFrame"][3]
            candidate_overflow += right["contentOverflowBottom"] > TOLERANCE
require(candidate_below_floor == 0 and candidate_overflow == 0, "automatic-minimum candidate violates its text floor/overflow contract")

require(len(source_pairs) == 4320, "source-order pairing coverage changed")
for key, pair in source_pairs.items():
    require(set(pair) == {"forward", "reverse"}, f"missing source-order counterpart: {key}")
    left, right = pair["forward"], pair["reverse"]
    for a, b in zip(left["rowValues"], right["rowValues"]):
        max_order_delta = max(max_order_delta, abs(a - b))
        require(abs(a - b) <= TOLERANCE, f"source order changed track sizes: {key}")
    right_items = {item["id"]: item for item in right["items"]}
    for item in left["items"]:
        counterpart = right_items[item["id"]]
        d = delta(item["frame"], counterpart["frame"])
        max_order_delta = max(max_order_delta, d)
        require(d <= TOLERANCE, f"source order changed semantic frame: {key} {item['id']}")

require(len(alignment_pairs) == 4320, "alignment pairing coverage changed")
for key, pair in alignment_pairs.items():
    require(set(pair) == {"start", "stretch"}, f"missing alignment counterpart: {key}")
    start, stretch = pair["start"], pair["stretch"]
    for index, (a, b) in enumerate(zip(start["rowValues"], stretch["rowValues"])):
        change = b - a
        member = None if index == 0 else start["scene"]["trackMembers"][(index - 1) % 6]
        if member == "auto":
            require(change >= -TOLERANCE, f"stretch shrank auto track: {key}")
            auto_track_growth_count += change > TOLERANCE
            max_auto_stretch_delta = max(max_auto_stretch_delta, change)
        else:
            max_non_auto_stretch_delta = max(max_non_auto_stretch_delta, abs(change))
            require(abs(change) <= TOLERANCE, f"stretch changed non-auto track: {key}")

summary = {
    "schema": "spinon.c11-4.repeated-text-row-span-positions-4-6-auto-minimum-independent-verification.v1",
    "sceneCount": len(records),
    "pairedSceneCount": len(pairs),
    "captureSha256": hashlib.sha256(OBSERVATION.read_bytes()).hexdigest(),
    "fixtureSha256": hashlib.sha256(FIXTURE.read_bytes()).hexdigest(),
    "captureScriptSha256": hashlib.sha256(CAPTURE.read_bytes()).hexdigest(),
    "fitPositions": [4, 5, 6],
    "tailOrderCount": len(TAILS),
    "caps": sorted(CAPS),
    "eligibleItemOccurrences": eligible_occurrences,
    "baselineBelowContentFloor": baseline_below_floor,
    "candidateBelowContentFloor": candidate_below_floor,
    "candidateVisibleContentOverflow": candidate_overflow,
    "automaticMinimumChangedItemFrames": changed_item_frames,
    "maxTrackDeltaPx": max_track_delta,
    "maxItemFrameDeltaPx": max_item_delta,
    "maxSourceOrderDeltaPx": max_order_delta,
    "maxNonAutoStretchDeltaPx": max_non_auto_stretch_delta,
    "maxAutoTrackStretchDeltaPx": max_auto_stretch_delta,
    "autoTrackGrowthPairs": auto_track_growth_count,
    "toleranceCssPx": TOLERANCE,
}
(EVIDENCE / "matrix-verification.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
