# C11.3.3 leading and trailing named implicit lines · precomparison

**Plan:** [leading named implicit lines](../../../../plan/c11-3-3-leading-named-implicit-lines.md)
**Normative source:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)
**Fixture:** [HTML](../../../../tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133.html) · [inventory](../../../../tests/fixtures/css/c11/grid-leading-named-implicit-lines-c1133-inventory.json) · [captured reference](../../../../tests/fixtures/css/references/c11-3-3-leading-named-implicit-lines-precomparison-v1.json)

## Oracle and capture boundary

The seven-scene local fixture was opened and measured with Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, viewport 1280×633 CSS px, DPR 1, LTR and horizontal-tb. The captured CSSOM value is `grid-template-columns`; item rectangles are relative to their grid roots. The screenshot is `chrome-headless-156.png` in this directory.

Agent Browser did not expose the Chromium binary revision or executable hash. The browser observation is therefore bounded by the reported user agent and is not a binary-pinned comparison. CSS Grid 2025 CRD §8.3 is the behavior authority. Its named-span example places an end at line 4, excludes line 3 from a startward search because it lies on the endward side, and requires a startward implicit line. The fixture also checks the matching occurrence rules and authored-name precedence.

## Captured values

| Scene | Resolved columns | Root size | Item rect relative to root |
|---|---|---|---|
| positive-missing-occurrences | `[30, 40, 15, 7, 15, 7]` | `[114, 20]` | `[107, 0, 7, 20]` |
| negative-missing-occurrences | `[15, 7, 30, 40]` | `[92, 20]` | `[0, 0, 15, 20]` |
| backward-span-one | `[7, 100, 15, 7]` | `[129, 20]` | `[0, 0, 129, 20]` |
| backward-span-two | `[15, 7, 100, 15, 7]` | `[144, 20]` | `[0, 0, 144, 20]` |
| backward-span-partial-explicit | `[7, 40, 100, 15]` | `[162, 20]` | `[0, 0, 162, 20]` |
| forward-span-two | `[50, 15, 7, 15, 7]` | `[94, 20]` | `[72, 0, 22, 20]` |
| backward-span-uses-authored-name | `[30, 40, 15]` | `[85, 20]` | `[30, 0, 55, 20]` |

SHA-256 values for the source fixture, inventory, reference and screenshot are recorded in the reference JSON and were checked when this record was prepared. This is precomparison only; no Spinon implementation result is implied.
