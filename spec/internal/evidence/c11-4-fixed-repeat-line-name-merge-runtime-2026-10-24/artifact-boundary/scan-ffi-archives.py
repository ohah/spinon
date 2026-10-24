#!/usr/bin/env python3
import hashlib
import json
import os
import shlex
import shutil
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
EVIDENCE = Path(__file__).resolve().parent
ARCHIVE = ROOT / "target/release/libspinon_ffi.a"
SCENE_FILE = ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_named_line_scenes.json"
SCENES = [
    scene["id"]
    for scene in json.loads(SCENE_FILE.read_text())
    if scene["id"].startswith("auto-repeat-named-fixed-repeat-")
]
FFI_SYMBOL = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
RUST_SYMBOL = "c11_4_auto_repeat_named_lines::runtime_source"


def llvm_nm_command():
    configured = os.environ.get("SPINON_LLVM_NM")
    if configured:
        return shlex.split(configured)
    candidates = sorted(Path("/opt/homebrew/Cellar/llvm@22").glob("*/bin/llvm-nm"))
    if candidates:
        return [str(candidates[-1])]
    found = shutil.which("llvm-nm")
    if found:
        return [found]
    return shlex.split(subprocess.check_output(["xcrun", "--find", "llvm-nm"], text=True).strip())


LLVM_NM = llvm_nm_command()
BUILD_CONFIGS = {
    "product": ["c04-runtime-gpu"],
    "fixtureTest": ["c04-runtime-gpu-fixture-test"],
}


def scan(label, features):
    command = [
        "mise",
        "exec",
        "--",
        "cargo",
        "build",
        "--locked",
        "--release",
        "--no-default-features",
        "-p",
        "spinon-ffi",
        "--lib",
        "--features",
        features[0],
    ]
    with (EVIDENCE / f"{label}-build.log").open("w") as output:
        subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT, check=True)

    data = ARCHIVE.read_bytes()
    marker_presence = {scene: scene.encode() in data for scene in SCENES}
    abi_text_present = FFI_SYMBOL.encode() in data
    nm = subprocess.run(
        [*LLVM_NM, "--demangle", "--defined-only", str(ARCHIVE)],
        capture_output=True,
        text=True,
        check=False,
    )
    matching = [
        line for line in nm.stdout.splitlines() if FFI_SYMBOL in line or RUST_SYMBOL in line
    ]
    (EVIDENCE / f"symbols-{label}.txt").write_text("\n".join(matching) + ("\n" if matching else ""))
    if nm.returncode != 0:
        raise RuntimeError(f"llvm-nm failed for {label}: {nm.stderr.strip()}")
    return {
        "command": command,
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(data),
        "archiveSha256": hashlib.sha256(data).hexdigest(),
        "sceneMarkerPresence": marker_presence,
        "allSceneMarkersPresent": all(marker_presence.values()),
        "anySceneMarkerPresent": any(marker_presence.values()),
        "fixtureAbiTextPresent": abi_text_present,
        "fixtureAbiDefinedSymbolPresent": any(FFI_SYMBOL in line for line in matching),
        "candidateRustRuntimeSourceSymbolPresent": any(RUST_SYMBOL in line for line in matching),
        "llvmNmExitCode": nm.returncode,
        "matchingDefinedSymbols": matching,
    }


results = {label: scan(label, features) for label, features in BUILD_CONFIGS.items()}
assert len(SCENES) == 3
assert not results["product"]["anySceneMarkerPresent"]
assert not results["product"]["fixtureAbiTextPresent"]
assert not results["product"]["fixtureAbiDefinedSymbolPresent"]
assert not results["product"]["candidateRustRuntimeSourceSymbolPresent"]
assert results["fixtureTest"]["allSceneMarkersPresent"]
assert results["fixtureTest"]["fixtureAbiTextPresent"]
assert results["fixtureTest"]["fixtureAbiDefinedSymbolPresent"]
assert results["fixtureTest"]["candidateRustRuntimeSourceSymbolPresent"]

staged_diff = subprocess.check_output(["git", "diff", "--cached", "--binary", "HEAD"], cwd=ROOT)
report = {
    "schemaVersion": 1,
    "sourceRevision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
    "stagedImplementationDiffSha256": hashlib.sha256(staged_diff).hexdigest(),
    "sceneIds": SCENES,
    "ffiFixtureSymbol": FFI_SYMBOL,
    "rustRuntimeSourceSymbolSubstring": RUST_SYMBOL,
    "llvmNmCommand": LLVM_NM,
    "llvmNmVersion": subprocess.check_output([*LLVM_NM, "--version"], text=True).splitlines()[0],
    "rustcVersion": subprocess.check_output(["rustc", "--version", "--verbose"], cwd=ROOT, text=True).strip(),
    "results": results,
}
(EVIDENCE / "artifact-boundary.json").write_text(json.dumps(report, indent=2) + "\n")
print(
    json.dumps(
        {
            label: {
                "archiveBytes": value["archiveBytes"],
                "archiveSha256": value["archiveSha256"],
                "sceneMarkers": sum(value["sceneMarkerPresence"].values()),
                "sceneCount": len(value["sceneMarkerPresence"]),
                "fixtureAbiDefinedSymbolPresent": value["fixtureAbiDefinedSymbolPresent"],
                "candidateRustRuntimeSourceSymbolPresent": value["candidateRustRuntimeSourceSymbolPresent"],
                "llvmNmExitCode": value["llvmNmExitCode"],
            }
            for label, value in results.items()
        },
        indent=2,
    )
)
