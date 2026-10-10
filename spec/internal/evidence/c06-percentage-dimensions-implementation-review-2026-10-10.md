# C06.1 백분율 크기 구현 공격 검토와 실행 근거

- **내부 계약:** [C06.1 백분율 크기 전달](../0037-c06-percentage-dimensions.md), 숫자 계약 버전 `0.1.0` 고정
- **계획:** [C06 값·단위 변환](../../../plan/c06-value-unit-conversion.md)
- **계획 공격:** [계획 검토 기록](c06-value-unit-conversion-plan-review-2026-10-10.md)
- **비교 모델:** [고정 Chromium 관찰값](../../../tests/fixtures/css/references/c06-percentage-dimensions-v1.json)
- **기준 코드:** `origin/main` `e68b0d13` 위의 `feat/css-c06-unit-conversion`
- **V8 기준:** 저장소 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- **판정 범위:** CSS `width`·`height`·`flex-basis` 백분율을 typed Stylo 값에서 Taffy percentage까지 보존하는 내부 구현. CSS 전체 호환, 실기기 성능, 제품 출시 적합성은 판정하지 않는다.

## 구현 검토에서 고친 문서 경계

검토 중 모바일 근거가 22-node Chromium/Rust 수치 fixture와 동일한 표본이라고 읽힐 수 있음을 찾았다. Android·iOS는 공통 3-node V8 fixture를 실행하고 `layout=ready`, viewport root frame, `boxes=3`, WGPU `presented`와 화면을 확인했지만, 로그는 자식별 실제 frame 숫자를 내보내지 않는다. 계획·계약·상태 문구를 고쳐 모바일 화면 검증을 Rust/Chromium 전체 수치 비교와 분리했다. 화면을 수치 계측 결과처럼 주장하지 않는다.

문서 번호 `0037`도 숫자 버전으로 오인될 수 있어 상태 대장을 “출시 전 내부 계약 숫자 버전 `0.1.0-draft` 고정”이라고 명시하고 C06.1 링크를 “문서 ID 0037”로 적었다. 모든 Cargo crate 버전은 `0.1.0`이며 이 작업의 버전 파일 변경은 없다.

## 독립 실패 경로 검토

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 기능 추가가 제품·crate 숫자 버전을 올리는가 | `Cargo.toml` 변경 없음. Spinon workspace crate는 `0.1.0`을 유지하며, 문서 일련번호는 버전과 분리했다. |
| 2 | Stylo의 `50%` fraction을 100으로 나누거나 곱하는가 | typed bridge는 `Percentage(0.5)`를 그대로 보존하고 Taffy에 `Dimension::percent(0.5)`를 전달한다. Chrome/Rust frame 비교와 typed assertion 통과. |
| 3 | 100%보다 큰 값을 100%로 clamp하는가 | `125% → 1.25` typed assertion과 content-box geometry `125×100 CSS px` 통과. |
| 4 | `0%`가 `0px` 또는 `auto`와 합쳐지는가 | typed 값은 `Percentage(0.0)`이고 Taffy layout에서 0×0 frame이다. |
| 5 | `auto`와 absolute CSS px가 새 variant에서 손실되는가 | Stylo typed `Auto`와 `LengthPx` 매핑을 단위 검증했다. 기존 layout projection suite도 통과. |
| 6 | 음수 CSS authored declaration을 내부 음수 percentage로 잘못 허용하는가 | Chrome fixture에서 음수 width/height declaration의 cascade 결과를 관찰했고 Rust oracle은 계산 결과 `auto`를 받는다. |
| 7 | FFI/내부 `LayoutDimension::Percent`에 NaN·무한대·음수를 직접 넣어 통과하는가 | adapter의 finite/nonnegative 검증이 각 값을 `InvalidStyle`로 거부한다. |
| 8 | Stylo가 수식 값을 문자열 px로 직렬화한 뒤 percentage로 오인하는가 | `calc(50% + 10px)` typed variant는 `Unsupported`; 문자열에 기대지 않고 속성·NodeId 오류로 fail-closed한다. |
| 9 | `flex-basis: content` 및 intrinsic sizing keyword를 임의의 percentage/길이로 바꾸는가 | Stylo typed `Content` 및 지원 외 `Size` variant는 `Unsupported`로 전파하고 projection이 거부한다. |
| 10 | width·height percentage가 부모 border-box 전체를 기준으로 삼는가 | content-box 부모에 padding 10px을 둔 fixture가 Chrome과 일치하며 자식 기준은 부모 content size다. |
| 11 | `border-box`와 nonzero border 지원을 과장하는가 | padding이 있는 `border-box` fixture는 비교한다. layout model이 border edge를 보존하지 않으므로 실제 nonzero border는 입력 인벤토리·계약에서 명시적으로 제외했다. |
| 12 | definite containing block에서 height percentage를 width 축으로 해석하는가 | Chrome/Rust fixture에서 height는 부모 content height를 기준으로 비교되고 허용 오차 안에 든다. |
| 13 | indefinite auto-height containing block의 child `% height`를 definite처럼 계산하는가 | Chrome reference의 `auto-parent`/`auto-height-child` 값을 별도 assertion으로 비교했다. 이 profile의 관찰 동작을 회귀 검증한다. |
| 14 | root `100%`가 viewport containing block을 받지 못하거나 root policy를 우회하는가 | HostDocument runtime 경로는 viewport containing block에서 root 100%를 계산한다. 별도로 `MatchViewport` root mismatch는 기존처럼 거부한다. |
| 15 | row Flex basis가 교차축 height를 참조하는가 | `flex-basis: 50%`·`25%`는 definite row main width 120px 기준 Chrome/Rust 및 전용 Taffy assertion을 통과한다. |
| 16 | column Flex basis가 width를 참조하는가 | `flex-basis: 50%`·`25%`는 definite column main height 60px 기준 Chrome/Rust 및 Taffy assertion을 통과한다. |
| 17 | width와 basis가 함께 지정되면 width가 basis를 덮거나 shrink/grow 결과가 틀리는가 | row/column fixture에서 basis와 width/height 충돌을 만들고 Taffy frame을 Chrome reference와 노드별 비교했다. |
| 18 | 계산 스타일 DTO가 다른 document generation/revision과 섞이거나 incremental cache에서 빠지는가 | Rust 비교는 HostDocument source revision과 style/environment revision을 assert한다. 전체 workspace의 C05.4 incremental-vs-full oracle 비교도 새 `ComputedElementStyle` 필드를 포함해 통과했다. |
| 19 | 미지원 percentage margin·padding·gap이 조용히 px로 처리되는가 | 각 속성을 독립 입력해 해당 property 이름을 담은 `UnsupportedComputedValue`로 거부함을 확인했다. |
| 20 | 두 플랫폼 중 하나가 launch route, V8 fixture, 레이아웃 또는 WGPU 제출을 건너뛰는가 | Android API 37 emulator와 iPhone 17 Pro/iOS 26.2 Simulator에서 공통 3-node JS fixture가 `status=0`, `layout=ready`, `boxes=3`, `presented`를 기록했고 두 화면을 저장했다. 모바일 child frame 수치는 보고되지 않으며 이 근거는 시각 smoke test다. |

## Chromium 수치 fixture와 Rust oracle

고정 기준은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport `320×800 CSS px`, DPR 1이다. capture input inventory는 22개 node이며 Rust runtime oracle은 지원 대상 18개 node의 typed dimension과 각 `x`, `y`, `width`, `height`를 CSSOM observation에 비교한다. 각 좌표별 절대 오차는 최대 `0.5 CSS px`다. `calc()`·percentage margin/padding/gap 4개 노드는 Chrome 관찰값만 보유하고 제품 성공 경로에 포함하지 않는다.

- HTML: [percentage-dimensions.html](../../../tests/fixtures/css/c06/percentage-dimensions.html)
- 입력 inventory: [percentage-dimensions-inventory.json](../../../tests/fixtures/css/c06/percentage-dimensions-inventory.json)
- V8 runtime fixture: [runtime-percentage-dimensions.js](../../../tests/fixtures/css/c06/runtime-percentage-dimensions.js)
- oracle 고정성 test: [c06-percentage-dimensions.test.mjs](../../../tools/css-reference/c06-percentage-dimensions.test.mjs)

Chrome의 `getComputedStyle()`는 root `height:100%`를 used value `800px`로 보고하지만 Stylo typed computed value는 `100%`다. 크기 문자열을 강제로 같게 비교하지 않고 typed percentage 종류와 결과 frame을 각각 확인한다. 나머지 fixture property 중 계산 단계가 같은 값은 문자열을 정확히 비교한다.

## Android·iOS Simulator

공통 JavaScript fixture는 viewport를 100% 차지하는 flex root와 각각 width/height/flex-basis 50%인 자식 둘을 만든다. 아래 로그의 viewport CSS 값은 각 simulator의 surface를 반영한다. Android backend는 emulator에서 ANGLE/SwiftShader software GPU이며, 성능 측정이 아니다.

| 플랫폼 | 실행 증거 | 화면 |
|---|---|---|
| Android API 37 emulator `emulator-5554` | [로그](c06-percentage-dimensions/android-api37.log): V8 eval success, `layout=ready`, `boxes=3`, root viewport frame, WGPU present | [캡처](c06-percentage-dimensions/android-api37.png) |
| iPhone 17 Pro / iOS 26.2 Simulator | [로그](c06-percentage-dimensions/ios-26.2.log): V8 eval success, `layout=ready`, `boxes=3`, root viewport frame, WGPU present | [캡처](c06-percentage-dimensions/ios-26.2.png) |

Android 실기기, hardware GPU 성능·호환성, simulator 간 자식별 frame의 수치 동등성은 미검증이다.

## 자동 검증

| 확인 | 결과 |
|---|---|
| `cargo fmt --all -- --check` | 통과 |
| `cargo test --locked --workspace` | 통과, 단위·통합·doc test 전체 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo check --locked -p spinon-ffi --all-features` | 통과 |
| `mise exec -- bun run test:css-reference` | 통과, 29 tests |
| `mise exec -- bun run build:android` + `emulator-5554` install/launch | 통과 |
| `mise exec -- bun run build:ios-sim` + iPhone 17 Pro Simulator install/launch | 통과 |

Android/iOS 앱은 저장소 고정 V8 revision에서 빌드했다. iOS simulator 빌드의 dSYM 단계에서 정적 archive object timestamp 관련 warning이 있었으나 최종 app build와 실행은 성공했다.
