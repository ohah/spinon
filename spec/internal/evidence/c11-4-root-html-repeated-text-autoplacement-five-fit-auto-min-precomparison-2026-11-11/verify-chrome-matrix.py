import json
from collections import Counter, defaultdict
from itertools import combinations, permutations
from pathlib import Path


directory = Path(__file__).parent
observation = directory / "chrome157-observation.ndjson"
expected_lines = [2, 2, 2, 3, 2, 2, 3, 1]
fit_position_sets = {tuple(value) for value in combinations(range(1, 7), 5)}
tail_options = {("min-content",), ("max-content",), ("auto",)}
cap_orders = {
    tuple(value)
    for value in permutations(("0px", "10px", "20%", "30px", "40px"))
}


def close(a, b, tolerance=1e-5):
    return abs(a - b) <= tolerance


with observation.open() as stream:
    metadata = json.loads(next(stream))
    records = [json.loads(line) for line in stream]

assert metadata["sceneCount"] == 17_280
assert metadata["uniqueTrackListCount"] == 2_160
assert metadata["fitPositionQuintuples"] == 6
assert metadata["tailOptions"] == 3
assert metadata["orderedCapAssignments"] == 120
assert "HeadlessChrome/157." in metadata["userAgent"]
assert metadata["platform"] == "MacIntel" and metadata["devicePixelRatio"] == 1
assert len(records) == 17_280
assert [record["index"] for record in records] == list(range(17_280))

pairs = defaultdict(dict)
unique_track_lists = set()
seen_scene_keys = set()
candidate_fit_occurrences = 0
candidate_fit_floor_violations = []
baseline_fit_below_floor = 0
tail_floor_violations = Counter()
payload_row_overflow = Counter()
item_grid_end_overflow = []
payload_grid_end_overflow = []
maximum_payload_end = Counter()
payload_grid_end_overflow_count = Counter()
stretch_rows_differ_by_tail_and_mode = Counter()

for record in records:
    scene = record["scene"]
    fit_positions = tuple(scene["fitPositions"])
    tail = tuple(scene["tail"])
    caps = tuple(scene["fitCaps"])
    unique_track_lists.add(scene["trackList"])
    assert fit_positions in fit_position_sets
    assert tail in tail_options
    assert caps in cap_orders
    assert len(scene["trackMembers"]) == 6
    assert len(fit_positions) == len(caps) == 5
    assert sum(member.startswith("fit-content(") for member in scene["trackMembers"]) == 5
    assert sum(member in {"min-content", "max-content", "auto"} for member in scene["trackMembers"]) == 1
    assert record["computedAutoRows"] == scene["trackList"]
    assert record["computedAlignContent"] == scene["alignContent"]
    assert close(record["gridFrame"][0], 40) and close(record["gridFrame"][1], 220)
    assert len(record["rows"]) == 9 and close(record["rows"][0], 10)
    assert len(record["items"]) == 9 and record["items"][0]["id"] == "empty-0"

    direct_items = record["items"][1:]
    expected_ids = [str(index) for index in range(1, 9)]
    if scene["domOrder"] == "reverse":
        expected_ids.reverse()
    assert [item["id"] for item in direct_items] == expected_ids

    for position, item in enumerate(direct_items, start=1):
        assert item["minHeight"] == scene["minHeight"]
        assert item["lineCount"] == expected_lines[int(item["id"]) - 1]
        assert item["lineCount"] == len(item["lineTops"])
        assert item["lineTops"] and all(
            close(item["lineTops"][index] - item["lineTops"][index - 1], 10)
            for index in range(1, len(item["lineTops"]))
        )
        track_size = record["rows"][position]
        assert close(item["frame"][3], track_size), (record["index"], item["id"], item["frame"], track_size)
        track_position = (position - 1) % 6 + 1
        payload_height = item["payload"][3]
        is_fit_track = track_position in fit_positions
        if is_fit_track and scene["minHeight"] == "auto":
            candidate_fit_occurrences += 1
            if item["frame"][3] + 1e-5 < payload_height:
                candidate_fit_floor_violations.append(
                    (record["index"], tail, track_position, item["id"], item["frame"][3], payload_height)
                )
        elif is_fit_track and item["frame"][3] + 1e-5 < payload_height:
            baseline_fit_below_floor += 1
        elif not is_fit_track and item["frame"][3] + 1e-5 < payload_height:
            tail_floor_violations[(tail[0], scene["minHeight"])] += 1

        if item["frame"][1] + item["frame"][3] > record["gridFrame"][1] + 1e-5:
            item_grid_end_overflow.append((record["index"], item["id"]))
        payload_end = item["payload"][1] + payload_height
        maximum_payload_end[scene["minHeight"]] = max(maximum_payload_end[scene["minHeight"]], payload_end)
        if payload_end > item["frame"][1] + item["frame"][3] + 1e-5:
            payload_row_overflow[(tail[0], scene["minHeight"])] += 1
        if payload_end > record["gridFrame"][1] + 1e-5:
            payload_grid_end_overflow.append((record["index"], item["id"], payload_end))
            payload_grid_end_overflow_count[(tail[0], scene["minHeight"])] += 1

    pair_key = (scene["trackList"], scene["alignContent"], scene["domOrder"])
    mode = scene["minHeight"]
    assert mode not in pairs[pair_key]
    pairs[pair_key][mode] = record
    seen_scene_key = (*pair_key, mode)
    assert seen_scene_key not in seen_scene_keys
    seen_scene_keys.add(seen_scene_key)

assert len(unique_track_lists) == 2_160
assert len(pairs) == 8_640
assert all(set(modes) == {"0px", "auto"} for modes in pairs.values())

paired_keys_with_changed_rows = 0
stretch_rows_differ_from_start = 0
for (track_list, alignment, dom_order), modes in pairs.items():
    if modes["0px"]["rows"] != modes["auto"]["rows"]:
        paired_keys_with_changed_rows += 1
    if alignment == "stretch":
        start = pairs[(track_list, "start", dom_order)]
        for minimum in ("0px", "auto"):
            if start[minimum]["rows"] != modes[minimum]["rows"]:
                stretch_rows_differ_from_start += 1
                tail = modes[minimum]["scene"]["tail"][0]
                stretch_rows_differ_by_tail_and_mode[(tail, minimum)] += 1

summary = {
    "schema": "spinon.c11-4.repeated-text-autoplacement-five-fit-auto-min-verification/v1",
    "browser": metadata["userAgent"],
    "platform": metadata["platform"],
    "dpr": metadata["devicePixelRatio"],
    "sceneCount": len(records),
    "pairedKeys": len(pairs),
    "uniqueTrackLists": len(unique_track_lists),
    "fitPositionQuintuples": len(fit_position_sets),
    "tailOptions": sorted(value[0] for value in tail_options),
    "orderedCapAssignments": len(cap_orders),
    "candidateFitTrackItemOccurrences": candidate_fit_occurrences,
    "baselineFitOccurrencesBelowPayloadFloor": baseline_fit_below_floor,
    "candidateFitFloorViolations": len(candidate_fit_floor_violations),
    "tailPayloadFloorViolations": {f"{tail}/{minimum}": count for (tail, minimum), count in sorted(tail_floor_violations.items())},
    "visiblePayloadRowOverflow": {f"{tail}/{minimum}": count for (tail, minimum), count in sorted(payload_row_overflow.items())},
    "pairedKeysWithChangedRows": paired_keys_with_changed_rows,
    "alignContentStretchRowsDifferFromStart": stretch_rows_differ_from_start,
    "alignContentStretchRowsDifferByTailAndMode": {f"{tail}/{minimum}": count for (tail, minimum), count in sorted(stretch_rows_differ_by_tail_and_mode.items())},
    "itemFrameGridEndOverflows": len(item_grid_end_overflow),
    "payloadGridEndOverflows": len(payload_grid_end_overflow),
    "payloadGridEndOverflowsByTailAndMode": {f"{tail}/{minimum}": count for (tail, minimum), count in sorted(payload_grid_end_overflow_count.items())},
    "maximumPayloadEndByMinHeight": dict(maximum_payload_end),
}
(directory / "verification.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
print(json.dumps(summary, separators=(",", ":")))
