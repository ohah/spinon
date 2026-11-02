# Root HTML named-area + fractional tracks 사전 비교

## 고정 입력과 oracle

- Browser: Agent Browser가 구동한 `HeadlessChrome/156.0.0.0`.
- Input: [source.html](./source.html); hash is recorded in [hashes.txt](./hashes.txt).
- Screenshot: [chrome-agent-browser.png](./chrome-agent-browser.png); hash is recorded in [hashes.txt](./hashes.txt).
- Browser observation: [chrome-observation.json](./chrome-observation.json).
- Independent expected frames: [expected.json](./expected.json).
- Reproduction: `agent-browser --session spinon-c11-area-fr-precomparison open file:///tmp/spinon-c11-root-grid-fr-areas-precomparison.html`, then `eval --stdin` to collect UA, viewport, computed track lists, `grid-template-areas`, and each semantic item's DOMRect; screenshot; close the session. The session was closed after capture.

The source is an explicit 120×70 CSS-pixel Grid with definite width and height, two `1fr 2fr` axes, zero gaps, and the fixed rectangular area map `"header header" "side content"`. It contains exactly one empty block item for each area and reverses visual/source order to expose accidental DOM-position matching.

## Independent calculation

For each definite axis, the sum of flex factors is 3. CSS Grid §11.7.1 therefore gives `track = available size × factor / 3`: columns are `[40,80]`; rows are `[70/3,140/3]`. Named-area edges place `header` over both columns on row 1; `side` and `content` occupy row 2's first and second columns. The computed root and semantic item frames follow from those track edges. Independent exact rationals remain separate from browser-rounded coordinates.

## Observed Chrome geometry

- Root frame: `[0,0,120,70]`; computed columns `40px 80px`; computed rows `23.3281px 46.6719px`; computed areas `"header header" "side content"`.
- Header: `[0,0,120,23.328125]`.
- Side: `[0,23.328125,40,46.671875]`.
- Content: `[40,23.328125,80,46.671875]`.
- Every frame component is within 0.0053 CSS px of the independent exact result, below the planned 0.02 CSS px comparison tolerance.

The screenshot is visual evidence only. DOMRect and computed tracks are the browser geometry oracle; browser output alone does not establish Spinon support or standards conformance.

## Current-code reproduction

Before implementation, `crates/spinon-runtime/src/session/tests/root_html_grid_fr/rejection_matrix.rs` contains the `named areas with fraction` case. It supplies area rows with `grid-template-columns:1fr 2fr` and expects a `grid-template-areas` diagnostic with no completed layout. The projection's `validate_root_html_named_area_template()` currently accepts only exact 40/80px columns and 20/50px rows. This is the bounded gap addressed by the plan.

Current-source confirmation: `cargo test -p spinon-runtime c11_1_root_html_default_runtime_rejects_fraction_outside_its_bounded_profile` passed 1 focused test. That test asserts the named-area + fraction input fails with a `grid-template-areas` diagnostic and publishes no completed layout.
