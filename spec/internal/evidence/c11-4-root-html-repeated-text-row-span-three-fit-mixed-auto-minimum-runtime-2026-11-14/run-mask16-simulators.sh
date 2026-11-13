#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
fixture="$evidence_dir/root-html-mask16-simulator-fixture.html"
android_asset="$repo_root/platforms/android/app/src/main/assets/root-html-default-rendering.html"
ios_asset="$repo_root/platforms/ios/Resources/root-html-default-rendering.html"
sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
adb_bin="$sdk_dir/platform-tools/adb"
emulator_bin="$sdk_dir/emulator/emulator"
avd_name="spinon_api37_2_presenttime_probe_20261018"
android_serial="emulator-5554"
app_id="dev.spinon.bootstrap"
apk_path="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
ios_bundle_id="dev.spinon.bootstrap"
ios_runtime="com.apple.CoreSimulator.SimRuntime.iOS-26-2"
ios_device_type="com.apple.CoreSimulator.SimDeviceType.iPhone-17-Pro"
ios_simulator_udid=""
emulator_pid=""
temporary_dir="$(mktemp -d "${TMPDIR:-/tmp}/spinon-c114-mask16.XXXXXX")"
assets_restored=0
assets_backed_up=0
artifacts_backed_up=0
android_apk_existed=0
ios_app_existed=0
android_apk_original_sha=""
ios_app_original_sha=""

cleanup() {
  status=$?
  if [[ -n "$emulator_pid" ]]; then
    "$adb_bin" -s "$android_serial" shell am force-stop "$app_id" >/dev/null 2>&1 || true
    "$adb_bin" -s "$android_serial" emu kill >/dev/null 2>&1 || true
    wait "$emulator_pid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$ios_simulator_udid" ]]; then
    xcrun simctl terminate "$ios_simulator_udid" "$ios_bundle_id" >/dev/null 2>&1 || true
    xcrun simctl shutdown "$ios_simulator_udid" >/dev/null 2>&1 || true
    xcrun simctl delete "$ios_simulator_udid" >/dev/null 2>&1 || true
  fi
  if [[ "$assets_restored" != "1" && "$assets_backed_up" == "1" ]]; then
    cp -p "$temporary_dir/android-original.html" "$android_asset"
    cp -p "$temporary_dir/ios-original.html" "$ios_asset"
  fi
  if [[ "$artifacts_backed_up" == "1" ]]; then
    if [[ "$android_apk_existed" == "1" ]]; then
      cp -p "$temporary_dir/android-original.apk" "$apk_path"
    else
      rm -f "$apk_path"
    fi
    if [[ "$ios_app_existed" == "1" ]]; then
      rm -rf "$ios_app"
      ditto "$temporary_dir/ios-original.app" "$ios_app"
    else
      rm -rf "$ios_app"
    fi
    {
      printf 'android_apk_existed_before=%s\n' "$android_apk_existed"
      printf 'ios_app_existed_before=%s\n' "$ios_app_existed"
      if [[ "$android_apk_existed" == "1" ]]; then
        printf 'android_apk_restored_sha256=%s\n' "$(shasum -a 256 "$apk_path" | awk '{print $1}')"
        printf 'android_apk_original_sha256=%s\n' "$android_apk_original_sha"
      fi
      if [[ "$ios_app_existed" == "1" ]]; then
        printf 'ios_app_executable_restored_sha256=%s\n' "$(shasum -a 256 "$ios_app/SpinonBootstrap" | awk '{print $1}')"
        printf 'ios_app_executable_original_sha256=%s\n' "$ios_app_original_sha"
      fi
    } > "$evidence_dir/build-artifact-restoration.txt"
  fi
  rm -rf "$temporary_dir"
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

if [[ ! -x "$adb_bin" || ! -x "$emulator_bin" ]]; then
  echo "Android SDK emulator/adb not found under $sdk_dir" >&2
  exit 2
fi
if [[ ! -f "$fixture" || ! -f "$android_asset" || ! -f "$ios_asset" ]]; then
  echo "Mask-16 simulator fixture or a platform Root HTML asset is missing" >&2
  exit 2
fi
if ! cmp -s "$android_asset" "$ios_asset"; then
  echo "Android and iOS Root HTML assets differ; refusing temporary replacement" >&2
  exit 2
fi
if ! git -C "$repo_root" diff --quiet -- "$android_asset" "$ios_asset"; then
  echo "A Root HTML asset already has local edits; refusing to replace it" >&2
  exit 2
fi
if "$adb_bin" devices | rg -q "^${android_serial}[[:space:]]+device$"; then
  echo "Android serial $android_serial is already in use" >&2
  exit 2
fi
if xcrun simctl list devices booted | rg -q 'iPhone 17 Pro'; then
  echo "An iPhone 17 Pro simulator is already booted; refusing to take it over" >&2
  exit 2
fi

cp -p "$android_asset" "$temporary_dir/android-original.html"
cp -p "$ios_asset" "$temporary_dir/ios-original.html"
assets_backed_up=1
if [[ -f "$apk_path" ]]; then
  cp -p "$apk_path" "$temporary_dir/android-original.apk"
  android_apk_existed=1
  android_apk_original_sha="$(shasum -a 256 "$apk_path" | awk '{print $1}')"
fi
if [[ -d "$ios_app" ]]; then
  ditto "$ios_app" "$temporary_dir/ios-original.app"
  ios_app_existed=1
  ios_app_original_sha="$(shasum -a 256 "$ios_app/SpinonBootstrap" | awk '{print $1}')"
fi
artifacts_backed_up=1
fixture_sha="$(shasum -a 256 "$fixture" | awk '{print $1}')"
original_sha="$(shasum -a 256 "$android_asset" | awk '{print $1}')"
cp "$fixture" "$android_asset"
cp "$fixture" "$ios_asset"

if ! SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android \
  > "$evidence_dir/android-build.log" 2>&1; then
  tail -n 80 "$evidence_dir/android-build.log" >&2
  exit 1
fi
if ! SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim \
  > "$evidence_dir/ios-build.log" 2>&1; then
  tail -n 80 "$evidence_dir/ios-build.log" >&2
  exit 1
fi

cp -p "$temporary_dir/android-original.html" "$android_asset"
cp -p "$temporary_dir/ios-original.html" "$ios_asset"
assets_restored=1
restored_sha="$(shasum -a 256 "$android_asset" | awk '{print $1}')"
if [[ "$restored_sha" != "$original_sha" ]] || ! cmp -s "$android_asset" "$ios_asset"; then
  echo "Root HTML platform assets did not restore byte-for-byte" >&2
  exit 1
fi
{
  printf 'temporary_fixture_sha256=%s\n' "$fixture_sha"
  printf 'restored_original_sha256=%s\n' "$restored_sha"
  printf 'assets_restored_byte_for_byte=true\n'
} > "$evidence_dir/asset-restoration.txt"

"$emulator_bin" -avd "$avd_name" -read-only -no-snapshot-load -no-snapshot-save \
  -no-window -no-audio -no-boot-anim -gpu swiftshader_indirect \
  > "$evidence_dir/android-emulator.log" 2>&1 &
emulator_pid=$!
booted=""
for attempt in $(seq 1 240); do
  device_state="$("$adb_bin" -s "$android_serial" get-state 2>/dev/null || true)"
  if [[ "$device_state" == "device" ]]; then
    booted="$("$adb_bin" -s "$android_serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')"
  fi
  [[ "$booted" == "1" ]] && break
  sleep 1
done
if [[ "$booted" != "1" ]]; then
  echo "Android API 37 AVD did not finish booting" >&2
  exit 1
fi
{
  printf 'avd=%s\n' "$avd_name"
  printf 'api=%s\n' "$("$adb_bin" -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
  printf 'abi=%s\n' "$("$adb_bin" -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
  printf 'density=%s\n' "$("$adb_bin" -s "$android_serial" shell wm density | tr -d '\r')"
  printf 'emulator_flags=-read-only -no-snapshot-load -no-snapshot-save -gpu swiftshader_indirect\n'
  printf 'temporary_overlay=true\n'
} > "$evidence_dir/android-environment.txt"
"$adb_bin" -s "$android_serial" install -r "$apk_path" > "$evidence_dir/android-install.log"
"$adb_bin" -s "$android_serial" logcat -c
"$adb_bin" -s "$android_serial" shell am start -n "$app_id/.MainActivity" \
  --ez spinon_c0412_root_html_default_render true > "$evidence_dir/android-launch.log"
sleep 6
"$adb_bin" -s "$android_serial" logcat -d -s SpinonBootstrap:I \
  > "$evidence_dir/android-runtime.log"
rg -q "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE case=default .*sha256=$fixture_sha" \
  "$evidence_dir/android-runtime.log"
rg -q 'SPINON_C0410_ENVIRONMENT status=0 root_html_environment=true .*layout=ready' \
  "$evidence_dir/android-runtime.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$evidence_dir/android-runtime.log"
"$adb_bin" -s "$android_serial" shell screencap -p /sdcard/spinon-c114-mask16.png
"$adb_bin" -s "$android_serial" pull /sdcard/spinon-c114-mask16.png \
  "$evidence_dir/android-mask16.png" >/dev/null
"$adb_bin" -s "$android_serial" shell am force-stop "$app_id"
"$adb_bin" -s "$android_serial" emu kill >/dev/null
wait "$emulator_pid" >/dev/null 2>&1 || true
emulator_pid=""

ios_simulator_udid="$(xcrun simctl create \
  "Spinon-C11.4-mask16-20261114" "$ios_device_type" "$ios_runtime")"
xcrun simctl boot "$ios_simulator_udid"
xcrun simctl bootstatus "$ios_simulator_udid" -b > "$evidence_dir/ios-boot.log"
xcrun simctl install "$ios_simulator_udid" "$ios_app"
xcrun simctl launch --terminate-running-process "$ios_simulator_udid" "$ios_bundle_id" \
  --spinon-c0412-root-html-default-render > "$evidence_dir/ios-launch.log"
sleep 6
xcrun simctl spawn "$ios_simulator_udid" log show --last 3m --style compact \
  --predicate 'process == "SpinonBootstrap"' > "$evidence_dir/ios-runtime.log"
rg -q "SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE case=default .*sha256=$fixture_sha" \
  "$evidence_dir/ios-runtime.log"
rg -q 'SPINON_C0410_ENVIRONMENT status=0 root_html_environment=true .*layout=ready' \
  "$evidence_dir/ios-runtime.log"
rg -q 'SPINON_C0410_DRAW .*status=0 presented' "$evidence_dir/ios-runtime.log"
xcrun simctl io "$ios_simulator_udid" screenshot "$evidence_dir/ios-mask16.png" \
  > "$evidence_dir/ios-screenshot.log" 2>&1
printf 'ios_udid=%s\nmodel=iPhone 17 Pro\nruntime=iOS 26.2\n' "$ios_simulator_udid" \
  > "$evidence_dir/ios-environment.txt"
xcrun simctl terminate "$ios_simulator_udid" "$ios_bundle_id" >/dev/null 2>&1 || true
xcrun simctl shutdown "$ios_simulator_udid" >/dev/null 2>&1 || true
xcrun simctl delete "$ios_simulator_udid" >/dev/null
ios_simulator_udid=""

shasum -a 256 "$evidence_dir"/android-mask16.png "$evidence_dir"/ios-mask16.png \
  > "$evidence_dir/screenshot-sha256.txt"
printf 'android=pass api37-arm64-sha256-source=%s\nios=pass iphone17pro-ios26.2-sha256-source=%s\n' \
  "$fixture_sha" "$fixture_sha"
printf 'fixture_test_build=c04-runtime-gpu-fixture-test enabled\n'
printf 'cleanup=temporary HTML source removed; build app outputs restored; read-only AVD stopped; disposable iOS simulator deleted\n'
