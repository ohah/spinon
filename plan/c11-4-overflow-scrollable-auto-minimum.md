# C11.4 · Grid item automatic minimum과 scrollable overflow

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md) · [fixture](../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-precomparison.html) · [기대값 JSON](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-overflow-scrollable-auto-minimum-plan-review-20.md)

**계획 PR diff 검토:** [별도 실패 관점 20개](./c11-4-overflow-scrollable-auto-minimum-pr-diff-review-20.md)

## 결손과 목표

상위 [C11.4 계획](./c11-4-grid-track-sizing.md)의 남은 implementation controls에는 unsupported overflow state가 있다. 현재 `spinon-layout`은 `LayoutOverflow::{Visible, Clip, Hidden, Scroll}`를 vendored Taffy `0.15.0`에 각각 전달하며 `Auto`는 scrollbar geometry가 정해질 때까지 `Hidden`으로 매핑한다. Taffy Grid track sizing은 `Visible`/`Clip`에서 content-based automatic minimum을 허용하고 `Hidden`/`Scroll`에서 0 minimum을 사용한다. 이 C11.4 경계에 Ahem text Columns 입력과 natural replaced-image Rows 입력을 넣어 computed track, item frame, minimum-contribution trace를 고정하는 direct Rust regression coverage가 없다. 기존 replaced-image oracle의 `Scroll` 사례만으로는 CSS text min-content 경로, `Clip` 분기 또는 `Auto` adapter mapping을 검증할 수 없다.

이 slice는 automatic-minimum 알고리즘과 Taffy adapter mapping을 검증하는 **Rust layout test 전용** 변경이다. 일반 CSS `overflow` parsing/admission, runtime clipping·scrollbars, scroll state, 앱 fixture ABI, 새 API 또는 C13 동작을 추가하지 않는다. 테스트가 기존 구현의 불일치를 찾으면 관측된 차이에 맞는 최소한의 layout adapter 수정만 별도로 계획하고, 제품 CSS 동작을 넓히지 않는다.

## 규범과 고정 비교 입력

[CSS Grid §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)은 조건을 만족한 Grid item의 automatic minimum을 overflow가 scrollable value인지에 따라 분기한다. [CSS Overflow §3](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties)는 `clip`과 `visible`을 non-scrollable로, `hidden`, `auto`, `scroll`을 scrollable로 구분한다. CSS Grid 알고리즘에는 한 축에 `minmax(auto,1fr)` 하나만 두어 자동 최소 크기를 허용하고, 1-track span으로 multi-track 예외를 피한다.

[사전 비교](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md)는 Ahem text로 Columns의 min-content 폭 100px과 60px Grid를, CSS width를 10px로 고정한 natural 10×10 replaced image로 Rows의 min-content 높이 10px과 5px Grid를 만든다. Independent expected values와 Chrome 156 observation은 각각 [expected.json](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)과 [관측 JSON](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/chrome156-observation.json)에 분리했다.

| 축·item overflow | 기대 track / item | 추적할 minimum 분기 |
| --- | --- | --- |
| Columns `visible`, `clip` | `100px`; Grid width `60px` | content-based automatic minimum `100px` |
| Columns `hidden`, `auto`, `scroll` | `60px`; Grid width `60px` | overflow automatic minimum `0px` |
| Rows `visible`, `clip` | track/item `10px`; Grid height `5px` | content-based automatic minimum `10px` |
| Rows `hidden`, `auto`, `scroll` | track `5px`, item `10×5px`; Grid height `5px` | overflow automatic minimum `0px` |
| 명시적 item min 0 | Columns track `60px`; Rows track `5px` | specified minimum `0px` control |
| `minmax(0,1fr)` track | Columns `60px`; Rows `5px` | fixed track minimum; item minimum contribution is not queried |

## 구현 범위와 판정

- `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests.rs`의 Ahem text Columns case와 `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests/row_axis_replaced.rs`의 natural-image Rows case에 총 14개 fixed input을 둔다. Text case는 기존 `Tree`/`LayoutInput`/`ParagraphInput`/`TextLayoutEngine` 경로를, replaced case는 `HostDocument`/`LayoutIntrinsicSizeSnapshot` 경로를 사용한다. Taffy가 row text layout 높이와 item frame이 다르면 `TextLayoutHeightMismatch`를 반환하므로, Rows에서는 image를 써 C15 text-layout 경계를 유지한다. Image의 CSS width는 natural width인 10px로 고정해 aspect ratio가 row minimum과 width 결과를 혼합하지 않게 하고, UA 기본 `overflow:clip`을 덮도록 baseline `visible`을 명시한다.
- 각 case의 root·track·item frame을 규범 기대값과 비교한다. Trace에서는 대상 axis와 item owner를 확인하고, 질의된 `AutomaticMinimum { content_based: true }`, `OverflowAutomaticMinimum`, `SpecifiedMinimum`을 value와 함께 구분한다. `minmax(0,1fr)` control은 fixed track minimum이 item auto-minimum query를 막으므로 Minimum contribution이 없어야 한다. Track base와 최종 frame만 같아도 잘못된 분기가 가려지면 실패해야 한다.
- Controls는 overflow를 visible로 고정해 item `min-width/height:0`와 track `minmax(0,1fr)`가 서로 다른 경계로 시험되게 한다.
- `to_taffy_overflow`의 `Auto -> Hidden` mapping과 vendored Taffy `0.15.0`의 관련 분기를 대조한다. vendor source 또는 CSS parser는 실패 원인이 확인되지 않으면 수정하지 않는다.
- scope가 순수 layout calculation unit tests이므로 앱 UI/Android/iOS simulator 결과를 주장하지 않는다. Android/iOS runtime 검증은 이번 계획의 acceptance gate가 아니다.
- 일반 CSS/root HTML에서 해당 overflow 값을 받아들이지 않는 경계는 유지한다. 실제 clipping, scroll offset, scrollbar, compositor pixels는 별도 C13 계획에 남긴다.
- C11.4 checkbox와 공식 완료 `72/213`은 변경하지 않는다.

## 구현 전 검토 및 구현 후 판정

계획 검토는 [20개 독립 실패 관점](./c11-4-overflow-scrollable-auto-minimum-plan-review-20.md)으로 마친 뒤 구현에 들어간다. 구현 시 목표는 새 Rust module의 이름 필터 test, `cargo fmt --all -- --check`, `cargo clippy -p spinon-layout --all-targets -- -D warnings`이다. 수정 범위가 달라지면 선택된 focused gate를 갱신하고, 전체 workspace CI나 simulator 실행 결과로 확대하지 않는다.

구현 코드가 생긴 뒤에는 계획 검토와 재사용하지 않는 새로운 구현/runtime 실패 관점 20개를 기록한다. PR diff는 사용자 요청에 따라 별도 20개 관점으로 검토한다. 테스트 fixture만으로 CSS parsing, UI clipping 또는 전체 C11.4를 통과했다고 표현하지 않는다.

## 구현 실행 기록 · 2026-10-28

- 새 `#[cfg(test)]` 모듈에 Columns Ahem text 7개와 Rows intrinsic image 7개를 추가했다. 최종 track base, root/item frame, item owner에 대한 Minimum contribution source/value를 검사한다. fixed zero track minimum에서는 Minimum query가 없음을 assert한다.
- 첫 실행에서 zero-track control의 contribution query가 없다는 점을 바로잡고, Rows text path의 `TextLayoutHeightMismatch`를 피하도록 replaced image path로 분리했다. 이어진 실행에서 image aspect ratio가 auto width를 5px까지 줄이는 축 coupling과 `<img>` UA 기본 `overflow:clip`도 발견했다. CSS/test 양쪽에서 image width를 10px로 고정하고 browser fixture의 visible baseline을 명시해 각각 재검증했다. 최초 계획 및 PR diff 검토에서 놓친 이 조건은 해당 기록에 덧붙였다.
- Agent Browser 0.31.1 / HeadlessChrome 156, macOS arm64, viewport `1280×633`, DPR 1, Ahem loaded에서 fixture·기대값·관측값 14개 ID, grid/track/item geometry, overflow computed value가 모두 일치했다. 새 screenshot·observation·metadata와 SHA-256 manifest를 저장했다. 전용 session을 닫은 뒤 active session 없음, `pid=null`, browser process 없음으로 확인했다.

| Gate | 결과 |
| --- | --- |
| `cargo test -p spinon-layout auto_minimum_follows_overflow_scrollability -- --nocapture` | 2 passed |
| `cargo test -p spinon-layout` | 295 passed, 0 failed; doc-tests 0 |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy -p spinon-layout --all-targets -- -D warnings` | 통과. 최초 실행의 `match_like_matches_macro` 지적을 수정한 뒤 재실행했다. |
| 구현/runtime 실패 관점 검토 | [20개 관점](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/implementation-review-20.md) |
| 구현 PR diff 실패 관점 검토 | [별도 20개 관점](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/pr-diff-review-20.md) |

변경은 `spinon-layout` Rust unit tests와 비교 자료에 한정한다. 앱 route, FFI, CSS parser 및 제품 runtime을 바꾸지 않아 Android/iOS simulator 검증은 수행하지 않았다. `spec/STATUS.md` C11.4 checkbox는 미완료로 유지하며 완료 집계는 이 변경으로 올리지 않는다.
