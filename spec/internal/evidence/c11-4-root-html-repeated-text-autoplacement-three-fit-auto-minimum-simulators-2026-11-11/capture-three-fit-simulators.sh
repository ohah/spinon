#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
android_serial="emulator-5554"
ios_bundle="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_initial_appearance="$(xcrun simctl ui "$ios_udid" appearance)"
android_initial_night="$(/Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell cmd uimode night | awk '{print $3}')"

cleanup() {
  local status=$?
  trap - EXIT
  xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap >/dev/null 2>&1 || true
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell am force-stop dev.spinon.bootstrap >/dev/null 2>&1 || true
  xcrun simctl ui "$ios_udid" appearance "$ios_initial_appearance" >/dev/null 2>&1 || true
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell cmd uimode night "$android_initial_night" >/dev/null 2>&1 || true
  xcrun simctl shutdown "$ios_udid" >/dev/null 2>&1 || true
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" emu kill >/dev/null 2>&1 || true
  for cleanup_attempt in {1..30}; do
    if ! /Users/yoonhb/Library/Android/sdk/platform-tools/adb devices -l | rg -q "^$android_serial[[:space:]]"; then
      break
    fi
    sleep 1
  done
  {
    xcrun simctl list devices booted
    /Users/yoonhb/Library/Android/sdk/platform-tools/adb devices -l
    pgrep -alf 'SpinonBootstrap|emulator -avd|agent-browser' || true
  } > "$evidence_dir/cleanup.log"
  exit "$status"
}
trap cleanup EXIT

test -d "$ios_bundle"
test -f "$android_apk"
test "$(xcrun simctl list devices booted | rg -c "$ios_udid")" = "1"
test "$(/Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" = "1"

{
  xcrun simctl list devices booted
  xcrun simctl ui "$ios_udid" appearance
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell getprop ro.build.version.sdk
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell getprop ro.product.cpu.abi
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell wm size
  /Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" shell wm density
} > "$evidence_dir/device-info.log"

xcrun simctl install "$ios_udid" "$ios_bundle"
/Users/yoonhb/Library/Android/sdk/platform-tools/adb -s "$android_serial" install -r "$android_apk"

xcrun simctl ui "$ios_udid" appearance light
xcrun simctl launch --terminate-running-process "$ios_udid" dev.spinon.bootstrap \
  > "$evidence_dir/ios-light-launch.log"
sleep 3
xcrun simctl spawn "$ios_udid" log show --style compact --last 5m \
  --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_BOOT" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' \
  > "$evidence_dir/ios-light.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$evidence_dir/ios-light.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$evidence_dir/ios-light.log"
xcrun simctl io "$ios_udid" screenshot "$evidence_dir/ios-iphone17pro-ios26_2-light.png"

xcrun simctl ui "$ios_udid" appearance dark
xcrun simctl launch --terminate-running-process "$ios_udid" dev.spinon.bootstrap \
  > "$evidence_dir/ios-dark-launch.log"
sleep 3
xcrun simctl spawn "$ios_udid" log show --style compact --last 5m \
  --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_BOOT" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' \
  > "$evidence_dir/ios-dark.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$evidence_dir/ios-dark.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$evidence_dir/ios-dark.log"
xcrun simctl io "$ios_udid" screenshot "$evidence_dir/ios-iphone17pro-ios26_2-dark.png"

adb_bin="/Users/yoonhb/Library/Android/sdk/platform-tools/adb"
"$adb_bin" -s "$android_serial" shell cmd uimode night no
android_light_log_start="$(date +'%m-%d %H:%M:%S.000')"
"$adb_bin" -s "$android_serial" shell am force-stop dev.spinon.bootstrap
"$adb_bin" -s "$android_serial" shell am start -n dev.spinon.bootstrap/.MainActivity \
  > "$evidence_dir/android-light-launch.log"
sleep 3
"$adb_bin" -s "$android_serial" logcat -d -T "$android_light_log_start" -s SpinonBootstrap:I \
  > "$evidence_dir/android-light.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$evidence_dir/android-light.log"
rg -q 'SPINON_C0410_DRAW status=0 presented' "$evidence_dir/android-light.log"
"$adb_bin" -s "$android_serial" exec-out screencap -p \
  > "$evidence_dir/android-api37_2-light.png"

"$adb_bin" -s "$android_serial" shell cmd uimode night yes
android_dark_log_start="$(date +'%m-%d %H:%M:%S.000')"
"$adb_bin" -s "$android_serial" shell am force-stop dev.spinon.bootstrap
"$adb_bin" -s "$android_serial" shell am start -n dev.spinon.bootstrap/.MainActivity \
  > "$evidence_dir/android-dark-launch.log"
sleep 3
"$adb_bin" -s "$android_serial" logcat -d -T "$android_dark_log_start" -s SpinonBootstrap:I \
  > "$evidence_dir/android-dark.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$evidence_dir/android-dark.log"
rg -q 'SPINON_C0410_DRAW status=0 presented' "$evidence_dir/android-dark.log"
"$adb_bin" -s "$android_serial" exec-out screencap -p \
  > "$evidence_dir/android-api37_2-dark.png"
