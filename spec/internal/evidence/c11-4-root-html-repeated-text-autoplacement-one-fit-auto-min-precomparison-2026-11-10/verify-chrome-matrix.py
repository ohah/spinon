#!/usr/bin/env python3
"""Independently check the one-fit auto-minimum Chrome matrix."""

from __future__ import annotations

import hashlib
import itertools
import json
from collections import Counter
from pathlib import Path


EVIDENCE = Path(__file__).resolve().parent
ROOT = EVIDENCE.parents[3]
OBSERVATION = EVIDENCE / "chrome-matrix-observation.ndjson"
BASELINE = (
    ROOT
    / "spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/chrome157-matrix-observation.ndjson"
)
FIXTURE = EVIDENCE / "oracle-fixture.html"
COLLECTOR = EVIDENCE / "capture-chrome.mjs"
LIFECYCLE = EVIDENCE / "agent-browser-lifecycle.json"
SUMMARY = EVIDENCE / "matrix-verification.json"

EXPECTED_LINES = [2, 2, 2, 3, 2, 2, 3, 1]
EXPECTED_CAPS = {"20px": 20.0, "20%": 44.0, "0%": 0.0, "120%": 264.0}
TAILS = set(itertools.permutations(["min-content", "min-content", "max-content", "max-content", "auto"]))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def close(actual: float, expected: float, tolerance: float = 0.01) -> bool:
    return abs(actual - expected) <= tolerance


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_matrix(path: Path) -> tuple[dict, list[dict]]:
    records = [json.loads(line) for line in path.read_text().splitlines()]
    require(bool(records) and records[0].get("record") == "metadata", f"{path}: missing metadata")
    require(all(record.get("record") == "scene" for record in records[1:]), f"{path}: unexpected record type")
    return records[0], records[1:]


metadata, scenes = read_matrix(OBSERVATION)
baseline_metadata, baseline_scenes = read_matrix(BASELINE)
require(metadata["schema"] == "spinon.c11-4.repeated-text-autoplacement-one-fit-auto-min-precomparison/v1", "unexpected schema")
require(metadata["sceneCount"] == 1440 and len(scenes) == 1440, "expected 1,440 candidate scenes")
require(len(baseline_scenes) == 1440, "baseline must contain 1,440 scenes")
require("HeadlessChrome/157.0.0.0" in metadata["userAgent"], "expected HeadlessChrome 157")
require(metadata["platform"] == "MacIntel" and metadata["devicePixelRatio"] == 1, "unexpected browser geometry environment")
require(metadata["viewportCssPx"] == [1280, 900], "unexpected viewport")
require(metadata["fixtureSha256"] == sha256(FIXTURE), "fixture hash mismatch")
require(metadata["collectorSha256"] == sha256(COLLECTOR), "collector hash mismatch")
require(metadata["payloadLineCounts"] == EXPECTED_LINES, "payload line counts changed")
lifecycle = json.loads(LIFECYCLE.read_text())
require(lifecycle["opened"] and lifecycle["closed"] and lifecycle["closeError"] is None, "Agent Browser session did not close cleanly")
require(lifecycle["terminalBrowserUsed"] is False and lifecycle["simulatorStarted"] is False, "unexpected browser or simulator path")

def key(scene: dict) -> tuple:
    return (scene["fitPosition"], tuple(scene["tail"]), scene["cap"], scene["align"])


candidate_by_key = {key(scene): scene for scene in scenes}
baseline_by_key = {key(scene): scene for scene in baseline_scenes}
require(len(candidate_by_key) == len(scenes) == 1440, "candidate matrix has duplicate keys")
require(len(baseline_by_key) == len(baseline_scenes) == 1440, "baseline matrix has duplicate keys")
require(candidate_by_key.keys() == baseline_by_key.keys(), "candidate and min-height:0 baseline inputs differ")

position_counts: Counter[int] = Counter()
cap_counts: Counter[str] = Counter()
tail_orders: set[tuple[str, ...]] = set()
alignments: Counter[str] = Counter()
fit_occurrences = 0
fit_floor_rescues = 0
content_floor_violations = 0
visible_overflow_scenes = 0
stretch_checks = 0
largest_overflow = 0.0

for scene_index, candidate in enumerate(scenes):
    baseline = baseline_by_key[key(candidate)]
    require(candidate["index"] == scene_index, f"scene index discontinuity at {scene_index}")
    require(candidate["fitPosition"] in range(1, 7), f"scene {scene_index}: invalid fit position")
    require(tuple(candidate["tail"]) in TAILS, f"scene {scene_index}: unknown tail order")
    require(candidate["cap"] in EXPECTED_CAPS, f"scene {scene_index}: unknown cap")
    require(candidate["align"] in {"start", "stretch"}, f"scene {scene_index}: unknown alignment")
    position_counts[candidate["fitPosition"]] += 1
    cap_counts[candidate["cap"]] += 1
    tail_orders.add(tuple(candidate["tail"]))
    alignments[candidate["align"]] += 1

    tracks = list(candidate["tail"])
    tracks.insert(candidate["fitPosition"] - 1, f"fit-content({candidate['cap']})")
    require(candidate["authoredAutoRows"] == tracks, f"scene {scene_index}: authored track order changed")
    require(candidate["computedAutoRows"].split() == tracks, f"scene {scene_index}: CSSOM track order changed")
    require(Counter(tracks) == Counter({"min-content": 2, "max-content": 2, "auto": 1, f"fit-content({candidate['cap']})": 1}),
        f"scene {scene_index}: track multiplicity changed")
    require(candidate["gridFrame"] == [40, 220] and len(candidate["rows"]) == 9, f"scene {scene_index}: grid frame/row count changed")
    require(len(candidate["items"]) == 9 and candidate["rows"][0] == 10, f"scene {scene_index}: explicit row/control changed")
    require(candidate["items"][0]["id"] == "empty-0" and candidate["items"][0]["minHeight"] == "0px", f"scene {scene_index}: control item changed")
    require(candidate["items"][0]["frame"] == [0, 0, 40, 10], f"scene {scene_index}: control frame changed")
    require(candidate["computedAutoRows"] == baseline["computedAutoRows"], f"scene {scene_index}: authored input differs from baseline")
    require(candidate["align"] == baseline["align"] and candidate["gridFrame"] == baseline["gridFrame"], f"scene {scene_index}: baseline scene metadata differs")

    top = 10.0
    scene_overflows = False
    for item_index, (item, baseline_item, row_size, baseline_row_size) in enumerate(
        zip(candidate["items"][1:], baseline["items"][1:], candidate["rows"][1:], baseline["rows"][1:]),
        start=1,
    ):
        line_count = EXPECTED_LINES[item_index - 1]
        floor = line_count * 10.0
        require(item["id"] == str(item_index), f"scene {scene_index}, item {item_index}: source order changed")
        require(item["minHeight"] == "auto", f"scene {scene_index}, item {item_index}: computed minimum is not auto")
        require(item["lineCount"] == line_count, f"scene {scene_index}, item {item_index}: text line count changed")
        require(item["payload"] is not None and close(item["payload"][2], 40) and close(item["payload"][3], floor),
            f"scene {scene_index}, item {item_index}: text payload geometry changed")
        require(close(item["frame"][0], 0) and close(item["frame"][1], top)
            and close(item["frame"][2], 40) and close(item["frame"][3], row_size),
            f"scene {scene_index}, item {item_index}: item frame does not match its row")
        require(close(item["payload"][0], 0) and close(item["payload"][1], top),
            f"scene {scene_index}, item {item_index}: payload origin changed")

        track = tracks[(item_index - 1) % 6]
        if track.startswith("fit-content("):
            fit_occurrences += 1
            require(close(row_size, floor), f"scene {scene_index}, item {item_index}: fit-content automatic minimum was clamped by its cap")
            if baseline_row_size + 0.01 < floor:
                fit_floor_rescues += 1
        elif candidate["align"] == "start":
            require(close(row_size, baseline_row_size), f"scene {scene_index}, item {item_index}: non-fit start track unexpectedly changed")

        if row_size + 0.01 < floor:
            content_floor_violations += 1
        if item["payload"][1] + floor > 220.01:
            scene_overflows = True
            largest_overflow = max(largest_overflow, item["payload"][1] + floor - 220)
        top += row_size

    visible_overflow_scenes += int(scene_overflows)

require(position_counts == Counter({position: 240 for position in range(1, 7)}), "fit-position coverage is incomplete")
require(cap_counts == Counter({cap: 360 for cap in EXPECTED_CAPS}), "cap coverage is incomplete")
require(len(tail_orders) == 30, "tail-order coverage is incomplete")
require(alignments == Counter({"start": 720, "stretch": 720}), "start/stretch scenes are not paired")
require(fit_occurrences == 1920, "fit-content cycle occurrence count changed")
require(fit_floor_rescues == 600, f"unexpected fit-floor rescue count: {fit_floor_rescues}")
require(content_floor_violations == 0, "a used row fell below its text content floor")
require(visible_overflow_scenes == 0 and close(largest_overflow, 0), "unexpected visible overflow")

by_list: dict[tuple[int, tuple[str, ...], str], dict[str, dict]] = {}
for scene in scenes:
    list_key = (scene["fitPosition"], tuple(scene["tail"]), scene["cap"])
    by_list.setdefault(list_key, {})[scene["align"]] = scene

for list_key, pair in by_list.items():
    require(set(pair) == {"start", "stretch"}, f"list {list_key}: missing alignment pair")
    start = pair["start"]
    stretch = pair["stretch"]
    require(close(sum(start["rows"]), 180), f"list {list_key}: start rows should total 180px")
    require(close(sum(stretch["rows"]), 220), f"list {list_key}: stretch rows should total 220px")
    auto_indices = [index for index in range(8) if start["authoredAutoRows"][index % 6] == "auto"]
    require(bool(auto_indices), f"list {list_key}: no auto row occurs in the eight implicit rows")
    per_auto_row = 40.0 / len(auto_indices)
    for index, (start_row, stretch_row) in enumerate(zip(start["rows"], stretch["rows"])):
        expected_delta = per_auto_row if index in {implicit + 1 for implicit in auto_indices} else 0.0
        require(close(stretch_row - start_row, expected_delta, tolerance=0.02),
            f"list {list_key}: stretch delta changed row {index}: {stretch_row - start_row} vs {expected_delta}")
        stretch_checks += 1

summary = {
    "browser": "Agent Browser / HeadlessChrome 157.0.0.0",
    "platform": metadata["platform"],
    "dpr": metadata["devicePixelRatio"],
    "viewportCssPx": metadata["viewportCssPx"],
    "fixtureSha256": sha256(FIXTURE),
    "collectorSha256": sha256(COLLECTOR),
    "baselineObservationSha256": sha256(BASELINE),
    "observationSha256": sha256(OBSERVATION),
    "sceneCount": len(scenes),
    "uniqueAuthoredTrackLists": len(by_list),
    "scenesPerFitPosition": dict(sorted(position_counts.items())),
    "tailPermutationCount": len(tail_orders),
    "fitContentCycleOccurrencesChecked": fit_occurrences,
    "fitFloorRescueOccurrencesAgainstMinHeightZero": fit_floor_rescues,
    "contentFloorViolations": content_floor_violations,
    "startRowTotalsCssPx": [180],
    "stretchRowTotalsCssPx": [220],
    "stretchRowRelationsChecked": stretch_checks,
    "visibleOverflowScenes": visible_overflow_scenes,
    "independentRelations": [
        "one fit-content track at each authored position with exact remaining-track multiplicity and 30 tail orders",
        "paired min-height:0 and min-height:auto scenes use identical track list, cap, alignment, text, and topology",
        "explicit control remains min-height:0 while all eight text items compute min-height:auto",
        "fit-content automatic minimum equals text content floor even when the cap is 0% or below that floor",
        "item and text payload frames follow the implicit-row cycle after the fixed explicit row",
        "align-content:stretch distributes the 40px free space only among auto-track occurrences",
        "visible overflow remains absent within this definite-height input matrix",
    ],
}
SUMMARY.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
print(json.dumps(summary, ensure_ascii=False, indent=2))
