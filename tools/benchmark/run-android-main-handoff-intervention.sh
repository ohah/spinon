#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
adb_serial="${SPINON_ANDROID_SERIAL:-}"
repeats=10
duration_seconds=10
input_count=5
output_root="${repo_root}/build/spinon/benchmark/android-main-handoff-intervention"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/run-android-main-handoff-intervention.sh [옵션]

옵션:
  --serial <adb serial>  대상 지정 (기본: 연결 기기가 하나면 자동 선택)
  --repeats <횟수>       조건별 독립 실행 횟수 (기본: 10)
  --duration <초>        회차별 Perfetto 길이 (기본: 10초)
  --taps <횟수>          회차별 입력 수 (기본: 5)
  --out <디렉터리>       결과 디렉터리
EOF
}

fail() {
    printf '오류: %s\n' "$1" >&2
    exit 1
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --serial)
            [[ $# -ge 2 ]] || fail "--serial 값이 없습니다"
            adb_serial="$2"
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
        --taps)
            [[ $# -ge 2 ]] || fail "--taps 값이 없습니다"
            input_count="$2"
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
        *) fail "알 수 없는 옵션: $1" ;;
    esac
done

[[ "$repeats" =~ ^[0-9]+$ ]] || fail "--repeats는 양의 정수여야 합니다"
[[ "$duration_seconds" =~ ^[0-9]+$ ]] || fail "--duration은 양의 정수여야 합니다"
[[ "$input_count" =~ ^[0-9]+$ ]] || fail "--taps는 양의 정수여야 합니다"
(( repeats >= 1 && repeats <= 50 )) || fail "--repeats는 1~50 범위여야 합니다"
(( duration_seconds >= 10 )) || fail "--duration은 최소 10초여야 합니다"
(( input_count >= 1 && input_count <= 100 )) || fail "--taps는 1~100 범위여야 합니다"
(( input_count <= 2 * (duration_seconds - 3) )) || fail "탭 수가 trace 길이에 비해 많습니다"

if [[ -z "$adb_serial" ]]; then
    devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
    count="$(printf '%s\n' "$devices" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$count" == "1" ]] || fail "연결 기기가 ${count}개입니다. --serial로 대상을 고정하세요"
    adb_serial="$devices"
fi
adb -s "$adb_serial" get-state | grep -qx device \
    || fail "대상 기기에 adb로 연결할 수 없습니다: $adb_serial"

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
mkdir -p "$matrix_dir/runs"
{
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'repeats_per_condition=%s\n' "$repeats"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'taps_per_condition_per_repeat=%s\n' "$input_count"
    printf 'adb_serial=%s\n' "$adb_serial"
    printf 'conditions=normal-main-handler,asynchronous-main-handler\n'
    printf 'condition_order=balanced-alternating\n'
    printf '\nround condition\n'
} > "${matrix_dir}/schedule.txt"

for ((round = 1; round <= repeats; round++)); do
    if (( round % 2 == 1 )); then
        order=(normal-main-handler asynchronous-main-handler)
    else
        order=(asynchronous-main-handler normal-main-handler)
    fi
    for condition in "${order[@]}"; do
        printf '%s %s\n' "$round" "$condition" | tee -a "${matrix_dir}/schedule.txt"
        args=(spinon-event --serial "$adb_serial" --duration "$duration_seconds"
            --taps "$input_count" --skip-install --out "${matrix_dir}/runs")
        if [[ "$condition" == "asynchronous-main-handler" ]]; then
            args+=(--async-main-handoff)
        fi
        bash "${repo_root}/tools/benchmark/capture-android-frame-attribution.sh" "${args[@]}"
        run_dir="$(find "${matrix_dir}/runs" -mindepth 1 -maxdepth 1 -type d \
            -name "*-spinon-event" -print | sort | tail -n 1)"
        [[ -n "$run_dir" ]] || fail "회차 결과 디렉터리를 찾지 못했습니다"
        printf 'condition=%s\n' "$condition" >> "${run_dir}/metadata.txt"
    done
done

printf '두 Handler 조건의 균형 비교 원본을 저장했습니다: %s\n' "$matrix_dir"
printf '이 실험은 sync barrier 대기 가설을 확인하는 진단이며 제품 구현 권고나 성능 순위가 아닙니다.\n'
