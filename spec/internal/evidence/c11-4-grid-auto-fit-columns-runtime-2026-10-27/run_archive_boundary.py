#!/usr/bin/env python3
"""Compare product and opt-in C11.4 WPT fixture symbols in the FFI archive."""

from __future__ import annotations

import hashlib
import json
import re
import shlex
import subprocess
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
ARCHIVE = ROOT / "target/debug/libspinon_ffi.a"
SOURCE_FILES = (
    "crates/spinon-ffi/Cargo.toml",
    "crates/spinon-ffi/src/runtime_gpu.rs",
    "crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_fit_columns_wpt.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_fit_columns_wpt_cases.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs",
)
SCENE_MARKER = re.compile(r"\bgrid-auto-fit-columns-(\d{2})\b")
FIXTURE_SYMBOL = re.compile(
    r"spinon_runtime_gpu_host_(?:new|eval)_c11_4_grid_track_sizing_fixture"
)
EXPECTED_MARKER = "c11_4_grid_auto_fit_columns_wpt_expected"


def command(arguments: list[str], *, log_path: Path) -> None:
    result = subprocess.run(
        arguments,
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    log_path.write_text(result.stdout + result.stderr, encoding="utf-8")
    if result.returncode:
        raise RuntimeError(
            f"command failed ({result.returncode}): {' '.join(arguments)}; see {log_path}"
        )


def source_sha256() -> str:
    digest = hashlib.sha256()
    for relative in SOURCE_FILES:
        digest.update(relative.encode("utf-8"))
        digest.update((ROOT / relative).read_bytes())
    return digest.hexdigest()


def scan_archive(name: str) -> dict[str, object]:
    if not ARCHIVE.is_file():
        raise RuntimeError(f"missing static archive: {ARCHIVE}")
    sysroot = Path(
        subprocess.run(
            ["rustc", "--print", "sysroot"],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=True,
        ).stdout.strip()
    )
    host = next(
        line.removeprefix("host: ")
        for line in subprocess.run(
            ["rustc", "--version", "--verbose"],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=True,
        ).stdout.splitlines()
        if line.startswith("host: ")
    )
    llvm_nm = sysroot / "lib/rustlib" / host / "bin/llvm-nm"
    if not llvm_nm.is_file():
        raise RuntimeError(f"rustc-matched llvm-nm is missing: {llvm_nm}")
    symbols = subprocess.run(
        [str(llvm_nm), "-g", "--defined-only", str(ARCHIVE)],
        cwd=ROOT,
        text=True,
        capture_output=True,
        errors="replace",
        check=True,
    ).stdout
    symbol_lines = [
        line for line in symbols.splitlines() if FIXTURE_SYMBOL.search(line)
    ]
    (EVIDENCE / f"{name}-archive-symbols.txt").write_text(
        "\n".join(symbol_lines) + ("\n" if symbol_lines else ""), encoding="utf-8"
    )
    string_process = subprocess.Popen(
        ["strings", str(ARCHIVE)],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    assert string_process.stdout is not None
    scene_markers: set[str] = set()
    has_expected_marker = False
    for raw_line in string_process.stdout:
        line = raw_line.decode("utf-8", errors="replace")
        scene_markers.update(
            f"grid-auto-fit-columns-{match}" for match in SCENE_MARKER.findall(line)
        )
        has_expected_marker = has_expected_marker or EXPECTED_MARKER in line
    string_stderr = string_process.communicate()[1].decode("utf-8", errors="replace")
    if string_process.returncode:
        raise RuntimeError(f"strings failed for {ARCHIVE}: {string_stderr}")
    symbol_names = sorted({
        name
        for line in symbol_lines
        for name in FIXTURE_SYMBOL.findall(line)
    })
    scene_ids = sorted(scene_markers)
    return {
        "archiveSha256": hashlib.sha256(ARCHIVE.read_bytes()).hexdigest(),
        "archiveBytes": ARCHIVE.stat().st_size,
        "symbolTool": shlex.join([str(llvm_nm), "-g", "--defined-only"]),
        "stringTool": "strings (UTF-8 decode with replacement; streamed scan)",
        "fixtureSymbols": symbol_names,
        "sceneMarkerCount": len(scene_ids),
        "sceneMarkers": scene_ids,
        "expectedReportMarkerPresent": has_expected_marker,
        "symbolScan": f"{name}-archive-symbols.txt",
    }


def main() -> None:
    stamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    source_hash = source_sha256()
    configs = (
        ("product", ["c04-runtime-gpu"]),
        ("fixture", ["c04-runtime-gpu-fixture-test"]),
    )
    records: dict[str, object] = {}
    for name, features in configs:
        command_line = [
            "mise",
            "exec",
            "--",
            "cargo",
            "build",
            "--locked",
            "-p",
            "spinon-ffi",
            "--lib",
            "--no-default-features",
            "--features",
            ",".join(features),
        ]
        build_log = EVIDENCE / f"{name}-archive-build.log"
        command(command_line, log_path=build_log)
        records[name] = {
            "features": features,
            "buildLog": build_log.name,
            **scan_archive(name),
        }

    product = records["product"]
    fixture = records["fixture"]
    assert isinstance(product, dict) and isinstance(fixture, dict)
    product_ok = (
        product["fixtureSymbols"] == []
        and product["sceneMarkerCount"] == 0
        and not product["expectedReportMarkerPresent"]
    )
    fixture_ok = (
        set(fixture["fixtureSymbols"])
        == {
            "spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture",
            "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture",
        }
        and fixture["sceneMarkerCount"] == 35
        and fixture["expectedReportMarkerPresent"]
    )
    record = {
        "observedUtc": stamp,
        "branchHead": subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=True,
        ).stdout.strip(),
        "sourceManifestSha256": source_hash,
        "archive": str(ARCHIVE.relative_to(ROOT)),
        "product": product,
        "fixture": fixture,
        "productBoundaryPassed": product_ok,
        "fixturePositiveControlPassed": fixture_ok,
        "scope": "macOS ARM64 static FFI archive only; app bundles and release artifacts are not covered",
    }
    summary = EVIDENCE / "archive-boundary-summary.json"
    summary.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(record, indent=2))
    if not product_ok or not fixture_ok:
        raise SystemExit("static archive boundary did not match the expected fixture gate")


if __name__ == "__main__":
    main()
