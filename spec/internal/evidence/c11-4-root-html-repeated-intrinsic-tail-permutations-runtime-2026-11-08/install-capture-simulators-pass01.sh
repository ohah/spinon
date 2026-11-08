#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-runtime-2026-11-08"
backup_root="$repo_root/build/spinon/c11-4-root-html-repeated-intrinsic-tail-permutations-20261108-pass01/pass01"
android_out="$evidence_dir/android/pass01"
ios_out="$evidence_dir/ios/pass01"
android_serial="emulator-5554"
android_package="dev.spinon.bootstrap"
ios_udid="97E99AA8-D144-4DA8-B1BF-7F6B34007F8B"
ios_bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-runtime.html"
fixture_sha="$(shasum -a 256 "$fixture" | awk '{print $1}')"
fixture_bytes="$(wc -c < "$fixture" | tr -d '[:space:]')"
expected_boxes=20
android_device_backup="$backup_root/android-installed-before.apk"
ios_app_backup="$backup_root/ios-installed-before.app"
ios_data_backup="$backup_root/ios-data-container-before"

mkdir -p "$android_out/final" "$ios_out/final" "$backup_root"
test ! -e "$android_device_backup"
test ! -e "$ios_app_backup"
test ! -e "$ios_data_backup"
[[ "$android_serial" == emulator-* ]]

adb -s "$android_serial" get-serialno > "$android_out/final/serial.txt"
adb -s "$android_serial" shell getprop ro.kernel.qemu > "$android_out/final/is-emulator.txt"
adb -s "$android_serial" shell getprop ro.build.version.sdk > "$android_out/final/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi > "$android_out/final/abi.txt"
test "$(tr -d '\r' < "$android_out/final/is-emulator.txt")" = "1"
test "$(tr -d '\r' < "$android_out/final/api-level.txt")" = "37"
test "$(tr -d '\r' < "$android_out/final/abi.txt")" = "arm64-v8a"
adb -s "$android_serial" shell wm size > "$android_out/final/screen-size.txt"
adb -s "$android_serial" shell wm density > "$android_out/final/density.txt"
adb -s "$android_serial" shell dumpsys package "$android_package" > "$android_out/final/package-before.txt"
adb -s "$android_serial" shell pidof "$android_package" > "$android_out/final/pid-before.txt" || true

android_package_paths="$(adb -s "$android_serial" shell pm path "$android_package" | tr -d '\r')"
printf '%s\n' "$android_package_paths" > "$android_out/final/installed-package-paths-before.txt"
android_base_path="$(printf '%s\n' "$android_package_paths" | sed -n 's/^package://p' | head -n 1)"
test -n "$android_base_path"
adb -s "$android_serial" pull "$android_base_path" "$android_device_backup" > "$android_out/final/pull-before.log" 2>&1

xcrun simctl list devices --json | python3 -c 'import json,sys; data=json.load(sys.stdin); devices=[device for group in data["devices"].values() for device in group if device["udid"]==sys.argv[1]]; assert len(devices)==1 and devices[0]["state"]=="Booted" and devices[0]["deviceTypeIdentifier"]=="com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro", devices; print(json.dumps(devices[0], sort_keys=True))' "$ios_udid" > "$ios_out/final/device-metadata.txt"
xcrun simctl getenv "$ios_udid" SIMULATOR_DEVICE_NAME > "$ios_out/final/device-name.txt"
ios_data_before="$(xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" data)"
ios_app_before="$(xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" app)"
test -d "$ios_data_before"
test -d "$ios_app_before"
ditto "$ios_data_before" "$ios_data_backup"
ditto "$ios_app_before" "$ios_app_backup"
printf '%s\n' "$ios_data_before" > "$ios_out/final/data-container-before.txt"
find "$ios_data_backup" -type f -exec shasum -a 256 {} \; \
  | sed "s|$ios_data_backup/||" | sort > "$ios_out/final/data-container-before.sha256"

restore_original_simulator_state() {
  restore_status="$?"
  trap - EXIT HUP INT TERM
  set +e

  if [[ -f "$android_device_backup" ]]; then
    adb -s "$android_serial" install -r "$android_device_backup" > "$android_out/final/restore-install.log" 2>&1
    adb -s "$android_serial" shell am force-stop "$android_package" > "$android_out/final/restore-force-stop.log" 2>&1
    adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" > "$android_out/final/restore-launch.log" 2>&1
  fi

  if [[ -d "$ios_app_backup" ]]; then
    xcrun simctl terminate "$ios_udid" "$ios_bundle_id" > "$ios_out/final/restore-terminate.log" 2>&1
    xcrun simctl install "$ios_udid" "$ios_app_backup" > "$ios_out/final/restore-install.log" 2>&1
    ios_data_after_restore="$(xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" data 2>/dev/null)"
    if [[ -d "$ios_data_backup" && -d "$ios_data_after_restore" ]]; then
      ditto "$ios_data_backup" "$ios_data_after_restore"
      printf '%s\n' "$ios_data_after_restore" > "$ios_out/final/data-container-after-restore.txt"
      find "$ios_data_after_restore" -type f -exec shasum -a 256 {} \; \
        | sed "s|$ios_data_after_restore/||" | sort > "$ios_out/final/data-container-after-restore.sha256"
    fi
    xcrun simctl launch "$ios_udid" "$ios_bundle_id" > "$ios_out/final/restore-launch.log" 2>&1
    ios_restore_pid="$(awk -F': ' '/dev\.spinon\.bootstrap:/ { print $2; exit }' "$ios_out/final/restore-launch.log")"
    if [[ -n "$ios_restore_pid" ]]; then
      printf '%s\n' "$ios_restore_pid" > "$ios_out/final/restore-pid.txt"
      xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
        --predicate 'process == "SpinonBootstrap" AND eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP_BOOT"' \
        > "$ios_out/final/restore-app.log" 2>&1
      rg -F "SpinonBootstrap[$ios_restore_pid:" "$ios_out/final/restore-app.log" \
        > "$ios_out/final/restore-current-process.log" 2>/dev/null
    fi
  fi

  exit "$restore_status"
}
trap restore_original_simulator_state EXIT HUP INT TERM

unzip -p "$apk" assets/index.html | shasum -a 256 | awk '{print $1}' > "$android_out/final/candidate-index.sha256"
test "$(cat "$android_out/final/candidate-index.sha256")" = "$fixture_sha"
test "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" = "$fixture_sha"
printf 'fixture_sha256=%s\nfixture_bytes=%s\nexpected_boxes=%s\n' \
  "$fixture_sha" "$fixture_bytes" "$expected_boxes" > "$evidence_dir/final-source-pass01.txt"

adb -s "$android_serial" install -r "$apk" > "$android_out/final/install.log" 2>&1
adb -s "$android_serial" shell am force-stop "$android_package" > "$android_out/final/force-stop.log" 2>&1
adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" > "$android_out/final/launch.log" 2>&1
android_pid=""
for attempt in $(seq 1 30); do
  android_pid="$(adb -s "$android_serial" shell pidof "$android_package" | tr -d '\r' || true)"
  if [[ -n "$android_pid" ]]; then break; fi
  sleep 1
done
test -n "$android_pid"
printf '%s\n' "$android_pid" > "$android_out/final/app-pid.txt"
for attempt in $(seq 1 45); do
  adb -s "$android_serial" logcat -d -v threadtime \
    | awk -v app_pid="$android_pid" '$3 == app_pid' > "$android_out/final/app-current-process.log"
  if rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=$expected_boxes" "$android_out/final/app-current-process.log" \
    && rg -q "SPINON_C0410_DRAW status=0 presented boxes=$expected_boxes" "$android_out/final/app-current-process.log"; then break; fi
  sleep 1
done
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_ASSET html_bytes=$fixture_bytes " "$android_out/final/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=$fixture_bytes" "$android_out/final/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=$expected_boxes" "$android_out/final/app-current-process.log"
rg -q "SPINON_C0410_DRAW status=0 presented boxes=$expected_boxes" "$android_out/final/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$android_out/final/app-current-process.log"
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$android_out/final/screenshot.png"
adb -s "$android_serial" shell dumpsys package "$android_package" > "$android_out/final/package-after.txt"

printf 'ios_udid=%s\n' "$ios_udid" > "$ios_out/final/selected-device.txt"
xcrun simctl terminate "$ios_udid" "$ios_bundle_id" > "$ios_out/final/terminate.log" 2>&1 || true
xcrun simctl install "$ios_udid" "$ios_app" > "$ios_out/final/install.log" 2>&1
xcrun simctl launch "$ios_udid" "$ios_bundle_id" > "$ios_out/final/launch.log" 2>&1
ios_pid="$(awk -F': ' '/dev\.spinon\.bootstrap:/ { print $2; exit }' "$ios_out/final/launch.log")"
test -n "$ios_pid"
printf '%s\n' "$ios_pid" > "$ios_out/final/app-pid.txt"
for attempt in $(seq 1 45); do
  xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
    --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT")' \
    > "$ios_out/final/app.log"
  rg -F "SpinonBootstrap[$ios_pid:" "$ios_out/final/app.log" > "$ios_out/final/app-current-process.log" || true
  if rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=$expected_boxes" "$ios_out/final/app-current-process.log" \
    && rg -q "SPINON_C0410_ENVIRONMENT status=0 .*layout=ready boxes=$expected_boxes" "$ios_out/final/app-current-process.log"; then break; fi
  sleep 1
done
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=$fixture_bytes " "$ios_out/final/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=$expected_boxes" "$ios_out/final/app-current-process.log"
rg -q "SPINON_C0410_ENVIRONMENT status=0 .*layout=ready boxes=$expected_boxes" "$ios_out/final/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$ios_out/final/app-current-process.log"
sleep 1
xcrun simctl io "$ios_udid" screenshot "$ios_out/final/screenshot.png" > "$ios_out/final/screenshot.log" 2>&1
xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" data > "$ios_out/final/data-container-after-candidate.txt"
ios_installed_app="$(xcrun simctl get_app_container "$ios_udid" "$ios_bundle_id" app)"
shasum -a 256 "$ios_installed_app/SpinonBootstrap" "$ios_installed_app/index.html" \
  "$android_out/final/screenshot.png" "$ios_out/final/screenshot.png" \
  > "$evidence_dir/candidate-capture-pass01.sha256"
printf 'Both simulators reached layout-ready for the same 20-box Root HTML fixture; the EXIT handler restores the previously installed apps and iOS app data.\n'
