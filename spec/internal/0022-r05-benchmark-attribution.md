# 0022 · R05 Android 프레임 지연 귀속·계측 계약

**계약 버전:** `0.1.0-draft` · **상태:** Android 진단 fixture 구현 · **제품 성능 완료:** 미완료 · **상태 대장:** R05

## 1. 목적과 경계

이 계약은 Android 화면 지연을 단일 원인으로 부르지 않고 앱 UI 코드, Spinon 런타임 호출, Android 앱 프레임 경로, SurfaceFlinger·표시 경로로 나눠 관찰한다. Android 에뮬레이터는 대조 실험과 도구 검증에만 사용한다. 이 계약과 실행 기록은 Spinon GPU 렌더러 성능이나 실기기 성능을 입증하지 않는다.

`AppDeadlineMissed`는 앱 프레임이 주어진 예산 안에 제출되지 않았음을 나타내며 Java, Rust, V8, RenderThread, GPU 중 어느 제품 코드가 원인인지 단독으로 판별하지 않는다. 해당 프레임의 앱 스레드 실행·대기 구간과 아래의 앱 추적 구간을 함께 대조한다. SurfaceFlinger·DisplayHAL 지연은 앱 프레임 지연과 분리해 기록한다. `BufferStuffing`은 제출 프레임이 표시보다 앞서 대기하는 상태이며, 앱 프레임 실행이 매번 예산을 넘었다는 뜻과 같지 않다. [Perfetto FrameTimeline 정의](https://perfetto.dev/docs/data-sources/frametimeline)

Perfetto `actual_frame_timeline_slice`에는 surface frame과 display frame 행이 모두 들어갈 수 있고 여러 행이 같은 display token을 가리킬 수 있다. 따라서 MainActivity의 앱 프레임·App Deadline Missed는 각각 고유 `surface_frame_token`으로 세며, Dropped Frame과 SurfaceFlinger deadline은 고유 `display_frame_token`으로 센다. 행 합계를 프레임 수로 사용하지 않는다. 앱 surface frame 수는 trace 동안의 모든 display vsync 수가 아니다. 진단 보고서에는 실제/예상 앱 token 집합의 차이와 앱 miss row/token 중복 여부를 함께 남긴다.

## 2. 귀속 단계

| 단계 | 관찰 대상 | 판정에 필요한 근거 |
| --- | --- | --- |
| Android 입력·앱 UI | 입력 전달, Main/UI thread 작업·대기, View 변경 | Perfetto input/view/sched 구간, 대조 화면의 동일 입력 |
| Spinon 경계 | 플랫폼 호출 worker와 진단 로그 큐·화면 갱신 | `SpinonR05:*` 앱 trace section, 입력 시각과 실행 thread |
| Android 앱 프레임 | 앱의 `doFrame`, RenderThread, GPU 완료·제출 | FrameTimeline 앱 예상/실제 timeline과 스레드 상태 |
| Android 표시 경로 | SurfaceFlinger 합성·표시 deadline | FrameTimeline의 SurfaceFlinger/DisplayHAL jank 분류 |

앱 프레임에 지연이 있다는 사실만으로 “Spinon Rust/V8 병목” 또는 “Android OS 병목”이라고 결론내리지 않는다. 앱 내부 원인 표시는 frame token에 대응하는 앱 trace section이나 스레드 상태가 있을 때만 한다. Perfetto의 FrameTimeline 분류만으로 앱 내부 함수 원인을 알 수 없다.

## 3. Android 대조 조건

`FrameAttributionActivity`는 동일 View 계층, 같은 버튼·상태 문구·ScrollView를 유지하고 탭 처리만 바꾼다.

| 모드 | 입력 | 앱 화면 변경 | 목적 |
| --- | --- | --- | --- |
| `idle` | 없음 | 없음 | 같은 화면을 띄운 유휴 기준선 |
| `input-only` | 버튼 탭 | 없음 | 입력·버튼 피드백 비용 |
| `status` | 버튼 탭 | 상태 TextView만 변경 | Android View 텍스트 변경 비용 |
| `log-scroll` | 버튼 탭 | 로그 TextView 추가 후 아래로 이동 | 진단 로그·동적 텍스트·ScrollView 비용 |
| `spinon-event` | 런타임 화면 버튼 탭 | 실제 V8 이벤트; 진단 모드에서는 상태 TextView·로그 뷰 변경 생략 | 같은 MainActivity에서 runtime 경로 관찰 |
| `spinon-ui-only` | Spinon 화면 버튼 탭 | 상태 TextView·로그 뷰 변경 생략, runtime dispatch 생략 | 같은 탭 경로의 UI-only 대조군 |
| `spinon-long-js` | 무한 JS 실행 중 입력·취소 | heartbeat, 대기 이벤트, 취소 완료 표시 | 응답성·취소 진단; 프레임 성능 비교에는 사용하지 않음 |

처음 네 모드는 Android View의 대조 실험이다. `spinon-ui-only`와 `spinon-event`는 같은 MainActivity·버튼을 쓰며 runtime 비교 계측 모드에서는 양쪽 모두 탭 상태 TextView와 로그 뷰를 변경하지 않는다. dispatch 보고서는 Logcat과 Perfetto에 남기고 Logcat 출력은 trace 종료 뒤 수행한다. 이 비교는 지연 구간을 나누는 원인 분석이며 절대 성능 순위가 아니다. 탭 시작부터 UI callback까지의 비동기 시간에는 Android main-thread handoff가 들어가므로 JS/runtime 실행 시간과 동일시하지 않는다.

## 4. 수집 범위와 추적 표식

[`tools/benchmark/android-frame-attribution.textproto`](../../tools/benchmark/android-frame-attribution.textproto)는 Android FrameTimeline, `sched_switch`/`sched_wakeup`, CPU 주파수·유휴 상태, `view`·`input`·`wm` ATrace와 Spinon 앱 구간을 요청한다. 상세 driver 표식을 조사할 때는 [`tools/benchmark/android-frame-attribution-detailed.textproto`](../../tools/benchmark/android-frame-attribution-detailed.textproto)를 사용한다. 이 설정은 `android.surfaceflinger.frame`도 요청하며 수집 스크립트의 `SPINON_ANDROID_TRACE_CONFIG` 환경변수로 선택한다. 2026-10-07 API 36 AVD trace의 `ftrace_setup_errors=37`은 상세 설정에서 `gfx` 범주를 추가할 때만 발생했다. 두 설정의 차이를 한 줄로 제한한 5초 대조에서 `gfx` 제외는 0건, 포함은 `Ftrace event unknown` 37건이었다. 오류들은 DPU/G2D/Mali/MDSS/panel/SDE vendor event로, 해당 Emulator 커널이 제공하지 않았다. 두 대조의 import log에는 `Ftrace event failed`나 ATrace 실패가 없었고 양쪽에서 `sched_slice`가 기록됐다. packet loss 0은 별도 원본 40초 상세 trace에서 확인했다. 그 원본에는 FrameTimeline과 앱 표식도 있었다. 따라서 공통 scheduler·프레임 경계는 분석할 수 있으나 vendor GPU/display driver 내부까지 계측한 것은 아니다. 상세 결과는 [Perfetto `gfx` 설정 오류 원인 분석](evidence/r05-perfetto-gfx-setup-errors-2026-10-07.md)에 기록한다. 구현된 앱 구간은 다음과 같다.

- `SpinonR05:control-input`, `control-status-update`, `control-log-append`, `control-log-scroll`
- `SpinonR05:spinon-event-submit`, `spinon-event-status-update`, `runtime-dispatch`
- `SpinonR05:runtime-worker-call`, `native-session-dispatch`, `native-session-ffi`, `native-session-report`, `native-session-logcat`, `native-session-byte-array`, `reply-receive`, `reply-send`, `reply-handoff`
- `SpinonR05:runtime-result-processing`, `runtime-main-thread-post`, `runtime-main-thread-handoff`, `runtime-main-thread-completion`
- `SpinonR05:v8-handler-call`, `node-callback`, `document-commit-bridge`, `document-commit-callback`, `text-callback`, `v8-microtasks`, `document-safe-point`
- `SpinonR05:runtime-log-enqueue`, `runtime-log-flush`, `runtime-log-scroll`
- `SpinonR05ControlInputCount`, `SpinonR05SpinonInputCount`, `SpinonR05RuntimeDispatchCount` counter는 각각 대조 화면 handler 도달, Spinon 입력 handler 도달, Android runtime worker `ExecutorService`의 dispatch 작업 접수 수를 기록한다. 마지막 counter는 Rust scheduler 접수나 V8 성공을 증명하지 않으므로 실제 처리 결과는 sequence와 런타임 응답의 status로 확인한다. ADB 탭 명령 수와 같다고 가정하지 않고 실행별 counter를 확인한다.

`reply-handoff`는 Android `ATrace_beginAsyncSection`으로 actor의 응답 전송 직전에 시작하고 caller가 채널 응답을 받은 직후 끝내는 교차 스레드 wall 구간이다. 프로세스 전역 32비트 `trace_cookie`를 사용하며, 같은 값은 런타임 보고서와 trace의 async slice에 기록된다. cookie는 ATrace API 폭에 맞춘 `u32` 증가자로, 프로세스에서 2^32회 할당된 뒤 재사용될 수 있다. 이미 닫힌 cookie의 재사용은 추적에 영향을 주지 않으며, 이론상 wrap 시점에 같은 cookie를 가진 async span이 동시에 열려 있으면 ID 충돌이 가능하다. trace의 signed 값은 요약기에서 같은 32비트 값으로 정규화한다. 채널 전송이 실패하면 보낸 쪽에서 구간을 닫고, 응답을 받지 못한 호출에서는 닫힌 응답 구간을 만들지 않는다. 이 구간은 전송·깨우기·caller 재개까지의 wall 경과이며 CPU 실행시간이나 `SyncSender::send` 단독 실행시간이 아니다. `response_wait_us` 전체를 대신하지 않는다.

런타임 보고서의 시간 단위는 마이크로초다. `v8_call_us`는 thread CPU 시간이 아니라 C++ V8 호출의 wall time이며, CPU 실행·대기 상태는 같은 Perfetto trace의 scheduler data로 별도 확인한다.

| 필드 | 시작·끝 | 포함되는 작업 |
| --- | --- | --- |
| `submission_lock_wait_us` | 세션 제출 mutex lock 호출부터 획득까지 | 다른 제출자가 보유한 mutex 대기 |
| `control_lock_wait_us` | 세션 제어 mutex lock 호출부터 획득까지 | 종료·취소 제어 상태 mutex 대기 |
| `scheduler_lock_wait_us` | scheduler mutex lock 호출부터 획득까지 | 큐 상태 mutex 대기 |
| `queue_residence_us` | scheduler 잠금·용량 검사를 지난 뒤 큐 삽입 직전부터 worker가 꺼낼 때까지 | 실제 큐 체류와 worker wake/scheduling 지연 |
| `v8_call_us` | C++ eval/dispatch 진입부터 반환까지 | JS callback, V8 microtask checkpoint, safe-point collection을 포함한 wall time |
| `post_v8_us`, `report_build_us` | V8 반환 뒤 post 처리와 보고서 생성 | Rust 통계 수집·보고 문자열 생성; dispatch 보고서에 기록 |
| `report_finalize_us` | 기본 보고 문자열 뒤 계측 suffix 생성 | `post_v8_us`, `report_build_us`의 문자열 추가 작업 |
| `actor_before_reply_us` | scheduler worker가 명령을 꺼낸 뒤 응답을 채널로 보내기 직전까지 | begin-execution 확인, V8 호출, post 처리, 보고서 생성·마무리의 actor wall time; queue 체류와 caller 응답 handoff는 제외 |
| `response_wait_us` | 큐 삽입이 끝난 뒤 caller가 응답을 받기까지 | 큐 체류, 런타임 처리, 응답 handoff |
| `submit_total_us` | Rust submit 진입부터 응답 수신까지 | submit lock 검사부터 응답 수신까지의 caller wall time |

구버전 로그의 `queue_wait_us`는 제출 시각부터 worker 수신까지 측정해 scheduler mutex 대기를 포함할 수 있다. 새 `queue_residence_us`와 같은 지표로 합산하거나 직접 비교하지 않는다. 이 표식은 앱 함수 범위만 보여 준다. V8 내부 함수별 시간, Rust 레이아웃 구간, GPU 명령 실행시간은 미계측이며 GPU vendor counter와 실기기 thermal/power 계측도 포함하지 않는다.

`native-session-ffi`는 호출 Java thread에서 FFI 제출부터 동기 응답까지의 wall 구간이며 실행 CPU 시간으로 해석하지 않는다. `response_wait_us`도 runtime 실행뿐 아니라 응답 뒤 caller가 다시 스케줄되기까지의 지연을 포함할 수 있다. Perfetto에서 `linux.ftrace`를 사용할 수 있으면 caller와 V8 owner 각각의 `sched_wakeup`→`sched_switch`를 대조한다. Android 기기에서 `linux.ftrace`가 등록되지 않았지만 `simpleperf trace-offcpu`가 제공되면 debuggable 앱에 한정한 대체 진단을 할 수 있다. 이 대체 자료는 owner TID의 schedule-out→schedule-in 경과를 보여 주지만 `sched_wakeup` 원인이나 thread 이전 상태를 복원하지 않으므로 Perfetto ftrace와 동일한 세부 귀속으로 취급하지 않는다. `/proc/<pid>/task/<tid>/schedstat` 전후 카운터를 읽을 수 있으면 CPU 시간·runqueue 누적 대기를 독립적으로 교차 확인할 수 있지만, 개별 스케줄 전환 시점과 대기 원인을 판별하는 자료는 아니다. 2026-10-08 Android 실기기 1회에서 8.8초 동안 CPU 시간 +8.711520849초, runqueue 대기 +0.228829ms를 확인했다. 내부 V8 구간 표식은 Android ATrace의 wall duration이며 V8 GC 내부 단계나 CPU 실행시간을 대신하지 않는다. `queue_residence_us`는 큐 삽입부터 worker 수신까지로, OS wake/scheduling 대기도 포함한다.

2026-10-08 Android 16/API 36 실기기에서는 Perfetto `linux.ftrace`가 없었지만 shell의 Android `atrace sched` category가 `sched_waking`, `sched_wakeup`, `sched_switch`를 기록했다. simpleperf 앱 범위 tracepoint는 0 samples였고 system-wide tracepoint 기록은 커널 sample 권한으로 거부됐다. ATrace에서 wake 이벤트 payload의 pid(대상), 이벤트 헤더의 task/TID(요청 주체), `sched_switch`의 `next_pid`와 이전 상태를 연결하면 직접 wake 요청자와 첫 재실행을 구분할 수 있다. 앱 함수 표식까지 맞출 때는 `-a dev.spinon.bootstrap`을 붙인다. 이 대체 경로는 Android 실기기 한 대의 debug 진단으로만 검증했다. [Linux event tracing](https://docs.kernel.org/trace/events.html) · [wake-to-switch latency 계산 예시](https://docs.kernel.org/next/trace/histogram.html) · [실제 결과](evidence/r06-android-physical-scheduler-2026-10-08.md#android-atrace-직접-wake-원인).


`actor_before_reply_us`도 wall 시간이며 worker thread CPU 사용량은 아니다. 이 값은 worker가 명령을 처리하는 동안 멈춰 스케줄되지 않은 시간도 포함할 수 있다. 따라서 긴 값은 V8·Rust 함수의 CPU 병목을 단독으로 증명하지 않으며 Perfetto thread state와 함께 분류한다. 호출별 보고서와 `runtime-worker-call`은 report의 `trace_cookie`와 `reply-handoff` slice의 cookie·시간 포함 관계로 연결하고, caller/owner TID는 연결된 보고서에서 가져와 thread state를 분류한다. report와 slice의 누락·중복·잘린 구간은 품질 오류로 센다. 프레임 비교 fixture는 dispatch 결과를 UI에 추가하지 않고 report Logcat 기록을 trace 종료 후 수행한다.

`reply-send`는 actor가 응답 채널을 보내는 같은 스레드의 Android ATrace wall 구간이다. actor thread가 표식 안에서 선점되면 선점 대기까지 길이에 포함되므로 `SyncSender::send`의 실행시간으로 읽지 않는다. `reply-handoff`는 별도 async slice로 actor send 직전부터 caller receive 직후까지를 연결한다. 두 구간이 다르면 actor의 send 반환 뒤 선점과 caller 재개 경계를 의심할 수 있지만, 어느 구간도 CPU 실행시간을 단독으로 증명하지 않는다. caller의 `response_wait_us`와 실제 caller/owner `sched_wakeup`·`sched_switch`·thread state를 같이 확인한다. `runOnUiThread` 게시부터 main callback 시작까지도 별도 Android UI 대기 구간으로 보고 runtime 처리시간에 더하거나 빼지 않는다.

`tools/benchmark/capture-android-frame-attribution.sh`는 기기·OS·화면 크기·밀도·열 상태·앱 APK digest·커밋·캡처 시점 작업 트리 digest를 기록하고 원본 trace, `gfxinfo framestats`, UI hierarchy·화면, Logcat을 지정한 출력 디렉터리에 저장한다. `diagnostic_ui_mutations_suppressed=true`는 paired runtime 조건에서 상태 TextView와 로그 뷰 변경을 양쪽 모두 생략했다는 뜻이다. 작업 트리 digest는 캡처 시점의 코드 상태를 식별하며 APK 빌드 입력의 provenance를 증명하지 않는다. 빌드 바이너리 식별자는 별도 APK SHA-256이다. `tools/benchmark/run-android-frame-attribution-matrix.sh`는 네 Android View 조건을 실행 순서별 seed로 섞는다. `tools/benchmark/run-android-ui-runtime-attribution-matrix.sh`와 `tools/benchmark/run-android-long-js-frame-attribution-matrix.sh`는 paired condition의 첫 조건을 seed로 선택하고 회차마다 순서를 교대해 반복 횟수가 짝수면 각 순서가 동일한 횟수가 되도록 한다. seed와 실제 실행 순서를 저장한다. 원본과 함께 전달되지 않은 표·요약은 재현 근거가 아니다.

`tools/benchmark/capture-android-simpleperf-offcpu.sh`는 Android API 27 이상에서 debuggable 앱의 `simpleperf --app ... --trace-offcpu`를 실행하고 owner TID, 기록 표본·유실 수, 선택적으로 읽힌 `schedstat` 카운터를 저장한다. 실행 길이·빈도·반복 수는 범위를 검사하고 시작·취소 UI 상태와 V8 취소 반환을 확인한다. 기기에서 가져온 profile의 SHA-256을 호스트 복사본과 비교하며 불일치는 실패로 처리한다. 기기 임시 profile은 성공한 회차마다 삭제하고, 중간 실패 때도 종료 처리에서 삭제를 시도한다. 이 수집기는 owner의 off-CPU 구간을 계측하며 개별 wake 요청자·화면 프레임 원인을 대신 귀속하지 않는다.

원본 trace에서 marker·counter·FrameTimeline을 집계하는 SQL은 [`summarize-android-frame-attribution.sql`](../../tools/benchmark/summarize-android-frame-attribution.sql)이다. 개발자는 선택적으로 [공식 Perfetto Trace Processor](https://perfetto.dev/docs/getting-started/command-line-analysis)를 설치해 `trace_processor query -f tools/benchmark/summarize-android-frame-attribution.sql <frame-attribution.pftrace>`를 실행할 수 있다. Trace Processor는 Spinon 빌드/런타임 의존성이 아니다.

UI/runtime paired matrix 요약기는 Trace Processor와 Bun을 사용한다. `SPINON_TRACE_PROCESSOR`에 실행 경로를 지정한 뒤 `bun tools/benchmark/summarize-android-ui-runtime-attribution.mjs <matrix 디렉터리>`를 실행한다. 이 요약기는 `schedule.txt`의 탭 수와 균형 순서를 검증하고, 실행별 counter·trace 품질·FrameTimeline·main thread 상태·dispatch 내부 구간·Rust actor 시간·caller/owner 스케줄 wake 지연·main callback handoff를 함께 집계한다. report의 caller TID와 async `reply-handoff` cookie·시간 범위로 worker 호출을 연결한다. 보고서와 span의 1:1 연결 실패, 필수 보고서 필드 누락, UI-only 대조의 응답 span, 입력·counter 불일치, 잘린 프레임 token, trace 품질 오류, 잘못된 실행 순서는 요약에 표시하고 종료 코드 1을 반환한다. 정상 종료는 계측 자료의 구조적 일관성만 뜻하며 성능 합격 판정은 하지 않는다.

## 5. 실행·해석 규칙

1. 도구 확인 단계는 Android 12/API 31 이상 에뮬레이터에서 먼저 한다. 에뮬레이터 결과는 UI 재현과 trace 파이프라인 검증이며 성능 순위·실기기 성능 주장을 지원하지 않는다.
2. 입력 비교에서는 같은 APK·기기·OS·화면 모드·조건 순서·입력 횟수를 유지한다. 기기 정보·실제 refresh mode·전경 상태·열 상태를 실행마다 저장한다. 화면 refresh가 고정되지 않으면 frame deadline과 refresh 변화를 그대로 기록한다.
3. 개발 원인 분석의 기본 표본은 조건마다 독립 실행 10회, 회당 기본 20초 trace와 10회 탭이다. 캡처 전 같은 화면을 5초 안정화하고, paired condition 순서는 seed로 시작 조건을 정한 뒤 회차마다 교대한다. 스크립트 기본값은 탐색용이며 benchmark 합격 기준이 아니다.
4. 탭 좌표가 UI hierarchy에서 발견되지 않거나 비활성 상태이면 회차를 시작하지 않는다. 측정 중 앱이 전경을 잃거나 Perfetto가 실패·유실을 보고한 회차는 제외 이유를 남기고 다시 실행한다. 느린 프레임, 긴 꼬리, 앱 deadline miss는 이상치라는 이유로 제외하지 않는다.
5. 소수 탭의 표본은 지연 원인 위치를 찾는 데만 쓴다. p95/p99를 보고할 때는 실제 관측 frame/action 표본 수, 실행별 분포와 불확실성을 함께 제공한다. 표본이 적으면 원시 개수와 개별 시각만 보고하고 백분위·우열을 주장하지 않는다.
6. 에뮬레이터와 실기기, debug와 release, 서로 다른 refresh mode를 한 통계 집합으로 합치지 않는다. 제품 성능 비교는 같은 실기기에서 동일한 기능 시나리오를 지원하는 release 빌드로 수행하고, 실행 순서를 번갈아 기록한다. 계측 trace를 켠 원인 분석 수치와 trace를 끈 성능 수치를 구분한다.
7. 첫 제품 성능 결론 전 trace 사용/미사용 대조로 계측 오버헤드를 확인한다. 2026-10-08 시뮬레이터 debug 대조는 실행했지만 분포가 0을 가로질러 오버헤드 크기는 미확정이다. 실기기·release 대조는 남아 있다. 버튼별 앱 trace marker가 없는 구간은 소스 귀속을 주장하지 않는다.

## 6. 구현 현황과 미완료 범위

Android 대조 Activity, 앱 trace section, Perfetto 수집기는 Android 16/API 36 ARM64 에뮬레이터에서 도구 검증한다. iOS에서는 Simulator 전용으로 runtime queue·FFI·V8·main callback·다음 `CADisplayLink` tick을 기록하는 32회 진단을 추가했다. Android에서는 기본 `runOnUiThread`와 `Handler.createAsync`의 25탭씩 개입 대조로 반복 지연에서 sync barrier의 영향을 확인했다. 추가로 동기 handoff 60회, asynchronous Handler 60회, cold process의 첫 asynchronous callback 20회와 runtime dispatch를 생략한 같은 화면 60탭을 수집했다. UI-only에서는 runtime dispatch counter가 0인데도 두 frame이 App Deadline Missed/Late Present였고, 첫 frame의 RenderThread 실행은 28.705ms였다. 상세 `gfx` trace에서는 runtime event와 UI-only 각각 60탭을 더 기록해 EGL swap 경계·VSync·SurfaceFlinger frame을 관찰했다. 따라서 runtime callback 지연과 Android View/RenderThread frame 지연은 별도 귀속한다. [2026-10-07 교차 플랫폼 원인 분석](evidence/r05-cross-platform-callback-root-cause-2026-10-07.md)에 실행별 결과와 미측정 항목, 측정 실패 이유, 필요한 후속 계측을 기록한다. Android의 원래 72ms 다음 `doFrame` 공백, 과거 27.877ms asynchronous first-dispatch outlier, RenderThread 내부 native 함수, host QEMU scheduling은 아직 확정하지 않았다. Composer transaction code 5는 Android 16 Composer AIDL V4 메서드 순서 대조로 `IComposerClient.executeCommands`에 매핑했지만 trace에서 Binder interface descriptor는 직접 확인하지 않았다. Android simpleperf CPU sampling은 emulator에서 `cpu-cycles`와 `cpu-clock` event를 사용할 수 없어 profile을 만들지 못했고 Perfetto stack sample은 0개였다. 상세 trace 두 건의 packet loss는 0이며 37개 `ftrace_setup_errors`가 모두 `gfx` 범주의 비가용 vendor driver event임을 최소차이 대조로 확인했다. AVD의 `gfx` 제외 대조에서도 scheduler slice는 기록됐지만, 그 짧은 무부하 실행은 프레임 표식 동등성을 검증하지 않았다. 별도 원본 40초 상세 trace에는 FrameTimeline·앱 표식이 있었으며 vendor GPU/display driver 내부 계측은 아니다. 2026-10-08 Android 16/API 36 ARM64 `SM-S731N` 실기기에서는 Perfetto query에 `linux.ftrace`가 없었지만 `simpleperf --app ... --trace-offcpu`는 동작했다. 실제 V8 긴 동기 JS 조건 5회에서 owner TID의 101개 짝지은 off-CPU 구간은 p50 15.117µs, p95 69.102µs, 최대 730.976µs였다. 같은 조건의 JS dispatch queue residence p50 1,100,059µs보다 짧아 해당 큐 지연의 주원인으로 OS 재스케줄 대기를 지목하지 않는다. 이는 Android 실기기 한 대의 디버그 원인 분석이며 runnable wait 세부 상태·release overhead·GPU frame 원인은 확정하지 않는다. 전체 실행 근거는 [Android 실기기 JavaScript owner 스케줄링 계측](evidence/r06-android-physical-scheduler-2026-10-08.md)에 둔다. iOS Instruments System Trace 시도는 기록 시간이 app probe와 겹치지 않았고 `Animation Hitches` template은 Simulator에서 지원되지 않아 iOS OS-level frame 귀속은 미완료다. `R05` 완료에는 Android·iOS 실기기 foreground 측정, 입력 성공과 화면 표시 결과 검사, 표시 시각·오차·제외 기준, 제품 release 계측 비교, 같은 기능을 지원하는 비교 대상의 실제 측정이 더 필요하다. 이 문서만으로 R05를 완료 처리하지 않는다.

2026-10-07 후속 반복에서는 Android API 36 ARM64 에뮬레이터에서 `spinon-event`와 `spinon-ui-only`를 10쌍으로 교대했다. 각 회차는 20초 trace와 세 번 입력을 사용했다. 30개 runtime report·`reply-handoff` span은 cookie가 모두 연결됐고 UI-only 30회에는 dispatch/span이 없었다. handoff 최대는 411.541µs였으며 앞선 63.009ms handoff는 재현되지 않았다. 별도 82.367ms UI callback tail은 main thread sleep 79.895ms와 겹쳤지만 에뮬레이터·host scheduler·OS 중 원인은 미확정이다. 회차당 입력 수는 계약의 10회보다 적으므로 탐색 반복 근거이며 기준 행렬을 대체하지 않는다. [반복 실행 근거](evidence/r05-reply-handoff-repeat-2026-10-07.md).

2026-10-08에는 Android API 36 ARM64 에뮬레이터에서 Perfetto session on/off를, iPhone 17 Pro / iOS 26.2 Simulator에서 Time Profiler on/off를 각각 10쌍 비교했다. Android는 쌍마다 10개 dispatch report·순번·APK·입력 좌표를 맞췄고, trace-on 원본 10개는 Perfetto v58.2로 data loss·error 0, 필수 앱 marker/counter, FrameTimeline token을 확인했다. iOS는 32개 sample status/PID/sequence와 전체 sample의 실제 xctrace 시간 중첩을 10개 trace에서 확인했다. 모든 쌍 차이 범위가 0을 가로질러 overhead 크기는 미확정이다. 이는 debug 시뮬레이터 계측 진단이며 물리 입력·실기기·release·실제 pixel present를 입증하지 않는다. 상세 결과·digest·원본 checksum은 [계측 오버헤드 시뮬레이터 실행 증거](evidence/r05-trace-overhead-simulators-2026-10-08.md)와 [checksum manifest](evidence/r05-trace-overhead-checksums-2026-10-08.sha256)에 기록한다.

## 7. R05.3 입력 이벤트와 actual-present 상관

기존 Android `View.onClick`, ADB tap, iOS selector/`CADisplayLink` 표본은 물리 입력 event timestamp와 GPU frame 표시 완료를 연결하지 않는다. `CADisplayLink` 또는 예상 presentation 시각을 actual present로 대체하지 않는다. 다음 계측 단계는 입력 출처·clock domain, 고유 render revision, exact surface/frame identity와 플랫폼 표시 신호를 한 표본으로 연결해야 한다. Android API 36의 현재 `SurfaceView` 경로에서는 FrameTimeline 지원을 가정할 수 없고 direct present timestamp가 없으면 미지원으로 남긴다. clock 변환이나 frame match를 입증할 수 없으면 event-to-present 지연을 산출하지 않는다. OS present 신호는 광학적 픽셀 발광을 증명하지 않는다. R05.3의 시뮬레이터 사전 기준·실행 gate·제외 규칙은 [입력→표시 계측 계획](../../plan/r05-input-to-presentation.md)을 따른다. 이 PR에서는 구현·시뮬레이터 검증을 수행하지 않으며, 실기기 실행은 사용자의 명시적 요청 뒤에만 진행한다.
