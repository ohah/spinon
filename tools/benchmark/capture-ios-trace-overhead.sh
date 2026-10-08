#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
mode=""
simulator_udid="${SPINON_IOS_UDID:-}"
duration_seconds=12
output_root="${repo_root}/build/spinon/benchmark/ios-trace-overhead"
skip_install=false
bundle_id="dev.spinon.bootstrap"
app_path="${repo_root}/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/capture-ios-trace-overhead.sh <on|off> [옵션]

옵션:
  --udid <시뮬레이터 UDID>   부팅된 iOS Simulator 지정
  --duration <초>            실행 시간 (기본: 12초)
  --out <디렉터리>           결과 상위 디렉터리
  --skip-install             이미 설치된 앱의 실행 파일 digest가 같을 때 재설치 생략
EOF
}

fail() {
    printf '오류: %s\n' "$1" >&2
    exit 1
}

[[ $# -ge 1 ]] || { usage; exit 2; }
mode="$1"
shift
[[ "$mode" == "on" || "$mode" == "off" ]] || fail "조건은 on 또는 off여야 합니다"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --udid)
            [[ $# -ge 2 ]] || fail "--udid 값이 없습니다"
            simulator_udid="$2"
            shift 2
            ;;
        --duration)
            [[ $# -ge 2 ]] || fail "--duration 값이 없습니다"
            duration_seconds="$2"
            shift 2
            ;;
        --out)
            [[ $# -ge 2 ]] || fail "--out 값이 없습니다"
            output_root="$2"
            shift 2
            ;;
        --skip-install)
            skip_install=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            fail "알 수 없는 옵션: $1"
            ;;
    esac
done

[[ "$duration_seconds" =~ ^[0-9]+$ ]] || fail "--duration은 양의 정수여야 합니다"
(( duration_seconds >= 10 && duration_seconds <= 120 )) || fail "--duration은 10~120초 범위여야 합니다"
[[ -d "$app_path" ]] || fail "iOS Simulator 앱이 없습니다. 먼저 mise exec -- bun run build:ios-sim 을 실행하세요"
command -v xcrun >/dev/null 2>&1 || fail "xcrun을 찾을 수 없습니다"
command -v shasum >/dev/null 2>&1 || fail "shasum을 찾을 수 없습니다"

booted_list="$(xcrun simctl list devices booted)"
if [[ -z "$simulator_udid" ]]; then
    booted_count="$(printf '%s\n' "$booted_list" | grep -cE '\([A-F0-9-]{36}\) \(Booted\)' || true)"
    [[ "$booted_count" == "1" ]] || fail "부팅된 시뮬레이터가 ${booted_count}개입니다. --udid로 대상을 고정하세요"
    simulator_udid="$(printf '%s\n' "$booted_list" | sed -nE 's/.*\(([A-F0-9-]{36})\) \(Booted\).*/\1/p' | head -n 1)"
fi
[[ "$booted_list" == *"${simulator_udid}"* ]] || fail "대상 시뮬레이터가 부팅 상태가 아닙니다: $simulator_udid"
if [[ "$booted_list" != *"iPhone 17 Pro (${simulator_udid}) (Booted)"* ]] \
    || [[ "$booted_list" != *"-- iOS 26.2 --"* ]]; then
    fail "계획된 iPhone 17 Pro / iOS 26.2 Simulator가 아닙니다"
fi

app_digest="$(find "$app_path" -type f -exec shasum -a 256 {} \; | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
app_binary_digest="$(shasum -a 256 "${app_path}/SpinonBootstrap" | awk '{print $1}')"
installed_app="$(xcrun simctl get_app_container "$simulator_udid" "$bundle_id" app 2>/dev/null || true)"
if [[ "$skip_install" == "false" ]]; then
    xcrun simctl install "$simulator_udid" "$app_path"
elif [[ -z "$installed_app" || ! -f "${installed_app}/SpinonBootstrap" ]]; then
    fail "--skip-install 대상 앱이 설치되어 있지 않습니다"
fi
installed_app="$(xcrun simctl get_app_container "$simulator_udid" "$bundle_id" app 2>/dev/null || true)"
[[ -n "$installed_app" && -f "${installed_app}/SpinonBootstrap" ]] || fail "설치된 Simulator 앱을 찾지 못했습니다"
installed_binary_digest="$(shasum -a 256 "${installed_app}/SpinonBootstrap" | awk '{print $1}')"
[[ "$installed_binary_digest" == "$app_binary_digest" ]] || fail "Simulator 설치 앱이 현재 빌드와 다릅니다"

if [[ "$output_root" != /* ]]; then output_root="${repo_root}/${output_root}"; fi
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="${output_root}/${stamp}-${mode}"
mkdir -p "$run_dir"
trace_path="${run_dir}/time-profiler.trace"
if [[ "$mode" == "on" ]]; then
    xcrun simctl terminate "$simulator_udid" "$bundle_id" >/dev/null 2>&1 || true
    xcrun xctrace record \
        --template 'Time Profiler' \
        --device "$simulator_udid" \
        --time-limit "${duration_seconds}s" \
        --output "$trace_path" \
        --launch -- "$app_path" --spinon-r05-attribution \
        > "${run_dir}/xctrace.log" 2>&1
    xcrun xctrace export --input "$trace_path" --toc > "${run_dir}/trace-toc.xml"
    app_pid="$(sed -nE 's/.*type="launched" return-exit-status="0" name="SpinonBootstrap" pid="([0-9]+)".*/\1/p' \
        "${run_dir}/trace-toc.xml" | head -n 1)"
    [[ "$app_pid" =~ ^[0-9]+$ ]] || fail "Time Profiler 원본에서 앱 PID를 확인할 수 없습니다"
else
    xcrun simctl terminate "$simulator_udid" "$bundle_id" >/dev/null 2>&1 || true
    xcrun simctl launch --terminate-running-process "$simulator_udid" "$bundle_id" \
        --spinon-r05-attribution > "${run_dir}/launch.txt"
    app_pid="$(sed -nE 's/.*: ([0-9]+).*/\1/p' "${run_dir}/launch.txt" | head -n 1)"
    [[ "$app_pid" =~ ^[0-9]+$ ]] || fail "trace-off 앱 PID를 확인할 수 없습니다"
    sleep "$duration_seconds"
    printf 'Time Profiler 수집 없이 동일 앱 probe를 실행했습니다.\n' > "${run_dir}/trace-disabled.txt"
fi

log_predicate="processIdentifier == ${app_pid} AND subsystem == \"dev.spinon.bootstrap\" AND category == \"r05-ios-attribution\""
xcrun simctl spawn "$simulator_udid" log show --last 5m --style compact \
    --predicate "$log_predicate" > "${run_dir}/unified-log.txt" 2> "${run_dir}/log-query-stderr.txt"
xcrun simctl io "$simulator_udid" screenshot "${run_dir}/after.png" >/dev/null 2>&1
xcrun simctl terminate "$simulator_udid" "$bundle_id" >/dev/null 2>&1 || true

{
    printf 'mode=%s\n' "$mode"
    printf 'started_utc=%s\n' "$stamp"
    printf 'git_commit=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
    printf 'source_tree_sha256=%s\n' "$(git -C "$repo_root" ls-files --cached --others --exclude-standard -z | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"
    printf 'simulator_udid=%s\n' "$simulator_udid"
    printf 'simulator_runtime=%s\n' "$(printf '%s\n' "$booted_list" | sed -nE 's/^-- (.*) --$/\1/p' | head -n 1)"
    printf 'app_bundle_id=%s\n' "$bundle_id"
    printf 'app_pid=%s\n' "$app_pid"
    printf 'app_bundle_sha256=%s\n' "$app_digest"
    printf 'app_binary_sha256=%s\n' "$app_binary_digest"
    printf 'duration_seconds=%s\n' "$duration_seconds"
} > "${run_dir}/metadata.txt"

if ! bun "${repo_root}/tools/benchmark/validate-ios-trace-overhead-run.mjs" "$run_dir"; then
    fail "iOS R05 실행 검증에 실패했습니다. 원본은 ${run_dir}에 보존했습니다"
fi
printf 'R05 iOS 계측 오버헤드 실행을 저장했습니다: %s\n' "$run_dir"
