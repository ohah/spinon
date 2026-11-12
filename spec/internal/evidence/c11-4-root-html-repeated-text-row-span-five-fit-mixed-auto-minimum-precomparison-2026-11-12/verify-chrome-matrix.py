#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
FIXTURE = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison.html"
OBSERVATION = HERE / "chrome157-five-fit-mixed-observation.ndjson.gz"
OUTPUT = HERE / "verification.json"
UNIFORM = ROOT / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/chrome157-observation.ndjson.gz"
UNIFORM_VERIFICATION = UNIFORM.with_name("verification.json")
CAPS = ("0px", "10px", "20%", "30px", "40px")
ALIGNMENTS = ("start", "stretch")
TOPOLOGIES = {
    "span2Six": ("A", "B", "C", "D", "E"),
    "span2Span3Six": ("A", "B", "C", "D"),
}
PLACEMENTS = {
    "span2Six": (("A", 2, 2), ("B", 3, 2), ("C", 4, 2), ("D", 5, 2), ("E", 6, 2)),
    "span2Span3Six": (("A", 2, 2), ("B", 3, 2), ("C", 4, 3), ("D", 5, 3)),
}
TEXTS = {
    "A": "AAAA BBBB CCCC DDDD EEEE FFFF",
    "B": "AAAAAA BBBBB CCCC DDD",
    "C": "AAAAA BBBB CCC DDDD",
    "D": "AAAA BBB CCCCC DDDD",
    "E": "AAA BBBB CCCCCC",
}
LINE_COUNTS = {"A": 6, "B": 4, "C": 4, "D": 4, "E": 3}
CAP_ORDERS = set(itertools.permutations(CAPS))
FIT_POSITION_SETS = set(itertools.combinations(range(1, 7), 5))
TOPOLOGY_NAMES = tuple(f"{family}-{order}" for family in TOPOLOGIES for order in ("forward", "reverse"))
TOLERANCE = 0.01
EXPECTED_LISTS = 6 * 120 * 3
EXPECTED_SCENES = EXPECTED_LISTS * (32 * 2 + 16 * 2) * 2


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def near(left, right, tolerance=TOLERANCE):
    return math.isfinite(left) and math.isfinite(right) and abs(left - right) <= tolerance


def family_order(scene):
    family, order = scene["topology"].rsplit("-", 1)
    require(family in TOPOLOGIES and order in ("forward", "reverse"), f"unknown topology: {scene['topology']}")
    return family, order


def geometry(record):
    items = {}
    for item in record["items"]:
        items[item["id"]] = (
            tuple(item["frame"]),
            tuple(item["payloadFrame"]) if item["payloadFrame"] is not None else None,
            tuple(tuple(rect) for rect in item["lineRects"]),
        )
    return tuple(record["rowValues"]), items


def geometry_error(left, right):
    require(len(left[0]) == len(right[0]), "paired used-row counts differ")
    error = max((abs(a - b) for a, b in zip(left[0], right[0])), default=0.0)
    require(left[1].keys() == right[1].keys(), "paired semantic item IDs differ")
    for identifier in left[1]:
        for left_vector, right_vector in zip(left[1][identifier], right[1][identifier]):
            if left_vector is None or right_vector is None:
                require(left_vector == right_vector, f"{identifier}: optional geometry shape differs")
                continue
            require(len(left_vector) == len(right_vector), f"{identifier}: geometry vector length differs")
            for left_value, right_value in zip(left_vector, right_vector):
                if isinstance(left_value, tuple):
                    require(len(left_value) == len(right_value), f"{identifier}: line-box count differs")
                    error = max(error, max((abs(a - b) for a, b in zip(left_value, right_value)), default=0.0))
                else:
                    error = max(error, abs(left_value - right_value))
    return error


def load_uniform_controls():
    verification = json.loads(UNIFORM_VERIFICATION.read_text())
    compressed = UNIFORM.read_bytes()
    require(verification["observations"]["sceneCount"] == 34_560, "five-fit uniform control count changed")
    require(verification["observations"]["compressedSha256"] == hashlib.sha256(compressed).hexdigest(),
            "five-fit uniform control hash differs from its verification record")
    controls = {}
    with gzip.open(UNIFORM, "rt") as stream:
        for index, line in enumerate(stream):
            row = json.loads(line)
            require(row["index"] == index, f"five-fit uniform control index differs at {index}")
            require("HeadlessChrome/157." in row["userAgent"] and row["devicePixelRatio"] == 1,
                    f"five-fit uniform control browser/DPR differs at {index}")
            scene = row["scene"]
            controls[(scene["trackList"], scene["alignContent"], scene["topology"], scene["minimum"])] = geometry(row)
    require(len(controls) == 34_560, f"incomplete uniform controls: {len(controls)}")
    return controls


def validate_record(row, metadata):
    scene = row["scene"]
    family, order = family_order(scene)
    ids = TOPOLOGIES[family]
    mask = scene["minimumMask"]
    require(isinstance(mask, int) and 0 <= mask < (1 << len(ids)), f"mask outside {family}: {mask}")
    require(scene["maskItemIds"] == list(ids), f"mask bit order differs at {row['index']}")
    expected_auto = [identifier for bit, identifier in enumerate(ids) if mask & (1 << bit)]
    require(scene["autoItems"] == expected_auto, f"mask-to-item mapping differs at {row['index']}")
    require(scene["fitPositions"] in [list(value) for value in FIT_POSITION_SETS], f"bad fit positions at {row['index']}")
    require(tuple(scene["fitCaps"]) in CAP_ORDERS, f"bad cap permutation at {row['index']}")
    require(len(set(scene["fitCaps"])) == 5 and set(scene["fitCaps"]) == set(CAPS), f"cap multiset differs at {row['index']}")
    require(len(scene["trackMembers"]) == 6, f"track list length differs at {row['index']}")
    expected_tracks = [None] * 6
    for position, cap in zip(scene["fitPositions"], scene["fitCaps"]):
        expected_tracks[position - 1] = f"fit-content({cap})"
    tail_positions = [index for index, value in enumerate(expected_tracks) if value is None]
    require(len(tail_positions) == 1 and scene["tail"] in [["min-content"], ["max-content"], ["auto"]],
            f"singleton tail differs at {row['index']}")
    expected_tracks[tail_positions[0]] = scene["tail"][0]
    require(scene["trackMembers"] == expected_tracks, f"cap position or singleton tail moved at {row['index']}")
    require(scene["trackList"] == " ".join(expected_tracks), f"track-list source differs at {row['index']}")
    require(row["computedAutoRows"] == scene["trackList"], f"computed auto rows differ at {row['index']}")
    require(row["computedAlignContent"] == scene["alignContent"] and scene["alignContent"] in ALIGNMENTS,
            f"alignment differs at {row['index']}")
    require(row["devicePixelRatio"] == 1 and "HeadlessChrome/157." in row["userAgent"],
            f"browser/DPR drift at {row['index']}")
    require(row["gridFrame"] == [0, 0, 40, 220], f"definite Grid frame differs at {row['index']}")

    rows = row["rowValues"]
    serialized_rows = [float(value.removesuffix("px")) for value in row["usedRows"].strip().split()]
    require(len(rows) == len(serialized_rows) == 7 and rows[0] == 10,
            f"expected explicit 10px row plus six implicit rows at {row['index']}")
    require(all(math.isfinite(value) and value >= 0 for value in rows), f"non-finite/negative row at {row['index']}")
    require(all(near(actual, serialized, 0.001) for actual, serialized in zip(rows, serialized_rows)),
            f"used row values differ from computed serialization at {row['index']}")

    expected_placements = PLACEMENTS[family]
    if order == "reverse":
        expected_placements = tuple(reversed(expected_placements))
    require(scene["items"] == [list(value) for value in expected_placements], f"placement changed at {row['index']}")
    placement = {identifier: (start, span) for identifier, start, span in scene["items"]}
    item_map = {item["id"]: item for item in row["items"]}
    require(len(item_map) == len(row["items"]) and set(item_map) == {"empty", *ids},
            f"item owner set differs at {row['index']}")
    require(item_map["empty"]["minHeight"] == "0px" and item_map["empty"]["frame"] == [0, 0, 40, 10],
            f"explicit control changed at {row['index']}")

    floor_shortfalls = {"auto": 0, "zero": 0}
    grid_end_overflow = {"auto": 0, "zero": 0}
    eligible_items = 0
    max_span_error = 0.0
    for identifier in ids:
        item = item_map[identifier]
        is_auto = identifier in expected_auto
        expected_minimum = "auto" if is_auto else "0px"
        require(item["minHeight"] == expected_minimum, f"{identifier} computed minimum differs at {row['index']}")
        require(item["text"] == TEXTS[identifier] and len(item["lineRects"]) == LINE_COUNTS[identifier],
                f"{identifier} text/line geometry differs at {row['index']}")
        require(item["payloadFrame"] is not None and len(item["payloadFrame"]) == 4,
                f"{identifier} payload frame missing at {row['index']}")
        start, span = placement[identifier]
        expected_y = sum(rows[:start - 1])
        expected_height = sum(rows[start - 1:start - 1 + span])
        span_error = max(abs(item["frame"][1] - expected_y), abs(item["frame"][3] - expected_height))
        max_span_error = max(max_span_error, span_error)
        require(span_error <= TOLERANCE and near(item["frame"][0], 0) and near(item["frame"][2], 40),
                f"{identifier} frame differs from covered rows at {row['index']}")
        require(near(item["payloadFrame"][1], item["frame"][1]), f"{identifier} payload top differs at {row['index']}")
        covered_tracks = scene["trackMembers"][start - 2:start - 2 + span]
        eligible = any(track == "auto" or track.startswith("fit-content(") for track in covered_tracks)
        mode = "auto" if is_auto else "zero"
        if eligible:
            eligible_items += 1
            shortfall = max(0.0, item["payloadFrame"][3] - item["frame"][3])
            if is_auto and shortfall > TOLERANCE:
                floor_shortfalls["auto"] += 1
            elif not is_auto and shortfall > TOLERANCE:
                floor_shortfalls["zero"] += 1
        overflow = max(0.0, item["payloadFrame"][1] + item["payloadFrame"][3] - 220.0)
        if overflow > TOLERANCE:
            grid_end_overflow[mode] += 1
    return floor_shortfalls, grid_end_overflow, eligible_items, max_span_error


def main():
    fixture_bytes = FIXTURE.read_bytes()
    fixture_sha = hashlib.sha256(fixture_bytes).hexdigest()
    controls = load_uniform_controls()
    expected_masks = {family: 1 << len(ids) for family, ids in TOPOLOGIES.items()}
    unique_lists = set()
    positions_seen = set()
    cap_orders_seen = set()
    tails_seen = set()
    mask_counts = collections.Counter()
    mask_floor = collections.Counter()
    mask_overflow = collections.Counter()
    baseline_geometry_changes = collections.Counter()
    max_source_order_delta = 0.0
    max_alignment_delta = 0.0
    max_endpoint_error = 0.0
    max_span_error = 0.0
    uniform_controls_checked = 0
    scene_count = 0
    eligible_items = 0
    lists_checked = 0

    with gzip.open(OBSERVATION, "rt") as stream:
        metadata = json.loads(next(stream))
        require(metadata["record"] == "metadata" and metadata["sceneCount"] == EXPECTED_SCENES,
                "metadata record or scene count differs")
        require(metadata["fixtureSha256"] == fixture_sha, "fixture SHA-256 differs")
        require(metadata["browserCli"] == "agent-browser 0.31.1", f"browser CLI changed: {metadata['browserCli']}")
        require("HeadlessChrome/157." in metadata["userAgent"] and metadata["platform"] == "MacIntel",
                "pinned Chrome/browser platform differs")
        require(metadata["devicePixelRatio"] == 1 and metadata["viewportCssPx"] == [1280, 900],
                "DPR or viewport changed")
        require(metadata["uniqueTrackListCount"] == EXPECTED_LISTS, "track list count differs")

        group = []
        for line in stream:
            if not line.strip():
                continue
            row = json.loads(line)
            require(row.get("record") == "scene" and row["index"] == scene_count,
                    f"missing/duplicate scene index at {scene_count}")
            scene_count += 1
            group.append(row)
            if len(group) < 192:
                continue

            first_scene = group[0]["scene"]
            track_list = first_scene["trackList"]
            require(all(record["scene"]["trackList"] == track_list for record in group),
                    f"track list changed inside batch {lists_checked}")
            require([record["scene"]["alignContent"] for record in group[:96]] == ["start"] * 96
                    and [record["scene"]["alignContent"] for record in group[96:]] == ["stretch"] * 96,
                    f"alignment block ordering differs for {track_list}")
            unique_lists.add(track_list)
            positions_seen.add(tuple(first_scene["fitPositions"]))
            cap_orders_seen.add(tuple(first_scene["fitCaps"]))
            tails_seen.add(tuple(first_scene["tail"]))

            by_source = {}
            by_alignment = {}
            geometries = {}
            for record in group:
                scene = record["scene"]
                family, order = family_order(scene)
                mask = scene["minimumMask"]
                floor, overflow, eligible, span_error = validate_record(record, metadata)
                mask_counts[(scene["topology"], mask)] += 1
                mask_floor[(scene["topology"], mask, "auto")] += floor["auto"]
                mask_floor[(scene["topology"], mask, "zero")] += floor["zero"]
                mask_overflow[(scene["topology"], mask, "auto")] += overflow["auto"]
                mask_overflow[(scene["topology"], mask, "zero")] += overflow["zero"]
                eligible_items += eligible
                max_span_error = max(max_span_error, span_error)
                g = geometry(record)
                source_key = (scene["alignContent"], family, mask)
                by_source.setdefault(source_key, {})[order] = g
                alignment_key = (scene["topology"], mask)
                by_alignment.setdefault(alignment_key, {})[scene["alignContent"]] = (record, g)
                geometries[(scene["alignContent"], scene["topology"], mask)] = g

                uniform = "zero" if mask == 0 else "auto" if mask == (1 << len(TOPOLOGIES[family])) - 1 else None
                if uniform:
                    key = (track_list, scene["alignContent"], scene["topology"], uniform)
                    require(key in controls, f"previous uniform control missing: {key}")
                    error = geometry_error(g, controls[key])
                    max_endpoint_error = max(max_endpoint_error, error)
                    uniform_controls_checked += 1
                    require(error <= TOLERANCE, f"uniform endpoint differs from prior matrix: {key}; {error}")

            require(len(by_source) == 96 and all(set(pair) == {"forward", "reverse"} for pair in by_source.values()),
                    f"source-order pairs incomplete for {track_list}")
            for key, pair in by_source.items():
                max_source_order_delta = max(max_source_order_delta, geometry_error(pair["forward"], pair["reverse"]))

            require(len(by_alignment) == 96 and all(set(pair) == set(ALIGNMENTS) for pair in by_alignment.values()),
                    f"alignment pairs incomplete for {track_list}")
            for (topology, mask), pair in by_alignment.items():
                start_record, start_geometry = pair["start"]
                stretch_record, stretch_geometry = pair["stretch"]
                start_rows = start_record["rowValues"]
                stretch_rows = stretch_record["rowValues"]
                track_members = start_record["scene"]["trackMembers"]
                auto_index = track_members.index("auto") if "auto" in track_members else -1
                auto_row_index = auto_index + 1
                start_total = sum(start_rows)
                stretch_total = sum(stretch_rows)
                if auto_index >= 0:
                    require(near(stretch_total, max(start_total, 220), 0.02),
                            f"auto-tail stretch total differs for {track_list}/{topology}/mask={mask}")
                    for index, (before, after) in enumerate(zip(start_rows, stretch_rows)):
                        if index == auto_row_index:
                            require(after >= before - TOLERANCE, f"stretch shrank auto track for {track_list}/{topology}/mask={mask}")
                        else:
                            max_alignment_delta = max(max_alignment_delta, abs(after - before))
                            require(near(before, after), f"stretch changed non-auto track for {track_list}/{topology}/mask={mask}")
                else:
                    for index, (before, after) in enumerate(zip(start_rows, stretch_rows)):
                        max_alignment_delta = max(max_alignment_delta, abs(after - before))
                        require(near(before, after), f"stretch changed intrinsic tail track for {track_list}/{topology}/mask={mask}")

            for topology in TOPOLOGY_NAMES:
                family, _ = topology.split("-", 1)
                zero = geometries[("start", topology, 0)]
                for mask in range(expected_masks[family]):
                    candidate = geometries[("start", topology, mask)]
                    if geometry_error(zero, candidate) > TOLERANCE:
                        baseline_geometry_changes[(family, mask)] += 1
            lists_checked += 1
            group = []

    require(not group, "trailing incomplete track-list group")
    require(scene_count == EXPECTED_SCENES, f"expected {EXPECTED_SCENES} scenes, got {scene_count}")
    require(len(unique_lists) == EXPECTED_LISTS, f"expected {EXPECTED_LISTS} track lists, got {len(unique_lists)}")
    require(positions_seen == FIT_POSITION_SETS, "not all six fit-position sets were captured")
    require(cap_orders_seen == CAP_ORDERS, "not all 120 typed cap orders were captured")
    require(tails_seen == {("min-content",), ("max-content",), ("auto",)}, "singleton tail set differs")
    require(lists_checked == EXPECTED_LISTS, "track-list groups are incomplete")
    require(uniform_controls_checked == 34_560, f"expected 34,560 uniform endpoint checks, got {uniform_controls_checked}")
    require(max_source_order_delta <= TOLERANCE, f"source-order geometry delta {max_source_order_delta}")
    require(max_alignment_delta <= TOLERANCE, f"non-auto alignment delta {max_alignment_delta}")
    require(max_span_error <= TOLERANCE, f"span-frame/covered-row error {max_span_error}")
    require(max_endpoint_error <= TOLERANCE, f"uniform endpoint regression {max_endpoint_error}")

    expected_per_mask = EXPECTED_LISTS * 2
    for topology in TOPOLOGY_NAMES:
        family, _ = topology.split("-", 1)
        for mask in range(expected_masks[family]):
            require(mask_counts[(topology, mask)] == expected_per_mask,
                    f"incomplete {topology} mask {mask}: {mask_counts[(topology, mask)]}")
            require(mask_floor[(topology, mask, "auto")] == 0,
                    f"auto item content-floor shortfall: {topology} mask {mask}")
    require(sum(value for (family, mask), value in baseline_geometry_changes.items() if mask) > 0,
            "mixed masks produced no geometric change from the all-zero baseline")

    compressed = OBSERVATION.read_bytes()
    result = {
        "schema": "spinon.c11-4.repeated-text-row-span-five-fit-mixed-auto-min-precomparison-verification/v1",
        "capturedAtUtc": metadata["capturedAtUtc"],
        "source": {
            "browserCli": metadata["browserCli"],
            "userAgent": metadata["userAgent"],
            "platform": metadata["platform"],
            "devicePixelRatio": metadata["devicePixelRatio"],
            "viewportCssPx": metadata["viewportCssPx"],
            "fixturePath": str(FIXTURE.relative_to(ROOT)),
            "fixtureBytes": len(fixture_bytes),
            "fixtureSha256": fixture_sha,
            "observationPath": OBSERVATION.name,
            "observationBytes": len(compressed),
            "observationSha256": hashlib.sha256(compressed).hexdigest(),
            "uniformControlObservationPath": str(UNIFORM.relative_to(ROOT)),
        },
        "sceneCount": scene_count,
        "uniqueTrackLists": len(unique_lists),
        "uniqueFitPositionSets": len(positions_seen),
        "uniqueTypedCapOrders": len(cap_orders_seen),
        "singletonTailValues": len(tails_seen),
        "sceneCountsByTopology": {topology: sum(mask_counts[(topology, mask)] for mask in range(expected_masks[topology.split("-", 1)[0]])) for topology in TOPOLOGY_NAMES},
        "maskCountsByTopology": {topology: expected_masks[topology.split("-", 1)[0]] for topology in TOPOLOGY_NAMES},
        "scenesPerTopologyMask": expected_per_mask,
        "uniformEndpointScenesRechecked": uniform_controls_checked,
        "eligibleAutomaticMinimumItems": eligible_items,
        "automaticMinimumContentFloorViolations": 0,
        "zeroMinimumContentFloorShortfalls": {
            family: sum(mask_floor[(topology, mask, "zero")] for topology in TOPOLOGY_NAMES if topology.startswith(family + "-") for mask in range(expected_masks[family]))
            for family in TOPOLOGIES
        },
        "gridEndOverflowOccurrences": sum(mask_overflow.values()),
        "gridEndOverflowOccurrencesByMinimumHeight": {
            mode: sum(mask_overflow[(topology, mask, mode)] for topology in TOPOLOGY_NAMES
                      for mask in range(expected_masks[topology.split("-", 1)[0]]))
            for mode in ("auto", "zero")
        },
        "mixedMaskGeometryChangesFromAllZero": {f"{family}|{mask}": value for (family, mask), value in sorted(baseline_geometry_changes.items()) if mask},
        "maxErrorsCssPx": {
            "spanFrameVsCoveredRows": max_span_error,
            "sourceOrderGeometry": max_source_order_delta,
            "nonAutoAlignmentGeometry": max_alignment_delta,
            "uniformZeroAndAutoEndpoints": max_endpoint_error,
        },
        "acceptance": {
            "all414720ScenesCaptured": scene_count == EXPECTED_SCENES,
            "allFiveFitListsAndTypedOrdersPresent": len(unique_lists) == EXPECTED_LISTS and len(cap_orders_seen) == 120,
            "allPerItemMasksPresent": all(mask_counts[(topology, mask)] == expected_per_mask for topology in TOPOLOGY_NAMES for mask in range(expected_masks[topology.split("-", 1)[0]])),
            "automaticMinimumItemsPreserveContentFloor": all(mask_floor[(topology, mask, "auto")] == 0 for topology in TOPOLOGY_NAMES for mask in range(expected_masks[topology.split("-", 1)[0]])),
            "sourceOrderAndAlignmentInvariants": max_source_order_delta <= TOLERANCE and max_alignment_delta <= TOLERANCE,
            "coveredRowFramesMatch": max_span_error <= TOLERANCE,
            "uniformEndpointsMatchExistingMatrix": uniform_controls_checked == 34_560 and max_endpoint_error <= TOLERANCE,
        },
        "limits": [
            "Agent Browser Chrome CSS observation only; no Spinon implementation result.",
            "Fixed five-fit typed-cap multiset, two row-span topology families, and definite 40x220px one-column Grid.",
            "No public CSS support, WPT, Android/iOS runtime geometry, device pixels, or C11.4 completion claim.",
        ],
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
