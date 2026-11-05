# C11.1 intrinsic Grid percentage-gap precomparison

**Status:** implementation candidate only. This evidence records the fixed reference input and expected results; it does not claim that Spinon supports percentage gaps in intrinsic Grid track sizing.

## Inputs and references

- Fixture: [root-html-grid-intrinsic-gap-percentage-precomparison.html](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-gap-percentage-precomparison.html)
- Fixture SHA-256: 17971089c82f36325020d8ded56caec4b987a9c8f2b869ae11979b1c6df65a4e
- Browser: Agent Browser with HeadlessChrome 156.0.0.0
- Viewport: 1280×720 CSS px
- Raw observation: [chrome156-observation.json](./chrome156-observation.json)
- Independent expected values: [expected.json](./expected.json)
- Comparison report: [comparison.json](./comparison.json)
- Browser screenshot: [chrome156-full.png](./chrome156-full.png)
- SHA-256 manifest: [SHA256SUMS.txt](./SHA256SUMS.txt)

## Independent calculation

The grid axes are definite fixed CSS-pixel dimensions and have no border, padding, or margin, so their content-box bases equal their stated width and height. CSS Gaps Level 1 §2.3 defines the percentage basis on the corresponding content-box axis. It separately defines cyclic percentage gaps as zero for intrinsic size contributions and as content-box percentages when laying out contents. These fixtures do not have a cyclic basis: both grid axes are explicitly definite. CSS Grid Layout 2025 CRD §10.1 treats each used gutter as an empty fixed-size track, and §11.1 includes gutters in available-space calculations.

For each scene, the first column is min-content: 40px. The first row is min-content: max(10px, 20px) = 20px. The second track on each axis receives the non-negative remainder after subtracting the first track and one used gutter.

For the 100×100px scenes, 10% resolves to 10px on either axis. The resulting track lists are columns [40, 50] and rows [20, 70] when both gutters are present. In the mixed-basis 120×80px scene, column-gap 10% resolves to 12px and row-gap 25% to 20px, yielding columns [40, 68], rows [20, 40], and the expected item origins x=52px and y=40px.

Percentage gaps over 100% are not capped. At 120%, the gutter is 120px; the flexible second track floors at 0px, while a fixed-size item in that track remains positioned after the gutter and visibly overflows the grid.

## Chrome observation

Chrome 156 reports the authored percentage in computed rowGap/columnGap. The used gutter is checked separately from the resolved track sizes and item positions. Its nine scenes match the independent geometry in expected.json, including the distinct row/column bases, 0%, values over 100%, and the legacy grid-gap alias.

The fixture contains no indefinite axis or cyclic percentage contribution. It does not test math-backed gaps, arbitrary Grid grammar, the Spinon runtime, pixels, or WPT conformance.

The Agent Browser session was closed after capture; the session list was empty and a subsequent process check found no Agent Browser process.

Normative sources: [CSS Gaps Module Level 1](https://www.w3.org/TR/css-gaps-1/) and [CSS Grid Layout Module Level 1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).
