# Root HTML four-member intrinsic implicit-row-list precomparison

## Oracle and fixture

- Chrome: Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1.
- Fixture: [`root-html-grid-fit-min-max-content-auto-implicit-row-list-precomparison.html`](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-precomparison.html), SHA-256 `fb1dea45dfe225df0e871ec376d1c7ee91cbc8bffd531d5b7ef662408af4d369`.
- The page has 24 permutations of `fit-content(20px|20%)`, `min-content`, `max-content`, and `auto`; each runs at `align-content:start|stretch`. This yields 96 scenes. Each fixed 40×100px Grid has a 40px explicit column, a 10px explicit row, zero gap/edges, visible overflow, and five implicit items with fixed payload heights `[20,10,12,6,20]px`.
- The fifth implicit row repeats the first track-list member. Its payload is deliberately equal to the first row's payload so the cycle can be checked by both track identity and equal used size.
- The fixture is a static Chrome oracle. It does not exercise Spinon or claim WPT/CSS Grid conformance.

## Independent expected values

The independent verifier [`verify-chrome-observation.mjs`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/verify-chrome-observation.mjs) builds expected rows from the fixed payloads and the CSS Grid track-list/stretched-auto-track rules, then checks every captured case. Its result is [`oracle-verification.json`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/oracle-verification.json): **96/96 cases matched**, covering all 24 orders, both cap forms and both alignment values. The full raw observations and all item/payload frames are in [`chrome-observation.json`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/chrome-observation.json).

With `align-content:start`, every case resolves used rows `[10,20,10,12,6,20]px`; total used size is 78px and 22px remains free. With `stretch`, only the auto max-sizing track positions grow. A list with one auto row receives 22px there. If the first list member is auto, the fifth implicit row is a second auto track and the two each receive 11px. The definite 100px block-size makes `20%` and `20px` cap cases geometrically equal while computed style preserves the authored cap unit.

This fixed-height payload set has equal min-content and max-content contributions. Therefore, it does not separate their geometry; the oracle specifically checks that Chrome retains the distinct computed track functions in every authored order. The plan does not use this fixture to claim arbitrary content's min/max algorithm parity.

## Captures and browser lifecycle

![Chrome 157 first start scene](./chrome-first-scene.png)

![Chrome 157 repeated-auto stretch scene](./chrome-repeated-auto-stretch.png)

- Pass01's invalid cycle payload pairing is retained as [pass01 observations](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/chrome-observation-pass01.json) and screenshots. Pass01 is excluded from the 96/96 result; the correction and reason are recorded in the [plan review](https://github.com/ohah/spinon/blob/main/plan/c11-4-root-html-fit-min-max-content-auto-rows-plan-review-20.md).
- Agent Browser reported no page errors or console messages. The session closed successfully and a post-close process search found no Agent Browser/Chrome process; see [agent-browser-lifecycle.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/agent-browser-lifecycle.json).

## Normative references

- [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks): `grid-auto-rows` takes one or more track sizes.
- [§12.5 intrinsic track sizing](https://www.w3.org/TR/css-grid-2/#algo-content): min-content and max-content functions use their corresponding contributions.
- [§12.8 stretched auto tracks](https://www.w3.org/TR/css-grid-2/#algo-stretch): remaining positive definite free space is divided equally among tracks with an auto max-sizing function.
