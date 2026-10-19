#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="$repo_root/spec/internal/evidence/c11-4-image-ratio-transfer-simulator-2026-10-19"
android_serial="emulator-5554"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
run_id="c114-ratio-$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="$evidence_root/runs/$run_id"
scenes=(
  column-auto-natural-ratio
  column-target-max-width-clamp
  row-target-max-height-clamp
  column-opposite-min-over-max
  column-border-box-transfer
  column-content-box-transfer
  column-target-min-width-clamp
)
pixel_samples='[{"id":"item-inside-root","nodeId":"c114-column-auto-natural-ratio-item","xCssPx":5,"yCssPx":4,"rgba":[229,72,77,255]},{"id":"item-overflow-beyond-root","nodeId":"c114-column-auto-natural-ratio-item","xCssPx":10,"yCssPx":4,"rgba":[229,72,77,255]},{"id":"root-background-control","nodeId":"c114-column-auto-natural-ratio-grid","xCssPx":6,"yCssPx":15,"rgba":[16,24,32,255]}]'
android_pixel_samples="$(printf '%s' "$pixel_samples" | base64 | tr '+/' '-_' | tr -d '=\n')"
ios_pixel_samples="$(printf '%s' "$pixel_samples" | base64 | tr -d '\n')"

if [[ "$(adb -s "$android_serial" shell getprop ro.kernel.qemu | tr -d '\r')" != 1 ]]; then
  echo "Android 대상이 API 37 AVD가 아닙니다: $android_serial" >&2
  exit 1
fi
if [[ -n "$(adb -s "$android_serial" shell pidof "$bundle_id" 2>/dev/null || true)" ]]; then
  echo "Android 대상에서 기존 Spinon 앱이 실행 중입니다. 중단합니다." >&2
  exit 1
fi
if ps -Ao command | rg -F "/CoreSimulator/Devices/$ios_udid/" | rg -F "/$bundle_id.app/" >/dev/null; then
  echo "iPhone 17 Pro Simulator에서 기존 Spinon 앱이 실행 중입니다. 중단합니다." >&2
  exit 1
fi
mkdir -p "$run_dir/android-avd" "$run_dir/ios-simulator"
test -f "$apk"
test -d "$ios_app"

cleanup() {
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
}
trap cleanup EXIT

adb -s "$android_serial" install -r "$apk" > "$run_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios-simulator/install.log" 2>&1

for scene_id in "${scenes[@]}"; do
  scene_run_id="$run_id-$scene_id"
  android_log="$run_dir/android-avd/runtime-$scene_id.log"
  android_png="$run_dir/android-avd/screenshot-$scene_id.png"
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  adb -s "$android_serial" logcat -c
  if [[ "$scene_id" == "column-auto-natural-ratio" ]]; then
    adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
      --ez spinon_c114_grid_track_sizing true \
      --es spinon_c114_grid_scene "$scene_id" \
      --es spinon_c114_grid_run_id "$scene_run_id" \
      --es spinon_c11_grid_pixel_matrix_run_id "$scene_run_id" \
      --es spinon_c11_grid_pixel_matrix_samples "$android_pixel_samples" \
      > "$run_dir/android-avd/launch-$scene_id.log" 2>&1
  else
    adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
      --ez spinon_c114_grid_track_sizing true \
      --es spinon_c114_grid_scene "$scene_id" \
      --es spinon_c114_grid_run_id "$scene_run_id" \
      > "$run_dir/android-avd/launch-$scene_id.log" 2>&1
  fi
  sleep 2
  adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap > "$android_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=$scene_id .*status=0" "$android_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  if [[ "$scene_id" == "column-auto-natural-ratio" ]] \
      && ! rg -q 'SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  adb -s "$android_serial" exec-out screencap -p > "$android_png"

  ios_log="$run_dir/ios-simulator/runtime-$scene_id.log"
  ios_png="$run_dir/ios-simulator/screenshot-$scene_id.png"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  if [[ "$scene_id" == "column-auto-natural-ratio" ]]; then
    xcrun simctl launch "$ios_udid" "$bundle_id" \
      --spinon-c114-grid-track-sizing \
      "--spinon-c114-grid-scene=$scene_id" \
      "--spinon-c114-grid-run-id=$scene_run_id" \
      "--spinon-c11-grid-metal-surface-pixels=$scene_run_id" \
      "--spinon-c11-grid-metal-surface-samples=$ios_pixel_samples" \
      > "$run_dir/ios-simulator/launch-$scene_id.log" 2>&1
  else
    xcrun simctl launch "$ios_udid" "$bundle_id" \
      --spinon-c114-grid-track-sizing \
      "--spinon-c114-grid-scene=$scene_id" \
      "--spinon-c114-grid-run-id=$scene_run_id" \
      > "$run_dir/ios-simulator/launch-$scene_id.log" 2>&1
  fi
  sleep 2
  xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
    --predicate "process == \"SpinonBootstrap\" AND composedMessage CONTAINS \"$scene_run_id\"" \
    > "$ios_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=$scene_id .*status=0" "$ios_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  if [[ "$scene_id" == "column-auto-natural-ratio" ]] \
      && ! rg -q 'SPINON_C114_METAL_PIXEL_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  xcrun simctl io "$ios_udid" screenshot "$ios_png" > "$run_dir/ios-simulator/screenshot-$scene_id.log" 2>&1
done

printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf 'Android API 37 AVD and iPhone 17 Pro/iOS 26.2 Simulator matched seven C11.4 image-ratio scenes. Evidence: %s\n' "$run_dir"
