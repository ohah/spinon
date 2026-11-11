#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
ios_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
android_serial="emulator-5554"
android_avd="spinon_c114_five_fit_20261111"
source_avd="spinon_api37_1_compat16k"
avd_root="$HOME/.android/avd"
adb_bin="/Users/yoonhb/Library/Android/sdk/platform-tools/adb"
emulator_bin="/Users/yoonhb/Library/Android/sdk/emulator/emulator"
avdmanager_bin="/Users/yoonhb/Library/Android/sdk/cmdline-tools/latest/bin/avdmanager"
emulator_pid=""
ios_started=0
avd_created=0

cleanup() {
  local status=$?
  trap - EXIT
  if [[ -n "$emulator_pid" ]]; then
    "$adb_bin" -s "$android_serial" emu kill >/dev/null 2>&1 || true
    for cleanup_attempt in {1..30}; do
      if ! "$adb_bin" devices -l | rg -q "^$android_serial[[:space:]]"; then
        break
      fi
      sleep 1
    done
    if kill -0 "$emulator_pid" >/dev/null 2>&1; then
      kill "$emulator_pid" >/dev/null 2>&1 || true
    fi
    wait "$emulator_pid" >/dev/null 2>&1 || true
  fi
  if [[ "$ios_started" == "1" ]]; then
    xcrun simctl shutdown "$ios_udid" >/dev/null 2>&1 || true
  fi
  if [[ "$avd_created" == "1" ]]; then
    if "$avdmanager_bin" delete avd --name "$android_avd" \
      > "$evidence_dir/android-avd-delete.log" 2>&1; then
      avd_created=0
    else
      printf 'avdmanager delete failed; removing exact task-owned paths\n' \
        >> "$evidence_dir/android-avd-delete.log"
      if [[ -d "$avd_root/$android_avd.avd" ]]; then
        find "$avd_root/$android_avd.avd" -depth -delete \
          >> "$evidence_dir/android-avd-delete.log" 2>&1 || status=1
      fi
      if [[ -f "$avd_root/$android_avd.ini" ]]; then
        find "$avd_root/$android_avd.ini" -depth -delete \
          >> "$evidence_dir/android-avd-delete.log" 2>&1 || status=1
      fi
      avd_created=0
    fi
  fi
  {
    xcrun simctl list devices booted
    "$adb_bin" devices -l
    pgrep -alf "SpinonBootstrap|emulator -avd $android_avd" || true
    if [[ -e "$HOME/.android/avd/$android_avd.ini" || -e "$HOME/.android/avd/$android_avd.avd" ]]; then
      printf 'task_avd=present\n'
      status=1
    elif [[ "$avd_created" == "0" ]]; then
      printf 'task_avd=removed\n'
    fi
  } > "$evidence_dir/cleanup-wrapper.log"
  if xcrun simctl list devices booted | rg -q 'Booted' ||
     "$adb_bin" devices -l | rg -q "^$android_serial[[:space:]]"; then
    status=1
    printf 'cleanup=failed\n' >> "$evidence_dir/cleanup-wrapper.log"
  else
    printf 'cleanup=passed\n' >> "$evidence_dir/cleanup-wrapper.log"
  fi
  exit "$status"
}
trap cleanup EXIT

if xcrun simctl list devices booted | rg -q 'Booted'; then
  printf 'refusing to change an already booted iOS Simulator\n' >&2
  exit 1
fi
if "$adb_bin" devices -l | awk 'NR > 1 && NF >= 2 { found=1 } END { exit !found }'; then
  printf 'refusing to change an already attached Android device\n' >&2
  exit 1
fi
if [[ -e "$HOME/.android/avd/$android_avd.ini" || -e "$HOME/.android/avd/$android_avd.avd" ]]; then
  printf 'refusing to overwrite an existing task AVD name: %s\n' "$android_avd" >&2
  exit 1
fi

xcrun simctl boot "$ios_udid"
ios_started=1
xcrun simctl bootstatus "$ios_udid" -b > "$evidence_dir/ios-boot.log" 2>&1
if [[ ! -d "$avd_root/$source_avd.avd" || ! -f "$avd_root/$source_avd.ini" ]]; then
  printf 'source API 37 AVD is incomplete: %s\n' "$source_avd" >&2
  exit 1
fi
avd_created=1
/bin/cp -cR "$avd_root/$source_avd.avd" "$avd_root/$android_avd.avd"
cp "$avd_root/$source_avd.ini" "$avd_root/$android_avd.ini"
sed -i '' "s#$source_avd#$android_avd#g" "$avd_root/$android_avd.ini"
sed -i '' "s/^avd.ini.displayname=.*/avd.ini.displayname=$android_avd/" \
  "$avd_root/$android_avd.avd/config.ini"
find "$avd_root/$android_avd.avd" -type f -name '*.lock' -depth -delete
{
  printf 'source_avd=%s\ntask_avd=%s\ncopy_mode=APFS clonefile\n' "$source_avd" "$android_avd"
  shasum -a 256 "$avd_root/$source_avd.ini" "$avd_root/$source_avd.avd/config.ini"
  rg '^(abi.type|hw.cpu.arch|image.sysdir.1|tag.id)=' \
    "$avd_root/$android_avd.avd/config.ini"
} > "$evidence_dir/android-avd-clone.log"
"$emulator_bin" -avd "$android_avd" -no-snapshot-save \
  > "$evidence_dir/android-emulator.log" 2>&1 &
emulator_pid=$!
android_connected=0
for attempt in {1..120}; do
  if "$adb_bin" -s "$android_serial" get-state 2>/dev/null | rg -q '^device$'; then
    android_connected=1
    break
  fi
  if ! kill -0 "$emulator_pid" >/dev/null 2>&1; then
    printf 'Android emulator exited before adb connected\n' >&2
    exit 1
  fi
  sleep 1
done
if [[ "$android_connected" != "1" ]]; then
  printf 'Android emulator did not connect to adb within 120 seconds\n' >&2
  exit 1
fi
android_ready=0
for attempt in {1..120}; do
  if [[ "$("$adb_bin" -s "$android_serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; then
    android_ready=1
    break
  fi
  sleep 1
done
if [[ "$android_ready" != "1" ]]; then
  printf 'Android AVD did not complete boot\n' >&2
  exit 1
fi

cloned_app_path="$("$adb_bin" -s "$android_serial" shell pm path dev.spinon.bootstrap \
  2>/dev/null | sed -n 's/^package://p' | tr -d '\r' | head -n 1 || true)"
{
  printf 'task_avd=%s\npreexisting_app=%s\n' "$android_avd" \
    "$([[ -n "$cloned_app_path" ]] && printf present || printf absent)"
  if [[ -n "$cloned_app_path" ]]; then
    "$adb_bin" -s "$android_serial" uninstall dev.spinon.bootstrap
  fi
  remaining_app_path="$("$adb_bin" -s "$android_serial" shell pm path dev.spinon.bootstrap \
    2>/dev/null | sed -n 's/^package://p' | tr -d '\r' | head -n 1 || true)"
  printf 'remaining_app_path=%s\n' "$remaining_app_path"
  [[ -z "$remaining_app_path" ]]
} > "$evidence_dir/android-task-avd-preparation.log" 2>&1

bash "$evidence_dir/capture-five-fit-simulators.sh"
