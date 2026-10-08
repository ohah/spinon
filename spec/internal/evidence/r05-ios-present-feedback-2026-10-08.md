# R05.3 iOS WGPU drawable 획득·입력 귀속 검증

**실행일:** 2026-10-08 · **환경:** iPhone 17 Pro / iOS 26.2 Simulator, Xcode 26.2 · **결과:** Simulator WGPU acquire hook·단일 UI 입력 귀속 및 callback ledger 자체 시험 통과 · **기기 표시 callback:** compile/link 통과, runtime 미실행 · **R05.3/R05:** 미완료

## 구현 범위

`R08GpuDemo.swift`의 기존 `R08WgpuCanvasView`와 WGPU 호출 경로를 유지하며, R05 probe가 켜진 경우에만 같은 UIView root `CAMetalLayer`를 `R05ProbeMetalLayer` 하위 클래스로 제공한다. `nextDrawable()`은 `super.nextDrawable()`을 한 번 호출하고 같은 drawable을 반환한다. 실제 WGPU draw 중 override가 호출되는지 로그로 관측한다.

각 WGPU draw에 `drawSequence`, 입력 sequence(없을 수 있음), render revision, surface generation을 담은 불변 ticket을 설정한다. 짧은 lock으로 활성 draw 개수와 ticket 소비 상태를 보호한다. 한 draw만 활성일 때 최초 `nextDrawable()` 호출에서 ticket을 한 번만 소비한다. overlap, sequence overflow, ticket이 없는 frame은 `unattributed`로 남긴다. native Metal 대조와 일반 R08 경로에는 probe layer를 적용하지 않는다.

iPhoneOS device target에서만 입력과 연결된 drawable에 `addPresentedHandler`를 등록한다. WGPU acquire 때 불변 복사한 layer token·surface generation·draw sequence·drawable ID 키로 bounded callback ledger에 exact join한다. pending 64개, monotonic 2초 deadline, 100ms serial timer, tombstone 64개·10초 보존을 적용한다. 이 값은 R05 진단용 상수다. timeout은 callback 누락의 확정 증거가 아니며, 기한 뒤 positive `presentedTime`도 성공 분자에서 제외한다. zero·음수·NaN·무한대·late·duplicate·stale·unmatched·capacity·close를 따로 분류한다. 이 정책과 callback receipt의 lock linearization은 [구현 계획](../../../plan/r05-ios-present-feedback.md)에 명시했다. callback은 view/layer를 캡처하지 않는다.

## 실행 결과

| 검사 | 결과 |
|---|---|
| Rust WGPU static library · `aarch64-apple-ios-sim` | `cargo build --locked --release` 성공 · `wgpu 30.0.1` |
| Rust WGPU static library · `aarch64-apple-ios` | `cargo build --locked --release` 성공 |
| 시뮬레이터 Swift 전체 소스 type-check | 통과 · 기존 iOS 26 deprecation warning만 있음 |
| iPhoneOS device Swift 전체 소스 type-check | 통과 · 기존 iOS 26 deprecation warning만 있음 |
| 격리 앱 harness · iOS Simulator build/install/run | 성공 · Metal backend · WGPU draw result 0 |
| 일반 R08 negative control | 첫 WGPU frame 제출 성공 · `SPINON_R05_DRAWABLE_ACQUIRE` 없음 |
| R05 probe UI test | XCTest 1/1 통과 · 최종 실행 4.784초 · iPhone 17 Pro / iOS 26.2 Simulator |
| probe의 첫 화면 draw | `draw_seq=1..3`, 각 acquire에 `input_seq=none`, `revision=0`, `generation=1` |
| UI test 중앙 탭 | `input_seq=1`, `revision=1`이 `draw_seq=4` acquire에 exact match · 다음 draw 5는 `input_seq=none` |
| iPhoneOS device target compile/link | 격리 앱 harness 성공 · 기기에서 설치·실행하지 않음 |
| 전체 V8 앱 bundle 빌드 | 미실행 · 고정 V8 checkout `build/v8-source/v8`이 없어 기존 앱 build 준비 단계가 중단됨 |
| ledger 자체 시험 | Swift standalone 6개 그룹을 한 실행에서 통과하고, 전체 suite를 다시 20회 독립 실행해 20/20 통과 · 분류·경계·identity·capacity·정렬된 surface lifecycle·overflow·tombstone·동시성·timer cancel/timeout·진단 queue 포화 |
| iPhoneOS device target | Rust target library 및 격리 앱 harness compile/link 통과 · callback runtime은 미실행 |

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
- `drawableID`는 layer token·surface generation·draw sequence와 결합해 join한다. 구현한 timeout은 callback 손실의 확정 판정이 아닌 bounded 관측 분류다.
- Simulator 자체 시험은 ledger state machine·serial timer를 검증하지만 OS의 `addPresentedHandler` 호출은 흉내 내지 않는다. 실제 iPhone에서 handler 도착·presentedTime·surface 재부착 경합은 사용자 요청 전 검증하지 않는다.
- `UITouch.timestamp`와 `presentedTime` 공통 clock, anchor residual, positive `presentedTime` 표본 및 event-to-present latency는 아직 확인하지 않았다.
- 전체 V8 앱 bundle 및 실제 앱 scheme 실행은 고정 V8 checkout 부재로 미확인이다. 이 checkout을 자동 다운로드하거나 기존 캐시를 정리하지 않았다.
- synthetic XCTest 입력이며 physical touch, optical scanout, pixel/photon 시각, 성능 비교가 아니다. 실기기 검증은 별도 사용자 요청 전 수행하지 않는다.

## acquire hook 적대 검토 · 20개 독립 실패 관점

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

### acquire hook 검토 결과

구현 중 중첩 draw와 `drawSequence` overflow가 ticket을 잘못 귀속하거나 debug 앱을 멈출 가능성을 확인해 fail-closed 분기를 추가했다. Simulator UI test는 입력 ticket과 WGPU drawable acquire를 연결했고 일반 R08 negative control은 probe 로그가 없음을 확인했다. device target compile/link도 통과했다.

## callback ledger 구현 적대 검토 · 별도 20개 관점

아래는 앞선 acquire hook 20개 검토를 재사용하지 않은 ledger·통합 전용 점검이다. 각 관점을 별도로 대조하고, 순수 Swift suite도 독립 실행 20회 반복했다. Simulator XCTest는 실제 WGPU acquire 경로만 확인한다.

| # | 공격 관점 | 실제 대조와 결과 |
|---:|---|---|
| 1 | drawable ID 재사용이 다른 surface나 frame과 잘못 join | layer token·generation·draw sequence·drawable ID 전체 키 exact match; 키와 ticket 불일치 입력을 거부했다. |
| 2 | 입력 없는 지속 frame이 callback 표본과 메모리를 불필요하게 늘림 | `inputSequence == nil` ticket은 ledger가 거부하며 device handler도 입력 ticket이 있을 때만 등록한다. |
| 3 | draw sequence 0 또는 ticket mismatch가 유효 키처럼 등록 | 0과 키/ticket sequence mismatch를 `invalid_identity`로 거부한다. |
| 4 | 동일 key 동시 등록이 pending entry를 덮어씀 | 활성 중복을 `duplicate_key`로 분류하고 기존 pending 하나를 보존한다. |
| 5 | pending 한도 초과가 내부 메모리를 계속 증가하거나 handler가 거부 key에 등록 | 1개 한도에서 두 번째 key가 거부되고 pending count가 1로 유지됐다. 통합 코드는 `.registered` 결과에서만 OS handler를 추가한다. 기본 상한은 64다. |
| 6 | deadline 덧셈 overflow가 즉시 만료나 성공으로 바뀜 | `UInt64.max - 5`에서 10ns를 더하는 경우 `deadline_overflow`, pending 0을 확인했다. |
| 7 | deadline 1ns 직전 callback이 조기 만료 | 109/110 경계에서 109는 eligible `presented`였다. |
| 8 | deadline과 같은 시각의 callback이 성공으로 계산 | 동일 경계 110은 먼저 timeout 후 `late_callback`; latency eligibility false였다. |
| 9 | timeout 후 양수 `presentedTime`이 성공 표본을 부활 | timeout tombstone 이후 callback은 late이며 양수 시간을 보존해도 분류·eligibility가 바뀌지 않는다. |
| 10 | callback이 영원히 없을 때 pending이 누적 | 30ms timeout·1ms sweep의 실제 Dispatch timer 시험이 timeout을 기록하고 pending을 0으로 만들었다. |
| 11 | `presentedTime == 0`을 0ms 성공으로 취급 | `zero_time`, ineligible로 분리했다. |
| 12 | 음수·NaN·±infinity를 유효 timestamp로 통과 | 네 입력 모두 `invalid_presented_time`, ineligible였다. |
| 13 | 같은 drawable callback 재전달이 표본 수를 증가 | 성공 뒤 중복은 `duplicate_callback`; 병렬 64 callback에서 성공 1건·중복 63건을 확인했다. |
| 14 | 외부 layer callback이 이 ledger 입력과 섞임 | 다른 token 등록은 `invalid_identity`, 다른 token callback은 `unmatched_callback`이다. |
| 15 | surface 교체 중 이전 세대 pending이 새 화면 결과로 귀속 | generation 1→2 변경이 이전 pending을 stale 처리했고 이전 callback도 stale callback으로 남았다. |
| 16 | detach 뒤 callback이 신규 surface 표본으로 합쳐지거나 terminal 로그 순서가 nondeterministic | retire가 pending을 `drawSequence` 기준으로 정렬해 stale terminal로 닫고, 뒤늦은 callback도 stale 처리했다. sequence 9·7·8로 등록한 fixture가 7·8·9 순서로 종결됐다. |
| 17 | generation 회귀·0이 이전 ID를 재사용 | 회귀와 0을 거부했고 현재 세대는 유지됐다. UIKit generation 증가는 checked overflow에서 fail-closed한다. |
| 18 | tombstone 폭주·시간 만료가 duplicate 구분을 깨뜨림 | 최대 2개 FIFO evict, retention 경계의 pruning을 확인했다. evict/만료 후 오래된 callback은 unmatched로만 남는다. |
| 19 | timer/callback/log sink가 상태를 역전시키거나 lock 안에서 UI를 막고, queue backlog가 무한히 자람 | timestamp capture·expiry·전이는 같은 lock 구간에서 직렬화한다. event queue cap 1 negative case에서 진단 event drop 수 1과 측정 무효 marker를 확인했다. 첫 실행에서 queued event가 semaphore보다 오래 살아 macOS `SIGTRAP: Semaphore object deallocated while in use`를 재현했다. drain closure가 semaphore/drop counter 수명을 보유하도록 수정한 뒤 전체 자체 시험을 재실행해 통과했다. sink의 pendingCount 재진입 조회·event 순서도 확인했다. |
| 20 | close와 callback/timeout 경쟁이 pending을 부활시키거나 상태 marker가 기기 callback 지원을 오표기 | close/callback 100회·expire/close 100회 경쟁에서 각 항목은 한 terminal 경로로만 종결됐고 pending은 매회 0이었다. 별도 timer close 시험에서 닫힌 pending의 timeout도 없었다. 최종 source audit에서 device `present_signal=unavailable` 표기를 발견해 device callback-ledger mode와 Simulator unavailable을 분리하고 두 대상 build/test로 확인했다. |

### ledger 검토 결과

검토 중 close 전 별도 timeout sweep이 상태 전이를 갈라놓는 문제와, lock 밖에서 이벤트를 enqueue하면 동시 기록 순서가 바뀔 수 있는 문제를 발견해 수정했다. 첫 queue 수명 시험은 semaphore 해제 SIGTRAP을 재현했고 drain 작업이 semaphore와 drop counter를 유지하도록 고쳤다. 테스트 fixture가 pending 한도 1인 ledger에서 pending 세 개를 기대하던 설정 오류도 분리해 한도 3의 lifecycle fixture로 재시험했다. ledger는 lock을 얻은 뒤 monotonic 시각을 읽고 만료·상태 전이를 한 구간에서 처리하며, diagnostic sink는 bounded serial queue에서 lock 밖으로 실행한다. Swift standalone self-test 6개 그룹과 전체 20회 반복이 20/20 통과했고, Simulator XCTest 1/1 및 iPhoneOS device target compile/link도 통과했다. 위 20개 관점은 개별 코드 경로·실패 입력·실행 결과를 구분해 대조했다. OS device callback runtime, 전체 V8 bundle, clock residual, 실기기·광학 표시 시각은 미검증이며 R05.3/R05 완료 체크는 하지 않는다.
