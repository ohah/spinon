#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="$repo_root/spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26"
run_id="${SPINON_C114_RUN_ID:-c114-image-auto-max-20261026T002422Z-040a5360-stagefix01}"
run_dir="$evidence_root/runs/$run_id"
scene_id="image-auto-max-mixed-tracks"
android_serial="emulator-5554"
android_avd_name="spinon_api37_1_compat16k"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
ios_model_identifier="iPhone18,1"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"

cleanup_apps() {
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
}
trap cleanup_apps EXIT

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

[[ -f "$apk" ]] || fail "Android APK missing: $apk"
[[ -d "$ios_app" ]] || fail "iOS Simulator app missing: $ios_app"
android_devices="$(adb devices | sed -n '2,2p' | tr -d '\r')"
[[ "$android_devices" == "$android_serial	device" ]] \
  || fail "Only the planned Android emulator may be attached: $android_devices"
[[ "$(adb -s "$android_serial" shell getprop sys.boot_completed | tr -d '\r')" == "1" ]] \
  || fail "Android emulator is not booted"
[[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" == "37" ]] \
  || fail "Android emulator API is not 37"
[[ "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" == "arm64-v8a" ]] \
  || fail "Android emulator ABI is not arm64-v8a"
[[ "$(adb -s "$android_serial" emu avd name | sed -n '1p' | tr -d '\r')" == "$android_avd_name" ]] \
  || fail "Unexpected Android AVD"
booted_ios_devices="$(xcrun simctl list devices booted)"
[[ "$booted_ios_devices" == *"iPhone 17 Pro ($ios_udid) (Booted)"* ]] \
  || fail "Planned iPhone 17 Pro / iOS 26.2 Simulator is not booted"
[[ "$(xcrun simctl getenv "$ios_udid" SIMULATOR_MODEL_IDENTIFIER | tail -n 1 | tr -d '\r')" == "$ios_model_identifier" ]] \
  || fail "Unexpected iOS Simulator model"

mkdir -p "$run_dir/android-avd" "$run_dir/ios-simulator"
printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf '%s\n' "$(git -C "$repo_root" rev-parse HEAD)" > "$run_dir/source-head.txt"
printf '%s\n' "$booted_ios_devices" > "$run_dir/ios-simulator/booted-devices.txt"
printf '%s\n' "$ios_model_identifier" > "$run_dir/ios-simulator/model-identifier.txt"
adb -s "$android_serial" shell getprop ro.build.version.sdk > "$run_dir/android-avd/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi > "$run_dir/android-avd/abi.txt"
adb -s "$android_serial" emu avd name | sed -n '1p' | tr -d '\r' > "$run_dir/android-avd/avd-name.txt"

adb -s "$android_serial" install -r "$apk" > "$run_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios-simulator/install.log" 2>&1
adb -s "$android_serial" logcat -c
adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
  --ez spinon_c114_grid_track_sizing true \
  --es spinon_c114_grid_scene "$scene_id" \
  --es spinon_c114_grid_run_id "$run_id" \
  > "$run_dir/android-avd/launch.log" 2>&1

android_log="$run_dir/android-avd/runtime.log"
for attempt in {1..60}; do
  adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap > "$android_log" 2>&1
  if rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready" "$android_log" \
      && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$android_log"; then
    break
  fi
  sleep 1
done
rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready boxes=2" "$android_log" \
  || fail "Android frame/trace assertion did not pass; inspect $android_log"
rg -q 'SPINON_C114_GRID_TRACE_CHUNK run_id='"$run_id" "$android_log" \
  || fail "Android C11.4 trace was not logged"
rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$android_log" \
  || fail "Android draw/present report is absent"
if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$android_log"; then
  fail "Android route reported a C11.4 assertion/configuration error"
fi
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$run_dir/android-avd/screenshot.png"
[[ -s "$run_dir/android-avd/screenshot.png" ]] || fail "Android screenshot is empty"
adb -s "$android_serial" shell am force-stop "$bundle_id" > "$run_dir/android-avd/terminate.log" 2>&1

xcrun simctl launch "$ios_udid" "$bundle_id" \
  --spinon-c114-grid-track-sizing \
  "--spinon-c114-grid-scene=$scene_id" \
  "--spinon-c114-grid-run-id=$run_id" \
  > "$run_dir/ios-simulator/launch.log" 2>&1
ios_log="$run_dir/ios-simulator/runtime.log"
for attempt in {1..60}; do
  xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
    --predicate 'process == "SpinonBootstrap"' > "$ios_log" 2>&1
  if rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready" "$ios_log" \
      && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$ios_log"; then
    break
  fi
  sleep 1
done
rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready boxes=2" "$ios_log" \
  || fail "iOS frame/trace assertion did not pass; inspect $ios_log"
rg -q 'SPINON_C114_GRID_TRACE_CHUNK run_id='"$run_id" "$ios_log" \
  || fail "iOS C11.4 trace was not logged"
rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$ios_log" \
  || fail "iOS draw/present report is absent"
if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$ios_log"; then
  fail "iOS route reported a C11.4 assertion/configuration error"
fi
sleep 1
xcrun simctl io "$ios_udid" screenshot "$run_dir/ios-simulator/screenshot.png" \
  > "$run_dir/ios-simulator/screenshot.log" 2>&1
[[ -s "$run_dir/ios-simulator/screenshot.png" ]] || fail "iOS screenshot is empty"
xcrun simctl terminate "$ios_udid" "$bundle_id" > "$run_dir/ios-simulator/terminate.log" 2>&1 || true

printf 'C11.4 image mixed-track scene passed on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator: %s\n' "$run_dir"
