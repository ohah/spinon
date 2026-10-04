#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
android_package="dev.spinon.bootstrap"
ios_bundle_id="dev.spinon.bootstrap"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_S03_SHUTDOWN_OUTPUT_DIR:-$repo_root/build/spinon/shutdown-validation/$run_id}"
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

[[ "$(uname -s)" == "Darwin" ]] || fail "Android 에뮬레이터와 iOS 시뮬레이터가 있는 macOS에서 실행해야 합니다"
command -v mise >/dev/null 2>&1 || fail "mise를 찾을 수 없습니다"
command -v adb >/dev/null 2>&1 || fail "Android SDK platform-tools의 adb를 찾을 수 없습니다"
command -v xcrun >/dev/null 2>&1 || fail "Xcode의 xcrun을 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"

v8_dir="${SPINON_V8_DIR:-$repo_root/build/v8-source/v8}"
expected_v8_revision="$(tr -d '\n' < "$repo_root/tools/v8/v8-revision.txt")"
actual_v8_revision="$(git -C "$v8_dir" rev-parse HEAD 2>/dev/null || true)"
[[ "$actual_v8_revision" == "$expected_v8_revision" ]] \
  || fail "V8 checkout이 없거나 고정 revision과 다릅니다. SPINON_V8_DIR에 $expected_v8_revision checkout을 지정하세요"
[[ -z "$(git -C "$v8_dir" status --porcelain --untracked-files=no)" ]] \
  || fail "V8 고정 소스에 추적 파일 변경이 있습니다"
android_v8_args="$v8_dir/out/boson-android-mac/args.gn"
ios_v8_args="$v8_dir/out/boson-ios-sim/args.gn"
[[ -f "$android_v8_args" && -f "$ios_v8_args" ]] \
  || fail "Android·iOS 시뮬레이터 V8 args.gn을 찾을 수 없습니다"
android_target_os="$(rg '^target_os = ' "$android_v8_args" | tail -n 1 || true)"
android_target_cpu="$(rg '^target_cpu = ' "$android_v8_args" | tail -n 1 || true)"
android_jitless="$(rg '^v8_jitless = ' "$android_v8_args" | tail -n 1 || true)"
ios_target_os="$(rg '^target_os = ' "$ios_v8_args" | tail -n 1 || true)"
ios_target_cpu="$(rg '^target_cpu = ' "$ios_v8_args" | tail -n 1 || true)"
ios_jitless="$(rg '^v8_jitless = ' "$ios_v8_args" | tail -n 1 || true)"
[[ "$android_target_os" == 'target_os = "android"' && "$android_target_cpu" == 'target_cpu = "arm64"' ]] \
  || fail "Android 검증 V8은 target_os=android, target_cpu=arm64여야 합니다"
[[ "$android_jitless" == 'v8_jitless = false' ]] \
  || fail "Android 검증 V8은 v8_jitless = false로 빌드되어야 합니다"
[[ "$ios_target_os" == 'target_os = "ios"' && "$ios_target_cpu" == 'target_cpu = "arm64"' ]] \
  || fail "iOS Simulator V8은 target_os=ios, target_cpu=arm64여야 합니다"
[[ "$ios_jitless" == 'v8_jitless = false' ]] \
  || fail "iOS Simulator V8은 v8_jitless = false로 빌드되어야 합니다"

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
  [[ "${#android_emulators[@]}" -eq 1 ]] || fail "실행 중인 Android 에뮬레이터가 ${#android_emulators[@]}개입니다"
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
  [[ "${#ios_simulators[@]}" -eq 1 ]] || fail "실행 중인 iOS 시뮬레이터가 ${#ios_simulators[@]}개입니다"
  ios_udid="${ios_simulators[0]}"
fi

[[ ! -e "$result_dir" ]] || fail "기존 결과를 덮어쓰지 않도록 새 결과 폴더를 지정해야 합니다"
mkdir -p "$result_dir"
printf 'V8 revision=%s\nAndroid serial=%s · %s · Android %s · %s\niOS simulator=%s\n' \
  "$actual_v8_revision" \
  "$android_serial" \
  "$(adb -s "$android_serial" shell getprop ro.product.model | tr -d '\r')" \
  "$(adb -s "$android_serial" shell getprop ro.build.version.release | tr -d '\r')" \
  "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" \
  "$ios_udid" > "$result_dir/environment.txt"
{
  printf 'macOS: %s %s\n' "$(sw_vers -productVersion)" "$(sw_vers -buildVersion)"
  xcodebuild -version | tr '\n' ' '
  printf '\nRust: %s\nBun: %s\n' "$(rustc --version)" "$(bun --version)"
  printf 'iOS runtime: %s\n' "$(xcrun simctl list devices booted | sed -nE 's/^-- (iOS [^-]+) --$/\1/p' | head -n 1)"
  printf 'iOS device: %s\n' "$(xcrun simctl list devices booted | sed -nE 's/^    ([^(]+) \(.*/\1/p' | head -n 1 | xargs)"
  printf 'Android V8: %s · %s · %s\n' "$android_target_os" "$android_target_cpu" "$android_jitless"
  printf 'iOS V8: %s · %s · %s\n' "$ios_target_os" "$ios_target_cpu" "$ios_jitless"
} >> "$result_dir/environment.txt"

printf 'Android 앱 빌드·실행\n'
if ! mise exec -- bun run build:android > "$result_dir/android-build.log" 2>&1; then
  tail -n 80 "$result_dir/android-build.log" >&2
  fail "Android 앱 빌드가 실패했습니다"
fi
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$android_apk" ]] || fail "Android APK가 없습니다: $android_apk"
adb -s "$android_serial" install -r "$android_apk" > "$result_dir/android-install.txt"
adb -s "$android_serial" logcat -c
adb -s "$android_serial" shell am force-stop "$android_package"
adb -s "$android_serial" shell am start -n "$android_package/.MainActivity" \
  --ez spinon_shutdown_probe true > "$result_dir/android-launch.txt"

for _ in $(seq 1 60); do
  adb -s "$android_serial" logcat -d -s SpinonBootstrap:I \
    | rg 'SPINON_SHUTDOWN_PROBE' > "$result_dir/android.log" || true
  rg -Fq 'shutdown_probe=PASS' "$result_dir/android.log" && break
  sleep 1
done
rg -Fq 'shutdown_probe=PASS' "$result_dir/android.log" || fail "Android 실제 V8 세션 종료 검증이 통과하지 않았습니다"
rg -Fq 'active_status=-8' "$result_dir/android.log" || fail "Android 활성 평가 취소 결과가 없습니다"
rg -Fq 'queued_statuses=[-6, -6, -6]' "$result_dir/android.log" || fail "Android 대기 명령 종료 결과가 다릅니다"
rg -Fq 'post_eval_status=-6' "$result_dir/android.log" || fail "Android 종료 뒤 eval 거부 결과가 없습니다"
rg -Fq 'post_dispatch_status=-6' "$result_dir/android.log" || fail "Android 종료 뒤 dispatch 거부 결과가 없습니다"
rg -Fq 'runtime_released=true worker_joined=true' "$result_dir/android.log" || fail "Android 작업자·런타임 종료 결과가 없습니다"
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$result_dir/android.png"

printf 'iOS 앱 빌드·실행\n'
if ! mise exec -- bun run build:ios-sim > "$result_dir/ios-build.log" 2>&1; then
  tail -n 80 "$result_dir/ios-build.log" >&2
  fail "iOS Simulator 앱 빌드가 실패했습니다"
fi
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
[[ -d "$ios_app" ]] || fail "iOS 앱이 없습니다: $ios_app"
xcrun simctl install "$ios_udid" "$ios_app"
xcrun simctl spawn "$ios_udid" log stream --style compact \
  --predicate 'process == "SpinonBootstrap" AND eventMessage CONTAINS[c] "SPINON_SHUTDOWN_PROBE"' \
  > "$result_dir/ios-stream.log" 2>&1 &
ios_stream_pid=$!
sleep 1
xcrun simctl terminate "$ios_udid" "$ios_bundle_id" >/dev/null 2>&1 || true
xcrun simctl launch --terminate-running-process "$ios_udid" "$ios_bundle_id" \
  --spinon-shutdown-probe > "$result_dir/ios-launch.txt"

ios_passed=false
for _ in $(seq 1 60); do
  if rg -Fq 'shutdown_probe=PASS' "$result_dir/ios-stream.log"; then
    ios_passed=true
    break
  fi
  sleep 1
done
kill "$ios_stream_pid" 2>/dev/null || true
wait "$ios_stream_pid" 2>/dev/null || true
ios_stream_pid=""
[[ "$ios_passed" == true ]] || fail "iOS 실제 V8 세션 종료 검증이 통과하지 않았습니다"
rg -Fq 'active_status=-8' "$result_dir/ios-stream.log" || fail "iOS 활성 평가 취소 결과가 없습니다"
rg -Fq 'queued_statuses=[-6, -6, -6]' "$result_dir/ios-stream.log" || fail "iOS 대기 명령 종료 결과가 다릅니다"
rg -Fq 'post_eval_status=-6' "$result_dir/ios-stream.log" || fail "iOS 종료 뒤 eval 거부 결과가 없습니다"
rg -Fq 'post_dispatch_status=-6' "$result_dir/ios-stream.log" || fail "iOS 종료 뒤 dispatch 거부 결과가 없습니다"
rg -Fq 'runtime_released=true worker_joined=true' "$result_dir/ios-stream.log" || fail "iOS 작업자·런타임 종료 결과가 없습니다"
rg 'SPINON_SHUTDOWN_PROBE' "$result_dir/ios-stream.log" > "$result_dir/ios.log"
xcrun simctl io "$ios_udid" screenshot "$result_dir/ios.png" >/dev/null

printf 'Android·iOS 실제 V8 종료 검증 통과\n결과 폴더: %s\n' "$result_dir"
