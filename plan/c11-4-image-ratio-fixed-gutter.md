# C11.4 · fixed max track span과 gutter를 포함한 automatic minimum

**상태:** 계획과 Chrome 155 사전 관찰만 완료했다. 구현·Rust 회귀·Android/iOS Simulator 실행은 아직 하지 않았다. C11.4와 C11은 계속 미완료다.

**상위 계획:** [C11.4 Grid track sizing](c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 발견한 경계

PR #354의 transferred-size 후보는 direct replaced Grid item의 반대 축 입력을 fixed max track limit으로 제한한다. 현재 `vendor/taffy/src/compute/grid/types/grid_item.rs`의 `spanned_fixed_track_limit()`은 item이 덮는 track의 fixed max 함수만 더하고, 그 사이의 fixed gutter는 더하지 않는다. 이 한도는 같은 파일에서 automatic minimum과 aspect-ratio transferred-size suggestion 양쪽에 사용된다.

예를 들어 `minmax(auto, 3px) 4px` 두 열을 item이 가로지르고 `column-gap: 3px`이면 §6.6의 grid-area maximum size는 `3 + 3 + 4 = 10px`이다. 현재 helper의 track-only 합은 `7px`이다. `width:36px; aspect-ratio:3/2`인 image의 row transferred suggestion은 계획 기준으로 `10 / 1.5 = 6.666667px`이어야 하며, track-only clamp는 `7 / 1.5 = 4.666667px`로 지나치게 작게 만든다. 세 track을 가로지르면 모든 내부 gutter를 한 번씩 포함해야 하며, item span 바깥의 track과 gutter는 합계에서 제외해야 한다.

CSS Grid Level 1 §6.6은 fixed max sizing function만 가진 track span의 size suggestion과 반대 축 transferred-size 입력을, 해당 grid area 최대 크기(고정 max 함수의 합과 그 사이 fixed gutter의 합) 이하로 제한한다. [W3C 2025-03-26 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto) 구현 기대값은 이 규칙에서 독립적으로 도출한다. Chrome 관찰을 표준 기대값으로 사용하지 않는다.

## 제한 구현 범위

- 기존 `RuntimeBlockFlexGridTrackSizingV1` 내부 fixture에서 direct HTML `img` Grid item의 §6.6 automatic minimum을 다룬다.
- 행과 열에서 item span에 포함된 fixed max track을 모두 합산하고, 이웃한 두 track 사이의 사용된 fixed pixel gutter를 정확히 `span track count - 1`개 더한다.
- 같은 grid-area maximum limit을 target-axis automatic minimum과 aspect-ratio transferred-size의 source-axis 입력에 적용한다. ratio 변환, opposite-axis min/max, target-axis preferred/max cap 순서는 유지한다.
- `border-box` limit은 그대로 적용한다. `content-box`에서는 반대 축 padding·border를 area limit에서 먼저 뺀 뒤 content-box transferred input을 clamp하고, ratio 변환 뒤 target 축 padding·border를 다시 더한다.
- 모든 spanned max function이 고정값으로 definite일 때만 limit을 만든다. `fit-content()` argument는 fixed max function으로 오인하지 않는다. flexible·intrinsic max function이 하나라도 포함되면 all-fixed clamp를 적용하지 않는다.
- 고정 `px` track·gutter, 두세 track span, span 안팎의 별도 gutter, 비대칭 fixed track 크기를 구현 입력으로 고정한다. 행·열 대칭, 외곽의 unspanned track 제외, zero-gap, content-box와 negative control을 포함한다.
- 계산 중 non-finite 값이나 합계 overflow는 부분 layout 또는 임의 clamp로 대체하지 않고 기존의 typed error 경계로 반환한다. gutter와 track 합계를 trace에서 구분해 관찰할 수 있어야 한다.

## 이 작업에서 판정하지 않는 범위

Percentage/calc track 및 gap, vertical writing mode, dynamic image replacement, nested/anonymous/multiple image items, auto-repeat placement·collapse 조합, PNG texel paint, WPT 적합성, 전체 C11.4/CSS Grid 지원은 다음 관문으로 남긴다. §11.5의 limited min/max-content contribution `spanned_track_limit()`은 별도 규칙과 단계이므로 이 §6.6 수정에 섞지 않고 기존 결과를 회귀 검사한다.

## 사전 fixture와 관찰 결과

고정 fixture [grid-track-sizing-c114-image-ratio-fixed-gutter.html](../tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html)는 natural size `24×16` PNG, authored ratio `3/2`, preferred input `36px`, DPR 1의 고정 grid를 사용한다. image decode 완료 후 container/item frame, used track list, gap, box-sizing, placement와 overflow를 수집했다. 실행 JSON, 원본 Agent Browser 출력과 screenshot은 [사전 비교 근거](../spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/README.md)에 있다.

Chrome 155는 row-transfer 장면에서 row `24px`, column-transfer 장면에서 column `54px`를 관찰했다. fixed track 합계와 gutter로부터 도출한 §6.6 기대값과 다르며, 이를 상호 운용 차이로 보존한다. 구현 acceptance는 spec에서 도출한 값과 Rust stage trace를 기준으로 삼고 Chrome 결과를 숨기지 않는다.

| fixture 장면 | §6.6 fixed area limit | ratio로 clamp된 입력 및 기대 size | Chrome 155 관찰 |
| --- | ---: | ---: | --- |
| `column-two-track-one-gutter` | `3 + 3 + 4 = 10px` | row `10 / 1.5 = 6.666667px` | row `24px` |
| `column-two-track-zero-gutter` | `3 + 4 = 7px` | row `7 / 1.5 = 4.666667px` | row `24px` |
| `column-three-track-two-gutters` | `3 + 3 + 4 + 3 + 5 = 18px` | row `18 / 1.5 = 12px` | row `24px` |
| `column-middle-span-excludes-outer-tracks-and-gutters` | `3 + 3 + 4 = 10px` | row `6.666667px` | row `24px` |
| `row-two-track-one-gutter` | `3 + 3 + 4 = 10px` | column `10 × 1.5 = 15px` | column `54px` |
| `row-three-track-two-gutters` | `3 + 3 + 4 + 3 + 5 = 18px` | column `18 × 1.5 = 27px` | column `54px` |
| `row-middle-span-excludes-outer-tracks-and-gutters` | `3 + 3 + 4 = 10px` | column `15px` | column `54px` |
| `column-content-box-subtracts-padding-and-border` | border-box `10px`; content width limit `6px` | content height `4px` + target edges `4px` = row `8px` | row `28px` |
| `column-flexible-max-is-not-fixed`, `column-fit-content-is-not-fixed-max` | no all-fixed limit | row remains un-clamped at `24px` | row `24px` |

## 구현 완료 조건

1. §6.6의 all-fixed predicate와 sum이 resolved grid item span에 정확히 적용되고, gutter를 span 내부에서만 한 번 더한다. gap이 이미 반영된 frame/track geometry에 다시 더해 layout을 이중 확대하지 않는다.
2. Rust unit matrix는 row·column, 2·3 tracks, zero/one/two gutters, middle span, non-fixed controls, content-box edges, min/max ordering, zero/negative/non-finite arithmetic와 이전 C11.4 regressions를 다룬다. trace는 source contribution, computed limit/gutter sum, stage과 final frame을 같은 revision으로 연결한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 앱 fixture에서 신규 scene의 root/item frame과 trace를 각각 실행한다. 명시된 overflow 장면은 grid border 안과 바깥의 item 색 표본을 확인하며 screenshot 자체를 pixel-proof로 확대하지 않는다.
4. fixture FFI는 opt-in feature에 유지하고 기본 product artifact와 test-positive artifact 경계를 다시 검사한다. C11.4 상위 체크와 완료 수를 올리지 않는다.
5. 구현 전에 이 계획을 서로 다른 실패 관점 20개로 검토한다. 구현 후 실제 변경 코드에는 별도의 20관점 구현 검토를 하고, PR 변경 diff도 사용자 요청대로 독립된 20관점으로 검토한다. 결함을 발견하면 수정 후 관련 관점을 재검토한다.
