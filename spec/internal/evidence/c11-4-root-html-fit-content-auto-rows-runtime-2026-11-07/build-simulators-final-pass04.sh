#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-runtime-2026-11-07"
source_index="$repo_root/examples/bootstrap/index.html"
source_root_app="$repo_root/examples/bootstrap/root-app.js"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-runtime.html"
output_dir="$evidence_dir"
original_index_sha="e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9"
original_root_app_sha="b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32"
mkdir -p "$output_dir/bootstrap-original/final-pass04"
cp "$source_index" "$output_dir/bootstrap-original/final-pass04/index.html"
cp "$source_root_app" "$output_dir/bootstrap-original/final-pass04/root-app.js"
test "$(shasum -a 256 "$source_index" | awk '{print $1}')" = "$original_index_sha"
test "$(shasum -a 256 "$source_root_app" | awk '{print $1}')" = "$original_root_app_sha"
restore_bootstrap() {
  cp "$output_dir/bootstrap-original/final-pass04/index.html" "$source_index"
  cp "$output_dir/bootstrap-original/final-pass04/root-app.js" "$source_root_app"
  test "$(shasum -a 256 "$source_index" | awk '{print $1}')" = "$original_index_sha"
  test "$(shasum -a 256 "$source_root_app" | awk '{print $1}')" = "$original_root_app_sha"
}
trap restore_bootstrap EXIT HUP INT TERM
cp "$fixture" "$source_index"
cat > "$source_root_app" <<'JS'
globalThis.__spinonRootHtmlDefaultRouteSmoke = "fit-content-auto-rows";
JS
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_DENSE_PLACEMENT_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
bash "$repo_root/tools/build-android-app.sh" > "$output_dir/android/final-pass04/build.log" 2>&1
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_DENSE_PLACEMENT_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_1_ROOT_HTML_COLUMN_FLOW_MIXED_CROSS_AXIS_FIXTURE_TEST=0 \
SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
bash "$repo_root/tools/build-ios-sim.sh" > "$output_dir/ios/final-pass04/build.log" 2>&1
restore_bootstrap
trap - EXIT HUP INT TERM
shasum -a 256 "$source_index" "$source_root_app" > "$output_dir/bootstrap-original/final-pass04/restored-bootstrap.sha256"
shasum -a 256 \
  "$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk" \
  "$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/Info.plist" \
  "$fixture" > "$output_dir/final-pass04-built-artifacts.sha256"
printf 'Android and iOS Simulator builds succeeded; fixture-test feature flags remained disabled.\n'
