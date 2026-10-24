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
TARGET = ROOT / "build/spinon/c11-4-direct-mixed-artifact-boundary/target"
ARCHIVE = TARGET / "release/libspinon_ffi.a"
SCENES = [
    "direct-min-content-forward",
    "direct-min-content-reversed",
    "direct-max-content-forward",
    "direct-max-content-reversed",
]
FFI_SYMBOL = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
RUST_SYMBOL = "mixed_direct_text_image_intrinsic::runtime_source"


def llvm_nm_command():
    configured = os.environ.get("SPINON_LLVM_NM")
    if configured:
        return shlex.split(configured)
    sysroot = Path(subprocess.check_output(["mise", "exec", "--", "rustc", "--print", "sysroot"], text=True).strip())
    host = next(
        line.split(":", 1)[1].strip()
        for line in subprocess.check_output(["mise", "exec", "--", "rustc", "-vV"], text=True).splitlines()
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
BUILD_CONFIGS = {
    "product": ["c04-runtime-gpu"],
    "fixtureTest": ["c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test"],
}


def scan(label, features):
    command = [
        "mise", "exec", "--", "cargo", "build", "--locked", "--release",
        "--no-default-features", "--target-dir", str(TARGET.relative_to(ROOT)), "-p", "spinon-ffi",
        "--lib", "--features", features[0],
    ]
    build = subprocess.run(
        command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, check=False,
    )
    build_log = build.stdout.replace(str(ROOT), "<REPO>")
    if Path.home():
        build_log = build_log.replace(str(Path.home()), "<HOME>")
    build_log = "\n".join(line.rstrip(" \t") for line in build_log.splitlines()) + "\n"
    build_log_path = RUN / f"{label}-build.log"
    build_log_path.write_text(build_log)
    if build.returncode != 0:
        raise RuntimeError(f"cargo build failed for {label}; see {label}-build.log")

    data = ARCHIVE.read_bytes()
    markers = {scene: scene.encode() in data for scene in SCENES}
    abi_text_present = FFI_SYMBOL.encode() in data
    nm = subprocess.run(
        [*LLVM_NM, "--demangle", "--defined-only", str(ARCHIVE)],
        capture_output=True,
        text=True,
        check=False,
    )
    if nm.returncode != 0:
        raise RuntimeError(f"llvm-nm failed for {label}: {nm.stderr.strip()}")
    matching = [line for line in nm.stdout.splitlines() if FFI_SYMBOL in line or RUST_SYMBOL in line]
    symbols_path = RUN / f"symbols-{label}.txt"
    symbols_path.write_text("\n".join(matching) + ("\n" if matching else ""))
    return {
        "command": command,
        "buildLogPath": str(build_log_path.relative_to(ROOT)),
        "symbolsPath": str(symbols_path.relative_to(ROOT)),
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(data),
        "archiveSha256": hashlib.sha256(data).hexdigest(),
        "sceneMarkerPresence": markers,
        "allSceneMarkersPresent": all(markers.values()),
        "anySceneMarkerPresent": any(markers.values()),
        "fixtureAbiTextPresent": abi_text_present,
        "fixtureAbiDefinedSymbolPresent": any(FFI_SYMBOL in line for line in matching),
        "candidateRustRuntimeSourceSymbolPresent": any(RUST_SYMBOL in line for line in matching),
        "llvmNmExitCode": nm.returncode,
        "matchingDefinedSymbols": matching,
    }


results = {label: scan(label, features) for label, features in BUILD_CONFIGS.items()}
assert len(SCENES) == 4
assert not results["product"]["anySceneMarkerPresent"]
assert not results["product"]["fixtureAbiTextPresent"]
assert not results["product"]["fixtureAbiDefinedSymbolPresent"]
assert not results["product"]["candidateRustRuntimeSourceSymbolPresent"]
assert results["fixtureTest"]["allSceneMarkersPresent"]
assert results["fixtureTest"]["fixtureAbiTextPresent"]
assert results["fixtureTest"]["fixtureAbiDefinedSymbolPresent"]
assert results["fixtureTest"]["candidateRustRuntimeSourceSymbolPresent"]

generated_paths = ["artifact-boundary.json"]
evidence_relative = EVIDENCE.relative_to(ROOT)
generated_pathspecs = [f":(exclude){evidence_relative / path}" for path in generated_paths]
generated_pathspecs.append(
    f":(exclude,glob){evidence_relative / 'runs/**'}"
)
staged_diff = subprocess.check_output(
    ["git", "diff", "--cached", "--binary", "HEAD", "--", ".", *generated_pathspecs],
    cwd=ROOT,
)
sysroot = Path(subprocess.check_output(["mise", "exec", "--", "rustc", "--print", "sysroot"], text=True).strip())
try:
    llvm_nm_display_path = f"<rust-sysroot>/{Path(LLVM_NM[0]).relative_to(sysroot)}"
except ValueError:
    llvm_nm_display_path = Path(LLVM_NM[0]).name
report = {
    "schemaVersion": 1,
    "reportSemantics": "Last successful scan; a later failed attempt leaves this report unchanged.",
    "runDirectory": str(RUN.relative_to(ROOT)),
    "sourceRevision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
    "stagedDiffSha256": hashlib.sha256(staged_diff).hexdigest(),
    "stagedDiffExcludedGeneratedPaths": generated_paths + ["runs/**"],
    "sceneIds": SCENES,
    "ffiFixtureSymbol": FFI_SYMBOL,
    "rustRuntimeSourceSymbolSubstring": RUST_SYMBOL,
    "targetDirectory": str(TARGET.relative_to(ROOT)),
    "llvmNmCommand": [llvm_nm_display_path, *LLVM_NM[1:]],
    "llvmNmVersion": subprocess.check_output([*LLVM_NM, "--version"], text=True).splitlines()[0],
    "rustcVersion": subprocess.check_output(["mise", "exec", "--", "rustc", "--version", "--verbose"], cwd=ROOT, text=True).strip(),
    "results": results,
}
report_path = EVIDENCE / "artifact-boundary.json"
temporary_report = EVIDENCE / f".artifact-boundary-{uuid4().hex}.tmp"
temporary_report.write_text(json.dumps(report, indent=2) + "\n")
os.replace(temporary_report, report_path)
print(json.dumps({
    label: {
        "archiveBytes": value["archiveBytes"],
        "archiveSha256": value["archiveSha256"],
        "sceneMarkers": sum(value["sceneMarkerPresence"].values()),
        "sceneCount": len(value["sceneMarkerPresence"]),
        "fixtureAbiDefinedSymbolPresent": value["fixtureAbiDefinedSymbolPresent"],
        "candidateRustRuntimeSourceSymbolPresent": value["candidateRustRuntimeSourceSymbolPresent"],
    }
    for label, value in results.items()
}, indent=2))
