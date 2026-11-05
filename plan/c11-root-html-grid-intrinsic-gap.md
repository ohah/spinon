# C11.1 · parsed Root HTML intrinsic Grid와 고정 px gap

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [intrinsic explicit tracks](./c11-root-html-intrinsic-grid-tracks.md) · [Root HTML Grid gap](./c11-root-html-grid-gap.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 Agent Browser와 독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-gap-precomparison-2026-11-05/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-gap-precomparison.html) · [expected](../spec/internal/evidence/c11-root-html-intrinsic-gap-precomparison-2026-11-05/expected.json) · [Chrome observation](../spec/internal/evidence/c11-root-html-intrinsic-gap-precomparison-2026-11-05/chrome156-observation.json)

**계획 실패 관점:** [20개](./c11-root-html-grid-intrinsic-gap-plan-review-20.md) · **계획 PR diff 적대 검토:** [20개](../spec/internal/evidence/c11-root-html-intrinsic-gap-precomparison-2026-11-05/plan-pr-diff-review-20.md). **구현/runtime 및 구현 PR diff 검토:** 계획 검토와 분리해 각 20개 관점으로 새로 수행한다.

## 결손과 목표

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`은 PR #1167에서 explicit intrinsic track을 연결했지만, `RootHtmlIntrinsicTrackSizingV1` layout contract는 `row-gap`과 `column-gap`이 모두 0인 경우만 허용한다. Root HTML 기본 경로의 일반 Grid gap과 Taffy track-sizing engine은 이미 gap 입력을 표현하므로, 현재 누락은 이 제한 intrinsic profile과 고정 양수 px gap의 결합이다.

이 계획은 `row-gap`·`column-gap`에 finite non-negative fixed CSS px 값만 허용한다. 명시 track의 intrinsic contribution과 definite `fr` free-space 분배에서 gap을 CSS Grid gutter로 반영한다. 새 runtime profile·public API·FFI route·fixture-only 성공 경로는 추가하지 않는다.

## 지원 범위

- 기존 `RootHtmlIntrinsicTrackSizingV1` placement mode와 PR #1167의 fixed positive width/height, zero margin/padding/border, no-area, `grid-auto-flow:row`, supported explicit track 및 item 제약을 유지한다.
- `row-gap`·`column-gap` 각각의 zero 또는 finite non-negative fixed px 값을 허용한다. 값은 기존 `LayoutLengthPercentage::LengthPx` 입력이어야 하고 negative·NaN·infinity는 layout 전에 거부한다.
- cascade에서 `gap` shorthand, `row-gap`, `column-gap`으로 온 값은 동일한 typed layout 경로를 사용한다. 네 positive scene은 zero-gap control, column-only 10px, row-only 10px, 양축 10px shorthand다.
- 고정 scene은 definite `100×100px` Grid, `min-content minmax(0px,1fr)` columns/rows와 fixed `40×10px`, `20×20px` direct block items를 사용한다. Explicit placement와 `align-content:start`/`justify-content:start`로 placement와 stretch를 분리한다.
- CSS Grid 2025-03-26 CRD §11.5–§11.7에 따라 gap을 gutter로 반영한다. fixed 40px/10px intrinsic base는 유지하고, 각 축의 남은 definite free space에서 gutter를 먼저 빼고 `1fr`을 계산한다.
- CSS-invalid negative `gap`은 Stylo cascade 처리와 runtime의 유효하지만 미지원 computed gap rejection을 구분한다. `10%`는 CSS-valid 대조군으로 Chrome에 기록하되 fixed-px-only 후보에서 explicit fail-closed한다.

## 거부 범위

Intrinsic track과 percentage·`calc()` gap 결합, zero/auto/percentage container axis, non-zero margin/padding/border, auto-repeat, named line/area, implicit intrinsic tracks, dense·column flow, arbitrary text/font, 일반 CSS author admission, public CSS support, WPT 전체 적합성과 C11.1/C11 완료는 포함하지 않는다. 별도 positive root 또는 다른 Grid owner의 성공 output이 하나의 rejected intrinsic Grid와 함께 부분 publish되어서는 안 된다.

명시적으로 제한한 유효 computed value는 node/property diagnostic으로 실패해야 한다. CSS-invalid declaration의 cascade fallback을 runtime 구현 rejection으로 잘못 분류하지 않는다. 지원되지 않는 gap을 0으로 대체해 layout success를 반환하거나 일부 frame만 publish하는 처리는 허용하지 않는다.

## Oracle 및 수락 기준

고정 Chrome 156 Agent Browser 관찰과 independent expected JSON은 [precomparison](../spec/internal/evidence/c11-root-html-intrinsic-gap-precomparison-2026-11-05/README.md)에 있다. 각 Grid의 content box는 `100×100px`다. gap 없는 control track은 columns `[40,60]`, rows `[10,90]`; column-gap 10px은 columns `[40,50]`; row-gap 10px은 rows `[10,80]`; 두 gap을 함께 쓰면 두 변화가 동시에 적용된다. Track list, root-relative item frames, 실제 computed gap과 item count를 확인하며 네 positive scene은 Chrome·독립 계산과 exact CSS px로 비교한다.

기본 Root HTML app runtime은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source/style/layout revision으로 네 positive scene을 실행한다. Frame 허용치는 0.5 CSS px 이하이며 track trace와 draw/present를 확인한다. 화면 캡처는 visual smoke로만 분류하고 surface readback을 하지 않으면 PixelCopy/Metal pixel 근거라고 부르지 않는다.

Negative matrix는 percentage/calc gap, non-zero box edge, implicit tracks, non-definite axes, invalid typed numeric gap, multiple Grid atomicity, stale revision과 기존 zero-gap no-gap regression을 포함한다. `spinon-layout` intrinsic trace test에서 gutter/free-space stage를 독립 산술과 대조한다. 변경된 Rust crates의 focused tests, scoped Clippy와 `cargo fmt --all -- --check`를 실행하며 broad suite 실패는 기준 HEAD 결과와 구분한다. Product/default artifact에 fixture 전용 ABI나 payload가 추가되지 않았는지 확인한다.

## 순서와 검토

1. 구현 전에 이 계획을 별도 실패 관점 20개로 검토한다.
2. 계획 PR을 같은 checkout에서 Rebase and merge한 뒤 구현 branch를 latest `origin/main`에서 만든다.
3. Runtime validator, typed gap, track-sizing trace 및 parsed Root HTML fixture를 구현한다. 기존 fixed/percentage/`fr`/fit-content/area/placement/dense/column-flow gap regressions를 유지한다.
4. 구현과 runtime 결과를 새 실패 관점 20개로 검토하고, 실제 변경 PR diff를 앞선 리뷰와 분리해 20개 관점으로 검토한다. 발견한 결함은 고치고 영향을 받은 관점을 재검토한다.
5. Android/iOS simulator build와 app-route 실행 근거를 hash와 함께 보존한다. PR 본문에는 Chrome 및 가능한 simulator screenshot을 넣는다. Agent Browser를 사용한 검증 뒤 해당 session/browser process를 닫는다.
6. PR은 프로젝트 양식과 기존 `enhancement`, `area: layout` labels를 쓰고, merge 후 실제 SHA/check 상태를 다시 확인한다. CI check rollup이 비어 있으면 CI pass라고 기록하지 않는다.
7. Merge 후 내부 계약 0059, `spec/STATUS.md`, C11 계획과 공개 roadmap에 구현 결과·제한을 맞춘다. 이 bounded slice로 C11.1/C11/C11.4 checkbox 또는 공식 전체 완료 집계는 변경하지 않는다.

## 미완료 선언

이 후보는 intrinsic explicit tracks에 대한 고정 px gutter 연결만 추가한다. Percentage/calc gutter, intrinsic implicit tracks, auto-repeat 조합, 전체 C11.4 sizing algorithm, public CSS support와 C11/C11.1 완료 판정을 대신하지 않는다. 내부 계약 0059의 출시 전 숫자 버전 `0.1.0`은 유지한다.
