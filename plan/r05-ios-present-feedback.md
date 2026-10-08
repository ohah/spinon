# R05.3 iOS Metal 표시 feedback 구현 계획

**상태:** 계획 재검토 완료 · WGPU acquire hook·입력 ticket·bounded callback ledger 구현 · Swift 자체 시험 6개 그룹·iOS 26.2 Simulator UI 검증 통과 · iPhoneOS device-target compile/link 통과 · 기기 callback runtime 미검증 · R05.3 미완료

**상위:** [R05 입력→표시 계측 계획](r05-input-to-presentation.md) · [R05 상태 대장](../spec/STATUS.md)

## 확인된 SDK 범위

Xcode `26.2 (17C52)`의 iPhoneOS SDK `26.2`에는 `MTLDrawable.drawableID`, `addPresentedHandler`, `presentedTime` 선언이 있고 모두 iOS 10.3 이상으로 표시된다. 동일 probe가 `arm64-apple-ios26.2` device target type-check를 통과했다. iOS Simulator SDK `26.2` 헤더에는 세 선언이 없고 `arm64-apple-ios26.2-simulator` type-check가 실패한다. 원본 명령과 출력은 [SDK matrix log](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/ios-sdk-matrix-typecheck.log)에 있다.

이는 iOS 기기용 공개 API가 존재한다는 컴파일 근거다. 실제 iPhone에서 callback과 시각 값이 전달된다는 runtime 근거는 아니다. Simulator SDK에 노출되지 않은 API를 selector·비공개 API로 호출하지 않는다. 시뮬레이터는 surface 경로와 입력 귀속을 검증하고 표시 feedback은 unavailable로 남긴다.

## 제안 경로와 경계

현재 화면은 `R08WgpuCanvasView`의 root layer로 `CAMetalLayer`를 제공한다. 고정된 의존성 `wgpu 30.0.1`의 Metal backend는 `UIView.layer`를 surface layer로 유지하고 `Surface::get_current_texture()`에서 그 layer의 `nextDrawable()`을 호출한다. 공개 wgpu `SurfaceTexture`는 Metal drawable ID나 표시 callback을 노출하지 않는다. 이는 설치된 `wgpu-hal 30.0.1`·`raw-window-metal 1.1.0` 소스에서 확인했다.

구현은 **같은 root layer를 `CAMetalLayer` 하위 클래스 하나로 바꾸고 `nextDrawable()`을 override하는 내부 probe**를 사용한다. 별도 Metal layer/drawable을 만들거나 WGPU surface 소유권을 바꾸지 않는다. override는 `super.nextDrawable()` 결과를 그대로 반환한다. device target에서만 그 drawable에 `addPresentedHandler`를 붙여 `drawableID`, `presentedTime`과 입력 ticket을 관측한다. Simulator에서는 acquire hook만 검증하고 표시 callback을 모사하지 않는다.

2026-10-08 iPhone 17 Pro / iOS 26.2 Simulator에서 저장소의 `R08GpuDemo.swift`와 실제 `spinon-wgpu-r08-spike` 정적 라이브러리를 연결한 격리 harness를 실행했다. WGPU draw 중 같은 root layer의 override가 호출됐고 XCTest 단일 탭은 `input_seq=1`, `revision=1`을 `draw_seq=4` acquire에 연결했다. probe를 끈 R08 대조에서는 acquire probe 로그가 없었다. 고정 `wgpu 30.0.1` 실행 경로에 hook이 진입하는 것은 확인했지만, 전체 V8 앱 번들에서의 통합·iPhone 표시 callback runtime·latency는 확인하지 않았다. 이 후보를 공개 Spinon API로 승격하지 않는다.

## 실행 순서

1. **완료:** Xcode 26.2 iPhoneOS/Simulator SDK probe를 각각 type-check해 API capability를 고정했다.
2. **완료:** R05의 같은 `R08WgpuCanvasView` root layer를 probe subclass로 구성했다. probe 비활성 R08 대조와 `nextDrawable()`의 super 호출·원본 객체 반환을 확인했다.
3. **완료:** iOS Simulator XCTest에서 입력 없음 draw와 입력 귀속 draw를 같은 draw sequence·revision·surface generation으로 기록했다. 단일 탭 입력과 acquire ticket이 exact match했고 후속 입력 없는 frame은 `input_seq=none`이었다.
4. **완료(기기 runtime 제외):** `#if !targetEnvironment(simulator)`에서만 drawable presented handler를 등록한다. ticket은 입력 sequence·draw sequence·revision·surface generation을 immutable value로 보존한다. iPhoneOS SDK 정적 라이브러리 연결 앱 target build를 통과했다. callback runtime은 미검증이다.
5. **구현 완료·실기기 확인 미완료:** 입력 ticket이 있는 drawable만 `layerToken + surfaceGeneration + drawSequence + drawableID`로 bounded ledger에 등록한다. pending 64개·2초 monotonic deadline·100ms serial sweep·tombstone 64개/10초 보존으로 누락·late·duplicate·stale·unmatched·capacity·close 및 invalid/zero timestamp를 나눈다. 표본 deadline은 진단 timeout이며 callback 손실의 확정 판정이 아니다. OS drawable callback 수신과 기기 동작은 아직 실행하지 않았다.
6. **미완료:** `UITouch.timestamp`, `ProcessInfo.systemUptime`, `CACurrentMediaTime()`의 실행 전후 anchor는 기록하지만 clock epoch residual을 판정하지 않았다. 실제 device callback에서 공통 clock과 residual 범위를 확정하지 못하면 event→present latency를 산출하지 않는다.
7. **완료:** Simulator에서 override의 실제 WGPU drawable 획득과 비활성 R08 대조를 확인했다. 격리 harness의 iPhoneOS device target compile/link도 통과했다. 전체 V8 앱 빌드는 고정 V8 checkout 부재로 실행되지 않았다. 물리 iOS 기기 runtime은 사용자가 요청할 때만 진행한다.
8. **미완료·실기기 요청 대기:** 허가된 뒤 physical iPhone의 WGPU 경로에서 isolated input 30개 이상을 실행한다. exact layer/drawable/ticket join, positive presented time, callback 유실·drop·stale rejection 및 clock bracket을 먼저 통과해야 latency 요약을 낸다. native Metal 대조와 WGPU 결과를 합치지 않는다.

## 합격과 미합격

- **SDK compile gate:** iPhoneOS device target용 WGPU harness의 compile/link는 통과했다. 전체 V8 앱 target은 V8 checkout 부재로 아직 빌드되지 않아 이 계획의 제품 통합 gate는 미완료다. API availability는 현재 iOS deployment target 18.0에서 확인했다.
- **Simulator path gate:** override가 같은 root `CAMetalLayer`의 WGPU acquisition 경로에 실제로 진입하고 비활성 R08 동작에 회귀가 없어야 한다. Simulator 결과는 `nextDrawable` acquire 확인까지만이다.
- **Device feedback gate:** 기기에서 각 callback의 `drawableID`, `presentedTime`, immutable ticket, surface generation이 정확히 결합되고 zero/drop·late·duplicate·missing callback이 success에서 배제되어야 한다. 현재 compile/link와 state-machine 시험만 통과했으며 이 gate는 미완료다.
- **Timing gate:** `UITouch.timestamp`와 Metal host time의 변환 오차 구간을 관측 자료로 설명할 수 있어야 한다. 단위 변환이나 API 선언만으로 같은 epoch라고 판정하지 않는다.
- **Claim gate:** MTLDrawable feedback은 OS display feedback이다. scanout의 광자 발광·손가락 접촉 시각과 동일하지 않다. 실기기 실행이 없으면 iOS latency 수치나 R05 cross-platform 완료를 주장하지 않는다.

## 계획 적대 검토 · 20개 독립 실패 관점

각 검토는 별개의 SDK 경계·surface 소유권·frame identity·callback 수명·시간 의미를 겨냥한다. 확인 근거와 미실행 관문을 구분했다.

| # | 공격 관점 | 계획의 대응·판정 |
|---:|---|---|
| 1 | iOS SDK와 Simulator SDK를 하나로 취급 | 두 SDK target을 분리했다. device type-check 통과, simulator 실패를 각각 보존했다. |
| 2 | versioned SDK symlink 착시 | versioned Simulator SDK가 `iPhoneSimulator.sdk` symlink임을 확인했고 두 경로 결과가 같다. |
| 3 | device type-check만으로 runtime 지원을 주장 | runtime·물리 기기 증거는 별도 gate로 남겼다. |
| 4 | simulator compile 실패를 iOS 전체 API 부재로 확대 | iPhoneOS SDK header와 device target compile을 반대 증거로 함께 기록한다. |
| 5 | device-only Metal API를 Simulator에서 강제 호출 | callback 코드는 `targetEnvironment(simulator)` 밖에 두고 selector 우회는 금한다. |
| 6 | command-buffer 완료를 drawable 표시 완료로 오인 | `addPresentedHandler`/`presentedTime`만 display feedback 후보로 쓴다. |
| 7 | `presentedTime == 0`을 0ms 성공으로 집계 | dropped·미표시 표본으로 분류하고 latency 계산에서 제외한다. |
| 8 | `drawableID`를 앱 전체 고유 ID로 취급 | layer와 surface generation을 join key에 포함한다. |
| 9 | handler 도착 순서로 입력을 추정 연결 | acquire 시점에 immutable input/revision/generation ticket을 캡처한다. |
| 10 | drawable acquire가 실제 WGPU surface와 다른 객체 | 기존 UIView root layer subclass의 override가 실제 WGPU draw 중 호출되고 drawable을 반환함을 Simulator log로 확인했다. |
| 11 | root layer subclass가 WGPU layer 선택과 맞지 않음 | pinned raw-window-metal path가 UIView root CAMetalLayer를 재사용하는지 확인한다. 기기/runtime 동작은 별도 확인한다. |
| 12 | override가 base `nextDrawable()`을 호출하지 않거나 다른 drawable 반환 | super 결과를 그대로 반환하고 추가 draw/present를 하지 않는 계약을 둔다. |
| 13 | callback이 실행 전에 ticket을 지워 입력 연결을 잃음 | ticket은 drawable handler closure의 immutable copy로 전달한다. 구현 후 callback ordering 검증이 필요하다. |
| 14 | callback이 늦게 와서 새 surface generation에 붙음 | callback 시 generation을 비교하고 stale outcome으로 분리한다. |
| 15 | callback 누락으로 표본 map이 무한 성장 | bounded table, timeout, lost accounting을 요구한다. |
| 16 | drop·중복·0-time callback을 평균에서 숨김 | 표본별 outcome을 보존하며 성공 분모에 넣지 않는다. |
| 17 | `UITouch.timestamp`와 host time을 epoch 확인 없이 뺌 | 전후 anchor 및 residual interval이 통과할 때만 수치를 계산한다. |
| 18 | 계측 callback이 main thread를 막아 지연을 스스로 생성 | callback에서 동기 대기하지 않고 bounded asynchronous 기록으로 전달한다. |
| 19 | native Metal 대조 결과를 WGPU 경로 증거로 대체 | 동일 R08 WGPU root layer 경로를 검사하고 native control은 별도 결과로 유지한다. |
| 20 | 시뮬레이터·컴파일 근거를 실기기/광학 완료로 확대 | Simulator는 acquisition hook만, device runtime은 사용자가 허용한 뒤, optical photon time은 별도 계측으로 한정한다. |

### 계획 검토 결과

20개 관점에서 SDK 두 종류, `MTLDrawable` 공개 선언, R08 Swift view layer, `wgpu 30.0.1`/`wgpu-hal 30.0.1` Metal acquire·present 경로, `raw-window-metal 1.1.0` UIKit root layer 선택을 대조했다. iPhoneOS SDK에는 세 public member가 있지만 Simulator SDK에는 없음을 분리했다. 같은 root layer subclass의 실제 WGPU acquire 진입을 Simulator XCTest로 확인하고 probe-off R08 대조도 통과했다. 기기 callback runtime이나 전체 V8 앱 통합은 검증하지 않았으며 R05.3 완료 판정으로 확대하지 않는다.

## 후속 범위 · callback 결과 ledger 계약

다음 구현은 이 내부 R05 진단 probe에만 적용한다. 공개 API나 첫 릴리스의 메모리·성능 정책으로 승격하지 않는다.

### 사전 고정 규칙

- **등록 대상:** `inputSequence != nil`인 WGPU drawable만 등록한다. 연속 화면 그리기 frame 전체를 추적하지 않아 callback 수와 계측 비용을 입력 표본 수에 한정한다. 귀속 ticket이 없거나 overlap으로 fail-closed된 draw에는 handler를 붙이지 않는다.
- **식별 키:** `layerToken + surfaceGeneration + drawSequence + drawableID`. `drawableID` 단독, callback 도착 순서, 최근 입력 시각으로 join하지 않는다. 값은 acquire 순간 불변 복사한다.
- **대기 상한:** layer별 pending 64개. 상한에 도달하면 새 callback handler를 등록하지 않고 `capacity_rejected`를 남긴다. 기존 renderer draw/present는 계속 진행한다.
- **deadline:** 등록 시 monotonic uptime 기준 2초. 마감 시각은 checked addition으로 계산하며 overflow면 `deadline_overflow`로 닫는다. 마감 시각과 같거나 늦게 도착한 handler는 성공으로 세지 않고 `late_callback`으로 기록한다. 늦은 callback의 양수 `presentedTime`은 진단 로그에 보존하지만 점수 분자에는 포함하지 않는다.
- **만료 실행:** layer 전용 serial Dispatch timer가 100ms 간격으로 만료를 점검한다. 입력·UI thread는 기다리지 않는다. timer callback은 lock으로 상태만 변경하고 OSLog를 lock 밖에서 쓴다. timeout 기준은 monotonic uptime이며 `UITouch.timestamp`나 Metal host time과 빼지 않는다.
- **동시 호출 선형화:** ledger lock을 얻은 뒤 monotonic 시각을 읽고, 만료 점검과 해당 호출의 상태 전이를 같은 lock 구간에서 처리한다. 따라서 timer/callback 경합은 lock 획득 순서 하나로 정해진다. 로그는 상태 전이와 같은 lock 구간에서 순서대로 bounded serial 진단 큐에 enqueue하고, sink 실행은 lock 밖에서 한다. 진단 큐는 최대 256 event이며 초과분은 drop count와 `diagnostic_events_dropped`로 표시하고 해당 실행의 측정은 무효 처리한다. `observed_uptime_ns`는 진단 ledger의 직렬화 시각이며 물리 callback 진입 시각이나 latency 계산값이 아니다.
- **terminal tombstone:** 완료·timeout·stale 키를 10초 동안 보존해 duplicate/late callback을 판별한다. tombstone 최대 64개이며 한도를 넘으면 가장 오래된 terminal 키부터 제거한다. pending과 tombstone은 별도 상한을 가진다.
- **surface 수명:** attach 시 generation을 설정한다. detach나 generation 교체 때 해당 surface의 pending 항목을 `stale_generation`으로 닫고, 늦은 callback은 새 generation 표본에 연결하지 않는다. generation 회귀·overflow는 등록을 거부한다.
- **종료:** close는 별도의 선행 만료 호출 없이 한 lock 전이로 ledger를 닫고 모든 pending을 `closed_pending`으로 계수한 뒤 timer를 cancel한다. 이미 보유된 callback closure는 ledger와 immutable key만 캡처한다. 종료 이후 도착은 `callback_after_close`이며 view/layer를 다시 참조하지 않는다.
- **callback 결과:** `presentedTime > 0`만 기한 안의 `presented`로 분류한다. 0은 `zero_time`, 음수·NaN·무한대는 `invalid_presented_time`이다. timeout은 실제 callback 손실을 증명하지 않으므로 `missing_callback_timeout`이라는 관측 분류를 쓴다.
- **Simulator 시험 경계:** Simulator SDK에 device drawable callback API가 없으므로 OS callback을 흉내 내 성공으로 주장하지 않는다. 같은 순수 Swift ledger에 결정론적 timestamp·callback을 주입해 state machine을 시험하고, Simulator UI test는 기존 실제 WGPU acquire 경로만 확인한다.

### 계획 적대 검토 · ledger 계약 20개 관점

| # | 독립 실패 관점 | 사전 대응·판정 기준 |
|---:|---|---|
| 1 | 지속 draw마다 callback을 등록해 측정이 앱 부하를 바꿈 | input ticket이 있는 drawable만 등록하고 입력 없는 frame은 계측하지 않는다. |
| 2 | drawable ID만으로 서로 다른 layer/surface를 합침 | layer token·surface generation·draw sequence를 키에 포함한다. |
| 3 | 같은 키를 두 번 등록해 첫 ticket을 덮어씀 | pending/tombstone 키 중복은 `duplicate_key`로 거부한다. |
| 4 | 다른 layer의 callback이 이 ledger를 오염 | 생성 시 고정 layer token이 맞지 않는 등록·callback을 거부한다. |
| 5 | 첫 attach 전 generation 기본값 0이 실제 surface로 쓰임 | attach에서 명시적으로 generation을 설정하기 전 등록은 거부한다. |
| 6 | detach 중 pending callback을 성공으로 남김 | detach 시 해당 generation pending을 `stale_generation`으로 종결한다. |
| 7 | generation 회귀·재사용이 이전 callback을 새 frame에 연결 | 단조 증가만 허용하고 이전 generation 키는 tombstone으로 막는다. |
| 8 | generation 정수 overflow가 앱을 종료하거나 ID를 재사용 | checked increment 실패 시 새 세대 등록을 fail-closed한다. |
| 9 | callback 폭주로 pending memory가 증가 | pending 64개 상한과 명시적 `capacity_rejected`를 둔다. |
| 10 | deadline 계산 overflow가 즉시 성공/만료로 오인 | checked addition 실패를 `deadline_overflow`로 기록한다. |
| 11 | deadline 경계 시각의 callback이 nondeterministic하게 성공 | lock 안에서 읽은 직렬화 시각이 `>= deadline`이면 항상 late/timeout으로 분류한다. |
| 12 | 직전·직후 callback의 경계 처리가 뒤집힘 | deadline 바로 전은 성공 가능, 동일 시각과 이후는 late라는 결정론적 비교를 사용한다. |
| 13 | timeout된 callback이 뒤늦게 와 성공 분모를 바꿈 | timeout tombstone을 보존하고 후속 callback은 late로 기록하되 점수에서 제외한다. |
| 14 | presentedTime 0을 0ms latency로 처리 | `zero_time`은 non-success terminal outcome이다. |
| 15 | 음수·NaN·무한대 timestamp를 양수 성공으로 취급 | 유한한 양수만 `presented`; 다른 값은 `invalid_presented_time`이다. |
| 16 | 같은 drawable callback 중복이 표본을 늘림 | terminal tombstone 동안 중복은 `duplicate_callback`으로 분류한다. |
| 17 | 등록하지 않은 callback이 근접 입력에 추정 귀속 | 키 exact match가 없으면 `unmatched_callback`으로 남긴다. |
| 18 | tombstone이 다시 무한히 성장 | 10초 보존과 64개 상한, 가장 오래된 항목 제거 규칙을 적용한다. |
| 19 | callback logger가 main thread를 막거나 느린 sink가 무한 이벤트 backlog를 만듦 | 100ms timer queue·짧은 lock·최대 256 event serial 진단 큐·drop 시 측정 무효 marker·lock 밖 sink 실행을 계약하고 semaphore 수명은 drain 작업까지 보존한다. |
| 20 | 종료와 timer/callback 경합이 상태를 부활시킴 | lock 안에서 close를 먼저 확정하고 pending 전부를 terminal 처리·timer cancel; 이후 callback은 `callback_after_close`로만 남긴다. |

검토 중 표본 대상·키 범위·경계 timestamp·late callback 분모·종료 수명·동시 호출 선형화·로그 순서·진단 queue memory 상한을 구체화했다. 64 pending, 2초 deadline, 100ms sweep, 64 tombstone, 10초 retention, 256 diagnostic event는 R05 진단용 검증 상수이며 성능·제품 정책이 아니다. 구현 검토는 아래의 새 코드와 deterministic state-machine test를 대상으로 별도 20개 관점으로 수행한다.

이 ledger 계약을 코드와 다시 대조한 재검토에서도 20개 관점을 각각 확인했다. 첫 대조에서 close가 만료 sweep과 분리되어 callback이 종료 요청 뒤 성공할 수 있는 순서, 진단 큐 상한 부재, queue가 semaphore보다 오래 살아날 때의 수명 오류를 찾아 계약과 구현을 보강했다. close를 단일 lock 전이로 만들고 진단 queue를 256개로 제한했으며 drop 표식으로 해당 측정을 무효화한다. 실제 재현된 semaphore 해제 SIGTRAP도 queue drain까지 semaphore를 유지하도록 수정했다. 마지막으로 6개 그룹 전체 자체 시험을 독립 실행 20회 반복해 20/20 통과했다. 구체적인 각 구현 관점과 결과는 [ledger 실행 근거](../spec/internal/evidence/r05-ios-present-feedback-2026-10-08.md)에 기록했다.
