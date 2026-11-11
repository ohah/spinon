#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
android_serial="emulator-5554"
adb_bin="/Users/yoonhb/Library/Android/sdk/platform-tools/adb"
apksigner_bin="/Users/yoonhb/Library/Android/sdk/build-tools/37.0.0/apksigner"
ios_bundle="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_initial_appearance="$(xcrun simctl ui "$ios_udid" appearance)"
android_initial_night="$("$adb_bin" -s "$android_serial" shell cmd uimode night | awk '{print $3}')"
backup_dir=""
ios_previous_app=""
android_previous_apk=""
android_previous_apk_sha256=""
ios_previous_data=""
ios_data_hash_before=""
android_data_archive=""
android_data_hash_before=""
capture_mutation_started=0

ios_data_hash() {
  local data_path="$1"
  (
    cd "$data_path"
    find . -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}'
  )
}

android_data_hash() {
  local manifest
  manifest="$("$adb_bin" -s "$android_serial" exec-out run-as dev.spinon.bootstrap sh -c \
    'cd /data/user/0/dev.spinon.bootstrap && find . -type f -exec sha256sum {} + | LC_ALL=C sort')"
  if [[ -n "$manifest" ]] && ! printf '%s\n' "$manifest" | rg -q '^[[:xdigit:]]{64}[[:space:]]'; then
    return 1
  fi
  printf '%s\n' "$manifest" | shasum -a 256 | awk '{print $1}'
}

cleanup() {
  local status=$?
  local cleanup_failed=0
  trap - EXIT
  xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap >/dev/null 2>&1 || true
  "$adb_bin" -s "$android_serial" shell am force-stop dev.spinon.bootstrap >/dev/null 2>&1 || true
  if [[ "$capture_mutation_started" == "1" ]]; then
    if [[ -n "$ios_previous_app" ]]; then
      xcrun simctl install "$ios_udid" "$backup_dir/SpinonBootstrap.app" \
        > "$evidence_dir/ios-app-restore.log" 2>&1 || { status=1; cleanup_failed=1; }
    else
      xcrun simctl uninstall "$ios_udid" dev.spinon.bootstrap \
        > "$evidence_dir/ios-app-restore.log" 2>&1 || { status=1; cleanup_failed=1; }
    fi
    if [[ -n "$android_previous_apk" ]]; then
      "$adb_bin" -s "$android_serial" install -r "$backup_dir/android-previous-app.apk" \
        > "$evidence_dir/android-app-restore.log" 2>&1 || { status=1; cleanup_failed=1; }
    else
      "$adb_bin" -s "$android_serial" uninstall dev.spinon.bootstrap \
        > "$evidence_dir/android-app-restore.log" 2>&1 || { status=1; cleanup_failed=1; }
    fi

    {
      current_ios_data="$(xcrun simctl get_app_container "$ios_udid" dev.spinon.bootstrap data 2>/dev/null || true)"
      if [[ -n "$ios_previous_data" && -n "$current_ios_data" ]]; then
        ios_data_hash_after="$(ios_data_hash "$current_ios_data")"
        printf 'ios_before=%s\nios_after=%s\n' "$ios_data_hash_before" "$ios_data_hash_after"
        if [[ "$ios_data_hash_after" != "$ios_data_hash_before" ]]; then
          rm -rf "$current_ios_data"
          ditto "$backup_dir/ios-previous-data" "$current_ios_data"
          ios_data_hash_restored="$(ios_data_hash "$current_ios_data")"
          printf 'ios_restored=%s\n' "$ios_data_hash_restored"
          [[ "$ios_data_hash_restored" == "$ios_data_hash_before" ]] || { status=1; cleanup_failed=1; }
        fi
      fi

      if [[ -n "$android_data_archive" ]]; then
        android_data_hash_after="$(android_data_hash)"
        printf 'android_before=%s\nandroid_after=%s\n' "$android_data_hash_before" "$android_data_hash_after"
        if [[ "$android_data_hash_after" != "$android_data_hash_before" ]]; then
          "$adb_bin" -s "$android_serial" shell -T run-as dev.spinon.bootstrap \
            sh -c 'cd /data/user/0/dev.spinon.bootstrap && find . -mindepth 1 -maxdepth 1 -exec rm -rf {} +' \
            || { status=1; cleanup_failed=1; }
          "$adb_bin" -s "$android_serial" shell -T run-as dev.spinon.bootstrap \
            tar -xf - -C /data/user/0/dev.spinon.bootstrap < "$android_data_archive" || { status=1; cleanup_failed=1; }
          android_data_hash_restored="$(android_data_hash)"
          printf 'android_restored=%s\n' "$android_data_hash_restored"
          [[ "$android_data_hash_restored" == "$android_data_hash_before" ]] || { status=1; cleanup_failed=1; }
        fi
      fi
    } > "$evidence_dir/app-data-preservation.log" 2>&1 || { status=1; cleanup_failed=1; }
  fi
  xcrun simctl ui "$ios_udid" appearance "$ios_initial_appearance" >/dev/null 2>&1 || true
  "$adb_bin" -s "$android_serial" shell cmd uimode night "$android_initial_night" >/dev/null 2>&1 || true
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
    "$adb_bin" devices -l
    pgrep -alf 'SpinonBootstrap|emulator -avd spinon_c114_five_fit_20261111' || true
  } > "$evidence_dir/cleanup.log"
  if [[ -n "$backup_dir" ]]; then
    if [[ "$cleanup_failed" == "0" ]]; then
      rm -rf "$backup_dir"
    else
      printf 'backup_preserved=%s\n' "$backup_dir" >> "$evidence_dir/cleanup.log"
    fi
  fi
  exit "$status"
}
trap cleanup EXIT

test -d "$ios_bundle"
test -f "$android_apk"
test "$(xcrun simctl list devices booted | rg -c "$ios_udid")" = "1"
test "$("$adb_bin" -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" = "1"

backup_dir="$(mktemp -d /private/tmp/spinon-c114-five-fit-capture.XXXXXX)"
ios_previous_app="$(xcrun simctl get_app_container "$ios_udid" dev.spinon.bootstrap app 2>/dev/null || true)"
if [[ -n "$ios_previous_app" ]]; then
  ditto "$ios_previous_app" "$backup_dir/SpinonBootstrap.app"
  ios_previous_data="$(xcrun simctl get_app_container "$ios_udid" dev.spinon.bootstrap data)"
  ditto "$ios_previous_data" "$backup_dir/ios-previous-data"
  ios_data_hash_before="$(ios_data_hash "$ios_previous_data")"
fi
android_previous_apk_path="$("$adb_bin" -s "$android_serial" shell pm path dev.spinon.bootstrap 2>/dev/null | sed -n 's/^package://p' | tr -d '\r' | head -n 1 || true)"
if [[ -n "$android_previous_apk_path" ]]; then
  "$adb_bin" -s "$android_serial" pull "$android_previous_apk_path" "$backup_dir/android-previous-app.apk" \
    > "$evidence_dir/android-app-backup.log"
  android_remote_apk_sha256="$("$adb_bin" -s "$android_serial" exec-out sha256sum "$android_previous_apk_path" | awk '{print $1}' | tr -d '\r')"
  android_previous_apk_sha256="$(shasum -a 256 "$backup_dir/android-previous-app.apk" | awk '{print $1}')"
  {
    printf 'remote_sha256=%s\nlocal_sha256=%s\n' "$android_remote_apk_sha256" "$android_previous_apk_sha256"
    test "$android_remote_apk_sha256" = "$android_previous_apk_sha256"
    "$apksigner_bin" verify --verbose "$backup_dir/android-previous-app.apk"
  } > "$evidence_dir/android-app-backup-integrity.log" 2>&1
  android_previous_apk="$backup_dir/android-previous-app.apk"
  android_data_archive="$backup_dir/android-previous-data.tar"
  "$adb_bin" -s "$android_serial" exec-out run-as dev.spinon.bootstrap \
    tar -cf - -C /data/user/0/dev.spinon.bootstrap . > "$android_data_archive"
  tar -tf "$android_data_archive" > /dev/null
  android_data_hash_before="$(android_data_hash)"
fi
capture_mutation_started=1

{
  xcrun simctl list devices booted
  xcrun simctl ui "$ios_udid" appearance
  "$adb_bin" -s "$android_serial" shell getprop ro.boot.qemu.avd_name
  "$adb_bin" -s "$android_serial" shell getprop ro.build.version.sdk
  "$adb_bin" -s "$android_serial" shell getprop ro.product.cpu.abi
  "$adb_bin" -s "$android_serial" shell wm size
  "$adb_bin" -s "$android_serial" shell wm density
} > "$evidence_dir/device-info.log"

xcrun simctl install "$ios_udid" "$ios_bundle"
"$adb_bin" -s "$android_serial" install -r "$android_apk"

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
  > "$evidence_dir/android-api37_1-light.png"

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
  > "$evidence_dir/android-api37_1-dark.png"
