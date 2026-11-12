#!/usr/bin/env python3
import collections
import gzip
import hashlib
import itertools
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = HERE / "chrome157-five-fit-mixed-observation.ndjson.gz"
OUTPUT = HERE / "runtime-matrix-3984.ndjson.gz"
MANIFEST = HERE / "runtime-sample-manifest.json"


def main():
    source_bytes = SOURCE.read_bytes()
    source_sha = hashlib.sha256(source_bytes).hexdigest()
    with gzip.open(SOURCE, "rt") as stream:
        metadata = json.loads(next(stream))
        records = [json.loads(line) for line in stream if line.strip()]

    assert metadata["sceneCount"] == 414_720
    assert len(records) == 414_720
    by_track = collections.defaultdict(list)
    track_by_descriptor = {}
    positions = set()
    tails = set()
    cap_orders = set()
    for row in records:
        scene = row["scene"]
        track_list = scene["trackList"]
        by_track[track_list].append(row)
        descriptor = (
            tuple(scene["fitPositions"]),
            tuple(scene["fitCaps"]),
            tuple(scene["tail"]),
        )
        previous = track_by_descriptor.setdefault(descriptor, track_list)
        assert previous == track_list
        positions.add(descriptor[0])
        cap_orders.add(descriptor[1])
        tails.add(descriptor[2])

    position_sets = sorted(positions)
    typed_cap_orders = sorted(cap_orders)
    tail_values = sorted(tails)
    assert (len(position_sets), len(typed_cap_orders), len(tail_values)) == (6, 120, 3)

    selected_order_lists = []
    for cap_index, cap_order in enumerate(typed_cap_orders):
        position_set = position_sets[cap_index % len(position_sets)]
        tail = tail_values[(cap_index // len(position_sets)) % len(tail_values)]
        selected_order_lists.append(
            track_by_descriptor[(position_set, cap_order, tail)]
        )
    assert len(set(selected_order_lists)) == 120
    representative_lists = set(selected_order_lists[:18])
    representative_contexts = {
        (
            tuple(by_track[track][0]["scene"]["fitPositions"]),
            tuple(by_track[track][0]["scene"]["tail"]),
        )
        for track in representative_lists
    }
    assert representative_contexts == {
        (position_set, tail)
        for position_set, tail in itertools.product(position_sets, tail_values)
    }

    order_index = {track: index for index, track in enumerate(selected_order_lists)}
    sample = []
    selected_masks = collections.defaultdict(set)
    for track_list in selected_order_lists:
        cap_index = order_index[track_list]
        representative = track_list in representative_lists
        for row in by_track[track_list]:
            scene = row["scene"]
            family, order = scene["topology"].rsplit("-", 1)
            assert order in ("forward", "reverse")
            mask = scene["minimumMask"]
            mask_count = 32 if family == "span2Six" else 16
            if representative:
                selected = 0 < mask < mask_count - 1
            else:
                expected_mask = 1 + cap_index % (mask_count - 2)
                selected = mask == expected_mask
            if not selected:
                continue
            sample.append(row)
            selected_masks[(family, tuple(scene["fitPositions"]), tuple(scene["tail"]))].add(mask)

    assert len(sample) == 3_984, f"expected 3,984 RuntimeSession scenes, got {len(sample)}"
    assert len({row["scene"]["trackList"] for row in sample}) == 120
    for family in ("span2Six", "span2Span3Six"):
        for position_set, tail in itertools.product(position_sets, tail_values):
            expected = set(range(1, (32 if family == "span2Six" else 16) - 1))
            assert selected_masks[(family, position_set, tail)] == expected

    sample_metadata = {
        "record": "metadata",
        "schema": "spinon.c11-4.five-fit-mixed-runtime-sample/v1",
        "sceneCount": len(sample),
        "fullOracleSceneCount": metadata["sceneCount"],
        "fullOracleSha256": source_sha,
        "selectedTrackListCount": len(order_index),
        "typedCapOrdersCovered": len(typed_cap_orders),
        "representativePositionTailContexts": len(representative_contexts),
        "partialMasksPerRepresentativeFamily": {"span2Six": 30, "span2Span3Six": 14},
        "sampledMaskRuleForOtherLists": "one rotating partial mask per topology family for each remaining typed cap order",
        "endpointCoverage": "existing five-fit uniform RuntimeSession matrix covers mask 0 and all-auto on all 2,160 track lists",
        "geometryVariantsPerMask": ["forward", "reverse", "start", "stretch"],
    }
    output_lines = [json.dumps(sample_metadata, ensure_ascii=False)]
    for sample_index, source_row in enumerate(sample):
        row = dict(source_row)
        row["sampleIndex"] = sample_index
        output_lines.append(json.dumps(row, ensure_ascii=False))
    output_bytes = ("\n".join(output_lines) + "\n").encode("utf-8")
    compressed = gzip.compress(output_bytes, mtime=0)
    OUTPUT.write_bytes(compressed)

    manifest = {
        "schema": "spinon.c11-4.five-fit-mixed-runtime-sample-manifest/v1",
        "sourcePath": SOURCE.name,
        "sourceSha256": source_sha,
        "outputPath": OUTPUT.name,
        "outputCompressedBytes": len(compressed),
        "outputSha256": hashlib.sha256(compressed).hexdigest(),
        "outputSceneCount": len(sample),
        "outputTrackListCount": len(order_index),
        "representativePositionTailContexts": len(representative_contexts),
        "partialMasksVerifiedPerRepresentativeContext": {"span2Six": 30, "span2Span3Six": 14},
    }
    MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(manifest, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
