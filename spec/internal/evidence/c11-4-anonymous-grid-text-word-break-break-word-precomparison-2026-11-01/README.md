# C11.4 anonymous Grid text `word-break: break-word` precomparison

This precomparison separates CSS Text's deprecated `word-break: break-word` mapping from the current `overflow-wrap: break-word` behavior. It is an oracle and independent fixture contract, not Spinon implementation evidence.

## Fixed input

The local fixture [`grid-track-sizing-c114-anonymous-text-break-word.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-break-word.html) uses 18 anonymous direct-text Grid scenes: `word-break: normal|break-word` × `overflow-wrap: normal|break-word|anywhere` × `min-content|max-content|10px` columns. Each Grid has a definite 120px width, one direct Text node `XXXX`, one auto row, no gap or edges, Ahem 10px/400, 10px line-height, `white-space: normal`, and horizontal-tb/LTR.

## Normative rules

[CSS Text Level 3 §5.1](https://www.w3.org/TR/css-text-3/#word-break-property) defines `word-break: break-word` as equivalent to `word-break: normal` plus `overflow-wrap: anywhere`, regardless of the actual `overflow-wrap` value, and says `word-break` affects intrinsic-size calculations. [CSS Text Level 3 §5.4](https://www.w3.org/TR/css-text-3/#overflow-wrap-property) distinguishes `overflow-wrap: anywhere`, whose emergency opportunities affect min-content sizing, from `overflow-wrap: break-word`, whose emergency opportunities do not affect min-content sizing. Grid track expectations use [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes) and [CSS Grid Level 1 §11](https://www.w3.org/TR/css-grid-1/#algo-track-sizing).

## Independent Ahem expectations

Pinned Ahem's U+0058 glyph advances 10px. `XXXX` therefore has a 40px max-content width. The expected columns are:

| Computed `word-break` | Computed `overflow-wrap` | `min-content` column | `max-content` column | Fixed `10px` row / text lines |
| --- | --- | ---: | ---: | --- |
| `normal` | `normal` | 40px | 40px | 10px / 1 unwrapped line; text advance overflows the track |
| `normal` | `break-word` | 40px | 40px | 40px / 4 emergency-wrapped lines |
| `normal` | `anywhere` | 10px | 40px | 40px / 4 emergency-wrapped lines |
| `break-word` | `normal` | 10px | 40px | 40px / 4 emergency-wrapped lines |
| `break-word` | `break-word` | 10px | 40px | 40px / 4 emergency-wrapped lines |
| `break-word` | `anywhere` | 10px | 40px | 40px / 4 emergency-wrapped lines |

For the three `min-content` scenes with a 10px contribution, the resolved root row is 40px with four text fragments; with a 40px contribution it is 10px with one fragment. Every `max-content` scene resolves to one 40px line. The fixed `10px` column resolves to 10px in all scenes, while its row distinguishes wrapping from overflow. The machine-readable independent values are in [`expected-values.json`](./expected-values.json).

## Browser observation

Chrome 156 Agent Browser measured all 18 scenes after the pinned Ahem font loaded. Its raw values, browser identity, computed properties, track strings, row heights, line counts, and text rectangles are in [`chrome156-observation.json`](./chrome156-observation.json). The observation verifier is [`verify-observation.mjs`](./verify-observation.mjs). A screenshot of the rendered matrix is [`chrome156-agent-browser.png`](./chrome156-agent-browser.png).

The Chrome observation is cross-check evidence; the expected table above follows CSS Text/Sizing/Grid rules and fixed Ahem glyph advances. It does not prove Spinon behavior, public CSS admission, WPT conformance, physical-device behavior, hardware GPU output, or completion of C11.4/C11.

## Reproduction metadata

- Agent Browser session `c114-break-word-precomparison` opened the local fixture in HeadlessChrome `156.0.0.0`, DPR 1. Ahem font loading and `CSS.supports("word-break", "break-word")` both returned true.
- The fixture font is [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- [`verify-observation.mjs`](./verify-observation.mjs) compared computed values, columns, rows, root frames, line counts and per-line advances; it reported 18/18 expected scenes.
- Agent Browser was closed after capture. A follow-up `pgrep -afil agent-browser` found no Agent Browser process; its only output was the process-check shell itself.
- W3C normative sections were viewed with the same Agent Browser session at the two links above. Their requirements are paraphrased here; the Chrome observation remains separate from the independent expectations.
