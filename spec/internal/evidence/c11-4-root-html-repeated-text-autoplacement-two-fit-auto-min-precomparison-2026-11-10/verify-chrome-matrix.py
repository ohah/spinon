#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path


EVIDENCE = Path(__file__).resolve().parent
ROOT = EVIDENCE.parents[3]
OBSERVATIONS = EVIDENCE / "chrome-matrix-observation.ndjson"
BASELINE = (
    ROOT
    / "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/chrome-matrix-observation.ndjson"
)
FIXTURE = EVIDENCE / "oracle-fixture.html"
COLLECTOR = EVIDENCE / "capture-chrome.mjs"
LIFECYCLE = EVIDENCE / "agent-browser-lifecycle.json"
OUTPUT = EVIDENCE / "matrix-verification.json"

EXPECTED_LINES = [2, 2, 2, 3, 2, 2, 3, 1]
EXPECTED_CAP_ASSIGNMENTS = {
    ("0px", "0%"),
    ("0%", "0px"),
    ("10px", "20px"),
    ("20px", "10px"),
    ("10px", "20%"),
    ("20%", "10px"),
    ("20%", "120%"),
    ("120%", "20%"),
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def near(actual: float, expected: float, tolerance: float = 0.01) -> bool:
    return abs(actual - expected) <= tolerance


def read_ndjson(path: Path) -> tuple[dict, list[dict]]:
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    require(bool(rows), f"{path}: file is empty")
    require(rows[0].get("record") == "metadata", f"{path}: metadata record is missing")
    return rows[0], rows[1:]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


metadata, scenes = read_ndjson(OBSERVATIONS)
baseline_metadata, baseline_scenes = read_ndjson(BASELINE)
require(metadata["schema"] == "spinon.c11-4.repeated-text-autoplacement-two-fit-content-auto-min-precomparison/v1", "unexpected observation schema")
require(metadata["sceneCount"] == 2880 and len(scenes) == 2880, "expected exactly 2,880 Chrome scenes")
require(len(baseline_scenes) == 2880, "min-height:0 baseline must contain 2,880 scenes")
require("HeadlessChrome/157.0.0.0" in metadata["userAgent"], "expected HeadlessChrome 157")
require(metadata["platform"] == "MacIntel", "unexpected Chrome platform")
require(metadata["devicePixelRatio"] == 1, "expected DPR 1")
require(metadata["viewportCssPx"] == [1280, 900], "unexpected viewport")
require(metadata["fixtureSha256"] == sha256(FIXTURE), "fixture SHA-256 does not match metadata")
require(metadata["collectorSha256"] == sha256(COLLECTOR), "collector SHA-256 does not match metadata")
lifecycle = json.loads(LIFECYCLE.read_text())
require(lifecycle["opened"] and lifecycle["closed"], "Agent Browser session was not closed cleanly")
require(lifecycle["closeError"] is None, "Agent Browser reported a close error")
require(lifecycle["terminalBrowserUsed"] is False, "Terminal Browser must not be used")

expected_fit_pairs = set(itertools.combinations(range(1, 7), 2))
expected_tails = set(
    itertools.permutations(["min-content", "min-content", "max-content", "auto"])
)
scene_by_track_list: dict[str, dict[str, dict]] = {}
fit_positions_seen = set()
cap_assignments_seen = set()
tails_seen = set()
alignments = {"start": 0, "stretch": 0}
fit_occurrences = 0
start_fit_occurrences = 0
fit_floor_rescued = 0
content_floor_violations = 0
overflow_scenes = 0

for index, (scene, baseline) in enumerate(zip(scenes, baseline_scenes)):
    require(scene["index"] == index, f"scene index discontinuity at {index}")
    require(baseline["index"] == index, f"baseline index discontinuity at {index}")
    current = scene["scene"]
    previous = baseline["scene"]
    require(current["trackList"] == previous["trackList"], f"scene {index}: baseline track list changed")
    require(current["alignContent"] == previous["alignContent"], f"scene {index}: baseline alignment changed")
    require(len(current["fitPositions"]) == 2, f"scene {index}: expected two fit positions")
    require(current["fitPositions"][0] < current["fitPositions"][1], f"scene {index}: fit positions are not distinct and ordered")
    require(tuple(current["fitPositions"]) in expected_fit_pairs, f"scene {index}: unsupported fit position pair")
    require(tuple(current["fitCaps"]) in EXPECTED_CAP_ASSIGNMENTS, f"scene {index}: unexpected cap assignment")
    require(tuple(current["tail"]) in expected_tails, f"scene {index}: unexpected tail order")
    require(current["trackMembers"].count("min-content") == 2, f"scene {index}: min-content multiplicity changed")
    require(current["trackMembers"].count("max-content") == 1, f"scene {index}: max-content multiplicity changed")
    require(current["trackMembers"].count("auto") == 1, f"scene {index}: auto multiplicity changed")
    require(sum(member.startswith("fit-content(") for member in current["trackMembers"]) == 2, f"scene {index}: fit-content multiplicity changed")
    require(current["trackList"] == " ".join(current["trackMembers"]), f"scene {index}: authored member order changed")
    require(current["alignContent"] in alignments, f"scene {index}: unsupported alignment")
    alignments[current["alignContent"]] += 1
    fit_positions_seen.add(tuple(current["fitPositions"]))
    cap_assignments_seen.add(tuple(current["fitCaps"]))
    tails_seen.add(tuple(current["tail"]))

    require(len(scene["rows"]) == 9 and len(scene["items"]) == 9, f"scene {index}: expected one explicit and eight implicit rows")
    require(scene["items"][0]["minHeight"] == "0px", f"scene {index}: explicit-row control minimum changed")
    require(all(item["minHeight"] == "auto" for item in scene["items"][1:]), f"scene {index}: an auto-placement item lost computed min-height:auto")
    require(all(value >= 0 for value in scene["rows"]), f"scene {index}: negative used row size")
    require(near(scene["gridFrame"][0], 40) and near(scene["gridFrame"][1], 220), f"scene {index}: Grid frame changed")
    require(near(scene["rows"][0], 10), f"scene {index}: explicit row changed")
    require(scene["computedAutoRows"] == current["trackList"], f"scene {index}: computed track order or cap values changed")
    require(scene["computedAlignContent"] == current["alignContent"], f"scene {index}: computed alignment changed")
    require(scene["items"][0]["id"] == "empty-0", f"scene {index}: explicit-row control moved")
    require(scene["items"][0]["frame"] == [0, 0, 40, 10], f"scene {index}: explicit-row control frame changed")

    top = 10.0
    scene_overflows = False
    for item_index, expected_line_count in enumerate(EXPECTED_LINES, start=1):
        item = scene["items"][item_index]
        baseline_item = baseline["items"][item_index]
        row_size = scene["rows"][item_index]
        payload_floor = item["payload"][3]
        require(item["id"] == str(item_index), f"scene {index}, item {item_index}: DOM order changed")
        require(near(baseline_item["payload"][3], payload_floor), f"scene {index}, item {item_index}: payload changed from the min-height:0 baseline")
        require(item["lineCount"] == expected_line_count, f"scene {index}, item {item_index}: line count changed")
        require(near(item["payload"][0], 0) and near(item["payload"][1], top), f"scene {index}, item {item_index}: payload origin changed")
        require(near(item["payload"][2], 40) and near(payload_floor, expected_line_count * 10), f"scene {index}, item {item_index}: payload geometry changed")
        require(near(item["frame"][0], 0) and near(item["frame"][1], top), f"scene {index}, item {item_index}: item position does not follow the implicit row")
        require(near(item["frame"][2], 40) and near(item["frame"][3], row_size), f"scene {index}, item {item_index}: item frame does not match the used row")
        require(len(item["lineTops"]) == expected_line_count, f"scene {index}, item {item_index}: line rectangles changed")
        if row_size + 0.01 < payload_floor:
            content_floor_violations += 1
        require(row_size + 0.01 >= payload_floor, f"scene {index}, item {item_index}: used row fell below its text content floor")
        if current["alignContent"] == "start":
            require(near(row_size, payload_floor), f"scene {index}, item {item_index}: start row is not its content-based minimum")
            start_fit_occurrences += int(current["trackMembers"][(item_index - 1) % 6].startswith("fit-content("))
        pos = (item_index - 1) % 6
        if current["trackMembers"][pos].startswith("fit-content("):
            fit_occurrences += 1
            if current["alignContent"] == "start":
                if baseline["rows"][item_index] + 0.01 < payload_floor:
                    require(row_size + 0.01 >= payload_floor, f"scene {index}, item {item_index}: automatic minimum did not rescue the fit row")
                    fit_floor_rescued += 1
        if top + payload_floor > 220.01:
            scene_overflows = True
        top += row_size
    overflow_scenes += int(scene_overflows)

    scene_by_track_list.setdefault(current["trackList"], {})[current["alignContent"]] = scene

require(fit_positions_seen == expected_fit_pairs, "fit position pair coverage is incomplete")
require(cap_assignments_seen == EXPECTED_CAP_ASSIGNMENTS, "ordered px/percentage cap assignments are incomplete")
require(tails_seen == expected_tails, "tail-order coverage is incomplete")
require(alignments == {"start": 1440, "stretch": 1440}, "start/stretch scene counts are not paired")
require(len(scene_by_track_list) == 1440, "expected 1,440 unique authored lists")
require(fit_occurrences == 7680, "fit-content row occurrence coverage is incomplete")
require(start_fit_occurrences == 3840, "start-aligned fit-content occurrence count changed")
require(fit_floor_rescued == 1920, "the min-height:0 comparison did not isolate 1,920 fit-row floor cases")
require(content_floor_violations == 0, "unexpected content floor violation")
require(overflow_scenes == 0, "auto-minimum matrix unexpectedly overflows the 220px grid")

auto_stretch_deltas = set()
auto_track_occurrence_counts = set()
non_auto_stretch_delta_max = 0.0
for track_list, alignments_for_list in scene_by_track_list.items():
    require(set(alignments_for_list) == {"start", "stretch"}, f"track list lacks an alignment pair: {track_list}")
    start = alignments_for_list["start"]
    stretch = alignments_for_list["stretch"]
    require(near(sum(start["rows"]), 180), f"{track_list}: start rows do not total 180px")
    require(near(sum(stretch["rows"]), 220), f"{track_list}: stretch rows do not fill 220px")
    auto_deltas = []
    for item_index in range(1, 9):
        delta = stretch["rows"][item_index] - start["rows"][item_index]
        position = (item_index - 1) % 6
        if start["scene"]["trackMembers"][position] == "auto":
            auto_deltas.append(delta)
        else:
            non_auto_stretch_delta_max = max(non_auto_stretch_delta_max, abs(delta))
    require(bool(auto_deltas), f"{track_list}: no repeated auto-row occurrence")
    expected_delta = 40.0 / len(auto_deltas)
    require(all(near(delta, expected_delta) for delta in auto_deltas), f"{track_list}: free space was not shared across auto-row occurrences")
    auto_track_occurrence_counts.add(len(auto_deltas))
    auto_stretch_deltas.update(round(delta, 4) for delta in auto_deltas)

require(non_auto_stretch_delta_max <= 0.01, "stretch changed a non-auto track")
require(auto_track_occurrence_counts == {1, 2}, "auto-row repetition counts changed")
require(auto_stretch_deltas == {20.0, 40.0}, "auto-row stretch deltas changed")

summary = {
    "schema": "spinon.c11-4.repeated-text-autoplacement-two-fit-content-auto-min-verification/v1",
    "browser": metadata["userAgent"],
    "fixtureSha256": metadata["fixtureSha256"],
    "collectorSha256": metadata["collectorSha256"],
    "sceneCount": len(scenes),
    "uniqueAuthoredTrackLists": len(scene_by_track_list),
    "fitPositionPairs": len(fit_positions_seen),
    "orderedCapAssignments": len(cap_assignments_seen),
    "tailOrders": len(tails_seen),
    "alignmentPairs": len(scene_by_track_list),
    "fitRowOccurrencesAcrossBothAlignments": fit_occurrences,
    "startFitRowOccurrences": start_fit_occurrences,
    "fitRowsRaisedToTextFloorVsMinHeightZero": fit_floor_rescued,
    "contentFloorViolations": content_floor_violations,
    "startTrackTotalCssPx": 180,
    "stretchTrackTotalCssPx": 220,
    "stretchAddedSpaceCssPx": 40,
    "autoTrackOccurrenceCounts": sorted(auto_track_occurrence_counts),
    "autoTrackStretchDeltasCssPx": sorted(auto_stretch_deltas),
    "nonAutoStretchMaxDeltaCssPx": round(non_auto_stretch_delta_max, 4),
    "autoMinVisibleOverflowScenes": overflow_scenes,
    "agentBrowserSessionClosed": lifecycle["closed"],
    "terminalBrowserUsed": lifecycle["terminalBrowserUsed"],
}
OUTPUT.write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, indent=2))
