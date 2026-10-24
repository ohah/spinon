#!/usr/bin/env python3
import hashlib
import json
import os
import shlex
import subprocess
import sys
from pathlib import Path


root = Path(__file__).resolve().parents[4]
evidence = Path(__file__).resolve().parent
scene_path = root / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_percentage_neighbor_scenes.json"
scenes = json.loads(scene_path.read_text())
markers = [scene["id"] for scene in scenes]
ffi_symbol = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
module_symbol = "c11_4_auto_repeat_negative_percentage_neighbors"
archives = {"product": Path(sys.argv[1]), "fixtureTest": Path(sys.argv[2])}
source_manifest_path = root / "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/source-manifest.json"
source_manifest = json.loads(source_manifest_path.read_text())
llvm_nm = shlex.split(os.environ.get("SPINON_LLVM_NM", "xcrun llvm-nm"))
results = {}

for label, archive in archives.items():
    data = archive.read_bytes()
    present = {marker: marker.encode() in data for marker in markers}
    proc = subprocess.Popen(
        [*llvm_nm, "--demangle", "--defined-only", str(archive)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    matching = []
    assert proc.stdout is not None
    for line in proc.stdout:
        if ffi_symbol in line or module_symbol in line:
            matching.append(line.rstrip())
    stderr = proc.stderr.read() if proc.stderr else ""
    code = proc.wait()
    (evidence / f"symbols-{label}-llvm22.txt").write_text(
        "\n".join(matching) + ("\n" if matching else "")
    )
    results[label] = {
        "path": str(archive),
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
        "sceneMarkerPresence": present,
        "allSceneMarkersPresent": all(present.values()),
        "anySceneMarkerPresent": any(present.values()),
        "ffiFixtureSymbolPresent": any(ffi_symbol in line for line in matching),
        "candidateModuleSymbolRows": [line for line in matching if module_symbol in line],
        "llvmNmExitCode": code,
        "llvmNmStderr": stderr.strip(),
    }

assert not results["product"]["anySceneMarkerPresent"]
assert not results["product"]["ffiFixtureSymbolPresent"]
assert not results["product"]["candidateModuleSymbolRows"]
assert results["fixtureTest"]["allSceneMarkersPresent"]
assert results["fixtureTest"]["ffiFixtureSymbolPresent"]
assert results["fixtureTest"]["candidateModuleSymbolRows"]

report = {
    "llvmNmExecutable": llvm_nm,
    "sourceRevision": source_manifest["sourceRevision"]["commit"],
    "sourceDiffSha256": source_manifest["sourceDiffSha256"],
    "sourceManifest": {
        "path": "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/source-manifest.json",
        "sha256": hashlib.sha256(
            source_manifest_path.read_bytes()
        ).hexdigest(),
    },
    "sceneCount": len(markers),
    "sceneIds": markers,
    "ffiFixtureSymbol": ffi_symbol,
    "candidateModuleSymbol": module_symbol,
    "results": results,
}
(evidence / "scan-results-llvm22.json").write_text(json.dumps(report, indent=2) + "\n")
print(
    json.dumps(
        {
            label: {
                "bytes": item["bytes"],
                "sha256": item["sha256"],
                "sceneMarkers": sum(item["sceneMarkerPresence"].values()),
                "sceneCount": len(item["sceneMarkerPresence"]),
                "ffiFixtureSymbolPresent": item["ffiFixtureSymbolPresent"],
                "candidateModuleSymbolRows": len(item["candidateModuleSymbolRows"]),
                "llvmNmExitCode": item["llvmNmExitCode"],
            }
            for label, item in results.items()
        },
        indent=2,
    )
)
