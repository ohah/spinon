#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE / "chrome157-five-fit-mixed-observation.ndjson.gz"
SAMPLE = HERE / "runtime-matrix-3984.ndjson.gz"
MANIFEST = HERE / "runtime-sample-manifest.json"
EXPECTED_SCENES = 3_984


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def read_rows(path):
    with gzip.open(path, "rt") as stream:
        return [json.loads(line) for line in stream if line.strip()]


def scene_key(row):
    scene = row["scene"]
    return (
        scene["trackList"],
        scene["topology"],
        scene["minimumMask"],
        scene["alignContent"],
        row["index"],
    )


def main():
    source_hash = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    sample_hash = hashlib.sha256(SAMPLE.read_bytes()).hexdigest()
    manifest = json.loads(MANIFEST.read_text())
    require(manifest["sourceSha256"] == source_hash, "source Chrome matrix hash differs")
    require(manifest["outputSha256"] == sample_hash, "runtime sample hash differs")
    require(manifest["outputSceneCount"] == EXPECTED_SCENES, "manifest sample count differs")

    source_rows = read_rows(SOURCE)
    require(source_rows[0]["record"] == "metadata", "source metadata is absent")
    source_scenes = source_rows[1:]
    require(len(source_scenes) == 414_720, "full Chrome source scene count differs")
    source_by_track = collections.defaultdict(list)
    descriptors = {}
    cap_orders = set()
    positions = set()
    tails = set()
    for row in source_scenes:
        scene = row["scene"]
        track = scene["trackList"]
        source_by_track[track].append(row)
        descriptor = (
            tuple(scene["fitPositions"]),
            tuple(scene["fitCaps"]),
            tuple(scene["tail"]),
        )
        descriptors.setdefault(descriptor, track)
        require(descriptors[descriptor] == track, "one authored descriptor maps to multiple lists")
        positions.add(descriptor[0])
        cap_orders.add(descriptor[1])
        tails.add(descriptor[2])

    sorted_positions = sorted(positions)
    sorted_caps = sorted(cap_orders)
    sorted_tails = sorted(tails)
    require((len(sorted_positions), len(sorted_caps), len(sorted_tails)) == (6, 120, 3),
            "full source axis counts differ")
    selected_tracks = []
    for cap_index, cap_order in enumerate(sorted_caps):
        position = sorted_positions[cap_index % 6]
        tail = sorted_tails[(cap_index // 6) % 3]
        selected_tracks.append(descriptors[(position, cap_order, tail)])
    require(len(set(selected_tracks)) == 120, "sample repeats a typed-cap context")

    representative = set(selected_tracks[:18])
    expected_keys = set()
    for cap_index, track in enumerate(selected_tracks):
        is_representative = track in representative
        for row in source_by_track[track]:
            scene = row["scene"]
            family = scene["topology"].rsplit("-", 1)[0]
            full_mask_count = 32 if family == "span2Six" else 16
            if is_representative:
                selected = 0 < scene["minimumMask"] < full_mask_count - 1
            else:
                rotating_mask = 1 + cap_index % (full_mask_count - 2)
                selected = scene["minimumMask"] == rotating_mask
            if selected:
                expected_keys.add(scene_key(row))

    sample_rows = read_rows(SAMPLE)
    metadata = sample_rows[0]
    require(metadata["schema"] == "spinon.c11-4.five-fit-mixed-runtime-sample/v1",
            "runtime sample schema differs")
    sample_scenes = sample_rows[1:]
    require(len(sample_scenes) == EXPECTED_SCENES, "runtime sample scene count differs")
    require([row["sampleIndex"] for row in sample_scenes] == list(range(EXPECTED_SCENES)),
            "runtime sample indices are not continuous")
    actual_keys = {scene_key(row) for row in sample_scenes}
    require(len(actual_keys) == EXPECTED_SCENES, "runtime sample contains duplicate scenes")
    require(actual_keys == expected_keys, "runtime sample does not match the balanced selection rule")
    require(metadata["fullOracleSha256"] == source_hash, "embedded source identity differs")
    require(metadata["selectedTrackListCount"] == 120, "selected track-list count differs")
    require(metadata["representativePositionTailContexts"] == 18, "representative context count differs")

    by_sample_track = collections.defaultdict(list)
    for row in sample_scenes:
        by_sample_track[row["scene"]["trackList"]].append(row)
    require(len(by_sample_track) == 120, "runtime sample list count differs")
    for track in representative:
        require(len(by_sample_track[track]) == 176, f"{track}: all partial masks are present")
    for track in set(selected_tracks) - representative:
        require(len(by_sample_track[track]) == 8, f"{track}: rotating partial-mask set is complete")

    representative_axes = {
        (tuple(row["scene"]["fitPositions"]), tuple(row["scene"]["tail"]))
        for track in representative
        for row in by_sample_track[track][:1]
    }
    require(len(representative_axes) == 18, "representative lists miss a fit-position/tail context")
    for family, mask_count in (("span2Six", 32), ("span2Span3Six", 16)):
        for position, tail in itertools.product(sorted_positions, sorted_tails):
            masks = {
                mask
                for track in representative
                for row in by_sample_track[track]
                if row["scene"]["topology"].startswith(family + "-")
                and tuple(row["scene"]["fitPositions"]) == position
                and tuple(row["scene"]["tail"]) == tail
                for mask in [row["scene"]["minimumMask"]]
            }
            require(masks == set(range(1, mask_count - 1)),
                    f"{family}/{position}/{tail}: intermediate mask set differs")

    summary = {
        "verifiedSceneCount": len(sample_scenes),
        "sourceOracleSceneCount": len(source_scenes),
        "selectedTrackListCount": len(by_sample_track),
        "typedCapOrdersCovered": len(sorted_caps),
        "representativePositionTailContexts": len(representative_axes),
        "intermediateMasksPerRepresentativeContext": {"span2Six": 30, "span2Span3Six": 14},
        "rotationTrackLists": len(set(selected_tracks) - representative),
        "acceptance": True,
    }
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
