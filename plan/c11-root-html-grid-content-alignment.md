# C11 parsed Root HTML Grid 콘텐츠 정렬 제한 구현 계획

**상태:** 제한 구현·Rust·Android/iOS 시뮬레이터 smoke 검증 완료, PR 전 리뷰 진행 중 · **대상:** `RuntimeRootHtmlTextPaintV1`의 좁은 Grid profile

## 목표와 경계

기본 parsed Root HTML runtime의 Grid container에서 `align-content`와 `justify-content`가 track 집합을 배치하는 동작을 제한 구현한다. 대상은 definite한 고정 크기 container, 비어 있지 않은 고정-px explicit row/column, sparse row auto-flow, implicit track이 필요 없는 direct block item, fixed-px gap, LTR·`horizontal-tb` profile로 제한한다. computed 값을 typed layout까지 전달하고 Taffy 결과의 track 시작점을 비교한다.

Grid item 자체의 `align-items`·`justify-items`, named area/line·repeat, implicit/auto track sizing, spanning/명시 placement, RTL·orthogonal writing mode, baseline content alignment, 일반 CSS admission 및 WPT 적합성은 이 계획의 범위가 아니다. 입력이 대상 profile 밖이면 해당 정렬 선언을 `UnsupportedComputedValue`로 거부해 일부 frame을 공개하지 않는다. Grid 외 요소에서 Root HTML `align-content`를 새로 허용하지 않는다.

`normal`/`stretch`의 auto track 확장 의미는 이번 profile에서 auto track을 제거해 영향이 없게 한다. 일반 Taffy adapter의 Grid `normal` track stretch 의미 변경도 이번 범위에 넣지 않는다. 첫/마지막 baseline은 vendored Taffy adapter가 시작 정렬로 축약하므로 허용하지 않는다.

적격성은 Root HTML Grid container computed snapshot에서 확인한다: non-empty fixed-px rows/columns, fixed finite width/height, sparse row flow, `grid-auto-rows/columns:auto`, no repeat/areas/line names, fixed-px non-negative gaps, zero margin/padding/border, visible overflow, static direct empty `<div>` items with all-auto placement, and item count no greater than explicit cell count. 조건을 하나라도 벗어난 Grid에 non-initial content alignment를 주면 layout 전에 거부한다. 이 profile은 기존 `GridPlacementMode::Existing` 배치 동작을 사용한다.

## 사전 비교

- 기준 구현: Agent Browser `HeadlessChrome/156.0.0.0`, DPR 1, viewport 1280×633, `tests/fixtures/css/c11/root-html-grid-content-alignment.html`.
- 규범 출처: [CSS Grid Layout 1 §11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#align-content), [CSS Box Alignment 3 §5.1.4](https://www.w3.org/TR/css-align-3/#grid-containers).
- 독립 기대값, Chrome 관찰, 캡처와 SHA-256은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-content-alignment-precomparison-2026-11-03/README.md)에 둔다.
- 양의 free space는 고정 track 크기와 gap을 container content size에서 뺀 값으로 계산한다. 위치 정렬은 start/end/center, 분배 정렬은 between/around/evenly의 CSS Box Alignment 공식을 따른다. safe 정렬은 음수 free space일 때 start fallback을 사용한다.
- CSS Grid `space-evenly`의 Chrome 위치는 device pixel이 아닌 CSS pixel 결과로 기록한다. Chrome의 1/64 CSS px 양자화 때문에 독립 수식과의 허용 오차는 1/64 CSS px이다.

## 구현 단계와 판정

1. Grid 전용 Root HTML style projection에서 `align-content`와 `justify-content`를 runtime typed parser로 읽고, 지원 profile 바깥 값은 명시 오류로 거부한다.
2. `normal`, `stretch`, start/end/flex-start/flex-end/center, 세 distributed value 및 사전 관찰한 `safe center`, `unsafe center`, `safe end`를 Taffy에 전달한다. baseline은 축약하지 않고 거부한다.
3. Root HTML Grid style projection에서 exact eligibility gate와 allowlist를 적용한다. 기존 `RootHtmlImplicitRowsV1`·`RootHtmlColumnFlowV1`, named-area, dense placement, fit-content, fixture-only Grid contract는 변경하지 않는다.
4. Projection positive/rejection tests와 Rust layout frame tests를 추가한다. 양 축에서 free-space 양수·음수, 안전 fallback, 분배 값, unsupported baseline/profile controls를 판정한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 고정 fixture 앱을 실행한다. 두 앱의 `layout=ready`, `boxes=7`과 화면 배치를 확인한다. 정확한 CSS frame 비교는 Rust runtime test와 Chrome 사전 비교에서 판정하고, 플랫폼 screenshot은 시각 smoke로만 기록한다. 이 후보에서는 PixelCopy/Metal surface readback을 수행하지 않는다.
6. focused Rust tests, 관련 package Clippy, `cargo fmt --all -- --check`를 실행하고, 구현/runtime review와 PR diff review를 각각 별도로 남긴다.

지원 geometry는 Chrome 156과 최대 1/64 CSS px 이내여야 한다. 명세 기대값과 다르면 차이를 조사하며 oracle에 구현을 맞추기 위해 허용 오차를 넓히지 않는다. 예상 밖 typed value, baseline, unsupported profile은 property·node 맥락을 포함해 layout publication 전에 실패해야 한다.

## 승격 조건

이 제한 slice는 C11.1이나 C11을 완료로 만들지 않는다. 일반 author CSS admission, named area·repeat·implicit track 조합, 실기기 GPU, WPT 및 전체 CSS Grid 결과를 증명하지 않는다. 완료 checkbox와 공식 `72/213`은 API/인터페이스 계약과 필요한 근거가 완성되기 전까지 그대로 둔다.
