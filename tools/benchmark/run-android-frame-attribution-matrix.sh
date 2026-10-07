#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
repeats=10
duration_seconds=20
input_count=10
adb_serial="${SPINON_ANDROID_SERIAL:-}"
output_root="${repo_root}/build/spinon/benchmark/android-frame-attribution-matrix"
random_seed="${RANDOM}"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/run-android-frame-attribution-matrix.sh [옵션]

옵션:
  --serial <adb serial>     대상 지정 (기본: 연결 기기가 하나면 자동 선택)
  --repeats <횟수>          조건별 독립 실행 횟수 (기본: 10)
  --duration <초>           회차별 Perfetto 길이 (기본: 20초)
  --taps <횟수>             회차별 입력 수 (기본: 10)
  --seed <정수>             실행 순서 재현용 난수 seed
  --out <디렉터리>          결과 상위 디렉터리
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
[[ "$input_count" =~ ^[0-9]+$ ]] || fail "--taps는 0 이상의 정수여야 합니다"
[[ "$random_seed" =~ ^[0-9]+$ ]] || fail "--seed는 0 이상의 정수여야 합니다"
(( repeats >= 1 && repeats <= 100 )) || fail "--repeats는 1~100 범위여야 합니다"
(( duration_seconds >= 10 )) || fail "--duration은 최소 10초여야 합니다"
(( input_count >= 1 && input_count <= 2 * (duration_seconds - 3) )) \
    || fail "탭 횟수가 trace 길이에 비해 많습니다"

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
mkdir -p "${matrix_dir}/runs"
schedule_file="${matrix_dir}/schedule.txt"
conditions=(input-only status log-scroll spinon-event)
RANDOM="$random_seed"

{
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'random_seed=%s\n' "$random_seed"
    printf 'repeats_per_condition=%s\n' "$repeats"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'input_count=%s\n' "$input_count"
    printf 'adb_serial=%s\n' "${adb_serial:-auto-single-device}"
    printf '\nround scenario\n'
} > "$schedule_file"

if [[ -z "$adb_serial" ]]; then
    connected_devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
    connected_count="$(printf '%s\n' "$connected_devices" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$connected_count" == "1" ]] || fail "연결 기기가 ${connected_count}개입니다. --serial로 대상을 고정하세요"
    adb_serial="$connected_devices"
fi
printf 'selected_adb_serial=%s\n' "$adb_serial" >> "$schedule_file"

debug_apk="${repo_root}/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$debug_apk" ]] || fail "먼저 mise exec -- bun run build:android 로 APK를 빌드하세요"
adb -s "$adb_serial" install -r "$debug_apk" > "${matrix_dir}/apk-install.txt"
printf 'APK SHA-256: %s\n' "$(shasum -a 256 "$debug_apk" | awk '{print $1}')" \
    >> "$schedule_file"

for ((round = 1; round <= repeats; round++)); do
    order=("${conditions[@]}")
    for ((index = ${#order[@]} - 1; index > 0; index--)); do
        swap_index=$((RANDOM % (index + 1)))
        swap_value="${order[$index]}"
        order[$index]="${order[$swap_index]}"
        order[$swap_index]="$swap_value"
    done

    for scenario in "${order[@]}"; do
        printf '%s %s\n' "$round" "$scenario" | tee -a "$schedule_file"
        bash "${repo_root}/tools/benchmark/capture-android-frame-attribution.sh" \
            "$scenario" \
            --serial "$adb_serial" \
            --duration "$duration_seconds" \
            --taps "$input_count" \
            --skip-install \
            --out "${matrix_dir}/runs"
    done
done

printf '조건별 %s회 추적을 저장했습니다: %s\n' "$repeats" "$matrix_dir"
printf '이 결과는 지정 기기의 원인 분석용이며 실기기 성능 주장은 별도 검증이 필요합니다.\n'
