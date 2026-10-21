# C11.4 nested Grid two-axis intrinsic sizing precomparison

## Purpose

This is plan-stage evidence for a bounded follow-up to [the depth-two nested Grid fixture](../../../../plan/c11-4-nested-grid-depth-two.md). It adds simultaneous inline- and block-axis intrinsic sizing to the existing fixed Ahem text topology. It does not report Spinon implementation behavior.

## Fixed input and browser observation

- Input: [HTML fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-two-axis-precomparison.html)
- Browser: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, macOS ARM, viewport `1280×633`, DPR 1.
- Font readiness: `document.fonts.check('10px Ahem') === true` before capture.
- Browser observation: [JSON](./chrome156-observation.json)
- Screenshot: [PNG](./chrome156-nested-grid-two-axis.png)
- Independent expected values: [JSON](./spec-derived-expectations.json)

Chrome reports matching `70×20px` sizes and `[70px]`/`[20px]` tracks in both min-content controls, and `100×10px` with `[100px]`/`[10px]` tracks in both max-content controls. In every case each nested Grid and its direct text item has the root-relative origin `(0,0)`. The absolute page offsets in the Chrome JSON are fixture placement details only.

## Independent calculation

The repository Ahem font at 10px gives `XX XXXXXXX` a 70px longest unbreakable word and a 100px unwrapped advance. CSS Grid 2025 CRD §5.2 defines intrinsic Grid container sizes from its sized tracks; §11.1 sizes columns before rows. Under the fixed one-track, zero-gutter setup, min-content inline sizing yields a 70px column and two 10px text lines, so the matching min-content row and each nested Grid's block size are 20px. Max-content inline sizing yields 100px and one 10px line, giving a 10px row. The same one-item contribution passes through each fixed nesting level. These are spec-derived expectations; Chrome is recorded independently as an observation.

## Limits

This checks one Ahem text item, one track per axis, depth-one and depth-two controls, and one pinned Chrome build. It does not prove Spinon behavior, browser interoperability, arbitrary nested Grid sizing, general CSS support, WPT conformance, glyph paint equality, or C11.4 completion. The implementation plan keeps the fixture opt-in and fail-closed.

## Hashes

`SHA256SUMS.txt` binds the HTML input, Ahem font, observations, expected values, and screenshot. Regenerate it from the repository root after any evidence change.
