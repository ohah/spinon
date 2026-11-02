#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03"
backup_dir="$evidence_dir/original-bootstrap"
artifact_dir="$repo_root/build/spinon/artifacts/c11-root-html-grid-percent-gap-2026-11-03"
mkdir -p "$artifact_dir"
[[ ! -e "$backup_dir" ]] || { echo "refusing to overwrite prior bootstrap backups: $backup_dir" >&2; exit 1; }
[[ ! -e "$artifact_dir/android-api37-avd.apk" ]] || { echo "refusing to overwrite a prior Android artifact" >&2; exit 1; }
[[ ! -e "$artifact_dir/SpinonBootstrap.app" ]] || { echo "refusing to overwrite a prior iOS app artifact" >&2; exit 1; }
mkdir "$backup_dir"

for source in \
  "$repo_root/examples/bootstrap/index.html" \
  "$repo_root/examples/bootstrap/root-app.js" \
  "$repo_root/build/spinon/bootstrap/index.html" \
  "$repo_root/build/spinon/bootstrap/root-app.js" \
  "$repo_root/build/spinon/bootstrap/app.js"; do
  [[ -f "$source" ]] || { echo "required bootstrap source is missing: $source" >&2; exit 1; }
done

cp -p "$repo_root/examples/bootstrap/index.html" "$backup_dir/index.html"
cp -p "$repo_root/examples/bootstrap/root-app.js" "$backup_dir/root-app.js"
cp -p "$repo_root/build/spinon/bootstrap/index.html" "$backup_dir/built-index.html"
cp -p "$repo_root/build/spinon/bootstrap/root-app.js" "$backup_dir/built-root-app.js"
cp -p "$repo_root/build/spinon/bootstrap/app.js" "$backup_dir/built-app.js"

restore_bootstrap() {
  cp -p "$backup_dir/index.html" "$repo_root/examples/bootstrap/index.html"
  cp -p "$backup_dir/root-app.js" "$repo_root/examples/bootstrap/root-app.js"
  cp -p "$backup_dir/built-index.html" "$repo_root/build/spinon/bootstrap/index.html"
  cp -p "$backup_dir/built-root-app.js" "$repo_root/build/spinon/bootstrap/root-app.js"
  cp -p "$backup_dir/built-app.js" "$repo_root/build/spinon/bootstrap/app.js"
}
trap restore_bootstrap EXIT

cp "$evidence_dir/root-html-grid-percentage-gap-runtime.html" "$repo_root/examples/bootstrap/index.html"
cp "$evidence_dir/root-app-runtime-probe.js" "$repo_root/examples/bootstrap/root-app.js"
cp "$evidence_dir/root-html-grid-percentage-gap-runtime.html" "$repo_root/build/spinon/bootstrap/index.html"
cp "$evidence_dir/root-app-runtime-probe.js" "$repo_root/build/spinon/bootstrap/root-app.js"

cd "$repo_root"
mise exec -- env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=0 \
  SPINON_ENABLE_C12_5_STICKY_FIXTURE_TEST=0 \
  SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_2_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_3_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST=0 \
  SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST=0 \
  SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=0 \
  bun run build:android > "$evidence_dir/android-build-pass01.log" 2>&1
rg -q "BUILD SUCCESSFUL" "$evidence_dir/android-build-pass01.log"
cp "$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk" "$artifact_dir/android-api37-avd.apk"

mise exec -- env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_BLOCK_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_12_ROOT_HTML_TEXT_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=0 \
  SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=0 \
  SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=0 \
  SPINON_ENABLE_C12_5_STICKY_FIXTURE_TEST=0 \
  SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_2_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C14_3_APP_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST=0 \
  SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST=0 \
  SPINON_ENABLE_C19_2_BOX_PAINT_OPACITY_FIXTURE_TEST=0 \
  SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=0 \
  bun run build:ios-sim > "$evidence_dir/ios-simulator-build-pass01.log" 2>&1
rg -q "BUILD SUCCEEDED" "$evidence_dir/ios-simulator-build-pass01.log"
ditto "$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app" "$artifact_dir/SpinonBootstrap.app"

restore_bootstrap
trap - EXIT

shasum -a 256 \
  "$evidence_dir/root-html-grid-percentage-gap-runtime.html" \
  "$evidence_dir/root-app-runtime-probe.js" \
  "$artifact_dir/android-api37-avd.apk" \
  "$artifact_dir/SpinonBootstrap.app/SpinonBootstrap" \
  > "$evidence_dir/build-sha256.txt"

cd "$repo_root"
commit="$(git rev-parse HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
printf '{\n  "commit": "%s",\n  "tree": "%s"\n}\n' "$commit" "$tree" \
  > "$evidence_dir/source-revision.json"
