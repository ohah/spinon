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
  local exit_status=$?
  if [[ -n "$ios_stream_pid" ]]; then
    kill "$ios_stream_pid" 2>/dev/null || true
    wait "$ios_stream_pid" 2>/dev/null || true
  fi
  return "$exit_status"
}
trap cleanup EXIT

scan_sample_sequence_is_complete() {
  local log_path="$1"
  local sample_marker
  local sample_index
  local expected_index
  local sample_indices=()

  [[ -r "$log_path" ]] || return 1
  while IFS= read -r sample_marker; do
    sample_index="${sample_marker#scan_sample_index=}"
    [[ "$sample_index" =~ ^[0-9]+$ ]] || return 1
    sample_indices+=("$sample_index")
  done < <(rg -o 'scan_sample_index=[0-9]+' "$log_path" || true)

  if [[ "${#sample_indices[@]}" -ne 10 ]]; then
    printf 'scan sample metadata count=%s expected=10\n' "${#sample_indices[@]}" >&2
    return 1
  fi
  for expected_index in {1..10}; do
    if [[ "${sample_indices[$((expected_index - 1))]}" != "$expected_index" ]]; then
      printf 'scan sample metadata index at position %s is %s expected=%s\n' \
        "$expected_index" "${sample_indices[$((expected_index - 1))]}" "$expected_index" >&2
      return 1
    fi
  done
}

run_scan_sample_gate_self_test() {
  local test_dir
  local sample_index
  local negative_control_status=0
  test_dir="$(mktemp -d)"
  for sample_index in {1..10}; do
    printf 'scan_sample_index=%s,seq=%s\n' "$sample_index" "$sample_index"
  done > "$test_dir/ten-records.log"
  scan_sample_sequence_is_complete "$test_dir/ten-records.log" || {
    rm -rf "$test_dir"
    printf 'scan sample gate rejected the 10-record positive control\n' >&2
    return 1
  }
  for sample_index in {1..9}; do
    printf 'scan_sample_index=%s,seq=%s\n' "$sample_index" "$sample_index"
  done > "$test_dir/nine-records.log"
  if (
    trap cleanup EXIT
    scan_sample_sequence_is_complete "$test_dir/nine-records.log" \
      || fail "synthetic 9-record negative control"
  ) 2> "$test_dir/nine-negative-control.log"; then
    rm -rf "$test_dir"
    printf 'scan sample verifier returned zero for the 9-record negative control\n' >&2
    return 1
  else
    negative_control_status=$?
  fi
  if [[ "$negative_control_status" -ne 1 ]] \
      || ! rg -Fq 'scan sample metadata count=9 expected=10' "$test_dir/nine-negative-control.log" \
      || ! rg -Fq '실패: synthetic 9-record negative control' "$test_dir/nine-negative-control.log"; then
    rm -rf "$test_dir"
    printf 'scan sample negative control failed with unexpected exit=%s\n' \
      "$negative_control_status" >&2
    return 1
  fi
  rm -rf "$test_dir"
  printf 'scan sample gate self-test passed: 10 accepted; 9 rejected with exit=%s\n' \
    "$negative_control_status"
}

if [[ "${SPINON_S03_VERIFY_SCAN_GATE_SELF_TEST:-0}" == "1" ]]; then
  command -v rg >/dev/null 2>&1 || {
    printf 'ripgrep is required for the scan sample gate self-test\n' >&2
    exit 1
  }
  run_scan_sample_gate_self_test
  exit $?
fi

hash_bundle_tree() {
  local bundle_dir="$1"
  (
    cd "$bundle_dir"
    {
      find . -type f -exec sh -c 'for path do
        mode="$(stat -f "%Sp" "$path")"
        digest="$(shasum -a 256 "$path")"
        printf "file %s %s %s\n" "$mode" "${digest%% *}" "$path"
      done' sh {} +
      find . -type l -exec sh -c 'for path do printf "symlink %s -> %s\n" "$path" "$(readlink "$path")"; done' sh {} +
    } | LC_ALL=C sort | shasum -a 256 | awk '{print $1}'
  )
}

hash_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

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

source_commit="$(git -C "$repo_root" rev-parse HEAD)"
source_branch="$(git -C "$repo_root" branch --show-current)"
source_dirty=false
if [[ -n "$(git -C "$repo_root" status --porcelain --untracked-files=all)" ]]; then
  source_dirty=true
fi
tracked_diff_sha256="$(git -C "$repo_root" diff --binary HEAD | shasum -a 256 | awk '{print $1}')"
untracked_source_sha256="$(git -C "$repo_root" ls-files --others --exclude-standard -z \
  | xargs -0 shasum -a 256 | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
android_model="$(adb -s "$android_serial" shell getprop ro.product.model | tr -d '\r')"
android_release="$(adb -s "$android_serial" shell getprop ro.build.version.release | tr -d '\r')"
android_api="$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
android_abi="$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
ios_device_line="$(printf '%s\n' "$booted_ios" | rg -F "($ios_udid) (Booted)" | head -n 1 || true)"
host_xcode_version="$(xcodebuild -version | tr '\n' ' ' | sed 's/[[:space:]]*$//')"
host_adb_version="$(adb version | head -n 1)"
host_mise_version="$(mise --version | head -n 1)"
host_bun_version="$(mise exec -- bun --version)"
android_ndk_version="$(cat "$repo_root/tools/android-ndk-version.txt")"
[[ -n "$android_model" && -n "$android_release" && "$android_api" =~ ^[0-9]+$ \
    && -n "$android_abi" ]] || fail "Android AVD의 모델·OS·API·ABI 정보를 모두 읽지 못했습니다"
[[ -n "$ios_device_line" ]] || fail "iOS Simulator 식별 정보를 읽지 못했습니다"
[[ -n "$host_xcode_version" && -n "$host_adb_version" \
    && -n "$host_mise_version" && -n "$host_bun_version" ]] \
  || fail "host build tool 버전을 모두 읽지 못했습니다"
[[ ! -e "$result_dir" ]] || fail "기존 검증 결과를 덮어쓰지 않도록 새 결과 폴더를 지정해야 합니다"
mkdir -p "$result_dir"
printf 'V8 revision=%s\nAndroid serial=%s\niOS simulator=%s\n' \
  "$(cat "$repo_root/tools/v8/v8-revision.txt")" "$android_serial" "$ios_udid" \
  > "$result_dir/environment.txt"
{
  printf 'run_id=%s\n' "$run_id"
  printf 'captured_at_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'repository_root=%s\nresult_dir=%s\n' "$repo_root" "$result_dir"
  printf 'source_commit=%s\nsource_branch=%s\nsource_dirty=%s\n' \
    "$source_commit" "$source_branch" "$source_dirty"
  printf 'tracked_diff_sha256=%s\nuntracked_source_sha256=%s\n' \
    "$tracked_diff_sha256" "$untracked_source_sha256"
  printf 'v8_revision=%s\nfixture_flag=SPINON_ENABLE_S03_DOM_GC_FIXTURE=1\n' \
    "$(cat "$repo_root/tools/v8/v8-revision.txt")"
  printf 'android_build_task=build:android\nandroid_build_configuration=assembleDebug\nandroid_ndk_version=%s\n' \
    "$android_ndk_version"
  printf 'android_serial=%s\nandroid_model=%s\nandroid_release=%s\nandroid_api=%s\nandroid_abi=%s\n' \
    "$android_serial" "$android_model" "$android_release" "$android_api" "$android_abi"
  printf 'ios_build_task=build:ios-sim\nios_build_configuration=Debug-iphonesimulator\n'
  printf 'ios_udid=%s\nios_device=%s\n' "$ios_udid" "$ios_device_line"
  printf 'host_macos=%s\nhost_xcode=%s\nhost_adb=%s\nhost_mise=%s\nhost_bun=%s\n' \
    "$(sw_vers -productVersion)" "$host_xcode_version" "$host_adb_version" \
    "$host_mise_version" "$host_bun_version"
} > "$result_dir/run-manifest.txt"

printf 'Android 에뮬레이터 앱을 빌드합니다.\n'
if ! SPINON_ENABLE_S03_DOM_GC_FIXTURE=1 mise exec -- bun run build:android \
  > "$result_dir/android-build.log" 2>&1; then
  tail -n 80 "$result_dir/android-build.log" >&2
  fail "Android 앱 빌드가 실패했습니다"
fi
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$android_apk" ]] || fail "Android APK가 없습니다: $android_apk"
{
  printf 'android_apk_path=platforms/android/app/build/outputs/apk/debug/app-debug.apk\n'
  printf 'android_apk_sha256=%s\n' "$(hash_file "$android_apk")"
  printf 'android_build_log_path=android-build.log\n'
  printf 'android_build_log_sha256=%s\n' "$(hash_file "$result_dir/android-build.log")"
} >> "$result_dir/run-manifest.txt"
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
rg -Fq 'large_registry_scan_samples=10' "$result_dir/android.log" \
  || fail "Android 대량 registry 재scan 10회 결과가 없습니다"
rg -Fq 'large_registry_scan_samples_us=' "$result_dir/android.log" \
  || fail "Android 대량 registry 재scan 표본이 없습니다"
rg -Fq 'large_registry_scan_observability=PASS' "$result_dir/android.log" \
  || fail "Android scan TID·clock 표본 metadata가 완전하지 않습니다"
scan_sample_sequence_is_complete "$result_dir/android.log" \
  || fail "Android scan metadata가 순서대로 10개 모두 기록되지 않았습니다"
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
{
  printf 'ios_app_bundle_path=build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app\n'
  printf 'ios_app_bundle_tree_sha256=%s\n' "$(hash_bundle_tree "$ios_app")"
  printf 'ios_build_log_path=ios-build.log\n'
  printf 'ios_build_log_sha256=%s\n' "$(hash_file "$result_dir/ios-build.log")"
} >> "$result_dir/run-manifest.txt"
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
rg -Fq '16,385개 wrapper 재scan · 통과 · 표본 10회' "$result_dir/ios-stream.log" \
  || fail "iOS 대량 registry 재scan 10회 결과가 없습니다"
rg -Fq 'S03_SCAN_OBSERVABILITY status=PASS' "$result_dir/ios-stream.log" \
  || fail "iOS scan TID·clock 표본 metadata가 완전하지 않습니다"
scan_sample_sequence_is_complete "$result_dir/ios-stream.log" \
  || fail "iOS scan metadata가 순서대로 10개 모두 기록되지 않았습니다"
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
