# C04.10 구현 최종 적대 검토

2026-10-09 현재 C04.10 변경 트리를 대상으로 런타임·FFI·GPU·Android·iOS·버전 경계의 실패 경로를 다시 확인했다. 내부 계약 버전은 출시 전 규칙대로 `0.1.0`에 고정했다. `0031`은 명세 문서 식별 번호이며 출시 버전이 아니다.

## 실패 경로 검토

| # | 공격 관점 | 검토 결과 |
|---:|---|---|
| 1 | 새 렌더러 crate·명세 추가가 제품/계약 숫자 버전을 올리는가 | 모두 `0.1.0`이다. `Cargo.lock`의 변경은 wgpu 등 의존성 잠금이며 Spinon 출시 버전 변경이 아니다. |
| 2 | 잘못된 출력 포인터·용량이 FFI 진입 뒤 역참조되는가 | 생성·호스트 함수는 출력 포인터와 용량을 검사한 뒤 보고를 쓴다. 기존 보고 복사 helper는 종료 NUL 공간 부족을 실패 처리한다. |
| 3 | null 또는 만료된 host를 정상 GPU 작업에 전달하는가 | C ABI가 null을 거부한다. 실제 host 수명은 호출자가 보장해야 한다고 계약과 Swift/Android 소유자에 명시되어 있다. 해제와 동시 호출은 허용하지 않는다. |
| 4 | Android create가 실패하거나 보고 버퍼가 부족할 때 renderer가 남는가 | 초기화 오류는 null과 오류 보고를 반환한다. 성공 후 보고 기록이 실패하면 renderer를 파괴하고 null을 반환한다. |
| 5 | iOS surface 준비/renderer 생성 사이 실패가 pending surface를 남기는가 | surface 생성 실패는 pending slot을 채우지 않는다. renderer 생성은 pending을 한 번 소비하고 초기화 오류 때 소유권을 해제한다. 출력 보고 실패 경로도 host 정리를 요청한다. |
| 6 | JavaScript 시간 제한이 V8 실행까지 취소한다고 오인되는가 | 계약은 `layout_timeout_millis`가 CSS worker 결과 대기만 제한한다고 명시한다. JS 평가는 동기이며 종료/취소 보장은 하지 않는다. |
| 7 | CSS 계산 실패·timeout·오래된 완료가 새 GPU scene으로 바뀌는가 | pending/failed/missing/요청 key 불일치 상태를 각각 오류로 처리한다. 계산 오류를 성공 장면으로 변환하지 않는다. |
| 8 | 문서 변경·style·환경 revision 중 하나가 다른 snapshot을 통과하는가 | scene key 전체 다섯 축을 비교한다. FFI와 style-to-render 시험은 오래된 축별 revision 및 publish 경합을 거부한다. |
| 9 | presentation 무효화와 scene publish 경합에서 예전 장면이 새 장면으로 남는가 | atomic sequence와 scene lock을 함께 확인한다. publish 경합 단위 시험에서 무효화가 이기면 이전 scene을 거부한다. renderer도 submit·present 경계에서 sequence를 재검사한다. |
| 10 | runtime 계산이 아직 없거나 비어 있을 때 이전 GPU buffer가 화면에 남는가 | `draw(None)`도 clear pass를 제출한다. 빈 장면 readback은 opaque black clear 값을 확인한다. |
| 11 | 중복 node·잘못된 preorder·비유한/음수 frame이 일부만 그려지는가 | typed scene 생성과 geometry 변환이 전체 입력을 검증하고 부분 장면 대신 실패한다. 관련 단위·fixture 비교 시험이 있다. |
| 12 | 숨김·transparent·부분 alpha 값이 잘못된 paint로 그려지는가 | 숨김/0 면적은 렌더 box에서 빠지고 transparent는 paint 없음이다. 부분 alpha와 허용 밖 paint는 실패한다. |
| 13 | surface format fallback에서 색을 이중 sRGB 변환하거나 지원 안 되는 조합을 허용하는가 | sRGB texture와 sRGB colorspace UNORM fallback을 구분한다. 형식 선택·인코딩 단위 시험이 있고 호환 형식이 없으면 초기화 오류다. |
| 14 | WGPU surface가 Lost/Outdated/Timeout/Occluded/Validation을 정상 제출로 보고하는가 | 각 acquisition 결과를 오류로 변환하고 present 성공으로 기록하지 않는다. 실제 driver fault 복구는 이 시뮬레이터 증거의 범위가 아니다. |
| 15 | 실패 주입 함수가 제품 기본 빌드 ABI에 노출되거나 여러 draw에서 계속 실패하는가 | C ABI·Rust hook은 `c04-runtime-gpu-test-hooks` 빌드에만 포함되고 atomic one-shot으로 한 draw에서 소비된다. 기본 feature workspace 테스트와 test-hook feature 테스트를 분리 실행했다. |
| 16 | 환경·표시 요청 폭주가 callback closure를 무한히 쌓거나 최신 canonical 상태를 놓치는가 | Java·Swift lane은 실행 1개와 dirty marker/후속 drain 1개로 제한한다. 단일 생산자 100,000건 및 8개 생산자×10,000건 JVM/Swift 시험에서 bound와 마지막 상태를 확인했다. |
| 17 | 작업 throw/rejection/close가 lane을 영구 scheduled 상태로 남기거나 종료 뒤 작업을 실행하는가 | Java 예외·executor rejection 및 Swift throw 뒤 state 복구/오류 전달을 lane 시험에서 확인했다. close 뒤 pending은 폐기하고 후속 admission은 차단한다. |
| 18 | Android `surfaceDestroyed`가 이전 native surface 사용이 끝나기 전에 반환하는가 | callback은 render lane에 surface 무효화를 요청하고 destroy barrier가 풀릴 때까지 기다린다. API 37 emulator lifecycle 로그와 10,000 pending draw shutdown 실행을 기록했다. 이 UI callback 대기는 의도된 단기 drain이며 무제한으로 정상 처리된다고 주장하지 않는다. |
| 19 | Android Activity 종료가 V8 host나 SurfaceView callback을 render queue 종료 전에 해제하는가 | runtime lane을 닫고 runtime 작업 이후 renderer queue 정리 작업을 직렬화한다. renderer/host 해제·barrier 처리 뒤 executor 종료를 기다리고 callback을 제거한다. pending draw drop와 drain 로그가 있다. |
| 20 | iOS background queue가 UIKit layer를 만지거나 resize/configure 중 draw가 새 generation을 통과하는가 | UIKit surface 준비·configure는 main queue precondition을 둔다. 장치 초기화/draw는 serial render queue에 두며 resize는 draw를 막고 barrier 뒤 configure 후 generation을 재확인한다. 종료는 scene을 무효화하고 lane을 닫은 후 runtime→render 순서로 해제한다. iPhone 17 Pro/iOS 26.2 Simulator 실행 근거가 있다. |

## 검토 중 발견해 고친 부분

전체 테스트에서 Android renderer reconcile의 이전 함수명(`ensureRenderer`/`recreateRenderer`)과 iOS shutdown의 직접 FFI 호출을 가정한 두 source-contract assertion이 refactor 뒤 낡아 있었다. 현재 `reconcileRenderState`의 renderer generation·stale admission을 검사하고, `beginShutdown`의 무효화 호출 순서를 검사하도록 테스트를 고쳤다. 구현 동작을 옛 이름에 맞춰 되돌리지는 않았다.

500줄을 넘던 수정 Rust 모듈도 책임별로 나눴다. WGPU backend/format helper와 renderer 단위 시험을 별도 하위 모듈로 옮겨 `renderer.rs`를 474줄로 줄였고, runtime layout snapshot 대기 API는 `ua_cascade/runtime_layout.rs`로 옮겼다. paint profile별 배경 처리도 cascade 본문에서 `runtime_paint.rs`로 이동했다. 추출 뒤 workspace/test-hook 테스트, Clippy, Android/iOS 빌드를 다시 실행했다.

## 실행 근거

- `mise exec -- bun run test` 통과: JS, CSS reference 23개, Android raw-touch 분석, 전체 기본 Rust workspace.
- `mise exec -- cargo test --locked --workspace --features spinon-ffi/c04-runtime-gpu-test-hooks` 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets --features spinon-ffi/c04-runtime-gpu-test-hooks -- -D warnings` 통과.
- `mise exec -- bun run test:c04-queue` 통과: JVM 5개, Swift 4개.
- `mise exec -- cargo fmt --all -- --check`, 빌드 스크립트 `bash -n`, `git diff --check` 통과.
- Android API 37 Debug 및 iPhone 17 Pro / iOS 26.2 Simulator Debug 앱을 draw 실패 시험 hook을 켠 상태로 다시 빌드했다. Simulator 실행 결과와 한계는 [대기열·종료·draw 실패 실행 기록](c04-runtime-css-to-gpu-queue-simulators-2026-10-09.md)에 있다.

실기기·하드웨어 GPU·실제 driver 오류·성능·전체 CSS 동작은 이 검토에서 증명하지 않았다. 이 검토는 PR 전체 검토를 대신하지 않는다.
