#!/usr/bin/env python3
import hashlib
import json
import subprocess
from pathlib import Path


root = Path(__file__).resolve().parents[4]
runtime = Path(__file__).resolve().parent
base_revision = "411bda24e2525b55d48a6c14ec484aec6ef8e97a"
paths = [
    "crates/spinon-ffi/src/runtime_gpu/fixtures.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_neighbor_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_percentage_neighbor_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_neighbors.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_negative_percentage_neighbors.rs",
    "crates/spinon-layout/src/grid.rs",
    "crates/spinon-layout/src/grid_track_sizing_tests/auto_repeat_calc_neighbors.rs",
    "crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs",
    "crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_repeat_neighbor_tests.rs",
    "crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/auto_repeat_calc_neighbor.rs",
    "platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java",
    "platforms/ios/Sources/C0410RuntimeGpuDemo.swift",
    "tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs",
    "tools/css-reference/fixture-module-boundary.test.mjs",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/spec-derived-expectations.json",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json",
    "plan/c11-4-auto-repeat-negative-percentage-neighbors.md",
    "spec/internal/0059-c11-grid-layout.md",
    "spec/STATUS.md",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/README.md",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/implementation-runtime-review-20.md",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/pr-diff-review-30.md",
    "spec/internal/evidence/c11-4-negative-percentage-merge-status-sync-2026-10-24/pr-diff-review-30.md",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/generate-source-manifest.py",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/README.md",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/scan-ffi-archives.py",
]
source_diff_paths = [
    "crates/spinon-ffi/src/runtime_gpu/fixtures.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_neighbor_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_percentage_neighbor_scenes.json",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_neighbors.rs",
    "crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_negative_percentage_neighbors.rs",
    "crates/spinon-layout/src/grid.rs",
    "crates/spinon-layout/src/grid_track_sizing_tests/auto_repeat_calc_neighbors.rs",
    "crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs",
    "crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_repeat_neighbor_tests.rs",
    "crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/auto_repeat_calc_neighbor.rs",
    "platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java",
    "platforms/ios/Sources/C0410RuntimeGpuDemo.swift",
    "tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs",
    "tools/css-reference/fixture-module-boundary.test.mjs",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py",
    "tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html",
    "plan/c11-4-auto-repeat-negative-percentage-neighbors.md",
    "spec/internal/0059-c11-grid-layout.md",
    "spec/STATUS.md",
]
screenshots = [
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/selected-screenshots/android-avd-negative-percentage-col-both-negative-final.png",
    "spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/selected-screenshots/ios-simulator-negative-percentage-col-both-negative-final.png",
]

def entry(relative):
    path = root / relative
    data = path.read_bytes()
    return {"path": relative, "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


diff = subprocess.check_output(["git", "diff", "--binary", base_revision, "--", *source_diff_paths], cwd=root)
source_diff_paths_changed = subprocess.check_output(
    ["git", "diff", "--name-only", base_revision, "--", *source_diff_paths], cwd=root, text=True
).splitlines()
manifest = {
    "schemaVersion": 1,
    "sourceRevision": {
        "commit": base_revision,
        "tree": subprocess.check_output(["git", "rev-parse", f"{base_revision}^{{tree}}"], cwd=root, text=True).strip(),
    },
    "sourceDiffSha256": hashlib.sha256(diff).hexdigest(),
    "sourceDiffPaths": source_diff_paths_changed,
    "scopeBoundary": "sourceDiffSha256 hashes git diff --binary from the plan merge base for implementation, fixture, capture, plan, and official contract/status paths; evidence inputs and screenshots have file-level hashes below.",
    "files": [entry(path) for path in paths],
    "screenshots": [entry(path) for path in screenshots],
    "runs": {
        "android": "android-avd-c114-signed-negative-percentage-neighbors-android-avd-20261024T061515084Z-2d9ec1ae",
        "ios": "ios-simulator-c114-signed-negative-percentage-neighbors-ios-simulator-20261024T061454873Z-3c7ad103",
    },
}
for platform in ("android", "ios"):
    run_name = manifest["runs"][platform]
    simulator = json.loads((runtime / run_name / "simulator.json").read_text())
    manifest["runs"][platform] = {
        "evidenceDirectory": run_name,
        "captureTrackedDiffSha256": simulator["sourceRevision"]["trackedDiffSha256"],
        "applicationSha256": simulator.get("apkSha256", simulator.get("appSha256")),
    }
(runtime / "source-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps({"sourceRevision": manifest["sourceRevision"], "sourceDiffSha256": manifest["sourceDiffSha256"], "files": len(paths), "screenshots": len(screenshots)}, indent=2))
