# C11.4 · `fit-content()` automatic-minimum floor WPT slice 계획

**상태:** 두 고정 scene의 opt-in 앱 runtime slice를 구현했고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 검증을 기록했다. 세부 결과와 미검증 경계는 [runtime evidence](../spec/internal/evidence/c11-4-fit-content-auto-min-floor-runtime-2026-10-28/README.md)에 있다. 이 제한 slice는 C11.4 완료를 뜻하지 않으며 공식 완료 `72/213`을 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 범위

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 세 번째 assertion을 제한된 C11.4 CSS→HostDocument/Stylo→typed Grid input→Taffy→Android/iOS 앱 fixture 경로에서 검증한다. `fit-content(40px)`의 argument보다 큰 grid item automatic minimum이 track의 하한으로 남는 동작을, `min-height:0` 대조 입력과 구분한다.

이 작업은 CSS Grid `fit-content()` 전체 지원이나 일반 CSS admission이 아니다. `ParsedRootHtml`, color scheme, media query, CSSOM, 전체 WPT suite, arbitrary Grid trees, glyph shaping, 실기기/hardware GPU는 범위에 넣지 않는다. 런타임 경로에서 규범 동작과 차이가 발견되면 fixture만 기대값에 맞추지 않고 원인을 추적한다. 공통 레이아웃 구현 수정이 필요하면 회귀 범위와 기존 제한 profile에 미치는 영향을 계획에 추가해 검증한다.

## 고정 출처와 관찰

- WPT revision: `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- 원본: [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html), SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- 대상 DOM은 원본 31–34행, assertion은 90–92행이다. 원본은 300px 높이 Grid, `fit-content(40px)` implicit rows, 10px와 100px nested blocks를 갖는다. 두 grid item은 기본 `min-height:auto`, `overflow:visible`이다.
- `contentLargerThanArgument`의 `min-height:0` 사례는 원본 36–38행·94–96행이다. 이번 작업에서는 자동 최소 크기 기여를 분리하기 위한 음성 대조군으로만 사용한다.
- Agent Browser의 HeadlessChrome 156 실행에서 upstream harness `OK`, 13 tests / 13 Pass, 대상 `grid-template-rows: 10px 100px`, root height `300px`를 관찰했다. 이 실행은 Chrome의 upstream WPT 결과이며 Spinon WPT 통과가 아니다. 고정 checkout과 `wpt.live` source SHA-256도 일치했다. 캡처와 기계 판독 기대값은 [사전 비교 자료](../spec/internal/evidence/c11-4-fit-content-auto-min-floor-precomparison-2026-10-28/README.md)에 둔다.

## 독립 기대값

CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)는 `fit-content(limit)`의 최소 크기가 auto minimum이며, 공식은 `max(minimum, min(limit, max-content))`라고 정의한다. [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)는 auto min track에 놓인 non-spanning item의 minimum contribution을 track base sizing에 사용한다. [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)는 scroll container가 아닌 Grid item의 automatic minimum을 설명한다.

WPT main input의 `fit-content(40px)` implicit rows는 다음 값을 내야 한다.

| 행 | item auto minimum / minimum contribution | max-content contribution | argument | 예상 track |
| --- | ---: | ---: | ---: | ---: |
| 첫 번째 | 10px | 10px | 40px | 10px |
| 두 번째 | 100px | 100px | 40px | 100px |

따라서 root used height는 `300px`, Rows는 `[10, 100]px`이다. 남은 block-axis 공간은 이 fit-content rows를 늘리지 않는다. 대조 입력은 두 번째 item에 `min-height:0`을 지정하고 content 100px을 유지한다. item minimum contribution은 0px이 되고 fit-content cap이 적용되어 두 번째 row는 40px이어야 한다. 대조군은 바닥의 원인이 automatic minimum임을 확인하며, 해당 WPT assertion 전체의 독립 통과로 세지 않는다.

## 입력과 구현 경계

구현된 opt-in fixture는 다음 두 고정 scene만 받는다.

1. `fit-content-auto-min-floor-wpt`: WPT DOM을 유지하며 `box-sizing:border-box; width:60px; height:300px; margin:0; padding:0; border:0; grid-template-columns:60px; grid-template-rows:none; grid-auto-rows:fit-content(40px); align-content:normal; row-gap:0; column-gap:0`을 사용한다. root width는 deterministic frame을 위해 60px로 고정한다. item 두 개와 각각의 10px/100px child를 두고, item의 `min-height:auto`와 `overflow:visible`을 유지한다. 예상 Rows `[10,100]px`.
2. `fit-content-auto-min-floor-zero-control`: 같은 root, alignment, DOM topology, content, track list에서 두 번째 item만 `min-height:0`으로 바꾼다. 예상 Rows `[10,40]px`; 100px child overflow는 별도 표면 sample로 확인한다.

두 scene은 기존 `c04-runtime-gpu-fixture-test` 아래의 전용 fixture route에 둔다. source allowlist는 exact scene ID만 받으며 altered styles, 다른 node topology, unknown ID를 거부한다. computed style, typed Grid input, node identity, frame과 track trace는 동일한 document/style/environment revision이어야 한다. fixture ID·문자열·심볼·payload는 default 및 product artifact에서 제외되고 opt-in positive control에만 포함되어야 한다.

Root·item의 시각 표시는 고정 단색 배경과 1px 경계만 사용해 track 경계를 구분한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 각 scene의 10px·100px/40px row 끝, root 아래 빈 영역, control의 100px overflow 표본을 CSS px→device px 변환과 함께 확인한다. 이 표면 검사는 geometry/clip 경계 보조 근거이며 glyph paint·하드웨어 표시 완료 증거가 아니다.

## 수용 기준

1. WPT source revision/hash와 Agent Browser 관찰, 독립 CSS Grid 계산을 expected JSON·README에 고정한다. Upstream pass와 Spinon 결과를 구분한다.
2. Rust layout oracle에서 main `[10,100]` 및 control `[10,40]`의 Rows stage와 root/item/child frames를 확인한다. 기대값은 Taffy output으로 역산하지 않는다.
3. 제한 fixture source/parser/cascade route에서 exact DOM·style·IDs를 확인하고 변형 입력은 fail-closed한다. Unknown scene은 source를 반환하지 않는다.
4. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 V8→HostDocument→Stylo→typed Grid→Taffy→WGPU 경로를 실행한다. Main scene root frame은 `[0,0,60,300]`, Rows `[10,100]`, item y/height는 `0/10`, `10/100`, child y/height도 `0/10`, `10/100`이다. Control scene Rows는 `[10,40]`, 두 번째 item은 y/height `10/40`, child는 y/height `10/100`이며 track 아래로 60px overflow한다. Frame·track trace·revision·draw/present를 각 scene에 결합한다.
5. Android PixelCopy와 iOS 제출 전 Metal drawable 표본이 계획된 CSS-pixel 영역에서 기대 색과 일치한다. 캡처 불가·빈 화면·stale revision은 실패다. Simulator 결과를 실기기/hardware GPU 또는 iOS compositor 이후 pixel로 확대하지 않는다.
6. default/product archive에서는 신규 fixture marker·symbol·payload가 없고 fixture-test archive에는 positive control로 포함된다. `cargo fmt`, 관련 focused Rust/FFI 테스트, artifact boundary 검사, simulator 앱 build/run 결과를 기록한다.
7. 계획 자체를 서로 다른 실패 관점 20개로 검토한다. 구현 후 새 실패 관점 20개를 추가로 검토하고, 각 PR은 프로젝트 규칙에 따라 별도 30개 diff 관점으로 검토한다. PR review 관점은 이 계획 review를 재사용하지 않는다.

## 범위 밖과 완료 상태

이 slice는 나머지 12개 WPT assertion 전체의 runtime coverage, WPT suite 내 Spinon pass, CSS Grid 일반 지원, 다른 track sizing 함수, 임의 DOM/Grid, production `ParsedRootHtml`, CSS environment parity, 실기기/hardware GPU 및 C11.4 전체 완료를 주장하지 않는다. 계획·구현 PR은 `spec/STATUS.md`의 C11.4 unchecked 상태와 공식 완료 `72/213`을 유지한다. API 명세와 일반 CSS 지원을 새로 선언하지 않는다.
