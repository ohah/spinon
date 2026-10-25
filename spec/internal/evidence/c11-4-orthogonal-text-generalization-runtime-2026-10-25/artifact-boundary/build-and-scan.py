#!/usr/bin/env python3
"""Build and compare default/fixture-test C11.4 FFI static archives."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import shutil
import subprocess


ROOT = Path(__file__).resolve().parents[5]
OUT = Path(__file__).resolve().parent
ARCHIVE_DIR = ROOT / "build/spinon/c11-4-orthogonal-text-intrinsic-generalization-artifact"
MATRIX_METADATA = ROOT / "spec/internal/evidence/c11-4-orthogonal-text-generalization-runtime-2026-10-25/matrix-metadata.json"
LLVM_NM = Path("/opt/homebrew/opt/llvm@22/bin/llvm-nm")
SCENE_IDS = [
    "single-token-min-content", "single-token-max-content",
    "two-words-min-content", "two-words-max-content",
    "three-words-min-content", "three-words-max-content",
    "collapsed-whitespace-min-content", "collapsed-whitespace-max-content",
    "spaces-only-min-content", "spaces-only-max-content",
]
FIXTURE_SYMBOL = "spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture"
DOM_ROOT_ID_PREFIX = b"c114-orthogonal-text-"
DOM_ROOT_ID_SUFFIX = b"-grid"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def implementation_paths() -> tuple[str, ...]:
    matrix = json.loads(MATRIX_METADATA.read_text(encoding="utf-8"))
    paths = tuple(matrix["source"]["implementationFiles"].keys())
    if len(paths) != 22:
        raise RuntimeError(f"matrix metadata must define the frozen 22 implementation paths; found {len(paths)}")
    return paths


def source_snapshot(paths: tuple[str, ...]) -> dict:
    files = {}
    for relative in paths:
        path = ROOT / relative
        if not path.is_file():
            raise RuntimeError(f"missing implementation source: {relative}")
        files[relative] = sha256(path.read_bytes())
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    diff = subprocess.run(["git", "diff", "--binary", head, "--", *paths], cwd=ROOT,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if diff.returncode:
        raise RuntimeError(f"git diff --binary {head} -- <implementation paths> failed: {diff.stderr}")
    return {
        "head": head,
        "implementationDiffSha256": sha256(diff.stdout.encode()),
        "implementationFiles": files,
        "implementationFilesSha256": sha256(json.dumps(files, sort_keys=True).encode()),
    }


def build(label: str, feature: str, paths: tuple[str, ...], expected_snapshot: dict) -> tuple[Path, dict, dict]:
    log = OUT / f"{label}-build.log"
    command = ["mise", "exec", "--", "cargo", "build", "--locked", "--release",
               "--no-default-features", "-p", "spinon-ffi", "--lib", "--features", feature]
    before = source_snapshot(paths)
    if before != expected_snapshot:
        raise RuntimeError(f"{label} source snapshot changed before build")
    result = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=True)
    archive = ROOT / "target/release/libspinon_ffi.a"
    preserved = ARCHIVE_DIR / f"{label}-spinon-ffi.a"
    if result.returncode == 0 and archive.is_file():
        ARCHIVE_DIR.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(archive, preserved)
    after = source_snapshot(paths)
    log.write_text(
        "$ " + " ".join(command) + "\n"
        + "sourceSnapshotBefore:\n" + json.dumps(before, indent=2) + "\n"
        + "buildOutput:\n" + result.stdout
        + "sourceSnapshotAfter:\n" + json.dumps(after, indent=2) + "\n"
        + f"sourceSnapshotsMatch={before == after == expected_snapshot}\n",
        encoding="utf-8",
    )
    if result.returncode:
        raise RuntimeError(f"{label} build failed with exit code {result.returncode}; see {log}")
    if not archive.is_file():
        raise RuntimeError(f"static archive not found: {archive}")
    if before != after or after != expected_snapshot:
        raise RuntimeError(f"{label} source snapshot changed during build; see {log}")
    return preserved, before, after


def inspect(label: str, archive: Path) -> dict:
    data = archive.read_bytes()
    markers = {scene_id: scene_id.encode() in data for scene_id in SCENE_IDS}
    expanded_dom_ids = {
        scene_id: f"c114-orthogonal-text-{scene_id}-grid".encode() in data
        for scene_id in SCENE_IDS
    }
    # Each expanded DOM ID is formatted at runtime. The archive stores the
    # unique C11.4 prefix and scene IDs separately, not ten expanded ID strings.
    root_prefix_present = DOM_ROOT_ID_PREFIX in data
    root_suffix_present = DOM_ROOT_ID_SUFFIX in data
    command = [str(LLVM_NM), "--demangle", "--defined-only", str(archive)]
    result = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True)
    if result.returncode:
        raise RuntimeError(f"llvm-nm failed on {label}: {result.stderr}")
    rows = [line for line in result.stdout.splitlines()
            if FIXTURE_SYMBOL in line or "c11_4::orthogonal_text_intrinsic" in line]
    (OUT / f"symbols-{label}.txt").write_text("\n".join(rows) + ("\n" if rows else ""))
    return {
        "path": str(archive.relative_to(ROOT)),
        "bytes": len(data),
        "sha256": sha256(data),
        "sceneMarkers": markers,
        "sceneMarkerCount": sum(markers.values()),
        "domRootTemplatePrefix": DOM_ROOT_ID_PREFIX.decode(),
        "domRootTemplatePrefixPresent": root_prefix_present,
        "domRootTemplatePrefixOccurrences": data.count(DOM_ROOT_ID_PREFIX),
        "domRootTemplateSuffix": DOM_ROOT_ID_SUFFIX.decode(),
        "domRootTemplateSuffixPresent": root_suffix_present,
        "domRootTemplateSuffixOccurrences": data.count(DOM_ROOT_ID_SUFFIX),
        "domRootTemplatePartsPresent": root_prefix_present and root_suffix_present,
        "expandedDomRootIdMarkers": expanded_dom_ids,
        "expandedDomRootIdMarkerCount": sum(expanded_dom_ids.values()),
        "expandedDomRootIdsStoredInArchive": any(expanded_dom_ids.values()),
        "fixtureFfiSymbolPresent": any(FIXTURE_SYMBOL in line for line in rows),
        "candidateModuleSymbolRows": [line for line in rows if "c11_4::orthogonal_text_intrinsic" in line],
        "llvmNmExitCode": result.returncode,
        "llvmNmStderr": result.stderr.strip(),
    }


def main() -> None:
    if not LLVM_NM.is_file():
        raise RuntimeError(f"LLVM 22 llvm-nm is not installed at {LLVM_NM}")
    paths = implementation_paths()
    baseline_snapshot = source_snapshot(paths)
    product, product_before, product_after = build("product", "c04-runtime-gpu", paths, baseline_snapshot)
    fixture, fixture_before, fixture_after = build("fixture-test", "c04-runtime-gpu-fixture-test", paths, baseline_snapshot)
    source_snapshot_checks = {
        "product": {"before": product_before, "after": product_after,
                    "matches": product_before == product_after == baseline_snapshot},
        "fixtureTest": {"before": fixture_before, "after": fixture_after,
                        "matches": fixture_before == fixture_after == baseline_snapshot},
    }
    assert all(check["matches"] for check in source_snapshot_checks.values())
    results = {"product": inspect("product", product),
               "fixtureTest": inspect("fixture-test", fixture)}
    assert results["product"]["sceneMarkerCount"] == 0
    assert not results["product"]["domRootTemplatePrefixPresent"]
    assert not results["product"]["domRootTemplatePartsPresent"]
    assert not results["product"]["fixtureFfiSymbolPresent"]
    assert not results["product"]["candidateModuleSymbolRows"]
    assert results["fixtureTest"]["sceneMarkerCount"] == len(SCENE_IDS)
    assert results["fixtureTest"]["domRootTemplatePrefixPresent"]
    assert results["fixtureTest"]["domRootTemplateSuffixPresent"]
    assert results["fixtureTest"]["domRootTemplatePartsPresent"]
    assert not results["fixtureTest"]["expandedDomRootIdsStoredInArchive"]
    assert results["fixtureTest"]["fixtureFfiSymbolPresent"]
    llvm_nm_version = subprocess.check_output([str(LLVM_NM), "--version"], cwd=ROOT, text=True).splitlines()[0]
    report = {
        "sourceSnapshot": baseline_snapshot,
        "sourceSnapshotChecks": source_snapshot_checks,
        "implementationPathListSource": str(MATRIX_METADATA.relative_to(ROOT)),
        "sourceSnapshotScope": "Compiled implementation sources only. implementationDiffSha256 is git diff --binary HEAD restricted to the 22 implementation paths; evidence/docs are intentionally excluded. All 22 on-disk file contents, including untracked files, are covered by per-file and aggregate SHA-256 values.",
        "llvmNm": str(LLVM_NM),
        "llvmNmVersion": llvm_nm_version,
        "fixtureFfiSymbol": FIXTURE_SYMBOL,
        "sceneCount": len(SCENE_IDS),
        "results": results,
        "domRootScanNote": "Expanded IDs are formatted at runtime and are not stored in the archive. The unique prefix is a feature-specific control; the -grid suffix is generic and also appears in the product archive.",
        "buildCommands": {
            "product": "mise exec -- cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu",
            "fixtureTest": "mise exec -- cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test",
        },
        "scope": "macOS ARM64 spinon-ffi static archives; does not inspect APK/IPA bundles",
    }
    (OUT / "scan-results.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    readme = [
        "# C11.4 static archive boundary scan",
        "",
        f"Source revision (HEAD): `{baseline_snapshot['head']}`",
        f"Implementation diff SHA-256: `{baseline_snapshot['implementationDiffSha256']}`",
        f"Implementation files: {len(baseline_snapshot['implementationFiles'])}",
        f"Implementation aggregate SHA-256: `{baseline_snapshot['implementationFilesSha256']}`",
        "Source scope: the binary diff is limited to the 22 compiled implementation paths; evidence/docs changes are intentionally excluded. Every on-disk implementation file is hashed directly, including untracked files.",
        f"Build source snapshots matched before/after both archives: `{all(check['matches'] for check in source_snapshot_checks.values())}`",
        f"Command: `python3 {Path(__file__).relative_to(ROOT)}`",
        f"Scanner: `{report['llvmNmVersion']}`",
        "",
        "| Archive | Bytes | SHA-256 | Scene IDs | DOM prefix | DOM suffix | DOM template parts | Expanded IDs | Fixture FFI symbol | llvm-nm |",
        "|---|---:|---|---:|---|---|---|---:|---|---:|",
    ]
    for label, value in results.items():
        display = "fixture-test" if label == "fixtureTest" else label
        readme.append(
            f"| {display} | {value['bytes']} | `{value['sha256']}` | "
            f"{value['sceneMarkerCount']}/{len(SCENE_IDS)} | "
            f"{value['domRootTemplatePrefixPresent']} ({value['domRootTemplatePrefixOccurrences']}) | "
            f"{value['domRootTemplateSuffixPresent']} ({value['domRootTemplateSuffixOccurrences']}) | "
            f"{value['domRootTemplatePartsPresent']} | "
            f"{value['expandedDomRootIdMarkerCount']}/10 | "
            f"{value['fixtureFfiSymbolPresent']} | {value['llvmNmExitCode']} |"
        )
    readme.extend([
        "",
        "Product negative controls: no C11.4 scene ID, unique DOM prefix/template parts, or fixture FFI symbol. Fixture-test positive controls: all ten scene IDs, the DOM template prefix/suffix parts, and the fixture FFI symbol are present.",
        "",
        "The ten fully expanded DOM root IDs are not expected as literal archive strings: runtime code formats `c114-orthogonal-text-{scene-id}-grid`. The `-grid` suffix is generic and also occurs in the product archive, so it is reported but is not used alone as a negative-control assertion.",
        "",
        "Matching `llvm-nm --demangle --defined-only` rows:",
        "",
        "```text",
        *(["(product): no C11.4 fixture or module symbols"] if not results["product"]["candidateModuleSymbolRows"] and not results["product"]["fixtureFfiSymbolPresent"] else []),
        *results["fixtureTest"]["candidateModuleSymbolRows"],
        *([f"fixture FFI export: {FIXTURE_SYMBOL}"] if results["fixtureTest"]["fixtureFfiSymbolPresent"] else []),
        "```",
        "",
        f"Full 22-path per-file source snapshot and all four before/after captures are in `scan-results.json`; each build log records its full before/after snapshots.",
        "",
        "Scope: macOS ARM64 `spinon-ffi` static archives only. This scan does not inspect APK/IPA bundles or device state.",
        "",
    ])
    (OUT / "README.md").write_text("\n".join(readme), encoding="utf-8")
    print(json.dumps({label: {"bytes": value["bytes"], "sha256": value["sha256"],
                              "sceneMarkers": value["sceneMarkerCount"],
                              "domRootTemplatePrefixPresent": value["domRootTemplatePrefixPresent"],
                              "domRootTemplateSuffixPresent": value["domRootTemplateSuffixPresent"],
                              "fixtureFfiSymbolPresent": value["fixtureFfiSymbolPresent"],
                              "llvmNmExitCode": value["llvmNmExitCode"]}
                      for label, value in results.items()}, indent=2))
    print(json.dumps({
        "sourceHead": baseline_snapshot["head"],
        "implementationDiffSha256": baseline_snapshot["implementationDiffSha256"],
        "implementationFileCount": len(baseline_snapshot["implementationFiles"]),
        "implementationFilesSha256": baseline_snapshot["implementationFilesSha256"],
        "sourceSnapshotsMatch": all(check["matches"] for check in source_snapshot_checks.values()),
    }, indent=2))


if __name__ == "__main__":
    main()
