# C11.4 direct mixed text/image non-spanning precomparison

## 판정 입력

- Standards: CSS Grid Layout Module Level 1, 2025 CRD §7.2.1 and §11.5 step 2. Each intrinsic single track is resolved from the largest relevant contribution among items wholly contained in it.
- Chrome oracle: Agent Browser CLI 0.31.1, HeadlessChrome 156.0.8078.48, macOS ARM64, viewport 1280×633 CSS px, DPR 1.
- Fixture: [`grid-mixed-direct-text-image-intrinsic-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-mixed-direct-text-image-intrinsic-c114-precomparison.html). It contains four scenes: min/max-content track × forward/reversed direct text and image source order.
- Text: Ahem 10px/10px, `XX XXXXX`, normal whitespace and word breaking. Ahem advance is 10px; min-content is the five-glyph longest word (`50px`) and max-content is all eight glyphs including the space (`80px`).
- Replaced item: [`c14-1-png-basic.png`](../../../../tests/fixtures/resources/c14-1-png-basic.png), natural size 24×16 CSS px.
- Root: definite content size 120×48px, two fixed 24px rows, one intrinsic column, no gap, border or padding. Each child is in column 1 and spans one track.

## Independent expectations

For the single `min-content` track, §11.5 step 2 takes the maximum min-content contribution: `max(50px, 24px) = 50px`. For the single `max-content` track, the max-content growth limit is the maximum max-content contribution: `max(80px, 24px) = 80px`. The 120px definite content width leaves enough space but cannot exceed that growth limit. Reversing source order must preserve each track size.

## Chrome observation

`chrome156-observation.json` preserves the browser version, viewport, font/image readiness, used track CSSOM values and item/root frames. All four scene track widths are 50/80px as independently expected. The validator checks the saved input fields and scene IDs, source-order pairs, screenshot presence and SHA-256 of the HTML fixture before comparing measurements. The Chrome output is an observation, not the source of normative expected values. `chrome156-agent-browser.png` is a 1280×633 viewport capture; the complete observation data is in the JSON file.

Reproduce the arithmetic/observation check with:

```sh
python3 spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/validate-precomparison.py
```

Result: `4/4 scenes PASS`, with min-content 50px and max-content 80px. This validates the saved Chrome observation against the independent model. It does not execute Spinon or prove CSS support.

## Limits

No Spinon implementation, Rust runtime, Android/iOS app, simulator, artifact boundary, WPT, real device or post-compositor pixel was tested. The plan covers only direct text plus one natural-size image, span 1, one intrinsic column and four fixed scene IDs. Nested/anonymous items, arbitrary CSS, larger spans and full C11.4 remain outside this precomparison.
