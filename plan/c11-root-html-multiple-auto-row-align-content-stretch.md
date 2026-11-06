# C11.4 parsed Root HTML explicit `align-content:stretch` · 두 implicit auto rows

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [두 implicit auto-row stretch](./c11-root-html-multiple-auto-row-stretch.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §§7.6, 7.6.1, 8.5, 10.5, 11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). §11.8은 관련 축의 content distribution이 `normal` 또는 `stretch`일 때 남은 양의 definite free space를 `auto` max track sizing function을 가진 tracks에 균등 분배한다.

**구현 전 비교:** [HeadlessChrome 157과 독립 기대값](../spec/internal/evidence/c11-root-html-explicit-align-content-stretch-precomparison-2026-11-07/README.md).

**계획 실패 관점 검토:** [20개](./c11-root-html-multiple-auto-row-align-content-stretch-plan-review-20.md). **계획 PR diff 검토:** 계획 PR의 실제 변경분에서 별도 실패 경로 30개를 점검한다. 구현 뒤에는 fresh runtime/code 20개와 구현 PR diff 30개를 각각 따로 검토한다.

## 결손과 목표

병합된 [PR #1231](https://github.com/ohah/spinon/pull/1231)은 fixed 40×40px Grid, 10px explicit row, direct text `A`/`B`가 만든 두 implicit `auto` rows를 `align-content:normal`로 늘리는 정확한 profile만 허용한다. 같은 fixed input에서 author가 `align-content:stretch`를 명시하면 CSS Grid §11.8의 row sizing은 동일해야 하지만 현재 Root HTML admission이 이 값을 거부한다.

기존 `RootHtmlAutoRowStretchV1` profile 안에서 해당 computed value 하나만 추가로 허용한다. `normal` scene은 그대로 유지한다. 새 public API, CSS parser, runtime selector, profile ID 또는 fixture FFI를 추가하지 않는다. Legacy `RootHtmlIntrinsicTrackSizingV1`의 `grid-auto-rows:auto` rejection은 바꾸지 않는다.

## 고정 입력과 독립 기대값

고정 input은 [사전 비교 fixture](../tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html)다. 표시되는 Grid owner는 정확히 하나이고 content box 40×40 CSS px, explicit column 40px 하나, explicit row 10px 하나, `grid-auto-rows:auto`, row auto-flow, gap 0, `align-items:normal`, zero padding/border, visible overflow를 사용한다. Direct child는 source order대로 40×10px fixed empty item 하나, `A` direct text 하나, `B` direct text 하나뿐이다. 두 text item은 auto-sized 10px/10px monospace 한 줄이다. 기존 `normal` fixture에서 바뀐 CSS 입력은 Grid의 `align-content:stretch` 하나다.

독립 기대값은 [expected.json](../spec/internal/evidence/c11-root-html-explicit-align-content-stretch-precomparison-2026-11-07/expected.json)에 고정했다. Explicit row base 10px과 각 implicit row의 10px text minimum contribution으로 initial tracks `[10,10,10]px`를 만든다. Definite 40px block size에는 positive free space 10px이 남으며 두 implicit auto max tracks에 각 5px씩 분배된다. Used rows는 `[10,15,15]px`, item frames는 `[0,0,40,10]`, `[0,10,40,15]`, `[0,25,40,15]`이고 text line counts는 1/1이다. 기존 `normal` control은 같은 결과, `start` control은 rows `[10,10,10]px`와 text frames `[0,10,40,10]`, `[0,20,40,10]`을 유지한다.

## 구현 범위와 거부 경계

- 기본 `RuntimeRootHtmlTextPaintV1` route, Grid owner 정확히 하나, LTR `horizontal-tb`, static direct `<div>` item 세 개, fixed track/size/content와 text grammar 등 #1231의 기존 profile 조건을 그대로 요구한다.
- `align-content:stretch`는 Grid owner의 exact computed value로 허용한다. Existing `normal` positive 경로를 보존하고 두 value가 같은 track sizes, item frames, NodeIds, source order 및 revision identity를 내는지 검증한다.
- Track trace에서 explicit row 1개와 implicit auto rows 2개의 경계, 각 10px minimum contribution 및 `StretchAutoTracks` 단계가 유지되어야 한다. Final used rows는 `[10,15,15]px`다. Trace나 rendering에 computed alignment identity를 보존하는 기존 계약이 있으면 `stretch` 값도 정확히 반영한다.
- `align-content:start`, `center`, `end`, `space-between`, `space-around`, `space-evenly`, baseline alignment, `normal`·`stretch`를 제외한 다른 computed value, altered item/style/track/gap/direction/writing-mode는 계속 reject한다. Admission은 parsed longhand computed `align-content` value만 본다. `place-content` 표기를 별도로 해석하거나 그 syntax 지원을 선언하지 않는다. 실패 시 partial layout, frame, draw command 또는 trace를 publish하지 않고 같은 RuntimeSession의 다음 valid request가 성공해야 한다.
- 기존 single-auto-row, two-auto-row `normal`, min-content-row, fixed-row profiles를 regression으로 고정한다. 새 product API, parser grammar, FFI나 mode를 추가하지 않는다. 기존 opt-in fixture-test boundary를 재사용한다.

## 구현·검증 순서

1. 이 plan과 precomparison만 포함한 계획 PR을 올리고 계획 실패 관점 20개와 실제 plan diff 실패 경로 30개 검토 뒤 Rebase and merge한다.
2. `align-content:stretch` exact computed value admission만 `RootHtmlAutoRowStretchV1` projection gate에 추가한다. Existing auto-track sizing/layout algorithm을 재사용하고 5px을 기대값 상수로 넣지 않는다.
3. Style-to-layout과 RuntimeSession에서 `normal`/`stretch` positive, `start` 및 unsupported-style rejection, previous controls/recovery, explicit/implicit track trace, per-item contribution과 exact frames를 focused tests로 비교한다.
4. Implementation/runtime changes를 fresh 20개 관점으로, 실제 implementation PR diff를 별도 30개 관점으로 검토한다. 결함을 고치면 영향을 받은 Rust·simulator evidence를 다시 실행한다.
5. 동일 fixture SHA와 Chrome 기대값으로 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 layout/draw/present를 확인한다. 화면은 visual smoke이며 geometry authority는 Rust assertions다. Screenshot이 유용하면 구현 PR 본문에 첨부한다.
6. changed-crate Clippy, `cargo fmt --all -- --check`, 기본 release artifact fixture-marker 부재와 opt-in fixture-test positive control을 기록한다. CI check rollup이 없으면 CI green으로 부르지 않는다.
7. 구현 PR 병합 뒤 내부 계약 0059, `spec/STATUS.md`, parent plan, public roadmap과 Tailnet 제공본을 제한 범위·merge SHA·검증 한계와 일치시킨다. Browser 확인은 Agent Browser로 하고 종료 상태를 기록한다.

## 명시적 미포함

이 slice는 arbitrary track/item count, arbitrary content/font/language, nested/inline/replaced content, 추가 shorthand parsing/support, 기타 content distribution values, generic auto-placement, percentage/calc sizing, gap/edge/overflow changes, general CSS Grid admission, WPT conformance, physical-device/hardware GPU parity, C11.4 전체 완료를 구현하지 않는다. C11.4/C11 checkboxes와 공식 `78/214`은 구현이 실제 종료 요건을 충족하기 전까지 변경하지 않는다. Internal contract version `0.1.0`을 유지한다.
