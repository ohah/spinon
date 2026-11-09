#!/usr/bin/env python3
"""Check fixed Chrome observations against the planned CSS Grid relations."""

from __future__ import annotations

import hashlib
import itertools
import json
import re
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
FIXTURE = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-two-fit-content-precomparison.html"
COLLECTOR = EVIDENCE / "capture-chrome.mjs"
OBSERVATION = EVIDENCE / "chrome-matrix-observation.ndjson"
SUMMARY = EVIDENCE / "matrix-verification.json"
EXPECTED_PAYLOADS = [
    "A A A A A A",
    "AAAA AAAA A",
    "AA AA AA AA",
    "AAA AAA AAA",
    "AAA AAA",
    "A A A A A A",
    "A A A A A A A A",
    "AA",
]
EXPECTED_LINE_COUNTS = [2, 2, 2, 3, 2, 2, 3, 1]
CAPS = {"0px": 0.0, "0%": 0.0, "10px": 10.0, "20px": 20.0, "20%": 44.0, "120%": 264.0}
CAP_ASSIGNMENTS = {
    ("0px", "0%"), ("0%", "0px"),
    ("10px", "20px"), ("20px", "10px"),
    ("10px", "20%"), ("20%", "10px"),
    ("20%", "120%"), ("120%", "20%"),
}


def close(actual: float, expected: float, tolerance: float = 0.01) -> bool:
    return abs(actual - expected) <= tolerance


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def all_tail_orders() -> set[tuple[str, ...]]:
    return set(itertools.permutations(("min-content", "min-content", "max-content", "auto")))


def main() -> None:
    fixture_hash = hashlib.sha256(FIXTURE.read_bytes()).hexdigest()
    collector_hash = hashlib.sha256(COLLECTOR.read_bytes()).hexdigest()
    records = [json.loads(line) for line in OBSERVATION.read_text().splitlines()]
    require(records and records[0].get("record") == "metadata", "metadata record is missing")
    metadata = records[0]
    scenes = records[1:]
    require(all(record.get("record") == "scene" for record in scenes), "unexpected record type")
    require(metadata["schema"] == "spinon.c11-4.repeated-text-autoplacement-two-fit-content-precomparison/v1",
        "observation schema changed")
    require(metadata["fixtureSha256"] == fixture_hash, "fixture hash mismatch")
    require(metadata["collectorSha256"] == collector_hash, "collector hash mismatch")
    require("HeadlessChrome/157.0.0.0" in metadata["userAgent"], "unexpected browser version")
    require(metadata["platform"] == "MacIntel" and metadata["devicePixelRatio"] == 1,
        "unexpected host or device scale")
    require(metadata["viewportCssPx"] == [1280, 900], "unexpected browser viewport")
    require(metadata["sceneCount"] == 2_880 and len(scenes) == 2_880, "scene count is not 2,880")
    require(metadata["uniqueTrackListCount"] == 1_440, "unique list count is not 1,440")

    expected_positions = [(first, second) for first in range(1, 7) for second in range(first + 1, 7)]
    tails = all_tail_orders()
    require(len(tails) == 12, "independent tail permutation count is not 12")
    require(metadata["fitPositionPairs"] == 15 and metadata["orderedCapAssignments"] == 8
        and metadata["tailOrders"] == 12 and metadata["alignments"] == ["start", "stretch"],
        "matrix dimensions changed")
    require(len(scenes) == 15 * 8 * 12 * 2, "matrix cardinality formula changed")

    by_key: dict[tuple, dict] = {}
    alignment_pairs: dict[str, dict[str, dict]] = {}
    cap_groups: dict[tuple, list[dict]] = {}
    fit_occurrences = 0
    overflowing_scenes: set[int] = set()
    maximum_overflow = 0.0
    used_row_values: list[float] = []

    for expected_index, record in enumerate(scenes):
        require(record["index"] == expected_index, f"scene index discontinuity at {expected_index}")
        scene = record["scene"]
        positions = tuple(scene["fitPositions"])
        caps = tuple(scene["fitCaps"])
        tail = tuple(scene["tail"])
        alignment = scene["alignContent"]
        key = (positions, tail, caps, alignment)
        require(key not in by_key, f"duplicate scene key: {key}")
        by_key[key] = record
        require(positions in expected_positions, f"unexpected fit position pair: {positions}")
        require(tail in tails, f"tail is outside the 12-order multiset: {tail}")
        require(caps in CAP_ASSIGNMENTS, f"unexpected ordered cap assignment: {caps}")
        require(alignment in {"start", "stretch"}, f"unexpected alignment: {alignment}")
        require(Counter(scene["trackMembers"]) == Counter({
            f"fit-content({caps[0]})": 1,
            f"fit-content({caps[1]})": 1,
            "min-content": 2,
            "max-content": 1,
            "auto": 1,
        }), f"track-list multiplicity mismatch: {key}")
        require(scene["trackMembers"][positions[0] - 1] == f"fit-content({caps[0]})"
            and scene["trackMembers"][positions[1] - 1] == f"fit-content({caps[1]})",
            f"cap assignment does not match positions: {key}")
        remaining = tuple(member for index, member in enumerate(scene["trackMembers"], start=1)
            if index not in positions)
        require(remaining == tail, f"authored tail order mismatch: {key}")
        require(record["computedAutoRows"].split() == scene["trackMembers"],
            f"computed CSSOM order/cap mismatch: {key}")
        require(record["computedAlignContent"] == alignment, f"computed alignment mismatch: {key}")
        require(record["gridFrame"] == [40, 220], f"Grid frame changed: {key}")
        rows = record["rows"]
        items = record["items"]
        require(len(rows) == 9 and len(items) == 9, f"expected one explicit + eight implicit rows: {key}")
        require(all(isinstance(value, (int, float)) and value >= 0 for value in rows),
            f"non-finite or negative row size: {key}")
        require(close(rows[0], 10), f"explicit row changed: {key}")
        used_row_values.extend(rows)
        explicit_item = items[0]
        require(explicit_item["id"] == "empty-0" and explicit_item["frame"] == [0, 0, 40, 10],
            f"explicit-row control changed: {key}")

        top = 10.0
        for implicit_index, expected_lines in enumerate(EXPECTED_LINE_COUNTS):
            item = items[implicit_index + 1]
            row_index = implicit_index + 1
            row_size = rows[row_index]
            require(item["id"] == str(implicit_index + 1), f"auto-placement order changed: {key}")
            frame = item["frame"]
            payload = item["payload"]
            require(len(frame) == 4 and close(frame[0], 0) and close(frame[1], top)
                and close(frame[2], 40) and close(frame[3], row_size),
                f"item frame does not match implicit row: {key}, item={implicit_index + 1}")
            require(payload is not None and len(payload) == 4 and close(payload[0], 0)
                and close(payload[1], top) and close(payload[2], 40)
                and close(payload[3], expected_lines * 10),
                f"payload box changed: {key}, item={implicit_index + 1}")
            require(item["lineCount"] == expected_lines, f"text line count changed: {key}")
            expected_line_tops = [round(top - 1 + 10 * line, 5) for line in range(expected_lines)]
            require(len(item["lineTops"]) == expected_lines
                and all(close(actual, expected, 0.01)
                    for actual, expected in zip(item["lineTops"], expected_line_tops)),
                f"text line positions changed: {key}")
            if top + expected_lines * 10 > 220.01:
                overflowing_scenes.add(expected_index)
                maximum_overflow = max(maximum_overflow, top + expected_lines * 10 - 220)

            track_position = implicit_index % 6 + 1
            if track_position in positions:
                cap_index = positions.index(track_position)
                expected_size = min(expected_lines * 10.0, CAPS[caps[cap_index]])
                require(close(row_size, expected_size),
                    f"independent fit-content relation failed: {key}, implicit row={implicit_index + 1}, actual={row_size}, expected={expected_size}")
                fit_occurrences += 1
            top += row_size

        align_key = scene["trackList"]
        alignment_pairs.setdefault(align_key, {})[alignment] = record
        cap_key = (positions, tail, alignment)
        cap_groups.setdefault(cap_key, []).append(record)

    require(len(by_key) == 2_880, "scene key uniqueness failed")
    require(len(alignment_pairs) == 1_440, "expected 1,440 unique track lists")
    require(len(cap_groups) == 360, "expected 360 cap-sensitivity groups")
    require(fit_occurrences == 7_680, f"expected 7,680 fit-content row occurrences, got {fit_occurrences}")

    maximum_non_auto_stretch_delta = 0.0
    maximum_auto_stretch_delta = 0.0
    stretch_pairs = 0
    for track_list, pair in alignment_pairs.items():
        require(set(pair) == {"start", "stretch"}, f"alignment pair missing: {track_list}")
        start = pair["start"]
        stretch = pair["stretch"]
        auto_row_indices = [implicit_index + 1 for implicit_index in range(8)
            if start["scene"]["trackMembers"][implicit_index % 6] == "auto"]
        require(auto_row_indices, f"no auto row occurrence: {track_list}")
        free_space = max(0.0, 220.0 - sum(start["rows"]))
        expected_delta = free_space / len(auto_row_indices)
        for row_index, (start_size, stretch_size) in enumerate(zip(start["rows"], stretch["rows"])):
            delta = stretch_size - start_size
            if row_index in auto_row_indices:
                require(close(delta, expected_delta, 0.02),
                    f"stretch free-space distribution changed: {track_list}, row={row_index}")
                maximum_auto_stretch_delta = max(maximum_auto_stretch_delta, abs(delta))
            else:
                require(close(delta, 0, 0.01),
                    f"stretch changed a non-auto row: {track_list}, row={row_index}")
                maximum_non_auto_stretch_delta = max(maximum_non_auto_stretch_delta, abs(delta))
        require(close(sum(stretch["rows"]), max(sum(start["rows"]), 220), 0.02),
            f"stretch total does not fill the definite block: {track_list}")
        stretch_pairs += 1

    cap_sensitive_groups = 0
    maximum_cap_variant_row_range = 0.0
    for key, group in cap_groups.items():
        require(len(group) == 8 and {tuple(record["scene"]["fitCaps"]) for record in group} == CAP_ASSIGNMENTS,
            f"incomplete cap assignment group: {key}")
        group_range = max(
            max(record["rows"][row] for record in group)
            - min(record["rows"][row] for record in group)
            for row in range(9)
        )
        maximum_cap_variant_row_range = max(maximum_cap_variant_row_range, group_range)
        if group_range > 0.01:
            cap_sensitive_groups += 1

    require(stretch_pairs == 1_440, f"stretch pair count is {stretch_pairs}")
    require(maximum_non_auto_stretch_delta <= 0.01, "non-auto stretch guard failed")
    fit_track_values = [
        record["rows"][position]
        for record in scenes
        for position in record["scene"]["fitPositions"]
    ]
    summary = {
        "schema": "spinon.c11-4.repeated-text-autoplacement-two-fit-content-precomparison-summary/v1",
        "browser": metadata["userAgent"],
        "platform": metadata["platform"],
        "devicePixelRatio": metadata["devicePixelRatio"],
        "viewportCssPx": metadata["viewportCssPx"],
        "fixtureSha256": fixture_hash,
        "collectorSha256": collector_hash,
        "observationSha256": hashlib.sha256(OBSERVATION.read_bytes()).hexdigest(),
        "sceneCount": len(scenes),
        "uniqueTrackListCount": len(alignment_pairs),
        "fitPositionPairs": len(expected_positions),
        "orderedCapAssignments": len(CAP_ASSIGNMENTS),
        "tailOrderCount": len(tails),
        "fitContentRowOccurrencesChecked": fit_occurrences,
        "alignmentPairsChecked": stretch_pairs,
        "capSensitivityGroups": cap_sensitive_groups,
        "totalCapSensitivityGroups": len(cap_groups),
        "maximumCapVariantRowRangeCssPx": maximum_cap_variant_row_range,
        "fitTrackUsedSizeRangeCssPx": [min(fit_track_values), max(fit_track_values)],
        "maximumNonAutoStretchDeltaCssPx": maximum_non_auto_stretch_delta,
        "maximumAutoStretchDeltaCssPx": maximum_auto_stretch_delta,
        "visiblePayloadOverflowSceneCount": len(overflowing_scenes),
        "maximumVisiblePayloadOverflowCssPx": maximum_overflow,
        "usedRowRangeCssPx": [min(used_row_values), max(used_row_values)],
        "independentRelations": [
            "six authored track members preserve the two cap positions and remaining 2/1/1 tail multiset",
            "eight auto-placed items start after one explicit row and cycle the six implicit track sizes",
            "each fit-content row equals min(fixed 10px line-height contribution, resolved cap) for this min-height:0 fixture",
            "item frames match used rows and text payload frames match the fixed line-count corpus",
            "align-content:stretch distributes free space equally only among auto track occurrences",
            "visible text may overflow beyond the definite 220px Grid box",
        ],
    }
    SUMMARY.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
