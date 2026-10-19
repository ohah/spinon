#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="$repo_root/spec/internal/evidence/c11-4-image-ratio-span-2026-10-19"
android_serial="emulator-5554"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
run_id="c114-span-$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="$evidence_root/runs/$run_id"
scenes=(
  column-auto-natural-ratio
  column-target-max-width-clamp
  row-target-max-height-clamp
  column-opposite-min-over-max
  column-border-box-transfer
  column-content-box-transfer
  column-target-min-width-clamp
  row-transfer-input-capped-by-fixed-column-span
  column-transfer-input-capped-by-fixed-row-span
)

if [[ "$(adb -s "$android_serial" shell getprop ro.kernel.qemu | tr -d '\r')" != 1 ]]; then
  echo "Android 대상이 에뮬레이터가 아닙니다: $android_serial" >&2
  exit 1
fi
if [[ -n "$(adb -s "$android_serial" shell pidof "$bundle_id" 2>/dev/null || true)" ]]; then
  echo "Android 대상에서 기존 Spinon 앱이 실행 중입니다. 중단합니다." >&2
  exit 1
fi
if ps -Ao command | rg -F "/CoreSimulator/Devices/$ios_udid/" | rg -F "/$bundle_id.app/" >/dev/null; then
  echo "지정한 iPhone Simulator에서 기존 Spinon 앱이 실행 중입니다. 중단합니다." >&2
  exit 1
fi
if ! xcrun simctl list devices booted | rg -F "$ios_udid" >/dev/null; then
  echo "지정한 iPhone Simulator가 부팅되어 있지 않습니다: $ios_udid" >&2
  exit 1
fi
mkdir -p "$run_dir/android-avd" "$run_dir/ios-simulator"
test -f "$apk"
test -d "$ios_app"

adb -s "$android_serial" shell getprop ro.build.version.sdk \
  > "$run_dir/android-avd/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi \
  > "$run_dir/android-avd/abi.txt"
xcrun simctl list devices booted > "$run_dir/ios-simulator/booted-devices.txt"
xcrun simctl getenv "$ios_udid" SIMULATOR_MODEL_IDENTIFIER \
  > "$run_dir/ios-simulator/model-identifier.txt" 2>&1 || true

cleanup() {
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
}
trap cleanup EXIT

adb -s "$android_serial" install -r "$apk" > "$run_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios-simulator/install.log" 2>&1

for scene_id in "${scenes[@]}"; do
  scene_run_id="$run_id-$scene_id"
  pixel_json=""
  case "$scene_id" in
    row-transfer-input-capped-by-fixed-column-span)
      pixel_json='[{"id":"inside-item","nodeId":"c114-row-transfer-fixed-column-item","xCssPx":4,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"right-of-grid","nodeId":"c114-row-transfer-fixed-column-item","xCssPx":12,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"below-grid","nodeId":"c114-row-transfer-fixed-column-item","xCssPx":4,"yCssPx":8,"rgba":[229,72,77,255]}]'
      ;;
    column-transfer-input-capped-by-fixed-row-span)
      pixel_json='[{"id":"inside-item","nodeId":"c114-column-transfer-fixed-row-item","xCssPx":2,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"right-of-grid","nodeId":"c114-column-transfer-fixed-row-item","xCssPx":8,"yCssPx":2,"rgba":[229,72,77,255]},{"id":"below-grid","nodeId":"c114-column-transfer-fixed-row-item","xCssPx":2,"yCssPx":12,"rgba":[229,72,77,255]}]'
      ;;
  esac
  android_pixel_samples="$(printf '%s' "$pixel_json" | base64 | tr '+/' '-_' | tr -d '=\n')"
  ios_pixel_samples="$(printf '%s' "$pixel_json" | base64 | tr -d '\n')"

  android_log="$run_dir/android-avd/runtime-$scene_id.log"
  android_png="$run_dir/android-avd/screenshot-$scene_id.png"
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  adb -s "$android_serial" logcat -c
  android_args=(
    --ez spinon_c114_grid_track_sizing true
    --es spinon_c114_grid_scene "$scene_id"
    --es spinon_c114_grid_run_id "$scene_run_id"
  )
  if [[ -n "$pixel_json" ]]; then
    android_args+=(
      --es spinon_c11_grid_pixel_matrix_run_id "$scene_run_id"
      --es spinon_c11_grid_pixel_matrix_samples "$android_pixel_samples"
    )
  fi
  adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
    "${android_args[@]}" > "$run_dir/android-avd/launch-$scene_id.log" 2>&1
  sleep 2
  adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap \
    > "$android_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=$scene_id .*status=0" "$android_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  if [[ -n "$pixel_json" ]] && ! rg -q \
      'SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$android_log"; then
    cat "$android_log" >&2
    exit 1
  fi
  adb -s "$android_serial" exec-out screencap -p > "$android_png"
  test -s "$android_png"

  ios_log="$run_dir/ios-simulator/runtime-$scene_id.log"
  ios_png="$run_dir/ios-simulator/screenshot-$scene_id.png"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  ios_args=(
    --spinon-c114-grid-track-sizing
    "--spinon-c114-grid-scene=$scene_id"
    "--spinon-c114-grid-run-id=$scene_run_id"
  )
  if [[ -n "$pixel_json" ]]; then
    ios_args+=(
      "--spinon-c11-grid-metal-surface-pixels=$scene_run_id"
      "--spinon-c11-grid-metal-surface-samples=$ios_pixel_samples"
    )
  fi
  xcrun simctl launch "$ios_udid" "$bundle_id" "${ios_args[@]}" \
    > "$run_dir/ios-simulator/launch-$scene_id.log" 2>&1
  sleep 2
  xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
    --predicate "process == \"SpinonBootstrap\" AND composedMessage CONTAINS \"$scene_run_id\"" \
    > "$ios_log" 2>&1
  if ! rg -q "SPINON_C114_GRID_SUMMARY .*scene_id=$scene_id .*status=0" "$ios_log" \
      || rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  if [[ -n "$pixel_json" ]] && ! rg -q \
      'SPINON_C114_METAL_PIXEL_SUMMARY status=matched .*expected_samples=3 observed_samples=3 matched_samples=3' "$ios_log"; then
    cat "$ios_log" >&2
    exit 1
  fi
  xcrun simctl io "$ios_udid" screenshot "$ios_png" \
    > "$run_dir/ios-simulator/screenshot-$scene_id.log" 2>&1
  test -s "$ios_png"
done

printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf 'Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 C11.4 이미지 비율 장면 9개를 실행했습니다. 신규 고정 트랙 반대 축 overflow 표본은 각 플랫폼에서 3/3 일치했습니다. 근거: %s\n' "$run_dir"
