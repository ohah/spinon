# R05.3 iOS WGPU drawable 획득·입력 귀속 검증

**실행일:** 2026-10-08 · **환경:** iPhone 17 Pro / iOS 26.2 Simulator, Xcode 26.2 · **결과:** Simulator WGPU acquire hook 및 단일 UI 입력의 draw/revision 귀속 통과 · **기기 표시 callback:** compile/link 통과, runtime 미실행 · **R05.3/R05:** 미완료

## 구현 범위

`R08GpuDemo.swift`의 기존 `R08WgpuCanvasView`와 WGPU 호출 경로를 유지하며, R05 probe가 켜진 경우에만 같은 UIView root `CAMetalLayer`를 `R05ProbeMetalLayer` 하위 클래스로 제공한다. `nextDrawable()`은 `super.nextDrawable()`을 한 번 호출하고 같은 drawable을 반환한다. 실제 WGPU draw 중 override가 호출되는지 로그로 관측한다.

각 WGPU draw에 `drawSequence`, 입력 sequence(없을 수 있음), render revision, surface generation을 담은 불변 ticket을 설정한다. 짧은 lock으로 활성 draw 개수와 ticket 소비 상태를 보호한다. 한 draw만 활성일 때 최초 `nextDrawable()` 호출에서 ticket을 한 번만 소비한다. overlap, sequence overflow, ticket이 없는 frame은 `unattributed`로 남긴다. native Metal 대조와 일반 R08 경로에는 probe layer를 적용하지 않는다.

iPhoneOS device target에서만 반환 drawable의 `addPresentedHandler`를 등록하고 `drawableID`, `presentedTime` 및 캡처한 ticket을 기록한다. `presentedTime == 0`은 `zero_time`으로 기록한다. callback은 view를 캡처하지 않으며 event-to-present 시간 계산은 하지 않는다. 현재 구현에는 callback ledger, callback timeout/lost 정리, epoch 변환 또는 latency 집계가 없다.

## 실행 결과

| 검사 | 결과 |
|---|---|
| Rust WGPU static library · `aarch64-apple-ios-sim` | `cargo build --locked --release` 성공 · `wgpu 30.0.1` |
| Rust WGPU static library · `aarch64-apple-ios` | `cargo build --locked --release` 성공 |
| 시뮬레이터 Swift 전체 소스 type-check | 통과 · 기존 iOS 26 deprecation warning만 있음 |
| iPhoneOS device Swift 전체 소스 type-check | 통과 · 기존 iOS 26 deprecation warning만 있음 |
| 격리 앱 harness · iOS Simulator build/install/run | 성공 · Metal backend · WGPU draw result 0 |
| 일반 R08 negative control | 첫 WGPU frame 제출 성공 · `SPINON_R05_DRAWABLE_ACQUIRE` 없음 |
| R05 probe UI test | XCTest 1/1 통과 · 최종 실행 4.734초 · iPhone 17 Pro / iOS 26.2 Simulator |
| probe의 첫 화면 draw | `draw_seq=1..3`, 각 acquire에 `input_seq=none`, `revision=0`, `generation=1` |
| UI test 중앙 탭 | `input_seq=1`, `revision=1`이 `draw_seq=4` acquire에 exact match · 다음 draw 5는 `input_seq=none` |
| iPhoneOS device target compile/link | 격리 앱 harness 성공 · 기기에서 설치·실행하지 않음 |
| 전체 V8 앱 bundle 빌드 | 미실행 · 고정 V8 checkout `build/v8-source/v8`이 없어 기존 앱 build 준비 단계가 중단됨 |

원본 실행 자료는 [`ios-layer-smoke/`](r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/)에 둔다. XCTest 탭 뒤 GPU 도형 색과 활성화 횟수를 담은 [최종 화면 캡처](r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/tap-after.png)와 [무결성 manifest](r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/SHA256SUMS)를 함께 보존한다. 실제 R08 Swift 소스를 그대로 컴파일하며, 로컬 고정 V8 checkout 없이 surface 경로를 검사하기 위해 V8 앱 래퍼만 기존 WGPU FFI forwarding shim으로 대체한다. 따라서 격리 harness 성공을 전체 앱 bundle 성공으로 보지 않는다.

대표 입력 귀속 로그:

```text
SPINON_R05_INPUT renderer=wgpu input_seq=1 ... generation=1
SPINON_R05_DRAWABLE_ACQUIRE renderer=wgpu state=available thread=main drawable_id=not_recorded draw_seq=4 input_seq=1 revision=1 generation=1
SPINON_R05_SUBMIT renderer=wgpu input_seq=1 revision=1 generation=1 ... result=0 present_signal=unavailable
SPINON_R05_DRAWABLE_ACQUIRE renderer=wgpu state=available thread=main drawable_id=not_recorded draw_seq=5 input_seq=none revision=1 generation=1
```

Simulator SDK에는 device-only drawable callback 멤버가 없으므로 위 `not_recorded`는 의도된 상태다. probe가 입력을 실제 OS frame에 표시했다거나 `presentedTime`을 관측했다는 뜻이 아니다.

## 미완료 경계

- Simulator에서 확인한 것은 `nextDrawable()` acquire 진입과 입력→acquire ticket 귀속이다. actual-present callback은 Simulator에서 실행되지 않는다.
- iPhoneOS device target은 API 분기를 정적 라이브러리와 함께 compile/link했지만 physical iPhone runtime을 확인하지 않았다.
- `drawableID`는 generation 범위 안에서만 join에 사용할 수 있다. callback 누락·중복·late/stale generation 처리 ledger와 bounded timeout은 아직 없다.
- `UITouch.timestamp`와 `presentedTime` 공통 clock, anchor residual, positive `presentedTime` 표본 및 event-to-present latency는 아직 확인하지 않았다.
- 전체 V8 앱 bundle 및 실제 앱 scheme 실행은 고정 V8 checkout 부재로 미확인이다. 이 checkout을 자동 다운로드하거나 기존 캐시를 정리하지 않았다.
- synthetic XCTest 입력이며 physical touch, optical scanout, pixel/photon 시각, 성능 비교가 아니다. 실기기 검증은 별도 사용자 요청 전 수행하지 않는다.

## 구현 적대 검토 · 20개 독립 실패 관점

각 항목은 코드·빌드·실행 근거를 대조했다. 실제로 주입하지 않은 고장 조건은 정적 검토와 런타임 통과를 구분했다.

| # | 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | probe를 끈 일반 R08이 실수로 계측 layer를 사용 | 일반 R08 WGPU 대조에서 기존 draw 성공, acquire probe 로그 0개를 확인했다. |
| 2 | native Metal 대조가 WGPU acquire hook으로 오염 | view 생성 분기상 probe subclass는 `useWgpu && r05PresentationProbe`일 때만 선택된다. native Metal은 이전 `R08MetalCanvasView` 경로다. 정적 검토. |
| 3 | base R08 layer 타입을 바꾸며 기존 renderer가 손상 | base view의 `layerClass`는 기존처럼 `CAMetalLayer.self`이며 일반 R08 실제 WGPU draw가 통과했다. |
| 4 | WGPU가 subclass가 아닌 별도 drawable source를 사용 | 실제 WGPU create/draw 중 `R05ProbeMetalLayer.nextDrawable()` 로그를 세 차례 확인했다. |
| 5 | override가 새 drawable을 만들어 surface 소유권을 바꿈 | override는 `super.nextDrawable()` 한 번의 반환값을 그대로 반환한다. 정적 검토. |
| 6 | drawable nil/획득 실패를 성공으로 기록 | nil이면 `state=unavailable`로 기록하고 동일 nil 결과를 반환하며 presented handler를 등록하지 않는다. nil 주입은 미실행. |
| 7 | 이전 입력 ticket이 다음 frame에 재사용 | ticket은 첫 acquire에서 한 번 소비된다. UI test 뒤 다음 draw는 `input_seq=none`이었다. |
| 8 | 입력 없는 초기 frame을 탭에 임의 연결 | 최초 draw 1–3은 `input_seq=none`; 탭 입력 sequence 1은 draw 4에만 기록됐다. |
| 9 | 입력 sequence와 render revision이 서로 다름 | XCTest tap에서 input 1·revision 1·draw 4가 한 로그에 exact match됐다. |
| 10 | view 재부착 뒤 drawable ID를 이전 세대에 붙임 | ticket에 surface generation을 기록한다. 실제 detach/reattach와 late callback 주입은 미실행이다. |
| 11 | 동시에 겹친 draw가 ticket을 서로 바꿔 붙임 | 활성 draw 수가 1이 아니면 acquire attribution을 fail-closed 처리한다. overlap 주입은 미실행. |
| 12 | `drawSequence`가 넘쳐 debug 앱이 중단되거나 wrap됨 | checked addition 실패 시 ticket을 만들지 않고 frame은 `unattributed` 처리한다. overflow 주입은 미실행. |
| 13 | WGPU acquire 실패 뒤 ticket이 남아 다음 draw로 샘 | renderer draw 호출 범위의 `defer`에서 활성 draw 상태를 정리한다. surface error 주입은 미실행. |
| 14 | layer lock을 callback 동안 보유해 main/UI를 막음 | lock 범위는 draw/ticket 값 복사뿐이고 `super.nextDrawable()`·OSLog·presented handler 안에서는 lock을 보유하지 않는다. 정적 검토. |
| 15 | callback이 mutable view를 강하게 잡아 수명을 연장 | callback은 logger value와 문자열 ticket만 캡처하고 view/layer를 캡처하지 않는다. device callback runtime은 미실행. |
| 16 | Simulator에서 비공개 selector로 device API를 호출 | drawable ID와 presented handler 참조는 `#if !targetEnvironment(simulator)` 분기 안에만 있다. Simulator build/test 통과. |
| 17 | device API 분기가 compile만 되고 링크되지 않음 | iPhoneOS SDK·Rust target으로 격리 앱 harness compile/link가 통과했다. physical install/run은 하지 않았다. |
| 18 | `presentedTime == 0`을 0ms 성공으로 집계 | device callback 코드는 `zero_time` outcome으로 남기며 latency를 계산하지 않는다. callback runtime은 미실행. |
| 19 | drawable ID·clock epoch을 잘못 가정해 latency 계산 | generation을 ticket에 보존하며 시간 차 계산을 구현하지 않았다. 공통 clock 보정 전까지 latency 미보고다. |
| 20 | 격리 harness 결과를 전체 앱·실기기·광학 완료로 확대 | 실제 R08 Swift와 실제 wgpu 정적 library를 썼지만 V8 wrapper는 제외했다. 별도 V8 app 빌드·physical iPhone·광학 측정은 미검증으로 기록했다. |

### 검토 결과

구현 중 중첩 draw와 `drawSequence` overflow가 ticket을 잘못 귀속하거나 debug 앱을 멈출 가능성을 확인해 fail-closed 분기를 추가했다. Simulator UI test는 입력 ticket과 WGPU drawable acquire를 연결했고 일반 R08 negative control은 probe 로그가 없음을 확인했다. device target compile/link도 통과했다. callback queue/timeout, callback 실패 주입, physical device runtime, 전체 V8 앱 bundle은 다음 검증 단계로 남겼으며 R05.3/R05 완료 체크는 하지 않는다.
