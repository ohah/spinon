#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
repeats=10
duration_seconds=12
simulator_udid="${SPINON_IOS_UDID:-}"
random_seed="$(date +%s)"
output_root="${repo_root}/build/spinon/benchmark/ios-trace-overhead"
max_pair_attempts=3
app_path="${repo_root}/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
bundle_id="dev.spinon.bootstrap"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/run-ios-trace-overhead-matrix.sh [옵션]

옵션:
  --udid <시뮬레이터 UDID>   부팅된 iPhone 17 Pro / iOS 26.2 지정
  --repeats <횟수>           유효한 paired run 수 (기본: 10)
  --duration <초>            실행 시간 (기본: 12초)
  --seed <정수>              AB/BA 순서 재현용 seed
  --out <디렉터리>           결과 상위 디렉터리
EOF
}

fail() {
    printf '오류: %s\n' "$1" >&2
    exit 1
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --udid)
            [[ $# -ge 2 ]] || fail "--udid 값이 없습니다"
            simulator_udid="$2"
            shift 2
            ;;
        --repeats)
            [[ $# -ge 2 ]] || fail "--repeats 값이 없습니다"
            repeats="$2"
            shift 2
            ;;
        --duration)
            [[ $# -ge 2 ]] || fail "--duration 값이 없습니다"
            duration_seconds="$2"
            shift 2
            ;;
        --seed)
            [[ $# -ge 2 ]] || fail "--seed 값이 없습니다"
            random_seed="$2"
            shift 2
            ;;
        --out)
            [[ $# -ge 2 ]] || fail "--out 값이 없습니다"
            output_root="$2"
            shift 2
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

[[ "$repeats" =~ ^[0-9]+$ ]] || fail "--repeats는 양의 정수여야 합니다"
[[ "$duration_seconds" =~ ^[0-9]+$ ]] || fail "--duration은 양의 정수여야 합니다"
[[ "$random_seed" =~ ^[0-9]+$ ]] || fail "--seed는 0 이상의 정수여야 합니다"
(( repeats >= 1 && repeats <= 100 )) || fail "--repeats는 1~100 범위여야 합니다"
(( duration_seconds >= 10 && duration_seconds <= 120 )) || fail "--duration은 10~120초 범위여야 합니다"
[[ -d "$app_path" ]] || fail "먼저 mise exec -- bun run build:ios-sim 으로 앱을 빌드하세요"

booted_list="$(xcrun simctl list devices booted)"
if [[ -z "$simulator_udid" ]]; then
    booted_count="$(printf '%s\n' "$booted_list" | grep -cE '\([A-F0-9-]{36}\) \(Booted\)' || true)"
    [[ "$booted_count" == "1" ]] || fail "부팅된 시뮬레이터가 ${booted_count}개입니다. --udid로 대상을 고정하세요"
    simulator_udid="$(printf '%s\n' "$booted_list" | sed -nE 's/.*\(([A-F0-9-]{36})\) \(Booted\).*/\1/p' | head -n 1)"
fi
[[ "$booted_list" == *"iPhone 17 Pro (${simulator_udid}) (Booted)"* ]] \
    || fail "계획된 iPhone 17 Pro Simulator가 부팅 상태가 아닙니다"
[[ "$booted_list" == *"-- iOS 26.2 --"* ]] || fail "계획된 iOS 26.2 Simulator가 아닙니다"

if [[ "$output_root" != /* ]]; then output_root="${repo_root}/${output_root}"; fi
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
[[ ! -e "$matrix_dir" ]] || fail "같은 초의 행렬 디렉터리가 이미 있습니다. --out으로 새 경로를 지정하세요"
mkdir -p "${matrix_dir}/runs"
schedule_file="${matrix_dir}/schedule.txt"
pairs_file="${matrix_dir}/pairs.tsv"
app_digest="$(find "$app_path" -type f -exec shasum -a 256 {} \; | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
source_tree_sha256="$(git -C "$repo_root" ls-files --cached --others --exclude-standard -z \
    | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"
xcrun simctl install "$simulator_udid" "$app_path" > "${matrix_dir}/app-install.txt"
RANDOM="$random_seed"
{
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'git_commit=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
    printf 'random_seed=%s\n' "$random_seed"
    printf 'repeats_per_condition=%s\n' "$repeats"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'simulator_udid=%s\n' "$simulator_udid"
    printf 'app_bundle_sha256=%s\n' "$app_digest"
    printf 'source_tree_sha256=%s\n' "$source_tree_sha256"
    printf '\nround\tattempt\torder\tstatus\trun_on\trun_off\n'
} > "$schedule_file"
printf 'round\tattempt\torder\ttrace_on_dir\ttrace_off_dir\n' > "$pairs_file"

capture_condition() {
    local condition="$1"
    local run_output_root="$2"
    local log_file="$3"
    if ! bash "${repo_root}/tools/benchmark/capture-ios-trace-overhead.sh" \
        "$condition" \
        --udid "$simulator_udid" \
        --duration "$duration_seconds" \
        --skip-install \
        --out "$run_output_root" > "$log_file" 2>&1; then
        cat "$log_file" >&2
        return 1
    fi
    cat "$log_file"
}

find_capture_dir() {
    local log_file="$1"
    local path
    path="$(sed -n 's/^R05 iOS 계측 오버헤드 실행을 저장했습니다: //p' "$log_file" | tail -n 1)"
    [[ -n "$path" && -d "$path" ]] || return 1
    printf '%s\n' "$path"
}

metadata_value() {
    local run_dir="$1"
    local key="$2"
    sed -n "s/^${key}=//p" "${run_dir}/metadata.txt" | head -n 1
}

for ((round = 1; round <= repeats; round++)); do
    if (((round + random_seed) % 2 == 0)); then
        first_condition=on
        second_condition=off
    else
        first_condition=off
        second_condition=on
    fi

    pair_succeeded=false
    for ((attempt = 1; attempt <= max_pair_attempts; attempt++)); do
        pair_root="${matrix_dir}/runs/round-$(printf '%02d' "$round")/attempt-${attempt}"
        on_dir=""
        off_dir=""
        failed_condition=""
        mkdir -p "$pair_root"
        printf '\n회차 %s · 시도 %s · 순서 %s/%s\n' \
            "$round" "$attempt" "$first_condition" "$second_condition"

        for condition in "$first_condition" "$second_condition"; do
            condition_root="${pair_root}/${condition}"
            mkdir -p "$condition_root"
            capture_log="${condition_root}/capture.log"
            if ! capture_condition "$condition" "$condition_root" "$capture_log"; then
                failed_condition="$condition"
                break
            fi
            run_dir="$(find_capture_dir "$capture_log")" \
                || { failed_condition="$condition-path"; break; }
            if [[ "$condition" == "on" ]]; then on_dir="$run_dir"; else off_dir="$run_dir"; fi
        done

        if [[ -z "$failed_condition" ]]; then
            on_digest="$(metadata_value "$on_dir" app_bundle_sha256)"
            off_digest="$(metadata_value "$off_dir" app_bundle_sha256)"
            on_source="$(metadata_value "$on_dir" source_tree_sha256)"
            off_source="$(metadata_value "$off_dir" source_tree_sha256)"
            on_udid="$(metadata_value "$on_dir" simulator_udid)"
            off_udid="$(metadata_value "$off_dir" simulator_udid)"
            if [[ "$on_digest" != "$off_digest" || "$on_digest" != "$app_digest" \
                || "$on_source" != "$off_source" || "$on_udid" != "$simulator_udid" \
                || "$on_source" != "$source_tree_sha256" \
                || "$off_udid" != "$simulator_udid" ]]; then
                failed_condition="paired-app-or-environment-mismatch"
            fi
        fi

        if [[ -z "$failed_condition" ]]; then
            order="${first_condition}/${second_condition}"
            printf '%s\t%s\t%s\t%s\t%s\n' "$round" "$attempt" "$order" "$on_dir" "$off_dir" >> "$pairs_file"
            printf '%s\t%s\t%s\tPASS\t%s\t%s\n' "$round" "$attempt" "$order" "$on_dir" "$off_dir" \
                >> "$schedule_file"
            pair_succeeded=true
            break
        fi

        order="${first_condition}/${second_condition}"
        printf '%s\t%s\t%s\tFAIL:%s\t%s\t%s\n' "$round" "$attempt" "$order" "$failed_condition" \
            "${on_dir:-없음}" "${off_dir:-없음}" >> "$schedule_file"
        printf '회차 %s 시도 %s 실패 (%s). 원본은 보존하고 paired run 전체를 다시 실행합니다.\n' \
            "$round" "$attempt" "$failed_condition" >&2
    done

    [[ "$pair_succeeded" == "true" ]] || fail "회차 ${round}의 paired run이 ${max_pair_attempts}회 안에 유효하게 완료되지 않았습니다. ${matrix_dir}를 확인하세요"
done

printf '\n유효한 iOS paired run %s쌍을 저장했습니다: %s\n' "$repeats" "$matrix_dir"
if [[ "$repeats" != "10" ]]; then
    printf '10쌍 미만·초과 실행은 사전 비교 모델의 전체 반복을 채우지 않아 탐색 결과로만 취급합니다.\n'
fi
printf '요약: mise exec -- bun run tools/benchmark/summarize-ios-trace-overhead.mjs %s\n' "$matrix_dir"
