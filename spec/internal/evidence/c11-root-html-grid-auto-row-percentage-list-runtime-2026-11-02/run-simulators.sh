#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02"
pass="${PASS:-pass03}"
artifact_dir="$repo_root/build/spinon/artifacts/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02"
android="/Users/yoonhb/Library/Android/sdk/platform-tools/adb"
android_serial="emulator-5556"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
ios_bundle="dev.spinon.bootstrap"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"

test ! -e "$evidence_dir/android-install-$pass.log"
test ! -e "$evidence_dir/android-launch-$pass.log"
test ! -e "$evidence_dir/android-runtime-$pass.log"
test ! -e "$evidence_dir/ios-install-$pass.log"
test ! -e "$evidence_dir/ios-launch-$pass.log"
test ! -e "$evidence_dir/ios-runtime-$pass.log"
test ! -e "$evidence_dir/ios-screenshot-$pass.log"
test ! -e "$evidence_dir/android-api37-avd-$pass.png"
test ! -e "$evidence_dir/ios-26_2-simulator-$pass.png"
test -f "$artifact_dir/android-api37-avd-$pass.apk"
test -d "$ios_app"

"$android" -s "$android_serial" install -r "$artifact_dir/android-api37-avd-$pass.apk" \
  > "$evidence_dir/android-install-$pass.log" 2>&1
"$android" -s "$android_serial" shell am force-stop "$ios_bundle"
"$android" -s "$android_serial" shell am start -W -n "$ios_bundle/.MainActivity" \
  > "$evidence_dir/android-launch-$pass.log" 2>&1

android_pid=""
android_ready=0
for _ in {1..30}; do
  android_pid="$("$android" -s "$android_serial" shell pidof "$ios_bundle" 2>/dev/null || true)"
  android_pid="${android_pid//$'\r'/}"
  if [[ -n "$android_pid" ]] && \
    "$android" -s "$android_serial" logcat --pid="$android_pid" -d -v threadtime -s SpinonBootstrap:I \
      | rg -q "SPINON_C0410_DRAW.*presented boxes=13"; then
    android_ready=1
    break
  fi
  sleep 1
done
test "$android_ready" -eq 1
"$android" -s "$android_serial" logcat --pid="$android_pid" -d -v threadtime -s SpinonBootstrap:I \
  > "$evidence_dir/android-runtime-$pass.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready status=0 layout=ready boxes=13" \
  "$evidence_dir/android-runtime-$pass.log"
rg -q "SPINON_C0410_DRAW.*presented boxes=13" "$evidence_dir/android-runtime-$pass.log"
"$android" -s "$android_serial" exec-out screencap -p > "$evidence_dir/android-api37-avd-$pass.png"
"$android" -s "$android_serial" shell am force-stop "$ios_bundle"
if "$android" -s "$android_serial" shell pidof "$ios_bundle" 2>/dev/null | rg -q '[0-9]'; then
  exit 1
fi

xcrun simctl install "$ios_udid" "$ios_app" > "$evidence_dir/ios-install-$pass.log" 2>&1
xcrun simctl launch --terminate-running-process "$ios_udid" "$ios_bundle" \
  > "$evidence_dir/ios-launch-$pass.log" 2>&1
sleep 8
xcrun simctl spawn "$ios_udid" log show --last 2m --style compact \
  --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_BOOT" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' \
  > "$evidence_dir/ios-runtime-$pass.log" 2>&1
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready status=0 layout=ready boxes=13" \
  "$evidence_dir/ios-runtime-$pass.log"
rg -q "SPINON_C0410_DRAW.*presented boxes=13" "$evidence_dir/ios-runtime-$pass.log"
xcrun simctl io "$ios_udid" screenshot "$evidence_dir/ios-26_2-simulator-$pass.png" \
  > "$evidence_dir/ios-screenshot-$pass.log" 2>&1
xcrun simctl terminate "$ios_udid" "$ios_bundle"
