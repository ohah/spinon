#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
repeats=10
duration_seconds=20
input_count=10
adb_serial="${SPINON_ANDROID_SERIAL:-}"
random_seed="$(date +%s)"
output_root="${repo_root}/build/spinon/benchmark/android-trace-overhead"
max_pair_attempts=3
trace_processor="${SPINON_TRACE_PROCESSOR:-trace_processor}"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/run-android-trace-overhead-matrix.sh [옵션]

옵션:
  --serial <adb serial>     Android API 36 ARM64 에뮬레이터 지정
  --repeats <횟수>          유효한 paired run 수 (기본: 10)
  --duration <초>           조건 길이 (기본: 20초)
  --taps <횟수>             회차별 탭 수 (기본: 10)
  --seed <정수>             AB/BA 순서 재현용 seed
  --out <디렉터리>          결과 상위 디렉터리

설정:
  SPINON_TRACE_PROCESSOR    Perfetto 원본 품질을 검사할 공식 trace_processor 경로
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
(( repeats >= 1 && repeats <= 100 )) || fail "--repeats는 1~100 범위여야 합니다"
(( duration_seconds >= 10 )) || fail "--duration은 최소 10초여야 합니다"
(( input_count >= 1 && input_count <= 2 * (duration_seconds - 3) )) \
    || fail "탭 횟수가 조건 길이에 비해 많습니다"

command -v adb >/dev/null 2>&1 || fail "adb를 찾을 수 없습니다"
command -v shasum >/dev/null 2>&1 || fail "shasum을 찾을 수 없습니다"
if [[ "$trace_processor" == */* ]]; then
    [[ -x "$trace_processor" ]] || fail "Trace Processor 실행 파일을 찾지 못했습니다: $trace_processor"
else
    command -v "$trace_processor" >/dev/null 2>&1 || fail "Perfetto trace 품질 검증에 Trace Processor가 필요합니다. SPINON_TRACE_PROCESSOR를 설정하세요"
fi
trace_processor_version="$("$trace_processor" --version | head -n 1)" \
    || fail "Trace Processor 버전을 확인할 수 없습니다"

if [[ -z "$adb_serial" ]]; then
    connected_devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
    connected_count="$(printf '%s\n' "$connected_devices" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$connected_count" == "1" ]] || fail "연결 기기가 ${connected_count}개입니다. --serial로 대상을 고정하세요"
    adb_serial="$connected_devices"
fi

adb -s "$adb_serial" get-state | grep -qx device || fail "대상 에뮬레이터에 연결할 수 없습니다: $adb_serial"
api_level="$(adb -s "$adb_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
device_model="$(adb -s "$adb_serial" shell getprop ro.product.model | tr -d '\r')"
is_emulator="$(adb -s "$adb_serial" shell getprop ro.kernel.qemu | tr -d '\r')"
[[ "$api_level" == "36" ]] || fail "계획된 Android API 36 에뮬레이터가 아닙니다 (현재 API $api_level)"
[[ "$device_model" == "sdk_gphone64_arm64" && "$is_emulator" == "1" ]] \
    || fail "계획된 sdk_gphone64_arm64 에뮬레이터가 아닙니다 ($device_model, qemu=$is_emulator)"

if [[ "$output_root" != /* ]]; then output_root="${repo_root}/${output_root}"; fi
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
[[ ! -e "$matrix_dir" ]] || fail "같은 초의 행렬 디렉터리가 이미 있습니다. --out으로 새 경로를 지정하세요"
mkdir -p "${matrix_dir}/runs"
schedule_file="${matrix_dir}/schedule.txt"
pairs_file="${matrix_dir}/pairs.tsv"
debug_apk="${repo_root}/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$debug_apk" ]] || fail "먼저 mise exec -- bun run build:android 로 APK를 빌드하세요"
apk_sha256="$(shasum -a 256 "$debug_apk" | awk '{print $1}')"
source_tree_sha256="$(git -C "$repo_root" ls-files --cached --others --exclude-standard -z \
    | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"

adb -s "$adb_serial" install -r "$debug_apk" > "${matrix_dir}/apk-install.txt"
RANDOM="$random_seed"
{
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'git_commit=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
    printf 'random_seed=%s\n' "$random_seed"
    printf 'repeats_per_condition=%s\n' "$repeats"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'input_count=%s\n' "$input_count"
    printf 'adb_serial=%s\n' "$adb_serial"
    printf 'android_api=%s\n' "$api_level"
    printf 'device_model=%s\n' "$device_model"
    printf 'apk_sha256=%s\n' "$apk_sha256"
    printf 'source_tree_sha256=%s\n' "$source_tree_sha256"
    printf 'trace_processor_version=%s\n' "$trace_processor_version"
    printf '\nround\tattempt\torder\tstatus\trun_on\trun_off\n'
} > "$schedule_file"
printf 'round\tattempt\torder\tperfetto_on_dir\tperfetto_off_dir\n' > "$pairs_file"

capture_condition() {
    local condition="$1"
    local run_output_root="$2"
    local log_file="$3"
    if ! bash "${repo_root}/tools/benchmark/capture-android-frame-attribution.sh" \
        spinon-event \
        --serial "$adb_serial" \
        --duration "$duration_seconds" \
        --taps "$input_count" \
        --skip-install \
        --perfetto "$condition" \
        --out "$run_output_root" > "$log_file" 2>&1; then
        cat "$log_file" >&2
        return 1
    fi
    cat "$log_file"
}

find_capture_dir() {
    local log_file="$1"
    local path
    path="$(sed -n 's/^R05 Android 프레임 추적을 저장했습니다: //p' "$log_file" | tail -n 1)"
    [[ -n "$path" && -d "$path" ]] || return 1
    printf '%s\n' "$path"
}

metadata_value() {
    local run_dir="$1"
    local key="$2"
    sed -n "s/^${key}=//p" "${run_dir}/metadata.txt" | head -n 1
}

dispatch_sequences() {
    local run_dir="$1"
    grep 'SPINON_RUNTIME_DISPATCH=' "${run_dir}/logcat.txt" \
        | sed -nE 's/.* seq=([0-9]+) .*/\1/p' | sort -n | paste -sd, -
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
            validation_log="${on_dir}/trace-validation.log"
            if ! bun "${repo_root}/tools/benchmark/validate-android-trace-overhead-run.mjs" \
                "$on_dir" "$trace_processor" > "$validation_log" 2>&1; then
                cat "$validation_log" >&2
                failed_condition="perfetto-trace-quality"
            else
                cat "$validation_log"
            fi
        fi

        if [[ -z "$failed_condition" ]]; then
            on_x="$(metadata_value "$on_dir" tap_x)"
            on_y="$(metadata_value "$on_dir" tap_y)"
            off_x="$(metadata_value "$off_dir" tap_x)"
            off_y="$(metadata_value "$off_dir" tap_y)"
            on_digest="$(metadata_value "$on_dir" source_tree_sha256)"
            off_digest="$(metadata_value "$off_dir" source_tree_sha256)"
            on_apk="$(awk 'NR == 1 { print $1 }' "${on_dir}/installed-apk.sha256")"
            off_apk="$(awk 'NR == 1 { print $1 }' "${off_dir}/installed-apk.sha256")"
            on_sequences="$(dispatch_sequences "$on_dir")"
            off_sequences="$(dispatch_sequences "$off_dir")"
            if [[ "$on_x" != "$off_x" || "$on_y" != "$off_y" \
                || "$on_digest" != "$off_digest" || "$on_apk" != "$off_apk" \
                || "$on_digest" != "$source_tree_sha256" || "$on_apk" != "$apk_sha256" \
                || "$on_sequences" != "$off_sequences" ]]; then
                failed_condition="paired-input-build-or-dispatch-sequence-mismatch"
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

printf '\n유효한 Android paired run %s쌍을 저장했습니다: %s\n' "$repeats" "$matrix_dir"
if [[ "$repeats" != "10" ]]; then
    printf '10쌍 미만·초과 실행은 사전 비교 모델의 전체 반복을 채우지 않아 탐색 결과로만 취급합니다.\n'
fi
printf '요약: mise exec -- bun run tools/benchmark/summarize-android-trace-overhead.mjs %s\n' "$matrix_dir"
