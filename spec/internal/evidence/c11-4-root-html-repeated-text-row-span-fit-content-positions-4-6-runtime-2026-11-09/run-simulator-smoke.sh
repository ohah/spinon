#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
source_index="$repo_root/examples/bootstrap/index.html"
source_root_app="$repo_root/examples/bootstrap/root-app.js"
scene_dir="$evidence_dir/sources"
android_sdk="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
android_avd="${SPINON_ANDROID_AVD:-spinon_c114_percentage_max_20261023}"
android_serial=""
android_pid=""
android_emulator_pid=""
android_started_here=0
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
ios_started_here=0
package="dev.spinon.bootstrap"
app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
original_index_sha="e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9"
original_root_app_sha="b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32"
platforms="${SPINON_SIMULATOR_PLATFORMS:-both}"
if [[ "$platforms" != "both" && "$platforms" != "android" && "$platforms" != "ios" ]]; then
  printf 'SPINON_SIMULATOR_PLATFORMS must be both, android, or ios\n' >&2
  exit 2
fi

mkdir -p "$evidence_dir/android" "$evidence_dir/ios"
test "$(shasum -a 256 "$source_index" | awk '{print $1}')" = "$original_index_sha"
test "$(shasum -a 256 "$source_root_app" | awk '{print $1}')" = "$original_root_app_sha"
test "$(git -C "$repo_root" show HEAD:examples/bootstrap/index.html | shasum -a 256 | awk '{print $1}')" = "$original_index_sha"
test "$(git -C "$repo_root" show HEAD:examples/bootstrap/root-app.js | shasum -a 256 | awk '{print $1}')" = "$original_root_app_sha"

restore_and_stop() {
  local status=$?
  local ios_stopped="not-started"
  local android_stopped="not-started"
  set +e
  git -C "$repo_root" show HEAD:examples/bootstrap/index.html > "$source_index"
  git -C "$repo_root" show HEAD:examples/bootstrap/root-app.js > "$source_root_app"
  if [[ "$(shasum -a 256 "$source_index" | awk '{print $1}')" != "$original_index_sha" \
     || "$(shasum -a 256 "$source_root_app" | awk '{print $1}')" != "$original_root_app_sha" ]]; then
    printf 'bootstrap source restoration failed\n' >&2
    status=1
  fi
  if (( ios_started_here )); then
    xcrun simctl shutdown "$ios_udid" >/dev/null 2>&1
    if xcrun simctl list devices booted | rg -q "$ios_udid"; then
      ios_stopped="false"
      status=1
    else
      ios_stopped="true"
    fi
  fi
  if (( android_started_here )); then
    if [[ -n "$android_serial" ]]; then
      "$android_sdk/platform-tools/adb" -s "$android_serial" emu kill >/dev/null 2>&1
    elif [[ -n "$android_emulator_pid" ]]; then
      kill -TERM "$android_emulator_pid" >/dev/null 2>&1
    fi
    if [[ -n "$android_emulator_pid" ]]; then
      wait "$android_emulator_pid" >/dev/null 2>&1
    fi
    if [[ -n "$android_serial" ]] \
      && "$android_sdk/platform-tools/adb" devices | rg -q "^$android_serial[[:space:]]+device$"; then
      android_stopped="false"
      status=1
    else
      android_stopped="true"
    fi
  fi
  printf 'ios_shutdown_after_run=%s\nandroid_shutdown_after_run=%s\n' \
    "$ios_stopped" "$android_stopped" > "$evidence_dir/simulator-lifecycle.txt"
  return "$status"
}
trap restore_and_stop EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

if [[ "$platforms" != "ios" ]] \
  && "$android_sdk/platform-tools/adb" devices | rg -q '^emulator-[0-9]+[[:space:]]+device$'; then
  printf 'an Android emulator is already attached; refusing to take it over\n' >&2
  exit 2
fi
if [[ "$platforms" != "android" ]] \
  && xcrun simctl list devices booted | rg -q "$ios_udid"; then
  printf 'the selected iPhone simulator is already booted; refusing to take it over\n' >&2
  exit 2
fi

if [[ "$platforms" != "ios" ]]; then
  "$android_sdk/emulator/emulator" -avd "$android_avd" -gpu swiftshader -no-snapshot \
    -no-boot-anim -no-window -no-audio \
    > "$evidence_dir/android/emulator.log" 2>&1 &
  android_emulator_pid=$!
  android_started_here=1
  for attempt in $(seq 1 120); do
    android_serial="$("$android_sdk/platform-tools/adb" devices \
      | awk '$1 ~ /^emulator-/ && $2 == "device" { print $1; exit }')"
    if [[ -n "$android_serial" ]]; then break; fi
    sleep 1
  done
  test -n "$android_serial"
  for attempt in $(seq 1 120); do
    if [[ "$("$android_sdk/platform-tools/adb" -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" = "1" ]]; then
      break
    fi
    sleep 1
  done
  test "$("$android_sdk/platform-tools/adb" -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" = "1"
  printf 'serial=%s\navd=%s\n' "$android_serial" "$android_avd" > "$evidence_dir/android/device.txt"
  printf 'release=' >> "$evidence_dir/android/device.txt"
  "$android_sdk/platform-tools/adb" -s "$android_serial" shell getprop ro.build.version.release | tr -d '\r' >> "$evidence_dir/android/device.txt"
  printf 'api_level=' >> "$evidence_dir/android/device.txt"
  "$android_sdk/platform-tools/adb" -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r' >> "$evidence_dir/android/device.txt"
  printf 'abi=' >> "$evidence_dir/android/device.txt"
  "$android_sdk/platform-tools/adb" -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r' >> "$evidence_dir/android/device.txt"
  printf 'screen_size=' >> "$evidence_dir/android/device.txt"
  "$android_sdk/platform-tools/adb" -s "$android_serial" shell wm size | tr -d '\r' >> "$evidence_dir/android/device.txt"
  printf 'density=' >> "$evidence_dir/android/device.txt"
  "$android_sdk/platform-tools/adb" -s "$android_serial" shell wm density | tr -d '\r' >> "$evidence_dir/android/device.txt"
fi

if [[ "$platforms" != "android" ]]; then
  xcrun simctl boot "$ios_udid"
  ios_started_here=1
  xcrun simctl bootstatus "$ios_udid" -b > "$evidence_dir/ios/bootstatus.log" 2>&1
  xcrun simctl list devices booted > "$evidence_dir/ios/booted-devices.txt"
  xcrun simctl getenv "$ios_udid" SIMULATOR_DEVICE_NAME > "$evidence_dir/ios/device-name.txt"
fi

for scene_file in "$scene_dir"/fit-*.html; do
  name="$(basename "$scene_file" .html)"
  android_out="$evidence_dir/android/$name"
  ios_out="$evidence_dir/ios/$name"
  mkdir -p "$android_out" "$ios_out"
  cp "$scene_file" "$source_index"
  cp "$scene_dir/root-app.js" "$source_root_app"
  source_sha="$(shasum -a 256 "$source_index" | awk '{print $1}')"
  source_bytes="$(wc -c < "$source_index" | tr -d ' ')"
  printf 'scene=%s\nsource_sha256=%s\nsource_bytes=%s\n' \
    "$name" "$source_sha" "$source_bytes" > "$android_out/source.txt"
  cp "$android_out/source.txt" "$ios_out/source.txt"

  if [[ "$platforms" != "ios" ]]; then
    SPINON_ENABLE_C04_RUNTIME_GPU=1 \
    SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
    bash "$repo_root/tools/build-android-app.sh" > "$android_out/build.log" 2>&1
    packaged_sha="$(unzip -p "$apk" assets/index.html | shasum -a 256 | awk '{print $1}')"
    test "$packaged_sha" = "$source_sha"
    shasum -a 256 "$apk" > "$android_out/artifact.sha256"
    printf 'packaged_html_sha256=%s\n' "$packaged_sha" >> "$android_out/source.txt"
    "$android_sdk/platform-tools/adb" -s "$android_serial" install -r "$apk" \
      > "$android_out/install.log" 2>&1
    "$android_sdk/platform-tools/adb" -s "$android_serial" shell am force-stop "$package" \
      > "$android_out/force-stop.log" 2>&1
    "$android_sdk/platform-tools/adb" -s "$android_serial" shell am start -n "$package/.MainActivity" \
      > "$android_out/launch.log" 2>&1
    android_pid=""
    for attempt in $(seq 1 60); do
      android_pid="$("$android_sdk/platform-tools/adb" -s "$android_serial" shell pidof "$package" | tr -d '\r' | awk '{print $1}')"
      if [[ -n "$android_pid" ]]; then break; fi
      sleep 1
    done
    test -n "$android_pid"
    printf '%s\n' "$android_pid" > "$android_out/app-pid.txt"
    for attempt in $(seq 1 60); do
      "$android_sdk/platform-tools/adb" -s "$android_serial" logcat -d -v threadtime \
        | awk -v app_pid="$android_pid" '$3 == app_pid' > "$android_out/app.log"
      if rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=$source_bytes" "$android_out/app.log" \
        && rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$android_out/app.log" \
        && rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$android_out/app.log"; then
        break
      fi
      sleep 1
    done
    rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=$source_bytes" "$android_out/app.log"
    rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready' "$android_out/app.log"
    rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$android_out/app.log"
    "$android_sdk/platform-tools/adb" -s "$android_serial" exec-out screencap -p \
      > "$android_out/screenshot.png"
    test -s "$android_out/screenshot.png"
    "$android_sdk/platform-tools/adb" -s "$android_serial" shell dumpsys activity activities \
      | rg -m 4 'topResumedActivity|mResumedActivity' > "$android_out/resumed-activity.txt"
    shasum -a 256 "$android_out/screenshot.png" >> "$android_out/artifact.sha256"
  fi

  if [[ "$platforms" != "android" ]]; then
    SPINON_ENABLE_C04_RUNTIME_GPU=1 \
      SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
      bash "$repo_root/tools/build-ios-sim.sh" > "$ios_out/build.log" 2>&1
    packaged_sha="$(shasum -a 256 "$app/index.html" | awk '{print $1}')"
    test "$packaged_sha" = "$source_sha"
    shasum -a 256 "$app/SpinonBootstrap" > "$ios_out/artifact.sha256"
    printf 'packaged_html_sha256=%s\n' "$packaged_sha" >> "$ios_out/source.txt"
    xcrun simctl install "$ios_udid" "$app" > "$ios_out/install.log" 2>&1
    xcrun simctl terminate "$ios_udid" "$package" > "$ios_out/terminate.log" 2>&1 || true
    xcrun simctl launch "$ios_udid" "$package" > "$ios_out/launch.log" 2>&1
    ios_bytes="$source_bytes"
    for attempt in $(seq 1 60); do
      xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
        --predicate 'process == "SpinonBootstrap" AND (eventMessage CONTAINS "SPINON_ROOT_HTML_DEFAULT_APP" OR eventMessage CONTAINS "SPINON_C0410_DRAW")' \
        > "$ios_out/app.log"
      if rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=$ios_bytes" "$ios_out/app.log" \
        && rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$ios_out/app.log" \
        && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=' "$ios_out/app.log"; then
        break
      fi
      sleep 1
    done
    rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=$ios_bytes" "$ios_out/app.log"
    rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready' "$ios_out/app.log"
    rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=' "$ios_out/app.log"
    xcrun simctl io "$ios_udid" screenshot "$ios_out/screenshot.png" \
      > "$ios_out/screenshot.log" 2>&1
    test -s "$ios_out/screenshot.png"
    shasum -a 256 "$app/index.html" "$ios_out/screenshot.png" >> "$ios_out/artifact.sha256"
  fi
done
