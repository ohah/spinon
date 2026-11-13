#!/usr/bin/env python3
"""Build the deterministic three-fit RuntimeSession sample from Chrome's oracle."""

from __future__ import annotations

import gzip
import hashlib
import itertools
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
SOURCE = HERE / "chrome157-three-fit-mixed-observation.ndjson.gz"
OUTPUT = HERE / "runtime-matrix-6912.ndjson.gz"
REPRESENTATIVE = HERE / "representative-scene-54448.json"
SOURCE_SHA256 = "d6f96c63404dbf6f432b2abc1f5bc0aea5d630ebdc6dfa8055c9d7456895b4ea"
CAPS = ("0px", "10px", "20%")
TAILS = (
    ("min-content", "max-content", "auto"),
    ("min-content", "auto", "max-content"),
    ("max-content", "min-content", "auto"),
    ("max-content", "auto", "min-content"),
    ("auto", "min-content", "max-content"),
    ("auto", "max-content", "min-content"),
)


def swap_permutations(values: list[str], index: int = 0) -> list[tuple[str, ...]]:
    if index == len(values):
        return [tuple(values)]
    result = []
    for next_index in range(index, len(values)):
        values[index], values[next_index] = values[next_index], values[index]
        result.extend(swap_permutations(values, index + 1))
        values[index], values[next_index] = values[next_index], values[index]
    return result


def render_track_list(
    positions: tuple[int, ...], caps: tuple[str, ...], tail: tuple[str, ...]
) -> str:
    tracks: list[str | None] = [None] * 6
    if len(positions) != len(caps):
        raise ValueError("fit positions and cap assignment lengths differ")
    for position, cap in zip(positions, caps):
        tracks[position - 1] = f"fit-content({cap})"
    remaining = iter(tail)
    return " ".join(track if track is not None else next(remaining) for track in tracks)


def selected_track_lists() -> list[dict[str, object]]:
    positions = list(itertools.combinations(range(1, 7), 3))
    cap_orders = swap_permutations(list(CAPS))
    assert len(positions) == 20 and len(cap_orders) == 6 and len(TAILS) == 6
    selected = []
    for rank in range(36):
        position_rank = rank % 20
        cap_rank = rank // 6
        tail_rank = rank % 6
        fit_positions = positions[position_rank]
        selected.append(
            {
                "sampleRank": rank,
                "positionRank": position_rank,
                "fitPositions": list(fit_positions),
                "capRank": cap_rank,
                "fitCaps": list(cap_orders[cap_rank]),
                "tailRank": tail_rank,
                "tail": list(TAILS[tail_rank]),
                "trackList": render_track_list(
                    fit_positions, cap_orders[cap_rank], TAILS[tail_rank]
                ),
            }
        )
    assert len({item["trackList"] for item in selected}) == 36
    return selected


def load_oracle() -> tuple[dict[str, object], list[dict[str, object]]]:
    compressed = SOURCE.read_bytes()
    actual_sha = hashlib.sha256(compressed).hexdigest()
    if actual_sha != SOURCE_SHA256:
        raise SystemExit(f"Chrome source hash mismatch: {actual_sha}")
    with gzip.open(SOURCE, "rt", encoding="utf-8") as source:
        metadata = json.loads(next(source))
        records = [json.loads(line) for line in source]
    if metadata.get("sceneCount") != 138_240 or len(records) != 138_240:
        raise SystemExit("Chrome oracle must contain exactly 138,240 scenes")
    return metadata, records


def main() -> None:
    source_metadata, records = load_oracle()
    source_sha = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    selection = selected_track_lists()
    selected_names = {item["trackList"] for item in selection}
    first_scene_index = {}
    for record in records:
        track_list = record["scene"]["trackList"]
        first_scene_index.setdefault(track_list, record["index"])
    selection.sort(key=lambda item: first_scene_index[item["trackList"]])
    for sample_rank, item in enumerate(selection):
        item["sourceFirstSceneIndex"] = first_scene_index[item["trackList"]]
        item["selectionRank"] = sample_rank

    scenes = []
    for record in records:
        if record["scene"]["trackList"] in selected_names:
            scenes.append({**record, "sampleIndex": len(scenes)})
    if len(scenes) != 6_912:
        raise SystemExit(f"expected 6,912 selected scenes, got {len(scenes)}")
    representative = next(record for record in records if record["index"] == 54_448)
    if representative["scene"]["minimumMask"] != 16:
        raise SystemExit("representative scene must retain the semantic mask-16 border-overflow case")

    metadata = {
        "record": "metadata",
        "schema": "spinon.c11-4.repeated-text-row-span-three-fit-mixed-runtime-sample/v1",
        "sceneCount": len(scenes),
        "fullOracleSceneCount": source_metadata["sceneCount"],
        "fullOracleSha256": source_sha,
        "selectedTrackListCount": len(selection),
        "trackListSelection": selection,
        "selectionRule": "rank k=0..35: position rank k mod 20; cap rank floor(k/6); tail rank k mod 6",
        "ordering": "original Chrome scene index; sampleIndex is contiguous",
    }
    lines = [json.dumps(metadata, separators=(",", ":"), ensure_ascii=False)]
    lines.extend(json.dumps(record, separators=(",", ":"), ensure_ascii=False) for record in scenes)
    with OUTPUT.open("wb") as output:
        with gzip.GzipFile(filename="", fileobj=output, mode="wb", compresslevel=9, mtime=0) as encoder:
            encoder.write(("\n".join(lines) + "\n").encode("utf-8"))
    REPRESENTATIVE.write_text(
        json.dumps(representative, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    print(f"source_sha256={source_sha}")
    print(f"selected_track_lists={len(selection)} scenes={len(scenes)}")
    print(f"source_alignment={source_metadata['alignments']}")
    print(f"output={OUTPUT.name} bytes={OUTPUT.stat().st_size}")
    print(f"output_sha256={hashlib.sha256(OUTPUT.read_bytes()).hexdigest()}")
    print(f"representative={REPRESENTATIVE.name} index={representative['index']}")


if __name__ == "__main__":
    main()
