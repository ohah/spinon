#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
EVIDENCE="${SPINON_C11_NAMED_AREA_EVIDENCE_DIR:-$ROOT/spec/internal/evidence/c11-root-html-grid-auto-minimum-named-area-runtime-2026-11-03}"
ANDROID_SERIAL="${SPINON_ANDROID_EMULATOR_SERIAL:-emulator-5556}"
IOS_UDID="${SPINON_IOS_SIMULATOR_UDID:-ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D}"
ANDROID_APK="$ROOT/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
IOS_APP="$ROOT/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
ANDROID_COMPONENT="dev.spinon.bootstrap/.MainActivity"
BUNDLE_ID="dev.spinon.bootstrap"
RUN_TAG="${SPINON_C11_NAMED_AREA_RUN_TAG:-$(date -u +%Y%m%dT%H%M%S)}"
if (($# == 0)); then
  scenes=(columns-auto-one columns-zero-one columns-auto-zero rows-auto-one rows-zero-one rows-auto-zero)
else
  scenes=("$@")
fi

cd "$ROOT"
cmp "$EVIDENCE/source/original-index.html" examples/bootstrap/index.html
cmp "$EVIDENCE/source/original-root-app.js" examples/bootstrap/root-app.js

restore_sources() {
  cp "$EVIDENCE/source/original-index.html" examples/bootstrap/index.html
  cp "$EVIDENCE/source/original-root-app.js" examples/bootstrap/root-app.js
}
trap restore_sources EXIT INT TERM

for scene in "${scenes[@]}"; do
  case "$scene" in
    columns-auto-one|columns-zero-one|columns-auto-zero|rows-auto-one|rows-zero-one|rows-auto-zero) ;;
    *) echo "지원하지 않는 C11.1 scene: $scene" >&2; exit 2 ;;
  esac
  run_id="c11na-${scene}-${RUN_TAG}"
  output_html="$EVIDENCE/source/scenes/$scene.html"
  cp "$output_html" examples/bootstrap/index.html
  cp "$EVIDENCE/source/runtime-root-app.js" examples/bootstrap/root-app.js

  SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
    mise exec -- bun run build:android > "$EVIDENCE/android/build-$scene.log" 2>&1
  shasum -a 256 "$ANDROID_APK" > "$EVIDENCE/android/apk-$scene.sha256"
  adb -s "$ANDROID_SERIAL" install -r "$ANDROID_APK" > "$EVIDENCE/android/install-$scene.log"
  adb -s "$ANDROID_SERIAL" shell am force-stop "$BUNDLE_ID"
  adb -s "$ANDROID_SERIAL" logcat -c
  adb -s "$ANDROID_SERIAL" shell am start -S -n "$ANDROID_COMPONENT" \
    --ez spinon_root_html_grid_auto_minimum_pixel_capture true \
    --es spinon_root_html_grid_auto_minimum_pixel_capture_run_id "$run_id" \
    > "$EVIDENCE/android/launch-$scene.log"

  pixelcopy_ready=0
  for _ in {1..100}; do
    if adb -s "$ANDROID_SERIAL" logcat -d -s SpinonBootstrap \
      | rg -q "SPINON_C111_ROOT_HTML_AUTO_MINIMUM_PIXELCOPY_SUMMARY status=matched run_id=$run_id"; then
      pixelcopy_ready=1
      break
    fi
    sleep 0.2
  done
  adb -s "$ANDROID_SERIAL" logcat -d -s SpinonBootstrap > "$EVIDENCE/android/spinon-$scene.log"
  test "$pixelcopy_ready" -eq 1
  rg -q "SPINON_C0410_ENVIRONMENT status=0 root_html_environment=true" \
    "$EVIDENCE/android/spinon-$scene.log"
  rg -q "SPINON_C111_ROOT_HTML_AUTO_MINIMUM_PIXELCOPY_SUMMARY status=matched run_id=$run_id.*expected_samples=2.*matched_samples=2" \
    "$EVIDENCE/android/spinon-$scene.log"
  test "$(rg -c "sample=.* exact=true" "$EVIDENCE/android/spinon-$scene.log")" -eq 2
  adb -s "$ANDROID_SERIAL" exec-out screencap -p > "$EVIDENCE/android/$scene-simulator.png"
  adb -s "$ANDROID_SERIAL" exec-out run-as "$BUNDLE_ID" cat \
    "/data/user/0/$BUNDLE_ID/cache/c11-root-html-grid-auto-minimum-pixelcopy/$run_id.png" \
    > "$EVIDENCE/android/$scene-pixelcopy.png"

  SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 \
    mise exec -- bun run build:ios-sim > "$EVIDENCE/ios/build-$scene.log" 2>&1
  shasum -a 256 "$IOS_APP/SpinonBootstrap" > "$EVIDENCE/ios/executable-$scene.sha256"
  xcrun simctl install "$IOS_UDID" "$IOS_APP" > "$EVIDENCE/ios/install-$scene.log"
  xcrun simctl terminate "$IOS_UDID" "$BUNDLE_ID" >/dev/null 2>&1 || true
  xcrun simctl launch "$IOS_UDID" "$BUNDLE_ID" \
    "--spinon-root-html-grid-auto-minimum-metal-surface-pixels=$run_id" \
    > "$EVIDENCE/ios/launch-$scene.log"
  sleep 2
  xcrun simctl spawn "$IOS_UDID" log show --style compact --last 3m \
    --predicate 'process == "SpinonBootstrap"' > "$EVIDENCE/ios/spinon-$scene.log"
  rg -q "SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass" "$EVIDENCE/ios/spinon-$scene.log"
  rg -q "SPINON_C111NA_METAL_PIXEL_SUMMARY status=matched run_id=$run_id.*expected_samples=2.*matched_samples=2" \
    "$EVIDENCE/ios/spinon-$scene.log"
  test "$(rg -c "SPINON_C111NA_METAL_PIXEL_SAMPLE run_id=$run_id.*exact=true" "$EVIDENCE/ios/spinon-$scene.log")" -eq 2
  xcrun simctl io "$IOS_UDID" screenshot "$EVIDENCE/ios/$scene-simulator.png" >/dev/null
done

restore_sources
cmp "$EVIDENCE/source/original-index.html" examples/bootstrap/index.html
cmp "$EVIDENCE/source/original-root-app.js" examples/bootstrap/root-app.js
