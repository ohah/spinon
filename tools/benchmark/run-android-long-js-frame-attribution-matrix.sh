#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
app_id="dev.spinon.bootstrap"
repeats=10
duration_seconds=20
input_count=3
adb_serial="${SPINON_ANDROID_SERIAL:-}"
output_root="${repo_root}/build/spinon/benchmark/android-long-js-frame-attribution"
random_seed="${RANDOM}"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/run-android-long-js-frame-attribution-matrix.sh [옵션]

옵션:
  --serial <adb serial>     대상 지정 (기본: 연결 기기가 하나면 자동 선택)
  --repeats <횟수>          조건별 독립 실행 횟수 (기본: 10)
  --duration <초>           회차별 Perfetto 길이 (기본: 20초)
  --taps <횟수>             각 조건의 JS 이벤트 수 (기본: 3, 최대: 5)
  --seed <정수>             조건 순서 재현용 난수 seed
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
[[ "$input_count" =~ ^[0-9]+$ ]] || fail "--taps는 양의 정수여야 합니다"
[[ "$random_seed" =~ ^[0-9]+$ ]] || fail "--seed는 0 이상의 정수여야 합니다"
(( repeats >= 1 && repeats <= 50 )) || fail "--repeats는 1~50 범위여야 합니다"
(( duration_seconds >= 10 )) || fail "--duration은 최소 10초여야 합니다"
(( input_count >= 1 && input_count <= 5 )) || fail "--taps는 1~5 범위여야 합니다"
(( input_count <= 2 * (duration_seconds - 3) )) \
    || fail "탭 횟수가 trace 길이에 비해 많습니다"

command -v adb >/dev/null 2>&1 || fail "adb를 찾을 수 없습니다"
command -v shasum >/dev/null 2>&1 || fail "shasum을 찾을 수 없습니다"

if [[ -z "$adb_serial" ]]; then
    connected_devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
    connected_count="$(printf '%s\n' "$connected_devices" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$connected_count" == "1" ]] \
        || fail "연결 기기가 ${connected_count}개입니다. --serial로 대상을 고정하세요"
    adb_serial="$connected_devices"
fi
adb -s "$adb_serial" get-state | grep -qx device \
    || fail "대상 기기에 adb로 연결할 수 없습니다: $adb_serial"
perfetto_sources="$(adb -s "$adb_serial" shell perfetto --query 2>&1)" \
    || fail "기기 Perfetto 데이터 소스 조회에 실패했습니다"
if ! printf '%s\n' "$perfetto_sources" | grep -Fq 'linux.ftrace'; then
    fail "기기 Perfetto에 linux.ftrace 데이터 소스가 없습니다. 이 frame-attribution 행렬을 실행할 수 없습니다. Android off-CPU scheduler 진단에는 tools/benchmark/capture-android-simpleperf-offcpu.sh를 사용하세요"
fi
is_emulator="$(adb -s "$adb_serial" shell getprop ro.kernel.qemu | tr -d '\r')"
if [[ "$is_emulator" == "1" ]]; then
    device_kind="에뮬레이터"
else
    device_kind="실기기"
fi

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
mkdir -p "${matrix_dir}/runs"
schedule_file="${matrix_dir}/schedule.txt"
debug_apk="${repo_root}/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$debug_apk" ]] || fail "먼저 mise exec -- bun run build:android 로 APK를 빌드하세요"

{
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'random_seed=%s\n' "$random_seed"
    printf 'repeats_per_condition=%s\n' "$repeats"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'events_per_condition=%s\n' "$input_count"
    printf 'adb_serial=%s\n' "$adb_serial"
    printf 'device_kind=%s\n' "$device_kind"
    printf 'APK SHA-256: %s\n' "$(shasum -a 256 "$debug_apk" | awk '{print $1}')"
    printf '\nround scenario\n'
} > "$schedule_file"

adb -s "$adb_serial" install -r "$debug_apk" > "${matrix_dir}/apk-install.txt"
RANDOM="$random_seed"
if (( RANDOM % 2 == 0 )); then
    first_condition=spinon-event
    second_condition=spinon-long-js
else
    first_condition=spinon-long-js
    second_condition=spinon-event
fi
printf 'condition_order=balanced-alternating\nfirst_condition=%s\n' "$first_condition" \
    >> "$schedule_file"

for ((round = 1; round <= repeats; round++)); do
    if (( round % 2 == 1 )); then
        order=("$first_condition" "$second_condition")
    else
        order=("$second_condition" "$first_condition")
    fi

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

printf '동일 APK·%s에서 조건별 %s회 추적을 저장했습니다: %s\n' \
    "$device_kind" \
    "$repeats" "$matrix_dir"
if [[ "$is_emulator" == "1" ]]; then
    printf '긴 JS 중 UI·이벤트 대기·취소 경로를 비교하는 에뮬레이터 원인 분석입니다.\n'
else
    printf '실기기 결과는 해당 기기·OS·빌드·입력 조건에 한정하며 다른 기기의 성능으로 일반화하지 않습니다.\n'
fi
