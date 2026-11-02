#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-percentage-runtime.html"
probe="$evidence_dir/root-app-runtime-probe.js"
source_index="$repo_root/examples/bootstrap/index.html"
source_root_app="$repo_root/examples/bootstrap/root-app.js"
generated_dir="$repo_root/build/spinon/bootstrap"
artifact_dir="$repo_root/build/spinon/artifacts"
android_serial="${SPINON_ANDROID_EMULATOR_SERIAL:-emulator-5556}"
ios_udid="${SPINON_IOS_SIMULATOR_UDID:-9DCCE836-7659-4914-B6C1-13F54BFFA9DB}"
skip_android="${SPINON_SKIP_ANDROID:-0}"
backup_dir="$(mktemp -d "${TMPDIR:-/private/tmp}/spinon-c11-percent.XXXXXX")"
android_installed=0
ios_installed=0
generated_dir_existed=0
[[ "$skip_android" == 0 || "$skip_android" == 1 ]]

mkdir -p "$artifact_dir" "$evidence_dir"
cp -p "$source_index" "$backup_dir/index.html"
cp -p "$source_root_app" "$backup_dir/root-app.js"
if [[ -d "$generated_dir" ]]; then generated_dir_existed=1; fi
mkdir -p "$backup_dir/generated"
for asset in app.js index.html root-app.js; do
  if [[ -f "$generated_dir/$asset" ]]; then
    cp -p "$generated_dir/$asset" "$backup_dir/generated/$asset"
  fi
done

restore_sources() {
  set +e
  if [[ "$android_installed" == 1 ]]; then
    adb -s "$android_serial" shell am force-stop dev.spinon.bootstrap >/dev/null 2>&1
  fi
  if [[ "$ios_installed" == 1 ]]; then
    xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap >/dev/null 2>&1
  fi
  cp -p "$backup_dir/index.html" "$source_index"
  cp -p "$backup_dir/root-app.js" "$source_root_app"
  for asset in app.js index.html root-app.js; do
    if [[ -f "$backup_dir/generated/$asset" ]]; then
      cp -p "$backup_dir/generated/$asset" "$generated_dir/$asset"
    else
      rm -f "$generated_dir/$asset"
    fi
  done
  if [[ "$generated_dir_existed" == 0 ]]; then rmdir "$generated_dir" 2>/dev/null; fi
  rm -f "$backup_dir/index.html" "$backup_dir/root-app.js" \
    "$backup_dir/generated/app.js" "$backup_dir/generated/index.html" \
    "$backup_dir/generated/root-app.js" "$backup_dir/android-apk-entries.txt" \
    "$backup_dir/android-runtime.log" "$backup_dir/ios-runtime.log"
  rmdir "$backup_dir/generated" "$backup_dir"
}
trap restore_sources EXIT INT TERM

normalize_evidence_log() {
  perl -0pi -e 's/[\t ]+$//mg; s/\n+\z/\n/' "$1"
}

printf 'C11 percentage runtime source manifest\n' > "$evidence_dir/candidate-source.sha256"
shasum -a 256 \
  crates/spinon-style-to-layout/src/projection/style_values.rs \
  crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs \
  crates/spinon-style-to-layout/src/projection/style_values/root_html_grid_tracks.rs \
  crates/spinon-style-to-layout/src/projection/style_values/grid_values/root_html_grid_track_tests.rs \
  crates/spinon-runtime/src/session/tests/root_html_grid_fr.rs \
  crates/spinon-runtime/src/session/tests/root_html_grid_fr/positive_matrix.rs \
  crates/spinon-runtime/src/session/tests/root_html_grid_fr/rejection_matrix.rs \
  tests/fixtures/css/c11/root-html-grid-percentage-runtime.html \
  "$probe" >> "$evidence_dir/candidate-source.sha256"

cp "$fixture" "$source_index"
cp "$probe" "$source_root_app"

xcrun simctl list devices booted | rg -F "iPhone 17 Pro ($ios_udid) (Booted)"

if [[ "$skip_android" == 0 ]]; then
[[ "$(adb -s "$android_serial" get-state 2>/dev/null)" == device ]]
[[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" == 37 ]]

env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  mise exec -- bun run build:android > "$evidence_dir/android-build.log" 2>&1
normalize_evidence_log "$evidence_dir/android-build.log"

android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$android_apk" ]]
android_artifact="$artifact_dir/android-c11-root-html-grid-percentage-2026-11-02.apk"
cp "$android_apk" "$android_artifact"
android_index_entry="assets/index.html"
android_root_app_entry="assets/root-app.js"
unzip -Z1 "$android_apk" > "$backup_dir/android-apk-entries.txt"
rg -q -x "$android_index_entry" "$backup_dir/android-apk-entries.txt"
rg -q -x "$android_root_app_entry" "$backup_dir/android-apk-entries.txt"
unzip -p "$android_apk" "$android_index_entry" | cmp - "$fixture"
printf 'apk=%s\nindex_entry=%s\nroot_app_entry=%s\nindex_sha256=' \
  "$android_artifact" "$android_index_entry" "$android_root_app_entry" \
  > "$evidence_dir/android-packaged-assets.txt"
unzip -p "$android_apk" "$android_index_entry" | shasum -a 256 \
  | awk '{print $1}' >> "$evidence_dir/android-packaged-assets.txt"
printf 'root_app_sha256=' >> "$evidence_dir/android-packaged-assets.txt"
unzip -p "$android_apk" "$android_root_app_entry" | shasum -a 256 \
  | awk '{print $1}' >> "$evidence_dir/android-packaged-assets.txt"

adb -s "$android_serial" install -r "$android_apk" > "$evidence_dir/android-install.log"
android_installed=1
adb -s "$android_serial" shell am start -n dev.spinon.bootstrap/.MainActivity \
  > "$evidence_dir/android-launch.log"
android_pid=""
for attempt in $(seq 1 30); do
  android_pid="$(adb -s "$android_serial" shell pidof dev.spinon.bootstrap \
    | tr -d '\r' | awk '{print $1}' || true)"
  if [[ -n "$android_pid" ]]; then
    adb -s "$android_serial" logcat --pid="$android_pid" -d -v threadtime \
      > "$backup_dir/android-runtime.log"
  if rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=' "$backup_dir/android-runtime.log"; then break; fi
  fi
  sleep 1
done
cp "$backup_dir/android-runtime.log" "$evidence_dir/android-runtime.log"
normalize_evidence_log "$evidence_dir/android-runtime.log"
adb -s "$android_serial" exec-out screencap -p \
  > "$evidence_dir/android-api37-avd-percentage-overflow.png"
[[ -n "$android_pid" ]]
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_ASSET' "$backup_dir/android-runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_ROUTE source=index.html' "$backup_dir/android-runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$backup_dir/android-runtime.log"
rg -q 'layout=ready boxes=15' "$backup_dir/android-runtime.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=15' "$backup_dir/android-runtime.log"
adb -s "$android_serial" shell am force-stop dev.spinon.bootstrap
android_installed=0
[[ -z "$(adb -s "$android_serial" shell pidof dev.spinon.bootstrap | tr -d '\r')" ]]
fi

env \
  SPINON_ENABLE_C04_RUNTIME_GPU=1 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 \
  SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=0 \
  SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0 \
  mise exec -- bun run build:ios-sim > "$evidence_dir/ios-simulator-build.log" 2>&1
normalize_evidence_log "$evidence_dir/ios-simulator-build.log"

ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
ios_executable="$ios_app/SpinonBootstrap"
[[ -d "$ios_app" && -f "$ios_executable" ]]
ios_index="$(find "$ios_app" -type f -name index.html -print -quit)"
[[ -n "$ios_index" ]]
cmp "$ios_index" "$fixture"
ios_artifact="$artifact_dir/ios-c11-root-html-grid-percentage-2026-11-02"
cp "$ios_executable" "$ios_artifact"
printf 'app=%s\nindex=%s\nindex_sha256=' "$ios_app" "$ios_index" \
  > "$evidence_dir/ios-packaged-assets.txt"
shasum -a 256 "$ios_index" | awk '{print $1}' >> "$evidence_dir/ios-packaged-assets.txt"

xcrun simctl install "$ios_udid" "$ios_app"
ios_installed=1
ios_launch_output="$(xcrun simctl launch --terminate-running-process "$ios_udid" dev.spinon.bootstrap)"
printf '%s\n' "$ios_launch_output" > "$evidence_dir/ios-launch.log"
ios_pid="$(printf '%s\n' "$ios_launch_output" | awk '{print $NF}')"
[[ "$ios_pid" =~ ^[0-9]+$ ]]
for attempt in $(seq 1 30); do
  xcrun simctl spawn "$ios_udid" log show --last 2m --style compact \
    --predicate "processID == $ios_pid" > "$backup_dir/ios-runtime.log" 2>/dev/null || true
  if rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=' "$backup_dir/ios-runtime.log"; then break; fi
  sleep 1
done
cp "$backup_dir/ios-runtime.log" "$evidence_dir/ios-26_2-simulator-runtime.log"
normalize_evidence_log "$evidence_dir/ios-26_2-simulator-runtime.log"
xcrun simctl io "$ios_udid" screenshot "$evidence_dir/ios-26_2-simulator-percentage-overflow.png" \
  > "$evidence_dir/ios-screenshot.log" 2>&1
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=' "$backup_dir/ios-runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_ROUTE source=index.html' "$backup_dir/ios-runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' "$backup_dir/ios-runtime.log"
rg -q 'layout=ready boxes=15' "$backup_dir/ios-runtime.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented boxes=15' "$backup_dir/ios-runtime.log"
xcrun simctl terminate "$ios_udid" dev.spinon.bootstrap
ios_installed=0

if [[ "$skip_android" == 0 ]]; then
  printf 'Android API 37 AVD and iPhone 17 Pro iOS 26.2 Simulator passed; app processes stopped.\n'
else
  printf 'iPhone 17 Pro iOS 26.2 Simulator passed; Android capture was skipped; app processes stopped.\n'
fi
