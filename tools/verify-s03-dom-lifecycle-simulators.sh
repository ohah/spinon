#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
android_package="dev.spinon.bootstrap"
ios_bundle_id="dev.spinon.bootstrap"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_S03_DOM_GC_OUTPUT_DIR:-$repo_root/build/spinon/dom-lifecycle-validation/$run_id}"
ios_stream_pid=""

fail() {
  printf '실패: %s\n결과 폴더: %s\n' "$1" "$result_dir" >&2
  exit 1
}

cleanup() {
  if [[ -n "$ios_stream_pid" ]]; then
    kill "$ios_stream_pid" 2>/dev/null || true
    wait "$ios_stream_pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT

[[ "$(uname -s)" == "Darwin" ]] || fail "Android와 iOS 시뮬레이터가 있는 macOS에서 실행해야 합니다"
command -v mise >/dev/null 2>&1 || fail "mise를 찾을 수 없습니다"
command -v adb >/dev/null 2>&1 || fail "Android SDK platform-tools의 adb를 찾을 수 없습니다"
command -v xcrun >/dev/null 2>&1 || fail "Xcode의 xcrun을 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"

android_serial="${SPINON_ANDROID_EMULATOR_SERIAL:-}"
if [[ -n "$android_serial" ]]; then
  [[ "$android_serial" =~ ^emulator-[0-9]+$ ]] || fail "Android 대상은 emulator-숫자 형식이어야 합니다"
  adb devices | awk -v serial="$android_serial" '$1 == serial && $2 == "device" { found = 1 } END { exit !found }' \
    || fail "지정한 Android 에뮬레이터가 실행 중이지 않습니다: $android_serial"
else
  android_emulators=()
  while IFS=$'\t' read -r serial state _; do
    if [[ "$serial" =~ ^emulator-[0-9]+$ && "$state" == "device" ]]; then
      android_emulators+=("$serial")
    fi
  done < <(adb devices | tail -n +2)
  [[ "${#android_emulators[@]}" -eq 1 ]] \
    || fail "실행 중인 Android 에뮬레이터가 ${#android_emulators[@]}개입니다"
  android_serial="${android_emulators[0]}"
fi

booted_ios="$(xcrun simctl list devices booted)"
ios_udid="${SPINON_IOS_SIMULATOR_UDID:-}"
if [[ -n "$ios_udid" ]]; then
  printf '%s\n' "$booted_ios" | rg -Fq "($ios_udid) (Booted)" \
    || fail "지정한 iOS 시뮬레이터가 부팅되어 있지 않습니다: $ios_udid"
else
  ios_simulators=()
  while IFS= read -r udid; do
    [[ -n "$udid" ]] && ios_simulators+=("$udid")
  done < <(printf '%s\n' "$booted_ios" | sed -nE 's/.*\(([0-9A-Fa-f-]{36})\) \(Booted\)[[:space:]]*$/\1/p')
  [[ "${#ios_simulators[@]}" -eq 1 ]] \
    || fail "실행 중인 iOS 시뮬레이터가 ${#ios_simulators[@]}개입니다"
  ios_udid="${ios_simulators[0]}"
fi

[[ ! -e "$result_dir" ]] || fail "기존 검증 결과를 덮어쓰지 않도록 새 결과 폴더를 지정해야 합니다"
mkdir -p "$result_dir"
printf 'V8 revision=%s\nAndroid serial=%s\niOS simulator=%s\n' \
  "$(cat "$repo_root/tools/v8/v8-revision.txt")" "$android_serial" "$ios_udid" \
  > "$result_dir/environment.txt"

printf 'Android 에뮬레이터 앱을 빌드합니다.\n'
if ! SPINON_ENABLE_S03_DOM_GC_FIXTURE=1 mise exec -- bun run build:android \
  > "$result_dir/android-build.log" 2>&1; then
  tail -n 80 "$result_dir/android-build.log" >&2
  fail "Android 앱 빌드가 실패했습니다"
fi
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$android_apk" ]] || fail "Android APK가 없습니다: $android_apk"
adb -s "$android_serial" install -r "$android_apk" > "$result_dir/android-install.txt"
adb -s "$android_serial" logcat -c
adb -s "$android_serial" shell am force-stop "$android_package"
adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" \
  --ez spinon_dom_gc true > "$result_dir/android-launch.txt"

android_pass_marker='DOM-GC 검증 status=0 dom_gc=PASS'
android_failed=false
for _ in $(seq 1 90); do
  adb -s "$android_serial" logcat -d -s SpinonBootstrap:I \
    | rg 'SPINON_RUNTIME_UI DOM-GC 검증' > "$result_dir/android.log" || true
  if rg -Fq "$android_pass_marker" "$result_dir/android.log"; then
    break
  fi
  if rg -Fq 'dom_gc=FAIL' "$result_dir/android.log"; then
    android_failed=true
    break
  fi
  sleep 1
done
[[ "$android_failed" == false ]] || fail "Android DOM wrapper 수명 probe가 실패했습니다"
rg -Fq "$android_pass_marker" "$result_dir/android.log" \
  || fail "Android 90초 안에 통과 결과를 기록하지 못했습니다"
rg -Fq 'attached_wrapper_recreated=PASS' "$result_dir/android.log" \
  || fail "Android wrapper 재생성 결과가 없습니다"
rg -Fq 'callback_closure_root=PASS' "$result_dir/android.log" \
  || fail "Android callback closure root 보존 결과가 없습니다"
rg -Fq 'callback_closure_release=PASS' "$result_dir/android.log" \
  || fail "Android callback 교체 뒤 wrapper 회수 결과가 없습니다"
rg -Fq 'large_registry_retained=PASS' "$result_dir/android.log" \
  || fail "Android 16,385개 V8 wrapper 유지 결과가 없습니다"
rg -Fq 'large_registry_released=PASS' "$result_dir/android.log" \
  || fail "Android 16,385개 V8 wrapper 회수 결과가 없습니다"
rg -Fq 'large_registry_nodes=16385' "$result_dir/android.log" \
  || fail "Android V8 wrapper 대량 회수 대상 수가 다릅니다"
rg -Fq 'large_registry_empty_wrappers=16385' "$result_dir/android.log" \
  || fail "Android 대량 회수 weak wrapper 수가 다릅니다"
rg -Fq 'initial_baseline_return=PASS' "$result_dir/android.log" \
  || fail "Android 초기 root fixture 뒤 자원 기준선 복귀 결과가 없습니다"
rg -Fq 'repeated_baseline=PASS' "$result_dir/android.log" \
  || fail "Android 반복 회수 자원 기준선 결과가 없습니다"
rg -Fq 'baseline_return_rounds=6/6' "$result_dir/android.log" \
  || fail "Android 반복 회수 전체 회차 결과가 없습니다"
rg -Fq 'collector_scan_stats=PASS' "$result_dir/android.log" \
  || fail "Android collector scan 계수 검사가 통과하지 않았습니다"
rg -Fq 'collector_succeeded=PASS' "$result_dir/android.log" \
  || fail "Android collector 오류 검사가 통과하지 않았습니다"
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$result_dir/android.png"

printf 'iOS Simulator 앱을 빌드합니다.\n'
if ! SPINON_ENABLE_S03_DOM_GC_FIXTURE=1 mise exec -- bun run build:ios-sim \
  > "$result_dir/ios-build.log" 2>&1; then
  tail -n 80 "$result_dir/ios-build.log" >&2
  fail "iOS Simulator 앱 빌드가 실패했습니다"
fi
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
[[ -d "$ios_app" ]] || fail "iOS 앱이 없습니다: $ios_app"
xcrun simctl install "$ios_udid" "$ios_app"
xcrun simctl spawn "$ios_udid" log stream --style compact \
  --predicate 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_R06_IOS"' \
  > "$result_dir/ios-stream.log" 2>&1 &
ios_stream_pid=$!
sleep 1
xcrun simctl terminate "$ios_udid" "$ios_bundle_id" >/dev/null 2>&1 || true
xcrun simctl launch --terminate-running-process "$ios_udid" "$ios_bundle_id" \
  --spinon-dom-gc-auto > "$result_dir/ios-launch.txt"

ios_pass_marker='통과 · Rust 노드·UTF-16 문자열·weak wrapper 기준선 대조'
ios_failed=false
for _ in $(seq 1 90); do
  if rg -Fq "$ios_pass_marker" "$result_dir/ios-stream.log"; then
    break
  fi
  if rg -Fq '실패 · Rust 노드·UTF-16 문자열·weak wrapper 기준선 대조' \
    "$result_dir/ios-stream.log"; then
    ios_failed=true
    break
  fi
  sleep 1
done
kill "$ios_stream_pid" 2>/dev/null || true
wait "$ios_stream_pid" 2>/dev/null || true
ios_stream_pid=""
[[ "$ios_failed" == false ]] || fail "iOS DOM wrapper 수명 probe가 실패했습니다"
rg -Fq "$ios_pass_marker" "$result_dir/ios-stream.log" \
  || fail "iOS 90초 안에 통과 결과를 기록하지 못했습니다"
rg -Fq 'callback closure root·호출 · 통과' "$result_dir/ios-stream.log" \
  || fail "iOS callback closure root 보존 결과가 없습니다"
rg -Fq 'callback 교체 후 해제·기준선 복귀 · 통과' "$result_dir/ios-stream.log" \
  || fail "iOS callback 교체 뒤 wrapper 회수 결과가 없습니다"
rg -Fq '16,385개 wrapper 유지 · 통과' "$result_dir/ios-stream.log" \
  || fail "iOS 16,385개 V8 wrapper 유지 결과가 없습니다"
rg -Fq '16,385개 wrapper 해제 · 통과 · empty=16385' "$result_dir/ios-stream.log" \
  || fail "iOS 16,385개 V8 wrapper 회수 결과가 없습니다"
rg -Fq '반복 수명 회수 · 통과 · 6/6회' "$result_dir/ios-stream.log" \
  || fail "iOS 반복 회수 자원 기준선 결과가 없습니다"
rg -Fq '회수 scan 계수 일관성 · 통과 · scanned=live+empty' "$result_dir/ios-stream.log" \
  || fail "iOS collector scan 계수 검사가 통과하지 않았습니다"
rg 'DOM 자원 기준|DOM 자원 최종 기준선|callback closure|16,385개 wrapper|반복 수명 회수|회수 scan 계수|통과 · Rust 노드' \
  "$result_dir/ios-stream.log" > "$result_dir/ios.log" || true
xcrun simctl io "$ios_udid" screenshot "$result_dir/ios.png" >/dev/null

printf 'Android·iOS DOM wrapper 수명 probe를 통과했습니다.\n결과 폴더: %s\n' "$result_dir"
