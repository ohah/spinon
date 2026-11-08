# C11.4 repeated intrinsic implicit-row list precomparison

This record separates the normative CSS Grid rule, independently calculated expected geometry, and Chrome 157 observations for one bounded six-member `grid-auto-rows` list.

## Normative rule

[CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks) defines `grid-auto-rows` as `<track-size>+`. When multiple track sizes are specified, the pattern repeats as needed. The first implicit track uses the first member. Repeated occurrences of `min-content` and `max-content` are distinct list members; they retain their position in that pattern.

The candidate list is:

```css
fit-content(20px) min-content max-content auto min-content max-content
```

The comparison fixture contains a definite 40×100px grid, one explicit 10px row, and eight implicit-row items. The second six-member cycle reaches the repeated `fit-content()` at implicit row seven and the repeated `min-content` at row eight. A 60px child in the seventh item tests its 20px track cap and visible overflow beyond the grid's bottom border.

## Independent expected values

The content payload heights for implicit rows 1–8 are `[20, 10, 12, 6, 6, 6, 60, 6]px`. The intended track sizes are:

- `align-content:start`: `[10, 20, 10, 12, 6, 6, 6, 20, 6]px`, total 96px, leaving 4px of free space.
- `align-content:stretch`: `[10, 20, 10, 12, 10, 6, 6, 20, 6]px`, total 100px; only the `auto` track receives the 4px.

`fit-content(20px)` and `fit-content(20%)` have the same 20px cap because the block-size basis is definite at 100px. The repeated fit-content track at implicit row seven remains 20px while its 60px payload extends 30px beyond the grid bottom for `start` and 34px for `stretch`.

The values above are derived from the CSS Grid track list and the explicit fixed-size payloads. They were not generated from Spinon. `expected.json` records them, and `verify-observation.mjs` checks all four scenes against the browser observations.

## Chrome observation

- Browser: Agent Browser, HeadlessChrome 157.0.0.0, macOS arm64
- Viewport: 1280×633 CSS px, DPR 1
- Fixture: [`root-html-grid-repeated-intrinsic-auto-rows-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-precomparison.html)
- Captured output: [`chrome-observation.json`](./chrome-observation.json)
- Screenshot: [`chrome-headless157.png`](./chrome-headless157.png)
- Independent oracle: [`expected.json`](./expected.json)
- Verifier: [`verify-observation.mjs`](./verify-observation.mjs)

Chrome reported the expected four used-track arrays, preserved both repeated members in computed `grid-auto-rows`, and placed the 60px payload outside the grid border while retaining a 20px fit-content track. This is a browser precomparison for the fixed fixture only; it does not prove general CSS Grid conformance or Spinon behavior.
