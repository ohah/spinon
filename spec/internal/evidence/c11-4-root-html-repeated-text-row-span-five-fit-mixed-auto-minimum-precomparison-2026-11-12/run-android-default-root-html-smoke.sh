#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-/Users/yoonhb/Library/Android/sdk}}"
emulator_bin="$sdk_dir/emulator/emulator"
adb_bin="$sdk_dir/platform-tools/adb"
avd_name="spinon_api37_2_presenttime_probe_20261018"
serial="emulator-5554"
app_id="dev.spinon.bootstrap"
apk_path="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
emulator_pid=""

cleanup() {
  if "$adb_bin" -s "$serial" get-state >/dev/null 2>&1; then
    "$adb_bin" -s "$serial" shell am force-stop "$app_id" >/dev/null 2>&1 || true
    "$adb_bin" -s "$serial" emu kill >/dev/null 2>&1 || true
  fi
  if [[ -n "$emulator_pid" ]]; then
    wait "$emulator_pid" >/dev/null 2>&1 || true
  fi
  "$adb_bin" kill-server >/dev/null 2>&1 || true
}
trap cleanup EXIT

"$emulator_bin" -avd "$avd_name" -read-only -no-snapshot-load -no-snapshot-save \
  -no-window -no-audio -gpu swiftshader_indirect \
  > "$evidence_dir/android-emulator.log" 2>&1 &
emulator_pid=$!

"$adb_bin" -s "$serial" wait-for-device
booted=""
for attempt in $(seq 1 180); do
  booted="$("$adb_bin" -s "$serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')"
  [[ "$booted" == "1" ]] && break
  sleep 1
done
if [[ "$booted" != "1" ]]; then
  echo "Android AVD did not finish booting" >&2
  exit 1
fi

{
  printf 'avd=%s\n' "$avd_name"
  printf 'api=%s\n' "$("$adb_bin" -s "$serial" shell getprop ro.build.version.sdk | tr -d '\r')"
  printf 'abi=%s\n' "$("$adb_bin" -s "$serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
  printf 'density=%s\n' "$("$adb_bin" -s "$serial" shell wm density | tr -d '\r')"
  printf 'emulator_flags=-read-only -no-snapshot-load -no-snapshot-save -gpu swiftshader_indirect\n'
} > "$evidence_dir/android-smoke-environment.txt"

"$adb_bin" -s "$serial" install -r "$apk_path" > "$evidence_dir/android-install.log"
for appearance in light dark; do
  if [[ "$appearance" == "light" ]]; then
    "$adb_bin" -s "$serial" shell cmd uimode night no
  else
    "$adb_bin" -s "$serial" shell cmd uimode night yes
  fi
  "$adb_bin" -s "$serial" logcat -c
  "$adb_bin" -s "$serial" shell am start -n "$app_id/.MainActivity" \
    --ez spinon_c0412_root_html_default_render true \
    > "$evidence_dir/android-launch-$appearance.log"
  sleep 5
  "$adb_bin" -s "$serial" logcat -d -s SpinonBootstrap:I \
    > "$evidence_dir/android-$appearance.log"
  rg -q 'SPINON_C0412_ROOT_HTML_DEFAULT_RENDER_SOURCE case=default' \
    "$evidence_dir/android-$appearance.log"
  rg -q 'SPINON_C0410_ENVIRONMENT status=0 root_html_environment=true .*layout=ready' \
    "$evidence_dir/android-$appearance.log"
  rg -q 'SPINON_C0410_DRAW .*status=0 presented' \
    "$evidence_dir/android-$appearance.log"
  "$adb_bin" -s "$serial" shell screencap -p /sdcard/spinon-default.png
  "$adb_bin" -s "$serial" pull /sdcard/spinon-default.png \
    "$evidence_dir/android-default-$appearance.png" >/dev/null
  "$adb_bin" -s "$serial" shell am force-stop "$app_id"
done

printf 'result=light-dark captured; layout=ready; draw=status 0 presented\n'
printf 'cleanup=read-only AVD shutdown; ADB daemon stopped by trap\n'
