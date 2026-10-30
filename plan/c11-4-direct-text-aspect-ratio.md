# C11.4 direct-text preferred aspect ratio intrinsic contribution

**상태:** plan-only candidate. Chrome 156 precomparison and independent CSS calculations are fixed in [precomparison evidence](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-precomparison-2026-10-30/README.md). The pre-implementation failure review and plan-PR delta review are complete. No Spinon implementation or simulator run is claimed by this plan. C11.4 and C11 stay incomplete; official completion remains `72/213`.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [generic direct-text intrinsic input](./c11-4-generic-direct-text-intrinsic-input.md) · [internal contract 0059](../spec/internal/0059-c11-grid-layout.md)

## Problem and standards basis

The generic C11.4 direct-text collector currently rejects any `layout_aspect_ratio` on an item. This prevents its opt-in path from exercising the ratio-aware Grid item sizing already represented by the typed style and Taffy Grid item. The nearby image implementation is not a safe algorithm to copy wholesale: Grid §6.6's transferred-size suggestion path has replaced-item-specific rules. This plan instead tests a non-replaced direct-text Grid item's ordinary ratio-aware sizing and the resulting intrinsic track contributions.

The normative basis is [CSS Grid Layout 2025 CRD §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing) for automatic sizing of grid items with a preferred aspect ratio, [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing) for intrinsic track sizing from item contributions, and [CSS Sizing 3 §2.2](https://www.w3.org/TR/css-sizing-3/#intrinsic-size-contributions) plus [CSS Sizing 4 aspect-ratio](https://www.w3.org/TR/css-sizing-4/#aspect-ratio) for contribution terminology and ratio behavior. Implementation and review must not route this non-replaced item through the replaced-item transferred-size-suggestion shortcut.

The fixed CSS fixture uses Ahem at 10px with `line-height:10px`, two direct block text items, `grid-template-columns:max-content max-content`, and normal self-alignment. With `aspect-ratio:2/1` on the 40px `AAAA` item, the auto row is 20px instead of the 10px no-ratio control. With a definite 40px row, that same item produces an 80px max-content column instead of the 40px control. Chrome 156 observed those values; the independent calculations and normalized frames are recorded separately in the evidence file.

## Restricted profile

- Route only through the existing opt-in `RuntimeBlockFlexGridTrackSizingV1` / C11.4 fixture-test path. This is not public CSS admission and does not broaden ordinary layout CSS support.
- Accept exactly two in-flow, HTML-namespace, block direct children of one Grid root. Each owns exactly one meaningful direct Text node, with ASCII `A–Z` and U+0020 only. No descendant text, anonymous Grid text, replaced element, nested content, span, explicit placement, nonzero `order`, or source-dependent node-ID exception.
- Use the pinned Ahem font, 10px font size, `line-height:10px`, horizontal-tb, LTR, `overflow:visible`, normal `align-items`/`justify-items`, no margins, padding, or border, and automatic width/height/min-size with no maximum size. No font substitution or system font fallback is allowed.
- The only positive ratio is a computed preferred ratio exactly `2.0` on the four-character `AAAA` item. Its sibling has no preferred ratio. Other ratios, `auto <ratio>`, invalid/non-finite values, multiple ratio items, or ratio on an unsupported item must fail closed. The ratio limit is part of this fixture's typed input contract, not a general ratio policy.
- Row scenes use two max-content columns and one auto row. Column scenes use two max-content columns and one definite 40px row. The Grid root is max-content sized. The four scenes and independent geometry are fixed by the precomparison fixture: `row-ratio`, `row-control`, `column-ratio`, and `column-control`.
- Collector acceptance must be based on actual HostDocument source and Stylo computed snapshots. The collector may not infer ratio from scene IDs or author-source strings. The Taffy item style must preserve the typed ratio through layout, and the row/column track trace must show the expected contribution. Keep the existing no-ratio generic direct-text scenes and legacy one-item fallbacks unchanged.

## Implementation and validation gates

1. Extend the fixture collector narrowly so it admits the exact two-item profile above while rejecting unsupported ratios, wrong item count/shape, stale style revisions, incorrect axes, and unsupported box constraints. Update internal contract 0059 without changing its numeric version `0.1.0`.
2. Add positive/control and adversarial Rust tests for ratio-to-block and ratio-to-inline direction, typed ratio preservation, contribution source, max-content tracks, no-ratio regressions, malformed ratios, dimensions/min-max/edges, and failure atomicity. Do not label the Chrome fixture as a WPT run.
3. Connect four explicit C11.4 fixture scenes through the existing runtime GPU scene registry and both platform runners. Compare typed input, `Rows`/`Columns` trace, root and item frames against the fixed local-coordinate oracle. Add app evidence only if those runner scenes execute the new collector through the live V8 → HostDocument → Stylo → runtime → Taffy path.
4. Build and run the four scenes on Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator. Record installed artifact hashes, route logs, scene/frame/track payloads, PixelCopy and Metal drawable sampling boundaries, and screenshots. Pixel and glyph evidence must be reported only for the layer actually sampled; the expected gate does not claim post-compositor iOS pixels or hardware-device GPU coverage.
5. Verify fixture-only FFI symbols remain absent from default product artifacts and present in the opted-in fixture-test positive control. Run focused Rust tests, relevant crate tests, formatting, and feature-enabled Clippy after implementation.
6. Complete a 20-item plan failure review before implementation, a separate 20-item implementation/runtime review after runtime evidence, and a distinct 30-item PR-delta review before merge. Correct failures and rerun the affected review items.
7. Keep `spec/STATUS.md`, internal contract 0059, and the public roadmap explicitly incomplete for C11.4/C11; do not raise the official `72/213` count for this limited fixture slice. After merge, verify the exact merge commit and status/roadmap synchronization.

## Explicit exclusions

This plan does not add general CSS support, WPT execution/conformance, arbitrary aspect ratios, `auto <ratio>`, `min-width`/`max-width` or `min-height`/`max-height` interactions, content-box edge transfer, replaced images, rich or anonymous text, multiple lines or wrapping, row/column spans, flex tracks, percentage/calc sizing, orthogonal or vertical writing modes, baseline alignment, font fallback, glyph raster parity, iOS compositor-after pixels, physical-device GPU proof, or C11.4/C11 completion.
