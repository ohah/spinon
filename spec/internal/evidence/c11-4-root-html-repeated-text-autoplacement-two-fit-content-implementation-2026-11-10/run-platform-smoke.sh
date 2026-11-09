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
ios_runtime="com.apple.CoreSimulator.SimRuntime.iOS-26-2"
ios_device_type="com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
baseline_hash="38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85"
run_directory="$(mktemp -d /tmp/spinon-c1144-platform-smoke.XXXXXX)"
android_data="$run_directory/android-data"
android_backup="$run_directory/android-root-html.before"
ios_backup="$run_directory/ios-root-html.before"
android_pid=""
ios_udid=""
ios_created=0
android_started=0
source_hash=""
cleanup() {
  result=$?
  trap - EXIT
  if [[ -n "$source_hash" ]]; then
    for spec in "android:$android_root_html:$android_backup" "ios:$ios_root_html:$ios_backup"; do
      IFS=: read -r label source backup <<< "$spec"
      current="$(shasum -a 256 "$source" | awk '{print $1}')"
      saved="$(shasum -a 256 "$backup" | awk '{print $1}')"
      if [[ "$current" == "$source_hash" ]]; then
        cp "$backup" "$source"
        printf '%s_root_html=restored:%s\n' "$label" "$(shasum -a 256 "$source" | awk '{print $1}')" >> "$script_directory/cleanup-state.txt"
        rm -f "$backup"
      elif [[ "$current" == "$saved" ]]; then
        printf '%s_root_html=unchanged:%s\n' "$label" "$current" >> "$script_directory/cleanup-state.txt"
        rm -f "$backup"
      else
        printf '%s_root_html=changed-during-run:%s backup=%s\n' "$label" "$current" "$backup" >> "$script_directory/cleanup-state.txt"
        result=1
      fi
    done
  fi
  if (( ios_created )); then
    xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
    xcrun simctl shutdown "$ios_udid" >/dev/null 2>&1 || true
    xcrun simctl delete "$ios_udid" >> "$script_directory/cleanup-state.txt" 2>&1 || result=1
    if xcrun simctl list devices | grep -Fq "$ios_udid"; then
      printf 'ios_profile=still-present:%s\n' "$ios_udid" >> "$script_directory/cleanup-state.txt"
      result=1
    else
      printf 'ios_profile=deleted:%s\n' "$ios_udid" >> "$script_directory/cleanup-state.txt"
    fi
  fi
  if (( android_started )); then
    "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
    "$adb" -s "$android_serial" emu kill >/dev/null 2>&1 || true
  fi
  if [[ -n "$android_pid" ]]; then wait "$android_pid" >/dev/null 2>&1 || true; fi
  "$adb" devices -l > "$script_directory/post-cleanup-android-devices.txt" 2>&1 || true
  xcrun simctl list devices booted > "$script_directory/post-cleanup-ios-devices.txt" 2>&1 || true
  if [[ -d "$run_directory" && "$run_directory" == /tmp/spinon-c1144-platform-smoke.* ]]; then
    rm -rf -- "$run_directory"
    printf 'android_temporary_data=deleted\n' >> "$script_directory/cleanup-state.txt"
  else
    printf 'android_temporary_data=preserved-unexpected-path:%s\n' "$run_directory" >> "$script_directory/cleanup-state.txt"
    result=1
  fi
  exit "$result"
}
trap cleanup EXIT
[[ -x "$adb" && -x "$emulator" ]] || { echo "Android SDK emulator/adb missing: $android_sdk" >&2; exit 1; }
[[ -f "$android_root_html" && -f "$ios_root_html" ]] || { echo "Default Root HTML assets are missing." >&2; exit 1; }
[[ -z "$("$adb" devices | awk 'NR > 1 && $2 == "device" { print $1 }')" ]] || { echo "Android device is already connected." >&2; exit 1; }
! xcrun simctl list devices booted | grep -q '(Booted)' || { echo "An iOS simulator is already booted." >&2; exit 1; }
"$emulator" -list-avds | grep -Fxq "$avd_name" || { echo "Android AVD is not registered: $avd_name" >&2; exit 1; }
xcrun simctl list runtimes | grep -Fq 'iOS 26.2' || { echo "iOS 26.2 runtime is unavailable." >&2; exit 1; }
[[ "$(shasum -a 256 "$android_root_html" | awk '{print $1}')" == "$baseline_hash" && "$(shasum -a 256 "$ios_root_html" | awk '{print $1}')" == "$baseline_hash" ]] || { echo "Root HTML app assets differ from expected clean baseline." >&2; exit 1; }
mkdir -p "$android_data"
cp "$android_root_html" "$android_backup"
cp "$ios_root_html" "$ios_backup"
printf 'avd=%s\nios_runtime=iOS 26.2\nios_device_type=iPhone 17 Pro\napp_asset_sha256=%s\n' "$avd_name" "$baseline_hash" "$baseline_hash" > "$script_directory/preflight.txt"
for file in userdata-qemu.img userdata-qemu.img.qcow2 cache.img cache.img.qcow2; do
  path="$HOME/.android/avd/$avd_name.avd/$file"
  [[ ! -e "$path" ]] || stat -f '%N size=%z mtime=%m' "$path" >> "$script_directory/preflight-avd-state.txt"
done
ios_udid="$(xcrun simctl create "Spinon C11.4.4 Temporary" "$ios_device_type" "$ios_runtime")"
ios_created=1
printf 'ios_udid=%s\n' "$ios_udid" >> "$script_directory/preflight.txt"
"$emulator" -avd "$avd_name" -port 5554 -read-only -no-snapshot-load -no-snapshot-save -no-snapstorage -no-window -no-audio -gpu swangle -no-boot-anim -accel on -datadir "$android_data" -data "$android_data/userdata-qemu.img" -cache "$android_data/cache.img" > "$script_directory/android-emulator.log" 2>&1 &
android_pid=$!
android_started=1
connected=0
for attempt in {1..120}; do
  kill -0 "$android_pid" 2>/dev/null || { tail -n 100 "$script_directory/android-emulator.log" >&2; echo "Android emulator exited before connecting." >&2; exit 1; }
  if [[ "$("$adb" -s "$android_serial" get-state 2>/dev/null || true)" == "device" ]]; then connected=1; break; fi
  sleep 2
done
(( connected )) || { echo "Android emulator did not connect in time." >&2; exit 1; }
booted=0
for attempt in {1..90}; do
  if [[ "$("$adb" -s "$android_serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; then booted=1; break; fi
  sleep 2
done
(( booted )) || { echo "Android AVD did not finish booting." >&2; exit 1; }
android_api="$("$adb" -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
android_abi="$("$adb" -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
[[ "$android_api" == "37" && "$android_abi" == "arm64-v8a" ]] || { echo "Expected API 37 ARM64; got $android_api $android_abi." >&2; exit 1; }
"$adb" -s "$android_serial" shell cmd uimode night no >/dev/null 2>&1 || true
xcrun simctl boot "$ios_udid"
xcrun simctl bootstatus "$ios_udid" -b
xcrun simctl ui "$ios_udid" appearance light
for source_path in "$script_directory"/scenes/*.html; do
  scene="$(basename "$source_path" .html)"
  source_hash="$(shasum -a 256 "$source_path" | awk '{print $1}')"
  cp "$source_path" "$android_root_html"
  cp "$source_path" "$ios_root_html"
  android_log="$script_directory/android-build-$scene.log"
  (cd "$repo_root" && mise exec -- bun run build:android) > "$android_log" 2>&1 || { tail -n 100 "$android_log" >&2; exit 1; }
  [[ -f "$apk" ]] || { echo "Android APK missing: $apk" >&2; exit 1; }
  "$adb" -s "$android_serial" install -r "$apk" > "$script_directory/android-install-$scene.log"
  "$adb" -s "$android_serial" logcat -c
  "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  "$adb" -s "$android_serial" shell am start -W -n "$bundle_id/.MainActivity" --ez spinon_c0412_root_html_default_render true > "$script_directory/android-launch-$scene.log" 2>&1
  android_ready=0
  for attempt in {1..15}; do
    sleep 2
    "$adb" -s "$android_serial" logcat -d -v time -s SpinonBootstrap:I > "$script_directory/android-runtime-$scene.log"
    if grep -Fq "sha256=$source_hash" "$script_directory/android-runtime-$scene.log" && grep -Fq 'layout=ready' "$script_directory/android-runtime-$scene.log" && grep -Eq 'SPINON_C0410_DRAW( generation=[0-9]+)? status=0 presented' "$script_directory/android-runtime-$scene.log"; then android_ready=1; break; fi
  done
  (( android_ready )) || { echo "Android route/draw failed: $scene" >&2; exit 1; }
  "$adb" -s "$android_serial" exec-out screencap -p > "$script_directory/android-$scene.png"
  "$adb" -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  ios_log="$script_directory/ios-build-$scene.log"
  (cd "$repo_root" && mise exec -- bun run build:ios-sim) > "$ios_log" 2>&1 || { tail -n 100 "$ios_log" >&2; exit 1; }
  [[ -d "$ios_app" ]] || { echo "iOS simulator app missing: $ios_app" >&2; exit 1; }
  xcrun simctl install "$ios_udid" "$ios_app" > /dev/null
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl launch --terminate-running-process "$ios_udid" "$bundle_id" --spinon-c0412-root-html-default-render > "$script_directory/ios-launch-$scene.log"
  ios_ready=0
  for attempt in {1..15}; do
    sleep 2
    xcrun simctl spawn "$ios_udid" log show --last 3m --style compact --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE" OR eventMessage CONTAINS "SPINON_C0410_ENVIRONMENT" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' > "$script_directory/ios-runtime-$scene.log" 2>&1 || true
    if grep -Fq "sha256=$source_hash" "$script_directory/ios-runtime-$scene.log" && grep -Fq 'layout=ready' "$script_directory/ios-runtime-$scene.log" && grep -Eq 'SPINON_C0410_DRAW( generation=[0-9]+)? status=0 presented' "$script_directory/ios-runtime-$scene.log"; then ios_ready=1; break; fi
  done
  (( ios_ready )) || { echo "iOS route/draw failed: $scene" >&2; exit 1; }
  xcrun simctl io "$ios_udid" screenshot "$script_directory/ios-$scene.png" > /dev/null
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  printf '%s: Android+iOS route/draw passed source=%s\n' "$scene" "$source_hash"
done
printf 'api=%s\nabi=%s\n' "$android_api" "$android_abi" > "$script_directory/android-device.txt"
printf 'runtime=iOS 26.2\ndevice=iPhone 17 Pro temporary\nudid=%s\n' "$ios_udid" > "$script_directory/ios-device.txt"
for file in userdata-qemu.img userdata-qemu.img.qcow2 cache.img cache.img.qcow2; do
  path="$HOME/.android/avd/$avd_name.avd/$file"
  [[ ! -e "$path" ]] || stat -f '%N size=%z mtime=%m' "$path" >> "$script_directory/post-run-avd-state.txt"
done
