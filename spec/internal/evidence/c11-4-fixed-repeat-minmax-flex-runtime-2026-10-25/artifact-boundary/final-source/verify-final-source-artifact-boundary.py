#!/usr/bin/env python3
"""Verify fixture exclusion in the final app-evidence source snapshot."""

import hashlib
import json
import subprocess
import uuid
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[6]
FINAL_SOURCE = Path(__file__).resolve().parent
APP_SIMULATOR_JSON = ROOT / (
    "spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/"
    "ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T234343852Z-a54267ba/"
    "simulator.json"
)
RUN = FINAL_SOURCE / "runs" / (
    "final-artifact-boundary-"
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
PAYLOAD_MARKERS = [
    "repeat(1, minmax(12px, 0.5fr)) repeat(auto-fill, minmax(24px, 0fr))",
]
MARKERS = SCENES + HOST_SYMBOLS + PAYLOAD_MARKERS


def output(command):
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def manifest_check(stage, app_metadata):
    input_files = app_metadata["sourceRevision"]["fixedRepeatMinmaxFlexInputFiles"]
    assert len(input_files) == 26, f"expected 26 app input hashes, got {len(input_files)}"
    rows = []
    for entry in input_files:
        path = ROOT / entry["path"]
        actual = sha256_bytes(path.read_bytes())
        rows.append({
            "path": entry["path"],
            "appEvidenceSha256": entry["sha256"],
            "currentSha256": actual,
            "matches": actual == entry["sha256"],
        })
    diff_sha256 = sha256_bytes(subprocess.check_output(
        ["git", "diff", "--binary", "HEAD"], cwd=ROOT,
    ))
    result = {
        "stage": stage,
        "simulatorJson": str(APP_SIMULATOR_JSON.relative_to(ROOT)),
        "appSha256": app_metadata["appSha256"],
        "appSourceCommit": app_metadata["sourceRevision"]["commit"],
        "currentSourceCommit": output(["git", "rev-parse", "HEAD"]),
        "appSourceTree": app_metadata["sourceRevision"]["tree"],
        "appTrackedDiffSha256": app_metadata["sourceRevision"]["trackedDiffSha256"],
        "currentTrackedDiffSha256": diff_sha256,
        "trackedDiffMatches": diff_sha256 == app_metadata["sourceRevision"]["trackedDiffSha256"],
        "inputFileCount": len(rows),
        "inputFileMatches": sum(row["matches"] for row in rows),
        "inputFiles": rows,
    }
    path = RUN / f"app-source-manifest-{stage}.json"
    path.write_text(json.dumps(result, indent=2) + "\n")
    assert all(row["matches"] for row in rows), f"app input hash mismatch during {stage}"
    assert result["appSourceCommit"] == result["currentSourceCommit"], "source commit differs from app run"
    assert result["trackedDiffMatches"], "tracked source diff differs from app run"
    return result


def toolchain_commands():
    sysroot = Path(output(["mise", "exec", "--", "rustc", "--print", "sysroot"]))
    host = next(
        line.split(":", 1)[1].strip()
        for line in output(["mise", "exec", "--", "rustc", "-vV"]).splitlines()
        if line.startswith("host:")
    )
    llvm_nm = sysroot / "lib/rustlib" / host / "bin/llvm-nm"
    strings = Path("/usr/bin/strings")
    assert llvm_nm.is_file(), f"Rust toolchain llvm-nm missing: {llvm_nm}"
    assert strings.is_file(), f"strings executable missing: {strings}"
    return [str(llvm_nm)], [str(strings)]


LLVM_NM, STRINGS = toolchain_commands()
RUN.mkdir(parents=True, exist_ok=False)
app_metadata = json.loads(APP_SIMULATOR_JSON.read_text())
before = manifest_check("before", app_metadata)
build_configs = {
    "product": "c04-runtime-gpu",
    "fixture-test": "c04-runtime-gpu-fixture-test",
}


def scan_build(label, feature):
    command = [
        "mise", "exec", "--", "cargo", "build", "--locked", "--release",
        "--no-default-features", "--target-dir", str(TARGET.relative_to(ROOT)),
        "-p", "spinon-ffi", "--lib", "--features", feature,
    ]
    build = subprocess.run(
        command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, check=False,
    )
    build_log = build.stdout.replace(str(ROOT), "<REPO>").replace(str(Path.home()), "<HOME>")
    (RUN / f"{label}-build.log").write_text(build_log.rstrip() + "\n")
    if build.returncode != 0:
        raise RuntimeError(f"{label} build failed; see {label}-build.log")
    if not ARCHIVE.is_file():
        raise RuntimeError(f"archive missing after {label} build")

    data = ARCHIVE.read_bytes()
    nm_command = [*LLVM_NM, "--demangle", "--defined-only", str(ARCHIVE)]
    nm = subprocess.run(nm_command, cwd=ROOT, capture_output=True, text=True, check=False)
    if nm.returncode != 0:
        raise RuntimeError(f"llvm-nm failed for {label}: {nm.stderr.strip()}")
    symbol_lines = [line for line in nm.stdout.splitlines() if any(symbol in line for symbol in HOST_SYMBOLS)]
    (RUN / f"{label}-matching-symbols.txt").write_text(
        "\n".join(symbol_lines) + ("\n" if symbol_lines else "")
    )
    (RUN / f"{label}-llvm-nm.log").write_text(
        "command: " + " ".join([*LLVM_NM, "--demangle", "--defined-only", "<ARCHIVE>"]) + "\n"
        + "version: " + output([*LLVM_NM, "--version"]).splitlines()[0] + "\n"
        + f"exitCode: {nm.returncode}\n"
        + ("stderr:\n" + nm.stderr if nm.stderr else "")
    )

    strings_command = [*STRINGS, "-a", str(ARCHIVE)]
    strings = subprocess.run(strings_command, cwd=ROOT, capture_output=True, check=False)
    if strings.returncode != 0:
        raise RuntimeError(
            f"strings failed for {label}: {strings.stderr.decode('utf-8', errors='replace').strip()}"
        )
    string_lines = strings.stdout.decode("utf-8", errors="replace").splitlines()
    string_matches = {
        marker: [line for line in string_lines if marker in line]
        for marker in MARKERS
    }
    (RUN / f"{label}-matching-strings.txt").write_text(
        "\n".join(
            f"{marker}: {matches[0]}" if matches else f"{marker}: <absent>"
            for marker, matches in string_matches.items()
        ) + "\n"
    )
    (RUN / f"{label}-strings-scan.log").write_text(
        "command: /usr/bin/strings -a <ARCHIVE>\n"
        + f"exitCode: {strings.returncode}\n"
        + f"lineCount: {len(string_lines)}\n"
        + f"scannedArchiveBytes: {len(data)}\n"
        + "scan: complete stdout consumed before marker filtering\n"
    )
    symbol_presence = {symbol: any(symbol in line for line in symbol_lines) for symbol in HOST_SYMBOLS}
    string_presence = {marker: bool(matches) for marker, matches in string_matches.items()}
    return {
        "feature": feature,
        "command": command,
        "buildLog": f"{RUN.relative_to(ROOT)}/{label}-build.log",
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(data),
        "archiveSha256": sha256_bytes(data),
        "llvmNmCommand": [*LLVM_NM, "--demangle", "--defined-only", "<ARCHIVE>"],
        "llvmNmLog": f"{RUN.relative_to(ROOT)}/{label}-llvm-nm.log",
        "matchingSymbolsLog": f"{RUN.relative_to(ROOT)}/{label}-matching-symbols.txt",
        "stringsCommand": [*STRINGS, "-a", "<ARCHIVE>"],
        "stringsLog": f"{RUN.relative_to(ROOT)}/{label}-strings-scan.log",
        "matchingStringsLog": f"{RUN.relative_to(ROOT)}/{label}-matching-strings.txt",
        "sceneMarkerPresence": {scene: string_presence[scene] for scene in SCENES},
        "fixtureHostStringPresence": {symbol: string_presence[symbol] for symbol in HOST_SYMBOLS},
        "fixtureHostDefinedSymbolPresence": symbol_presence,
        "payloadMarkerPresence": {marker: string_presence[marker] for marker in PAYLOAD_MARKERS},
        "stringsExitCode": strings.returncode,
        "llvmNmExitCode": nm.returncode,
    }


results = {label: scan_build(label, feature) for label, feature in build_configs.items()}
after = manifest_check("after", app_metadata)
assert before["inputFileMatches"] == 26 and after["inputFileMatches"] == 26
assert before["trackedDiffMatches"] and after["trackedDiffMatches"]
assert not any(results["product"]["sceneMarkerPresence"].values())
assert not any(results["product"]["fixtureHostStringPresence"].values())
assert not any(results["product"]["fixtureHostDefinedSymbolPresence"].values())
assert not any(results["product"]["payloadMarkerPresence"].values())
assert all(results["fixture-test"]["sceneMarkerPresence"].values())
assert all(results["fixture-test"]["fixtureHostStringPresence"].values())
assert all(results["fixture-test"]["fixtureHostDefinedSymbolPresence"].values())
assert all(results["fixture-test"]["payloadMarkerPresence"].values())

sysroot = Path(output(["mise", "exec", "--", "rustc", "--print", "sysroot"]))
host = output(["mise", "exec", "--", "rustc", "-vV"]).splitlines()
host_metadata = {
    "capturedUtc": datetime.now(timezone.utc).isoformat(),
    "os": output(["sw_vers"]).splitlines(),
    "kernelArchitecture": output(["uname", "-srm"]),
    "rustc": output(["mise", "exec", "--", "rustc", "--version", "--verbose"]).splitlines(),
    "cargo": output(["mise", "exec", "--", "cargo", "--version"]),
    "llvmNm": str(Path(LLVM_NM[0]).relative_to(sysroot)),
    "llvmNmVersion": output([*LLVM_NM, "--version"]).splitlines()[0],
    "strings": STRINGS[0],
    "stringsProvidedBy": output(["xcrun", "--find", "strings"]),
    "xcodebuildVersion": output(["xcodebuild", "-version"]).splitlines(),
}
(RUN / "host-toolchain.json").write_text(json.dumps(host_metadata, indent=2) + "\n")
report = {
    "schemaVersion": 1,
    "runDirectory": str(RUN.relative_to(ROOT)),
    "sourceSnapshotMatchesIosAppEvidence": True,
    "appEvidenceSourceManifestBefore": before,
    "appEvidenceSourceManifestAfter": after,
    "iosAppSha256": app_metadata["appSha256"],
    "iosSimulatorEvidence": str(APP_SIMULATOR_JSON.relative_to(ROOT)),
    "sceneIds": SCENES,
    "fixtureHostSymbols": HOST_SYMBOLS,
    "payloadMarkers": PAYLOAD_MARKERS,
    "hostToolchain": host_metadata,
    "targetDirectory": str(TARGET.relative_to(ROOT)),
    "results": results,
    "scope": "final-source Rust static archive boundary only; not final APK/IPA exclusion",
}
(RUN / "final-artifact-boundary.json").write_text(json.dumps(report, indent=2) + "\n")
(FINAL_SOURCE / "latest-run.json").write_text(json.dumps({
    "runDirectory": str(RUN.relative_to(ROOT)),
    "report": str((RUN / "final-artifact-boundary.json").relative_to(ROOT)),
    "iosSimulatorEvidence": str(APP_SIMULATOR_JSON.relative_to(ROOT)),
    "iosAppSha256": app_metadata["appSha256"],
}, indent=2) + "\n")
print(json.dumps({
    "runDirectory": str(RUN.relative_to(ROOT)),
    "appSourceInputs": f"{before['inputFileMatches']}/26 before, {after['inputFileMatches']}/26 after",
    "trackedDiffMatchesAppEvidence": before["trackedDiffMatches"] and after["trackedDiffMatches"],
    "iosAppSha256": app_metadata["appSha256"],
    "product": {
        "archiveBytes": results["product"]["archiveBytes"],
        "archiveSha256": results["product"]["archiveSha256"],
        "sceneMarkers": sum(results["product"]["sceneMarkerPresence"].values()),
        "hostSymbols": sum(results["product"]["fixtureHostDefinedSymbolPresence"].values()),
        "payloadMarkers": sum(results["product"]["payloadMarkerPresence"].values()),
    },
    "fixtureTest": {
        "archiveBytes": results["fixture-test"]["archiveBytes"],
        "archiveSha256": results["fixture-test"]["archiveSha256"],
        "sceneMarkers": sum(results["fixture-test"]["sceneMarkerPresence"].values()),
        "hostSymbols": sum(results["fixture-test"]["fixtureHostDefinedSymbolPresence"].values()),
        "payloadMarkers": sum(results["fixture-test"]["payloadMarkerPresence"].values()),
    },
}, indent=2))
