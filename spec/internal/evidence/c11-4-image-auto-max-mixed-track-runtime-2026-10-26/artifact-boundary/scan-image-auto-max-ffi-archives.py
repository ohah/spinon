#!/usr/bin/env python3
import hashlib
import json
import os
import shlex
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from uuid import uuid4

ROOT = Path(__file__).resolve().parents[5]
EVIDENCE = Path(__file__).resolve().parent
RUN = EVIDENCE / "runs" / (
    "artifact-boundary-"
    + datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    + "-"
    + uuid4().hex[:8]
)
TARGET = ROOT / "build/spinon/c11-4-image-auto-max-artifact-boundary/target"
ARCHIVE = TARGET / "release/libspinon_ffi.a"
ARCHIVE_COPIES = TARGET / "artifact-boundary-copies"
SCENE_MARKERS = [
    "image-auto-max-mixed-tracks",
    "c114-image-auto-max-mixed-grid",
    "c114-image-auto-max-mixed-item",
    "minmax(auto, max-content) 17px",
    "https://spinon.invalid/c14-1-png-basic.png",
]
FFI_SYMBOLS = [
    "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture",
    "spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture",
]
SHARED_PNG = ROOT / "tests/fixtures/resources/c14-1-png-basic.png"


def llvm_nm_command():
    configured = os.environ.get("SPINON_LLVM_NM")
    if configured:
        return shlex.split(configured)
    sysroot = Path(subprocess.check_output(
        ["mise", "exec", "--", "rustc", "--print", "sysroot"],
        cwd=ROOT, text=True,
    ).strip())
    host = next(
        line.split(":", 1)[1].strip()
        for line in subprocess.check_output(
            ["mise", "exec", "--", "rustc", "-vV"], cwd=ROOT, text=True
        ).splitlines()
        if line.startswith("host:")
    )
    toolchain_nm = sysroot / "lib/rustlib" / host / "bin/llvm-nm"
    if toolchain_nm.is_file():
        return [str(toolchain_nm)]
    found = shutil.which("llvm-nm")
    if found:
        return [found]
    return shlex.split(subprocess.check_output(["xcrun", "--find", "llvm-nm"], text=True).strip())


LLVM_NM = llvm_nm_command()
RUN.mkdir(parents=True, exist_ok=False)
ARCHIVE_COPIES.mkdir(parents=True, exist_ok=True)
shared_png_bytes = SHARED_PNG.read_bytes()
BUILD_CONFIGS = {
    "product": "c04-runtime-gpu",
    "fixtureTest": "c04-runtime-gpu-fixture-test",
}


def build_and_scan(label, features):
    command = [
        "mise", "exec", "--", "cargo", "build", "--locked", "--release",
        "--no-default-features", "--target-dir",
        str(TARGET.relative_to(ROOT)), "-p", "spinon-ffi", "--lib",
        "--features", features,
    ]
    build = subprocess.run(
        command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, check=False,
    )
    build_log = build.stdout.replace(str(ROOT), "<REPO>")
    build_log = build_log.replace(str(Path.home()), "<HOME>")
    build_log = "\n".join(line.rstrip(" \t") for line in build_log.splitlines()) + "\n"
    build_log_path = RUN / f"{label}-build.log"
    build_log_path.write_text(build_log)
    if build.returncode != 0:
        raise RuntimeError(f"cargo build failed for {label}; see {build_log_path}")
    if not ARCHIVE.is_file():
        raise RuntimeError(f"static archive missing after {label} build: {ARCHIVE}")

    archive_copy = ARCHIVE_COPIES / f"libspinon_ffi-{label}.a"
    shutil.copyfile(ARCHIVE, archive_copy)
    data = archive_copy.read_bytes()
    marker_presence = {marker: marker.encode() in data for marker in SCENE_MARKERS}
    abi_text_presence = {symbol: symbol.encode() in data for symbol in FFI_SYMBOLS}

    nm = subprocess.run(
        [*LLVM_NM, "--demangle", "--defined-only", str(archive_copy)],
        cwd=ROOT, capture_output=True, text=True, check=False,
    )
    if nm.returncode != 0:
        raise RuntimeError(f"llvm-nm failed for {label}: {nm.stderr.strip()}")
    matching_symbols = [
        line for line in nm.stdout.splitlines()
        if any(symbol in line for symbol in FFI_SYMBOLS)
    ]
    symbol_path = RUN / f"fixture-ffi-symbols-{label}.txt"
    symbol_path.write_text("\n".join(matching_symbols) + ("\n" if matching_symbols else ""))

    strings = subprocess.Popen(
        ["strings", str(archive_copy)], cwd=ROOT,
        stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
    )
    matching_strings = []
    assert strings.stdout is not None
    for raw_line in strings.stdout:
        line = raw_line.decode("utf-8", errors="replace").rstrip("\r\n")
        if any(marker in line for marker in SCENE_MARKERS + FFI_SYMBOLS):
            matching_strings.append(line)
    strings_returncode = strings.wait()
    if strings_returncode != 0:
        raise RuntimeError(f"strings failed for {label}: exit {strings_returncode}")
    string_path = RUN / f"fixture-markers-{label}.txt"
    string_path.write_text("\n".join(matching_strings) + ("\n" if matching_strings else ""))

    result = {
        "features": features,
        "command": command,
        "buildLogPath": str(build_log_path.relative_to(ROOT)),
        "archivePath": str(archive_copy.relative_to(ROOT)),
        "archiveBytes": len(data),
        "archiveSha256": hashlib.sha256(data).hexdigest(),
        "sceneMarkerPresence": marker_presence,
        "allSceneMarkersPresent": all(marker_presence.values()),
        "anySceneMarkerPresent": any(marker_presence.values()),
        "fixtureAbiTextPresence": abi_text_presence,
        "anyFixtureAbiTextPresent": any(abi_text_presence.values()),
        "fixtureAbiDefinedSymbols": matching_symbols,
        "fixtureAbiDefinedSymbolPresence": {
            symbol: any(symbol in line for line in matching_symbols)
            for symbol in FFI_SYMBOLS
        },
        "allFixtureAbiSymbolsDefined": all(
            any(symbol in line for line in matching_symbols)
            for symbol in FFI_SYMBOLS
        ),
        "sharedImageFixtureAsset": {
            "path": str(SHARED_PNG.relative_to(ROOT)),
            "bytes": len(shared_png_bytes),
            "sha256": hashlib.sha256(shared_png_bytes).hexdigest(),
            "exactBytesPresent": shared_png_bytes in data,
        },
        "stringsMatchesPath": str(string_path.relative_to(ROOT)),
        "llvmNmMatchesPath": str(symbol_path.relative_to(ROOT)),
    }
    return result


results = {
    label: build_and_scan(label, features)
    for label, features in BUILD_CONFIGS.items()
}
product = results["product"]
fixture = results["fixtureTest"]
assert not product["anySceneMarkerPresent"]
assert not product["anyFixtureAbiTextPresent"]
assert not product["fixtureAbiDefinedSymbols"]
assert fixture["allSceneMarkersPresent"]
assert fixture["allFixtureAbiSymbolsDefined"]

sysroot = Path(subprocess.check_output(
    ["mise", "exec", "--", "rustc", "--print", "sysroot"], cwd=ROOT, text=True
).strip())
try:
    nm_display = f"<rust-sysroot>/{Path(LLVM_NM[0]).relative_to(sysroot)}"
except ValueError:
    nm_display = Path(LLVM_NM[0]).name
report = {
    "schemaVersion": 1,
    "sourceRevision": subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip(),
    "runDirectory": str(RUN.relative_to(ROOT)),
    "targetDirectory": str(TARGET.relative_to(ROOT)),
    "llvmNmCommand": [nm_display, *LLVM_NM[1:]],
    "llvmNmVersion": subprocess.check_output(
        [*LLVM_NM, "--version"], cwd=ROOT, text=True
    ).splitlines()[0],
    "rustcVersion": subprocess.check_output(
        ["mise", "exec", "--", "rustc", "--version", "--verbose"],
        cwd=ROOT, text=True,
    ).strip(),
    "sceneMarkers": SCENE_MARKERS,
    "fixtureAbiSymbols": FFI_SYMBOLS,
    "sharedImageFixtureAssetNote": (
        "The PNG is a shared pre-existing fixture resource referenced by other opt-in scenes; "
        "its archive presence does not attribute it uniquely to this new scene."
    ),
    "results": results,
    "productNegativeControlPassed": (
        not product["anySceneMarkerPresent"]
        and not product["anyFixtureAbiTextPresent"]
        and not product["fixtureAbiDefinedSymbols"]
    ),
    "fixturePositiveControlPassed": (
        fixture["allSceneMarkersPresent"]
        and fixture["allFixtureAbiSymbolsDefined"]
    ),
    "scopeNote": "This checks the Rust spinon-ffi release static archive only, not complete APK or IPA exclusion.",
}
report_path = EVIDENCE / "artifact-boundary.json"
temporary_report = EVIDENCE / f".artifact-boundary-{uuid4().hex}.tmp"
temporary_report.write_text(json.dumps(report, indent=2) + "\n")
os.replace(temporary_report, report_path)
print(json.dumps({
    label: {
        "archiveBytes": result["archiveBytes"],
        "archiveSha256": result["archiveSha256"],
        "markers": f"{sum(result['sceneMarkerPresence'].values())}/{len(SCENE_MARKERS)}",
        "fixtureSymbols": f"{sum(result['fixtureAbiDefinedSymbolPresence'].values())}/{len(FFI_SYMBOLS)}",
        "sharedPngBytesPresent": result["sharedImageFixtureAsset"]["exactBytesPresent"],
    }
    for label, result in results.items()
}, indent=2))
