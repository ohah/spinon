#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-runtime-2026-11-02"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-column-flow-implicit-columns-precomparison.html"
bootstrap_dir="$repo_root/examples/bootstrap"
backup_dir="$evidence_dir/original-platform-assets"
temporary_dir="$(mktemp -d "${TMPDIR:-/tmp}/spinon-c11-implicit-columns.XXXXXX")"

restore_bootstrap_sources() {
  cp "$temporary_dir/index.html" "$bootstrap_dir/index.html"
  cp "$temporary_dir/root-app.js" "$bootstrap_dir/root-app.js"
  rm "$temporary_dir/index.html" "$temporary_dir/root-app.js"
  rmdir "$temporary_dir"
}

mkdir -p "$backup_dir" "$evidence_dir/android-avd" "$evidence_dir/ios-simulator"
cp "$bootstrap_dir/index.html" "$backup_dir/index.html"
cp "$bootstrap_dir/root-app.js" "$backup_dir/root-app.js"
cp "$bootstrap_dir/index.html" "$temporary_dir/index.html"
cp "$bootstrap_dir/root-app.js" "$temporary_dir/root-app.js"
trap restore_bootstrap_sources EXIT HUP INT TERM

cp "$fixture" "$bootstrap_dir/index.html"
cp "$evidence_dir/root-app-marker.js" "$bootstrap_dir/root-app.js"

mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android \
  > "$evidence_dir/android-avd/build.log" 2>&1
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 bun run build:ios-sim \
  > "$evidence_dir/ios-simulator/build.log" 2>&1
