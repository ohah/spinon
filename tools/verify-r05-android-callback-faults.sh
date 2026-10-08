#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
android_package="dev.spinon.bootstrap"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_R05_CALLBACK_OUTPUT_DIR:-$repo_root/build/spinon/r05-callback-faults/$run_id}"
logcat_pid=""
expected_api_full="${SPINON_R05_CALLBACK_EXPECTED_API_FULL:-37.2}"

case "$expected_api_full" in
  35)
    expected_api_level="35"
    expected_page_size="4096"
    ;;
  37.0)
    expected_api_level="37"
    expected_page_size="4096"
    ;;
  37.1)
    expected_api_level="37"
    expected_page_size="16384"
    ;;
  37.2)
    expected_api_level="37"
    expected_page_size="16384"
    ;;
  *)
    printf '실패: 허용되지 않은 API 대상입니다: %s\n' "$expected_api_full" >&2
    exit 1
    ;;
esac

fail() {
  printf '실패: %s\n결과 폴더: %s\n' "$1" "$result_dir" >&2
  exit 1
}

cleanup() {
  if [[ -n "$logcat_pid" ]]; then
    kill "$logcat_pid" 2>/dev/null || true
    wait "$logcat_pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT

[[ "$(uname -s)" == "Darwin" ]] || fail "macOS에서 실행해야 합니다"
command -v mise >/dev/null 2>&1 || fail "mise를 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"

sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
adb="$sdk_dir/platform-tools/adb"
[[ -x "$adb" ]] || fail "Android SDK adb를 찾을 수 없습니다: $adb"

android_serial="${SPINON_ANDROID_EMULATOR_SERIAL:-}"
[[ "$android_serial" =~ ^emulator-[0-9]+$ ]] \
  || fail "SPINON_ANDROID_EMULATOR_SERIAL에 emulator-숫자 대상을 지정하세요. 실기기는 허용하지 않습니다"
"$adb" devices | awk -v serial="$android_serial" \
  '$1 == serial && $2 == "device" { found = 1 } END { exit !found }' \
  || fail "지정한 Android 에뮬레이터가 실행 중이지 않습니다: $android_serial"

api_level="$("$adb" -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
api_full="$("$adb" -s "$android_serial" shell getprop ro.build.version.sdk_full | tr -d '\r')"
page_size="$("$adb" -s "$android_serial" shell getconf PAGE_SIZE | tr -d '\r')"
if [[ "$expected_api_full" == "35" ]]; then
  [[ "$api_level" == "$expected_api_level" && "$page_size" == "$expected_page_size" ]] \
    || fail "API 35·4KB AVD가 아닙니다: api=$api_level api_full=${api_full:-미제공} page_size=$page_size"
else
  [[ "$api_level" == "$expected_api_level" && "$api_full" == "$expected_api_full" \
      && "$page_size" == "$expected_page_size" ]] \
    || fail "API ${expected_api_full}·${expected_page_size}B AVD가 아닙니다: api=$api_level api_full=${api_full:-미제공} page_size=$page_size"
fi

[[ ! -e "$result_dir" ]] || fail "기존 결과를 덮어쓰지 않도록 새 경로를 지정하세요"
mkdir -p "$result_dir"
{
  printf 'source_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  printf 'source_worktree_dirty=%s\n' "$(git -C "$repo_root" status --porcelain --untracked-files=no | wc -l | tr -d ' ')"
  printf 'android_serial=%s\n' "$android_serial"
  printf 'model=%s\n' "$("$adb" -s "$android_serial" shell getprop ro.product.model | tr -d '\r')"
  printf 'expected_api_full=%s\nexpected_page_size=%s\n' "$expected_api_full" "$expected_page_size"
  printf 'api=%s\napi_full=%s\npage_size=%s\n' "$api_level" "${api_full:-unavailable}" "$page_size"
  printf 'abi=%s\n' "$("$adb" -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
} > "$result_dir/environment.txt"
{
  for source_file in \
    platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java \
    platforms/android/app/src/main/java/dev/spinon/bootstrap/R05PresentFenceProbe.java \
    platforms/android/app/src/debug/java/dev/spinon/bootstrap/R05PresentFenceFailureFixture.java \
    platforms/android/app/src/release/java/dev/spinon/bootstrap/R05PresentFenceFailureFixture.java \
    tools/verify-r05-android-callback-faults.sh; do
    shasum -a 256 "$repo_root/$source_file"
  done
} > "$result_dir/source-files.sha256"

printf 'Android debug APK 빌드\n'
if ! mise exec -- bun run build:android > "$result_dir/android-build.log" 2>&1; then
  tail -n 80 "$result_dir/android-build.log" >&2
  fail "Android debug APK 빌드가 실패했습니다"
fi

apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$apk" ]] || fail "Android APK가 없습니다: $apk"
shasum -a 256 "$apk" > "$result_dir/apk.sha256"
"$adb" -s "$android_serial" install -r "$apk" > "$result_dir/install.log"
"$adb" -s "$android_serial" logcat -c
"$adb" -s "$android_serial" shell am force-stop "$android_package"
"$adb" -s "$android_serial" logcat -v threadtime -s SpinonBootstrap:V \
  > "$result_dir/logcat.txt" 2>&1 &
logcat_pid=$!
"$adb" -s "$android_serial" shell am start -n "$android_package/.MainActivity" \
  --ez spinon_r05_callback_faults true > "$result_dir/launch.log"

for _ in $(seq 1 100); do
  if rg -Fq 'SPINON_R05_CALLBACK_FAULT_SUMMARY status=PASS' "$result_dir/logcat.txt"; then
    break
  fi
  if rg -Fq 'SPINON_R05_CALLBACK_FAULT_SUMMARY status=FAIL' "$result_dir/logcat.txt"; then
    tail -n 100 "$result_dir/logcat.txt" >&2
    fail "callback fault fixture가 실패했습니다"
  fi
  sleep 1
done

rg -Fq 'SPINON_R05_CALLBACK_FAULT_SUMMARY status=PASS' "$result_dir/logcat.txt" \
  || fail "100초 안에 fixture PASS 결과가 없습니다"
rg -Fq 'scenario=actual_main_handler_timeout outcome=PASS' "$result_dir/logcat.txt" \
  || fail "실제 Handler timeout 결과가 없습니다"
rg -Fq 'scenario=timeout_callback_race outcome=PASS' "$result_dir/logcat.txt" \
  || fail "timeout/callback 경합 결과가 없습니다"
rg -Fq 'scenario=callback_executor_overflow outcome=PASS' "$result_dir/logcat.txt" \
  || fail "callback executor overflow 결과가 없습니다"

app_pid="$("$adb" -s "$android_serial" shell pidof "$android_package" | tr -d '\r')"
[[ -n "$app_pid" ]] || fail "검증 후 앱 PID가 없습니다"
printf 'app_pid=%s\n' "$app_pid" > "$result_dir/app-pid.txt"
"$adb" -s "$android_serial" exec-out screencap -p > "$result_dir/result.png"
printf 'R05 callback fault fixture 통과: API %s\n결과 폴더: %s\n' \
  "$expected_api_full" "$result_dir"
