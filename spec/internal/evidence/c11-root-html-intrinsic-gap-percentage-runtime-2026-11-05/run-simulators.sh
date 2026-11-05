#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-gap-percentage-runtime.html"
android_asset="$repo_root/platforms/android/app/src/main/assets/root-html-default-rendering.html"
ios_asset="$repo_root/platforms/ios/Resources/root-html-default-rendering.html"
android_serial="${ANDROID_SERIAL:-emulator-5554}"
ios_simulator="${IOS_SIMULATOR_UUID:-9DCCE836-7659-4914-B6C1-13F54BFFA9DB}"
bundle_id="dev.spinon.bootstrap"
v8_dir="$repo_root/build/c11-v8-rebuild/v8-source/v8"
tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/spinon-c11-gap.XXXXXX")"
android_started=0
ios_started=0

cp -p "$android_asset" "$tmp_dir/android-before.html"
cp -p "$ios_asset" "$tmp_dir/ios-before.html"
android_before_sha="$(shasum -a 256 "$tmp_dir/android-before.html" | cut -d ' ' -f 1)"
ios_before_sha="$(shasum -a 256 "$tmp_dir/ios-before.html" | cut -d ' ' -f 1)"

restore_assets() {
  if [[ "$android_started" == "1" ]]; then
    adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  fi
  if [[ "$ios_started" == "1" ]]; then
    xcrun simctl terminate "$ios_simulator" "$bundle_id" >/dev/null 2>&1 || true
  fi
  cp -p "$tmp_dir/android-before.html" "$android_asset"
  cp -p "$tmp_dir/ios-before.html" "$ios_asset"
  test "$(shasum -a 256 "$android_asset" | cut -d ' ' -f 1)" = "$android_before_sha"
  test "$(shasum -a 256 "$ios_asset" | cut -d ' ' -f 1)" = "$ios_before_sha"
  rm "$tmp_dir/android-before.html" "$tmp_dir/ios-before.html"
  rmdir "$tmp_dir"
}
trap restore_assets EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

test -d "$v8_dir"
test "$(shasum -a 256 "$fixture" | cut -d ' ' -f 1)" = "$(cat "$evidence_dir/runtime-fixture.sha256" | cut -d ' ' -f 1)"
mkdir -p "$evidence_dir/android" "$evidence_dir/ios"
cp "$fixture" "$android_asset"
cp "$fixture" "$ios_asset"

export SPINON_ENABLE_C04_RUNTIME_GPU=1
export SPINON_V8_DIR="$v8_dir"
if ! "$repo_root/tools/build-android-app.sh" > "$evidence_dir/android/build.log" 2>&1; then
  tail -n 100 "$evidence_dir/android/build.log"
  exit 1
fi

android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
test -f "$android_apk"
shasum -a 256 "$android_apk" > "$evidence_dir/android/apk.sha256"
unzip -p "$android_apk" assets/root-html-default-rendering.html | shasum -a 256 > "$evidence_dir/android/apk-root-html-asset.sha256"
adb -s "$android_serial" logcat -c
adb -s "$android_serial" install -r "$android_apk" > "$evidence_dir/android/install.log" 2>&1
adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
  --ez spinon_c0412_root_html_default_render true > "$evidence_dir/android/launch.log" 2>&1
android_started=1
sleep 8
adb -s "$android_serial" exec-out screencap -p > "$evidence_dir/android/simulator-screen.png"
adb -s "$android_serial" logcat -d -v threadtime > "$evidence_dir/android/logcat-full.txt"
rg "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE|SPINON_C0410_ENVIRONMENT status=|SPINON_C0410_DRAW .*presented boxes=" "$evidence_dir/android/logcat-full.txt" > "$evidence_dir/android/logcat-runtime.txt" || true
adb -s "$android_serial" shell am force-stop "$bundle_id"
android_started=0

if ! "$repo_root/tools/build-ios-sim.sh" > "$evidence_dir/ios/build.log" 2>&1; then
  tail -n 100 "$evidence_dir/ios/build.log"
  exit 1
fi

ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
test -d "$ios_app"
shasum -a 256 "$ios_app/Info.plist" > "$evidence_dir/ios/app-info.sha256"
shasum -a 256 "$ios_app/root-html-default-rendering.html" > "$evidence_dir/ios/app-root-html-asset.sha256"
xcrun simctl install "$ios_simulator" "$ios_app" > "$evidence_dir/ios/install.log" 2>&1
xcrun simctl launch "$ios_simulator" "$bundle_id" \
  --spinon-c0412-root-html-default-render > "$evidence_dir/ios/launch.log" 2>&1
ios_started=1
sleep 8
xcrun simctl io "$ios_simulator" screenshot "$evidence_dir/ios/simulator-screen.png" > "$evidence_dir/ios/screenshot.log" 2>&1
xcrun simctl spawn "$ios_simulator" log show --last 3m --style compact --predicate 'process == "SpinonBootstrap"' > "$evidence_dir/ios/unified-log-full.txt" 2>&1
rg "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE|SPINON_C0410_ENVIRONMENT status=|SPINON_C0410_DRAW .*presented boxes=" "$evidence_dir/ios/unified-log-full.txt" > "$evidence_dir/ios/unified-log-runtime.txt" || true
xcrun simctl terminate "$ios_simulator" "$bundle_id"
ios_started=0

shasum -a 256 "$evidence_dir/android/simulator-screen.png" > "$evidence_dir/android/simulator-screen.sha256"
shasum -a 256 "$evidence_dir/ios/simulator-screen.png" > "$evidence_dir/ios/simulator-screen.sha256"
printf 'android_asset_before=%s\nios_asset_before=%s\nfixture=%s\n' \
  "$android_before_sha" "$ios_before_sha" \
  "$(shasum -a 256 "$fixture" | cut -d ' ' -f 1)" > "$evidence_dir/asset-restoration.txt"
