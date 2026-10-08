#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
app_id="dev.spinon.bootstrap"
scenario=""
input_count=10
duration_seconds=20
adb_serial="${SPINON_ANDROID_SERIAL:-}"
output_root="${repo_root}/build/spinon/benchmark/android-frame-attribution"
trace_config="${SPINON_ANDROID_TRACE_CONFIG:-${repo_root}/tools/benchmark/android-frame-attribution.textproto}"
skip_install=false
async_main_handoff=false
perfetto_enabled=true
if [[ "${trace_config}" != /* ]]; then
    trace_config="${repo_root}/${trace_config}"
fi

usage() {
    cat <<'EOF'
사용법:
  bash tools/benchmark/capture-android-frame-attribution.sh <조건> [옵션]

조건:
  idle          같은 Android 대조 화면을 띄우고 입력하지 않음
  input-only    탭 입력만 처리하고 화면 텍스트는 바꾸지 않음
  status        탭마다 상태 TextView만 갱신
  log-scroll    탭마다 로그 TextView에 추가하고 ScrollView를 아래로 이동
  spinon-event  실제 Spinon V8 실행 화면에서 터치 이벤트를 보냄
  spinon-ui-only 같은 화면에서 런타임 dispatch를 생략하고 UI 경로만 측정
  spinon-long-js 무한 JavaScript 실행 중 UI 입력·이벤트 대기·취소를 확인

옵션:
  --serial <adb serial>     대상 지정 (기본: 연결 기기가 하나면 자동 선택)
  --taps <횟수>             입력 횟수 (기본: 10, idle은 0)
  --duration <초>           Perfetto 캡처 길이 (기본: 20초)
  --out <디렉터리>          결과 디렉터리
  --skip-install            설치된 APK digest가 현재 빌드와 같을 때 재설치 생략
  --async-main-handoff      spinon-event 완료 callback을 동기화 장벽 우회 Handler에 게시
  --perfetto <on|off>        spinon-event에서 OS Perfetto 수집 여부 (기본: on)

설정:
  SPINON_ANDROID_TRACE_CONFIG  기본 Perfetto 설정 대신 사용할 파일 경로 (절대 경로 또는 저장소 상대 경로)
EOF
}

fail() {
    printf '오류: %s\n' "$1" >&2
    exit 1
}

if [[ $# -lt 1 ]]; then
    usage
    exit 2
fi
scenario="$1"
shift

while [[ $# -gt 0 ]]; do
    case "$1" in
        --serial)
            [[ $# -ge 2 ]] || fail "--serial 값이 없습니다"
            adb_serial="$2"
            shift 2
            ;;
        --taps)
            [[ $# -ge 2 ]] || fail "--taps 값이 없습니다"
            input_count="$2"
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
        --async-main-handoff)
            async_main_handoff=true
            shift
            ;;
        --perfetto)
            [[ $# -ge 2 ]] || fail "--perfetto 값이 없습니다"
            case "$2" in
                on) perfetto_enabled=true ;;
                off) perfetto_enabled=false ;;
                *) fail "--perfetto는 on 또는 off여야 합니다" ;;
            esac
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

case "$scenario" in
    idle|input-only|status|log-scroll|spinon-event|spinon-ui-only|spinon-long-js) ;;
    *) fail "지원하지 않는 조건: $scenario" ;;
esac
if [[ "$async_main_handoff" == "true" && "$scenario" != "spinon-event" ]]; then
    fail "--async-main-handoff는 spinon-event 조건에서만 사용할 수 있습니다"
fi
if [[ "$perfetto_enabled" == "false" && "$scenario" != "spinon-event" ]]; then
    fail "--perfetto off는 spinon-event trace 오버헤드 대조에서만 사용할 수 있습니다"
fi
[[ "$input_count" =~ ^[0-9]+$ ]] || fail "--taps는 0 이상의 정수여야 합니다"
[[ "$duration_seconds" =~ ^[0-9]+$ ]] || fail "--duration은 양의 정수여야 합니다"
(( duration_seconds >= 10 )) || fail "--duration은 최소 10초여야 합니다"
if [[ "$scenario" == "idle" ]]; then input_count=0; fi
(( input_count <= 100 )) || fail "한 회차 입력은 최대 100회입니다"
(( input_count <= 2 * (duration_seconds - 3) )) || fail "탭 횟수가 trace 길이에 비해 많습니다"

command -v adb >/dev/null 2>&1 || fail "adb를 찾을 수 없습니다"
command -v shasum >/dev/null 2>&1 || fail "shasum을 찾을 수 없습니다"

if [[ -z "$adb_serial" ]]; then
    device_list="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
    device_count="$(printf '%s\n' "$device_list" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$device_count" == "1" ]] || fail "연결 기기가 ${device_count}개입니다. --serial로 대상을 고정하세요"
    adb_serial="$device_list"
fi

adb -s "$adb_serial" get-state | grep -qx device || fail "대상 기기에 adb로 연결할 수 없습니다: $adb_serial"
api_level="$(adb -s "$adb_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
(( api_level >= 31 )) || fail "Perfetto FrameTimeline은 Android 12/API 31 이상이 필요합니다 (현재 API $api_level)"
is_emulator="$(adb -s "$adb_serial" shell getprop ro.kernel.qemu | tr -d '\r')"
if [[ "$is_emulator" == "1" ]]; then
    device_kind="에뮬레이터"
else
    device_kind="실기기"
fi
if [[ "$perfetto_enabled" == "true" ]]; then
    perfetto_sources="$(adb -s "$adb_serial" shell perfetto --query 2>&1)" \
        || fail "기기 Perfetto 데이터 소스 조회에 실패했습니다"
    if ! printf '%s\n' "$perfetto_sources" | grep -Fq 'linux.ftrace'; then
        fail "기기 Perfetto에 linux.ftrace 데이터 소스가 없습니다. 이 추적 설정으로 앱 ATrace·커널 스케줄러 원인을 수집할 수 없습니다. Android off-CPU scheduler 진단에는 tools/benchmark/capture-android-simpleperf-offcpu.sh를 사용하세요"
    fi
fi

case "$scenario" in
    spinon-event|spinon-ui-only|spinon-long-js)
        activity="${app_id}/.MainActivity"
        expected_activity="$activity"
        start_args=(--ez spinon_runtime_threads true)
        if [[ "$scenario" == "spinon-event" || "$scenario" == "spinon-ui-only" ]]; then
            start_args+=(--ez spinon_frame_attribution true
                --ei spinon_frame_duration_seconds "$duration_seconds")
        fi
        if [[ "$async_main_handoff" == "true" ]]; then
            start_args+=(--ez spinon_async_main_handoff true)
        fi
        if [[ "$scenario" == "spinon-ui-only" ]]; then
            start_args+=(--ez spinon_ui_only true)
        fi
        if [[ "$scenario" == "spinon-long-js" ]]; then
            target_text="긴 JavaScript 실행 시작"
        else
            target_text="터치 이벤트 보내기"
        fi
        ;;
    *)
        activity="${app_id}/.MainActivity"
        expected_activity="${app_id}/.FrameAttributionActivity"
        target_text="벤치마크 탭"
        if [[ "$scenario" == "idle" ]]; then
            mode="input-only"
        else
            mode="$scenario"
        fi
        start_args=(--es spinon_frame_mode "$mode")
        ;;
esac

stamp="$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="${output_root}/${stamp}-${scenario}"
mkdir -p "$run_dir"
diagnostic_ui_mutations_suppressed=false
report_logcat_deferred_seconds=none
if [[ "$scenario" == "spinon-event" || "$scenario" == "spinon-ui-only" ]]; then
    diagnostic_ui_mutations_suppressed=true
    report_logcat_deferred_seconds=$((duration_seconds + 10))
fi
printf '%s\n' "$repo_root" > "${run_dir}/repository-root.txt"
remote_trace="/data/misc/perfetto-traces/spinon-r05-${stamp}.pftrace"
duration_ms=$((duration_seconds * 1000))
if [[ "$perfetto_enabled" == "true" ]]; then
    [[ -f "${trace_config}" ]] || fail "Perfetto 설정 파일을 찾지 못했습니다: ${trace_config}"
    sed "s/^duration_ms: [0-9][0-9]*/duration_ms: ${duration_ms}/" \
        "${trace_config}" \
        > "${run_dir}/perfetto.textproto"
else
    printf 'Perfetto 수집을 끈 대조 실행입니다.\n' > "${run_dir}/perfetto-disabled.txt"
fi

debug_apk="${repo_root}/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
[[ -f "$debug_apk" ]] || fail "먼저 mise exec -- bun run build:android 로 APK를 빌드하세요"
if [[ "$skip_install" == "false" ]]; then
    adb -s "$adb_serial" install -r "$debug_apk" > "${run_dir}/apk-install.txt"
else
    remote_installed_apk="$(adb -s "$adb_serial" shell pm path "$app_id" \
        | sed -n 's/^package://p' | head -n 1 | tr -d '\r')"
    [[ -n "$remote_installed_apk" ]] || fail "--skip-install 대상 앱이 설치되어 있지 않습니다"
    expected_apk_sha256="$(shasum -a 256 "$debug_apk" | awk '{print $1}')"
    installed_apk_sha256="$(adb -s "$adb_serial" shell sha256sum "$remote_installed_apk" \
        | awk '{print $1}' | tr -d '\r')"
    [[ "$expected_apk_sha256" == "$installed_apk_sha256" ]] \
        || fail "--skip-install 대상 APK가 현재 빌드와 다릅니다"
    printf '설치 생략 · APK SHA-256 일치\n' > "${run_dir}/apk-install.txt"
fi
adb -s "$adb_serial" shell am force-stop "$app_id"
adb -s "$adb_serial" shell am start -W -n "$activity" "${start_args[@]}" \
    > "${run_dir}/activity-start.txt"

read_window_xml() {
    adb -s "$adb_serial" shell uiautomator dump /sdcard/spinon-r05-window.xml \
        >/dev/null 2>&1 || true
    adb -s "$adb_serial" exec-out cat /sdcard/spinon-r05-window.xml 2>/dev/null || true
}

find_text_node() {
    local wanted_text="$1"
    local xml
    xml="$(read_window_xml)"
    printf '%s' "$xml" | tr '>' '\n' | grep -F "text=\"${wanted_text}\"" | head -n 1 || true
}

find_text_center() {
    local wanted_text="$1"
    local max_attempts="${2:-40}"
    local attempt node bounds x1 y1 x2 y2
    attempt=0
    while (( attempt < max_attempts )); do
        node="$(find_text_node "$wanted_text")"
        if [[ -n "$node" && "$node" == *'enabled="true"'* ]]; then
            bounds="$(printf '%s\n' "$node" | sed -nE 's/.*bounds="\[([0-9]+),([0-9]+)\]\[([0-9]+),([0-9]+)\]".*/\1 \2 \3 \4/p')"
            if [[ -n "$bounds" ]]; then
                read -r x1 y1 x2 y2 <<< "$bounds"
                printf '%s %s\n' "$(((x1 + x2) / 2))" "$(((y1 + y2) / 2))"
                return 0
            fi
        fi
        attempt=$((attempt + 1))
        sleep 0.25
    done
    return 1
}

center_from_window_file() {
    local window_file="$1"
    local wanted_text="$2"
    local node bounds x1 y1 x2 y2
    node="$(tr '>' '\n' < "$window_file" | grep -F "text=\"${wanted_text}\"" | head -n 1 || true)"
    [[ -n "$node" ]] || return 1
    bounds="$(printf '%s\n' "$node" | sed -nE 's/.*bounds="\[([0-9]+),([0-9]+)\]\[([0-9]+),([0-9]+)\]".*/\1 \2 \3 \4/p')"
    [[ -n "$bounds" ]] || return 1
    read -r x1 y1 x2 y2 <<< "$bounds"
    printf '%s %s\n' "$(((x1 + x2) / 2))" "$(((y1 + y2) / 2))"
}

wait_for_window_text() {
    local fragment="$1"
    local max_attempts="${2:-12}"
    local attempt=0 xml
    while (( attempt < max_attempts )); do
        xml="$(read_window_xml)"
        if [[ "$xml" == *"$fragment"* ]]; then
            printf '%s' "$xml"
            return 0
        fi
        attempt=$((attempt + 1))
        sleep 0.25
    done
    return 1
}

tap_center="$(find_text_center "$target_text")" \
    || fail "활성화된 탭 대상 '${target_text}'를 찾지 못했습니다"
read -r tap_x tap_y <<< "$tap_center"

if ! adb -s "$adb_serial" shell dumpsys activity activities | grep -Fq "$expected_activity"; then
    fail "실험 앱이 전경 Activity가 아닙니다: $expected_activity"
fi
app_pid="$(adb -s "$adb_serial" shell pidof "$app_id" | awk '{print $1}' | tr -d '\r')"
[[ "$app_pid" =~ ^[0-9]+$ ]] || fail "실험 앱 프로세스 ID를 확인할 수 없습니다"
sleep 5

{
    printf 'scenario=%s\n' "$scenario"
    printf 'started_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'git_commit=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
    git -C "$repo_root" status --short --untracked-files=all > "${run_dir}/source-status.txt"
    printf 'source_tree_sha256=%s\n' "$(git -C "$repo_root" ls-files --cached --others --exclude-standard -z | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"
    printf 'adb_serial=%s\n' "$adb_serial"
    printf 'device_kind=%s\n' "$device_kind"
    printf 'android_api=%s\n' "$api_level"
    printf 'android_release=%s\n' "$(adb -s "$adb_serial" shell getprop ro.build.version.release | tr -d '\r')"
    printf 'device_model=%s\n' "$(adb -s "$adb_serial" shell getprop ro.product.model | tr -d '\r')"
    printf 'device_abi=%s\n' "$(adb -s "$adb_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
    printf 'app_pid=%s\n' "$app_pid"
    printf 'screen_size=%s\n' "$(adb -s "$adb_serial" shell wm size | tr -d '\r')"
    printf 'screen_density=%s\n' "$(adb -s "$adb_serial" shell wm density | tr -d '\r')"
    printf 'duration_seconds=%s\n' "$duration_seconds"
    printf 'warmup_seconds=5\n'
    printf 'input_count=%s\n' "$input_count"
    printf 'perfetto_enabled=%s\n' "$perfetto_enabled"
    printf 'diagnostic_ui_mutations_suppressed=%s\n' "$diagnostic_ui_mutations_suppressed"
    printf 'async_main_handoff=%s\n' "$async_main_handoff"
    printf 'report_logcat_deferred_seconds=%s\n' "$report_logcat_deferred_seconds"
    printf 'tap_x=%s\n' "$tap_x"
    printf 'tap_y=%s\n' "$tap_y"
    printf '\n-- 디스플레이 설정 --\n'
    adb -s "$adb_serial" shell dumpsys display > "${run_dir}/display.txt"
    grep -m 1 'renderFrameRate' "${run_dir}/display.txt" || true
    printf '\n-- 열 상태 --\n'
    adb -s "$adb_serial" shell dumpsys thermalservice > "${run_dir}/thermal.txt" 2>&1 || true
    grep -i -m 5 'status\|temperature' "${run_dir}/thermal.txt" || true
    printf '\n-- 배터리 상태 --\n'
    adb -s "$adb_serial" shell dumpsys battery | grep -E 'level:|status:|temperature:' || true
} > "${run_dir}/metadata.txt"

adb -s "$adb_serial" shell uiautomator dump /sdcard/spinon-r05-window.xml \
    >/dev/null 2>&1 || true
adb -s "$adb_serial" exec-out cat /sdcard/spinon-r05-window.xml \
    > "${run_dir}/window.xml"
adb -s "$adb_serial" exec-out screencap -p > "${run_dir}/before.png"
adb -s "$adb_serial" shell dumpsys gfxinfo "$app_id" reset \
    > "${run_dir}/gfxinfo-before-reset.txt"

measurement_window_start=$SECONDS
perfetto_host_pid=""
if [[ "$perfetto_enabled" == "true" ]]; then
    adb -s "$adb_serial" shell perfetto --txt -c - -o "$remote_trace" \
        < "${run_dir}/perfetto.textproto" > "${run_dir}/perfetto-runner.txt" 2>&1 &
    perfetto_host_pid=$!
fi
sleep 2
if [[ "$perfetto_enabled" == "true" ]] && ! kill -0 "$perfetto_host_pid" 2>/dev/null; then
    wait "$perfetto_host_pid" || true
    fail "Perfetto 캡처가 시작되지 않았습니다. ${run_dir}/perfetto-runner.txt를 확인하세요"
fi

validation_error=""
if [[ "$scenario" == "spinon-long-js" ]]; then
    dispatch_center="$(center_from_window_file "${run_dir}/window.xml" "터치 이벤트 보내기")" \
        || validation_error="초기 화면에서 이벤트 버튼 위치를 찾지 못했습니다"
    cancel_center="$(center_from_window_file "${run_dir}/window.xml" "실행 취소")" \
        || validation_error="초기 화면에서 취소 버튼 위치를 찾지 못했습니다"
    if [[ -n "$dispatch_center" && -n "$cancel_center" ]]; then
        read -r dispatch_x dispatch_y <<< "$dispatch_center"
        read -r cancel_x cancel_y <<< "$cancel_center"
    fi

    printf 'action=start-long-js host_time=%s x=%s y=%s\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$tap_x" "$tap_y" \
        >> "${run_dir}/input-events.txt"
    adb -s "$adb_serial" shell input tap "$tap_x" "$tap_y"
    sleep 0.75

    for ((tap_index = 1; tap_index <= input_count; tap_index++)); do
        [[ -z "$validation_error" ]] || break
        printf 'action=dispatch tap=%s host_time=%s x=%s y=%s\n' \
            "$tap_index" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$dispatch_x" "$dispatch_y" \
            >> "${run_dir}/input-events.txt"
        adb -s "$adb_serial" shell input tap "$dispatch_x" "$dispatch_y"
        sleep 0.25
    done

    if [[ -z "$validation_error" ]]; then
        adb -s "$adb_serial" exec-out screencap -p > "${run_dir}/during.png"
        printf 'action=cancel host_time=%s x=%s y=%s\n' \
            "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$cancel_x" "$cancel_y" \
            >> "${run_dir}/input-events.txt"
        adb -s "$adb_serial" shell input tap "$cancel_x" "$cancel_y"
    fi
else
    for ((tap_index = 1; tap_index <= input_count; tap_index++)); do
        if ! adb -s "$adb_serial" shell dumpsys activity activities | grep -Fq "$expected_activity"; then
            validation_error="입력 ${tap_index}회 직전 실험 Activity가 전경을 잃었습니다"
            break
        fi
        current_pid="$(adb -s "$adb_serial" shell pidof "$app_id" | awk '{print $1}' | tr -d '\r')"
        if [[ "$current_pid" != "$app_pid" ]]; then
            validation_error="입력 ${tap_index}회 직전 앱 PID가 바뀌었습니다"
            break
        fi
        printf 'tap=%s host_time=%s x=%s y=%s\n' \
            "$tap_index" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$tap_x" "$tap_y" \
            >> "${run_dir}/input-events.txt"
        adb -s "$adb_serial" shell input tap "$tap_x" "$tap_y"
        sleep 0.5
    done
fi

if [[ "$perfetto_enabled" == "true" ]]; then
    wait "$perfetto_host_pid"
    adb -s "$adb_serial" pull "$remote_trace" \
        "${run_dir}/frame-attribution.pftrace" >/dev/null
    adb -s "$adb_serial" shell rm -f "$remote_trace"
else
    elapsed_seconds=$((SECONDS - measurement_window_start))
    remaining_seconds=$((duration_seconds - elapsed_seconds))
    if (( remaining_seconds > 0 )); then sleep "$remaining_seconds"; fi
fi
if [[ "$scenario" == "spinon-event" || "$scenario" == "spinon-ui-only" ]]; then
    sleep 6
fi
adb -s "$adb_serial" shell dumpsys gfxinfo "$app_id" framestats \
    > "${run_dir}/gfxinfo-framestats.txt"
final_app_pid="$(adb -s "$adb_serial" shell pidof "$app_id" | awk '{print $1}' | tr -d '\r')"
printf 'app_pid_end=%s\n' "$final_app_pid" >> "${run_dir}/metadata.txt"
if [[ "$final_app_pid" != "$app_pid" && -z "$validation_error" ]]; then
    validation_error="측정 중 앱 프로세스가 재시작되었습니다"
fi
adb -s "$adb_serial" logcat -d --pid="$app_pid" -s SpinonBootstrap:I SpinonFrameBench:I '*:S' \
    > "${run_dir}/logcat.txt"
adb -s "$adb_serial" exec-out screencap -p > "${run_dir}/after.png"
if [[ -z "$validation_error" ]] \
    && ! adb -s "$adb_serial" shell dumpsys activity activities | grep -Fq "$expected_activity"; then
    validation_error="측정 종료 시 실험 Activity가 전경을 잃었습니다"
fi
if [[ "$scenario" == "spinon-long-js" ]]; then
    printf '%s\n' "$(read_window_xml)" > "${run_dir}/completion.xml"
    if ! grep -Fq '취소 완료 · 대기 중이던 JS 이벤트 처리 완료' "${run_dir}/completion.xml"; then
        validation_error="수집 후에도 취소·대기 이벤트 완료 상태가 화면에 나타나지 않았습니다"
    elif ! grep -Fq '통과 · 메인 UI heartbeat' "${run_dir}/logcat.txt" \
            || ! grep -Fq '통과 · 대기 이벤트 처리' "${run_dir}/logcat.txt"; then
        validation_error="Logcat에서 heartbeat 또는 대기 이벤트 성공을 확인하지 못했습니다"
    elif ! grep -Fq '취소 요청 · status=0' "${run_dir}/logcat.txt" \
            || grep -Fq '시간 초과 · 안전을 위해 V8 취소를 요청합니다' "${run_dir}/logcat.txt"; then
        validation_error="수동 취소 성공을 확인하지 못했거나 안전 시간 초과가 발생했습니다"
    fi
fi
if [[ "$scenario" == "spinon-event" ]]; then
    report_count="$(grep -c 'SPINON_RUNTIME_DISPATCH=' "${run_dir}/logcat.txt" || true)"
    bad_report_count="$(grep 'SPINON_RUNTIME_DISPATCH=' "${run_dir}/logcat.txt" \
        | grep -vc 'status=0 ' || true)"
    sequence_values="$(grep 'SPINON_RUNTIME_DISPATCH=' "${run_dir}/logcat.txt" \
        | sed -nE 's/.* seq=([0-9]+) .*/\1/p' | sort -n)"
    sequence_count=0
    expected_sequence=""
    sequence_error=false
    while IFS= read -r sequence; do
        [[ -n "$sequence" ]] || continue
        if [[ ! "$sequence" =~ ^[0-9]+$ ]]; then
            sequence_error=true
            continue
        fi
        if [[ -z "$expected_sequence" ]]; then
            expected_sequence="$sequence"
        elif (( sequence != expected_sequence + 1 )); then
            sequence_error=true
        fi
        expected_sequence="$sequence"
        sequence_count=$((sequence_count + 1))
    done <<< "$sequence_values"
    if [[ "$report_count" -ne "$input_count" || "$bad_report_count" -ne 0 \
        || "$sequence_count" -ne "$input_count" || "$sequence_error" == "true" ]]; then
        validation_error="runtime 보고서 ${input_count}개/status=0/연속 순번 검증 실패 (보고서=${report_count}, 실패=${bad_report_count}, 순번=${sequence_count})"
    fi
fi
remote_apk="$(adb -s "$adb_serial" shell pm path "$app_id" \
    | sed -n 's/^package://p' | head -n 1 | tr -d '\r')"
[[ -n "$remote_apk" ]] || fail "설치된 APK 경로를 찾지 못했습니다"
adb -s "$adb_serial" shell sha256sum "$remote_apk" > "${run_dir}/installed-apk.sha256"

printf 'R05 Android 프레임 추적을 저장했습니다: %s\n' "$run_dir"
if [[ "$is_emulator" == "1" ]]; then
    printf '에뮬레이터 결과는 원인 분석용이며 실기기 성능 주장의 근거가 아닙니다.\n'
else
    printf '실기기 결과는 해당 모델·OS·화면 주사율·빌드 조건에 한정해 해석해야 합니다.\n'
fi
if [[ -n "$validation_error" ]]; then
    fail "$validation_error (원본은 ${run_dir}에 보존했습니다)"
fi
