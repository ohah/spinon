#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
run_id="c114-image-auto-max-20261026T013203Z-7a7aa224-final03"
scene_id="image-auto-max-mixed-tracks"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
bundle_id="dev.spinon.bootstrap"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
run_dir="$repo_root/spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/runs/$run_id/ios-simulator"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

cleanup_app() {
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true
}
trap cleanup_app EXIT

[[ -d "$ios_app" ]] || fail "iOS Simulator app missing: $ios_app"
booted_devices="$(xcrun simctl list devices booted)"
[[ "$booted_devices" == *"iPhone 17 Pro ($ios_udid) (Booted)"* ]] \
  || fail "Planned iPhone 17 Pro / iOS 26.2 Simulator is not booted"
[[ "$(xcrun simctl getenv "$ios_udid" SIMULATOR_MODEL_IDENTIFIER | tail -n 1 | tr -d '\r')" == "iPhone18,1" ]] \
  || fail "Unexpected iOS Simulator model"

mkdir -p "$run_dir"
printf '%s\n' "$run_id" > "$run_dir/run-id.txt"
printf '%s\n' "$(git -C "$repo_root" rev-parse HEAD)" > "$run_dir/source-head.txt"
printf '%s\n' "$booted_devices" > "$run_dir/booted-devices.txt"
printf '%s\n' "$(xcrun simctl getenv "$ios_udid" SIMULATOR_MODEL_IDENTIFIER | tail -n 1 | tr -d '\r')" \
  > "$run_dir/model-identifier.txt"

xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/install.log" 2>&1
xcrun simctl launch "$ios_udid" "$bundle_id" \
  --spinon-c114-grid-track-sizing \
  "--spinon-c114-grid-scene=$scene_id" \
  "--spinon-c114-grid-run-id=$run_id" \
  > "$run_dir/launch.log" 2>&1

runtime_log="$run_dir/runtime.log"
for attempt in {1..60}; do
  xcrun simctl spawn "$ios_udid" log show --style compact --last 2m \
    --predicate 'process == "SpinonBootstrap"' > "$runtime_log" 2>&1
  if rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready" "$runtime_log" \
      && rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$runtime_log"; then
    break
  fi
  sleep 1
done
rg -q "SPINON_C114_GRID_SUMMARY run_id=$run_id scene_id=$scene_id .*status=0 .*layout=ready boxes=2" "$runtime_log" \
  || fail "iOS frame/trace assertion did not pass; inspect $runtime_log"
rg -q 'SPINON_C114_GRID_TRACE_CHUNK run_id='"$run_id" "$runtime_log" \
  || fail "iOS C11.4 trace was not logged"
rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=2 .*surface_texture=' "$runtime_log" \
  || fail "iOS draw/present report is absent"
if rg -q 'SPINON_C114_GRID_ASSERTION_ERROR|SPINON_C114_GRID_TRACE_MISSING|SPINON_C114_GRID_ARGUMENT_ERROR' "$runtime_log"; then
  fail "iOS route reported a C11.4 assertion/configuration error"
fi

sleep 1
xcrun simctl io "$ios_udid" screenshot "$run_dir/screenshot.png" \
  > "$run_dir/screenshot.log" 2>&1
[[ -s "$run_dir/screenshot.png" ]] || fail "iOS screenshot is empty"
xcrun simctl terminate "$ios_udid" "$bundle_id" > "$run_dir/terminate.log" 2>&1 || true

printf 'C11.4 image mixed-track scene passed on iPhone 17 Pro / iOS 26.2 Simulator: %s\n' "$run_dir"
