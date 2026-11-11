import json
from collections import Counter, defaultdict
from itertools import permutations
from pathlib import Path


directory = Path(__file__).parent
observation = directory / "chrome157-observation.ndjson"
expected_lines = [2, 2, 2, 3, 2, 2, 3, 1]
cap_values = ("0px", "10px", "20%", "30px", "40px", "50px")
cap_orders = {tuple(value) for value in permutations(cap_values)}


def close(a, b, tolerance=1e-5):
    return abs(a - b) <= tolerance


with observation.open(encoding="utf-8") as stream:
    metadata = json.loads(next(stream))
    records = [json.loads(line) for line in stream]

assert metadata["sceneCount"] == 5_760
assert metadata["uniqueTrackListCount"] == 720
assert metadata["fitPositions"] == [1, 2, 3, 4, 5, 6]
assert metadata["orderedCapAssignments"] == 720
assert tuple(metadata["capValues"]) == cap_values
assert "HeadlessChrome/157." in metadata["userAgent"]
assert metadata["platform"] == "MacIntel" and metadata["devicePixelRatio"] == 1
assert len(records) == 5_760
assert [record["index"] for record in records] == list(range(5_760))

pairs = defaultdict(dict)
unique_track_lists = set()
seen_scene_keys = set()
candidate_fit_occurrences = 0
candidate_fit_floor_violations = []
baseline_fit_below_floor = 0
payload_row_overflow = Counter()
payload_grid_end_overflow = Counter()
item_grid_end_overflow = []
maximum_payload_end = Counter()
candidate_below_floor_by_cap = Counter()
baseline_below_floor_by_cap = Counter()

for record in records:
    scene = record["scene"]
    caps = tuple(scene["fitCaps"])
    assert caps in cap_orders
    assert len(scene["trackMembers"]) == 6
    assert scene["trackMembers"] == [f"fit-content({cap})" for cap in caps]
    assert scene["trackList"] == " ".join(scene["trackMembers"])
    unique_track_lists.add(scene["trackList"])
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
        track_position = (position - 1) % 6
        payload_height = item["payload"][3]
        cap = caps[track_position]
        if scene["minHeight"] == "auto":
            candidate_fit_occurrences += 1
            if item["frame"][3] + 1e-5 < payload_height:
                candidate_fit_floor_violations.append(
                    (record["index"], cap, item["id"], item["frame"][3], payload_height)
                )
                candidate_below_floor_by_cap[cap] += 1
        elif item["frame"][3] + 1e-5 < payload_height:
            baseline_fit_below_floor += 1
            baseline_below_floor_by_cap[cap] += 1

        if item["frame"][1] + item["frame"][3] > record["gridFrame"][1] + 1e-5:
            item_grid_end_overflow.append((record["index"], item["id"]))
        payload_end = item["payload"][1] + payload_height
        maximum_payload_end[scene["minHeight"]] = max(maximum_payload_end[scene["minHeight"]], payload_end)
        if payload_end > item["frame"][1] + item["frame"][3] + 1e-5:
            payload_row_overflow[scene["minHeight"]] += 1
        if payload_end > record["gridFrame"][1] + 1e-5:
            payload_grid_end_overflow[scene["minHeight"]] += 1

    pair_key = (scene["trackList"], scene["alignContent"], scene["domOrder"])
    mode = scene["minHeight"]
    assert mode not in pairs[pair_key]
    pairs[pair_key][mode] = record
    seen_scene_key = (*pair_key, mode)
    assert seen_scene_key not in seen_scene_keys
    seen_scene_keys.add(seen_scene_key)

assert len(unique_track_lists) == 720
assert len(pairs) == 2_880
assert all(set(modes) == {"0px", "auto"} for modes in pairs.values())
assert candidate_fit_occurrences == 23_040

paired_keys_with_changed_rows = 0
stretch_rows_differ_from_start = Counter()
stretch_geometry_differs_from_start = Counter()
for (track_list, alignment, dom_order), modes in pairs.items():
    if modes["0px"]["rows"] != modes["auto"]["rows"]:
        paired_keys_with_changed_rows += 1
    if alignment == "stretch":
        start = pairs[(track_list, "start", dom_order)]
        for minimum in ("0px", "auto"):
            if start[minimum]["rows"] != modes[minimum]["rows"]:
                stretch_rows_differ_from_start[minimum] += 1
            start_items = start[minimum]["items"]
            stretch_items = modes[minimum]["items"]
            if any(
                left["frame"] != right["frame"]
                or left["payload"] != right["payload"]
                or left["lineTops"] != right["lineTops"]
                for left, right in zip(start_items, stretch_items)
            ):
                stretch_geometry_differs_from_start[minimum] += 1

summary = {
    "schema": "spinon.c11-4.repeated-text-autoplacement-six-fit-auto-min-verification/v1",
    "browser": metadata["userAgent"],
    "platform": metadata["platform"],
    "dpr": metadata["devicePixelRatio"],
    "sceneCount": len(records),
    "pairedKeys": len(pairs),
    "uniqueTrackLists": len(unique_track_lists),
    "orderedCapAssignments": len(cap_orders),
    "candidateFitTrackItemOccurrences": candidate_fit_occurrences,
    "baselineFitOccurrencesBelowPayloadFloor": baseline_fit_below_floor,
    "candidateFitFloorViolations": len(candidate_fit_floor_violations),
    "baselineBelowFloorByCap": dict(sorted(baseline_below_floor_by_cap.items())),
    "candidateBelowFloorByCap": dict(sorted(candidate_below_floor_by_cap.items())),
    "visiblePayloadRowOverflow": dict(sorted(payload_row_overflow.items())),
    "pairedKeysWithChangedRows": paired_keys_with_changed_rows,
    "alignContentStretchRowsDifferFromStart": dict(sorted(stretch_rows_differ_from_start.items())),
    "alignContentStretchFullGeometryDiffersFromStart": dict(sorted(stretch_geometry_differs_from_start.items())),
    "itemFrameGridEndOverflows": len(item_grid_end_overflow),
    "payloadGridEndOverflows": dict(sorted(payload_grid_end_overflow.items())),
    "maximumPayloadEndByMinHeight": dict(maximum_payload_end),
}
(directory / "verification.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
print(json.dumps(summary, separators=(",", ":")))
