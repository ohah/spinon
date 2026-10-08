# R05.3 Android 트랜잭션 표시 fence 검증 계획

**상태:** Android API 36·37.2 네 조합의 3×10 반복 측정 완료 · usable fence 119/120 · 구현 실패 관점 검토 완료 · iOS와 기기 일반화는 미완료 · R05.3 미완료

## 목표

Android API 37.2 `SurfaceControl.JankData.getPresentTimeNanos()`는 현재 AVD의 WGPU·GLES 표본 60개에서 모두 unknown이었다. 같은 출력 `SurfaceView` frame에 묶은 `SurfaceControl.Transaction`의 완료 callback과 `TransactionStats.getPresentFence().getSignalTime()`이 표시 시각을 제공하는지 별도로 확인한다.

사전 확인에서 API 36·37.2 AVD 모두 `service.sf.present_timestamp=0`이었다. Android Open Source Project의 SurfaceFlinger는 이 property를 `mHasReliablePresentFences` 값으로 설정한다. 이 값은 원인 후보를 가리키지만 `JankData`의 unknown 값을 직접 설명한다고 단정하지 않는다. 공개 `getPresentFence()` 문서도 기기가 present fence를 지원하지 않을 때 빈 fence를 반환한다고 한다. property는 읽기만 하며 시험 목적으로 1로 덮어쓰지 않는다. `addTransactionCompletedListener`와 관련 API는 AOSP에서 `sdk_desired_present_time` flag가 꺼진 경우 등록을 건너뛰는 `@FlaggedApi`이므로 runtime callback 수신 여부도 별도 gate다.

공식 계약상 `addTransactionCommittedListener`는 업데이트가 표시 준비된 때이고, `addTransactionCompletedListener`는 transaction이 표시된 때다. `getLatchTimeNanos()`는 framework가 frame을 latch해 표시 대기열에 넣은 시각이라 표시 시각으로 대체하지 않는다. `getPresentFence()`가 유효하지 않거나 signal time이 invalid/pending이면 해당 sample은 미측정으로 남긴다.

## 비교 모델과 환경

| 대상 | 고정 조건 | 판정 자료 |
|---|---|---|
| Android API 37.2 WGPU/Vulkan | 기존 `emulator-5562`, 1080×1920, 60 Hz, on-demand `SurfaceView`, Goldfish GFXStream/llvmpipe | transaction request ID·FrameTimeline VSync ID·JankData exact join·present fence·revision |
| Android API 37.2 GLES 2.0 | 동일 AVD·화면·입력 절차, ANGLE/Vulkan SwiftShader | 같은 항목. GPU stack이 달라 속도 순위는 만들지 않음 |
| Android API 36 WGPU/Vulkan | 기존 `emulator-5554`, 화면·입력 절차 동일, 해당 API에서 제공되는 공개 transaction API | request ID에 닫힌 단일 transaction callback·fence·revision; JankData 미지원과 분리 |
| Android API 36 GLES 2.0 | 동일 API 36 AVD | API 36 WGPU와 같은 판정. GLES draw 요청·완료 로그와 transaction callback을 배열 순서로 연결하지 않음 |

먼저 API 36·37.2 각 AVD에서 renderer별 단일 진단 입력을 보내 transaction completed callback, fence 유효성·signal time, API feature-flag 동작을 확인한다. 어느 조합에서든 유효한 양수 signal time과 request→frame 결속 조건이 확인된 경우에만 그 조합의 반복 block으로 진행한다. 신호가 지원되지 않거나 callback이 등록되지 않으면 해당 조합의 3×10 반복은 생략하고 capability 실패를 기록한다. capability를 통과한 renderer·API 조합은 새 앱 프로세스의 독립 block 3개, block마다 점수 입력 10개와 별도 1개 unscored drain 입력으로 실행한다. block 합격은 점수 입력 각각에 request-keyed transaction callback·유효한 양수 monotonic signal·현 surface generation·단일 draw를 요구한다. API 37.2는 그 입력과 같은 target VSync ID의 JankData exact join까지 요구한다. clock offset anchor 구간이 겹치지 않거나 앱이 sleep/background에 들어간 입력은 지연 계산에서 제외하되 원시 결과에는 남긴다. 누락·중복·pending은 실패 수로 보존한다. 장치별 30개 결과는 계측 capability와 고정 AVD fixture만 말하며 renderer 성능 비교·실기기 일반화·광학 scanout으로 해석하지 않는다.

## 구현 절차

1. 입력 `MotionEvent.getEventTimeNanos()`에서 sequence·revision·surface generation을 고정하고 synthetic 입력과 실제 touch를 구분한다.
2. FrameTimeline join 실험과 같은 on-demand 조건에서 preferred VSync ID를 고른다. 각 입력은 VSync callback 전후로 competing draw가 없어야 한다.
3. 하나의 `SurfaceControl.Transaction`에 `addTransactionCompletedListener`를 먼저 등록하고, API 37.2에서는 같은 transaction에 `setFrameTimeline(vsyncId)`를 설정한다.
4. 해당 transaction을 `SurfaceView.applyTransactionToFrame()`에 넘긴 뒤 한 번만 draw한다. `GLSurfaceView`는 `RENDERMODE_WHEN_DIRTY`를 유지하며 `requestRender()`와 실제 `onDrawFrame`을 별도 로그로 남긴다.
5. 완료 callback closure에는 그 입력의 `request_id`, `input_seq`, revision, generation 및 target VSync ID를 보존한다. callback 배열·도착 순서로 request를 추정하지 않는다. API 37.2 JankData가 있으면 ID가 target과 일치하는지도 독립 대조한다.
6. callback에서 `TransactionStats.getPresentFence()`를 얻고 `isValid()`와 `getSignalTime()`을 기록한다. 양수 signal time이 callback 안에서 읽은 `System.nanoTime()`보다 미래이거나 latch time보다 앞서지 않으면 해당 fence를 정상 signal로 인정하지 않는다. API가 제공한 fence는 모든 제어 흐름에서 `close()`한다. listener 경계에서 예외를 격리해 밖으로 던지지 않으며, bounded executor가 거부한 경우에도 framework 전달 command를 실행해 `TransactionStats` cleanup이 건너뛰어지지 않게 한다. callback에서 `await`, polling, main/UI thread 대기를 하지 않는다.
7. `getLatchTimeNanos()`는 진단 값으로만 기록한다. committed callback·callback uptime·deadline·expected presentation time·GPU command 완료·draw 완료를 presentation time으로 바꾸지 않는다.
8. MotionEvent의 event time과 함께 입력 시점의 uptime 및 `System.nanoTime()` bracket을 기록한다. callback에서도 같은 bracket을 기록해 uptime−monotonic offset interval의 겹침을 확인한다. Android의 공식 clock 설명을 기준으로 변환이 확인되고 기기 sleep이 개입하지 않은 입력만 event→OS-present 후보 구간을 계산한다. 겹침이 없거나 앱이 background/화면 sleep을 거친 입력은 지연 계산에서 제외하되 원시 표본은 보존한다.
9. API 37.2와 API 36 각각 WGPU·GLES에서 3×10개 scored sample을 수집한다. 마지막 drain 입력은 callback 대기용이며 절대 latency 분포에 넣지 않는다.
10. API 36·37.2 AVD에서 `service.sf.present_timestamp`, 공개 SDK methods, API feature-flag에 따른 실제 등록 결과를 읽기 전용으로 보존한다. property를 바꾸지 않고 callback이 no-op인지 timeout인지 구분한다.
11. platform callback 누락·invalid/unsupported fence·pending fence·negative/zero signal·stale surface·generation 교체·timeout을 서로 다른 outcome으로 보존한다. 실패를 제출 시각이나 latch 시각으로 메우지 않는다.

## 판정과 중단 조건

- `addTransactionCompletedListener` callback이 왔어도 transaction을 특정 SurfaceView buffer frame과 연결할 수 없으면 입력 귀속에 쓰지 않는다. on-demand idle 조건과 `applyTransactionToFrame()`의 계약을 충족하고, 입력별 단일 draw가 확인된 표본만 검사한다.
- API 37.2에서 transaction fence signal time과 동일 request의 JankData VSync ID가 모두 확인되면 Android AVD frame/revision 연결 후보가 성립한다. API 36에서는 JankData ID가 없으므로 request별 transaction-to-frame 연결 계약만 별도 판정하고 37.2와 같은 증거 수준이라 합치지 않는다.
- valid fence가 unsupported/invalid/pending이거나 완료 callback이 누락되면 해당 조합은 timestamp capability 실패로 남긴다. latch 시각·callback 도착 시각·expected time으로 대체하지 않는다.
- API가 주는 transaction-present signal은 패널의 물리 scanout/광자 발광을 증명하지 않는다. 이 probe에서 사용자 체감 입력 지연이나 renderer 성능 순위를 결론 내리지 않는다.
- 시뮬레이터 결과만 낸다. 실기기 검증은 사용자가 별도로 요청하기 전에는 수행하지 않는다.
- iOS는 별도 gate다. Xcode 26.2 iPhoneOS SDK의 device target type-check는 통과했고 Simulator SDK type-check는 실패했다. device runtime은 아직 없으며 [iOS Metal 표시 feedback 계획](r05-ios-present-feedback.md)의 별도 결과를 기다린다. Android 결과를 iOS 결과로 대체하지 않는다.

## 공식 근거

- [`SurfaceControl.Transaction.addTransactionCompletedListener`](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer)): transaction이 표시된 때 한 번 callback을 보낸다.
- [`SurfaceControl.TransactionStats`](https://developer.android.com/reference/android/view/SurfaceControl.TransactionStats): `getPresentFence()`는 표시 시점을 알리는 fence를 돌려주며, `getLatchTimeNanos()`는 latch/표시 대기열 진입 시각이다.
- [`SyncFence.getSignalTime()`](https://developer.android.com/reference/android/hardware/SyncFence#getSignalTime): fence가 signal된 CLOCK_MONOTONIC 시각을 반환하며 invalid/pending sentinel을 정의한다.
- [`SurfaceView.applyTransactionToFrame()`](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction)): transaction을 다음 SurfaceView frame과 동기화한다. 다음 frame을 결정적으로 정하려면 호출 전 rendering이 멈춰 있어야 한다.
- [`MotionEvent.getEventTimeNanos()`](https://developer.android.com/reference/android/view/MotionEvent#getEventTimeNanos()): uptime time base의 nanosecond-precision 입력 시각이다.
- [`SystemClock`](https://developer.android.com/reference/android/os/SystemClock): uptime·monotonic clock의 정의와 sleep 영향.
- [AOSP SurfaceFlinger Android 16 QPR2](https://android.googlesource.com/platform/frameworks/native/+/refs/heads/android16-qpr2-release/services/surfaceflinger/SurfaceFlinger.cpp): `service.sf.present_timestamp`를 `mHasReliablePresentFences`에 따라 설정한다. 이는 현재 AVD의 property 관측에 대한 플랫폼 근거이며, `JankData` unknown 값의 단일 원인 확정은 아니다.
- [AOSP `SurfaceControl.java`](https://android.googlesource.com/platform/frameworks/base/+/a4dc88d7199beb4416663c581e070c08ff20ace1/core/java/android/view/SurfaceControl.java): `addTransactionCompletedListener`의 flag gate, executor 전달, callback 정상 반환 뒤 `TransactionStats` 정리를 확인한다.

## 계획 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 계획에서 고정한 대응 |
|---:|---|---|
| 1 | transaction commit callback을 화면 표시로 취급 | committed callback은 제외하고 completed callback만 사용한다. |
| 2 | GPU 작업 완료와 compositor 표시를 혼동 | transaction present fence만 표시 signal 후보로 기록한다. |
| 3 | latch 시각을 present 시각으로 대체 | `getLatchTimeNanos()`는 진단 열로 분리하고 latency 계산에서 금지한다. |
| 4 | `applyTransactionToFrame()` 전에 listener를 등록하지 않아 callback을 잃음 | 같은 transaction에 listener를 먼저 붙인 다음 SurfaceView에 넘긴다. |
| 5 | transaction이 의도한 입력 frame 아닌 다른 frame에 붙음 | idle SurfaceView·single draw 조건과 request/revision/generation을 검증한다. |
| 6 | continuous rendering에서 다음 frame이 비결정적 | GLES dirty render와 WGPU on-demand만 사용하고 continuous 표본은 no-go로 남긴다. |
| 7 | transaction을 넘긴 뒤 추가 draw가 경쟁 | 입력당 VSync callback·transaction·draw 하나만 허용하고 draw sequence를 검사한다. |
| 8 | callback 없음 또는 다음 frame 미제출을 성공으로 집계 | bounded timeout 뒤 pending/lost로 세고 표본을 실패 처리한다. |
| 9 | callback 도착 순서로 입력을 연결 | callback closure가 보존한 request ID로만 귀속한다. |
| 10 | API 37.2의 VSync ID가 다른 frame인데 timestamp만 연결 | 같은 request의 JankData VSync ID exact match를 독립 요구한다. |
| 11 | API 36에서 JankData 부재를 37.2와 동등하게 표현 | API 36 transaction callback 증거를 따로 표시하고 두 결과를 합치지 않는다. |
| 12 | empty/invalid fence를 빠른 0ms 표시로 해석 | `isValid`, `SIGNAL_TIME_INVALID`, `SIGNAL_TIME_PENDING`을 구분하고 invalid는 제외한다. |
| 13 | getSignalTime pending·미래 signal·latch보다 앞선 signal을 0ms 또는 정상 present로 처리 | pending·future·`latch_time > signal_time`을 별도 실패로 기록하며 기다리거나 대체 timestamp를 쓰지 않는다. |
| 14 | SyncFence native 자원 누수 | callback의 모든 정상·오류·예외 경로에서 소유한 fence를 닫는다. |
| 15 | UI/GL thread에서 fence 대기 또는 polling | `getSignalTime()`만 비동기로 확인하고 await·sleep·busy polling을 금한다. |
| 16 | executor가 callback을 거부하거나 handler가 예외를 던지면 framework가 소유한 `TransactionStats` 종료 경로도 실행되지 않을 수 있음 | AOSP wrapper가 `executor.execute(...)` 안에 callback과 `TransactionStats.close()`를 같이 넣는 것을 확인했다. bounded executor의 포화 시 전달 command를 즉시 실행하고, listener 경계에서 `RuntimeException`·`LinkageError`를 격리한다. inline overflow 표본은 점수에서 제외한다. |
| 17 | uptime과 CLOCK_MONOTONIC을 같은 값이라 근거 없이 가정 | 공식 clock 계약과 bracketed anchor를 검증하고 sleep이 낀 block은 무효화한다. |
| 18 | expected presentation time 또는 JankData timestamp unknown을 대체값으로 사용 | 실제 fence signal만 latency 후보로 쓰고 다른 값은 보조 관측치로 분리한다. |
| 19 | 해당 AVD가 reliable present fence를 지원하지 않는데 반복 입력으로 이를 감추거나 property를 강제 활성화 | `service.sf.present_timestamp`를 사전 기록하고 0이면 단일 capability probe만 실행한다. 플랫폼 property를 변경하지 않는다. |
| 20 | API method가 SDK stub에만 있고 runtime `sdk_desired_present_time` flag가 꺼져 callback이 no-op | compile-time signature가 아니라 runtime callback을 capability 증거로 삼고 flag/runtime 누락을 별도 결과로 남긴다. |

### 계획 검토 결과

계획은 20개 별도 실패 관점으로 검토했다. 각 항목을 Android API 35/36/37.2 SDK stub, 공개 reference, AOSP `SurfaceControl`/SurfaceFlinger source 및 현재 R08 호출 순서와 대조했다. SDK `javap`에서 `addTransactionCompletedListener`·`getPresentFence`·`SyncFence.getSignalTime`가 API 35/36/37.2 stubs에 있는 것을 확인했고, GLES `RENDERMODE_WHEN_DIRTY` 및 WGPU event-driven draw도 소스에서 확인했다. 검토 중 세 가지 전제를 좁혔다. (1) 두 AVD의 `service.sf.present_timestamp`가 0이고 AOSP가 이를 reliable present fence 여부에 따라 설정하므로, 반복 측정 전에 1회 capability probe를 둔다. (2) API signature만으로 flag-gated runtime callback을 보장하지 않으므로 callback 수신을 runtime gate로 삼는다. (3) transaction listener wrapper가 callback 정상 반환 후에만 stats close를 호출하므로, executor rejection/handler exception이 cleanup을 생략하지 않도록 inline fallback과 예외 격리를 계획에 넣었다. 이 세 발견 때문에 positive fence가 없는 AVD에서는 3×10 반복을 생략한다. 계획 검토 이후 구현은 시작하지 않았다. 구현 뒤에는 이 계획 검토와 다른 20개 실패 관점으로 구현을 다시 검토한다.

## 실행 결과

### 단일 capability probe · 2026-10-08

네 조합에서 각각 synthetic 중앙 tap 하나를 보냈다. 모두 transaction completed callback을 받았고 반환 fence는 valid·signaled·양수였다. 네 표본 모두 입력 시점과 callback 시점 uptime−monotonic offset bracket이 겹쳤다. Android API 37.2 WGPU·GLES 표본은 각각 target VSync ID `141549`·`143230`과 같은 JankData exact join을 기록했다. 두 exact JankData의 `presentTimeNanos`는 여전히 unknown(-1)이다. API 36은 API 37.2 JankData ID 경로가 없으므로 `applyTransactionToFrame`에 닫힌 transaction callback 경로만 따로 판정한다. 네 AVD/renderer 표본은 모두 `service.sf.present_timestamp=0`인 상태에서 얻었으며 이 property를 변경하지 않았다.

| API / renderer | callback / fence | input-to-frame key | offset bracket | 원본 |
|---|---|---|---|---|
| API 36 WGPU | callback 1, valid/signaled, signal `716097838572168` | request 1 / input 1 / generation 1; next SurfaceView frame | 겹침 | [로그](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api36-wgpu.log) · [화면](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api36-wgpu.png) |
| API 36 GLES | callback 1, valid/signaled, signal `716117315873886` | request 1 / input 1 / generation 1; next SurfaceView frame | 겹침 | [로그](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api36-gles.log) · [화면](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api36-gles.png) |
| API 37.2 WGPU | callback 1, valid/signaled, signal `6279767707286` | request 1 / input 1 / generation 1 / VSync `141549`; JankData exact | 겹침 | [로그](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api37_2-wgpu.log) · [화면](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api37_2-wgpu.png) |
| API 37.2 GLES | callback 1, valid/signaled, signal `6297397464502` | request 1 / input 1 / generation 1 / VSync `143230`; JankData exact | 겹침 | [로그](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api37_2-gles.log) · [화면](../spec/internal/evidence/r05-present-fence-capability-2026-10-08/android-api37_2-gles.png) |

단일 표본은 전체 matrix 결과가 아니며 성능 결론을 내리지 않는다. 네 조합 모두 양수 signal을 반환했으므로 계획의 capability gate에 따라 각 조합별 독립 process 3개 × 10 scored + 1 drain 입력을 실행한다. `fence_signal_usable`은 fence callback 자체의 상태일 뿐, JankData exact join·renderer draw·clock 변환까지 포함한 종합 합격 표시가 아니다. 반복 결과가 끝날 때까지 event→OS-present 통계를 완료로 표시하지 않는다.

## 계획 재검토 · positive-fence 경로 20개 독립 실패 관점

단일 probe 네 건과 새 구현의 request/fence/clock 경계를 대조해 계획의 positive-signal 분기를 다시 공격했다. 이전 20개 표를 재사용하지 않고 아래 새로운 실패 경계를 각각 확인했다. 검토 결과 합격 조건에서 API 37.2 per-input JankData exact join 및 clock offset bracket 겹침을 명시하고, `fence_signal_usable`을 종합 표본 합격과 구분했다.

| # | 새로운 실패 관점 | 대응 및 재검토 결과 |
|---:|---|---|
| 1 | 단일 positive 표본을 30개 반복 결과처럼 일반화 | capability와 matrix 판정을 분리하고 네 조합 모두 3개 block을 요구한다. |
| 2 | API 36 결과를 JankData exact join이라고 부름 | API 36은 next-frame transaction 귀속만 별도로 판정한다. |
| 3 | API 37.2에서 다른 VSync record를 fence 입력에 연결 | 입력별 target ID와 같은 VSync ID의 exact record를 모든 scored input에 요구한다. |
| 4 | fence callback의 `usable` 로그를 전체 sample 성공으로 오해 | `fence_signal_usable`은 signal 자체 상태로 제한하고 전체 합격은 offline join에서 판정한다. |
| 5 | transaction callback이 왔으나 해당 buffer draw가 없었음 | `draw_accepted` 또는 revision 일치 GLES draw 기록을 같이 요구한다. |
| 6 | 이전 buffer가 남은 상태에서 transaction이 완료됨 | on-demand 단일 입력, `applyTransactionToFrame`, generation 및 renderer draw evidence를 조합한다. |
| 7 | API 37.2의 VSync token은 exact지만 JankData timestamp는 unknown | exact join과 timestamp 가용성을 별도 필드로 보존하고 fence signal만 timestamp 후보로 삼는다. |
| 8 | API 36에서 present fence API 존재를 모든 기기 지원으로 확대 | 결과 범위를 emulator/API/renderer로 고정하고 실제 기기 표본을 별도로 요구한다. |
| 9 | `service.sf.present_timestamp=0`을 단일 원인으로 단정 | 읽기 전용 platform 관측으로 남기고 runtime valid fence가 반증한 범위를 기록한다. |
| 10 | transaction 완료 시각을 실제 광자 발광으로 설명 | OS transaction present signal로만 부르고 scanout/photon 주장을 금한다. |
| 11 | 입력 clock bracket이 다르면 임의 offset 하나로 변환 | 입력과 callback 각 offset interval 교집합 없이는 latency 산출을 금한다. |
| 12 | offset bracket은 겹치지만 sleep/background가 끼어듦 | sleep/background block은 원시 표본 보존, latency 대상에서 제외한다. |
| 13 | CLOCK_MONOTONIC signal time을 zero·sentinel·미래시각·latch 역전과 혼동 | invalid/pending/nonpositive/future/latch-order 상태를 분리하고 정상 signaled로 합치지 않는다. |
| 14 | fence close 실패를 숨긴 채 유효 sample 집계 | close error를 로그하고 usable 신호에서 제외한다. |
| 15 | API 37.2 JankData callback이 fence callback보다 늦게 도착 | block 종료 drain을 유지하고 offline key join으로 순서 독립 판정한다. |
| 16 | drain 입력을 10개 점수에 포함 | 각 프로세스의 input sequence 1–10만 점수화하고 11은 unscored drain으로 고정한다. |
| 17 | callback queue inline fallback sample을 정상 latency로 포함 | `callback_inline_overflow=true`를 점수에서 제외한다. |
| 18 | generation이 바뀐 늦은 callback을 새 화면에 연결 | request closure의 원래 generation과 current generation을 함께 검사한다. |
| 19 | 한 renderer만 positive라 다른 renderer 반복을 생략 | gate를 renderer/API 조합별로 적용한다. 네 조합 모두 positive라 각각 반복한다. |
| 20 | 표본 누락·중복을 세 block 합계에서 감춤 | block별 request/input/callback/draw/exact-join 수를 각각 판정한다. |

## 추가 검토 결과

계획의 positive-fence gate를 네 조합 각각에서 통과해 반복 block을 완료했다. 각 block의 입력·submit·fence callback은 11/11/11이고 점수 입력은 sequence 1–10이다. 반복 결과와 구현 후 별도 실패 관점 검토는 [Android present-fence 실행 보고서](../spec/internal/evidence/r05-android-present-fence-2026-10-08.md)에 기록했다. 실제 fence가 pending인 표본은 성공으로 바꾸지 않았다.

### 반복 block 결과 · 2026-10-08

| API / renderer | Block 1 | Block 2 | Block 3 | usable 합계 | 후보 event→fence 중앙값 |
|---|---:|---:|---:|---:|---:|
| API 36 WGPU/Vulkan | 10/10 | 10/10 | 10/10 | 30/30 | 16.723 ms |
| API 36 GLES 2.0 | 10/10 | 10/10 | 10/10 | 30/30 | 12.939 ms |
| API 37.2 WGPU/Vulkan | 10/10 | 10/10 | 9/10 | 29/30 | 40.585 ms |
| API 37.2 GLES 2.0 | 10/10 | 10/10 | 10/10 | 30/30 | 40.521 ms |

API 37.2 WGPU block 3 input 1의 valid fence는 pending sentinel을 반환해 점수에서 제외했다. API 37.2의 JankData 60/60은 정확한 target VSync ID와 join됐지만 presentTimeNanos는 모두 -1이다. 표의 event→fence는 synthetic adb input과 clock offset bracket 교집합을 이용한 OS transaction-present 후보이며 물리 scanout·finger-to-photon 또는 renderer 성능 순위가 아니다. 자세한 블록별 수치, 로그·화면, 20개 구현 실패 관점과 남은 검증은 [실행 보고서](../spec/internal/evidence/r05-android-present-fence-2026-10-08.md)에 있다.
