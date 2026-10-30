#!/usr/bin/env python3
"""Verify C11.4 aspect-ratio min/max fixture isolation in macOS ARM64 archives."""

import hashlib
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from uuid import uuid4


SCRIPT = Path(__file__).resolve()
EVIDENCE = SCRIPT.parent
ROOT = next(parent for parent in SCRIPT.parents
            if (parent / "Cargo.toml").is_file() and (parent / "spec").is_dir())
RUN = EVIDENCE / "minmax-runs" / (
    datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid4().hex[:8]
)
TARGET = ROOT / "build/spinon/c11-4-direct-text-aspect-ratio-min-max-archive-target"
ARCHIVE = TARGET / "release/libspinon_ffi.a"
SCENE_MARKERS = [
    "numeric-baseline",
    "min-width-60",
    "min-width-45",
    "max-width-30-min-width-0",
    "min-height-30",
    "max-height-15-min-height-0",
    "max-height-15-min-height-0-min-width-0",
    "max-height-17-75-min-controls",
    "min-width-60-clamped-by-max-height-20",
    "min-height-20-clamps-transferred-max-width-30",
    "source-min-width-60-over-max-width-40",
    "source-min-height-30-over-max-height-20",
    "auto-ratio-min-width-60",
    "no-ratio-min-width-60",
    "no-ratio-min-height-30",
    "numeric-border-box-min-width-60",
    "auto-content-box-min-width-60",
    "numeric-border-box-max-height-15-min-controls",
    "auto-content-box-max-height-15-min-controls",
]
ABI_SYMBOL = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
SOURCES = [
    ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_text_aspect_ratio_minmax.rs",
    ROOT / "crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs",
]


def run(args: list[str], *, timeout: int = 1800, allow_failure: bool = False):
    result = subprocess.run(args, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=timeout, check=False)
    if result.returncode and not allow_failure:
        raise RuntimeError(f"command failed ({result.returncode}): {' '.join(args)}")
    return result


def defined_symbols(archive: Path, label: str) -> tuple[list[str], str, str, str]:
    sysroot = Path(run(["mise", "exec", "--", "rustc", "--print", "sysroot"]).stdout.strip())
    host = next(line.split(":", 1)[1].strip() for line in
                run(["mise", "exec", "--", "rustc", "-vV"]).stdout.splitlines()
                if line.startswith("host:"))
    tool = sysroot / "lib/rustlib" / host / "bin/llvm-nm"
    result = run([tool, "--demangle", "--defined-only", str(archive)])
    diagnostics = result.stderr or ""
    (RUN / f"{label}-llvm-nm-stderr.log").write_text(diagnostics)
    version = run([tool, "--version"]).stdout.splitlines()
    return result.stdout.splitlines(), "\n".join(version), str(tool), diagnostics


def build_and_scan(label: str, features: str) -> dict:
    target_relative = str(TARGET.relative_to(ROOT))
    command = ["mise", "exec", "--", "cargo", "build", "--locked", "--release",
               "--no-default-features", "--target-dir", target_relative,
               "-p", "spinon-ffi", "--lib", "--features", features]
    build = run(command)
    (RUN / f"{label}-build.log").write_text(build.stdout)
    archive_bytes = ARCHIVE.read_bytes()
    markers = {scene: scene.encode() in archive_bytes for scene in SCENE_MARKERS}
    symbol_lines, nm_version, nm_path, nm_stderr = defined_symbols(ARCHIVE, label)
    symbol_matches = [line for line in symbol_lines if ABI_SYMBOL in line]
    (RUN / f"{label}-fixture-symbols.txt").write_text(
        "\n".join(symbol_matches) + ("\n" if symbol_matches else ""))
    return {
        "features": features,
        "command": command,
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(archive_bytes),
        "archiveSha256": hashlib.sha256(archive_bytes).hexdigest(),
        "sceneMarkers": markers,
        "sceneMarkersPresent": sum(markers.values()),
        "sceneMarkerCount": len(markers),
        "fixtureAbiTextPresent": ABI_SYMBOL.encode() in archive_bytes,
        "fixtureAbiDefinedSymbolPresent": bool(symbol_matches),
        "fixtureSymbolMatchesPath": str((RUN / f"{label}-fixture-symbols.txt").relative_to(ROOT)),
        "buildLogPath": str((RUN / f"{label}-build.log").relative_to(ROOT)),
        "llvmNmPath": nm_path,
        "llvmNmVersion": nm_version,
        "llvmNmDiagnostics": nm_stderr.strip().splitlines(),
    }


def main() -> None:
    RUN.mkdir(parents=True, exist_ok=False)
    before = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
              for path in SOURCES}
    product = build_and_scan("product", "c04-runtime-gpu")
    fixture = build_and_scan(
        "fixture-test", "c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test")
    after = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
             for path in SOURCES}
    if before != after:
        raise AssertionError("archive build changed a Rust source input")
    if product["sceneMarkersPresent"] or product["fixtureAbiTextPresent"] \
            or product["fixtureAbiDefinedSymbolPresent"]:
        raise AssertionError("default c04-runtime-gpu archive includes C11.4 fixture payload")
    if fixture["sceneMarkersPresent"] != len(SCENE_MARKERS) \
            or not fixture["fixtureAbiTextPresent"] \
            or not fixture["fixtureAbiDefinedSymbolPresent"]:
        raise AssertionError("fixture-test positive control is missing scene markers or ABI")
    report = {
        "schemaVersion": 1,
        "scope": "macOS ARM64 spinon-ffi release static archive only",
        "createdAtUtc": datetime.now(timezone.utc).isoformat(),
        "sourceSha256BeforeAndAfter": before,
        "sceneMarkers": SCENE_MARKERS,
        "fixtureAbiSymbol": ABI_SYMBOL,
        "targetDirectory": str(TARGET.relative_to(ROOT)),
        "runDirectory": str(RUN.relative_to(ROOT)),
        "results": {"product": product, "fixtureTest": fixture},
        "limitations": [
            "This checks only scene marker bytes and one fixture ABI symbol in spinon-ffi.a.",
            "It does not establish fixture exclusion from the final Android APK or iOS app bundle.",
            "It does not prove scene geometry, simulator behavior, WPT conformance, or C11 completion.",
        ],
    }
    report_path = EVIDENCE / "minmax-archive-boundary.json"
    report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({
        "runDirectory": str(RUN.relative_to(ROOT)),
        "report": str(report_path.relative_to(ROOT)),
        "product": {key: product[key] for key in (
            "archiveBytes", "archiveSha256", "sceneMarkersPresent", "sceneMarkerCount",
            "fixtureAbiDefinedSymbolPresent")},
        "fixtureTest": {key: fixture[key] for key in (
            "archiveBytes", "archiveSha256", "sceneMarkersPresent", "sceneMarkerCount",
            "fixtureAbiDefinedSymbolPresent")},
    }, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
