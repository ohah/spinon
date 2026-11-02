#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02"
original_dir="$evidence_dir/original-bootstrap"
mkdir -p "$original_dir"

cp -p "$repo_root/examples/bootstrap/index.html" "$original_dir/index.html"
cp -p "$repo_root/examples/bootstrap/root-app.js" "$original_dir/root-app.js"
cp -p "$repo_root/build/spinon/bootstrap/index.html" "$original_dir/built-index.html"
cp -p "$repo_root/build/spinon/bootstrap/root-app.js" "$original_dir/built-root-app.js"
cp -p "$repo_root/build/spinon/bootstrap/app.js" "$original_dir/built-app.js"

restore_bootstrap() {
  cp -p "$original_dir/index.html" "$repo_root/examples/bootstrap/index.html"
  cp -p "$original_dir/root-app.js" "$repo_root/examples/bootstrap/root-app.js"
  cp -p "$original_dir/built-index.html" "$repo_root/build/spinon/bootstrap/index.html"
  cp -p "$original_dir/built-root-app.js" "$repo_root/build/spinon/bootstrap/root-app.js"
  cp -p "$original_dir/built-app.js" "$repo_root/build/spinon/bootstrap/app.js"
}
trap restore_bootstrap EXIT

cp "$evidence_dir/candidate-index.html" "$repo_root/examples/bootstrap/index.html"
cp "$evidence_dir/root-app-runtime-probe.js" "$repo_root/examples/bootstrap/root-app.js"

cd "$repo_root"
mise exec -- env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  bun run build:android > "$evidence_dir/android-build.log" 2>&1
rg -q "BUILD SUCCESSFUL" "$evidence_dir/android-build.log"

mise exec -- env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=0 \
  bun run build:ios-sim > "$evidence_dir/ios-simulator-build.log" 2>&1
rg -q "BUILD SUCCEEDED" "$evidence_dir/ios-simulator-build.log"

restore_bootstrap
trap - EXIT
