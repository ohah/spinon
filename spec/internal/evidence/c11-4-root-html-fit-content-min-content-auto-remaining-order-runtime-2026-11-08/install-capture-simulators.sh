#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08"
android_out="$evidence_dir/android/final"
ios_out="$evidence_dir/ios/final"
android_serial="emulator-5554"
android_package="dev.spinon.bootstrap"
ios_udid="97E99AA8-D144-4DA8-B1BF-7F6B34007F8B"
ios_bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html"
fixture_sha="$(shasum -a 256 "$fixture" | awk '{print $1}')"
fixture_bytes="$(wc -c < "$fixture" | tr -d '[:space:]')"

mkdir -p "$android_out" "$ios_out"
test "$(unzip -p "$apk" assets/index.html | shasum -a 256 | awk '{print $1}')" = "$fixture_sha"
test "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" = "$fixture_sha"
test "$(shasum -a 256 "$repo_root/examples/bootstrap/index.html" | awk '{print $1}')" = "e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9"
test "$(shasum -a 256 "$repo_root/examples/bootstrap/root-app.js" | awk '{print $1}')" = "b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32"
printf 'fixture_sha256=%s\nfixture_bytes=%s\n' "$fixture_sha" "$fixture_bytes" > "$evidence_dir/final-source.json"

printf 'android_serial=%s\n' "$android_serial" > "$android_out/device.txt"
adb -s "$android_serial" get-serialno >> "$android_out/device.txt"
adb -s "$android_serial" shell getprop ro.kernel.qemu > "$android_out/is-emulator.txt"
adb -s "$android_serial" shell getprop ro.build.version.release > "$android_out/android-release.txt"
adb -s "$android_serial" shell getprop ro.build.version.sdk > "$android_out/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi > "$android_out/abi.txt"
test "$(tr -d '\r' < "$android_out/is-emulator.txt")" = "1"
test "$(tr -d '\r' < "$android_out/api-level.txt")" = "37"
test "$(tr -d '\r' < "$android_out/abi.txt")" = "arm64-v8a"
adb -s "$android_serial" shell wm size > "$android_out/screen-size.txt"
adb -s "$android_serial" shell wm density > "$android_out/density.txt"
adb -s "$android_serial" shell cmd uimode night > "$android_out/appearance.txt"
adb -s "$android_serial" shell dumpsys package "$android_package" > "$android_out/package-before.txt"
adb -s "$android_serial" install -r "$apk" > "$android_out/install.log" 2>&1
adb -s "$android_serial" shell am force-stop "$android_package" > "$android_out/force-stop.log" 2>&1
adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" > "$android_out/launch.log" 2>&1

android_pid=""
for attempt in $(seq 1 30); do
  android_pid="$(adb -s "$android_serial" shell pidof "$android_package" | tr -d '\r' || true)"
  if [[ -n "$android_pid" ]]; then break; fi
  sleep 1
done
test -n "$android_pid"
printf '%s\n' "$android_pid" > "$android_out/app-pid.txt"
for attempt in $(seq 1 45); do
  adb -s "$android_serial" logcat -d -v threadtime \
    | awk -v app_pid="$android_pid" '$3 == app_pid' > "$android_out/app-current-process.log"
  if rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$android_out/app-current-process.log" \
    && rg -q 'SPINON_C0410_DRAW status=0 presented boxes=12' "$android_out/app-current-process.log"; then break; fi
  sleep 1
done
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_ASSET html_bytes=$fixture_bytes " "$android_out/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=$fixture_bytes" "$android_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$android_out/app-current-process.log"
rg -q 'SPINON_C0410_DRAW status=0 presented boxes=12' "$android_out/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$android_out/app-current-process.log"
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$android_out/screenshot.png"
adb -s "$android_serial" shell dumpsys activity activities | rg -m 4 'topResumedActivity|mResumedActivity' > "$android_out/resumed-activity.txt"
adb -s "$android_serial" shell dumpsys package "$android_package" > "$android_out/package-after.txt"
cp "$android_out/app-pid.txt" "$android_out/current-app-pid.txt"

printf 'ios_udid=%s\n' "$ios_udid" > "$ios_out/device.txt"
xcrun simctl list devices booted > "$ios_out/booted-devices.txt"
rg -q "$ios_udid" "$ios_out/booted-devices.txt"
xcrun simctl list devices --json | python3 -c 'import json,sys; data=json.load(sys.stdin); devices=[device for group in data["devices"].values() for device in group if device["udid"]==sys.argv[1]]; assert len(devices)==1 and devices[0]["state"]=="Booted" and devices[0]["deviceTypeIdentifier"]=="com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro", devices; print(json.dumps(devices[0], sort_keys=True))' "$ios_udid" > "$ios_out/device-metadata.json"
xcrun simctl getenv "$ios_udid" SIMULATOR_DEVICE_NAME > "$ios_out/device-name.txt"
xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" data > "$ios_out/data-container-before.txt"
xcrun simctl install "$ios_udid" "$ios_app" > "$ios_out/install.log" 2>&1
xcrun simctl terminate "$ios_udid" "$ios_bundle_id" > "$ios_out/terminate.log" 2>&1 || true
xcrun simctl launch "$ios_udid" "$ios_bundle_id" > "$ios_out/launch.log" 2>&1
ios_pid="$(awk -F': ' '/dev\.spinon\.bootstrap:/ { print $2; exit }' "$ios_out/launch.log")"
test -n "$ios_pid"
printf '%s\n' "$ios_pid" > "$ios_out/app-pid.txt"
for attempt in $(seq 1 30); do
  xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
    --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT")' \
    > "$ios_out/app.log"
  if rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$ios_out/app.log"; then break; fi
  sleep 1
done
rg -F "SpinonBootstrap[$ios_pid:" "$ios_out/app.log" > "$ios_out/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=$fixture_bytes " "$ios_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$ios_out/app-current-process.log"
rg -q 'SPINON_C0410_ENVIRONMENT status=0 .*layout=ready boxes=12' "$ios_out/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$ios_out/app-current-process.log"
xcrun simctl io "$ios_udid" screenshot "$ios_out/screenshot.png" > "$ios_out/screenshot.log" 2>&1
xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" data > "$ios_out/data-container-after.txt"
if cmp -s "$ios_out/data-container-before.txt" "$ios_out/data-container-after.txt"; then
  printf 'same\n' > "$ios_out/data-container-identity.txt"
else
  printf 'changed\n' > "$ios_out/data-container-identity.txt"
fi
ios_installed_app="$(xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" app)"
shasum -a 256 "$ios_installed_app/index.html" > "$ios_out/installed-fixture.sha256"
test "$(awk '{print $1}' "$ios_out/installed-fixture.sha256")" = "$fixture_sha"
shasum -a 256 \
  "$apk" "$ios_app/SpinonBootstrap" "$ios_app/index.html" \
  "$android_out/screenshot.png" "$ios_out/screenshot.png" \
  > "$evidence_dir/final-capture.sha256"
printf 'Android emulator and iPhone 17 Pro iOS Simulator launched the same default Root HTML fixture; Bootstrap sources were restored and no erase/uninstall/clear command was run.\n'
