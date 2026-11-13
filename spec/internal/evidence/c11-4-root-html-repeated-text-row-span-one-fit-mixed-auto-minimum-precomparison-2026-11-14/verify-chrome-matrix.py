#!/usr/bin/env python3
"""Independently verify the one-fit mixed-minimum Chrome observation."""

from collections import defaultdict
from gzip import open as gzip_open
from hashlib import sha256
from itertools import permutations
import json
import math
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
OBSERVATION = HERE / "chrome157-one-fit-mixed-observation.ndjson.gz"
BASELINES = (
    ROOT
    / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson",
    ROOT
    / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson",
)
OUTPUT = HERE / "verification.json"
CAPTURE_MANIFEST = HERE / "capture-manifest.json"
SAMPLE_SCREENSHOT = HERE / "chrome157-one-fit-mixed-sample.png"
TOLERANCE = 0.01
TOPOLOGIES = (
    "span2-forward",
    "span2-reversed",
    "span2-before-span3",
    "span3-before-span2",
    "span2-six-forward",
    "span2-six-reversed",
    "span2-span3-six-forward",
    "span2-span3-six-reversed",
)
MASK_COUNTS = {
    "span2-forward": 4,
    "span2-reversed": 4,
    "span2-before-span3": 8,
    "span3-before-span2": 8,
    "span2-six-forward": 32,
    "span2-six-reversed": 32,
    "span2-span3-six-forward": 16,
    "span2-span3-six-reversed": 16,
}
SOURCE_PAIRS = (
    ("span2-forward", "span2-reversed"),
    ("span2-before-span3", "span3-before-span2"),
    ("span2-six-forward", "span2-six-reversed"),
    ("span2-span3-six-forward", "span2-span3-six-reversed"),
)


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def close(a, b, tolerance=TOLERANCE):
    return math.isfinite(a) and math.isfinite(b) and abs(a - b) <= tolerance


def max_delta(left, right):
    require(len(left) == len(right), "geometry vector length changed")
    return max((abs(a - b) for a, b in zip(left, right)), default=0.0)


def all_tail_orders():
    seed = ("min-content", "min-content", "max-content", "max-content", "auto")
    return sorted(set(permutations(seed)))


def expected_track_lists():
    result = set()
    tails = all_tail_orders()
    require(len(tails) == 30, f"expected 30 unique tail orders, got {len(tails)}")
    for fit_position in range(1, 7):
        caps = ("20px", "20%", "0%", "120%") if fit_position <= 3 else (
            "0px",
            "0%",
            "10px",
            "20px",
            "20%",
            "120%",
        )
        for tail in tails:
            for cap in caps:
                tracks = list(tail)
                tracks.insert(fit_position - 1, f"fit-content({cap})")
                result.add((fit_position, tuple(tracks), cap, tail))
    require(len(result) == 900, f"expected 900 exact track-list profiles, got {len(result)}")
    return result


def read_baselines():
    baseline = {}
    counts = defaultdict(int)
    hashes = []
    for path in BASELINES:
        digest = sha256(path.read_bytes()).hexdigest()
        hashes.append({"path": str(path.relative_to(ROOT)), "sha256": digest})
        with path.open(encoding="utf-8") as source:
            for line_number, line in enumerate(source, 1):
                record = json.loads(line)
                scene = record["scene"]
                mode = scene["minHeight"]
                require(mode in ("0px", "auto"), f"{path.name}:{line_number}: unknown endpoint")
                key = (scene["trackList"], scene["alignContent"], scene["topology"], mode)
                require(key not in baseline, f"duplicate baseline endpoint {key}")
                baseline[key] = record
                counts[mode] += 1
    require(len(baseline) == 14_400, f"baseline endpoint scenes: {len(baseline)}")
    require(counts == {"0px": 7_200, "auto": 7_200}, f"baseline modes: {dict(counts)}")
    return baseline, hashes


def track_row_span(item):
    start = int(item[1])
    span = int(item[2])
    return start, span


def geometry_snapshot(record):
    items = {
        item["id"]: {
            "frame": tuple(item["frame"]),
            "payloadFrame": tuple(item["payloadFrame"]) if item["payloadFrame"] is not None else None,
            "lineRects": tuple(tuple(rect) for rect in item["lineRects"]),
            "minHeight": item["minHeight"],
        }
        for item in record["items"]
    }
    return {
        "rows": tuple(record["rowValues"]),
        "items": items,
    }


def main():
    expected_lists = expected_track_lists()
    baseline, baseline_hashes = read_baselines()
    capture_manifest = json.loads(CAPTURE_MANIFEST.read_text(encoding="utf-8"))
    observation_digest = sha256(OBSERVATION.read_bytes()).hexdigest()
    screenshot_digest = sha256(SAMPLE_SCREENSHOT.read_bytes()).hexdigest()
    fixture_digest = sha256((ROOT / capture_manifest["fixturePath"]).read_bytes()).hexdigest()
    require(observation_digest == capture_manifest["observationSha256"], "observation hash differs from capture manifest")
    require(screenshot_digest == capture_manifest["sampleScreenshotSha256"], "screenshot hash differs from capture manifest")
    require(fixture_digest == capture_manifest["fixtureSha256"], "fixture hash differs from capture manifest")
    metadata = None
    scene_count = 0
    seen_scenes = set()
    lists_seen = set()
    lists_by_position = defaultdict(set)
    topology_counts = defaultdict(int)
    masks_by_topology = defaultdict(set)
    scene_counts_by_position = defaultdict(int)
    candidate_eligible_occurrences = 0
    candidate_floor_violations = 0
    grid_end_overflow_scene_indices = set()
    max_grid_end_overflow = 0.0
    max_span_row_sum_error = 0.0
    max_source_order_delta = 0.0
    max_non_auto_stretch_delta = 0.0
    max_auto_stretch_delta = 0.0
    max_uniform_endpoint_delta = 0.0
    mixed_scenes_with_geometry_change = 0
    geometry_by_scene = {}
    uniform_endpoint_counts = defaultdict(int)
    sample_scene = None

    with gzip_open(OBSERVATION, "rt", encoding="utf-8") as source:
        for line_number, line in enumerate(source, 1):
            record = json.loads(line)
            if record.get("record") == "metadata":
                require(metadata is None, "duplicate metadata record")
                metadata = record
                require(record["sceneCount"] == 120_960, "metadata scene count mismatch")
                require(record["uniqueTrackListCount"] == 900, "metadata track-list count mismatch")
                require("HeadlessChrome/157." in record["userAgent"], "browser version changed")
                continue

            scene = record["scene"]
            index = record["index"]
            require(index == scene_count, f"line {line_number}: expected scene index {scene_count}, got {index}")
            if index == capture_manifest["sampleIndex"]:
                sample_scene = scene
            fit_position = scene["fitPosition"]
            topology = scene["topology"]
            minimum_mask = scene["minimumMask"]
            mask_ids = scene["maskItemIds"]
            maximum_mask = (1 << len(mask_ids)) - 1
            require(topology in MASK_COUNTS, f"unsupported topology {topology}")
            require(0 <= minimum_mask <= maximum_mask, f"{topology}: mask outside its semantic item set")
            require(len(mask_ids) == len(set(mask_ids)) and mask_ids == sorted(mask_ids), f"{topology}: mask IDs unstable")
            require(scene["autoItems"] == [
                item_id for bit, item_id in enumerate(mask_ids) if minimum_mask & (1 << bit)
            ], f"{topology}: mask bits do not map to semantic item IDs")
            require(scene["rowProfile"] == ("three-row" if fit_position <= 3 else "six-row"), f"{topology}: wrong row profile")
            require(len(scene["trackMembers"]) == 6, f"scene {index}: track list member count changed")
            require(scene["trackMembers"].count("auto") == 1, f"{topology}: expected one auto track")
            require(sum(member.startswith("fit-content(") for member in scene["trackMembers"]) == 1, f"{topology}: expected one fit-content track")
            signature = (
                fit_position,
                tuple(scene["trackMembers"]),
                scene["cap"],
                tuple(scene["tail"]),
            )
            require(signature in expected_lists, f"unexpected authored track-list profile at scene {index}")
            require(scene["trackList"] == " ".join(scene["trackMembers"]), f"scene {index}: authored order changed")
            require(scene["trackMembers"].count("fit-content(" + scene["cap"] + ")") == 1, f"scene {index}: cap does not match fit track")
            require(record["computedAutoRows"] == scene["trackList"], f"scene {index}: computed grid-auto-rows changed")
            require(record["computedAlignContent"] == scene["alignContent"], f"scene {index}: align-content changed")
            require(record["devicePixelRatio"] == 1, f"scene {index}: DPR changed")
            require(record["gridFrame"][2:] == [40, 220], f"scene {index}: Grid border-box size changed")
            require(all(math.isfinite(value) and value >= -TOLERANCE for value in record["rowValues"]), f"scene {index}: invalid used row")
            expected_rows = 4 if fit_position <= 3 else 7
            require(len(record["rowValues"]) == expected_rows, f"scene {index}: expected {expected_rows} used rows")
            key = (scene["trackList"], scene["alignContent"], topology, minimum_mask)
            require(key not in seen_scenes, f"duplicate scene {key}")
            seen_scenes.add(key)
            lists_seen.add(signature)
            lists_by_position[fit_position].add(signature)
            topology_counts[topology] += 1
            masks_by_topology[topology].add(minimum_mask)
            scene_counts_by_position[fit_position] += 1

            items_by_id = {item["id"]: item for item in record["items"]}
            require("empty" in items_by_id and items_by_id["empty"]["minHeight"] == "0px", f"scene {index}: explicit-row control changed")
            expected_item_ids = set(mask_ids) | {"empty"}
            require(set(items_by_id) == expected_item_ids, f"scene {index}: direct item set changed")
            geometry = geometry_snapshot(record)
            for item_id, start, span in scene["items"]:
                item = items_by_id[item_id]
                expected_minimum = "auto" if item_id in scene["autoItems"] else "0px"
                require(item["minHeight"] == expected_minimum, f"scene {index}/{item_id}: computed minimum {item['minHeight']}")
                require(item["rowStart"] == str(start), f"scene {index}/{item_id}: computed row start changed")
                require(item["rowEnd"] == f"span {span}", f"scene {index}/{item_id}: computed row end changed")
                first_row = start - 1
                last_row = first_row + span
                rows = record["rowValues"]
                expected_top = sum(rows[:first_row])
                expected_height = sum(rows[first_row:last_row])
                require(close(item["frame"][1], expected_top), f"scene {index}/{item_id}: frame top does not match used tracks")
                span_error = abs(item["frame"][3] - expected_height)
                max_span_row_sum_error = max(max_span_row_sum_error, span_error)
                require(span_error <= TOLERANCE, f"scene {index}/{item_id}: frame height does not equal covered row sum")
                if item_id in scene["autoItems"]:
                    implicit_track_start = start - 2
                    spanned_tracks = scene["trackMembers"][implicit_track_start : implicit_track_start + span]
                    eligible = any(
                        member == "auto" or member.startswith("fit-content(")
                        for member in spanned_tracks
                    )
                    if eligible:
                        candidate_eligible_occurrences += 1
                        payload = item["payloadFrame"]
                        require(payload is not None, f"scene {index}/{item_id}: auto-minimum payload missing")
                        floor_shortfall = payload[3] - item["frame"][3]
                        if floor_shortfall > TOLERANCE:
                            candidate_floor_violations += 1
                            raise AssertionError(
                                f"scene {index}/{item_id}: auto-minimum content floor shortfall {floor_shortfall}"
                            )
                payload = item["payloadFrame"]
                if payload is not None:
                    content_bottom = record["gridFrame"][3]
                    overflow = max(0.0, payload[1] + payload[3] - content_bottom)
                    if overflow > TOLERANCE:
                        grid_end_overflow_scene_indices.add(index)
                        max_grid_end_overflow = max(max_grid_end_overflow, overflow)

            if minimum_mask in (0, maximum_mask):
                mode = "auto" if minimum_mask == maximum_mask else "0px"
                base_key = (scene["trackList"], scene["alignContent"], topology, mode)
                base_record = baseline.get(base_key)
                require(base_record is not None, f"missing uniform baseline {base_key}")
                base_geometry = geometry_snapshot(base_record)
                delta = 0.0
                for a, b in zip(geometry["rows"], base_geometry["rows"]):
                    delta = max(delta, abs(a - b))
                for item_id, item in geometry["items"].items():
                    base_item = base_geometry["items"].get(item_id)
                    require(base_item is not None, f"baseline item missing {item_id}")
                    delta = max(delta, max_delta(item["frame"], base_item["frame"]))
                    if item["payloadFrame"] is not None:
                        delta = max(delta, max_delta(item["payloadFrame"], base_item["payloadFrame"]))
                    require(item["lineRects"] == base_item["lineRects"], f"scene {index}/{item_id}: uniform endpoint line breaks changed")
                max_uniform_endpoint_delta = max(max_uniform_endpoint_delta, delta)
                require(delta <= TOLERANCE, f"scene {index}: uniform endpoint differs from existing oracle by {delta}")
                uniform_endpoint_counts[mode] += 1
            elif minimum_mask not in (0, maximum_mask):
                zero_key = (scene["trackList"], scene["alignContent"], topology, 0)
                zero_record = geometry_by_scene.get(zero_key)
                if zero_record is not None:
                    if any(
                        max_delta(geometry["items"][item_id]["frame"], zero_record["items"][item_id]["frame"]) > TOLERANCE
                        for item_id in mask_ids
                    ):
                        mixed_scenes_with_geometry_change += 1

            geometry_by_scene[key] = geometry
            scene_count += 1

    require(metadata is not None, "metadata record missing")
    require(scene_count == metadata["sceneCount"], f"captured {scene_count} scenes")
    require(len(lists_seen) == len(expected_lists), f"expected {len(expected_lists)} track lists, got {len(lists_seen)}")
    require(uniform_endpoint_counts == {"0px": 7_200, "auto": 7_200}, f"uniform endpoint coverage {dict(uniform_endpoint_counts)}")
    require(masks_by_topology == {name: set(range(count)) for name, count in MASK_COUNTS.items()}, "incomplete per-topology mask coverage")
    expected_topology_counts = {
        "span2-forward": 2_880,
        "span2-reversed": 2_880,
        "span2-before-span3": 5_760,
        "span3-before-span2": 5_760,
        "span2-six-forward": 34_560,
        "span2-six-reversed": 34_560,
        "span2-span3-six-forward": 17_280,
        "span2-span3-six-reversed": 17_280,
    }
    require(dict(topology_counts) == expected_topology_counts, f"topology counts changed: {dict(topology_counts)}")
    require(scene_counts_by_position == {1: 5_760, 2: 5_760, 3: 5_760, 4: 34_560, 5: 34_560, 6: 34_560}, "fit-position coverage changed")

    source_pair_count = 0
    for forward, reverse in SOURCE_PAIRS:
        keys = [
            key for key in geometry_by_scene
            if key[2] == forward
        ]
        for track_list, alignment, _topology, mask in keys:
            reverse_key = (track_list, alignment, reverse, mask)
            left = geometry_by_scene[(track_list, alignment, forward, mask)]
            right = geometry_by_scene.get(reverse_key)
            require(right is not None, f"missing reverse-order pair {reverse_key}")
            row_delta = max_delta(left["rows"], right["rows"])
            max_source_order_delta = max(max_source_order_delta, row_delta)
            require(row_delta <= TOLERANCE, f"source-order track sizes differ for {track_list}/{mask}: {row_delta}")
            for item_id in left["items"]:
                require(item_id in right["items"], f"source-order item missing {item_id}")
                delta = max_delta(left["items"][item_id]["frame"], right["items"][item_id]["frame"])
                max_source_order_delta = max(max_source_order_delta, delta)
                require(delta <= TOLERANCE, f"source-order geometry changed {item_id}: {delta}")
            source_pair_count += 1
    require(source_pair_count == 60_480, f"source-order pairs: {source_pair_count}")

    alignment_pair_count = 0
    align_keys = [
        key for key in geometry_by_scene
        if key[1] == "start"
    ]
    for track_list, _alignment, topology, mask in align_keys:
        stretch = geometry_by_scene.get((track_list, "stretch", topology, mask))
        start = geometry_by_scene[(track_list, "start", topology, mask)]
        require(stretch is not None, f"missing alignment pair {track_list}/{topology}/{mask}")
        auto_index = next(index for index, member in enumerate(track_list.split()) if member == "auto") + 1
        require(len(start["rows"]) == len(stretch["rows"]), "alignment pair row count changed")
        for row_index, (before, after) in enumerate(zip(start["rows"], stretch["rows"])):
            delta = abs(after - before)
            if row_index == auto_index:
                require(after + TOLERANCE >= before, "stretch shrank the auto track")
                max_auto_stretch_delta = max(max_auto_stretch_delta, delta)
            else:
                max_non_auto_stretch_delta = max(max_non_auto_stretch_delta, delta)
                require(delta <= TOLERANCE, f"stretch changed non-auto row {row_index}: {delta}")
        members = track_list.split()
        for item_id, item in start["items"].items():
            if item_id == "empty":
                continue
            placement = next(value for value in scene_placements(track_list, topology) if value[0] == item_id)
            implicit_track_start = placement[1] - 2
            implicit_track_end = implicit_track_start + placement[2]
            spans_auto = any(member == "auto" for member in members[implicit_track_start:implicit_track_end])
            if not spans_auto:
                stretched_item = stretch["items"][item_id]
                before = item["frame"]
                after = stretched_item["frame"]
                require(
                    max_delta((before[0], before[2], before[3]), (after[0], after[2], after[3])) <= TOLERANCE,
                    f"stretch changed non-auto item size {item_id}",
                )
        alignment_pair_count += 1
    require(alignment_pair_count == 60_480, f"alignment pairs: {alignment_pair_count}")

    sample_index = capture_manifest["sampleIndex"]
    require(isinstance(sample_index, int) and 0 <= sample_index < scene_count, "invalid representative sample index")
    require(sample_scene is not None, "representative scene is absent from observation")
    sample_maximum_mask = (1 << len(sample_scene["maskItemIds"])) - 1
    require(sample_scene["minimumMask"] not in (0, sample_maximum_mask), "representative screenshot is not a mixed minimum scene")
    report = {
        "schema": "spinon.c11-4.one-fit-row-span-mixed-minimum-independent-verification/v1",
        "browserUserAgent": metadata["userAgent"],
        "fixture": {
            "path": metadata["fixturePath"],
            "sha256": metadata["fixtureSha256"],
        },
        "observation": {
            "path": capture_manifest["observationPath"],
            "bytes": capture_manifest["observationBytes"],
            "sha256": capture_manifest["observationSha256"],
        },
        "sampleScreenshot": {
            "path": capture_manifest["sampleScreenshot"],
            "sha256": capture_manifest["sampleScreenshotSha256"],
        },
        "baselineHashes": baseline_hashes,
        "sceneCount": scene_count,
        "uniqueTrackLists": len(lists_seen),
        "scenesByTopology": dict(topology_counts),
        "minimumMaskCountsByTopology": {name: len(masks_by_topology[name]) for name in TOPOLOGIES},
        "scenesByFitPosition": {str(key): value for key, value in sorted(scene_counts_by_position.items())},
        "uniformEndpointScenesCompared": sum(uniform_endpoint_counts.values()),
        "maximumUniformEndpointDeltaCssPx": max_uniform_endpoint_delta,
        "candidateEligibleAutoMinimumItemOccurrences": candidate_eligible_occurrences,
        "candidateContentFloorViolations": candidate_floor_violations,
        "mixedScenesWithGeometryChangeFromZero": mixed_scenes_with_geometry_change,
        "sourceOrderPairs": source_pair_count,
        "maximumSourceOrderDeltaCssPx": max_source_order_delta,
        "alignmentPairs": alignment_pair_count,
        "maximumNonAutoStretchDeltaCssPx": max_non_auto_stretch_delta,
        "maximumAutoStretchDeltaCssPx": max_auto_stretch_delta,
        "maximumSpanRowSumErrorCssPx": max_span_row_sum_error,
        "gridEndOverflowScenes": len(grid_end_overflow_scene_indices),
        "maximumGridEndOverflowCssPx": max_grid_end_overflow,
        "representativeSampleIndex": sample_index,
        "representativeSampleShowsGridEndOverflow": capture_manifest["sampleShowsGridEndOverflow"],
        "toleranceCssPx": TOLERANCE,
    }
    OUTPUT.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))


def scene_placements(track_list, topology):
    """Return the fixed placement tuple used by the matching topology."""
    profiles = {
        "span2-forward": (("A", 2, 2), ("B", 3, 2)),
        "span2-reversed": (("B", 3, 2), ("A", 2, 2)),
        "span2-before-span3": (("A", 2, 2), ("B", 3, 2), ("C", 2, 3)),
        "span3-before-span2": (("C", 2, 3), ("B", 3, 2), ("A", 2, 2)),
        "span2-six-forward": (("A", 2, 2), ("B", 3, 2), ("C", 4, 2), ("D", 5, 2), ("E", 6, 2)),
        "span2-six-reversed": (("E", 6, 2), ("D", 5, 2), ("C", 4, 2), ("B", 3, 2), ("A", 2, 2)),
        "span2-span3-six-forward": (("A", 2, 2), ("B", 3, 2), ("C", 4, 3), ("D", 5, 3)),
        "span2-span3-six-reversed": (("D", 5, 3), ("C", 4, 3), ("B", 3, 2), ("A", 2, 2)),
    }
    require(topology in profiles, f"unknown topology {topology}")
    return profiles[topology]


if __name__ == "__main__":
    main()
