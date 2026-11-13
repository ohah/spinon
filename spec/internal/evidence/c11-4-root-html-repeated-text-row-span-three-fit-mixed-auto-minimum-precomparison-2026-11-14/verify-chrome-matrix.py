#!/usr/bin/env python3
"""Independently verify the three-fit mixed-minimum Chrome observation."""

from collections import defaultdict
from gzip import open as gzip_open
from itertools import combinations, permutations
import json
import math
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
OBSERVATION = HERE / "chrome157-three-fit-mixed-observation.ndjson.gz"
BASELINE = ROOT / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson"
TOLERANCE = 0.01
CAPS = ("0px", "10px", "20%")
TAILS = ("min-content", "max-content", "auto")
TOPOLOGIES = {
    "span2Six-forward": ("span2Six", ("A", "B", "C", "D", "E")),
    "span2Six-reverse": ("span2Six", ("A", "B", "C", "D", "E")),
    "span2Span3Six-forward": ("span2Span3Six", ("A", "B", "C", "D")),
    "span2Span3Six-reverse": ("span2Span3Six", ("A", "B", "C", "D")),
}
TEXT = {
    "A": ("AAAA BBBB CCCC DDDD EEEE FFFF", 6),
    "B": ("AAAAAA BBBBB CCCC DDD", 4),
    "C": ("AAAAA BBBB CCC DDDD", 4),
    "D": ("AAAA BBB CCCCC DDDD", 4),
    "E": ("AAA BBBB CCCCCC", 3),
}


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def close(a, b, tolerance=TOLERANCE):
    return math.isfinite(a) and math.isfinite(b) and abs(a - b) <= tolerance


def delta(a, b):
    require(len(a) == len(b), "geometry vector length changed")
    return max((abs(left - right) for left, right in zip(a, b)), default=0.0)


def expected_track_lists():
    results = set()
    for positions in combinations(range(6), 3):
        for caps in permutations(CAPS):
            for tail in permutations(TAILS):
                tracks = [None] * 6
                for position, cap in zip(positions, caps):
                    tracks[position] = f"fit-content({cap})"
                tail_iter = iter(tail)
                tracks = [track if track is not None else next(tail_iter) for track in tracks]
                results.add((" ".join(tracks), tuple(position + 1 for position in positions), caps, tail))
    return results


def load_baseline():
    baseline = {}
    with BASELINE.open(encoding="utf-8") as source:
        for line_number, line in enumerate(source, 1):
            record = json.loads(line)
            scene = record["scene"]
            key = (scene["trackList"], scene["alignContent"], scene["topology"])
            state = scene["minimum"]
            require(state in ("zero", "auto"), f"baseline line {line_number}: unknown endpoint")
            require((key, state) not in baseline, f"baseline duplicate endpoint {key}/{state}")
            baseline[(key, state)] = record
    require(len(baseline) == 11_520, f"baseline endpoints: {len(baseline)}")
    return baseline


def main():
    baseline = load_baseline()
    expected_lists = expected_track_lists()
    require(len(expected_lists) == 720, f"expected track-list generator produced {len(expected_lists)}")

    metadata = None
    list_signatures = set()
    topology_counts = defaultdict(int)
    masks_by_topology = defaultdict(set)
    source_pairs = defaultdict(dict)
    alignment_pairs = defaultdict(dict)
    endpoint_count = 0
    candidate_eligible_occurrences = 0
    candidate_floor_violations = 0
    maximum_span_row_sum_error = 0.0
    maximum_source_order_delta = 0.0
    maximum_non_auto_stretch_delta = 0.0
    maximum_auto_stretch_delta = 0.0
    maximum_uniform_endpoint_delta = 0.0
    grid_end_overflow_scenes = 0
    mixed_grid_end_overflow_scenes = 0
    maximum_grid_end_overflow = 0.0
    overflow_sample = None
    scene_count = 0

    with gzip_open(OBSERVATION, "rt", encoding="utf-8") as source:
        for line_number, line in enumerate(source, 1):
            entry = json.loads(line)
            if entry.get("record") == "metadata":
                require(metadata is None and line_number == 1, "metadata must be the first and only metadata record")
                metadata = entry
                continue
            require(entry.get("record") == "scene", f"line {line_number}: unknown record")
            record = {key: value for key, value in entry.items() if key != "record"}
            scene = record["scene"]
            index = scene_count
            require(record["index"] == index, f"scene order is not contiguous at {index}")
            scene_count += 1

            topology = scene["topology"]
            require(topology in TOPOLOGIES, f"scene {index}: unknown topology {topology}")
            family, semantic_ids = TOPOLOGIES[topology]
            mask_ids = scene["maskItemIds"]
            require(tuple(mask_ids) == semantic_ids, f"scene {index}: mask bit order changed")
            full_mask = (1 << len(mask_ids)) - 1
            mask = scene["minimumMask"]
            require(isinstance(mask, int) and 0 <= mask <= full_mask, f"scene {index}: invalid minimum mask")
            auto_items = [item_id for bit, item_id in enumerate(mask_ids) if mask & (1 << bit)]
            require(scene["autoItems"] == auto_items, f"scene {index}: mask/item mapping changed")
            require(scene["alignContent"] in ("start", "stretch"), f"scene {index}: alignment changed")

            members = scene["trackMembers"]
            require(len(members) == 6 and scene["trackList"] == " ".join(members), f"scene {index}: authored track list changed")
            positions = scene["fitPositions"]
            caps = scene["fitCaps"]
            tail = scene["tail"]
            require(len(positions) == len(caps) == 3, f"scene {index}: three-fit multiplicity changed")
            require(tuple(sorted(positions)) == tuple(positions) and len(set(positions)) == 3, f"scene {index}: fit positions invalid")
            require(len(tail) == 3 and sorted(tail) == sorted(TAILS), f"scene {index}: intrinsic tail permutation changed")
            for position, cap in zip(positions, caps):
                require(cap in CAPS and members[position - 1] == f"fit-content({cap})", f"scene {index}: cap/order mismatch")
            require(sum(member.startswith("fit-content(") for member in members) == 3, f"scene {index}: fit count changed")
            require(members.count("min-content") == members.count("max-content") == members.count("auto") == 1,
                    f"scene {index}: tail multiplicity changed")
            signature = (scene["trackList"], tuple(positions), tuple(caps), tuple(tail))
            list_signatures.add(signature)
            require(signature in expected_lists, f"scene {index}: unplanned track-list order")
            require(record["computedAutoRows"] == scene["trackList"], f"scene {index}: computed authored list changed")
            require(record["computedAlignContent"] == scene["alignContent"], f"scene {index}: computed alignment changed")

            rows = record["rowValues"]
            require(len(rows) == 7 and all(math.isfinite(value) and value >= 0 for value in rows),
                    f"scene {index}: invalid used row values")
            used_rows = [float(value.removesuffix("px")) for value in record["usedRows"].split()]
            require(len(used_rows) == 7 and all(close(a, b, 0.001) for a, b in zip(rows, used_rows)),
                    f"scene {index}: used-row serialization mismatch")
            require(delta(record["gridFrame"], [0, 0, 42, 222]) <= 0.001, f"scene {index}: Grid border box changed")

            expected_items = {item_id: (start, span) for item_id, start, span in scene["items"]}
            actual_items = {item["id"]: item for item in record["items"]}
            require(set(actual_items) == {"empty", *expected_items}, f"scene {index}: item set changed")
            require(actual_items["empty"]["minHeight"] == "0px"
                    and delta(actual_items["empty"]["frame"], [1, 1, 40, 10]) <= TOLERANCE,
                    f"scene {index}: explicit-row control changed")
            for item_id, (start_line, span) in expected_items.items():
                item = actual_items[item_id]
                expected_minimum = "auto" if item_id in auto_items else "0px"
                require(item["minHeight"] == expected_minimum, f"scene {index}: {item_id} minimum mask not applied")
                require(item["text"] == TEXT[item_id][0], f"scene {index}: {item_id} payload changed")
                require(len(item["lineRects"]) == TEXT[item_id][1], f"scene {index}: {item_id} line wrapping changed")
                require(item["payloadFrame"] is not None, f"scene {index}: {item_id} payload frame missing")
                first_row = start_line - 2
                expected_y = 1 + sum(rows[: start_line - 1])
                expected_height = sum(rows[first_row + 1 : first_row + span + 1])
                frame_error = max(abs(item["frame"][1] - expected_y), abs(item["frame"][3] - expected_height))
                maximum_span_row_sum_error = max(maximum_span_row_sum_error, frame_error)
                require(frame_error <= TOLERANCE, f"scene {index}: {item_id} frame differs from occupied row sum")
                require(close(item["frame"][0], 1) and close(item["frame"][2], 40), f"scene {index}: {item_id} column changed")

                covered = members[first_row : first_row + span]
                eligible = any(track == "auto" or track.startswith("fit-content(") for track in covered)
                if item_id in auto_items and eligible:
                    candidate_eligible_occurrences += 1
                    if item["frame"][3] + TOLERANCE < item["payloadFrame"][3]:
                        candidate_floor_violations += 1

            scene_overflow = max(
                (item["payloadFrame"][1] + item["payloadFrame"][3] - record["gridFrame"][3]
                 for item in actual_items.values() if item["payloadFrame"] is not None),
                default=0.0,
            )
            scene_overflow += 1.0
            if scene_overflow > TOLERANCE:
                grid_end_overflow_scenes += 1
                maximum_grid_end_overflow = max(maximum_grid_end_overflow, scene_overflow)
                if mask not in (0, full_mask):
                    mixed_grid_end_overflow_scenes += 1
                item_d = actual_items.get("D")
                if index == 54_448:
                    overflow_sample = {
                        "index": index,
                        "minimumMask": mask,
                        "topology": topology,
                        "overflowItemDByCssPx": item_d["payloadFrame"][1] + item_d["payloadFrame"][3]
                        - (record["gridFrame"][3] - 1),
                    }

            topology_counts[topology] += 1
            masks_by_topology[topology].add(mask)
            direction = "reverse" if topology.endswith("-reverse") else "forward"
            base_topology = topology.removesuffix("-forward").removesuffix("-reverse")
            source_key = (scene["trackList"], scene["alignContent"], mask, base_topology)
            source_pairs[source_key][direction] = {
                "rows": rows,
                "items": {item_id: actual_items[item_id]["frame"] for item_id in expected_items},
            }
            alignment_pairs[(scene["trackList"], topology, mask)][scene["alignContent"]] = rows

            if mask in (0, full_mask):
                endpoint_count += 1
                old_state = "zero" if mask == 0 else "auto"
                old_key = ((scene["trackList"], scene["alignContent"], topology), old_state)
                previous = baseline.get(old_key)
                require(previous is not None, f"scene {index}: existing uniform endpoint missing")
                maximum_uniform_endpoint_delta = max(
                    maximum_uniform_endpoint_delta,
                    delta(rows, previous["rowValues"]),
                )
                old_items = {item["id"]: item for item in previous["items"]}
                for item_id in expected_items:
                    normalized_frame = [
                        actual_items[item_id]["frame"][0] - 1,
                        actual_items[item_id]["frame"][1] - 1,
                        *actual_items[item_id]["frame"][2:],
                    ]
                    maximum_uniform_endpoint_delta = max(
                        maximum_uniform_endpoint_delta,
                        delta(normalized_frame, old_items[item_id]["frame"]),
                    )
                    normalized_payload = [
                        actual_items[item_id]["payloadFrame"][0] - 1,
                        actual_items[item_id]["payloadFrame"][1] - 1,
                        *actual_items[item_id]["payloadFrame"][2:],
                    ]
                    maximum_uniform_endpoint_delta = max(
                        maximum_uniform_endpoint_delta,
                        delta(normalized_payload, old_items[item_id]["payloadFrame"]),
                    )

    require(metadata is not None, "capture metadata missing")
    require(scene_count == 138_240, f"scene count: {scene_count}")
    require(len(list_signatures) == len(expected_lists) == 720, f"track-list coverage: {len(list_signatures)}")
    expected_counts = {
        "span2Six-forward": 46_080,
        "span2Six-reverse": 46_080,
        "span2Span3Six-forward": 23_040,
        "span2Span3Six-reverse": 23_040,
    }
    require(dict(topology_counts) == expected_counts, f"topology counts: {dict(topology_counts)}")
    require(all(len(masks_by_topology[name]) == count for name, count in (("span2Six-forward", 32),
                ("span2Six-reverse", 32), ("span2Span3Six-forward", 16), ("span2Span3Six-reverse", 16))),
            "not all topology masks were observed")
    require(len(source_pairs) == 69_120, f"source-order pair count: {len(source_pairs)}")
    for key, pair in source_pairs.items():
        require(set(pair) == {"forward", "reverse"}, f"source-order partner missing: {key}")
        forward, reverse = pair["forward"], pair["reverse"]
        for left, right in zip(forward["rows"], reverse["rows"]):
            maximum_source_order_delta = max(maximum_source_order_delta, abs(left - right))
        require(forward["items"].keys() == reverse["items"].keys(), f"source-order item IDs changed: {key}")
        for item_id in forward["items"]:
            maximum_source_order_delta = max(maximum_source_order_delta,
                                             delta(forward["items"][item_id], reverse["items"][item_id]))
    require(maximum_source_order_delta <= TOLERANCE, "source-order reversal changed semantic geometry")

    require(len(alignment_pairs) == 69_120, f"alignment pair count: {len(alignment_pairs)}")
    for key, pair in alignment_pairs.items():
        require(set(pair) == {"start", "stretch"}, f"alignment partner missing: {key}")
        start, stretch = pair["start"], pair["stretch"]
        scene_track_list = key[0].split()
        auto_row_index = scene_track_list.index("auto") + 1
        start_total, stretch_total = sum(start), sum(stretch)
        require(close(stretch_total, max(start_total, 220), 0.02), f"stretch total mismatch: {key}")
        for row, (before, after) in enumerate(zip(start, stretch)):
            change = after - before
            if row == auto_row_index:
                maximum_auto_stretch_delta = max(maximum_auto_stretch_delta, change)
                require(change >= -TOLERANCE, f"stretch shrank auto row: {key}")
            else:
                maximum_non_auto_stretch_delta = max(maximum_non_auto_stretch_delta, abs(change))
                require(abs(change) <= TOLERANCE, f"stretch changed non-auto row: {key}/{row}")
    require(candidate_floor_violations == 0, "eligible automatic-minimum item fell below content floor")
    require(grid_end_overflow_scenes == 1_344, f"Grid-end overflow scenes: {grid_end_overflow_scenes}")
    require(mixed_grid_end_overflow_scenes == 1_236,
            f"mixed-mask Grid-end overflow scenes: {mixed_grid_end_overflow_scenes}")
    require(close(maximum_grid_end_overflow, 20.0), f"maximum Grid-end overflow: {maximum_grid_end_overflow}")
    require(overflow_sample is not None and overflow_sample["minimumMask"] == 16
            and overflow_sample["topology"] == "span2Six-forward"
            and close(overflow_sample["overflowItemDByCssPx"], 10.0),
            f"representative outside-border scene changed: {overflow_sample}")
    require(endpoint_count == 11_520, f"uniform endpoint count: {endpoint_count}")
    require(maximum_uniform_endpoint_delta <= TOLERANCE, "uniform endpoint differs from prior three-fit evidence")

    result = {
        "schema": "spinon.c11-4.repeated-text-row-span-three-fit-mixed-auto-min-independent-verification/v1",
        "browserUserAgent": metadata["userAgent"],
        "fixture": {"path": metadata["fixturePath"], "sha256": metadata["fixtureSha256"]},
        "observation": {"path": OBSERVATION.name, "bytes": OBSERVATION.stat().st_size},
        "sceneCount": scene_count,
        "uniqueTrackLists": len(list_signatures),
        "scenesByTopology": dict(topology_counts),
        "maskCountsByTopology": {name: len(masks) for name, masks in sorted(masks_by_topology.items())},
        "sourceOrderPairs": len(source_pairs),
        "alignmentPairs": len(alignment_pairs),
        "candidateEligibleItemOccurrences": candidate_eligible_occurrences,
        "candidateContentFloorViolations": candidate_floor_violations,
        "gridEndOverflowScenes": grid_end_overflow_scenes,
        "mixedMaskGridEndOverflowScenes": mixed_grid_end_overflow_scenes,
        "maximumGridEndOverflowCssPx": maximum_grid_end_overflow,
        "representativeOverflowScene": overflow_sample,
        "maximumSpanRowSumErrorCssPx": maximum_span_row_sum_error,
        "maximumSourceOrderDeltaCssPx": maximum_source_order_delta,
        "maximumNonAutoStretchDeltaCssPx": maximum_non_auto_stretch_delta,
        "maximumAutoStretchDeltaCssPx": maximum_auto_stretch_delta,
        "uniformEndpointScenesCompared": endpoint_count,
        "maximumUniformEndpointDeltaCssPx": maximum_uniform_endpoint_delta,
    }
    (HERE / "verification.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
