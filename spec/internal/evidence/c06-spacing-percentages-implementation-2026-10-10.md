# C06.2 백분율 spacing 구현 검토와 실행 근거

- **검토 대상:** 현재 `feat/css-c06-unit-conversion` 작업 트리의 C06.2 변경
- **구현 전 계획 검토:** [계획 실패 경로 20개](c06-2-spacing-plan-review-2026-10-10.md). 아래 코드는 계획표를 재사용하지 않고 별도 실패 경로로 확인했다.
- **내부 계약:** [0038 · C06.2 백분율 spacing](../0038-c06-spacing-percentages.md), 숫자 버전 `0.1.0` 유지
- **비교 모델:** [Chrome 154 고정 관찰값](../../../tests/fixtures/css/references/c06-spacing-percentages-v1.json)
- **실행 경계:** Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator. Android 실기기는 이번 작업 범위에 포함하지 않았다.

## 구현에서 고친 점

- 첫 Clippy 실행에서 fixture 코드의 불필요한 borrow 2곳을 지적했다. 이를 제거하고 전체 workspace Clippy를 다시 통과시켰다.
- spacing 비교 테스트가 603줄로 커져 fixture 작성·참조 코드와 검증 케이스를 별도 모듈로 나눴다. 현재 파일은 `cases.rs` 200줄 미만, `fixture.rs` 500줄 미만이다.
- 직접 만든 layout 입력이 non-finite margin, 음수/non-finite padding·gap, 불명확한 basis 또는 root gap을 Taffy까지 넘기지 않는지 검사하는 케이스를 추가했다. finite percentage라도 최종 frame이 overflow하면 결과를 반환하지 않는 검사도 추가했다.
- 71개 node 비교의 실제 최댓값을 출력하도록 oracle assertion을 보강했다. 이번 실행에서 최대 절대 오차는 `0.01953125 CSS px`다.

## 구현 코드에 대한 독립 실패 경로 20개

| # | 코드 공격 관점 | 확인·판정 근거 |
|---:|---|---|
| 1 | CSS 문자열을 재파싱하며 typed Stylo percentage를 px로 오해하는가 | `ComputedValues` typed 값을 `ComputedCssSpacingValue`로 뽑고 `ComputedCssSpacingValue` assertion 및 71-node frame 비교가 통과했다. [`spacing.rs`](../../../crates/spinon-style/src/stylo_dom/cascade/incremental/spacing.rs), [`cases.rs`](../../../crates/spinon-style-to-layout/src/spacing_percentage_tests/cases.rs). |
| 2 | Stylo의 0–1 fraction을 다시 100으로 나누거나 곱하는가 | `10% → 0.1`, custom property `12.5% → 0.125`, Taffy gap geometry를 typed assertion으로 확인했다. |
| 3 | `0%`가 `0px`로 조기 합쳐지는가 | `m-zero` snapshot이 `Percentage(0.0)`인지 직접 assert한다. |
| 4 | 100% 초과 percentage를 1로 clamp하는가 | `m-over100`은 `Percentage(1.25)`이며 Chrome 대비 geometry 허용치 안에 든다. |
| 5 | signed margin을 unsigned 처리하거나 음수 오프셋을 없애는가 | `-5%`가 typed `-0.05`로 남고 네 edge fixture 좌표와 일치한다. |
| 6 | top/bottom margin을 containing block height로 계산하는가 | containing width 200px, height 120px에서 네 margin edge 결과를 Chrome 78-node fixture와 비교했다. |
| 7 | padding의 세로 edge를 부모 height 기준으로 계산하는가 | 네 padding edge와 `33.333%`·`125%` 값을 포함한 모든 지원 node frame을 비교했다. |
| 8 | 부모 border-box 전체를 child spacing basis로 쓰는가 | `border-box 260px`·padding 30px인 Chrome case의 content width 200px 기준을 비교했다. layout 모델에 없는 nonzero border는 지원 주장에 넣지 않았다. |
| 9 | RTL logical inline edge를 항상 left/right 하나로 고정하는가 | LTR/RTL logical margin·padding case를 같은 71-node 비교에 포함해 physical frame을 확인했다. |
| 10 | 1–4 값 shorthand 확장이나 stylesheet→inline longhand cascade를 놓치는가 | Chrome fixture의 shorthand·longhand override 선언을 Stylo cascade와 Taffy frame까지 통과시켰다. |
| 11 | parse-time invalid 음수 padding/gap을 내부에서 다른 값으로 다시 해석하는가 | authored invalid declaration fixture는 Stylo 계산 결과를 기준으로 71-node geometry를 비교한다. 임의 clamp/default 변환은 없다. |
| 12 | `var()` percentage를 serialization 문자열이나 px로 바꾸는가 | `var(--space)`는 typed `Percentage(0.125)`와 200px 부모에서 25px 오프셋을 assert한다. |
| 13 | `var()` 치환 후 invalid padding/gap이 이전 선언을 잘못 복원하는가 | negative custom-property fixture의 최종 cascade·geometry를 고정 Chrome reference와 대조했다. |
| 14 | gap shorthand 한 값/두 값 및 longhand override의 축 배치가 바뀌는가 | `gap`, `row-gap`, `column-gap` CSSOM observation과 각 지원 frame을 고정 fixture에서 비교했다. |
| 15 | Flex `gap:normal`을 임의 font 단위로 계산하는가 | Stylo `Normal`은 projection에서 0으로 정규화되며 reference layout과 일치한다. |
| 16 | row/column gap percentage를 반대 축에 적용하는가 | definite row `column-gap`은 content width, definite column `row-gap`은 content height로 Taffy geometry를 검사했다. |
| 17 | nowrap row Flex의 교차축 `row-gap`을 형제 사이 간격으로 더하는가 | row Flex cross-axis gap을 갖는 Chrome fixture에서 child frame을 그대로 비교했다. 줄 간격이 없는 경로를 main-axis gap으로 해석하지 않는다. |
| 18 | auto size를 전부 indefinite로 보거나 전부 definite로 간주하는가 | block-flow auto-width row와 definite parent에 stretch된 auto-height column은 definite gap으로 통과하고, cyclic auto-height column gap은 node·property·axis 오류로 거부한다. |
| 19 | root spacing, `margin:auto`, `calc()`가 조용히 0 또는 px로 바뀌는가 | root gap은 `UnsupportedRootPercentageSpacing`; root padding basis는 `IndefinitePercentageBasis`; auto margin·calc는 `UnsupportedComputedValue`로 닫히는 Rust/runtime tests를 확인했다. |
| 20 | 비유한 입력·음수 padding/gap 또는 계산 overflow가 부분 frame으로 반환되는가 | property별 `InvalidStyle` assertion과 `f32::MAX` margin의 `NonFiniteFrame` 검사가 통과한다. 계산된 layout 실패에서 부분 frame output은 반환되지 않는다. |

검토 중 발견한 Clippy 지적과 과대 테스트 파일은 위와 같이 수정했다. 이 검토는 계획 검토 표와 별도이며, 열거된 각 항목은 코드 경로·테스트·고정 oracle 가운데 하나 이상에 연결했다.

## Chromium·Rust 수치 대조

고정 기준은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, `320×800 CSS px`, DPR 1이다. reference는 78개 node다. Rust runtime 성공 비교는 71개 node이며 각 node의 `x`, `y`, `width`, `height`를 독립 비교한다. 허용치는 각 값 `0.5 CSS px`이고 이번 실행 최대 절대 오차는 `0.01953125 CSS px`다. 원본은 [`chromium-geometry-test.log`](c06-spacing-percentages/chromium-geometry-test.log), 전체 고정 observation은 [reference JSON](../../../tests/fixtures/css/references/c06-spacing-percentages-v1.json)이다.

geometry 성공 집계에서 제외한 7개는 `margin:auto` 1개, intrinsic cyclic gap subtree 3개, 현재 profile 바깥의 `inline-flex` oracle-only subtree 3개다. cyclic subtree는 후속 root 좌표 비교가 가능하도록 test harness에서만 같은 20px placeholder로 대체한다. 별도의 실패 assertion이 원래 auto-height column gap을 `IndefinitePercentageBasis`로 거부하는지 확인하므로 placeholder는 제품 동작이나 cyclic gap 지원으로 계산하지 않는다.

## Android·iOS Simulator 실행

양 플랫폼은 같은 7-node JavaScript fixture를 실제 V8 → HostDocument → Stylo → Taffy → WGPU surface 경로로 실행했다. 화면은 같은 CSS viewport fixture를 보여주지만 모바일 자식별 frame 수치 oracle은 아니다.

| 플랫폼 | 런타임 결과 | 원본 근거 |
|---|---|---|
| Android API 37 emulator `emulator-5554` | `SPINON_C062_EVAL status=0`, `layout=ready`, `boxes=7`; renderer draw `status=0 presented boxes=7`. emulator renderer는 ANGLE/SwiftShader software 경로여서 hardware GPU 결과로 해석하지 않는다. | [logcat](c06-spacing-percentages/android-api37.log) · [화면](c06-spacing-percentages/android-api37.png) |
| iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`) | C06.2 eval `status=0`; 후속 environment log와 화면에 `layout=ready`, `boxes=7`; renderer draw `status=0 presented boxes=7`. | [Simulator log](c06-spacing-percentages/ios-26.2.log) · [화면](c06-spacing-percentages/ios-26.2.png) |

iOS unified log는 긴 `SPINON_C062_EVAL` 한 줄 일부를 `<…>`로 자른다. 동일 캡처의 다음 environment/draw 기록과 화면 상태를 함께 남겼다. 앱 최초 종료 시도는 실행 중인 이전 프로세스가 없다는 응답이었고, 새 프로세스 launch·eval·draw는 성공했다. 실기기·hardware GPU·성능은 검증하지 않았다.

## 자동 검사와 빌드

| 명령 | 결과 |
|---|---|
| `cargo test --locked --workspace` | 통과. 전체 crate test 성공, 기존 ignored test 2개는 미실행. [`workspace-tests.log`](c06-spacing-percentages/workspace-tests.log) |
| `mise exec -- bun run test:css-reference` | 33 통과, 0 실패. [`css-reference-tests.log`](c06-spacing-percentages/css-reference-tests.log) |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android` | Android ARM64 release library·debug APK 빌드 성공; API 37 emulator에 설치·실행했다. |
| `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim` | iOS Simulator 앱 빌드 성공; iPhone 17 Pro / iOS 26.2 Simulator에 설치·실행했다. |

Android Gradle은 compile SDK `37.2`와 AGP `8.13.2` 지원 표 경고를 출력했으며 SDK의 NDK `27.1.12297006`을 사용했다. iOS V8 archive dSYM 단계는 archive object의 동일 timestamp warning을 냈지만 앱 빌드·실행은 성공했다. 이 경고는 테스트 결과로 숨기지 않고 기록한다.

## 미검증 경계

Android 실기기, 실제 hardware GPU, 다른 Android API/기기 행렬, 모바일 child frame 수치 동등성, wrap/Grid/inline-flex, 외부 CSS 로더, CSSOM, CSS math, text measurement와 성능은 이 결과로 검증하지 않았다. 이 C06.2 실행 기록은 C06.3 절대 길이 단위 변환을 검증하지 않는다. C06 상위와 C06.4–C06.6은 미완료다.
