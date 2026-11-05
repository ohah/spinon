#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-intrinsic-gap-runtime-2026-11-05"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-gap-runtime.html"
android_asset="$repo_root/platforms/android/app/src/main/assets/root-html-default-rendering.html"
ios_asset="$repo_root/platforms/ios/Resources/root-html-default-rendering.html"
v8_root="$repo_root/build/c11-v8-rebuild/v8-source/v8"
backup_dir="$(mktemp -d "${TMPDIR:-/tmp}/spinon-c11-gap.XXXXXX")"

cp "$android_asset" "$backup_dir/android.html"
cp "$ios_asset" "$backup_dir/ios.html"
restore_assets() {
  cp "$backup_dir/android.html" "$android_asset"
  cp "$backup_dir/ios.html" "$ios_asset"
}
trap restore_assets EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

cp "$fixture" "$android_asset"
cp "$fixture" "$ios_asset"
SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_V8_DIR="$v8_root" mise exec -- bun run build:android \
  > "$evidence_dir/android/build.log" 2>&1
SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_V8_DIR="$v8_root" mise exec -- bun run build:ios-sim \
  > "$evidence_dir/ios/build.log" 2>&1
