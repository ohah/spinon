#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
android_package="dev.spinon.bootstrap"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_R06_FAIRNESS_OUTPUT_DIR:-$repo_root/build/spinon/priority-fairness-validation/$run_id}"
pass_marker='status=0 priority_probe=PASS'
stream_marker='status=0 priority_stream_probe=PASS capacity=64 initial_high=63 late_high=1024 accepted_high=1087 background_seq=2 background_order=1088'
probe_started=false
probe_passed=false

fail() {
  printf '실패: %s\n' "$1" >&2
  exit 1
}

[[ "$(uname -s)" == "Darwin" ]] || fail "이 검증은 Android SDK가 설치된 macOS에서 실행해야 합니다"
command -v mise >/dev/null 2>&1 || fail "mise를 찾을 수 없습니다"
command -v adb >/dev/null 2>&1 || fail "Android SDK platform-tools의 adb를 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"

android_serial="${SPINON_ANDROID_EMULATOR_SERIAL:-}"
if [[ -n "$android_serial" ]]; then
  [[ "$android_serial" =~ ^emulator-[0-9]+$ ]] \
    || fail "Android 대상은 emulator-숫자 형식이어야 합니다. 실기기 serial은 허용하지 않습니다"
  if ! adb devices | awk -v serial="$android_serial" '$1 == serial && $2 == "device" { found = 1 } END { exit !found }'; then
    fail "지정한 Android 에뮬레이터가 연결·실행 중이지 않습니다: $android_serial"
  fi
else
  android_emulators=()
  while IFS=$'\t' read -r serial state _; do
    if [[ "$serial" =~ ^emulator-[0-9]+$ && "$state" == "device" ]]; then
      android_emulators+=("$serial")
    fi
  done < <(adb devices | tail -n +2)
  if [[ "${#android_emulators[@]}" -ne 1 ]]; then
    fail "실행 중인 Android 에뮬레이터가 ${#android_emulators[@]}개입니다. 하나만 연결하거나 SPINON_ANDROID_EMULATOR_SERIAL을 지정하세요"
  fi
  android_serial="${android_emulators[0]}"
fi

[[ ! -e "$result_dir" && ! -L "$result_dir" ]] || fail "결과 폴더가 이미 있습니다: $result_dir"
mkdir -p "$result_dir"
android_log="$result_dir/android.log"
build_log="$result_dir/android-build.log"
launch_log="$result_dir/android-launch.txt"
device_list="$result_dir/adb-devices.txt"
adb devices -l > "$device_list"

android_model="$(adb -s "$android_serial" shell getprop ro.product.model | tr -d '\r')"
android_release="$(adb -s "$android_serial" shell getprop ro.build.version.release | tr -d '\r')"
android_abi="$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
[[ "$android_abi" == "arm64-v8a" ]] || fail "현재 Android smoke APK는 arm64-v8a 에뮬레이터만 지원합니다: $android_abi"
printf 'Android 에뮬레이터만 사용: %s · %s · Android %s · %s\n' \
  "$android_serial" "$android_model" "$android_release" "$android_abi"

cleanup() {
  if [[ "$probe_started" == true && "$probe_passed" != true ]]; then
    adb -s "$android_serial" shell am force-stop "$android_package" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

cd "$repo_root"
if ! mise exec -- bun run build:android > "$build_log" 2>&1; then
  tail -n 100 "$build_log" >&2
  fail "Android 에뮬레이터 빌드가 실패했습니다"
fi
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$apk" ]] || fail "Android APK가 없습니다: $apk"

adb -s "$android_serial" install -r "$apk" > "$result_dir/install.txt"
adb -s "$android_serial" logcat -c
adb -s "$android_serial" shell am force-stop "$android_package"
probe_started=true
adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" --ez spinon_priority_probe true \
  > "$launch_log"

passed=false
for _ in $(seq 1 120); do
  adb -s "$android_serial" logcat -d -s SpinonBootstrap:V \
    | rg 'SPINON_PRIORITY_(PROBE|FAIRNESS_PROBE)' > "$android_log" || true
  if rg -Fq "$pass_marker" "$android_log" && rg -Fq "$stream_marker" "$android_log"; then
    passed=true
    break
  fi
  if rg -Fq 'SPINON_PRIORITY_PROBE status=-' "$android_log" \
    || rg -Fq 'SPINON_PRIORITY_FAIRNESS_PROBE status=-' "$android_log" \
    || rg -Fq 'priority_probe=FAIL' "$android_log" \
    || rg -Fq 'priority_stream_probe=FAIL' "$android_log"; then
    cat "$android_log" >&2
    fail "Android 실제 V8 우선순위 유입 진단이 실패했습니다"
  fi
  sleep 1
done
[[ "$passed" == "true" ]] || {
  cat "$android_log" >&2
  fail "120초 안에 Android 검증 통과 로그를 받지 못했습니다"
}
rg -q 'owner_tid=[1-9][0-9]*' "$android_log" || fail "Android 로그에 유효한 owner thread가 없습니다"
adb -s "$android_serial" exec-out screencap -p > "$result_dir/android.png"
shasum -a 256 "$android_log" "$result_dir/android.png" > "$result_dir/SHA256SUMS"
probe_passed=true

printf 'Android 우선순위·유입 검증 통과\n'
cat "$android_log"
printf '로그와 캡처: %s\n' "$result_dir"
