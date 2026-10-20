# C11.4 definite minimum-size `auto-fit` 축 차이 사전 비교

**날짜:** 2026-10-20
**목적:** `auto-fit`의 repeat count가 definite minimum-size를 기준으로 할 때 columns/rows에서 달라지는 Chrome 155 관찰을 재현하고, 규범 기대와 분리해 보존한다. 이 파일은 Chrome 관찰이며 Spinon runtime 결과나 WPT 적합성 근거가 아니다.

## 입력과 환경

- fixture: [`grid-auto-fit-min-size-axis-c114.html`](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/tests/fixtures/css/c11/grid-auto-fit-min-size-axis-c114.html), SHA-256 `8c985bbecc9c6df45122dee59bf3491da4ca656b986743dd89c5b130a02973b3`
- 관찰 JSON: [`chrome155-observation.json`](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-observation.json), SHA-256 `2e0d1dbafb81877a37f36ec7bfc1f47b7236507d4a23522d44dff738a3add305`
- browser 화면: [representative viewport](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-auto-fit-axis.png), 1600×1500 pixels, SHA-256 `e42855b766907d48a4b242b4af4343bc310151d95c383680bbd1a254a6216a73` · [full fixture page](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-auto-fit-axis-full.png), 1600×4096 pixels, SHA-256 `810bc7e0a00ee8986f5cc7c747733f81af5a1f24e20d1c0ff734136619eb239b`
- Browser: Google Chrome `155.0.8059.85` arm64, V8 `15.5.35.25`; Agent Browser `0.31.1`; macOS `26.5.1 (25F80)`; viewport `1600×1500` CSS px, DPR 1. `chrome://version` reported source revision `0e66b94549a8bd85f03fbdf189b752d6e7b63c32` and branch `8059@{#1304}`.
- The fixture records `navigator.userAgent`, all CSS inputs, `getComputedStyle()` track lists, root/item rectangles and numeric track sizes for 30 cases. Full raw output is preserved without deriving it from Spinon.

## 핵심 관찰

For the same `minmax(40px, 1fr) 20px` fragment, 8px gap, `20px` prefix and `10px` suffix, max-content preferred size, and 200px definite minimum-size:

| Axis | `auto-fill` repeats | `auto-fit` repeats | `auto-fit` resolved repeated tracks | root size |
| --- | ---: | ---: | --- | --- |
| columns | 3 | 2 | four `0px` tracks | `200×16px` |
| rows | 3 | 3 | six `0px` tracks | `16×200px` |

The same column/row split remains with fixed `40px 20px` pattern, prefix-only, suffix-only, no neighbor, empty/single/span-two/out-of-flow items, and `vertical-rl` writing mode. For the minimum-size boundaries 190px and 342px both axes report 2 and 4 repeats. A definite 200px preferred-size control reports 2 for `auto-fill` and `auto-fit` on both axes. Per-case values, including the actual empty/occupied tracks and item rectangles, are in the JSON.

An additional Agent Browser probe places an item in the implicit track after the empty repeat range. At the same min-size-only inputs, Chrome reports `x=28px` for the prefix-only columns case and `y=28px` for rows: the leading gutter remains `8px` after the `20px` prefix. The [probe](./auto-fit-min-prefix-gap-probe.html), [observation JSON](./chrome155-auto-fit-min-prefix-gap-observation.json), and [screenshot](./chrome155-auto-fit-min-prefix-gap-probe.png) preserve this detail for the simulator gutter assertion.

## Normative comparison

[CSS Grid Layout Module Level 1 §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation) says `auto-fit` uses the same repeat count as `auto-fill`; it collapses empty repeated tracks only after item placement. With the fixture's minimum-size basis and both explicit neighbors, the repeated minimum contribution is `38 + 76n`: 190px needs 2 repeats, 200px needs 3, and 342px needs 4. Thus the columns `auto-fit` result at 200px differs from both the paired `auto-fill` observation and the independent normative calculation.

The fixture also separates the minimum-size path from the definite preferred-size control. At a definite 200px preferred size, the largest pattern count under the content-box size is 2. It does not use that result as the expected count for the minimum-size-only cases.

## Chromium source investigation and limits

Chrome's `chrome://version` revision and the public source commit are not the same identifier. Direct Agent Browser opens of the Chrome-reported revision and `refs/branch-heads/8059` returned `NOT_FOUND`; therefore this note does not claim the inspected Gitiles file is the exact source revision of the observed binary. The public source commit [8c0f71f… / `NGGridLayoutAlgorithm::ComputeAutomaticRepetitions`](https://chromium.googlesource.com/chromium/src/+/8c0f71f21625df16367c644d18fa3b4c3b8a4db8/third_party/blink/renderer/core/layout/ng/grid/ng_grid_layout_algorithm.cc#L1116) was read in Agent Browser. It computes repetitions separately for each track direction, uses floor with a finite maximum available size, and otherwise uses ceil from the minimum available size. This makes the axis-specific available-size basis a concrete investigation target. The runtime's private inline/block min/max inputs were not captured, so this is a diagnostic hypothesis, not a proven cause.

The writing-mode control swaps the physical orientation of columns and rows while the mismatch stays with the logical Grid axis. This narrows the next investigation but does not prove which Blink intrinsic-size pass supplies a finite maximum.

## 해석 경계

- The spec-derived counts, not the observed Chrome `auto-fit` counts, define future Spinon test expectations.
- A future Spinon result must be recorded separately from this Chrome oracle. Agreement with the browser alone cannot override the stated CSS Grid rule.
- No claim is made about public CSS support, WPT conformance, physical devices, hardware GPU, or compositor-after pixels.
