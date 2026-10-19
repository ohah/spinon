#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19"
android_serial="emulator-5554"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
scenes=(
  auto-image-definite
  minmax-auto-image
  minmax-min-max-image
  fit-content-below-minimum
  fit-content-above-intrinsic
  fit-content-percent
  fixed-small-track-overflow
  spanning-min-content-items
  min-content-row-image
)
pixel_samples='[{"id":"item-inside-root","nodeId":"c114-fixed-small-image-item","xCssPx":5,"yCssPx":8,"rgba":[229,72,77,255]},{"id":"item-overflow-beyond-root","nodeId":"c114-fixed-small-image-item","xCssPx":20,"yCssPx":8,"rgba":[229,72,77,255]},{"id":"root-background-control","nodeId":"c114-fixed-small-image-grid","xCssPx":6,"yCssPx":25,"rgba":[16,24,32,255]}]'
android_pixel_samples="$(printf '%s' "$pixel_samples" | base64 | tr '+/' '-_' | tr -d '=\n')"
ios_pixel_samples="$(printf '%s' "$pixel_samples" | base64 | tr -d '\n')"

test -f "$apk"
test -d "$ios_app"
adb -s "$android_serial" install -r "$apk" \
  > "$evidence_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" \
  > "$evidence_dir/ios-simulator/install.log" 2>&1

for scene_id in "${scenes[@]}"; do
  run_id="c114-image-android-${scene_id}-20261019-r4"
  android_log="$evidence_dir/android-avd/runtime-${scene_id}.log"
  android_png="$evidence_dir/android-avd/screenshot-${scene_id}.png"

  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  adb -s "$android_serial" logcat -c
  if [[ "$scene_id" == "fixed-small-track-overflow" ]]; then
    adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
      --ez spinon_c114_grid_track_sizing true \
      --es spinon_c114_grid_scene "$scene_id" \
      --es spinon_c114_grid_run_id "$run_id" \
      --es spinon_c11_grid_pixel_matrix_run_id "$run_id" \
      --es spinon_c11_grid_pixel_matrix_samples "$android_pixel_samples" \
      > "$evidence_dir/android-avd/launch-${scene_id}.log" 2>&1
  else
    adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
      --ez spinon_c114_grid_track_sizing true \
      --es spinon_c114_grid_scene "$scene_id" \
      --es spinon_c114_grid_run_id "$run_id" \
      > "$evidence_dir/android-avd/launch-${scene_id}.log" 2>&1
  fi
  sleep 2
  adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap \
    > "$android_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=${scene_id}" "$android_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  if [[ "$scene_id" == "fixed-small-track-overflow" ]] \
      && ! rg -q 'SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  adb -s "$android_serial" exec-out screencap -p > "$android_png"

  run_id="c114-image-ios-${scene_id}-20261019-r4"
  ios_log="$evidence_dir/ios-simulator/runtime-${scene_id}.log"
  ios_png="$evidence_dir/ios-simulator/screenshot-${scene_id}.png"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  if [[ "$scene_id" == "fixed-small-track-overflow" ]]; then
    xcrun simctl launch "$ios_udid" "$bundle_id" \
      --spinon-c114-grid-track-sizing \
      "--spinon-c114-grid-scene=$scene_id" \
      "--spinon-c114-grid-run-id=$run_id" \
      "--spinon-c11-grid-metal-surface-pixels=$run_id" \
      "--spinon-c11-grid-metal-surface-samples=$ios_pixel_samples" \
      > "$evidence_dir/ios-simulator/launch-${scene_id}.log" 2>&1
  else
    xcrun simctl launch "$ios_udid" "$bundle_id" \
      --spinon-c114-grid-track-sizing \
      "--spinon-c114-grid-scene=$scene_id" \
      "--spinon-c114-grid-run-id=$run_id" \
      > "$evidence_dir/ios-simulator/launch-${scene_id}.log" 2>&1
  fi
  sleep 2
  xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
    --predicate "process == \"SpinonBootstrap\" AND composedMessage CONTAINS \"$run_id\"" \
    > "$ios_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=${scene_id}" "$ios_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  if [[ "$scene_id" == "fixed-small-track-overflow" ]] \
      && ! rg -q 'SPINON_C114_METAL_PIXEL_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  xcrun simctl io "$ios_udid" screenshot "$ios_png" \
    > "$evidence_dir/ios-simulator/screenshot-${scene_id}.log" 2>&1
done

adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
