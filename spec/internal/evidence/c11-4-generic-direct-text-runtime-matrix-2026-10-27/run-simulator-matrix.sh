#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="$repo_root/spec/internal/evidence/c11-4-generic-direct-text-runtime-matrix-2026-10-27"
run_id="${SPINON_C114_MATRIX_RUN_ID:-c114-direct-text-matrix-$(date -u +%Y%m%dT%H%M%SZ)-pass01}"
run_dir="$evidence_root/runs/$run_id"
android_serial="emulator-5554"
android_avd_name="spinon_api37_1_compat16k"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
ios_model_identifier="iPhone18,1"
bundle_id="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
scenes=(
  generic-root-normal
  generic-root-nowrap
  generic-root-break-all
  generic-root-content-variation
  generic-root-whitespace
  generic-root-reorder
)

cleanup_apps() {
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
}
trap cleanup_apps EXIT

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

await_android_scene() {
  local scene_id="$1"
  local scene_run_id="$2"
  local scene_dir="$3"
  local android_log="$scene_dir/runtime.log"

  for attempt in {1..60}; do
    adb -s "$android_serial" logcat -d -v time -s SpinonBootstrap > "$android_log" 2>&1
    if rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0 .*layout=ready" "$android_log" \
        && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=3 .*surface_texture=' "$android_log"; then
      break
    fi
    sleep 1
  done
  rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0 .*layout=ready boxes=3" "$android_log" \
    || fail "Android matcher failed for $scene_id; inspect $android_log"
  rg -q "SPINON_C114_GRID_TRACE_CHUNK run_id=$scene_run_id" "$android_log" \
    || fail "Android C11.4 trace is absent for $scene_id"
  rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=3 .*surface_texture=' "$android_log" \
    || fail "Android draw/present report is absent for $scene_id"
  if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$android_log"; then
    fail "Android route reported a C11.4 assertion/configuration error for $scene_id"
  fi
}

await_ios_scene() {
  local scene_id="$1"
  local scene_run_id="$2"
  local scene_dir="$3"
  local ios_log="$scene_dir/runtime.log"

  for attempt in {1..60}; do
    xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
      --predicate 'process == "SpinonBootstrap"' > "$ios_log" 2>&1
    if rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0 .*layout=ready" "$ios_log" \
        && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=3 .*surface_texture=' "$ios_log"; then
      break
    fi
    sleep 1
  done
  rg -q "SPINON_C114_GRID_SUMMARY run_id=$scene_run_id scene_id=$scene_id .*status=0 .*layout=ready boxes=3" "$ios_log" \
    || fail "iOS matcher failed for $scene_id; inspect $ios_log"
  rg -q "SPINON_C114_GRID_TRACE_CHUNK run_id=$scene_run_id" "$ios_log" \
    || fail "iOS C11.4 trace is absent for $scene_id"
  rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=3 .*surface_texture=' "$ios_log" \
    || fail "iOS draw/present report is absent for $scene_id"
  if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$ios_log"; then
    fail "iOS route reported a C11.4 assertion/configuration error for $scene_id"
  fi
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

for scene in "${scenes[@]}"; do
  mkdir -p "$run_dir/android-avd/$scene" "$run_dir/ios-simulator/$scene"
done
printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf '%s\n' "$(git -C "$repo_root" rev-parse HEAD)" > "$run_dir/source-head.txt"
git -C "$repo_root" diff --binary HEAD -- \
  crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs \
  crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/generic_direct_text.rs \
  crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs \
  crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs \
  platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java \
  platforms/ios/Sources/C0410RuntimeGpuDemo.swift > "$run_dir/source-diff.patch"
shasum -a 256 "$run_dir/source-diff.patch" > "$run_dir/source-diff.sha256"
shasum -a 256 "$apk" > "$run_dir/android-apk.sha256"
find "$ios_app" -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256 > "$run_dir/ios-app-bundle.sha256"
printf '%s\n' "$booted_ios_devices" > "$run_dir/ios-simulator/booted-devices.txt"
printf '%s\n' "$ios_model_identifier" > "$run_dir/ios-simulator/model-identifier.txt"
adb -s "$android_serial" shell getprop ro.build.version.sdk > "$run_dir/android-avd/api-level.txt"
adb -s "$android_serial" shell getprop ro.product.cpu.abi > "$run_dir/android-avd/abi.txt"
adb -s "$android_serial" emu avd name | sed -n '1p' | tr -d '\r' > "$run_dir/android-avd/avd-name.txt"

adb -s "$android_serial" install -r "$apk" > "$run_dir/android-avd/install.log" 2>&1
xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios-simulator/install.log" 2>&1

for scene_index in "${!scenes[@]}"; do
  scene="${scenes[$scene_index]}"
  scene_run_id="$run_id-android-$scene_index"
  scene_dir="$run_dir/android-avd/$scene"
  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  adb -s "$android_serial" logcat -c
  adb -s "$android_serial" shell am start -n "$bundle_id/.MainActivity" \
    --ez spinon_c114_grid_track_sizing true \
    --es spinon_c114_grid_scene "$scene" \
    --es spinon_c114_grid_run_id "$scene_run_id" \
    > "$scene_dir/launch.log" 2>&1
  await_android_scene "$scene" "$scene_run_id" "$scene_dir"
  if [[ "$scene" == "generic-root-normal" ]]; then
    sleep 1
    adb -s "$android_serial" exec-out screencap -p > "$run_dir/android-avd/screenshot.png"
    [[ -s "$run_dir/android-avd/screenshot.png" ]] || fail "Android screenshot is empty"
  fi
  adb -s "$android_serial" shell am force-stop "$bundle_id" > "$scene_dir/terminate.log" 2>&1
  printf 'PASS\t%s\t%s\n' "$scene" "$scene_run_id" >> "$run_dir/android-avd/matrix.tsv"
done

for scene_index in "${!scenes[@]}"; do
  scene="${scenes[$scene_index]}"
  scene_run_id="$run_id-ios-$scene_index"
  scene_dir="$run_dir/ios-simulator/$scene"
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl launch "$ios_udid" "$bundle_id" \
    --spinon-c114-grid-track-sizing \
    "--spinon-c114-grid-scene=$scene" \
    "--spinon-c114-grid-run-id=$scene_run_id" \
    > "$scene_dir/launch.log" 2>&1
  await_ios_scene "$scene" "$scene_run_id" "$scene_dir"
  if [[ "$scene" == "generic-root-normal" ]]; then
    sleep 1
    xcrun simctl io "$ios_udid" screenshot "$run_dir/ios-simulator/screenshot.png" \
      > "$run_dir/ios-simulator/screenshot.log" 2>&1
    [[ -s "$run_dir/ios-simulator/screenshot.png" ]] || fail "iOS screenshot is empty"
  fi
  xcrun simctl terminate "$ios_udid" "$bundle_id" > "$scene_dir/terminate.log" 2>&1 || true
  printf 'PASS\t%s\t%s\n' "$scene" "$scene_run_id" >> "$run_dir/ios-simulator/matrix.tsv"
done

printf 'C11.4 generic direct-text matrix passed 6/6 on Android API 37 ARM64 AVD and 6/6 on iPhone 17 Pro / iOS 26.2 Simulator: %s\n' "$run_dir"
