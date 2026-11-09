#!/usr/bin/env python3
"""Verify the fixed Chrome observation against CSS Grid invariants."""

from __future__ import annotations

import hashlib
import itertools
import json
import re
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
FIXTURE = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-fit-content-positions-precomparison.html"
OBSERVATION = EVIDENCE / "chrome157-matrix-observation.ndjson"
SUMMARY = EVIDENCE / "matrix-verification.json"
EXPECTED_FIXTURE_SHA256 = "0829c8dc6d39a2cc20fea772479b732e17b44ee070555cef8e5c48a0e016a2af"
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
CAPS = {"20px": 20.0, "20%": 44.0, "0%": 0.0, "120%": 264.0}
TAIL_MULTIPLICITY = Counter({"min-content": 2, "max-content": 2, "auto": 1})


def close(actual: float, expected: float, *, tolerance: float = 0.01) -> bool:
    return abs(actual - expected) <= tolerance


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def tail_orders() -> set[tuple[str, ...]]:
    return set(itertools.permutations(TAIL_MULTIPLICITY.elements()))


def main() -> None:
    fixture_hash = hashlib.sha256(FIXTURE.read_bytes()).hexdigest()
    require(fixture_hash == EXPECTED_FIXTURE_SHA256, f"fixture hash mismatch: {fixture_hash}")
    records = [json.loads(line) for line in OBSERVATION.read_text().splitlines()]
    require(records and records[0].get("record") == "metadata", "missing NDJSON metadata record")
    require(all(record.get("record") == "scene" for record in records[1:]), "unexpected NDJSON record type")
    matrix = {key: value for key, value in records[0].items() if key != "record"}
    matrix["scenes"] = [{key: value for key, value in record.items() if key != "record"} for record in records[1:]]
    require("HeadlessChrome/157.0.0.0" in matrix["userAgent"], "unexpected browser build")
    require(matrix["dpr"] == 1 and matrix["platform"] == "MacIntel", "unexpected browser geometry environment")
    require(matrix["payloads"] == EXPECTED_PAYLOADS, "payload order/content changed")
    require(matrix["payloadLineCounts"] == EXPECTED_LINE_COUNTS, "Chrome text line-count observation changed")
    require(matrix["uniqueTailOrders"] == 30, "tail permutation count is not 30")
    require(matrix["fitContentPositions"] == [1, 2, 3, 4, 5, 6], "fit-content position set changed")
    require(matrix["caseCount"] == 1440 and len(matrix["scenes"]) == 1440, "matrix cardinality is not 1,440")

    for text in EXPECTED_PAYLOADS:
        require(len(text.encode("ascii")) <= 32, f"payload exceeds grammar length: {text!r}")
        require(re.fullmatch(r"[A-Z]{1,6}( [A-Z]{1,6})*", text) is not None, f"payload violates text grammar: {text!r}")

    tails = tail_orders()
    require(len(tails) == 30, "independent tail permutation generator did not produce 30 orders")
    by_key: dict[tuple[int, tuple[str, ...], str, str], dict] = {}
    position_counts = Counter()
    fit_occurrences = 0
    overflowing_scenes = set()
    maximum_payload_overflow = 0.0

    for scene in matrix["scenes"]:
        position = scene["fitPosition"]
        tail = tuple(scene["tail"])
        cap = scene["cap"]
        align = scene["align"]
        key = (position, tail, cap, align)
        require(position in range(1, 7), f"unsupported fit-content position: {position}")
        require(tail in tails, f"tail order is outside the 30-order multiset: {tail}")
        require(cap in CAPS and align in {"start", "stretch"}, f"unexpected cap/alignment: {cap}, {align}")
        require(key not in by_key, f"duplicate matrix key: {key}")
        by_key[key] = scene
        position_counts[position] += 1

        authored = list(tail)
        authored.insert(position - 1, f"fit-content({cap})")
        require(scene["authoredAutoRows"] == authored, f"authored track order mismatch: {key}")
        require(Counter(authored) == TAIL_MULTIPLICITY + Counter({f"fit-content({cap})": 1}), f"track multiset mismatch: {key}")
        require(scene["computedAutoRows"].split() == authored, f"computed CSSOM order mismatch: {key}")
        require(scene["gridFrame"] == [40, 220], f"Grid frame mismatch: {key}")
        require(len(scene["rows"]) == 9 and len(scene["items"]) == 9, f"expected explicit row and 8 implicit rows: {key}")
        require(close(scene["rows"][0], 10), f"explicit row changed: {key}")
        require(scene["items"][0]["frame"] == [0, 0, 40, 10], f"explicit control item changed: {key}")

        item_top = 10.0
        for implicit_index, item in enumerate(scene["items"][1:]):
            row_index = implicit_index + 1
            row_size = scene["rows"][row_index]
            frame = item["frame"]
            payload = item["payload"]
            require(close(frame[0], 0) and close(frame[1], item_top) and close(frame[2], 40) and close(frame[3], row_size), f"item/frame-to-row relation failed: {key}, item={implicit_index + 1}")
            require(payload is not None, f"missing text payload: {key}, item={implicit_index + 1}")
            require(close(payload[0], 0) and close(payload[1], item_top) and close(payload[2], 40), f"payload position/inline size changed: {key}, item={implicit_index + 1}")
            require(item["lineCount"] == EXPECTED_LINE_COUNTS[implicit_index], f"text line count changed: {key}, item={implicit_index + 1}")
            require(close(payload[3], item["lineCount"] * 10), f"payload line-height geometry changed: {key}, item={implicit_index + 1}")
            item_top += row_size

            if payload[1] + payload[3] > 220.01:
                overflowing_scenes.add(key)
                maximum_payload_overflow = max(maximum_payload_overflow, payload[1] + payload[3] - 220)

            if implicit_index % 6 + 1 == position:
                # The 220px definite content block-size gives percentage caps of
                # 44px, 0px and 264px. With min-height:0, the fit-content row's
                # used size is the lesser of the fixed-line-height payload
                # block height and the authored cap for this fixed-width input.
                expected_size = min(item["lineCount"] * 10.0, CAPS[cap])
                require(close(row_size, expected_size), f"independent fit-content sizing relation failed: {key}, implicit-row={implicit_index + 1}, actual={row_size}, expected={expected_size}")
                fit_occurrences += 1

    require(len(by_key) == 1440, f"unique matrix keys: {len(by_key)}")
    require(position_counts == Counter({position: 240 for position in range(1, 7)}), f"position coverage mismatch: {position_counts}")
    require(fit_occurrences == 1920, f"fit-content cycle occurrence count mismatch: {fit_occurrences}")

    stretch_checks = 0
    for scene in matrix["scenes"]:
        if scene["align"] != "start":
            continue
        key = (scene["fitPosition"], tuple(scene["tail"]), scene["cap"], "stretch")
        stretched = by_key[key]
        auto_rows = [
            implicit_index + 1
            for implicit_index in range(8)
            if scene["authoredAutoRows"][implicit_index % 6] == "auto"
        ]
        require(auto_rows, f"the fixed list produced no auto track: {key}")
        free_space = max(0.0, 220.0 - sum(scene["rows"]))
        per_auto_track = free_space / len(auto_rows)
        for row_index, (start_size, stretch_size) in enumerate(zip(scene["rows"], stretched["rows"])):
            expected_delta = per_auto_track if row_index in auto_rows else 0.0
            require(close(stretch_size - start_size, expected_delta, tolerance=0.02), f"stretch changed a non-auto track or misdistributed free space: {key}, row={row_index}, delta={stretch_size - start_size}, expected={expected_delta}")
        stretch_checks += 1

    require(len(overflowing_scenes) == 54, f"visible payload overflow scene count changed: {len(overflowing_scenes)}")
    require(close(maximum_payload_overflow, 20), f"maximum visible overflow changed: {maximum_payload_overflow}")

    per_position_fit_sizes = {}
    per_position_cycle_fit_sizes = {}
    for position in range(1, 7):
        per_position_fit_sizes[str(position)] = {}
        per_position_cycle_fit_sizes[str(position)] = {}
        cycle_row_indices = [index + 1 for index in range(8) if index % 6 + 1 == position]
        for cap in CAPS:
            per_position_fit_sizes[str(position)][cap] = sorted(
                {
                    scene["rows"][position]
                    for scene in matrix["scenes"]
                    if scene["fitPosition"] == position and scene["cap"] == cap
                }
            )
            per_position_cycle_fit_sizes[str(position)][cap] = [
                sorted(
                    {
                        scene["rows"][row_index]
                        for scene in matrix["scenes"]
                        if scene["fitPosition"] == position and scene["cap"] == cap
                    }
                )
                for row_index in cycle_row_indices
            ]

    summary = {
        "browser": "Agent Browser 0.31.1 / HeadlessChrome 157.0.0.0",
        "platform": matrix["platform"],
        "dpr": matrix["dpr"],
        "fixtureSha256": fixture_hash,
        "observationSha256": hashlib.sha256(OBSERVATION.read_bytes()).hexdigest(),
        "sceneCount": len(by_key),
        "scenesPerFitPosition": dict(sorted(position_counts.items())),
        "tailPermutationCount": len(tails),
        "fitContentCycleOccurrencesChecked": fit_occurrences,
        "payloadLineCounts": EXPECTED_LINE_COUNTS,
        "fitTrackSizesByPositionAndCap": per_position_fit_sizes,
        "fitTrackCycleSizesByPositionAndCap": per_position_cycle_fit_sizes,
        "stretchRelationsChecked": stretch_checks,
        "visibleOverflowScenes": len(overflowing_scenes),
        "maximumPayloadOverflowCssPx": maximum_payload_overflow,
        "independentRelations": [
            "exact six-member track multiset and authored/computed order",
            "implicit track cycle starts after the explicit row and repeats every six tracks",
            "fit-content used size equals min(fixed line-height times observed line count, px/percentage cap)",
            "item frames match their used rows and text payload frames match observed line count",
            "positive align-content stretch space changes only auto tracks and is shared evenly",
            "visible text payload overflow remains observable beyond the 220px Grid box",
        ],
    }
    SUMMARY.write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
