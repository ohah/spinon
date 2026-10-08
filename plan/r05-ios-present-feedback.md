# R05.3 iOS Metal 표시 feedback 구현 계획

**상태:** 계획 검토 완료 · WGPU acquire hook과 입력 ticket 연결 구현 · iOS 26.2 Simulator UI 검증 통과 · iPhoneOS device-target compile/link 통과 · 기기 callback runtime 미검증 · R05.3 미완료

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
5. **미완료:** `drawableID`는 layer 안에서만 단조 증가하므로 join key에 surface/layer generation을 포함한다. `presentedTime == 0`, callback 누락, surface 교체 전후 callback, 중복 callback은 success가 아니라 별도 실패/누락 결과로 분류해야 한다. 현재 코드는 callback 로그만 남기며 bounded callback ledger, deadline, lost accounting을 구현하지 않았다.
6. **미완료:** `UITouch.timestamp`, `ProcessInfo.systemUptime`, `CACurrentMediaTime()`의 실행 전후 anchor는 기록하지만 clock epoch residual을 판정하지 않았다. 실제 device callback에서 공통 clock과 residual 범위를 확정하지 못하면 event→present latency를 산출하지 않는다.
7. **완료:** Simulator에서 override의 실제 WGPU drawable 획득과 비활성 R08 대조를 확인했다. 격리 harness의 iPhoneOS device target compile/link도 통과했다. 전체 V8 앱 빌드는 고정 V8 checkout 부재로 실행되지 않았다. 물리 iOS 기기 runtime은 사용자가 요청할 때만 진행한다.
8. **미완료·실기기 요청 대기:** 허가된 뒤 physical iPhone의 WGPU 경로에서 isolated input 30개 이상을 실행한다. exact layer/drawable/ticket join, positive presented time, callback 유실·drop·stale rejection 및 clock bracket을 먼저 통과해야 latency 요약을 낸다. native Metal 대조와 WGPU 결과를 합치지 않는다.

## 합격과 미합격

- **SDK compile gate:** iPhoneOS device target용 WGPU harness의 compile/link는 통과했다. 전체 V8 앱 target은 V8 checkout 부재로 아직 빌드되지 않아 이 계획의 제품 통합 gate는 미완료다. API availability는 현재 iOS deployment target 18.0에서 확인했다.
- **Simulator path gate:** override가 같은 root `CAMetalLayer`의 WGPU acquisition 경로에 실제로 진입하고 비활성 R08 동작에 회귀가 없어야 한다. Simulator 결과는 `nextDrawable` acquire 확인까지만이다.
- **Device feedback gate:** 각 callback의 `drawableID`, `presentedTime`, immutable ticket, surface generation이 정확히 결합되고 zero/drop·late·duplicate·missing callback이 success에서 배제되어야 한다.
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
