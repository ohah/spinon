#!/usr/bin/env python3
"""Recheck the C11.4 fixture payload and defined-host-symbol archive boundary."""

from hashlib import sha256
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
SCENE_MARKERS = (
    b"template-area-repeat-cross-axis-fixed-cols-fill-rows-area-short",
    b"template-area-repeat-cross-axis-fixed-cols-fill-rows-two-track-pattern",
    b"template-area-repeat-cross-axis-fixed-cols-fit-rows-one-row-occupied",
    b"template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area",
    b"template-area-repeat-cross-axis-fixed-cols-fit-rows-middle-collapsed-between-occupied",
    b"template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows",
    b"template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-one-row",
    b"template-area-repeat-cross-axis-fixed-rows-fill-cols-shorthand-area-short",
    b"template-area-repeat-cross-axis-fixed-rows-fit-cols-two-track-span",
    b"template-area-repeat-cross-axis-fixed-rows-fit-cols-empty-area",
    b"template-area-repeat-cross-axis-fixed-rows-fill-cols-area-adds-one-column",
)
PAYLOAD_MARKERS = (
    b"repeat(2,20px) / repeat(auto-fill,20px)",
)
SYMBOLS = (
    "_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture",
    "_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture",
)


def contains(path: Path, needle: bytes) -> bool:
    carry = b""
    with path.open("rb") as source:
        while chunk := source.read(1024 * 1024):
            data = carry + chunk
            if needle in data:
                return True
            carry = data[-(len(needle) - 1) :] if len(needle) > 1 else b""
    return False


def digest(path: Path) -> str:
    result = sha256()
    with path.open("rb") as source:
        while chunk := source.read(1024 * 1024):
            result.update(chunk)
    return result.hexdigest()


def inspect(
    archive_name: str,
    symbol_dump_name: str,
    result_name: str,
    expect_fixture: bool,
) -> None:
    archive = HERE / archive_name
    symbol_dump = HERE / symbol_dump_name
    defined_symbols = {
        line.split()[-1]
        for line in symbol_dump.read_text().splitlines()
        if line.split()
    }
    scene_markers = {
        marker.decode(): contains(archive, marker) for marker in SCENE_MARKERS
    }
    payload_markers = {
        marker.decode(): contains(archive, marker) for marker in PAYLOAD_MARKERS
    }
    result = {
        "artifact": str(archive.relative_to(HERE.parent.parent.parent.parent.parent)),
        "bytes": archive.stat().st_size,
        "sha256": digest(archive),
        "scene_markers_in_archive": scene_markers,
        "scene_marker_count": sum(scene_markers.values()),
        "scene_marker_total": len(scene_markers),
        "all_scene_markers_in_archive": all(scene_markers.values()),
        "payload_markers_in_archive": payload_markers,
        "host_symbols": {
            symbol: {
                "defined_by_llvm_nm": symbol in defined_symbols,
                "symbol_bytes_in_archive": contains(archive, symbol.encode()),
            }
            for symbol in SYMBOLS
        },
    }
    if any(value is not expect_fixture for value in scene_markers.values()):
        raise SystemExit(f"unexpected scene-marker boundary in {archive_name}")
    if any(value is not expect_fixture for value in payload_markers.values()):
        raise SystemExit(f"unexpected payload boundary in {archive_name}")
    if any(
        symbol_data["defined_by_llvm_nm"] is not expect_fixture
        or symbol_data["symbol_bytes_in_archive"] is not expect_fixture
        for symbol_data in result["host_symbols"].values()
    ):
        raise SystemExit(f"unexpected host-symbol boundary in {archive_name}")
    (HERE / result_name).write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


inspect(
    "spinon-ffi-c04-runtime-gpu.a",
    "fixture-free-defined-symbols.txt",
    "fixture-free-boundary-result.json",
    expect_fixture=False,
)
inspect(
    "spinon-ffi-c04-runtime-gpu-fixture-test.a",
    "fixture-test-defined-symbols.txt",
    "fixture-test-boundary-result.json",
    expect_fixture=True,
)
