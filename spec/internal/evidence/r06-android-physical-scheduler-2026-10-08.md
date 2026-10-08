# R06 · Android 실기기 JavaScript owner 스케줄링 계측

**측정일:** 2026-10-08 KST · **상태:** 실기기 원인 분리 근거 · **R06 완료 판정:** 미완료

## 확인 결과

Android 실기기에서 JavaScript owner thread의 CPU 실행 표본과 schedule-out→schedule-in 사이의 off-CPU 시간을 확인했다. 앞서 Perfetto `linux.ftrace` 데이터 소스가 없는 장치라 Perfetto만으로는 scheduler 이벤트를 얻을 수 없다고 판단했지만, 그 판단은 너무 넓었다. 이 기기의 `simpleperf`는 `trace-offcpu`와 디버그 앱 대상 `--app` 기록을 지원한다.

같은 무한 동기 JavaScript 작업을 다섯 번 실행해 owner thread에서 101개의 짝지어진 off-CPU 구간을 기록했다. 구간 길이는 p50 **15.117µs**, p95 **69.102µs**, 최댓값 **730.976µs**였다. 각 실행의 `cpu-clock` 표본은 약 8,000개였고 `simpleperf`는 표본 유실 0건을 보고했다.

별도 1회 실행에서는 `/proc/<pid>/task/<tid>/schedstat` 누적 카운터도 프로파일 전후로 읽었다. 8.8초 간격 동안 owner thread CPU 시간은 8.711520849초, runqueue 대기 카운터는 0.228829ms, timeslice 수는 24 증가했다. 이는 같은 실행 구간의 간단한 교차 확인이다. 누적값이라 개별 schedule-out 구간이나 깨움 원인을 보여주지는 않으며, 5회 off-CPU 표본과 합쳐 통계 내지 않았다. Linux 커널 문서는 schedstat 세 필드를 CPU 시간, runqueue 대기 시간, timeslice 수로 정의한다. [Linux scheduler statistics](https://docs.kernel.org/scheduler/sched-stats.html)

기존 같은 기기·같은 긴 JavaScript 조건의 5회 입력 행렬에서 JS dispatch queue residence p50은 **1,100,059µs**였고, V8 dispatch p50은 **17µs**였다. 따라서 관측한 약 1.10초의 dispatch 대기는 OS가 owner thread를 다시 스케줄하기까지의 대기만으로 설명되지 않는다. 실행 중인 동기 JavaScript 작업이 끝날 때까지 같은 Isolate의 다음 작업이 큐에서 대기하는 head-of-line blocking이 지배적인 경로다. 이 결과는 해당 디버그 진단 화면과 기기에서의 원인 분리이며, 제품 프레임 성능이나 다른 기기의 성능 우위를 주장하지 않는다.

## 측정 환경과 방법

| 항목 | 값 |
| --- | --- |
| 기기 | `SM-S731N`, ARM64, 화면 1080×2340, active refresh 60Hz (지원 모드 60/120Hz) |
| OS | Android 16 / API 36 |
| 앱 | `dev.spinon.bootstrap`, debuggable APK, `v8_jitless=false` |
| APK SHA-256 | `1767ff14b69a8451a272d060b4467a54f777daf8941da0a36b0477ad1560bf84` |
| 소스 기준 | `bb4d41f4edfdef76f8d24b429ea8b8cff2e0c811` |
| 시나리오 | V8 owner에서 `while (true) { /* 취소 경로 검증 */ }` 평가를 실행하고 그 실행 중 앱 프로세스만 프로파일링 |
| 프로파일러 | `simpleperf record --app dev.spinon.bootstrap -e cpu-clock -f 1000 --trace-offcpu --duration 8 -g` |
| 반복 | 독립 프로세스 5회, 각 8초 |
| 앱 owner | 매 실행 Logcat의 `SPINON_RUNTIME_SESSION owner_tid`와 profile 내 TID를 연결 |
| 보조 계측 | 별도 1회에서 앱 owner thread의 `/proc/<pid>/task/<tid>/schedstat` 전후값과 기기 `/proc/uptime`을 읽음 |
| 입력 방식 | 시작·종료 버튼에 ADB 탭을 주입. 사람 손가락 입력·입력→화면 표시 시간은 측정하지 않음 |

Android `simpleperf` 문서에 따르면 `--trace-offcpu`는 `cpu-clock` 표본과 `sched:sched_switch` 이벤트를 함께 기록하며, schedule-out 표본에서 다음 schedule-in까지를 off-CPU 시간으로 계산한다. 이 보고서의 간격은 profile의 owner TID에 대한 `context_switch` 기록에서 `switch_on=false` 다음 `switch_on=true`를 짝지어 다시 계산했다. [Android simpleperf off-CPU 기록 방식](https://android.googlesource.com/platform/system/extras/+/refs/heads/main/simpleperf/doc/executable_commands_reference.md)

## 결과

실행별 p95는 소표본에서 nearest-rank 방식으로 계산했다. 전체 p50은 101개 구간의 중앙값이고, 전체 p95는 nearest-rank 방식이다.

| 실행 | owner TID | owner `cpu-clock` 표본 | 짝지은 off-CPU 구간 | off-CPU 합계 | p50 | p95 | 최대 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 6082 | 8,020 | 29 | 381.757µs | 12.539µs | 20.976µs | 28.164µs |
| 2 | 6430 | 8,019 | 20 | 442.850µs | 13.632µs | 69.102µs | 98.594µs |
| 3 | 6641 | 8,016 | 20 | 1,753.752µs | 19.570µs | 574.297µs | 730.976µs |
| 4 | 6865 | 8,023 | 11 | 199.883µs | 16.406µs | 32.305µs | 32.305µs |
| 5 | 7226 | 8,020 | 21 | 321.521µs | 15.117µs | 23.008µs | 30.195µs |
| 전체 | 실행별 TID 연결 | 40,098 | 101 | 3,099.763µs | **15.117µs** | **69.102µs** | **730.976µs** |

큐 지표는 별도의 paired matrix에서 수집했으므로 off-CPU 표본과 같은 분모로 합산하지 않는다.

| 조건 | queue residence p50 | queue residence p95 | V8 호출 p50 |
| --- | ---: | ---: | ---: |
| 무한 동기 JS 실행 중 제출한 dispatch | 1,100,059µs | 1,483,983µs | 17µs |
| 일반 `spinon-event` | 226µs | 1,400µs | 565µs |

긴 JS 조건에서 queue p50은 일반 조건보다 약 4,867배 길었지만, V8 dispatch 구간 p50 자체는 17µs였다. 이는 대기열 체류와 실행 중인 동기 JavaScript의 비선점 동작을 구분해 보여준다. 작업 우선순위가 더 높아도 현재 V8 평가가 반환되기 전에는 다음 JS 작업을 실행하지 않는다.

## Android ATrace 직접 wake 원인

Perfetto `linux.ftrace` 데이터 소스는 이 기기에 없지만, `adb shell atrace --async_start -b 16384 sched`로 Android 커널 `sched_waking`, `sched_wakeup`, `sched_switch` 이벤트를 수집할 수 있었다. 앱 구간과 함께 보기 위해 한 반복에서는 `-a dev.spinon.bootstrap`을 추가해 이미 앱에 있던 ATrace 표식도 보존했다. 이 경로는 simpleperf의 system-wide kernel sample 권한 오류와 앱 범위 tracepoint의 0 samples를 우회했다. 시도 결과는 [simpleperf 시도 기록](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/simpleperf-attempts.txt)에 기록했다.

Linux trace event 형식에서 sched_waking·sched_wakeup의 pid는 깨우는 대상이고 trace 행의 task/TID는 이벤트를 낸 쪽이다. sched_switch의 next_pid로 대상이 실제 CPU에 올라온 시각을 짝지었다. 이 방식은 이벤트별 wakeup 요청자와 요청 후 실행까지의 시간을 식별한다. [Linux event tracing](https://docs.kernel.org/trace/events.html) · [커널의 wakeup-to-switch 지연 계산 예시](https://docs.kernel.org/next/trace/histogram.html)

다섯 번의 Android 실기기 캡처 모두에서 JS owner TID 15638 대상으로 `spinon-platform`의 wake 요청이 관측됐다. Java [`ThreadPoolExecutor`](../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java#L68-L72)의 실제 스레드 이름은 `spinon-platform-call`이며 Linux thread name 길이 제한으로 trace에는 `spinon-platform`으로 보인다. 앱 표식이 있는 캡처에서는 `scheduler-enqueue` 직후 이 스레드의 wake 요청이 이어졌다. Rust는 [작업 삽입 뒤 `Condvar::notify_one()`](../../../crates/spinon-runtime/src/session/scheduler.rs#L49)으로 owner를 알리고, owner는 빈 큐에서 [`Condvar::wait()`](../../../crates/spinon-runtime/src/session/scheduler.rs#L75-L91) 중이다. 계측 순서와 코드 경계가 일치한다.

| 캡처 | owner wake 대상 / 발생 주체 TID | wakeup 후 첫 switch-in | owner가 CPU에서 나갈 때의 상태 | trace 기록 수 |
| --- | --- | ---: | --- | ---: |
| 1 | spinon-platform 16035 · V8 DefaultWorke 15646 | 691µs · 99µs | R 10회 · S 1회 | 21,641 / 21,641 |
| 2 | spinon-platform 15749 | 15µs | R 4회 · S 0회 | 13,118 / 13,118 |
| 3 | spinon-platform 15884 | 40µs | R 7회 · S 0회 | 14,111 / 14,111 |
| 취소 포함 | spinon-platform 16035 | 20µs | R 5회 · S 1회 | 21,407 / 21,407 |
| 앱 표식 포함 | spinon-platform 15637 · V8 DefaultWorke 15646 | 27µs · 13µs | R 4회 · S 2회 | 19,239 / 19,239 |

R로 owner가 나간 구간은 계속 runnable한 채 CPU를 양보하거나 다른 CPU로 이동한 것이므로, 이후 sched_wakeup이 아니라 sched_switch에서 실행 재개를 확인했다. 두 캡처에서는 `V8 DefaultWorke`가 추가로 owner를 깨웠다. 이 이름은 Linux thread name 길이 제한으로 잘린 값이다. 코드는 [`v8::platform::NewDefaultPlatform()`](../../../native/v8/src/spinon_v8.cc#L78)으로 V8 플랫폼을 만들지만, trace만으로 내부 V8 작업의 정확한 호출 지점이나 작업 종류까지는 식별하지 않았다. 따라서 이 간헐적 wake는 큐 제출과 연결된 다섯 번의 platform-call wake와 구분해 미확정 항목으로 남긴다. 실행별 wake 행, TID, latency, 상태 및 buffer 수는 [wakeup-events.csv](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/wakeup-events.csv)에 두었다. [압축 원본 다섯 개와 SHA-256](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/) 및 [대응 Logcat](r06-android-physical-scheduler-2026-10-08/logs/atrace-wakeup/)을 보관한다. 다섯 trace 모두 `entries-in-buffer == entries-written`이며 기록 손실·buffer overrun은 없었다.

취소 시점까지 포함한 앱 표식 캡처에서는 `long-eval-cancel-submit` (2867134.500612), `long-eval-cancel-call` (2867134.502472), owner의 `reply-send` (2867134.502620), `actor-condvar-wait` (2867134.502645), owner `sched_switch prev_state=S` (2867134.502652) 순서였다. 취소 호출 뒤 owner 대상 `sched_wakeup`은 0건이다. [대응 Logcat](r06-android-physical-scheduler-2026-10-08/logs/atrace-wakeup/app-markers-1-logcat.txt)의 평가는 `status=-8`, `v8_call_us=2095577`로 반환했다. Android [`runtime-control` 스레드의 취소 호출](../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java#L628-L636)은 이미 동기 JavaScript를 실행 중인 owner를 OS scheduler wake로 깨우지 않았다. V8 평가가 종료 요청을 확인해 반환한 뒤 actor가 큐 대기로 복귀했다. V8 경계는 [`TerminateExecution()`](../../../native/v8/src/spinon_v8.cc#L1000-L1002)에 연결된다. 이 결론은 이 캡처에서 확인된 이벤트 순서에 한정한다.

판정은 이 한 Android 16 실기기·같은 debuggable APK의 진단 시나리오에 한정한다. 수 microsecond wake-to-run 값은 ATrace 활성 상태의 작은 표본이므로 성능 합격 기준이나 다른 기기·release 성능으로 일반화하지 않는다. 직접 원인 귀속은 가능해졌다. 이 시나리오에서 작업 제출과 함께 관측된 owner wake는 Android platform-call worker가 Rust Condvar를 알린 결과다. 긴 동기 JS 중 관측한 다른 CPU 전환은 대부분 runnable 상태의 재선택이었다. 취소 trace에서는 취소 호출 뒤 owner 대상 OS wakeup이 없었다.

## Perfetto 경로를 사용하지 않은 이유

이 기기의 `adb shell perfetto --query`에는 `linux.ftrace`가 등록되어 있지 않았다. 동일한 `linux.ftrace` scheduler 설정의 실기기 smoke trace는 774바이트였고 `sched_slice`, `thread_state`, raw ftrace event가 모두 0이었다. Android Emulator 양성 대조에서는 같은 최소 설정이 scheduler event를 기록했다. 이 장치에서 Perfetto ftrace의 빈 테이블을 scheduler idle로 해석하면 안 된다.

등록된 `linux.perf`로 앱 명령줄을 지정한 보조 실험도 했다. 이 결과는 7,830개 표본 중 7,379개가 idle `swapper`였고 앱에 귀속 가능한 callsite 표본은 12개뿐이라 앱 scheduler 분석에 사용하지 않았다. 보조 trace와 설정은 [`traces/perfetto-linux-perf-physical-probe.pftrace`](r06-android-physical-scheduler-2026-10-08/traces/perfetto-linux-perf-physical-probe.pftrace), [`설정`](r06-android-physical-scheduler-2026-10-08/traces/perfetto-linux-perf-physical-probe.textproto)에 둔다.

## 한계와 남은 확인
- simpleperf --trace-offcpu 자체는 개별 switch state나 waker를 저장하지 않는다. 이번 별도 ATrace 추적이 그 경계를 보완해 작업 제출 시 깨운 TID와 R/S 전환을 식별했다. 간헐적으로 관찰된 V8 DefaultWorker의 정확한 내부 호출 지점·작업 종류, 다른 workload의 wake source는 커널 call stack 없이 확정하지 않았다.
- `simpleperf` profile은 schedule-out부터 schedule-in까지의 off-CPU 경과를 측정하지만 개별 `sched_switch` 이전 상태나 waker를 보존하지 않는다. ATrace 보완 캡처로 큐 제출 wake 요청자와 일부 R/S 전환은 식별했다. 다만 simpleperf의 101개 off-CPU 구간 각각을 ATrace 이벤트와 일대일 연결하지 않았으므로 모든 구간의 대기 원인 분류는 남아 있다.
- Perfetto `linux.ftrace`가 없는 이 기기의 앱 프레임 `sched_slice`·`thread_state`와 사용자 입력 시각은 수집하지 않았다. `simpleperf`는 owner scheduling만 보고 GPU·화면 frame 지연을 설명하지 않는다.
- `cpu-clock` 1kHz의 디버그 표본은 원인 분석이다. 프로파일러 overhead on/off 대조, release 빌드, 반복 refresh mode 대조를 하지 않았으므로 성능 벤치마크로 해석하지 않는다.
- `schedstat` 보조 실행은 누적 CPU·runnable wait만 비교한다. 샘플 간격 동안 runnable 상태가 아닌 blocked/off-CPU 시간을 분리하지 못하고, 개별 wakeup/schedule 이벤트의 원인 귀속 자료로 대체하지 않는다.
- Android 실기기 한 대만 확인했다. iOS 실기기의 OS scheduler 귀속은 미검증이며, 기존 iOS Simulator callback/`CADisplayLink` 결과를 대신하지 않는다.
- 긴 JS 5회 matrix의 기존 ADB 입력 결과와 이번 simpleperf 표본은 모두 실기기 코드 경로 확인이다. 물리 손가락 입력, optical input-to-photon latency, 다른 앱 부하 조건은 미측정이다.

## 원본 근거

- 직접 wake 이벤트 7건과 capture 요약: [wakeup-events.csv](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/wakeup-events.csv)
- ATrace 설정·simpleperf 대체 시도: [capture-conditions.txt](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/capture-conditions.txt), [simpleperf-attempts.txt](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/simpleperf-attempts.txt)
- 압축 trace 원본: [5개 trace](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/) · [SHA-256 목록](r06-android-physical-scheduler-2026-10-08/traces/atrace-wakeup/SHA256SUMS)
- 취소·앱 표식 로그: [Logcat 원본](r06-android-physical-scheduler-2026-10-08/logs/atrace-wakeup/)

- 5개 압축 profile: [`traces/simpleperf/`](r06-android-physical-scheduler-2026-10-08/traces/simpleperf/)
- owner별 off-CPU 구간 101개: [`offcpu-intervals.csv`](r06-android-physical-scheduler-2026-10-08/traces/simpleperf/offcpu-intervals.csv)
- 실행별·전체 집계: [`offcpu-summary.csv`](r06-android-physical-scheduler-2026-10-08/traces/simpleperf/offcpu-summary.csv), [`offcpu-overall.csv`](r06-android-physical-scheduler-2026-10-08/traces/simpleperf/offcpu-overall.csv)
- owner TID, 시작·종료, 취소 결과와 sample loss 기록: [`logs/simpleperf/`](r06-android-physical-scheduler-2026-10-08/logs/simpleperf/)
- 보조 schedstat 전후 카운터, 실행 로그와 대응 profile: [`logs/schedstat-crosscheck-20261007T233634Z/`](r06-android-physical-scheduler-2026-10-08/logs/schedstat-crosscheck-20261007T233634Z/), [`압축 profile`](r06-android-physical-scheduler-2026-10-08/traces/simpleperf/schedstat-crosscheck-20261007T233634Z.perf.data.gz)
- 기존 5회 dispatch queue 행렬: `build/spinon/benchmark/android-physical-long-js/20261007T230231Z/`에서 요약한 로그와 trace는 같은 evidence 하위의 [`logs/`](r06-android-physical-scheduler-2026-10-08/logs/), [`traces/`](r06-android-physical-scheduler-2026-10-08/traces/)에 보관한다.
- 실행 명령: [`capture-android-simpleperf-offcpu.sh`](../../../tools/benchmark/capture-android-simpleperf-offcpu.sh)
