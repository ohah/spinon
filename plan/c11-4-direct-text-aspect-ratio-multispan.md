# C11.4 direct-text preferred aspect-ratio two-track span 계획

**상태:** 고정 HTML 입력, Agent Browser HeadlessChrome 156 관측, CSS Grid 2025 CRD 기반 독립 기대값을 준비했다. 구현과 Android/iOS Simulator 51-scene 검증, Rust/FFI focused tests, fixture archive boundary, 구현/runtime 실패 관점 검토 20개 및 implementation PR diff review 30개를 완료했다. PR 검증은 진행 중이다. 공식 C11/C11.4 checkbox와 `72/213`은 유지한다.

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [C11.4 direct-text aspect-ratio min/max](./c11-4-direct-text-aspect-ratio-min-max.md) · [C11 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 문제와 목표

기존 direct-text aspect-ratio 계획은 `span`을 명시적으로 거부한다. PR #850의 min/max 확장도 Ahem `AAAA` ratio owner 하나와 짧은 `A` item 하나가 각각 단일 track에 놓이는 고정 profile이다. C11.4에는 text와 multi-track span을 각각 다루는 경로가 있지만 **preferred aspect-ratio와 min/max가 있는 direct-text item의 intrinsic contribution이 두 track 사이에 분배되는 조합**은 아직 고정하지 않았다.

이 계획은 fixture-test 전용 경로에서 해당 조합만 닫는다. 제한 입력의 content-size contribution을 CSS Grid track sizing의 spanning-item distribution으로 넘기고, 최종 track/frame 및 overflow를 Android/iOS simulator에서 비교한다. Public CSS admission, 일반 layout API, 전체 C11.4 또는 C11 완료는 목표가 아니다.

기준은 [CSS Grid Layout 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#extra-space), 그리고 [CSS Sizing Level 4 §4](https://www.w3.org/TR/css-sizing-4/#aspect-ratio)다. 문서의 초안/CRD 지위와 Chrome 관측을 규범 적합성으로 혼동하지 않는다.

## 고정 입력과 독립 기대값

Canonical fixture는 [grid-direct-text-aspect-ratio-multispan-precomparison.html](../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-multispan-precomparison.html)이다. Root 하나는 `width:100px`, `grid-template-columns:max-content max-content`, `grid-template-rows:auto`, gap 0을 쓴다. 정확히 하나의 in-flow HTML block child가 direct text `AAAA`를 소유하고 `grid-column:1 / span 2`로 두 열을 차지하며 row placement는 `auto / auto`다. Font는 Ahem 10px, line-height 10px, horizontal-tb/LTR다. Item의 width/height는 auto, margin·padding·border는 0이다. Ratio owner는 이 item 하나이며 numeric `2 / 1` 또는 no-ratio control만 둔다.

독립 기대값과 실제 관측은 [precomparison README](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/README.md), [`expected-values.json`](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/expected-values.json), [Agent Browser 관측 JSON](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/chrome-observation-agent-browser.json), [재검증 스크립트](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/verify-precomparison.mjs), [화면](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/chrome-oracle-agent-browser.png)에 기록했다. Browser는 이 canonical HTML과 로컬 Ahem TTF를 사용했고 font readiness가 true인지 검사했다.

| case | item 제약 | columns | row | item frame | direct text 폭 |
|---|---|---:|---:|---:|---:|
| `span-baseline` | ratio only | `[20, 20]` | `20` | `40×20` | `40` |
| `span-min-width-45` | `min-width:45px` | `[22.5, 22.5]` | `22.5` | `45×22.5` | `40` |
| `span-min-width-60` | `min-width:60px` | `[30, 30]` | `30` | `60×30` | `40` |
| `span-min-height-30` | `min-height:30px` | `[30, 30]` | `30` | `60×30` | `40` |
| `span-max-width-30-min-width-0` | `min-width:0; max-width:30px` | `[15, 15]` | `15` | `30×15` | `40` |
| `span-max-height-15-auto-min-control` | `min-height:0; max-height:15px` | `[20, 20]` | `15` | `40×15` | `40` |
| `span-max-height-15-min-width-0` | `min-width:0; min-height:0; max-height:15px` | `[15, 15]` | `15` | `30×15` | `40` |
| `span-max-height-17-75-range` | `min-width:0; min-height:0; max-height:17.75px` | `[17.75, 17.75]` | `17.75` | `35.5×17.75` | `40` |
| `span-no-ratio` | `aspect-ratio:auto` | `[20, 20]` | `10` | `40×10` | `40` |

CSS Grid §11.5.1 independently predicts equal distribution here: both spanned tracks have identical `max-content` sizing functions and no single-span item sets a prior size, so a 40/45/60/30/35.5px inline contribution yields 20/22.5/30/15/17.75px per track. The 40px Ahem min-content floor remains in `span-max-height-15-auto-min-control`; `min-width:0` removes that floor and exposes the 30px ratio/max-height contribution. The Chrome output corroborates these calculations; it is not their source.

## Fixture-only input contract

- Route only through the existing `RuntimeBlockFlexGridTrackSizingV1` C11.4 fixture-test path and a dedicated direct-text aspect-ratio span profile. Default product artifacts and general `RuntimeBlockFlexGridTrackSizingV1` callers must not gain this admission.
- Require one current HTML Grid root and exactly one in-flow HTML block item. Its sole meaningful direct Text node is `AAAA`; nested/anonymous text, extra item, image/replaced item, hidden item, out-of-flow item, nonzero `order`, and stale document/style revisions fail closed.
- Before layout, require the typed Grid template input `max-content max-content`, one auto row, zero gap, width 100px, and no root edge. Require the single item at column line 1 spanning exactly 2 tracks. Row placement remains automatic. Do not compare CSSOM's resolved pixel track list with the authored/typed track input. Named lines, negative lines, implicit tracks, row spans, other spans, and auto placement across columns are rejected.
- Require Ahem 10px, line-height 10px, horizontal-tb, LTR, normal wrapping, width/height auto, and zero item margin/padding/border. Only one numeric `2 / 1` typed preferred ratio is admitted, and its computed serialization must match. `aspect-ratio:auto` is permitted only for the no-ratio control.
- Reuse the existing direct-text min/max subset: `auto`/`none` defaults or finite `0..=60px` values. Percentages, `calc()`, intrinsic keywords, negative/non-finite values, and values above 60px fail before layout. `box-sizing` remains content-box with zero edges, so this slice has no edge-transfer ambiguity.
- Preserve the exact ratio owner NodeId and one style revision through computed-style collection, paragraph creation, style-to-layout projection, Grid sizing, and trace. Never infer an owner from the scene ID or source CSS string.

## 구현 및 수용 기준

1. Vendored Taffy 0.15.0의 non-replaced ratio content-size contribution과 §11.5 spanning-item distribution 경로를 대조한다. Ratio/min/max 계산은 non-replaced 경로에 유지하고 text item을 replaced-image `transferred size suggestion`으로 보내지 않는다.
2. 기존 `generic_direct_text.rs`는 895줄이므로 span-profile collector/validation을 이 파일에 더 크게 쌓지 않고 별도 모듈로 둔다. 코드 변경 전 helper 계약을 고정한다. 불변 snapshot/root/style 입력을 받아 소유된 paragraph style들과 정확한 ratio-owner NodeId를 반환하며, 공유 style을 바꾸거나 fixture ID로 owner를 추론하지 않는다.
3. Add all nine fixed cases to a dedicated profile/matrix. For each case compare computed input and owner, per-track base/growth-limit changes and spanning planned increases, final columns/row, root/item/text frame, and inline overflow against the independent JSON. Do not set a case expected value from the current Spinon result.
4. Negative controls must detect or reject: ratio inversion; row/column axis swap; span 1 or 3; wrong start/end lines; row span; auto placement; named/negative/implicit line; one track changed from max-content; extra/missing item; nested or replaced content; wrong Ahem metrics/text; auto-minimum floor dropped; explicit `min-width:0` ignored; min/max order reversed; one axis max overwriting a stronger same-axis minimum; wrong/multiple ratio owner; ratio serialization/typed mismatch; stale revision; unsupported percentage/calc/negative/non-finite/>60px min/max.
5. Run existing 42 direct-text aspect-ratio/minmax cases plus the nine new cases on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator. Require 51/51 scenes per platform and exact track/frame/overflow match. Fix PixelCopy/Metal sample coordinates from the independent geometry before running the app; sample item background bounds and clear pixels after the item/root. Assert text overhang from Chrome text range plus runtime paragraph/track geometry, not simulator pixels. The track seam is geometry-trace evidence only because one continuous item paints across it. State that the AVD uses software rendering and iOS samples are taken before compositor submission.
6. Run focused Rust/runtime/layout tests, the existing direct-text regression groups, relevant FFI route tests, Clippy and rustfmt. Verify default artifacts omit fixture route/symbols/matrix payload and the opt-in fixture artifact contains the positive control. Report only commands actually run; do not treat an empty PR check rollup as CI success.
7. Keep C11/C11.4 checkboxes and official `72/213` unchanged. Update internal contract 0059 only with the bounded fixture contract after implementation is accepted.

## 검토와 한계

현재 동일한 C11.4 text route의 simulator screenshot은 Grid item의 색 배경만 그리고 Ahem glyph는 그리지 않는다([Android AVD example](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvut1ldd-e32f3c4c/grid-text-aspect-ratio-minmax-min-width-45.png), [runtime evidence와 sample 한계](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.md)). 따라서 10px/4.5px text overhang은 Chrome oracle, direct Text range, measured paragraph layout, C11.4 track trace와 item frame으로 확인한다. PixelCopy/Metal은 item box edge와 그 바깥의 clear 영역을 검증하며 glyph가 paint 됐다는 증거로 세지 않는다.

- 구현 전 계획 자체를 20개의 서로 다른 실패 관점으로 검토하고, 계획 PR diff도 별도 20개 관점으로 확인한다. 구현 뒤 새 코드/runtime 20개와 implementation PR diff 20개를 각각 검토한다. 동일한 항목을 말만 바꾸어 세지 않는다.
- 현재 계획 검토: [실패 관점 20개](./c11-4-direct-text-aspect-ratio-multispan-plan-review-20.md).
- 계획 PR #854 변경 검토: [실패 관점 20개](./c11-4-direct-text-aspect-ratio-multispan-pr-diff-review-20.md).
- simulator glyph-paint 한계에 따른 pixel 기준 보정 계획 검토: [실패 관점 20개](./c11-4-direct-text-aspect-ratio-multispan-pixel-amendment-review-20.md).
- pixel 기준 보정 계획 PR 변경 검토: [실패 관점 30개](./c11-4-direct-text-aspect-ratio-multispan-pixel-amendment-pr-diff-review-30.md).
- canonical HTML과 계획의 row placement 불일치 보정 검토: [실패 관점 20개](./c11-4-direct-text-aspect-ratio-multispan-row-placement-plan-review-20.md).
- row placement 보정 PR 변경 검토: [실패 관점 30개](./c11-4-direct-text-aspect-ratio-multispan-row-placement-pr-diff-review-30.md).
- 계획 PR과 구현 PR은 별도 PR이며 `Rebase and merge`를 사용한다. 계획 PR에는 `planning`/`area: layout`, 구현 PR에는 `enhancement`/`area: layout` 라벨을 쓴다.
- 범위는 이 단일 2-column span topology, Ahem `AAAA`, numeric 2:1 ratio, 0..=60px min/max fixture 입력에 한정한다. 다른 span length/topology, more than two tracks, unequal/fixed/flexible/fit-content tracks, gaps, multiple or mixed items, images, `auto <ratio>`, nonzero edges, arbitrary font/script, vertical writing, baseline alignment, general CSS admission, full WPT, real-device/hardware GPU, glyph raster/pixel proof, iOS compositor-after pixels, C11.4 전체 또는 C11 완료는 포함하지 않는다.
