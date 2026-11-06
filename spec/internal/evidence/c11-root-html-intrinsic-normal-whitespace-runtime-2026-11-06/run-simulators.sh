#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-runtime-2026-11-06"
pass_name="${1:-pass01}"
if [[ ! "$pass_name" =~ ^pass[0-9]{2}$ ]]; then
  echo "Pass name must use passNN format" >&2
  exit 1
fi
run_dir="$evidence_dir/$pass_name"
bootstrap_dir="$repo_root/build/spinon/bootstrap"
original_bootstrap="$evidence_dir/original-bootstrap/pass01"
original_app="$evidence_dir/original-app/pass01"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-normal-whitespace-runtime.html"
root_app="$evidence_dir/root-app.js"
android_serial="emulator-5554"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
export ANDROID_HOME="/Users/yoonhb/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"

if [[ -e "$run_dir/environment.txt" ]]; then
  echo "Refusing to overwrite existing simulator evidence: $pass_name" >&2
  exit 1
fi
for source in \
  "$original_bootstrap/index.html" \
  "$original_bootstrap/root-app.js" \
  "$original_app/index.html" \
  "$original_app/root-app.js"; do
  if [[ ! -s "$source" ]]; then
    echo "Missing preserved original app asset: $source" >&2
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
if [[ -n "$(adb -s "$android_serial" shell pidof dev.spinon.bootstrap | tr -d '\r')" ]]; then
  echo "Refusing to replace an already running Android app on $android_serial" >&2
  exit 1
fi
if ! xcrun simctl list devices available | rg -q "$ios_udid.*Booted"; then
  echo "iPhone 17 Pro iOS 26.2 simulator is not booted: $ios_udid" >&2
  exit 1
fi
if ! xcrun simctl spawn "$ios_udid" launchctl list | rg -q 'dev\.spinon\.bootstrap'; then
  :
else
  echo "Refusing to replace an already running iOS app on $ios_udid" >&2
  exit 1
fi

mkdir -p "$run_dir/android" "$run_dir/ios"
{
  printf 'candidate_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  printf 'fixture_sha256=%s\n' "$(shasum -a 256 "$fixture" | awk '{print $1}')"
  printf 'root_app_source_sha256=%s\n' "$(shasum -a 256 "$root_app" | awk '{print $1}')"
  printf 'android_serial=%s\n' "$android_serial"
  printf 'android_api=37\n'
  printf 'android_abi=arm64-v8a\n'
  printf 'ios_udid=%s\n' "$ios_udid"
  printf 'ios_device=iPhone 17 Pro\n'
  printf 'ios_runtime=iOS 26.2\n'
  printf 'android_c11_fixture_flags=0\n'
  printf 'ios_c11_fixture_flags=0\n'
  printf 'physical_android_target=not used\n'
} > "$run_dir/environment.txt"

restore_assets() {
  local original_status=$?
  local restore_status=0
  trap - EXIT INT TERM
  adb -s "$android_serial" shell am force-stop dev.spinon.bootstrap >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap >/dev/null 2>&1 || true
  cp -p "$original_bootstrap/index.html" "$bootstrap_dir/index.html" || restore_status=1
  cp -p "$original_bootstrap/root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
  if [[ -d "$ios_app" ]]; then
    cp -p "$original_app/index.html" "$ios_app/index.html" || restore_status=1
    cp -p "$original_app/root-app.js" "$ios_app/root-app.js" || restore_status=1
  else
    restore_status=1
  fi
  cmp -s "$original_bootstrap/index.html" "$bootstrap_dir/index.html" || restore_status=1
  cmp -s "$original_bootstrap/root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
  if [[ -d "$ios_app" ]]; then
    cmp -s "$original_app/index.html" "$ios_app/index.html" || restore_status=1
    cmp -s "$original_app/root-app.js" "$ios_app/root-app.js" || restore_status=1
  fi
  if [[ "$restore_status" == 0 ]]; then
    printf 'asset_restoration=PASS (bootstrap and iOS app bundle byte-for-byte)\n' >> "$run_dir/environment.txt"
  else
    printf 'asset_restoration=FAIL\n' >> "$run_dir/environment.txt"
  fi
  if [[ "$original_status" != 0 ]]; then
    exit "$original_status"
  fi
  exit "$restore_status"
}
trap restore_assets EXIT INT TERM

SPINON_ENABLE_C04_RUNTIME_GPU=1 "$repo_root/tools/build-android-app.sh" > "$run_dir/android/build.log" 2>&1
cp "$fixture" "$bootstrap_dir/index.html"
cp "$root_app" "$bootstrap_dir/root-app.js"
"$repo_root/platforms/android/gradlew" -p "$repo_root/platforms/android" \
  -PspinonC04RuntimeGpu=1 :app:assembleDebug -x :app:prepareSpinonBootstrap \
  > "$run_dir/android/asset-overlay-build.log" 2>&1
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
adb -s "$android_serial" install -r "$android_apk" > "$run_dir/android/install.log"
adb -s "$android_serial" shell am start -n dev.spinon.bootstrap/.MainActivity > "$run_dir/android/launch.log" 2>&1
sleep 8
android_pid="$(adb -s "$android_serial" shell pidof dev.spinon.bootstrap | tr -d '\r')"
if [[ -z "$android_pid" ]]; then
  echo "Android app process was not launched" >&2
  exit 1
fi
adb -s "$android_serial" logcat -d --pid="$android_pid" -s SpinonBootstrap > "$run_dir/android/app.log"
adb -s "$android_serial" exec-out screencap -p > "$run_dir/android/screenshot.png"
unzip -p "$android_apk" assets/index.html | shasum -a 256 > "$run_dir/android/asset-hash.txt"
unzip -p "$android_apk" assets/root-app.js | shasum -a 256 >> "$run_dir/android/asset-hash.txt"
rg -q 'SPINON_C0410_ENVIRONMENT .*layout=ready boxes=8' "$run_dir/android/app.log"
rg -q 'SPINON_C0410_DRAW .*presented boxes=8 .*runtime_text_count=2' "$run_dir/android/app.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_EVAL .*error=none .*layout=ready boxes=8' "$run_dir/android/app.log"

SPINON_ENABLE_C04_RUNTIME_GPU=1 "$repo_root/tools/build-ios-sim.sh" > "$run_dir/ios/build.log" 2>&1
cp "$fixture" "$ios_app/index.html"
cp "$root_app" "$ios_app/root-app.js"
shasum -a 256 "$ios_app/index.html" "$ios_app/root-app.js" > "$run_dir/ios/asset-hash.txt"
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios/install.log"
xcrun simctl launch "$ios_udid" dev.spinon.bootstrap > "$run_dir/ios/launch.log"
sleep 8
xcrun simctl spawn "$ios_udid" log show --last 2m --style compact \
  --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_C0410_DRAW" OR eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_EVAL")' \
  > "$run_dir/ios/app.log"
xcrun simctl io "$ios_udid" screenshot "$run_dir/ios/screenshot.png" > "$run_dir/ios/screenshot.log" 2>&1
rg -q 'SPINON_C0410_ENVIRONMENT .*layout=ready boxes=8' "$run_dir/ios/app.log"
rg -q 'SPINON_C0410_DRAW .*presented boxes=8 .*runtime_text_count=2' "$run_dir/ios/app.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_EVAL .*error=none .*layout=ready boxes=8' "$run_dir/ios/app.log"

printf 'android_process=%s\n' "$android_pid" >> "$run_dir/environment.txt"
printf 'android_installed_fixture_sha256=%s\n' "$(unzip -p "$android_apk" assets/index.html | shasum -a 256 | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'ios_installed_fixture_sha256=%s\n' "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'screenshot_scope=visual smoke only; no PixelCopy or Metal drawable readback\n' >> "$run_dir/environment.txt"
