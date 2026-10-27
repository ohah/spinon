#!/usr/bin/env python3
"""Scan opt-in C11.4 fixture strings/symbols in macOS ARM64 FFI archives."""

import hashlib
import json
import os
import shlex
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from uuid import uuid4


SCRIPT = Path(__file__).resolve()
EVIDENCE = SCRIPT.parent.parent
ROOT = next(parent for parent in SCRIPT.parents
            if (parent / "Cargo.toml").is_file() and (parent / "spec").is_dir())
RUN = EVIDENCE / "artifact-boundary" / "runs" / (
    "flex-stage-archive-" + datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    + "-" + uuid4().hex[:8]
)
TARGET = ROOT / "build/spinon/c11-4-flex-stage-runtime-archive-target"
ARCHIVE = TARGET / "release/libspinon_ffi.a"
SCENES = ["weighted-columns", "frozen-row", "subunit-columns", "zero-fr-row"]
FFI_SYMBOL = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
RUST_SYMBOL = "flex_track_expansion::runtime_source"
SOURCE_FILES = [
    ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    ROOT / "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/flex_track_expansion.rs",
]


def command(args: list[str], *, timeout: int = 1800, allow_failure: bool = False):
    result = subprocess.run(args, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=timeout, check=False)
    if result.returncode and not allow_failure:
        raise RuntimeError(f"command failed ({result.returncode}): {' '.join(args)}")
    return result


def llvm_nm() -> list[str]:
    configured = os.environ.get("SPINON_LLVM_NM")
    if configured:
        return shlex.split(configured)
    sysroot = Path(command(["mise", "exec", "--", "rustc", "--print", "sysroot"]).stdout.strip())
    host = next(line.split(":", 1)[1].strip() for line in command(
        ["mise", "exec", "--", "rustc", "-vV"]).stdout.splitlines() if line.startswith("host:"))
    toolchain = sysroot / "lib/rustlib" / host / "bin/llvm-nm"
    if toolchain.is_file():
        return [str(toolchain)]
    found = shutil.which("llvm-nm")
    if found:
        return [found]
    return [command(["xcrun", "--find", "llvm-nm"]).stdout.strip()]


def build_and_scan(label: str, features: str, nm: list[str]) -> dict:
    build_command = ["mise", "exec", "--", "cargo", "build", "--locked", "--release",
                     "--no-default-features", "--target-dir", str(TARGET.relative_to(ROOT)),
                     "-p", "spinon-ffi", "--lib", "--features", features]
    build = subprocess.run(build_command, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                           stderr=subprocess.STDOUT, timeout=1800, check=False)
    build_log = "\n".join(line.rstrip(" \t") for line in build.stdout.splitlines()) + "\n"
    (RUN / f"{label}-build.log").write_text(build_log)
    if build.returncode:
        raise RuntimeError(f"cargo build failed for {label}; see {RUN / f'{label}-build.log'}")
    archive = ARCHIVE.read_bytes()
    marker_presence = {scene: scene.encode() in archive for scene in SCENES}
    abi_text = FFI_SYMBOL.encode() in archive
    symbols = command([*nm, "--demangle", "--defined-only", str(ARCHIVE)])
    matches = [line for line in symbols.stdout.splitlines()
               if FFI_SYMBOL in line or RUST_SYMBOL in line]
    (RUN / f"{label}-symbols.txt").write_text("\n".join(matches) + ("\n" if matches else ""))
    return {
        "features": features,
        "command": build_command,
        "buildLogPath": str((RUN / f"{label}-build.log").relative_to(ROOT)),
        "archivePath": str(ARCHIVE.relative_to(ROOT)),
        "archiveBytes": len(archive),
        "archiveSha256": hashlib.sha256(archive).hexdigest(),
        "sceneMarkerPresence": marker_presence,
        "fixtureAbiTextPresent": abi_text,
        "fixtureAbiDefinedSymbolPresent": any(FFI_SYMBOL in line for line in matches),
        "candidateRustRuntimeSourceSymbolPresent": any(RUST_SYMBOL in line for line in matches),
        "symbolsPath": str((RUN / f"{label}-symbols.txt").relative_to(ROOT)),
        "matchingDefinedSymbols": matches,
        "llvmNmVersion": command([*nm, "--version"]).stdout.splitlines()[0],
    }


def main() -> None:
    RUN.mkdir(parents=True, exist_ok=False)
    before = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
              for path in SOURCE_FILES}
    nm = llvm_nm()
    product = build_and_scan("product", "c04-runtime-gpu", nm)
    fixture = build_and_scan(
        "fixture-test", "c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test", nm)
    after = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
             for path in SOURCE_FILES}
    if before != after:
        raise AssertionError("archive build 도중 Rust 입력이 바뀌었습니다.")
    if any(product["sceneMarkerPresence"].values()) or product["fixtureAbiTextPresent"] \
            or product["fixtureAbiDefinedSymbolPresent"] \
            or product["candidateRustRuntimeSourceSymbolPresent"]:
        raise AssertionError("기본 c04-runtime-gpu archive에 fixture code가 들어 있습니다.")
    if not all(fixture["sceneMarkerPresence"].values()) \
            or not fixture["fixtureAbiTextPresent"] \
            or not fixture["fixtureAbiDefinedSymbolPresent"]:
        raise AssertionError("fixture-test positive control에서 marker/ABI가 누락됐습니다.")
    report = {
        "schemaVersion": 1,
        "scope": "macOS ARM64 spinon-ffi static archive only",
        "createdAtUtc": datetime.now(timezone.utc).isoformat(),
        "sourceSha256BeforeAndAfter": before,
        "sceneIds": SCENES,
        "fixtureAbiSymbol": FFI_SYMBOL,
        "rustRuntimeSourceSymbolSubstring": RUST_SYMBOL,
        "targetDirectory": str(TARGET.relative_to(ROOT)),
        "results": {"product": product, "fixtureTest": fixture},
        "limitations": [
            "이 결과는 APK, iOS app/IPA 또는 release bundle 전체의 fixture exclusion을 증명하지 않습니다.",
            "정적 spinon-ffi archive의 scene 문자열과 fixture 심볼 경계만 검사합니다.",
            "최적화 archive에서 내부 Rust runtime_source 심볼은 inline·제거될 수 있어 참고값으로만 기록합니다.",
        ],
    }
    path = EVIDENCE / "artifact-boundary" / "artifact-boundary.json"
    path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({name: {
        "archiveBytes": value["archiveBytes"],
        "archiveSha256": value["archiveSha256"],
        "sceneMarkersPresent": sum(value["sceneMarkerPresence"].values()),
        "sceneCount": len(value["sceneMarkerPresence"]),
        "fixtureAbiDefinedSymbolPresent": value["fixtureAbiDefinedSymbolPresent"],
        "candidateRustRuntimeSourceSymbolPresent": value["candidateRustRuntimeSourceSymbolPresent"],
    } for name, value in report["results"].items()}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
