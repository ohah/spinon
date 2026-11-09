#!/usr/bin/env bash
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(git -C "$script_directory" rev-parse --show-toplevel)"
android_root_html="$repo_root/platforms/android/app/src/main/assets/root-html-default-rendering.html"
ios_root_html="$repo_root/platforms/ios/Resources/root-html-default-rendering.html"
android_sdk="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
adb="$android_sdk/platform-tools/adb"
emulator="$android_sdk/emulator/emulator"
avd_name="spinon_api37_2_presenttime_probe_20261018"
android_serial="emulator-5554"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
android_source_backup="$(mktemp /tmp/spinon-c114-auto-rows-android.XXXXXX)"
ios_source_backup="$(mktemp /tmp/spinon-c114-auto-rows-ios.XXXXXX)"
android_pid=""
android_started=0
ios_started=0
temporary_source_hash=""

archive_logs() {
  local raw_log raw_name manifest="$script_directory/raw-log-sha256.txt"
  : > "$manifest"
  while IFS= read -r -d '' raw_log; do
    raw_name="${raw_log##*/}"
    [[ "$raw_name" == "${manifest##*/}" ]] && continue
    raw_hash="$(shasum -a 256 "$raw_log" | awk '{print $1}')"
    printf '%s  %s.gz\n' "$raw_hash" "$raw_name" >> "$manifest"
    gzip -n -f "$raw_log"
  done < <(find "$script_directory" -maxdepth 1 -type f \( -name '*.log' -o -name '*.txt' \) -print0 | sort -z)
}

restore_and_stop() {
  result=$?
  trap - EXIT
  if [[ -n "$temporary_source_hash" ]]; then
    restore_source() {
      source_path=$1
      backup_path=$2
      label=$3
      current_hash="$(shasum -a 256 "$source_path" | awk '{print $1}')"
      backup_hash="$(shasum -a 256 "$backup_path" | awk '{print $1}')"
      if [[ "$current_hash" == "$temporary_source_hash" ]]; then
        cp "$backup_path" "$source_path"
        printf '%s_restored_sha256=%s\n' "$label" "$(shasum -a 256 "$source_path" | awk '{print $1}')" \
          >> "$script_directory/source-restore.txt"
        rm -f "$backup_path"
      elif [[ "$current_hash" == "$backup_hash" ]]; then
        printf '%s_unchanged_sha256=%s\n' "$label" "$current_hash" \
          >> "$script_directory/source-restore.txt"
        rm -f "$backup_path"
      else
        printf '%s_changed_during_smoke=%s\n%s_backup=%s\n' \
          "$label" "$current_hash" "$label" "$backup_path" \
          >> "$script_directory/source-restore.txt"
        result=1
      fi
    }
    : > "$script_directory/source-restore.txt"
    restore_source "$android_root_html" "$android_source_backup" android_root_html
    restore_source "$ios_root_html" "$ios_source_backup" ios_root_html
  else
    rm -f "$android_source_backup" "$ios_source_backup"
  fi
  if (( ios_started )); then
    xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
    xcrun simctl shutdown "$ios_udid" >/dev/null 2>&1 || true
  fi
  if (( android_started )); then
    "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
    "$adb" -s "$android_serial" emu kill >/dev/null 2>&1 || true
  fi
  if [[ -n "$android_pid" ]]; then
    wait "$android_pid" >/dev/null 2>&1 || true
  fi
  "$adb" devices -l > "$script_directory/post-cleanup-android-devices.txt" 2>&1 || true
  xcrun simctl list devices booted > "$script_directory/post-cleanup-ios-devices.txt" 2>&1 || true
  archive_logs || result=1
  exit "$result"
}
trap restore_and_stop EXIT

if [[ ! -x "$adb" || ! -x "$emulator" ]]; then
  echo "Android SDK emulator/adb not found: $android_sdk" >&2
  exit 1
fi
if [[ ! -f "$android_root_html" || ! -f "$ios_root_html" ]]; then
  echo "Default Root HTML render asset is missing." >&2
  exit 1
fi
if [[ -n "$("$adb" devices | awk 'NR > 1 && $2 == "device" { print $1 }')" ]]; then
  echo "Android device already connected; refusing to use a physical or pre-existing device." >&2
  exit 1
fi
if xcrun simctl list devices booted | grep -q '(Booted)'; then
  echo "An iOS simulator is already booted; refusing to take it over." >&2
  exit 1
fi
if ! "$emulator" -list-avds | grep -Fxq "$avd_name"; then
  echo "Required existing Android AVD is not registered: $avd_name" >&2
  exit 1
fi
if ! xcrun simctl list devices available | grep -Fq "$ios_udid"; then
  echo "Required iPhone 17 Pro simulator is unavailable: $ios_udid" >&2
  exit 1
fi
if ! xcrun simctl list runtimes | grep -Fq 'iOS 26.2'; then
  echo "Required iOS 26.2 runtime is unavailable." >&2
  exit 1
fi
if [[ "$(shasum -a 256 "$android_root_html" | awk '{print $1}')" != \
      "38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85" || \
      "$(shasum -a 256 "$ios_root_html" | awk '{print $1}')" != \
      "38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85" ]]; then
  echo "Default Root HTML assets differ from the known clean source; refusing to replace them." >&2
  exit 1
fi

cp "$android_root_html" "$android_source_backup"
cp "$ios_root_html" "$ios_source_backup"
printf 'avd=%s\nios_udid=%s\nandroid_asset_sha256=%s\nios_asset_sha256=%s\n' \
  "$avd_name" "$ios_udid" \
  "$(shasum -a 256 "$android_source_backup" | awk '{print $1}')" \
  "$(shasum -a 256 "$ios_source_backup" | awk '{print $1}')" \
  > "$script_directory/preflight.txt"

"$emulator" -avd "$avd_name" -port 5554 -no-window -no-audio -gpu swiftshader_indirect \
  -no-boot-anim -no-snapshot-save > "$script_directory/android-emulator.log" 2>&1 &
android_pid=$!
android_started=1
"$adb" -s "$android_serial" wait-for-device
for attempt in {1..90}; do
  booted="$("$adb" -s "$android_serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')"
  [[ "$booted" == "1" ]] && break
  sleep 2
done
if [[ "$("$adb" -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" != "1" ]]; then
  echo "Android AVD did not finish booting." >&2
  exit 1
fi
android_api="$("$adb" -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
android_abi="$("$adb" -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
if [[ "$android_api" != "37" || "$android_abi" != "arm64-v8a" ]]; then
  echo "Expected Android API 37 ARM64, got API $android_api / ABI $android_abi." >&2
  exit 1
fi
"$adb" -s "$android_serial" shell cmd uimode night no >/dev/null 2>&1 || true

ios_started=1
xcrun simctl boot "$ios_udid"
xcrun simctl bootstatus "$ios_udid" -b
xcrun simctl ui "$ios_udid" appearance light

for source_path in "$script_directory"/scene-*.html; do
  scene_name="$(basename "$source_path" .html)"
  temporary_source_hash="$(shasum -a 256 "$source_path" | awk '{print $1}')"
  cp "$source_path" "$android_root_html"
  cp "$source_path" "$ios_root_html"

  android_build_log="$script_directory/android-build-${scene_name}.log"
  if ! (cd "$repo_root" && mise exec -- bun run build:android) > "$android_build_log" 2>&1; then
    tail -n 100 "$android_build_log" >&2
    exit 1
  fi
  if [[ ! -f "$apk" ]]; then
    echo "Android APK was not produced: $apk" >&2
    exit 1
  fi
  "$adb" -s "$android_serial" install -r "$apk" \
    > "$script_directory/android-install-${scene_name}.log"
  "$adb" -s "$android_serial" logcat -c
  "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  "$adb" -s "$android_serial" shell am start -W -n "$bundle_id/.MainActivity" \
    --ez spinon_c0412_root_html_default_render true \
    > "$script_directory/android-launch-${scene_name}.log" 2>&1
  android_runtime_log="$script_directory/android-runtime-${scene_name}.log"
  android_ready=0
  for attempt in {1..15}; do
    sleep 2
    "$adb" -s "$android_serial" logcat -d -v time -s SpinonBootstrap:I > "$android_runtime_log"
    if grep -Fq "sha256=$temporary_source_hash" "$android_runtime_log" \
      && grep -Fq 'layout=ready' "$android_runtime_log" \
      && grep -Eq 'SPINON_C0410_DRAW( generation=[0-9]+)? status=0 presented' "$android_runtime_log"; then
      android_ready=1
      break
    fi
  done
  if (( ! android_ready )); then
    echo "Android default Root HTML route did not publish the expected source and draw: $scene_name" >&2
    exit 1
  fi
  "$adb" -s "$android_serial" exec-out screencap -p \
    > "$script_directory/android-${scene_name}.png"
  "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true

  ios_build_log="$script_directory/ios-build-${scene_name}.log"
  if ! (cd "$repo_root" && mise exec -- bun run build:ios-sim) > "$ios_build_log" 2>&1; then
    tail -n 100 "$ios_build_log" >&2
    exit 1
  fi
  if [[ ! -d "$ios_app" ]]; then
    echo "iOS simulator app was not produced: $ios_app" >&2
    exit 1
  fi
  xcrun simctl install "$ios_udid" "$ios_app" > "$script_directory/ios-install-${scene_name}.log"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl launch --terminate-running-process "$ios_udid" "$bundle_id" \
    --spinon-c0412-root-html-default-render \
    > "$script_directory/ios-launch-${scene_name}.log"
  ios_runtime_log="$script_directory/ios-runtime-${scene_name}.log"
  ios_ready=0
  for attempt in {1..15}; do
    sleep 2
    xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
      --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' \
      > "$ios_runtime_log" 2>&1 || true
    if grep -Fq "sha256=$temporary_source_hash" "$ios_runtime_log" \
      && grep -Fq 'layout=ready' "$ios_runtime_log" \
      && grep -Eq 'SPINON_C0410_DRAW( generation=[0-9]+)? status=0 presented' "$ios_runtime_log"; then
      ios_ready=1
      break
    fi
  done
  if (( ! ios_ready )); then
    echo "iOS default Root HTML route did not publish the expected source and draw: $scene_name" >&2
    exit 1
  fi
  xcrun simctl io "$ios_udid" screenshot "$script_directory/ios-${scene_name}.png" \
    > "$script_directory/ios-screenshot-${scene_name}.log"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  printf '%s: Android and iOS default Root HTML route passed (%s)\n' \
    "$scene_name" "$temporary_source_hash"
done

printf 'android_api=%s\nandroid_abi=%s\n' \
  "$android_api" "$android_abi" > "$script_directory/android-device.txt"
printf 'ios_runtime=iOS 26.2\nios_device=iPhone 17 Pro\n' \
  > "$script_directory/ios-device.txt"
