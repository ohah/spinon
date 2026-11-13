#!/usr/bin/env python3
"""Independently check exact three-fit runtime sample membership and order."""

from __future__ import annotations

import gzip
import hashlib
import itertools
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ORACLE = HERE / "chrome157-three-fit-mixed-observation.ndjson.gz"
SAMPLE = HERE / "runtime-matrix-6912.ndjson.gz"
REPRESENTATIVE = HERE / "representative-scene-54448.json"
ORACLE_SHA256 = "d6f96c63404dbf6f432b2abc1f5bc0aea5d630ebdc6dfa8055c9d7456895b4ea"
CAP_ORDERS = (
    ("0px", "10px", "20%"),
    ("0px", "20%", "10px"),
    ("10px", "0px", "20%"),
    ("10px", "20%", "0px"),
    ("20%", "10px", "0px"),
    ("20%", "0px", "10px"),
)
TAIL_ORDERS = (
    ("min-content", "max-content", "auto"),
    ("min-content", "auto", "max-content"),
    ("max-content", "min-content", "auto"),
    ("max-content", "auto", "min-content"),
    ("auto", "min-content", "max-content"),
    ("auto", "max-content", "min-content"),
)


def render_track_list(
    positions: tuple[int, ...], caps: tuple[str, ...], tail: tuple[str, ...]
) -> str:
    tracks: list[str | None] = [None] * 6
    for index, position in enumerate(positions):
        tracks[position - 1] = f"fit-content({caps[index]})"
    remaining = iter(tail)
    return " ".join(track if track is not None else next(remaining) for track in tracks)


def expected_selections() -> list[dict[str, object]]:
    positions = list(itertools.combinations(range(1, 7), 3))
    return [
        {
            "sampleRank": rank,
            "positionRank": rank % 20,
            "fitPositions": list(positions[rank % 20]),
            "capRank": rank // 6,
            "fitCaps": list(CAP_ORDERS[rank // 6]),
            "tailRank": rank % 6,
            "tail": list(TAIL_ORDERS[rank % 6]),
            "trackList": render_track_list(
                positions[rank % 20],
                CAP_ORDERS[rank // 6],
                TAIL_ORDERS[rank % 6],
            ),
        }
        for rank in range(36)
    ]


def load_lines(path: Path) -> list[dict[str, object]]:
    with gzip.open(path, "rt", encoding="utf-8") as stream:
        return [json.loads(line) for line in stream]


def main() -> None:
    oracle_hash = hashlib.sha256(ORACLE.read_bytes()).hexdigest()
    if oracle_hash != ORACLE_SHA256:
        raise SystemExit(f"oracle hash mismatch: {oracle_hash}")
    oracle = load_lines(ORACLE)
    sample = load_lines(SAMPLE)
    metadata, scenes = sample[0], sample[1:]
    if metadata.get("schema") != "spinon.c11-4.repeated-text-row-span-three-fit-mixed-runtime-sample/v1":
        raise SystemExit("unexpected runtime sample schema")
    if metadata.get("fullOracleSha256") != oracle_hash:
        raise SystemExit("sample is not bound to the pinned Chrome oracle")
    if metadata.get("fullOracleSceneCount") != 138_240 or len(oracle) != 138_241:
        raise SystemExit("pinned Chrome oracle must contain 138,240 scenes")
    if metadata.get("sceneCount") != 6_912 or len(scenes) != 6_912:
        raise SystemExit(f"runtime sample must contain 6,912 scenes, got {len(scenes)}")
    if metadata.get("selectedTrackListCount") != 36:
        raise SystemExit("runtime sample metadata must declare 36 track lists")

    expected = expected_selections()
    expected_by_list = {item["trackList"]: item for item in expected}
    if len(expected_by_list) != 36:
        raise SystemExit("deterministic selector did not produce 36 distinct track lists")
    first_indices: dict[str, int] = {}
    for record in oracle[1:]:
        track_list = record["scene"]["trackList"]
        if track_list in expected_by_list:
            first_indices.setdefault(track_list, record["index"])
    expected.sort(key=lambda item: first_indices[item["trackList"]])
    for rank, item in enumerate(expected):
        item["sourceFirstSceneIndex"] = first_indices[item["trackList"]]
        item["selectionRank"] = rank
    if metadata.get("trackListSelection") != expected:
        raise SystemExit("track-list metadata differs from independently generated selection")
    if metadata.get("selectionRule") != "rank k=0..35: position rank k mod 20; cap rank floor(k/6); tail rank k mod 6":
        raise SystemExit("selection rule metadata differs")

    selected = set(expected_by_list)
    expected_scenes = [record for record in oracle[1:] if record["scene"]["trackList"] in selected]
    if len(expected_scenes) != 6_912:
        raise SystemExit(f"oracle selection contains {len(expected_scenes)} scenes")
    if len(scenes) != len(expected_scenes):
        raise SystemExit("sample and source selection lengths differ")
    for index, (actual, source) in enumerate(zip(scenes, expected_scenes)):
        if actual.get("sampleIndex") != index:
            raise SystemExit(f"sampleIndex mismatch at scene {index}")
        if actual != {**source, "sampleIndex": index}:
            raise SystemExit(f"sample scene differs from Chrome oracle at {index}")
        if index and scenes[index - 1]["index"] >= actual["index"]:
            raise SystemExit(f"original Chrome order regresses at sample scene {index}")

    representative = json.loads(REPRESENTATIVE.read_text(encoding="utf-8"))
    source_representative = next(record for record in oracle[1:] if record["index"] == 54_448)
    if representative != source_representative:
        raise SystemExit("representative border-overflow scene differs from Chrome oracle")
    if representative["scene"]["minimumMask"] != 16:
        raise SystemExit("representative scene no longer isolates item E with auto minimum")
    if representative["scene"]["trackList"] != "fit-content(20%) min-content auto fit-content(0px) fit-content(10px) max-content":
        raise SystemExit("representative track-list context changed")

    batches: dict[str, list[dict[str, object]]] = {}
    for record in scenes:
        batches.setdefault(record["scene"]["trackList"], []).append(record)
    if list(batches) != [item["trackList"] for item in expected]:
        raise SystemExit("sample track-list order differs from source-oracle order")
    expected_masks = {
        "span2Six-forward": set(range(32)),
        "span2Six-reverse": set(range(32)),
        "span2Span3Six-forward": set(range(16)),
        "span2Span3Six-reverse": set(range(16)),
    }
    for track_list, batch in batches.items():
        if len(batch) != 192:
            raise SystemExit(f"{track_list}: expected 192 scenes, got {len(batch)}")
        masks: dict[str, set[int]] = {}
        alignments: dict[str, set[str]] = {}
        for record in batch:
            scene = record["scene"]
            topology = scene["topology"]
            masks.setdefault(topology, set()).add(scene["minimumMask"])
            alignments.setdefault(topology, set()).add(scene["alignContent"])
            ids = ["A", "B", "C", "D", "E"] if topology.startswith("span2Six-") else ["A", "B", "C", "D"]
            expected_auto = [item for bit, item in enumerate(ids) if scene["minimumMask"] & (1 << bit)]
            if scene["autoItems"] != expected_auto:
                raise SystemExit(f"{track_list}/{topology}: semantic mask differs")
            empty = next(item for item in record["items"] if item["id"] == "empty")
            if empty["minHeight"] != "0px":
                raise SystemExit(f"{track_list}/{topology}: explicit empty row is not zero-minimum")
        if masks != expected_masks:
            raise SystemExit(f"{track_list}: incomplete topology/mask coverage")
        if any(values != {"start", "stretch"} for values in alignments.values()):
            raise SystemExit(f"{track_list}: incomplete alignment coverage")

    positions = {tuple(item["fitPositions"]) for item in expected}
    caps = {tuple(item["fitCaps"]) for item in expected}
    tails = {tuple(item["tail"]) for item in expected}
    if len(positions) != 20 or caps != set(CAP_ORDERS) or tails != set(TAIL_ORDERS):
        raise SystemExit("sample does not cover all fit positions, cap orders, and tail orders")
    print(json.dumps({
        "oracleSha256": oracle_hash,
        "selectedTrackLists": len(batches),
        "runtimeScenes": len(scenes),
        "fitPositionSets": len(positions),
        "capOrders": len(caps),
        "tailOrders": len(tails),
        "sourceOrderPairs": len(scenes) // 2,
        "originalSceneRange": [scenes[0]["index"], scenes[-1]["index"]],
        "sampleSha256": hashlib.sha256(SAMPLE.read_bytes()).hexdigest(),
        "representativeSceneIndex": representative["index"],
    }, indent=2))


if __name__ == "__main__":
    main()
