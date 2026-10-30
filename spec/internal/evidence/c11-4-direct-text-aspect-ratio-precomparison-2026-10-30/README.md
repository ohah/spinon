# C11.4 direct-text aspect-ratio precomparison

**Base:** `origin/main` tree `8da1c9ead6bb0320021cde11165be1a9c009b74b` (commit `0906ed5f47b4d3964c14ee0f96eca1b24a4bd945`), 2026-10-30. This browser run uses the 100px-root scene revision in the current implementation branch; its input fixture hash is recorded below.

This is a Chrome reference observation for a proposed, opt-in Grid fixture slice. It is not a Spinon run, a WPT result, or evidence of general CSS support. The page was opened with Agent Browser in HeadlessChrome 156.0.0.0 at a 1280×633 CSS-pixel viewport and DPR 1. The Ahem `@font-face` loaded before geometry capture. The capture and screenshot are in this directory.

## Independent expected values

The fixture uses a 10px Ahem font with a 10px line-height. `A` therefore has a 10px max-content inline size and `AAAA` has 40px. Both Grid columns are max-content, the grid has a definite 100px width, and the preferred ratio is `2 / 1`. The content-sized columns leave unused space inside the root.

| Scene | Independent calculation | Expected local geometry |
| --- | --- | --- |
| `row-ratio` | The 40px inline size and 2:1 ratio yield a 20px block size; the auto row accommodates the 20px contribution. | tracks `10px 40px` × `20px`; root `100×20`; items `(0,0,10,20)`, `(10,0,40,20)` |
| `row-control` | Without a preferred ratio, the two single-line items contribute 10px block size. | tracks `10px 40px` × `10px`; root `100×10`; items `(0,0,10,10)`, `(10,0,40,10)` |
| `column-ratio` | The definite 40px block size and 2:1 ratio yield an 80px inline size; the max-content column accommodates it. | tracks `80px 10px` × `40px`; root `100×40`; items `(0,0,80,40)`, `(80,0,10,40)` |
| `column-control` | Without a ratio, the text max-content inline sizes remain 40px and 10px. | tracks `40px 10px` × `40px`; root `100×40`; items `(0,0,40,40)`, `(40,0,10,40)` |

The CSS Grid 2025 CRD describes automatic grid-item sizing with a preferred aspect ratio in §6.2 and uses item size contributions when resolving intrinsic tracks in §11.5. CSS Sizing 3 defines intrinsic contributions and preferred aspect ratio. The fixture tests the item's ordinary ratio-aware sizing and its effect on the intrinsic track result; it does **not** use the replaced-item transferred-size-suggestion shortcut from Grid §6.6. References: [Grid §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing), [Grid §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [Sizing 3 §2.2](https://www.w3.org/TR/css-sizing-3/#intrinsic-size-contributions), [Sizing 4 aspect-ratio](https://www.w3.org/TR/css-sizing-4/#aspect-ratio).

## Chrome observation

Chrome 156 observed the four independent expected results exactly: row sizes 20px/10px and column tracks 80px+10px/40px+10px inside the same 100px root. The complete measured JSON is [`chrome-observation.json`](./chrome-observation.json); the viewport screenshot is [`chrome-156.png`](./chrome-156.png). Scene root positions in the raw capture are document coordinates; the table above normalizes item positions to each scene root.

The fixture uses only one ratio value, one Ahem font size, two direct block text items, HTML namespace, horizontal-tb/LTR defaults, zero item edges, max-content columns, normal alignment, and either one auto row or one definite 40px row. It does not establish behavior for general text, arbitrary ratios/fonts, writing modes, constraints, replaced elements, WPT coverage, paint pixels, or product CSS admission. At capture time the proposed runtime route was still pending; its later Android/iOS simulator results are recorded in [runtime evidence](../c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.md).

## Inputs and hashes

- Fixture: `tests/fixtures/css/c11/grid-direct-text-aspect-ratio.html` — `f507edb05f1554e6d2c8ff4fac483c67021f38bc9f24bd850f2d8c1d7875d3ec`
- Font: `tests/fixtures/resources/c15/Ahem.ttf` — `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Screenshot: `chrome-156.png` — `b90e14811966b88f7da715b5362f70a2671cd8680107162aea945ec286156a7c`
