#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
app_id="dev.spinon.bootstrap"
adb_serial="${SPINON_ANDROID_SERIAL:-}"
repeats=5
duration_seconds=6
sample_frequency=1000
output_root="${repo_root}/build/spinon/benchmark/android-simpleperf-offcpu"

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/capture-android-simpleperf-offcpu.sh [옵션]

옵션:
  --serial <adb serial>       대상 지정 (기본: 연결 기기가 하나면 자동 선택)
  --repeats <횟수>            독립 실행 횟수 (기본: 5, 최대: 20)
  --duration <초>             실행별 프로파일 길이 (기본: 6초, 최대: 8초)
  --frequency <Hz>            cpu-clock 표본 빈도 (기본: 1000)
  --out <디렉터리>            결과 상위 디렉터리
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
        --frequency)
            [[ $# -ge 2 ]] || fail "--frequency 값이 없습니다"
            sample_frequency="$2"
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
[[ "$sample_frequency" =~ ^[0-9]+$ ]] || fail "--frequency는 양의 정수여야 합니다"
(( repeats >= 1 && repeats <= 20 )) || fail "--repeats는 1~20 범위여야 합니다"
(( duration_seconds >= 1 && duration_seconds <= 8 )) \
    || fail "--duration은 1~8초 범위여야 하며, 긴 JavaScript 자동 취소보다 짧아야 합니다"
(( sample_frequency >= 100 && sample_frequency <= 2000 )) \
    || fail "--frequency는 100~2000Hz 범위여야 합니다"

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
api_level="$(adb -s "$adb_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
(( api_level >= 27 )) || fail "simpleperf off-CPU 기록은 Android 8.1/API 27 이상이 필요합니다 (현재 API $api_level)"

simpleperf_features="$(adb -s "$adb_serial" shell simpleperf list --show-features 2>&1)" \
    || fail "기기에서 simpleperf 기능 목록을 읽지 못했습니다"
if ! printf '%s\n' "$simpleperf_features" | grep -Fxq 'trace-offcpu'; then
    fail "기기 simpleperf가 trace-offcpu를 지원하지 않습니다"
fi
simpleperf_record_help="$(adb -s "$adb_serial" shell simpleperf help record 2>&1)" \
    || fail "기기 simpleperf record 사용법을 읽지 못했습니다"
if ! printf '%s\n' "$simpleperf_record_help" | grep -Fq -- '--app package_name'; then
    fail "기기 simpleperf가 debuggable 앱의 --app 프로파일링을 지원하지 않습니다"
fi

package_dump="$(adb -s "$adb_serial" shell dumpsys package "$app_id" 2>&1)" \
    || fail "Spinon 진단 앱의 패키지 상태를 읽지 못했습니다"
if ! printf '%s\n' "$package_dump" | grep -Fq 'DEBUGGABLE'; then
    fail "simpleperf --app 프로파일링에는 debuggable 진단 APK가 필요합니다"
fi

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
matrix_dir="${output_root}/${stamp}"
mkdir -p "${matrix_dir}/runs"
printf 'started_utc=%s\nrepeats=%s\nduration_seconds=%s\nsample_frequency_hz=%s\nadb_serial=%s\nandroid_api=%s\ndevice_model=%s\ndevice_abi=%s\n' \
    "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$repeats" "$duration_seconds" \
    "$sample_frequency" "$adb_serial" "$api_level" \
    "$(adb -s "$adb_serial" shell getprop ro.product.model | tr -d '\r')" \
    "$(adb -s "$adb_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" \
    > "${matrix_dir}/metadata.txt"
printf 'simpleperf_features_sha256=%s\n' \
    "$(printf '%s\n' "$simpleperf_features" | shasum -a 256 | awk '{print $1}')" \
    >> "${matrix_dir}/metadata.txt"

read_window_xml() {
    adb -s "$adb_serial" shell uiautomator dump /sdcard/spinon-simpleperf-window.xml \
        >/dev/null 2>&1 || true
    adb -s "$adb_serial" exec-out cat /sdcard/spinon-simpleperf-window.xml \
        2>/dev/null || true
}

find_text_center() {
    local wanted_text="$1"
    local required_enabled="$2"
    local max_attempts="${3:-30}"
    local attempt xml node bounds x1 y1 x2 y2
    for ((attempt = 0; attempt < max_attempts; attempt++)); do
        xml="$(read_window_xml)"
        node="$(printf '%s' "$xml" | tr '>' '\n' | grep -F "text=\"${wanted_text}\"" | head -n 1 || true)"
        if [[ -n "$node" && ( "$required_enabled" != "true" || "$node" == *'enabled="true"'* ) ]]; then
            bounds="$(printf '%s\n' "$node" | sed -nE 's/.*bounds="\[([0-9]+),([0-9]+)\]\[([0-9]+),([0-9]+)\]".*/\1 \2 \3 \4/p')"
            if [[ -n "$bounds" ]]; then
                read -r x1 y1 x2 y2 <<< "$bounds"
                printf '%s %s\n' "$(((x1 + x2) / 2))" "$(((y1 + y2) / 2))"
                return 0
            fi
        fi
        sleep 0.25
    done
    return 1
}

capture_schedstat_snapshot() {
    local output_path="$1"
    local app_pid="$2"
    local owner_tid="$3"
    local device_uptime schedstat

    device_uptime="$(adb -s "$adb_serial" shell cat /proc/uptime 2>/dev/null | awk '{print $1}' | tr -d '\r')"
    if [[ ! "$device_uptime" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
        {
            printf 'status=unavailable\n'
            printf 'reason=device uptime counter is not readable\n'
        } > "$output_path"
        return 1
    fi
    if schedstat="$(adb -s "$adb_serial" shell cat "/proc/${app_pid}/task/${owner_tid}/schedstat" 2>/dev/null | tr -d '\r')" \
        && [[ "$schedstat" =~ ^[[:space:]]*[0-9]+[[:space:]]+[0-9]+[[:space:]]+[0-9]+[[:space:]]*$ ]]; then
        {
            printf 'status=available\n'
            printf 'android_pid=%s\n' "$app_pid"
            printf 'android_tid=%s\n' "$owner_tid"
            printf 'device_uptime_seconds=%s\n' "$device_uptime"
            printf 'cpu_time_ns=%s\n' "$(awk '{print $1}' <<< "$schedstat")"
            printf 'runqueue_wait_ns=%s\n' "$(awk '{print $2}' <<< "$schedstat")"
            printf 'timeslices=%s\n' "$(awk '{print $3}' <<< "$schedstat")"
        } > "$output_path"
        return 0
    fi

    {
        printf 'status=unavailable\n'
        printf 'device_uptime_seconds=%s\n' "$device_uptime"
        printf 'reason=thread schedstat is not readable or does not expose three fields\n'
    } > "$output_path"
    return 1
}

write_schedstat_delta() {
    local before_path="$1"
    local after_path="$2"
    local output_path="$3"
    local before_status after_status

    before_status="$(sed -n 's/^status=//p' "$before_path")"
    after_status="$(sed -n 's/^status=//p' "$after_path")"
    if [[ "$before_status" != "available" || "$after_status" != "available" ]]; then
        printf 'status=unavailable\nreason=one or both snapshots are unavailable\n' > "$output_path"
        return 1
    fi
    if [[ "$(sed -n 's/^android_pid=//p' "$before_path")" != \
            "$(sed -n 's/^android_pid=//p' "$after_path")" \
        || "$(sed -n 's/^android_tid=//p' "$before_path")" != \
            "$(sed -n 's/^android_tid=//p' "$after_path")" ]]; then
        printf 'status=invalid\nreason=process or thread identity changed between snapshots\n' > "$output_path"
        return 1
    fi

    awk -F= '
        FNR == NR { before[$1] = $2; next }
        { after[$1] = $2 }
        END {
            interval = after["device_uptime_seconds"] - before["device_uptime_seconds"]
            cpu_delta = after["cpu_time_ns"] - before["cpu_time_ns"]
            wait_delta = after["runqueue_wait_ns"] - before["runqueue_wait_ns"]
            slices_delta = after["timeslices"] - before["timeslices"]
            if (interval <= 0 || cpu_delta < 0 || wait_delta < 0 || slices_delta < 0) {
                printf "status=invalid\nreason=counter interval or delta is invalid\n"
                exit 1
            }
            printf "status=available\n"
            printf "sample_interval_device_uptime_seconds=%.6f\n", interval
            printf "cpu_time_delta_ns=%.0f\n", cpu_delta
            printf "runqueue_wait_delta_ns=%.0f\n", wait_delta
            printf "timeslices_delta=%.0f\n", slices_delta
        }
    ' "$before_path" "$after_path" > "$output_path"
}

for ((run = 1; run <= repeats; run++)); do
    run_dir="${matrix_dir}/runs/run-${run}"
    mkdir -p "$run_dir"
    remote_profile="/data/local/tmp/spinon-simpleperf-offcpu-${stamp}-${run}.perf.data"
    adb -s "$adb_serial" logcat -c
    adb -s "$adb_serial" shell am force-stop "$app_id"
    adb -s "$adb_serial" shell am start -W -n "${app_id}/.MainActivity" \
        --ez spinon_runtime_threads true > "${run_dir}/activity-start.txt"

    start_center="$(find_text_center '긴 JavaScript 실행 시작' true)" \
        || fail "실행 ${run}: 활성화된 긴 JavaScript 시작 버튼을 찾지 못했습니다"
    cancel_center="$(find_text_center '실행 취소' false)" \
        || fail "실행 ${run}: 취소 버튼 위치를 찾지 못했습니다"
    read -r start_x start_y <<< "$start_center"
    read -r cancel_x cancel_y <<< "$cancel_center"
    adb -s "$adb_serial" shell input tap "$start_x" "$start_y"
    running_xml=''
    for ((attempt = 0; attempt < 20; attempt++)); do
        running_xml="$(read_window_xml)"
        if [[ "$running_xml" == *'긴 JavaScript 실행 중 · 화면 입력 가능 · JS 이벤트 대기 중'* ]]; then
            break
        fi
        sleep 0.1
    done
    if [[ "$running_xml" != *'긴 JavaScript 실행 중 · 화면 입력 가능 · JS 이벤트 대기 중'* ]]; then
        fail "실행 ${run}: 긴 JavaScript 실행 상태를 화면에서 확인하지 못했습니다"
    fi
    running_cancel_node="$(printf '%s' "$running_xml" \
        | tr '>' '\n' \
        | grep -F 'text="실행 취소"' \
        | head -n 1 || true)"
    if [[ -z "$running_cancel_node" || "$running_cancel_node" != *'enabled="true"'* ]]; then
        fail "실행 ${run}: 긴 JavaScript 실행 중 취소 버튼이 활성 상태가 아닙니다"
    fi
    printf '%s\n' "$running_xml" > "${run_dir}/running-window.xml"

    owner_tid="$(adb -s "$adb_serial" logcat -d -s SpinonBootstrap:I \
        | sed -n 's/.*SPINON_RUNTIME_SESSION=session=ready owner_tid=\([0-9][0-9]*\).*/\1/p' \
        | tail -n 1)"
    app_pid="$(adb -s "$adb_serial" shell pidof "$app_id" | tr -d '\r')"
    [[ "$owner_tid" =~ ^[0-9]+$ && "$app_pid" =~ ^[0-9]+$ ]] \
        || fail "실행 ${run}: 앱 PID 또는 JavaScript owner TID를 찾지 못했습니다"
    printf 'app_id=%s\nandroid_pid=%s\nandroid_owner_tid=%s\n' \
        "$app_id" "$app_pid" "$owner_tid" > "${run_dir}/owner-thread.txt"
    capture_schedstat_snapshot "${run_dir}/schedstat-before.txt" "$app_pid" "$owner_tid" || true

    adb -s "$adb_serial" shell "simpleperf record --app $app_id -e cpu-clock -f $sample_frequency --trace-offcpu --duration $duration_seconds -g -o $remote_profile" \
        > "${run_dir}/simpleperf-record.txt" 2>&1 \
        || fail "실행 ${run}: simpleperf 기록에 실패했습니다 (${run_dir}/simpleperf-record.txt)"
    capture_schedstat_snapshot "${run_dir}/schedstat-after.txt" "$app_pid" "$owner_tid" || true
    write_schedstat_delta \
        "${run_dir}/schedstat-before.txt" \
        "${run_dir}/schedstat-after.txt" \
        "${run_dir}/schedstat-delta.txt" || true
    if ! grep -Eq 'Samples recorded: [1-9][0-9,]*\. Samples lost: 0\.' \
            "${run_dir}/simpleperf-record.txt"; then
        fail "실행 ${run}: 표본이 없거나 유실되었습니다 (${run_dir}/simpleperf-record.txt)"
    fi

    adb -s "$adb_serial" pull "$remote_profile" "${run_dir}/profile.perf.data" \
        > "${run_dir}/profile-pull.txt"
    printf '%s\n' "$(adb -s "$adb_serial" shell pidof "$app_id" | tr -d '\r')" \
        > "${run_dir}/app-pid.txt"
    printf '%s\n' "$(adb -s "$adb_serial" logcat -d -s SpinonBootstrap:I)" \
        > "${run_dir}/logcat.txt"
    if ! grep -Fq '무한 JavaScript 평가 제출 · UI 메인 스레드는 대기하지 않음' \
            "${run_dir}/logcat.txt"; then
        fail "실행 ${run}: 긴 JavaScript 진입 로그가 없습니다"
    fi
    if grep -Fq '시간 초과 · 안전을 위해 V8 취소를 요청합니다' "${run_dir}/logcat.txt"; then
        fail "실행 ${run}: profiler 종료 전에 긴 JavaScript 자동 취소가 발생했습니다"
    fi
    printf 'action=cancel_after_profile host_time=%s x=%s y=%s\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$cancel_x" "$cancel_y" \
        > "${run_dir}/input-events.txt"
    adb -s "$adb_serial" shell input tap "$cancel_x" "$cancel_y"
    for ((attempt = 0; attempt < 40; attempt++)); do
        adb -s "$adb_serial" logcat -d -s SpinonBootstrap:I > "${run_dir}/logcat.txt"
        if grep -Fq '긴 평가 반환 · status=-8' "${run_dir}/logcat.txt"; then
            break
        fi
        sleep 0.1
    done
    if ! grep -Fq '취소 요청 · status=0' "${run_dir}/logcat.txt" \
            || ! grep -Fq '긴 평가 반환 · status=-8' "${run_dir}/logcat.txt"; then
        fail "실행 ${run}: 취소 요청·반환 결과를 확인하지 못했습니다"
    fi
    if grep -Fq '시간 초과 · 안전을 위해 V8 취소를 요청합니다' "${run_dir}/logcat.txt"; then
        fail "실행 ${run}: profiler가 12초 자동 취소 뒤에 끝나 결과가 오염됐습니다"
    fi

    adb -s "$adb_serial" shell sha256sum "$remote_profile" \
        > "${run_dir}/device-profile.sha256"
    shasum -a 256 "${run_dir}/profile.perf.data" > "${run_dir}/host-profile.sha256"
    printf '실행 %s/%s: 앱 프로세스 on/off-CPU를 기록하고 JavaScript owner TID를 확인했습니다.\n' \
        "$run" "$repeats"
    if grep -Fq 'status=available' "${run_dir}/schedstat-delta.txt"; then
        printf '  schedstat 보조 카운터: %s\n' \
            "$(tr '\n' ' ' < "${run_dir}/schedstat-delta.txt" | sed 's/[[:space:]]*$//')"
    else
        printf '  schedstat 보조 카운터는 기기 권한 또는 커널 설정으로 읽을 수 없었습니다.\n'
    fi
done

printf 'Android simpleperf off-CPU 결과를 저장했습니다: %s\n' "$matrix_dir"
printf '프로파일은 지정된 debuggable 앱에 한정하며 다른 기기·OS·release 성능으로 일반화하지 않습니다.\n'
