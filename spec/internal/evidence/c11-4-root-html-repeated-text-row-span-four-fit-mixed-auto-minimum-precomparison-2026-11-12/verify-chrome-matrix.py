#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
import math
import struct
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
FIXTURE = ROOT / "tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison.html"
OBSERVATION = HERE / "chrome157-four-fit-mixed-observation.ndjson.gz"
MANIFEST = HERE / "capture-manifest.json"
CAPTURE = HERE / "capture-chrome.mjs"
SCREENSHOT = HERE / "chrome157-four-fit-mixed-sample.png"
OUTPUT = HERE / "verification.json"
UNIFORM = ROOT / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/chrome157-observation.ndjson"
UNIFORM_VERIFICATION = UNIFORM.with_name("verification.json")
CAPS = ("0px", "10px", "20%", "30px")
ALIGNMENTS = ("start", "stretch")
TAIL_ORDERS = (("min-content", "auto"), ("auto", "min-content"))
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
FIT_POSITION_SETS = set(itertools.combinations(range(1, 7), 4))
CAP_ORDERS = set(itertools.permutations(CAPS))
TOPOLOGY_NAMES = tuple(f"{family}-{order}" for family in TOPOLOGIES for order in ("forward", "reverse"))
TOLERANCE = 0.01
EXPECTED_LISTS = len(FIT_POSITION_SETS) * len(CAP_ORDERS) * len(TAIL_ORDERS)
EXPECTED_SCENES = EXPECTED_LISTS * (32 * 2 + 16 * 2) * len(ALIGNMENTS)


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
                    require(len(left_value) == len(right_value), f"{identifier}: line-box counts differ")
                    error = max(error, max((abs(a - b) for a, b in zip(left_value, right_value)), default=0.0))
                else:
                    error = max(error, abs(left_value - right_value))
    return error


def load_uniform_controls():
    verification = json.loads(UNIFORM_VERIFICATION.read_text())
    data = UNIFORM.read_bytes()
    require(verification["observations"]["sceneCount"] == 11_520, "four-fit uniform control count changed")
    require(verification["observations"]["sha256"] == hashlib.sha256(data).hexdigest(),
            "four-fit uniform control hash differs from its verification record")
    controls = {}
    for index, line in enumerate(data.decode("utf-8").splitlines()):
        if not line.strip():
            continue
        row = json.loads(line)
        require(row["index"] == index, f"four-fit uniform control index differs at {index}")
        require("HeadlessChrome/157." in row["userAgent"] and row["devicePixelRatio"] == 1,
                f"four-fit uniform control browser/DPR differs at {index}")
        scene = row["scene"]
        controls[(scene["trackList"], scene["alignContent"], scene["topology"], scene["minimum"])] = geometry(row)
    require(len(controls) == 11_520, f"incomplete four-fit uniform controls: {len(controls)}")
    return controls


def validate_record(row):
    scene = row["scene"]
    family, order = family_order(scene)
    identifiers = TOPOLOGIES[family]
    mask = scene["minimumMask"]
    require(isinstance(mask, int) and 0 <= mask < (1 << len(identifiers)), f"mask outside {family}: {mask}")
    require(scene["maskItemIds"] == list(identifiers), f"mask bit order differs at {row['index']}")
    expected_auto = [identifier for bit, identifier in enumerate(identifiers) if mask & (1 << bit)]
    require(scene["autoItems"] == expected_auto, f"mask-to-item mapping differs at {row['index']}")
    require(tuple(scene["fitPositions"]) in FIT_POSITION_SETS, f"bad fit positions at {row['index']}")
    require(tuple(scene["fitCaps"]) in CAP_ORDERS, f"bad typed cap permutation at {row['index']}")
    require(len(scene["trackMembers"]) == 6, f"track list length differs at {row['index']}")

    expected_tracks = [None] * 6
    for position, cap in zip(scene["fitPositions"], scene["fitCaps"]):
        expected_tracks[position - 1] = f"fit-content({cap})"
    tail_positions = [index for index, value in enumerate(expected_tracks) if value is None]
    require(len(tail_positions) == 2 and tuple(scene["tail"]) in TAIL_ORDERS,
            f"two intrinsic tail tracks differ at {row['index']}")
    for position, track in zip(tail_positions, scene["tail"]):
        expected_tracks[position] = track
    require(scene["trackMembers"] == expected_tracks, f"cap position or tail order changed at {row['index']}")
    require(scene["trackList"] == " ".join(expected_tracks), f"authored track-list differs at {row['index']}")
    require(row["computedAutoRows"] == scene["trackList"], f"computed auto rows differ at {row['index']}")
    require(scene["alignContent"] in ALIGNMENTS and row["computedAlignContent"] == scene["alignContent"],
            f"alignment differs at {row['index']}")
    require(row["devicePixelRatio"] == 1 and "HeadlessChrome/157." in row["userAgent"],
            f"browser/DPR drift at {row['index']}")
    require(all(near(actual, expected, 0.001) for actual, expected in zip(row["gridFrame"], [0, 0, 40, 220])),
            f"definite Grid frame differs at {row['index']}")

    rows = row["rowValues"]
    serialized_rows = [float(value.removesuffix("px")) for value in row["usedRows"].strip().split()]
    require(len(rows) == len(serialized_rows) == 7 and near(rows[0], 10, 0.001),
            f"expected explicit 10px row plus six implicit rows at {row['index']}")
    require(all(math.isfinite(value) and value >= 0 for value in rows), f"non-finite/negative row at {row['index']}")
    require(all(near(actual, serialized, 0.001) for actual, serialized in zip(rows, serialized_rows)),
            f"used row values differ from computed serialization at {row['index']}")

    placements = PLACEMENTS[family]
    if order == "reverse":
        placements = tuple(reversed(placements))
    require(scene["items"] == [list(value) for value in placements], f"explicit placements changed at {row['index']}")
    placement_by_id = {identifier: (start, span) for identifier, start, span in scene["items"]}
    item_map = {item["id"]: item for item in row["items"]}
    require(len(item_map) == len(row["items"]) and set(item_map) == {"empty", *identifiers},
            f"item owner set differs at {row['index']}")
    require(item_map["empty"]["minHeight"] == "0px" and
            all(near(actual, expected, TOLERANCE) for actual, expected in zip(item_map["empty"]["frame"], [0, 0, 40, 10])),
            f"explicit-row empty control changed at {row['index']}")

    floor_shortfalls = {"auto": 0, "zero": 0}
    grid_end_overflow = {"auto": 0, "zero": 0}
    eligible_items = 0
    max_span_error = 0.0
    for identifier in identifiers:
        item = item_map[identifier]
        is_auto = identifier in expected_auto
        mode = "auto" if is_auto else "zero"
        expected_minimum = "auto" if is_auto else "0px"
        require(item["minHeight"] == expected_minimum, f"{identifier} computed minimum differs at {row['index']}")
        require(item["text"] == TEXTS[identifier] and len(item["lineRects"]) == LINE_COUNTS[identifier],
                f"{identifier} payload or line geometry differs at {row['index']}")
        require(item["payloadFrame"] is not None and len(item["payloadFrame"]) == 4,
                f"{identifier} payload frame missing at {row['index']}")
        start, span = placement_by_id[identifier]
        expected_y = sum(rows[:start - 1])
        expected_height = sum(rows[start - 1:start - 1 + span])
        span_error = max(abs(item["frame"][1] - expected_y), abs(item["frame"][3] - expected_height))
        max_span_error = max(max_span_error, span_error)
        require(span_error <= TOLERANCE and near(item["frame"][0], 0) and near(item["frame"][2], 40),
                f"{identifier} frame differs from its covered rows at {row['index']}")
        require(near(item["payloadFrame"][1], item["frame"][1]),
                f"{identifier} payload top differs from its item at {row['index']}")
        covered_tracks = scene["trackMembers"][start - 2:start - 2 + span]
        eligible = any(track == "auto" or track.startswith("fit-content(") for track in covered_tracks)
        if eligible:
            eligible_items += 1
            shortfall = max(0.0, item["payloadFrame"][3] - item["frame"][3])
            if shortfall > TOLERANCE:
                floor_shortfalls[mode] += 1
        overflow = max(0.0, item["payloadFrame"][1] + item["payloadFrame"][3] - 220.0)
        if overflow > TOLERANCE:
            grid_end_overflow[mode] += 1
    return floor_shortfalls, grid_end_overflow, eligible_items, max_span_error


def main():
    fixture_bytes = FIXTURE.read_bytes()
    fixture_sha = hashlib.sha256(fixture_bytes).hexdigest()
    manifest = json.loads(MANIFEST.read_text())
    capture_sha = hashlib.sha256(CAPTURE.read_bytes()).hexdigest()
    require(manifest["schema"] == "spinon.c11-4.repeated-text-row-span-four-fit-mixed-auto-min-precomparison/v1",
            "capture manifest schema differs")
    require(manifest["fixtureSha256"] == fixture_sha, "manifest fixture hash differs")
    require(manifest["collector"]["sha256"] == capture_sha, "manifest collector hash differs")
    require(manifest["sceneCount"] == EXPECTED_SCENES and manifest["uniqueTrackListCount"] == EXPECTED_LISTS,
            "manifest matrix dimensions differ")
    controls = load_uniform_controls()
    expected_masks = {family: 1 << len(identifiers) for family, identifiers in TOPOLOGIES.items()}
    unique_lists = set()
    positions_seen = set()
    cap_orders_seen = set()
    tails_seen = set()
    mask_counts = collections.Counter()
    mask_floor = collections.Counter()
    mask_overflow = collections.Counter()
    mixed_geometry_changes = collections.Counter()
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
                "metadata scene count differs")
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
                    f"track list changed inside group {lists_checked}")
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
                floor, overflow, eligible, span_error = validate_record(record)
                mask_counts[(scene["topology"], mask)] += 1
                mask_floor[(scene["topology"], mask, "auto")] += floor["auto"]
                mask_floor[(scene["topology"], mask, "zero")] += floor["zero"]
                mask_overflow[(scene["topology"], mask, "auto")] += overflow["auto"]
                mask_overflow[(scene["topology"], mask, "zero")] += overflow["zero"]
                eligible_items += eligible
                max_span_error = max(max_span_error, span_error)
                current_geometry = geometry(record)
                source_key = (scene["alignContent"], family, mask)
                by_source.setdefault(source_key, {})[order] = current_geometry
                alignment_key = (scene["topology"], mask)
                by_alignment.setdefault(alignment_key, {})[scene["alignContent"]] = (record, current_geometry)
                geometries[(scene["alignContent"], scene["topology"], mask)] = current_geometry

                uniform = "zero" if mask == 0 else "auto" if mask == (1 << len(TOPOLOGIES[family])) - 1 else None
                if uniform:
                    key = (track_list, scene["alignContent"], scene["topology"], uniform)
                    require(key in controls, f"prior four-fit uniform control missing: {key}")
                    endpoint_error = geometry_error(current_geometry, controls[key])
                    max_endpoint_error = max(max_endpoint_error, endpoint_error)
                    uniform_controls_checked += 1
                    require(endpoint_error <= TOLERANCE, f"uniform endpoint differs from prior matrix: {key}; {endpoint_error}")

            require(len(by_source) == 96 and all(set(pair) == {"forward", "reverse"} for pair in by_source.values()),
                    f"source-order pairs incomplete for {track_list}")
            for pair in by_source.values():
                max_source_order_delta = max(max_source_order_delta, geometry_error(pair["forward"], pair["reverse"]))

            require(len(by_alignment) == 96 and all(set(pair) == set(ALIGNMENTS) for pair in by_alignment.values()),
                    f"alignment pairs incomplete for {track_list}")
            for (topology, mask), pair in by_alignment.items():
                start_record, _ = pair["start"]
                stretch_record, _ = pair["stretch"]
                start_rows = start_record["rowValues"]
                stretch_rows = stretch_record["rowValues"]
                track_members = start_record["scene"]["trackMembers"]
                auto_track_index = track_members.index("auto")
                auto_row_index = auto_track_index + 1
                start_total = sum(start_rows)
                stretch_total = sum(stretch_rows)
                require(near(stretch_total, max(start_total, 220), 0.02),
                        f"auto-tail stretch total differs for {track_list}/{topology}/mask={mask}")
                for index, (before, after) in enumerate(zip(start_rows, stretch_rows)):
                    if index == auto_row_index:
                        require(after >= before - TOLERANCE,
                                f"stretch shrank auto track for {track_list}/{topology}/mask={mask}")
                    else:
                        max_alignment_delta = max(max_alignment_delta, abs(after - before))
                        require(near(before, after),
                                f"stretch changed non-auto track for {track_list}/{topology}/mask={mask}")

            for topology in TOPOLOGY_NAMES:
                family, _ = topology.split("-", 1)
                zero = geometries[("start", topology, 0)]
                for mask in range(expected_masks[family]):
                    candidate = geometries[("start", topology, mask)]
                    if geometry_error(zero, candidate) > TOLERANCE:
                        mixed_geometry_changes[(family, mask)] += 1
            lists_checked += 1
            group = []

    require(not group, "trailing incomplete track-list group")
    require(scene_count == EXPECTED_SCENES, f"expected {EXPECTED_SCENES} scenes, got {scene_count}")
    require(len(unique_lists) == EXPECTED_LISTS, f"expected {EXPECTED_LISTS} track lists, got {len(unique_lists)}")
    require(positions_seen == FIT_POSITION_SETS, "not all fifteen four-fit position sets were captured")
    require(cap_orders_seen == CAP_ORDERS, "not all twenty-four typed cap orders were captured")
    require(tails_seen == set(TAIL_ORDERS), "both intrinsic tail orders were not captured")
    require(lists_checked == EXPECTED_LISTS, "track-list groups are incomplete")
    require(uniform_controls_checked == 11_520,
            f"expected 11,520 uniform endpoint checks, got {uniform_controls_checked}")
    require(max_source_order_delta <= TOLERANCE, f"source-order geometry delta {max_source_order_delta}")
    require(max_alignment_delta <= TOLERANCE, f"non-auto alignment geometry delta {max_alignment_delta}")
    require(max_span_error <= TOLERANCE, f"span-frame/covered-row error {max_span_error}")
    require(max_endpoint_error <= TOLERANCE, f"uniform endpoint regression {max_endpoint_error}")

    expected_per_mask = EXPECTED_LISTS * len(ALIGNMENTS)
    for topology in TOPOLOGY_NAMES:
        family, _ = topology.split("-", 1)
        for mask in range(expected_masks[family]):
            require(mask_counts[(topology, mask)] == expected_per_mask,
                    f"incomplete {topology} mask {mask}: {mask_counts[(topology, mask)]}")
            require(mask_floor[(topology, mask, "auto")] == 0,
                    f"auto item content-floor shortfall: {topology} mask {mask}")
    require(sum(value for (_, mask), value in mixed_geometry_changes.items() if mask) > 0,
            "mixed masks did not change geometry from the all-zero baseline")

    compressed = OBSERVATION.read_bytes()
    screenshot_bytes = SCREENSHOT.read_bytes()
    require(screenshot_bytes.startswith(b"\x89PNG\r\n\x1a\n"), "sample screenshot is not a PNG")
    screenshot_width, screenshot_height = struct.unpack(">II", screenshot_bytes[16:24])
    require((screenshot_width, screenshot_height) == (480, 900), "sample screenshot dimensions differ")
    require(manifest["capturedAtUtc"] == metadata["capturedAtUtc"], "manifest capture timestamp differs")
    require(manifest["observationPath"] == OBSERVATION.name and
            manifest["observationBytes"] == len(compressed) and
            manifest["observationSha256"] == hashlib.sha256(compressed).hexdigest(),
            "manifest observation file identity differs")
    require(manifest["sampleScreenshot"] == SCREENSHOT.name and
            manifest["sampleScreenshotBytes"] == len(screenshot_bytes) and
            manifest["sampleScreenshotSha256"] == hashlib.sha256(screenshot_bytes).hexdigest(),
            "manifest screenshot file identity differs")
    result = {
        "schema": "spinon.c11-4.repeated-text-row-span-four-fit-mixed-auto-min-precomparison-verification/v1",
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
        "tailOrders": len(tails_seen),
        "sceneCountsByTopology": {
            topology: sum(mask_counts[(topology, mask)] for mask in range(expected_masks[topology.split("-", 1)[0]]))
            for topology in TOPOLOGY_NAMES
        },
        "maskCountsByTopology": {topology: expected_masks[topology.split("-", 1)[0]] for topology in TOPOLOGY_NAMES},
        "scenesPerTopologyMask": expected_per_mask,
        "uniformEndpointScenesRechecked": uniform_controls_checked,
        "eligibleAutomaticMinimumItems": eligible_items,
        "automaticMinimumContentFloorViolations": 0,
        "zeroMinimumContentFloorShortfalls": {
            family: sum(mask_floor[(topology, mask, "zero")]
                        for topology in TOPOLOGY_NAMES if topology.startswith(family + "-")
                        for mask in range(expected_masks[family]))
            for family in TOPOLOGIES
        },
        "gridEndOverflowOccurrences": sum(mask_overflow.values()),
        "gridEndOverflowOccurrencesByMinimumHeight": {
            mode: sum(mask_overflow[(topology, mask, mode)] for topology in TOPOLOGY_NAMES
                      for mask in range(expected_masks[topology.split("-", 1)[0]]))
            for mode in ("auto", "zero")
        },
        "mixedMaskGeometryChangesFromAllZero": {
            f"{family}|{mask}": value for (family, mask), value in sorted(mixed_geometry_changes.items()) if mask
        },
        "maxErrorsCssPx": {
            "spanFrameVsCoveredRows": max_span_error,
            "sourceOrderGeometry": max_source_order_delta,
            "nonAutoAlignmentGeometry": max_alignment_delta,
            "uniformZeroAndAutoEndpoints": max_endpoint_error,
        },
        "acceptance": {
            "all138240ScenesCaptured": scene_count == EXPECTED_SCENES,
            "all720FourFitTrackListsPresent": len(unique_lists) == EXPECTED_LISTS,
            "allPerItemMasksPresent": all(mask_counts[(topology, mask)] == expected_per_mask
                                           for topology in TOPOLOGY_NAMES
                                           for mask in range(expected_masks[topology.split("-", 1)[0]])),
            "automaticMinimumItemsPreserveContentFloor": all(mask_floor[(topology, mask, "auto")] == 0
                                                              for topology in TOPOLOGY_NAMES
                                                              for mask in range(expected_masks[topology.split("-", 1)[0]])),
            "sourceOrderAndAlignmentInvariants": max_source_order_delta <= TOLERANCE and max_alignment_delta <= TOLERANCE,
            "coveredRowFramesMatch": max_span_error <= TOLERANCE,
            "uniformEndpointsMatchExistingMatrix": uniform_controls_checked == 11_520 and max_endpoint_error <= TOLERANCE,
        },
        "limits": [
            "Agent Browser Chrome CSS observation only; no Spinon implementation result.",
            "Fixed four-fit typed-cap multiset, two row-span topology families, and definite 40x220px one-column Grid.",
            "No public CSS support, WPT, Android/iOS runtime geometry, device pixels, or C11.4 completion claim.",
        ],
    }
    OUTPUT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
