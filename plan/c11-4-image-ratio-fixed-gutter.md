# C11.4 · fixed span size suggestion clamp와 gutter trace

**상태:** definite preferred-size suggestion의 §6.6 fixed-area clamp와 span 내부 gutter trace를 구현했다. focused Rust·runtime fixture·FFI 검증, 전체 `spinon-layout` 테스트와 workspace Clippy, 기본 artifact 경계, Android API 37 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 두 장면 matrix, 독립 구현 검토 20개, 별도 PR diff 적대 검토 30개를 통과했다. [구현·Simulator 근거](../spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20/README.md) · [구현 검토 20개](../spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20/implementation-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20/pr-diff-review-30.md). PR 생성·required checks·merge와 Tailnet preview 갱신이 남아 있다. C11.4와 C11은 계속 미완료다.

**상위 계획:** [C11.4 Grid track sizing](c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 소스 대조로 수정한 발견

처음에는 `spanned_fixed_track_limit()`이 span 내부 gutter를 빼먹는다고 가정했다. 소스를 다시 대조해 보니 이 가정은 틀렸다. GridTrackVec는 gutter 항목을 실제 track 사이에 보관하고, `track_range_excluding_lines()`는 item 양 끝 line만 제외해 내부 gutter를 포함한다. `GridTrack::gutter(gap)`의 fixed-pixel max 함수도 `definite_value()`로 해석되므로 helper는 내부 fixed gutter를 이미 한 번 합산한다. gutter를 별도로 다시 더하면 결과가 틀어진다.

실제 누락은 같은 파일의 `minimum_contribution()`에서 확인했다. definite preferred size가 있으면 min/max를 적용한 뒤 곧바로 `PreferredSize`를 반환해 §6.6의 all-fixed grid-area maximum clamp를 적용하지 않는다. `width:36px` image가 `minmax(auto,3px) 4px` 두 열과 `3px` gap을 span하면 specified size suggestion은 grid-area limit `3 + 3 + 4 = 10px` 이하이어야 하지만 현재 코드는 `36px`을 반환한다. 기존 content-based automatic minimum과 반대 축 transferred-size 경로는 이미 helper를 호출하며, helper의 span range는 내부 gutter를 포함한다.

CSS Grid Level 1 §6.6은 all-fixed span의 specified size suggestion과 content size suggestion을 해당 grid area 최대 크기 이하로 제한하고, 반대 축 transferred-size 입력에도 같은 한도를 적용한다. 한도는 fixed max sizing functions의 합과 span 내부 fixed gutter의 합이다. [W3C 2025-03-26 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto) 구현 기대값은 이 규칙에서 독립적으로 도출한다. Chrome 관찰만으로 내부 contribution이나 transferred-size 중간값을 추론하지 않는다.

## 제한 구현 범위

- 기존 `RuntimeBlockFlexGridTrackSizingV1` 내부 fixture에서 direct HTML `img` Grid item의 §6.6 size suggestion과 automatic minimum을 다룬다.
- 기존 span range가 제공하는 fixed max track과 내부 gutter 합계를 재사용한다. gutter를 다시 더하지 않으며 trace에서 fixed track sum과 gutter sum을 나누어 기록한다.
- definite preferred size suggestion에도 all-fixed grid-area maximum limit을 적용한다. 기존 content-based automatic minimum과 aspect-ratio transferred-size source clamp도 회귀 검증한다. ratio 변환, opposite-axis min/max, target-axis preferred/max cap 순서는 유지한다.
- `border-box` limit은 그대로 적용한다. `content-box`에서는 반대 축 padding·border를 area limit에서 먼저 뺀 뒤 content-box transferred input을 clamp하고, ratio 변환 뒤 target 축 padding·border를 다시 더한다.
- 모든 spanned max function이 고정값으로 definite일 때만 limit을 만든다. `fit-content()` argument는 fixed max function으로 오인하지 않는다. flexible·intrinsic max function이 하나라도 포함되면 all-fixed clamp를 적용하지 않는다.
- 고정 `px` track·gutter, 두세 track span, span 안팎의 별도 gutter, 비대칭 fixed track 크기를 검증 입력으로 고정한다. 행·열 대칭, 외곽의 unspanned track 제외, zero-gap, content-box와 negative control을 포함한다.
- Grid 입력 검증은 track, gap, 크기 값의 음수·비유한 값을 거부한다. fixed-area track/gutter 합은 `f64`로 누적한다. `MAX_GRID_TRACKS` 이하의 유한 `f32` 입력 합은 `f64`에서 overflow하지 않으며, `f32`로 표현할 수 없는 limit은 어떤 유한 `f32` contribution도 초과할 수 없어 clamp하지 않는다. 큰 값 회귀는 trace 합계의 유한성과 모든 반환 frame의 유한성을 확인한다. gutter와 track 합계는 trace에서 구분한다.

## 이 작업에서 판정하지 않는 범위

Percentage/calc track 및 gap, vertical writing mode, dynamic image replacement, nested/anonymous/multiple image items, auto-repeat placement·collapse 조합, PNG texel paint, WPT 적합성, 전체 C11.4/CSS Grid 지원은 다음 관문으로 남긴다. §11.5의 limited min/max-content contribution `spanned_track_limit()`은 별도 규칙과 단계이므로 이 §6.6 수정에 섞지 않고 기존 결과를 회귀 검사한다.

## 사전 fixture와 관찰 결과

고정 fixture [grid-track-sizing-c114-image-ratio-fixed-gutter.html](../tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html)는 natural size `24×16` PNG, authored ratio `3/2`, preferred input `36px`, DPR 1의 고정 grid를 사용한다. image decode 완료 후 container/item frame, used track list, gap, box-sizing, placement와 overflow를 수집했다. 실행 JSON, 원본 Agent Browser 출력과 screenshot은 [사전 비교 근거](../spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/README.md)에 있다.

Chrome 155는 row-transfer 장면에서 item row `24px`, column-transfer 장면에서 item column `54px`를 관찰했다. item frame만으로 transferred-size suggestion 중간값은 알 수 없다. 첫 장면의 used column track `29px 4px`은 §6.6의 capped specified contribution `10px` 기대와 다른 결과를 보인다. 이를 관찰값으로 보존하되 구현 acceptance는 spec에서 도출한 값과 Rust stage trace를 기준으로 삼는다.

| fixture 장면 | §6.6 fixed area limit | Rust suggestion 기대값 | Chrome 155 관찰 |
| --- | ---: | ---: | --- |
| `column-two-track-one-gutter` | `3 + 3 + 4 = 10px` | width suggestion `10px`; row transfer `10 / 1.5 = 6.666667px` | columns `29px 4px`, item `36×24px` |
| `column-two-track-zero-gutter` | `3 + 4 = 7px` | width suggestion `7px`; row transfer `7 / 1.5 = 4.666667px` | item `36×24px` |
| `column-three-track-two-gutters` | `3 + 3 + 4 + 3 + 5 = 18px` | width suggestion `18px`; row transfer `18 / 1.5 = 12px` | item `36×24px` |
| `column-middle-span-excludes-outer-tracks-and-gutters` | `3 + 3 + 4 = 10px` | width suggestion `10px`; row transfer `6.666667px` | item `36×24px` |
| `row-two-track-one-gutter` | `3 + 3 + 4 = 10px` | height suggestion `10px`; column transfer `10 × 1.5 = 15px` | item `54×36px` |
| `row-three-track-two-gutters` | `3 + 3 + 4 + 3 + 5 = 18px` | height suggestion `18px`; column transfer `18 × 1.5 = 27px` | item `54×36px` |
| `row-middle-span-excludes-outer-tracks-and-gutters` | `3 + 3 + 4 = 10px` | height suggestion `10px`; column transfer `15px` | item `54×36px` |
| `column-content-box-subtracts-padding-and-border` | border-box `10px`; content width limit `6px` | width suggestion `10px`; content height transfer `4px` + target edges `4px` = `8px` | row `28px` |
| `column-flexible-max-is-not-fixed`, `column-fit-content-is-not-fixed-max` | no all-fixed limit | no grid-area suggestion clamp | row `24px` |

## 구현 완료 조건

1. §6.6 all-fixed predicate와 max-size sum이 resolved span에 적용되고 definite preferred-size suggestion도 limit 이하로 clamp된다. 내부 gutter는 existing span range에서 한 번만 포함한다.
2. Rust unit matrix는 row·column, 2·3 tracks, zero/one/two gutters, middle span, non-fixed controls, content-box edges, min/max ordering, 기존 음수 gap 입력 거부, `f32::MAX` 초과 fixed-area limit과 이전 C11.4 regressions를 다룬다. trace는 source contribution, fixed track sum, gutter sum, combined limit, stage과 final frame을 같은 revision으로 연결한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 `fixed-gutter-column-suggestion-clamp`, `fixed-gutter-row-suggestion-clamp` scene의 root/item frame과 trace를 각각 실행한다. 1px grid border 안팎에서 image color 표본을 확인하며 screenshot 자체를 pixel-proof로 확대하지 않는다.
4. fixture FFI는 opt-in feature에 유지하고 기본 product artifact와 test-positive artifact 경계를 다시 검사한다. C11.4 상위 체크와 완료 수를 올리지 않는다.
5. 구현 전에 이 계획을 서로 다른 실패 관점 20개로 검토했다. 구현 후 실제 변경 코드에는 별도의 20관점 검토를 기록한다. PR diff는 프로젝트 규칙에 따라 별도의 30관점으로 검토한다. 두 표를 재사용하지 않고 결함을 발견하면 수정 후 관련 관점을 재검토한다.
