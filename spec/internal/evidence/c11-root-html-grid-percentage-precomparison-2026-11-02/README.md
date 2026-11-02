# C11.1 Root HTML Grid percentage-track precomparison

## Fixed input and independent expected values

The fixed multi-case HTML source is [`root-html-grid-percentage-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-percentage-precomparison.html), SHA-256 `2f58aa4b35548ce178a053afeeb56a788b6de0c52abbe6bab0ee34ad80ce71c7`. The positive-only [`root-html-grid-percentage-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-percentage-runtime.html) is also the planned simulator input. It contains the `both-axes` case and a second `overfull-columns` Grid whose 120% track places its second 8px marker at x=248px, beyond the 200px root edge. The expected values are computed from [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes): percentage columns use the definite inner inline size and percentage rows use the definite inner block size. Gaps advance grid lines without reducing those percentage bases. When the container size depends on its tracks, the percentage behaves as `auto` during intrinsic sizing and resolves against the resulting container size for layout.

The exact rational calculations and runtime-positive/negative scene partition are in [`expected.json`](./expected.json). The bounded positive set covers both axes, mixed percentage and fixed-pixel tracks, percentage plus `fr`, nonzero fixed-pixel gaps, a track list above 100%, the 0%/100% boundaries, and a definite zero-size axis. The simulator source adds a visible overfull track and places its marker outside the root edge. The indefinite row axis, percentage gap, and percentage tracks combined with named areas are recorded as valid CSS browser controls that this Root HTML profile will continue to reject.

## Agent Browser observation

Agent Browser 0.31.1 opened both local fixtures using HeadlessChrome 156.0.0.0 on macOS ARM at DPR 1, viewport 1280×633. The full source URLs, computed tracks/gaps, root frames, item origins, browser user agent, fixture hashes, and screenshot hashes are recorded in [`chrome156-observation.json`](./chrome156-observation.json). The screenshots are [`chrome156-agent-browser.png`](./chrome156-agent-browser.png) and [`chrome156-runtime-scene.png`](./chrome156-runtime-scene.png).

[`SHA256SUMS.txt`](./SHA256SUMS.txt) binds both fixtures, the independent expected values, browser observations, and screenshots.

Chrome agreed with the independent expected values in the definite-axis scenes. In particular, the mixed two-axis case returned columns `60px 120px 20px` and rows `30px 60px 10px`; the `25% 1fr` column case returned `60px 170px` after one 10px gap; the 120%/10% case returned `240px 20px` with the second marker at x=248px; and the 0%/100% and zero-basis cases returned `0px 200px` and `0px 0px`. The updated positive runtime fixture independently returned the same 240×120 primary Grid and a second 200×40 Grid with columns `240px 20px`, 8px column gap, and item origins x=0/248. Its screenshot keeps the blue marker visible to the right of the dark root background. The indefinite row control returned two 6px rows and an auto-height 12px grid, consistent with the specification's intrinsic-sizing behavior. These are browser observations, not product-runtime expectations for the rejected cases.

## Boundary

This precomparison does not test Spinon, Android/iOS, surface pixels, or WPT. The fixed positive input uses no border, padding, margins on grid items, intrinsic content, explicit placement, implicit tracks, named areas, or non-pixel gap. It does not by itself establish C11.1 or C11 completion.
