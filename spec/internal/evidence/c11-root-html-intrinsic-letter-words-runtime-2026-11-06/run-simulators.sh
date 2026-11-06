#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06"
pass_name=pass01
if [[ "$#" -gt 0 ]]; then pass_name="$1"; fi
if [[ ! "$pass_name" =~ ^pass[0-9]{2}$ ]]; then
  echo "Pass name must use passNN format" >&2
  exit 1
fi
run_dir="$evidence_dir/$pass_name"
bootstrap_dir="$repo_root/build/spinon/bootstrap"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-runtime.html"
root_app="$evidence_dir/root-app.js"
android_serial="emulator-5554"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
bundle_id="dev.spinon.bootstrap"
run_id="c11word-20261106"
export ANDROID_HOME="/Users/yoonhb/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"

if [[ -e "$run_dir/environment.txt" ]]; then
  echo "Refusing to overwrite existing simulator evidence: $pass_name" >&2
  exit 1
fi
for source in \
  "$bootstrap_dir/index.html" \
  "$bootstrap_dir/root-app.js" \
  "$ios_app/index.html" \
  "$ios_app/root-app.js"; do
  if [[ ! -s "$source" ]]; then
    echo "Missing existing app asset: $source" >&2
    exit 1
  fi
done
if [[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" != "37" ]]; then
  echo "Android target is not API 37: $android_serial" >&2
  exit 1
fi
if [[ "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" != "arm64-v8a" ]]; then
  echo "Android target is not ARM64: $android_serial" >&2
  exit 1
fi
if [[ -n "$(adb -s "$android_serial" shell pidof "$bundle_id" | tr -d '\r' || true)" ]]; then
  echo "Refusing to replace an already running Android app on $android_serial" >&2
  exit 1
fi
if ! xcrun simctl list devices available | rg -q "$ios_udid.*Booted"; then
  echo "iPhone 17 Pro iOS 26.2 simulator is not booted: $ios_udid" >&2
  exit 1
fi
if xcrun simctl spawn "$ios_udid" launchctl list | rg -q 'dev\.spinon\.bootstrap'; then
  echo "Refusing to replace an already running iOS app on $ios_udid" >&2
  exit 1
fi

mkdir -p "$run_dir/android" "$run_dir/ios" "$run_dir/original/bootstrap" "$run_dir/original/ios-app"
cp -p "$bootstrap_dir/index.html" "$run_dir/original/bootstrap/index.html"
cp -p "$bootstrap_dir/root-app.js" "$run_dir/original/bootstrap/root-app.js"
cp -p "$ios_app/index.html" "$run_dir/original/ios-app/index.html"
cp -p "$ios_app/root-app.js" "$run_dir/original/ios-app/root-app.js"

{
  printf 'candidate_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  while IFS= read -r source_path; do
    shasum -a 256 "$repo_root/$source_path"
  done <<'EOF' > "$run_dir/source-inputs.sha256"
crates/spinon-runtime/src/session/tests/root_html_grid_intrinsic_auto_rows.rs
crates/spinon-runtime/src/session/tests/root_html_grid_intrinsic_auto_rows/combined_multiple_text.rs
crates/spinon-runtime/src/session/tests/root_html_grid_intrinsic_auto_rows/letter_words.rs
crates/spinon-runtime/src/session/tests/root_html_grid_intrinsic_auto_rows/multiple_text.rs
crates/spinon-runtime/src/session/tests/root_html_grid_intrinsic_auto_rows/normal_whitespace/rejects_invalid.rs
crates/spinon-style-to-layout/src/c11_1_root_html_intrinsic_auto_rows_text_tests.rs
crates/spinon-style-to-layout/src/c11_1_root_html_intrinsic_auto_rows_text_tests/combined_multiple_text.rs
crates/spinon-style-to-layout/src/c11_1_root_html_intrinsic_auto_rows_text_tests/letter_words.rs
crates/spinon-style-to-layout/src/c11_1_root_html_intrinsic_auto_rows_text_tests/normal_whitespace.rs
crates/spinon-style-to-layout/src/projection/style_values.rs
crates/spinon-style-to-layout/src/projection/style_values/root_html_intrinsic_auto_rows.rs
platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java
platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java
platforms/android/app/src/main/java/dev/spinon/bootstrap/SurfaceViewPixelCapture.java
platforms/android/app/src/c11_grid_metal_surface_pixels_fixture_test/java/dev/spinon/bootstrap/C114RootHtmlLetterWordPixelCopyAdapter.java
platforms/android/app/src/c11_grid_metal_surface_pixels_fixture_test/resources/META-INF/services/dev.spinon.bootstrap.RuntimeGpuSurfacePixelCaptureAdapter
platforms/ios/Sources/C0410RuntimeGpuDemo.swift
tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-runtime.html
spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06/root-app.js
spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06/run-simulators.sh
EOF
  printf 'candidate_source_manifest_sha256=%s\n' "$(shasum -a 256 "$run_dir/source-inputs.sha256" | awk '{print $1}')"
  printf 'fixture_sha256=%s\n' "$(shasum -a 256 "$fixture" | awk '{print $1}')"
  printf 'root_app_source_sha256=%s\n' "$(shasum -a 256 "$root_app" | awk '{print $1}')"
  printf 'android_serial=%s\n' "$android_serial"
  printf 'android_api=37\n'
  printf 'android_abi=arm64-v8a\n'
  printf 'ios_udid=%s\n' "$ios_udid"
  printf 'ios_device=iPhone 17 Pro\n'
  printf 'ios_runtime=iOS 26.2\n'
  printf 'android_c11_pixel_readback_flag=1\n'
  printf 'ios_c11_pixel_readback_flag=1\n'
  printf 'physical_android_target=not used\n'
} > "$run_dir/environment.txt"

restore_assets() {
  local original_status=$?
  local restore_status=0
  trap - EXIT INT TERM
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  cp -p "$run_dir/original/bootstrap/index.html" "$bootstrap_dir/index.html" || restore_status=1
  cp -p "$run_dir/original/bootstrap/root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
  cp -p "$run_dir/original/ios-app/index.html" "$ios_app/index.html" || restore_status=1
  cp -p "$run_dir/original/ios-app/root-app.js" "$ios_app/root-app.js" || restore_status=1
  cmp -s "$run_dir/original/bootstrap/index.html" "$bootstrap_dir/index.html" || restore_status=1
  cmp -s "$run_dir/original/bootstrap/root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
  cmp -s "$run_dir/original/ios-app/index.html" "$ios_app/index.html" || restore_status=1
  cmp -s "$run_dir/original/ios-app/root-app.js" "$ios_app/root-app.js" || restore_status=1
  if [[ "$restore_status" == 0 ]]; then
    printf 'asset_restoration=PASS (bootstrap and iOS app bundle byte-for-byte)\n' >> "$run_dir/environment.txt"
  else
    printf 'asset_restoration=FAIL\n' >> "$run_dir/environment.txt"
  fi
  if [[ "$original_status" != 0 ]]; then exit "$original_status"; fi
  exit "$restore_status"
}
trap restore_assets EXIT INT TERM

SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
  mise exec -- bun run build:android > "$run_dir/android/build.log" 2>&1
cp "$fixture" "$bootstrap_dir/index.html"
cp "$root_app" "$bootstrap_dir/root-app.js"
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
  "$repo_root/platforms/android/gradlew" -p "$repo_root/platforms/android" \
  :app:assembleDebug -x :app:prepareSpinonBootstrap \
  > "$run_dir/android/asset-overlay-build.log" 2>&1
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
shasum -a 256 "$android_apk" > "$run_dir/android/apk.sha256"
adb -s "$android_serial" install -r "$android_apk" > "$run_dir/android/install.log"
pixelcopy_cache="/data/user/0/$bundle_id/cache/c11-root-html-grid-letter-word-pixelcopy/surface-$run_id.png"
if adb -s "$android_serial" shell run-as "$bundle_id" test -f "$pixelcopy_cache"; then
  adb -s "$android_serial" exec-out run-as "$bundle_id" cat "$pixelcopy_cache" \
    > "$run_dir/android/prior-pixelcopy-cache.png"
fi
adb -s "$android_serial" shell run-as "$bundle_id" rm -f "$pixelcopy_cache"
printf 'pixelcopy_cache_reset=removed only %s after preserving any prior PNG\n' "$pixelcopy_cache" \
  >> "$run_dir/environment.txt"
adb -s "$android_serial" shell am start -S -n "$bundle_id/.MainActivity" \
  --ez spinon_root_html_letter_word_pixelcopy true \
  --es spinon_root_html_letter_word_pixelcopy_run_id "$run_id" \
  > "$run_dir/android/launch.log" 2>&1
pixelcopy_ready=0
android_pid=""
for attempt in $(seq 1 100); do
  android_pid="$(adb -s "$android_serial" shell pidof "$bundle_id" | tr -d '\r' || true)"
  if [[ "$android_pid" =~ ^[0-9]+$ ]] \
      && adb -s "$android_serial" logcat -d -s SpinonBootstrap \
          | awk -v pid="$android_pid" '$3 == pid' \
          | rg -q "SPINON_C111WORD_PIXELCOPY_SUMMARY status=matched run_id=$run_id.*expected_samples=1.*matched_samples=1"; then
    pixelcopy_ready=1
    break
  fi
  sleep 0.2
done
if [[ ! "$android_pid" =~ ^[0-9]+$ ]]; then
  echo "Android app process PID was not observed" >&2
  exit 1
fi
adb -s "$android_serial" logcat -d -s SpinonBootstrap \
  | awk -v pid="$android_pid" '$3 == pid' > "$run_dir/android/app.log"
if [[ "$pixelcopy_ready" != 1 ]]; then
  echo "Android PixelCopy did not report one matched sample" >&2
  exit 1
fi
rg -q 'SPINON_C0410_ENVIRONMENT .*layout=ready boxes=8' "$run_dir/android/app.log"
rg -q 'SPINON_C0410_DRAW .*presented boxes=8 .*runtime_text_count=2' "$run_dir/android/app.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_EVAL .*error=none' "$run_dir/android/app.log"
rg -q "SPINON_C111WORD_PIXELCOPY_SUMMARY status=matched run_id=$run_id.*expected_samples=1.*matched_samples=1" "$run_dir/android/app.log"
test "$(rg -c "SPINON_C111WORD_PIXELCOPY_SAMPLE run_id=$run_id.*exact=true" "$run_dir/android/app.log")" -eq 1
adb -s "$android_serial" exec-out screencap -p > "$run_dir/android/screenshot.png"
adb -s "$android_serial" exec-out run-as "$bundle_id" cat \
  "$pixelcopy_cache" \
  > "$run_dir/android/pixelcopy.png"
unzip -p "$android_apk" assets/index.html | shasum -a 256 > "$run_dir/android/asset-hash.txt"
unzip -p "$android_apk" assets/root-app.js | shasum -a 256 >> "$run_dir/android/asset-hash.txt"

SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
  mise exec -- bun run build:ios-sim > "$run_dir/ios/build.log" 2>&1
cp "$fixture" "$ios_app/index.html"
cp "$root_app" "$ios_app/root-app.js"
shasum -a 256 "$ios_app/index.html" "$ios_app/root-app.js" \
  > "$run_dir/ios/asset-hash.txt"
shasum -a 256 "$ios_app/SpinonBootstrap" > "$run_dir/ios/executable.sha256"
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios/install.log"
xcrun simctl launch "$ios_udid" "$bundle_id" \
  "--spinon-root-html-grid-auto-minimum-metal-surface-pixels=$run_id" \
  > "$run_dir/ios/launch.log"
ios_pid="$(awk -F': ' '/^dev\.spinon\.bootstrap: [0-9]+$/ { print $2 }' "$run_dir/ios/launch.log")"
if [[ ! "$ios_pid" =~ ^[0-9]+$ ]]; then
  echo "iOS app process PID was not reported by simctl launch" >&2
  exit 1
fi
sleep 3
xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
  --predicate 'process == "SpinonBootstrap"' \
  | awk -v pid="$ios_pid" 'index($0, "SpinonBootstrap[" pid ":") > 0' \
  > "$run_dir/ios/app.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass" "$run_dir/ios/app.log"
rg -q "SPINON_C111WORD_METAL_PIXEL_SUMMARY status=matched run_id=$run_id.*expected_samples=1.*matched_samples=1" "$run_dir/ios/app.log"
test "$(rg -c "SPINON_C111WORD_METAL_PIXEL_SAMPLE run_id=$run_id.*exact=true" "$run_dir/ios/app.log")" -eq 1
xcrun simctl io "$ios_udid" screenshot "$run_dir/ios/screenshot.png" \
  > "$run_dir/ios/screenshot.log" 2>&1

printf 'android_installed_fixture_sha256=%s\n' "$(unzip -p "$android_apk" assets/index.html | shasum -a 256 | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'ios_installed_fixture_sha256=%s\n' "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'pixel_scope=Android SurfaceView PixelCopy and iOS Metal drawable before compositor\n' >> "$run_dir/environment.txt"
