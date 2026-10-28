# C11.4 · Grid item automatic minimum과 scrollable overflow

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md) · [fixture](../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-precomparison.html) · [기대값 JSON](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-overflow-scrollable-auto-minimum-plan-review-20.md)

**계획 PR diff 검토:** [별도 실패 관점 20개](./c11-4-overflow-scrollable-auto-minimum-pr-diff-review-20.md)

## 결손과 목표

상위 [C11.4 계획](./c11-4-grid-track-sizing.md)의 남은 implementation controls에는 unsupported overflow state가 있다. 현재 `spinon-layout`은 `LayoutOverflow::{Visible, Clip, Hidden, Scroll}`를 vendored Taffy `0.15.0`에 각각 전달하며 `Auto`는 scrollbar geometry가 정해질 때까지 `Hidden`으로 매핑한다. Taffy Grid track sizing은 `Visible`/`Clip`에서 content-based automatic minimum을 허용하고 `Hidden`/`Scroll`에서 0 minimum을 사용한다. 이 C11.4 경계에 Ahem text 입력을 넣어 양 axis의 computed track, item frame, minimum-contribution trace를 고정하는 direct Rust regression coverage가 없다. 기존 replaced-image oracle의 `Scroll` 사례만으로는 CSS text min-content 경로, `Clip` 분기 또는 `Auto` adapter mapping을 검증할 수 없다.

이 slice는 automatic-minimum 알고리즘과 Taffy adapter mapping을 검증하는 **Rust layout test 전용** 변경이다. 일반 CSS `overflow` parsing/admission, runtime clipping·scrollbars, scroll state, 앱 fixture ABI, 새 API 또는 C13 동작을 추가하지 않는다. 테스트가 기존 구현의 불일치를 찾으면 관측된 차이에 맞는 최소한의 layout adapter 수정만 별도로 계획하고, 제품 CSS 동작을 넓히지 않는다.

## 규범과 고정 비교 입력

[CSS Grid §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)은 조건을 만족한 Grid item의 automatic minimum을 overflow가 scrollable value인지에 따라 분기한다. [CSS Overflow §3](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties)는 `clip`과 `visible`을 non-scrollable로, `hidden`, `auto`, `scroll`을 scrollable로 구분한다. CSS Grid 알고리즘에는 한 축에 `minmax(auto,1fr)` 하나만 두어 자동 최소 크기를 허용하고, 1-track span으로 multi-track 예외를 피한다.

[사전 비교](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md)의 Ahem 10px fixture는 Columns에서 min-content 폭 100px과 60px Grid를, Rows에서 min-content 높이 10px과 5px Grid를 만든다. 독립 산술과 Chrome 156 값은 각각 [expected.json](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)과 [관측 JSON](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/chrome156-observation.json)에 분리했다.

| 축·item overflow | 기대 track / item | 추적할 minimum 분기 |
| --- | --- | --- |
| Columns `visible`, `clip` | `100px`; Grid width `60px` | content-based automatic minimum `100px` |
| Columns `hidden`, `auto`, `scroll` | `60px`; Grid width `60px` | overflow automatic minimum `0px` |
| Rows `visible`, `clip` | `10px`; Grid height `5px` | content-based automatic minimum `10px` |
| Rows `hidden`, `auto`, `scroll` | `5px`; Grid height `5px` | overflow automatic minimum `0px` |
| 명시적 item min 0 | Columns `60px`; Rows `5px` | specified minimum `0px` control |
| `minmax(0,1fr)` track | Columns `60px`; Rows `5px` | automatic minimum non-content-based `0px` control |

## 구현 범위와 판정

- `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests.rs`에 14개 fixed input을 둔다. 기존 `Tree`/`LayoutInput`/`GridContainerInput`/`ParagraphInput`과 `TextLayoutEngine`에 Ahem을 등록해 실제 text min-content를 제공한다. 테스트 모듈 선언 하나만 `lib.rs`에 추가한다.
- 각 case의 root·track·item frame을 규범 기대값과 비교한다. Trace에서는 대상 axis와 item owner를 확인하고, `AutomaticMinimum { content_based: true }`, `OverflowAutomaticMinimum`, `SpecifiedMinimum`, `AutomaticMinimum { content_based: false }`를 value와 함께 구분한다. 최종 frame만 같아도 잘못된 분기가 가려지면 실패해야 한다.
- Controls는 overflow를 visible로 고정해 item `min-width/height:0`와 track `minmax(0,1fr)`가 서로 다른 경계로 시험되게 한다.
- `to_taffy_overflow`의 `Auto -> Hidden` mapping과 vendored Taffy `0.15.0`의 관련 분기를 대조한다. vendor source 또는 CSS parser는 실패 원인이 확인되지 않으면 수정하지 않는다.
- scope가 순수 layout calculation unit tests이므로 앱 UI/Android/iOS simulator 결과를 주장하지 않는다. Android/iOS runtime 검증은 이번 계획의 acceptance gate가 아니다.
- 일반 CSS/root HTML에서 해당 overflow 값을 받아들이지 않는 경계는 유지한다. 실제 clipping, scroll offset, scrollbar, compositor pixels는 별도 C13 계획에 남긴다.
- C11.4 checkbox와 공식 완료 `72/213`은 변경하지 않는다.

## 구현 전 검토 및 구현 후 판정

계획 검토는 [20개 독립 실패 관점](./c11-4-overflow-scrollable-auto-minimum-plan-review-20.md)으로 마친 뒤 구현에 들어간다. 구현 시 목표는 새 Rust module의 이름 필터 test, `cargo fmt --all -- --check`, `cargo clippy -p spinon-layout --all-targets -- -D warnings`이다. 수정 범위가 달라지면 선택된 focused gate를 갱신하고, 전체 workspace CI나 simulator 실행 결과로 확대하지 않는다.

구현 코드가 생긴 뒤에는 계획 검토와 재사용하지 않는 새로운 구현/runtime 실패 관점 20개를 기록한다. PR diff는 사용자 요청에 따라 별도 20개 관점으로 검토한다. 테스트 fixture만으로 CSS parsing, UI clipping 또는 전체 C11.4를 통과했다고 표현하지 않는다.
