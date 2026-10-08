#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../../" && pwd)"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_ANDROID_DEVICE_OUTPUT_DIR:-$repo_root/spec/internal/evidence/r05-android-physical-callback-faults-2026-10-08/run-$run_id}"
serial="${SPINON_ANDROID_DEVICE_SERIAL:-}"
package="dev.spinon.bootstrap"
logcat_pid=""
temporary_log=""
app_pid=""

fail() {
  printf '실패: %s\n결과 폴더: %s\n' "$1" "$result_dir" >&2
  exit 1
}

cleanup() {
  if [[ -n "$logcat_pid" ]]; then
    kill "$logcat_pid" 2>/dev/null || true
    wait "$logcat_pid" 2>/dev/null || true
  fi
  if [[ -n "$temporary_log" && -f "$temporary_log" ]]; then
    if [[ -n "$app_pid" ]]; then
      awk -v pid="$app_pid" '$3 == pid' "$temporary_log" \
        > "$result_dir/logcat.txt"
    else
      cp "$temporary_log" "$result_dir/logcat-tag-window-unfiltered.txt"
    fi
    rm -f "$temporary_log"
  fi
  if [[ -n "$serial" ]]; then
    current_focus="$("$adb" -s "$serial" shell dumpsys window 2>/dev/null \
      | rg 'mCurrentFocus=' | head -n 1 || true)"
    if [[ "$current_focus" == *"$package"* ]]; then
      "$adb" -s "$serial" shell monkey -p com.android.chrome 1 \
        > "$result_dir/restore-chrome.log" 2>&1 || true
      for _ in $(seq 1 10); do
        current_focus="$("$adb" -s "$serial" shell dumpsys window 2>/dev/null \
          | rg 'mCurrentFocus=' | head -n 1 || true)"
        [[ "$current_focus" == *"com.android.chrome"* ]] && break
        sleep 1
      done
      "$adb" -s "$serial" shell dumpsys window \
        | rg 'mCurrentFocus=|mFocusedApp=' > "$result_dir/foreground-after-restore.txt" || true
    else
      printf '사용자 또는 시스템이 다른 앱으로 이동해 Chrome 복귀를 생략했습니다.\n' \
        > "$result_dir/restore-chrome.log"
    fi
  fi
}

[[ "$(uname -s)" == "Darwin" ]] || fail "macOS에서 실행해야 합니다"
command -v mise >/dev/null 2>&1 || fail "mise를 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"
[[ -n "$serial" && "$serial" != emulator-* ]] \
  || fail "실제 USB Android 기기의 ADB serial을 지정하세요"

sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
adb="$sdk_dir/platform-tools/adb"
[[ -x "$adb" ]] || fail "Android SDK adb를 찾을 수 없습니다: $adb"
"$adb" devices -l | awk -v serial="$serial" \
  '$1 == serial && $2 == "device" && $0 ~ / usb:/ { found = 1 } END { exit !found }' \
  || fail "지정한 USB 실기기가 연결되지 않았습니다"

api_level="$("$adb" -s "$serial" shell getprop ro.build.version.sdk | tr -d '\r')"
api_full="$("$adb" -s "$serial" shell getprop ro.build.version.sdk_full | tr -d '\r')"
page_size="$("$adb" -s "$serial" shell getconf PAGE_SIZE | tr -d '\r')"
abi="$("$adb" -s "$serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
[[ "$api_level" == 36 && "$api_full" == 36.1 && "$page_size" == 4096 \
   && "$abi" == arm64-v8a ]] \
  || fail "계획과 다른 기기 조건입니다: api=$api_level api_full=$api_full page_size=$page_size abi=$abi"

window_state="$("$adb" -s "$serial" shell dumpsys window)"
power_state="$("$adb" -s "$serial" shell dumpsys power)"
[[ "$power_state" == *"mWakefulness=Awake"* ]] || fail "기기 화면이 깨어 있지 않습니다"
[[ "$window_state" != *"mDreamingLockscreen=true"* ]] \
  || fail "기기가 잠금 화면에 있습니다"

[[ ! -e "$result_dir" ]] || fail "기존 결과를 덮어쓰지 않도록 새 경로를 지정하세요"
mkdir -p "$result_dir"
trap cleanup EXIT

v8_dir="${SPINON_V8_DIR:-$repo_root/build/v8-source/v8}"
v8_required_revision="$(tr -d '\n' < "$repo_root/tools/v8/v8-revision.txt")"
v8_checkout_revision="$(git -C "$v8_dir" rev-parse HEAD 2>/dev/null || printf 'missing')"
v8_checkout_clean=false
if [[ "$v8_checkout_revision" == "$v8_required_revision" ]] \
    && [[ -z "$(git -C "$v8_dir" status --porcelain 2>/dev/null)" ]]; then
  v8_checkout_clean=true
fi
[[ "$v8_checkout_clean" == true ]] \
  || fail "고정 V8 revision이 clean checkout 상태가 아닙니다"
{
  printf 'device_alias=physical-android-01\ntransport=usb\n'
  printf 'model=%s\nproduct=%s\n' \
    "$("$adb" -s "$serial" shell getprop ro.product.model | tr -d '\r')" \
    "$("$adb" -s "$serial" shell getprop ro.product.device | tr -d '\r')"
  printf 'api=%s\napi_full=%s\npage_size=%s\nabi=%s\n' \
    "$api_level" "$api_full" "$page_size" "$abi"
  printf 'fingerprint=%s\n' \
    "$("$adb" -s "$serial" shell getprop ro.build.fingerprint | tr -d '\r')"
  printf 'source_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  printf 'source_tracked_changes=%s\n' \
    "$(git -C "$repo_root" status --porcelain --untracked-files=no | wc -l | tr -d ' ')"
  printf 'v8_required_revision=%s\nv8_checkout_revision=%s\nv8_checkout_clean=%s\n' \
    "$v8_required_revision" "$v8_checkout_revision" "$v8_checkout_clean"
  printf 'stay_on_while_plugged_in=%s\nscreen_off_timeout_ms=%s\n' \
    "$("$adb" -s "$serial" shell settings get global stay_on_while_plugged_in | tr -d '\r')" \
    "$("$adb" -s "$serial" shell settings get system screen_off_timeout | tr -d '\r')"
  printf 'foreground_before=%s\n' \
    "$(printf '%s\n' "$window_state" | rg 'mCurrentFocus=' | head -n 1)"
} > "$result_dir/environment.txt"
for source_file in \
  platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java \
  platforms/android/app/src/main/java/dev/spinon/bootstrap/R05PresentFenceProbe.java \
  platforms/android/app/src/main/java/dev/spinon/bootstrap/R08GpuDemo.java \
  platforms/android/app/src/debug/java/dev/spinon/bootstrap/R05PresentFenceFailureFixture.java \
  platforms/android/app/src/debug/java/dev/spinon/bootstrap/R05AppliedTransactionLifecycleFixture.java \
  platforms/android/app/src/release/java/dev/spinon/bootstrap/R05PresentFenceFailureFixture.java \
  tools/v8/v8-revision.txt \
  tools/verify-r05-android-callback-faults.sh; do
  shasum -a 256 "$repo_root/$source_file"
done > "$result_dir/source-files.sha256"

if ! mise exec -- bun run build:android > "$result_dir/android-build.log" 2>&1; then
  tail -n 80 "$result_dir/android-build.log" >&2
  fail "Android debug APK 빌드가 실패했습니다"
fi

apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$apk" ]] || fail "debug APK를 찾을 수 없습니다"
shasum -a 256 "$apk" > "$result_dir/apk.sha256"
"$adb" -s "$serial" install -r "$apk" > "$result_dir/install.log"

# Global log buffer는 비우지 않는다. 새 process PID로 시험 구간을 분리한다.
"$adb" -s "$serial" shell am force-stop "$package"
temporary_log="$result_dir/logcat-tag-window.tmp"
"$adb" -s "$serial" logcat -v threadtime -s SpinonBootstrap:V \
  > "$temporary_log" 2>&1 &
logcat_pid=$!
"$adb" -s "$serial" shell am start -n "$package/.MainActivity" \
  --ez spinon_r05_callback_faults true > "$result_dir/launch.log"

for _ in $(seq 1 20); do
  app_pid="$("$adb" -s "$serial" shell pidof "$package" 2>/dev/null \
    | tr -d '\r' | awk '{ print $1 }' || true)"
  [[ -n "$app_pid" ]] && break
  sleep 1
done
[[ -n "$app_pid" ]] || fail "Spinon 앱 process가 시작되지 않았습니다"
printf 'app_pid=%s\n' "$app_pid" > "$result_dir/app-pid.txt"

result_status="timeout"
for _ in $(seq 1 100); do
  if awk -v pid="$app_pid" '$3 == pid' "$temporary_log" \
      | rg -Fq 'SPINON_R05_CALLBACK_FAULT_SUMMARY status=PASS'; then
    result_status="pass"
    break
  fi
  if awk -v pid="$app_pid" '$3 == pid' "$temporary_log" \
      | rg -Fq 'SPINON_R05_CALLBACK_FAULT_SUMMARY status=FAIL'; then
    result_status="fail"
    break
  fi
  sleep 1
done

sleep 1
kill "$logcat_pid" 2>/dev/null || true
wait "$logcat_pid" 2>/dev/null || true
logcat_pid=""
awk -v pid="$app_pid" '$3 == pid' "$temporary_log" > "$result_dir/logcat.txt"
rm -f "$temporary_log"
temporary_log=""

# pass 한 줄만으로는 통과 처리하지 않고 scenario를 각각 확인한다.
required_markers=(
  'SPINON_R05_CALLBACK_FAULT_SUMMARY status=PASS checks=12 failures=0 race_iterations=100 idle_samples=60 pending=0 queue=0 active=0'
  'SPINON_R05_CALLBACK_FAULT scenario=idle_60s outcome=PASS samples=60 first_bad=none pending=0 queue=0 active=0'
  'SPINON_R05_APPLIED_TRANSACTION scenario=actual_surface_callback_baseline outcome=PASS'
  'SPINON_R05_APPLIED_TRANSACTION scenario=actual_surface_callback_after_recreation outcome=PASS'
  'SPINON_R05_APPLIED_TRANSACTION scenario=actual_surface_callback_recovery outcome=PASS'
  'SPINON_R05_CALLBACK_FAULT scenario=actual_main_handler_timeout outcome=PASS'
  'SPINON_R05_CALLBACK_FAULT scenario=callback_executor_overflow outcome=PASS'
  'SPINON_R05_CALLBACK_FAULT scenario=timeout_callback_race outcome=PASS iterations=100'
)
for marker in "${required_markers[@]}"; do
  if ! rg -Fq "$marker" "$result_dir/logcat.txt"; then
    result_status="incomplete"
  fi
done
queued_count="$(rg -c -F \
  'SPINON_R05_APPLIED_TRANSACTION scenario=callback_queued outcome=PASS' \
  "$result_dir/logcat.txt" || true)"
[[ "$queued_count" == 3 ]] || result_status="incomplete"
for pair in '1 1' '2 1' '3 2'; do
  read -r revision generation <<< "$pair"
  if ! rg -q "SPINON_R05_DRAW renderer=opengl_es draw_seq=[0-9]+ revision=$revision generation=$generation" \
      "$result_dir/logcat.txt"; then
    result_status="incomplete"
  fi
done
printf 'runner_result=%s\nqueued_callbacks=%s\n' \
  "$result_status" "$queued_count" > "$result_dir/assertions.txt"

app_pid_after="$("$adb" -s "$serial" shell pidof "$package" 2>/dev/null \
  | tr -d '\r' | awk '{ print $1 }' || true)"
printf 'app_pid_after=%s\n' "$app_pid_after" >> "$result_dir/assertions.txt"
[[ "$app_pid_after" == "$app_pid" ]] || result_status="incomplete"

{
  printf 'api=%s\napi_full=%s\npage_size=%s\nabi=%s\n' \
    "$("$adb" -s "$serial" shell getprop ro.build.version.sdk | tr -d '\r')" \
    "$("$adb" -s "$serial" shell getprop ro.build.version.sdk_full | tr -d '\r')" \
    "$("$adb" -s "$serial" shell getconf PAGE_SIZE | tr -d '\r')" \
    "$("$adb" -s "$serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
} > "$result_dir/postflight-device.txt"
rg -Fxq 'api=36' "$result_dir/postflight-device.txt" \
  && rg -Fxq 'api_full=36.1' "$result_dir/postflight-device.txt" \
  && rg -Fxq 'page_size=4096' "$result_dir/postflight-device.txt" \
  && rg -Fxq 'abi=arm64-v8a' "$result_dir/postflight-device.txt" \
  || result_status="incomplete"

# screenshot을 먼저 저장한 뒤 hierarchy와 Activity 상태를 읽는다.
"$adb" -s "$serial" exec-out screencap -p > "$result_dir/result.png"
"$adb" -s "$serial" shell uiautomator dump /sdcard/spinon-r05-window.xml \
  > "$result_dir/uiautomator-dump.log" 2>&1
"$adb" -s "$serial" shell cat /sdcard/spinon-r05-window.xml \
  > "$result_dir/ui-hierarchy.xml"
"$adb" -s "$serial" shell dumpsys activity activities \
  > "$result_dir/activity.txt"
"$adb" -s "$serial" shell dumpsys window \
  | rg 'mCurrentFocus=|mFocusedApp=' > "$result_dir/window-focus.txt"
printf 'result=%s\n' "$result_status" >> "$result_dir/assertions.txt"

if [[ "$result_status" != pass ]]; then
  printf '실기기 fixture 결과: %s\n결과 폴더: %s\n' "$result_status" "$result_dir" >&2
  exit 1
fi
printf 'Android 실기기 callback fixture 통과: API %s\n결과 폴더: %s\n' \
  "$api_full" "$result_dir"
