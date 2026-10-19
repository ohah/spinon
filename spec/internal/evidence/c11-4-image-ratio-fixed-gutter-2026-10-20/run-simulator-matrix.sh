#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="$repo_root/spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20"
android_serial="emulator-5554"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
ios_model_identifier="iPhone18,1"
android_avd_name="spinon_api37_1_compat16k"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
run_id="c114-fg-$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="$evidence_root/runs/$run_id"
scenes=(
  fixed-gutter-column-suggestion-clamp
  fixed-gutter-row-suggestion-clamp
)
android_started=0
ios_started=0

fail() {
  echo "$*" >&2
  exit 1
}

cleanup() {
  if [[ "$android_started" == 1 ]]; then
    adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  fi
  if [[ "$ios_started" == 1 ]]; then
    xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

[[ "$(adb -s "$android_serial" shell getprop ro.kernel.qemu | tr -d '\r')" == 1 ]] \
  || fail "Android 대상이 에뮬레이터가 아닙니다: $android_serial"
[[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" == 37 ]] \
  || fail "Android 대상 API가 37이 아닙니다: $android_serial"
[[ "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" == arm64-v8a ]] \
  || fail "Android 대상 ABI가 ARM64가 아닙니다: $android_serial"
actual_android_avd_name="$(adb -s "$android_serial" emu avd name | head -n 1 | tr -d '\r')"
[[ "$actual_android_avd_name" == "$android_avd_name" ]] \
  || fail "예상한 Android AVD가 아닙니다: $actual_android_avd_name"
booted_ios_devices="$(xcrun simctl list devices booted)"
[[ "$booted_ios_devices" == *"-- iOS 26.2 --"* \
    && "$booted_ios_devices" == *"iPhone 17 Pro ($ios_udid) (Booted)"* ]] \
  || fail "지정한 iPhone 17 Pro / iOS 26.2 Simulator가 부팅되어 있지 않습니다: $ios_udid"
actual_ios_model_identifier="$(xcrun simctl getenv "$ios_udid" SIMULATOR_MODEL_IDENTIFIER \
  | tail -n 1 | tr -d '\r')"
[[ "$actual_ios_model_identifier" == "$ios_model_identifier" ]] \
  || fail "iOS Simulator 모델이 iPhone 17 Pro가 아닙니다: $actual_ios_model_identifier"
[[ -z "$(adb -s "$android_serial" shell pidof "$bundle_id" 2>/dev/null || true)" ]] \
  || fail "Android AVD에서 Spinon 앱이 이미 실행 중입니다."
if ps -Ao command | rg -F "/CoreSimulator/Devices/$ios_udid/" \
    | rg -F "/$bundle_id.app/" >/dev/null; then
  fail "iPhone Simulator에서 Spinon 앱이 이미 실행 중입니다."
fi

mkdir -p "$run_dir/android-avd" "$run_dir/ios-simulator"
[[ -f "$apk" ]] || fail "Android APK가 없습니다: $apk"
[[ -d "$ios_app" ]] || fail "iOS Simulator 앱이 없습니다: $ios_app"

adb -s "$android_serial" shell getprop ro.build.version.sdk \
  > "$run_dir/android-avd/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi \
  > "$run_dir/android-avd/abi.txt"
adb -s "$android_serial" emu avd name > "$run_dir/android-avd/avd-name.txt"
printf '%s\n' "$booted_ios_devices" > "$run_dir/ios-simulator/booted-devices.txt"
printf '%s\n' "$actual_ios_model_identifier" > "$run_dir/ios-simulator/model-identifier.txt"
git -C "$repo_root" rev-parse HEAD > "$run_dir/source-head.txt"

adb -s "$android_serial" install -r "$apk" > "$run_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios-simulator/install.log" 2>&1

for scene_id in "${scenes[@]}"; do
  scene_run_id="$run_id-$scene_id"
  case "$scene_id" in
    fixed-gutter-column-suggestion-clamp)
      pixel_json='[{"id":"grid-border","nodeId":"c114-fixed-gutter-column-grid","xCssPx":0,"yCssPx":2,"rgba":[34,197,94,255]},{"id":"inside-item","nodeId":"c114-fixed-gutter-column-item","xCssPx":2,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"right-of-grid","nodeId":"c114-fixed-gutter-column-item","xCssPx":12,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"below-grid","nodeId":"c114-fixed-gutter-column-item","xCssPx":2,"yCssPx":6,"rgba":[229,72,77,255]}]'
      ;;
    fixed-gutter-row-suggestion-clamp)
      pixel_json='[{"id":"grid-border","nodeId":"c114-fixed-gutter-row-grid","xCssPx":0,"yCssPx":2,"rgba":[34,197,94,255]},{"id":"inside-item","nodeId":"c114-fixed-gutter-row-item","xCssPx":2,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"right-of-grid","nodeId":"c114-fixed-gutter-row-item","xCssPx":6,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"below-grid","nodeId":"c114-fixed-gutter-row-item","xCssPx":2,"yCssPx":12,"rgba":[229,72,77,255]}]'
      ;;
    *) fail "unlisted C11.4 scene: $scene_id" ;;
  esac
  android_samples="$(printf '%s' "$pixel_json" | base64 | tr '+/' '-_' | tr -d '=\n')"
  ios_samples="$(printf '%s' "$pixel_json" | base64 | tr -d '\n')"

  android_log="$run_dir/android-avd/runtime-$scene_id.log"
  android_png="$run_dir/android-avd/screenshot-$scene_id.png"
  adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
    --ez spinon_c114_grid_track_sizing true \
    --es spinon_c114_grid_scene "$scene_id" \
    --es spinon_c114_grid_run_id "$scene_run_id" \
    --es spinon_c11_grid_pixel_matrix_run_id "$scene_run_id" \
    --es spinon_c11_grid_pixel_matrix_samples "$android_samples" \
    > "$run_dir/android-avd/launch-$scene_id.log" 2>&1
  android_started=1
  for attempt in {1..30}; do
    adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap \
      | rg -F "$scene_run_id" > "$android_log" || true
    if rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0" "$android_log" \
        && rg -q "SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=$scene_run_id .*expected_samples=4 observed_samples=4 matched_samples=4" "$android_log"; then
      break
    fi
    sleep 1
  done
  rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0" "$android_log" \
    || fail "Android C11.4 frame/trace assertion failed for $scene_id; see $android_log"
  rg -q "SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched run_id=$scene_run_id .*expected_samples=4 observed_samples=4 matched_samples=4" "$android_log" \
    || fail "Android PixelCopy samples did not match for $scene_id; see $android_log"
  if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR|SPINON_C114_GRID_PIXEL_MATRIX status=error' "$android_log"; then
    fail "Android C11.4 runtime reported an assertion/configuration error for $scene_id; see $android_log"
  fi
  sleep 1
  adb -s "$android_serial" exec-out screencap -p > "$android_png"
  [[ -s "$android_png" ]] || fail "Android screenshot is empty for $scene_id"
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  android_started=0

  ios_log="$run_dir/ios-simulator/runtime-$scene_id.log"
  ios_png="$run_dir/ios-simulator/screenshot-$scene_id.png"
  xcrun simctl launch "$ios_udid" "$bundle_id" \
    --spinon-c114-grid-track-sizing \
    "--spinon-c114-grid-scene=$scene_id" \
    "--spinon-c114-grid-run-id=$scene_run_id" \
    "--spinon-c11-grid-metal-surface-pixels=$scene_run_id" \
    "--spinon-c11-grid-metal-surface-samples=$ios_samples" \
    > "$run_dir/ios-simulator/launch-$scene_id.log" 2>&1
  ios_started=1
  for attempt in {1..30}; do
    xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
      --predicate "process == \"SpinonBootstrap\" AND composedMessage CONTAINS \"$scene_run_id\"" \
      > "$ios_log" 2>&1
    if rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0" "$ios_log" \
        && rg -q "SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=$scene_run_id .*expected_samples=4 observed_samples=4 matched_samples=4" "$ios_log"; then
      break
    fi
    sleep 1
  done
  rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0" "$ios_log" \
    || fail "iOS C11.4 frame/trace assertion failed for $scene_id; see $ios_log"
  rg -q "SPINON_C114_METAL_PIXEL_SUMMARY status=matched run_id=$scene_run_id .*expected_samples=4 observed_samples=4 matched_samples=4" "$ios_log" \
    || fail "iOS Metal samples did not match for $scene_id; see $ios_log"
  if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR|SPINON_C114_METAL_PIXEL_ERROR' "$ios_log"; then
    fail "iOS C11.4 runtime reported an assertion/configuration error for $scene_id; see $ios_log"
  fi
  sleep 1
  xcrun simctl io "$ios_udid" screenshot "$ios_png" \
    > "$run_dir/ios-simulator/screenshot-$scene_id.log" 2>&1
  [[ -s "$ios_png" ]] || fail "iOS screenshot is empty for $scene_id"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  ios_started=0
done

printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf 'C11.4 fixed-gutter scenes passed on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator: %s\n' "$run_dir"
