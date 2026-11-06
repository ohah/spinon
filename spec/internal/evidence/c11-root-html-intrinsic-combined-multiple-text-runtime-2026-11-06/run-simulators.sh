#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-intrinsic-combined-multiple-text-runtime-2026-11-06"
bootstrap_dir="$repo_root/build/spinon/bootstrap"
original_dir="$evidence_dir/original-bootstrap/pass01"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-combined-multiple-text-precomparison.html"
android_serial="emulator-5554"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
export ANDROID_HOME="/Users/yoonhb/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"

if [[ -e "$evidence_dir/environment.txt" ]]; then
  echo "Refusing to overwrite existing pass01 simulator evidence" >&2
  exit 1
fi
if [[ ! -s "$original_dir/index.html" || ! -s "$original_dir/root-app.js" ]]; then
  echo "The original bootstrap assets must be captured before running this pass" >&2
  exit 1
fi

restore_bootstrap_assets() {
  adb -s "$android_serial" shell am force-stop dev.spinon.bootstrap >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap >/dev/null 2>&1 || true
  cp -p "$original_dir/index.html" "$bootstrap_dir/index.html"
  cp -p "$original_dir/root-app.js" "$bootstrap_dir/root-app.js"
}
trap restore_bootstrap_assets EXIT INT TERM

if [[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" != "37" ]]; then
  echo "Android target is not API 37: $android_serial" >&2
  exit 1
fi
if [[ "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" != "arm64-v8a" ]]; then
  echo "Android target is not ARM64: $android_serial" >&2
  exit 1
fi
if ! xcrun simctl list devices available | rg -q "$ios_udid.*Booted"; then
  echo "iPhone 17 Pro iOS 26.2 simulator is not booted: $ios_udid" >&2
  exit 1
fi

{
  printf 'candidate_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  printf 'fixture_sha256=%s\n' "$(shasum -a 256 "$fixture" | awk '{print $1}')"
  printf 'root_app_source_sha256=%s\n' "$(shasum -a 256 "$evidence_dir/root-app.js" | awk '{print $1}')"
  printf 'android_serial=%s\n' "$android_serial"
  printf 'android_api=37\n'
  printf 'android_abi=arm64-v8a\n'
  printf 'ios_udid=%s\n' "$ios_udid"
  printf 'ios_device=iPhone 17 Pro\n'
  printf 'ios_runtime=iOS 26.2\n'
  printf 'android_default_fixture_flags=0\n'
  printf 'ios_default_fixture_flags=0\n'
  printf 'physical_android_target=not used\n'
} > "$evidence_dir/environment.txt"

SPINON_ENABLE_C04_RUNTIME_GPU=1 "$repo_root/tools/build-android-app.sh" > "$evidence_dir/android/build.log" 2>&1
cp "$fixture" "$bootstrap_dir/index.html"
cp "$evidence_dir/root-app.js" "$bootstrap_dir/root-app.js"
"$repo_root/platforms/android/gradlew" -p "$repo_root/platforms/android" \
  -PspinonC04RuntimeGpu=1 :app:assembleDebug -x :app:prepareSpinonBootstrap \
  > "$evidence_dir/android/asset-overlay-build.log" 2>&1
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
adb -s "$android_serial" install -r "$android_apk" > "$evidence_dir/android/install.log"
adb -s "$android_serial" shell am start -n dev.spinon.bootstrap/.MainActivity > "$evidence_dir/android/launch.log" 2>&1
sleep 8
android_pid="$(adb -s "$android_serial" shell pidof dev.spinon.bootstrap | tr -d '\r')"
if [[ -z "$android_pid" ]]; then
  echo "Android app process was not launched" >&2
  exit 1
fi
adb -s "$android_serial" logcat -d --pid="$android_pid" -s SpinonBootstrap > "$evidence_dir/android/app.log"
adb -s "$android_serial" exec-out screencap -p > "$evidence_dir/android/screenshot.png"
unzip -p "$android_apk" assets/index.html | shasum -a 256 > "$evidence_dir/android/asset-hash.txt"
unzip -p "$android_apk" assets/root-app.js | shasum -a 256 >> "$evidence_dir/android/asset-hash.txt"

SPINON_ENABLE_C04_RUNTIME_GPU=1 "$repo_root/tools/build-ios-sim.sh" > "$evidence_dir/ios/build.log" 2>&1
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
cp "$fixture" "$ios_app/index.html"
cp "$evidence_dir/root-app.js" "$ios_app/root-app.js"
shasum -a 256 "$ios_app/index.html" "$ios_app/root-app.js" > "$evidence_dir/ios/asset-hash.txt"
xcrun simctl install "$ios_udid" "$ios_app" > "$evidence_dir/ios/install.log"
xcrun simctl launch "$ios_udid" dev.spinon.bootstrap > "$evidence_dir/ios/launch.log"
sleep 8
xcrun simctl spawn "$ios_udid" log show --last 2m --style compact --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_EVAL")' > "$evidence_dir/ios/app.log"
xcrun simctl io "$ios_udid" screenshot "$evidence_dir/ios/screenshot.png" > "$evidence_dir/ios/screenshot.log" 2>&1

printf 'android_process=%s\n' "$android_pid" >> "$evidence_dir/environment.txt"
printf 'android_installed_fixture_sha256=%s\n' "$(unzip -p "$android_apk" assets/index.html | shasum -a 256 | awk '{print $1}')" >> "$evidence_dir/environment.txt"
