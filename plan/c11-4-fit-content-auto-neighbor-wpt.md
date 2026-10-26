# C11.4 · `fit-content()` 옆 `auto` implicit row WPT slice 계획

**상태:** 계획 자체 실패 관점 검토 결과와 PR diff 검토 결과는 각각 별도 evidence 보고서에 기록한다. 이 PR은 계획·사전비교 문서만 추가하며 구현은 시작하지 않는다. C11.4와 공식 완료 집계 `72/213`은 변경하지 않는다.

**기준:** PR 생성 시 최신 `origin/main` `d48d31698842866241dcb3317c6f8ff1ee068d1c`.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**WPT 사전비교·hash:** [Chrome 관측과 산술](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/README.md) · [독립 기대값](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/expected.json) · [WPT source pin](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/source-pins.json)

**계획 실패 관점 검토:** [20개 관점 결과](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/plan-review-20.md)

## 목적과 경계

기존 C11.4 pure Rust oracle은 `align-content:normal`에서 모두 `auto`인 row와 고정 row가 함께 있을 때의 `StretchAutoTracks` stage를 확인한다. 기존 fit-content oracle은 cap/floor, percent/calc, text/image contribution을 별도 입력으로 확인하지만, `fit-content()` row가 `auto` row와 나란히 있을 때 definite free space를 어느 track이 받는지 직접 검사하지 않는다. 이 계획은 WPT `fit-content-track-sizing.html`의 마지막 assertion 하나를 별도 test-only slice로 추가한다.

범위는 `grid-auto-rows: fit-content(40px) auto`, definite 300px block size, 10px 높이 content가 있는 두 item, 기본 `align-content: normal`이다. 40px cap의 growth limit이 아니라 `auto` 인접 track의 CSS Grid Level 2 2025 CRD §12.8 stretch eligibility를 검증한다. WPT 파일 안의 다른 cap/floor, percentage/calc, spanning, zoom, implicit column 및 단독 auto-row assertion은 포함하지 않는다.

## 고정 출처와 규범 기대값

WPT checkout은 revision `c999c58338ee1d223df5ad62e48be1202ced0d37`이고, source file SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. [원본 lines 130–132](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L130-L132)는 `autoRowAndFitContentRow`의 `grid-auto-rows: fit-content(40px) auto`에 대해 `10px 290px`을 단언한다.

독립 기대값은 [CSS Grid Layout Module Level 2, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#min-size-auto), [§12.2](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-terms), [§12.4](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-init), [§12.5](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-content), [§12.6](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-grow-tracks), [§12.7](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-flex-tracks), [§12.8](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-stretch)에서 도출한다. §12.2는 `fit-content()` track의 min sizing function을 `auto`로, max sizing function을 `max-content`로 정의한다. §6.6에 따라 computed overflow가 scrollable하지 않고 축에서 min sizing function이 `auto`인 track 하나 이상을 span하는 item의 automatic minimum은 content-based다. WPT wrapper의 기본 overflow는 `visible`이고 각 item은 한 implicit row만 span한다. 따라서 두 nested 10px child가 item의 content size suggestion과 minimum contribution을 각각 10px로 만든다. §12.4는 두 auto min track base를 0px로 초기화한다. definite sizing의 §12.5에서 각 single-span item의 10px minimum contribution이 base를 `[10,10]px`로 올린다. §12.5에서 두 track의 max-content growth limits는 `[10,10]px`이며, 첫 fit-content track은 argument로 `min(10,40)=10px`에 clamp된다. §12.8의 stretch eligibility에서는 fit-content max function이 `auto`가 아니므로 제외되고 두 번째 `auto` row만 대상이다.

§12.6 `Maximize Tracks`에서 두 base가 각자 10px growth limit에 도달해 그대로 `[10,10]px`다. flexible track은 없어 §12.7 뒤에도 bases는 `[10,10]px`다. row gap, padding, border가 0이므로 §12.8 직전 free space는 `300 - 10 - 10 = 280px`; 둘째 `auto` row만 이를 받아 최종 bases `[10,290]px`가 된다.

```text
grid content height                   = 300px
§12.4 initial bases                 = [0px, 0px]
§12.5 minimum contributions          = [10px, 10px]
§12.5 growth limits                   = [10px, 10px]
§12.6 / §12.7 bases                   = [10px, 10px]
row-gap + padding + border            = 0px
§12.8 positive definite free space    = 300 - 10 - 10 = 280px
stretch-eligible row indexes          = [1]
expected §12.8 post-stage bases        = [10px, 290px]
expected stage                        = StretchAutoTracks { applied: true }
```

단위 테스트의 root width는 유일한 60px column과 같은 60px로 고정해 block-axis만 검사한다. WPT 원본은 block-level Grid의 inline width를 지정하지 않아 containing block 폭을 사용한다. 이 차이는 row sizing 입력에 영향을 주지 않는다. frame oracle은 root `(0,0,60,300)`, 첫 item `(0,0,60,10)`, 둘째 item `(0,10,60,290)`, 두 nested content box `(0,0,60,10)` 및 `(0,10,60,10)`이다. Taffy/Spinon 결과를 보고 기대값을 조정하지 않는다.

## 제안 test 경계

새 pure Rust 단위 테스트 파일은 `crates/spinon-layout/src/grid_fit_content_auto_neighbor_oracle_tests.rs`다. 테스트 이름은 `css_grid_2025_fit_content_row_leaves_space_for_neighbor_auto_row`로 제안한다. 이를 `crates/spinon-layout/src/lib.rs`에서 `#[cfg(test)] mod grid_fit_content_auto_neighbor_oracle_tests;` 한 줄로 등록한다. 기존 `grid_stretch_auto_tracks_oracle_tests.rs`는 수정하지 않는다. 그 파일은 균등 stretch의 `[auto, auto, fixed]`와 normal control을 이미 소유하므로, 새 파일은 fit-content/auto 혼합의 WPT 대응 입력과 trace를 소유한다.

입력 경계:

- `GridContainerInput.rows`는 빈 목록으로 두고 `auto_rows = GridAutoRows::TrackList([FitContent(40px), Auto])`로 implicit rows를 만든다.
- `columns = [LengthPx(60)]`, root width 60px, height 300px, zero row/column gap, zero padding/border, `LayoutAlignContent::Normal`, `TrackSizing` profile이다.
- direct grid item은 두 개이며 각 item height는 auto, 자식 block height는 10px fixed다. 이는 WPT의 auto-height wrapper 안 10px nested block 구조와 contribution/stretch frame을 구분한다.
- layout trace가 root grid node에 귀속되고 row pass가 정확히 하나인지 확인한다. 그 pass에서 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서를 확인하고, `StretchAutoTracks`는 한 번만 있으며 `applied:true`여야 한다.
- `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`의 CSS row bases가 각각 `[10,10]px`인지 확인한다. `StretchAutoTracks` 뒤에는 `[10,290]px`인지 확인한다. gutter/collapsed sentinel은 CSS track 수에서 제외하고 별도 `is_gutter`/`is_collapsed` 조건을 검사한다.
- root 및 두 item/nested item frame을 위의 독립 기대값과 `1e-4 CSS px` 절대 허용치로 비교한다. WPT CSSOM 문자열 expectation은 `10px 290px`이다.

현재 `GridAutoRows::TrackList`와 `FitContent`/`Auto` typed track 표현은 `TrackSizing` profile에 존재한다. 구현 전에 adapter가 두 순서의 실제 implicit tracks를 만들고 CSS order를 보존하는지 테스트 대상에서 확인한다. 실패하면 기대값을 바꾸지 말고 입력 mapping 또는 알고리즘 경계를 다시 검토한다.

## 사전 비교와 검토 게이트

Chrome 156 Agent Browser에서 WPT 원본 selected DOM 및 같은 row-axis 입력을 관찰했다. 두 경우 모두 `gridTemplateRows="10px 290px"`, item heights `[10,290]`을 반환했다. WPT original source는 `file://`로 열었기 때문에 `/resources/testharness.js`가 `file:///resources/testharness.js`로 resolve되어 WPT testharness는 실행되지 않았다. 이는 CSSOM/frame precomparison이며 WPT PASS가 아니다. 환경·raw output·screenshot/hash는 [사전비교 evidence](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/README.md)에 고정한다.

계획 자체의 적대적 검토 20개와 이 계획 PR diff 적대 검토 30개는 서로 다른 기록으로 수행한다. 계획 20개 결과는 [plan-review-20.md](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/plan-review-20.md)에 기록한다. 계획 PR diff 30개는 별도 [pr-diff-review-30.md](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26/pr-diff-review-30.md)에 기록하며, plan review 표를 재사용하지 않는다. 구현은 계획 PR 병합 뒤 최신 main에서 새 브랜치를 만들고, 이 계획 review와 다른 실패 관점의 새 20개 runtime/code review를 완료한 뒤에만 시작한다.

문서 내부 링크·WPT source hash 검증과 RSPress `docs:build`를 계획 PR에서 확인한다. 이 문서 작업은 제품 소스, runtime fixture, 내부 API 계약, `spec/STATUS.md`, C11.4 완료 체크를 변경하지 않는다. CI 결과가 비어 있으면 통과로 쓰지 않는다.

## 남은 범위

다른 fit-content caps/floors, intrinsic text/image contribution matrix, spanning item, percent/calc/cyclic basis, auto-repeat, orthogonal writing mode, gutters, negative/overflow spans, axes cross-product, runtime adapter, Android/iOS, rendering/surface sample, WPT harness integration, 전체 C11.4 및 공개 CSS 지원은 미검증·계획 범위 밖이다. 이 한 case를 계획하거나 나중에 구현해도 C11.4 전체 완료 또는 공식 `72/213` 집계를 변경하지 않는다.
