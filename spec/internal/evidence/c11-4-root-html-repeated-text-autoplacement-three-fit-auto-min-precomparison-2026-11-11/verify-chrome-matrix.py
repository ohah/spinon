#!/usr/bin/env python3
import hashlib
import itertools
import json
from collections import Counter
from pathlib import Path


EVIDENCE = Path(__file__).resolve().parent
OBSERVATIONS = EVIDENCE / "chrome157-observation.ndjson"
FIXTURE = EVIDENCE / "oracle-fixture.html"
COLLECTOR = EVIDENCE / "capture-chrome.mjs"
LIFECYCLE = EVIDENCE / "agent-browser-lifecycle.json"
OUTPUT = EVIDENCE / "verification.json"
EXPECTED_LINES = [2, 2, 2, 3, 2, 2, 3, 1]
CAPS = ["0px", "10px", "20%"]
TAILS = ["min-content", "max-content", "auto"]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def near(actual: float, expected: float, tolerance: float = 0.01) -> bool:
    return abs(actual - expected) <= tolerance


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_observations() -> tuple[dict, list[dict]]:
    rows = [json.loads(line) for line in OBSERVATIONS.read_text().splitlines()]
    require(bool(rows) and rows[0].get("record") == "metadata", "metadata record is missing")
    return rows[0], rows[1:]


metadata, scenes = read_observations()
require(metadata["schema"] == "spinon.c11-4.repeated-text-autoplacement-three-fit-auto-min-precomparison/v1", "unexpected observation schema")
require(metadata["sceneCount"] == 5760 and len(scenes) == 5760, "expected 5,760 observations")
require("HeadlessChrome/157.0.0.0" in metadata["userAgent"], "expected HeadlessChrome 157")
require(metadata["platform"] == "MacIntel" and metadata["devicePixelRatio"] == 1, "unexpected platform or DPR")
require(metadata["viewportCssPx"] == [1280, 900], "unexpected viewport")
require(metadata["fixtureSha256"] == sha256(FIXTURE), "fixture hash differs from capture metadata")
require(metadata["collectorSha256"] == sha256(COLLECTOR), "collector hash differs from capture metadata")
lifecycle = json.loads(LIFECYCLE.read_text())
require(lifecycle["opened"] and lifecycle["closed"] and lifecycle["closeError"] is None, "Agent Browser did not close cleanly")
require(lifecycle["terminalBrowserUsed"] is False and lifecycle["simulatorStarted"] is False, "unexpected browser or simulator path")

expected_triples = set(itertools.combinations(range(1, 7), 3))
expected_tails = set(itertools.permutations(TAILS))
expected_caps = set(itertools.permutations(CAPS))
track_lists: set[str] = set()
triples_seen: set[tuple[int, ...]] = set()
tails_seen: set[tuple[str, ...]] = set()
caps_seen: set[tuple[str, ...]] = set()
mode_counts = Counter()
scene_by_key: dict[tuple[str, str, str], dict[str, dict]] = {}
baseline_floor_violations = 0
baseline_eligible_floor_violations = 0
candidate_floor_violations = 0
candidate_eligible_floor_violations = 0
fit_auto_floor_rescued = 0
start_row_totals: dict[str, set[float]] = {"0px": set(), "auto": set()}
stretch_row_totals: dict[str, set[float]] = {"0px": set(), "auto": set()}
baseline_visible_item_overflow_scenes = 0
candidate_visible_item_overflow_scenes = 0
overflow_beyond_grid_scenes = 0
scene_by_list_alignment_order_min: dict[tuple[str, str, str, str], dict] = {}

for expected_index, record in enumerate(scenes):
    require(record["index"] == expected_index, f"non-contiguous scene index at {expected_index}")
    scene = record["scene"]
    require(scene["fitPositions"] == sorted(set(scene["fitPositions"])), f"scene {expected_index}: fit positions are not unique and sorted")
    require(len(scene["fitPositions"]) == 3 and tuple(scene["fitPositions"]) in expected_triples, f"scene {expected_index}: unexpected fit-position triple")
    require(len(scene["fitCaps"]) == 3 and tuple(scene["fitCaps"]) in expected_caps, f"scene {expected_index}: unexpected cap assignment")
    require(tuple(scene["tail"]) in expected_tails, f"scene {expected_index}: unexpected tail order")
    require(len(scene["trackMembers"]) == 6, f"scene {expected_index}: wrong track-list length")
    require(sum(member.startswith("fit-content(") for member in scene["trackMembers"]) == 3, f"scene {expected_index}: fit track multiplicity changed")
    require(scene["trackMembers"].count("min-content") == 1 and scene["trackMembers"].count("max-content") == 1 and scene["trackMembers"].count("auto") == 1, f"scene {expected_index}: tail multiplicity changed")
    require(scene["trackList"] == " ".join(scene["trackMembers"]), f"scene {expected_index}: authored track order changed")
    for position, cap in zip(scene["fitPositions"], scene["fitCaps"]):
        require(scene["trackMembers"][position - 1] == f"fit-content({cap})", f"scene {expected_index}: cap moved from its authored position")
    fit_positions = set(scene["fitPositions"])
    require([member for index, member in enumerate(scene["trackMembers"], start=1) if index not in fit_positions] == scene["tail"], f"scene {expected_index}: intrinsic tail order changed")
    require(scene["alignContent"] in {"start", "stretch"}, f"scene {expected_index}: unsupported alignment")
    require(scene["domOrder"] in {"forward", "reverse"}, f"scene {expected_index}: unsupported DOM order")
    require(scene["minHeight"] in {"0px", "auto"}, f"scene {expected_index}: unsupported min-height")
    mode_counts[(scene["minHeight"], scene["alignContent"], scene["domOrder"])] += 1
    triples_seen.add(tuple(scene["fitPositions"]))
    tails_seen.add(tuple(scene["tail"]))
    caps_seen.add(tuple(scene["fitCaps"]))
    track_lists.add(scene["trackList"])
    scene_by_list_alignment_order_min[(scene["trackList"], scene["alignContent"], scene["domOrder"], scene["minHeight"])] = record

    require(len(record["rows"]) == 9 and len(record["items"]) == 9, f"scene {expected_index}: expected explicit row plus eight implicit rows")
    require(record["items"][0]["id"] == "empty-0" and record["items"][0]["minHeight"] == "0px", f"scene {expected_index}: explicit row control changed")
    require(record["items"][0]["frame"] == [0, 0, 40, 10], f"scene {expected_index}: explicit-row control geometry changed")
    require(all(value >= 0 for value in record["rows"]), f"scene {expected_index}: negative used row")
    require(record["computedAutoRows"] == scene["trackList"], f"scene {expected_index}: computed authored track list changed")
    require(record["computedAlignContent"] == scene["alignContent"], f"scene {expected_index}: computed alignment changed")
    require(near(record["gridFrame"][0], 40) and near(record["gridFrame"][1], 220), f"scene {expected_index}: definite Grid frame changed")
    require(near(record["rows"][0], 10), f"scene {expected_index}: explicit row changed")

    expected_ids = list(range(1, 9)) if scene["domOrder"] == "forward" else list(range(8, 0, -1))
    top = 10.0
    scene_item_overflow = False
    for row_index, item_id in enumerate(expected_ids, start=1):
        item = record["items"][row_index]
        row_size = record["rows"][row_index]
        payload_height = EXPECTED_LINES[item_id - 1] * 10
        require(item["id"] == str(item_id), f"scene {expected_index}: item source order changed at implicit row {row_index}")
        require(item["minHeight"] == scene["minHeight"], f"scene {expected_index}: item minimum differs from scene mode")
        require(near(item["frame"][0], 0) and near(item["frame"][1], top) and near(item["frame"][2], 40) and near(item["frame"][3], row_size), f"scene {expected_index}: item frame differs from its row")
        require(item["payload"] is not None, f"scene {expected_index}: payload is missing")
        require(near(item["payload"][0], 0) and near(item["payload"][1], top) and near(item["payload"][2], 40) and near(item["payload"][3], payload_height), f"scene {expected_index}: fixed payload geometry changed")
        require(item["lineCount"] == EXPECTED_LINES[item_id - 1], f"scene {expected_index}: text line count changed")
        require(len(item["lineTops"]) == item["lineCount"], f"scene {expected_index}: line fragment count changed")
        require(all(near(item["lineTops"][line] - item["lineTops"][line - 1], 10) for line in range(1, len(item["lineTops"]))), f"scene {expected_index}: line spacing changed")
        require(near(item["lineTops"][0] - item["payload"][1], -1), f"scene {expected_index}: first text Range rect offset changed")
        track_member = scene["trackMembers"][(row_index - 1) % 6]
        eligible_track = track_member == "auto" or track_member.startswith("fit-content(")
        below_floor = row_size + 0.01 < payload_height
        if below_floor:
            scene_item_overflow = True
            if scene["minHeight"] == "0px":
                baseline_floor_violations += 1
                if eligible_track:
                    baseline_eligible_floor_violations += 1
            else:
                candidate_floor_violations += 1
                if eligible_track:
                    candidate_eligible_floor_violations += 1
        top += row_size
    if scene_item_overflow and scene["minHeight"] == "0px":
        baseline_visible_item_overflow_scenes += 1
    elif scene_item_overflow and scene["minHeight"] == "auto":
        candidate_visible_item_overflow_scenes += 1
    if record["items"][-1]["frame"][1] + record["items"][-1]["frame"][3] > 220.01:
        overflow_beyond_grid_scenes += 1
    scene_by_list_alignment_order_min[(scene["trackList"], scene["alignContent"], scene["domOrder"], scene["minHeight"])] = record
    total = round(sum(record["rows"]), 3)
    (start_row_totals if scene["alignContent"] == "start" else stretch_row_totals)[scene["minHeight"]].add(total)

require(triples_seen == expected_triples, "fit-position triple coverage is incomplete")
require(tails_seen == expected_tails, "tail-order coverage is incomplete")
require(caps_seen == expected_caps, "cap assignment coverage is incomplete")
require(len(track_lists) == 720, f"expected 720 unique track lists, got {len(track_lists)}")
require(all(count == 720 for count in mode_counts.values()) and len(mode_counts) == 8, "each minimum/alignment/order cohort must contain all 720 lists")
require(candidate_floor_violations == 0 and candidate_eligible_floor_violations == 0, "min-height:auto candidate fell below payload content floor")
require(baseline_eligible_floor_violations > 0, "baseline did not expose an eligible auto-minimum floor case")
require(candidate_visible_item_overflow_scenes == 0, "min-height:auto candidate has visible payload overflow from an undersized row")
require(overflow_beyond_grid_scenes == 0, "last item extends beyond the definite Grid block size")
require(start_row_totals["auto"] == {180.0}, f"candidate start rows must sum to 180px, got {start_row_totals['auto']}")
require(stretch_row_totals["auto"] == {220.0}, f"candidate stretch rows must sum to 220px, got {stretch_row_totals['auto']}")

paired_keys = set()
for (track_list, align, order, mode), record in scene_by_list_alignment_order_min.items():
    if mode != "auto":
        continue
    baseline = scene_by_list_alignment_order_min[(track_list, align, order, "0px")]
    paired_keys.add((track_list, align, order))
    for row_index, item in enumerate(record["items"][1:], start=1):
        baseline_row = baseline["rows"][row_index]
        payload_height = item["payload"][3]
        member = record["scene"]["trackMembers"][(row_index - 1) % 6]
        if member == "auto" or member.startswith("fit-content("):
            if baseline_row + 0.01 < payload_height:
                require(record["rows"][row_index] + 0.01 >= payload_height, "auto minimum did not restore an eligible item floor")
                fit_auto_floor_rescued += 1

require(len(paired_keys) == 2880, f"expected 2,880 paired baseline/candidate keys, got {len(paired_keys)}")
require(fit_auto_floor_rescued == baseline_eligible_floor_violations, "not every eligible baseline floor case was restored")

auto_stretch_deltas = Counter()
non_auto_stretch_max_delta = 0.0
auto_occurrence_counts = set()
for track_list in track_lists:
    for dom_order in ("forward", "reverse"):
        start = scene_by_list_alignment_order_min[(track_list, "start", dom_order, "auto")]
        stretch = scene_by_list_alignment_order_min[(track_list, "stretch", dom_order, "auto")]
        auto_indices = []
        for row_index in range(1, 9):
            member = start["scene"]["trackMembers"][(row_index - 1) % 6]
            delta = stretch["rows"][row_index] - start["rows"][row_index]
            if member == "auto":
                auto_indices.append(row_index)
                auto_stretch_deltas[round(delta, 3)] += 1
            else:
                non_auto_stretch_max_delta = max(non_auto_stretch_max_delta, abs(delta))
        auto_occurrence_counts.add(len(auto_indices))
        expected_delta = 40 / len(auto_indices)
        require(all(near(stretch["rows"][index] - start["rows"][index], expected_delta) for index in auto_indices), f"{track_list}: stretch is not divided across auto-row occurrences")
        require(near(sum(stretch["rows"]) - sum(start["rows"]), 40), f"{track_list}: stretch did not distribute 40px")

require(non_auto_stretch_max_delta <= 0.01, "align-content:stretch changed a non-auto row")
require(auto_occurrence_counts == {1, 2}, f"unexpected auto-row occurrence counts: {auto_occurrence_counts}")

summary = {
    "schema": "spinon.c11-4.repeated-text-autoplacement-three-fit-auto-min-verification/v1",
    "browser": metadata["userAgent"],
    "fixtureSha256": metadata["fixtureSha256"],
    "collectorSha256": metadata["collectorSha256"],
    "sceneCount": len(scenes),
    "pairedMinHeight0AutoKeys": len(paired_keys),
    "uniqueAuthoredTrackLists": len(track_lists),
    "fitPositionTriples": len(triples_seen),
    "tailOrders": len(tails_seen),
    "orderedCapAssignments": len(caps_seen),
    "baselineRowsBelowPayloadFloor": baseline_floor_violations,
    "baselineEligibleAutoMinimumRowsBelowFloor": baseline_eligible_floor_violations,
    "candidateRowsBelowPayloadFloor": candidate_floor_violations,
    "candidateEligibleAutoMinimumRowsBelowFloor": candidate_eligible_floor_violations,
    "eligibleRowsRaisedToPayloadFloor": fit_auto_floor_rescued,
    "candidateStartTrackTotalCssPx": sorted(start_row_totals["auto"]),
    "baselineStartTrackTotalsCssPx": sorted(start_row_totals["0px"]),
    "candidateStretchTrackTotalCssPx": sorted(stretch_row_totals["auto"]),
    "autoTrackOccurrenceCounts": sorted(auto_occurrence_counts),
    "autoTrackStretchDeltasCssPx": sorted(auto_stretch_deltas),
    "nonAutoStretchMaxDeltaCssPx": round(non_auto_stretch_max_delta, 5),
    "baselineScenesWithVisibleItemOverflow": baseline_visible_item_overflow_scenes,
    "candidateScenesWithVisibleItemOverflow": candidate_visible_item_overflow_scenes,
    "scenesWhoseLastItemCrossesGridEnd": overflow_beyond_grid_scenes,
    "agentBrowserSessionClosed": lifecycle["closed"],
    "terminalBrowserUsed": lifecycle["terminalBrowserUsed"],
}
OUTPUT.write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
