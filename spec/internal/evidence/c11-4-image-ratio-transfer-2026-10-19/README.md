# C11.4 replaced-image transferred-size precomparison

2026-10-19, fixed 27 Chrome 155 observations at DPR 1: 26 direct replaced-image scenes using the existing 24×16 CSS px PNG, and one non-replaced `div` control. The first sixteen image cases were captured before changing the Grid sizing implementation. Later Agent Browser sessions added specified-size precedence, target-axis caps, scrollable overflow, replaced versus non-replaced behavior, opposite-axis `min > max`, content-/border-box decoration, and target-axis minimum coverage.

## Oracle and decision rules

- Oracle: local Agent Browser session using HeadlessChrome 155.0.0.0 at DPR 1; no remote image requests. The fixture waits for the PNG decode and requires its natural size to be 24×16 CSS px.
- Normative rule: CSS Grid Level 1 §6.6 selects the specified size suggestion first, then the transferred size suggestion for a replaced item with a preferred aspect ratio and definite opposite-axis preferred size, otherwise the content size suggestion. The transferred value is clamped by definite opposite-axis min/max sizes before ratio conversion and by relevant-axis preferred/max sizes afterward. [W3C CSS Grid Level 1, 2025 CRD, §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto).
- The PNG's natural ratio is 3:2. The fixture also records the computed `aspect-ratio` so natural `auto` and authored `3 / 2` cases remain distinguishable. Track-list strings and overflow direction are exact; each grid/item CSS-pixel size is compared to Chrome within 0.05 CSS px. Auto-min cases differ from their explicit zero-minimum and scrollable-overflow controls as observed below.
- This oracle establishes direct replaced-image behavior only. It does not establish nested/anonymous items, dynamic replacement, WPT conformance, or all Grid contribution stages.

## Frozen Chrome observations

| Case | Track | Grid | Item | Outside grid | Distinguishing input |
| --- | --- | --- | --- | --- | --- |
| `column-auto-natural-ratio` | columns 12px | 8×20 | 12×8 | right | height 8px, natural 3:2 ratio |
| `column-zero-natural-ratio` | columns 8px | 8×20 | 12×8 | right | same image, zero inline minimum |
| `column-min-content-natural-ratio` | columns 12px | 8×20 | 12×8 | right | min-content track control |
| `column-max-height-clamp` | columns 6px | 4×20 | 6×4 | right | height 8px, max-height 4px |
| `column-min-height-clamp` | columns 18px | 16×20 | 18×12 | right | height 8px, min-height 12px |
| `column-authored-ratio` | columns 12px | 8×20 | 12×8 | right | authored aspect-ratio 3:2 |
| `column-authored-max-height-clamp` | columns 6px | 4×20 | 6×4 | right | authored ratio, max-height 4px |
| `column-authored-min-height-clamp` | columns 18px | 16×20 | 18×12 | right | authored ratio, min-height 12px |
| `column-specified-size-precedes-transfer` | columns 10px | 8×20 | 10×8 | right | definite width 10px precedes height transfer |
| `column-target-max-width-clamp` | columns 9px | 8×20 | 9×8 | right | transferred suggestion is capped by target `max-width: 9px` |
| `column-opposite-min-over-max` | columns 18px | 8×20 | 18×12 | right | opposite `min-height:12px` wins over `max-height:4px` before transfer |
| `column-border-box-transfer` | columns 12px | 8×20 | 12×8 | right | border-box with 1px padding and 1px border on each side |
| `column-content-box-transfer` | columns 16px | 8×20 | 16×12 | right | content-box adds 1px padding and 1px border per side after conversion |
| `column-target-min-width-clamp` | columns 18px | 8×20 | 18×8 | right | target `min-width:18px` applies while definite height stays 8px |
| `column-authored-ratio-scrollable-overflow-min-zero` | columns 8px | 8×20 | 12×8 | right | scrollable overflow gives an automatic minimum of zero |
| `column-non-replaced-ratio-content-size` | columns 12px | 8×20 | 12×8 | right | empty non-replaced `div` with ratio; no intrinsic image resource |
| `row-auto-natural-ratio` | rows 8px | 20×6 | 12×8 | bottom | width 12px, natural 3:2 ratio |
| `row-zero-natural-ratio` | rows 6px | 20×6 | 12×8 | bottom | same image, zero block minimum |
| `row-min-content-natural-ratio` | rows 8px | 20×6 | 12×8 | bottom | min-content track control |
| `row-max-width-clamp` | rows 4px | 20×3 | 6×4 | bottom | width 12px, max-width 6px |
| `row-min-width-clamp` | rows 12px | 20×9 | 18×12 | bottom | width 12px, min-width 18px |
| `row-authored-ratio` | rows 8px | 20×6 | 12×8 | bottom | authored aspect-ratio 3:2 |
| `row-authored-max-width-clamp` | rows 4px | 20×3 | 6×4 | bottom | authored ratio, max-width 6px |
| `row-authored-min-width-clamp` | rows 12px | 20×9 | 18×12 | bottom | authored ratio, min-width 18px |
| `row-specified-size-precedes-transfer` | rows 6px | 20×6 | 12×6 | none | definite height 6px precedes width transfer |
| `row-target-max-height-clamp` | rows 6px | 20×4 | 12×6 | bottom | transferred suggestion is capped by target `max-height: 6px` |
| `row-opposite-min-over-max` | rows 12px | 20×4 | 18×12 | bottom | opposite `min-width:18px` wins over `max-width:6px` before transfer |

Captured geometry and browser identity are in [chrome155-observation.json](./chrome/chrome155-observation.json); the rendered input is [chrome155-image-ratio-transfer.png](./chrome/chrome155-image-ratio-transfer.png). The repository fixture is [grid-track-sizing-c114-image-ratio-transfer.html](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-transfer.html).

The evidence files are covered by [SHA256SUMS.txt](./SHA256SUMS.txt); verify from the repository root with `shasum -a 256 -c spec/internal/evidence/c11-4-image-ratio-transfer-2026-10-19/SHA256SUMS.txt`.

## Verification status

This file records Chrome's oracle observations; implementation and app results are linked separately in [the implementation review](./implementation-review-20.md), [the PR diff review](./pr-diff-review-30.md), and [the seven-scene simulator evidence](../c11-4-image-ratio-transfer-simulator-2026-10-19/README.md). The pure-Rust matrix compares 23 direct replaced-image cases in both applicable Grid axes, plus a scrollable-overflow control; two additional layout tests cover content-box and border-box decoration, and a separate non-replaced control checks the `AutomaticMinimum` source. CSS projection tests cover direct-image, profile, non-image, and nested-image boundaries. Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator each passed all seven app scenes. This limited slice does not complete C11.4 or C11.
