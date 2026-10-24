#!/usr/bin/env python3
"""Build and compare the product and fixture-test spinon-ffi archives."""

import hashlib
import json
import subprocess
import uuid
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
EVIDENCE = Path(__file__).resolve().parent
RUN = EVIDENCE / "runs" / (
    "artifact-boundary-"
    + datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    + "-"
    + uuid.uuid4().hex[:8]
)
TARGET = ROOT / "build/spinon/c11-4-fixed-repeat-minmax-flex-artifact-boundary/target"
ARCHIVE = TARGET / "release/libspinon_ffi.a"
SCENES = [
    "fixed-repeat-minmax-flex-columns-fill",
    "fixed-repeat-minmax-flex-columns-fit",
    "fixed-repeat-minmax-flex-rows-fill",
    "fixed-repeat-minmax-flex-rows-fit",
    "fixed-repeat-minmax-flex-columns-groups",
    "fixed-repeat-minmax-flex-columns-flex-factor-low",
]
HOST_SYMBOLS = [
    "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture",
    "spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture",
]
RUST_SYMBOL = "fixed_repeat_minmax_flex::runtime_source"
SOURCE_INPUTS = [
    "crates/spinon-ffi/Cargo.toml",
    "crates/spinon-ffi/src/runtime_gpu.rs",
    "crates/spinon-ffi/src/runtime_gpu/ffi.rs",
    "crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_minmax_flex.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_minmax_flex_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs",
    "Cargo.lock",
]


def output(command, cwd=ROOT):
    return subprocess.check_output(command, cwd=cwd, text=True).strip()


def llvm_nm_command():
    sysroot = Path(output(["mise", "exec", "--", "rustc", "--print", "sysroot"]))
    host = next(
        line.split(":", 1)[1].strip()
        for line in output(["mise", "exec", "--", "rustc", "-vV"]).splitlines()
        if line.startswith("host:")
    )
    return [str(sysroot / "lib/rustlib" / host / "bin/llvm-nm")]


LLVM_NM = llvm_nm_command()
RUN.mkdir(parents=True, exist_ok=False)
BUILD_CONFIGS = {
    "product": "c04-runtime-gpu",
    "fixture-test": "c04-runtime-gpu-fixture-test",
}


def run_build(label, feature):
    command = [
        "mise", "exec", "--", "cargo", "build", "--locked", "--release",
        "--no-default-features", "--target-dir", str(TARGET.relative_to(ROOT)),
        "-p", "spinon-ffi", "--lib", "--features", feature,
    ]
    build = subprocess.run(
        command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, check=False,
    )
    log = build.stdout.replace(str(ROOT), "<REPO>")
    if Path.home():
        log = log.replace(str(Path.home()), "<HOME>")
    (RUN / f"{label}-build.log").write_text(log.rstrip() + "\n")
    if build.returncode != 0:
        raise RuntimeError(f"cargo build failed for {label}; see {label}-build.log")
    if not ARCHIVE.is_file():
        raise RuntimeError(f"archive missing after {label} build: {ARCHIVE}")

    data = ARCHIVE.read_bytes()
    marker_presence = {scene: scene.encode() in data for scene in SCENES}
    host_text_presence = {symbol: symbol.encode() in data for symbol in HOST_SYMBOLS}
    nm = subprocess.run(
        [*LLVM_NM, "--demangle", "--defined-only", str(ARCHIVE)],
        capture_output=True, text=True, check=False,
    )
    if nm.returncode != 0:
        raise RuntimeError(f"llvm-nm failed for {label}; see {label}-llvm-nm.log")
    matches = [
        line for line in nm.stdout.splitlines()
        if any(symbol in line for symbol in HOST_SYMBOLS) or RUST_SYMBOL in line
    ]
    (RUN / f"{label}-matching-symbols.txt").write_text(
        "\n".join(matches) + ("\n" if matches else "")
    )
    (RUN / f"{label}-llvm-nm.log").write_text(
        "command: " + " ".join([*LLVM_NM, "--demangle", "--defined-only", "<ARCHIVE>"]) + "\n"
        + "version: " + output([*LLVM_NM, "--version"]).splitlines()[0] + "\n"
        + f"exitCode: {nm.returncode}\n"
        + ("stderr:\n" + nm.stderr if nm.stderr else "")
    )
    return {
        "feature": feature,
        "command": command,
        "buildLog": str((RUN / f"{label}-build.log").relative_to(ROOT)),
        "llvmNmLog": str((RUN / f"{label}-llvm-nm.log").relative_to(ROOT)),
        "matchingSymbolsLog": str((RUN / f"{label}-matching-symbols.txt").relative_to(ROOT)),
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(data),
        "archiveSha256": hashlib.sha256(data).hexdigest(),
        "sceneMarkerPresence": marker_presence,
        "fixtureHostTextPresence": host_text_presence,
        "fixtureHostDefinedSymbolPresence": {
            symbol: any(symbol in line for line in matches) for symbol in HOST_SYMBOLS
        },
        "runtimeSourceDefinedSymbolPresent": any(RUST_SYMBOL in line for line in matches),
        "llvmNmExitCode": nm.returncode,
        "matchingDefinedSymbols": matches,
    }


results = {}
for label, feature in BUILD_CONFIGS.items():
    results[label] = run_build(label, feature)

product = results["product"]
fixture = results["fixture-test"]
assert not any(product["sceneMarkerPresence"].values()), product
assert not any(product["fixtureHostTextPresence"].values()), product
assert not any(product["fixtureHostDefinedSymbolPresence"].values()), product
assert not product["runtimeSourceDefinedSymbolPresent"], product
assert all(fixture["sceneMarkerPresence"].values()), fixture
assert all(fixture["fixtureHostTextPresence"].values()), fixture
assert all(fixture["fixtureHostDefinedSymbolPresence"].values()), fixture
assert fixture["runtimeSourceDefinedSymbolPresent"], fixture

source_hashes = {}
for relative in SOURCE_INPUTS:
    source_hashes[relative] = hashlib.sha256((ROOT / relative).read_bytes()).hexdigest()

sysroot = Path(output(["mise", "exec", "--", "rustc", "--print", "sysroot"]))
llvm_nm_display = f"<rust-sysroot>/{Path(LLVM_NM[0]).relative_to(sysroot)}"
report = {
    "schemaVersion": 1,
    "sourceRevision": output(["git", "rev-parse", "HEAD"]),
    "host": output(["mise", "exec", "--", "rustc", "-vV"]).split("\n"),
    "cargoVersion": output(["mise", "exec", "--", "cargo", "--version"]),
    "llvmNm": llvm_nm_display,
    "llvmNmVersion": output([*LLVM_NM, "--version"]).splitlines()[0],
    "targetDirectory": str(TARGET.relative_to(ROOT)),
    "sceneIds": SCENES,
    "fixtureHostSymbols": HOST_SYMBOLS,
    "rustRuntimeSourceSymbolSubstring": RUST_SYMBOL,
    "sourceInputSha256": source_hashes,
    "runDirectory": str(RUN.relative_to(ROOT)),
    "results": results,
    "scope": "Rust spinon-ffi static archives only; no APK/IPA exclusion claim",
}
(RUN / "artifact-boundary.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({
    label: {
        "archiveBytes": result["archiveBytes"],
        "archiveSha256": result["archiveSha256"],
        "sceneMarkers": sum(result["sceneMarkerPresence"].values()),
        "fixtureHostSymbols": sum(result["fixtureHostDefinedSymbolPresence"].values()),
        "runtimeSourceDefinedSymbolPresent": result["runtimeSourceDefinedSymbolPresent"],
    }
    for label, result in results.items()
}, indent=2))
