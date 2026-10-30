# C11.4 direct-text aspect-ratio precomparison

**Source:** `origin/main` tree `57156c15a1d5c66a2ea2db57aac3d61ba2b35a5f` (commit `349dec1e5ffc5184a45e396688177209d725d2af`), 2026-10-30.

This is a Chrome reference observation for a proposed, opt-in Grid fixture slice. It is not a Spinon run, a WPT result, or evidence of general CSS support. The page was opened with Agent Browser in HeadlessChrome 156.0.0.0 at a 1280×633 CSS-pixel viewport and DPR 1. The Ahem `@font-face` loaded before geometry capture. The capture and screenshot are in this directory.

## Independent expected values

The fixture uses a 10px Ahem font with a 10px line-height. `A` therefore has a 10px max-content inline size and `AAAA` has 40px. Both Grid columns are max-content, and the grid itself is max-content sized. The preferred ratio is `2 / 1`.

| Scene | Independent calculation | Expected local geometry |
| --- | --- | --- |
| `row-ratio` | The 40px inline size and 2:1 ratio yield a 20px block size; the auto row accommodates the 20px contribution. | tracks `10px 40px` × `20px`; root `50×20`; items `(0,0,10,20)`, `(10,0,40,20)` |
| `row-control` | Without a preferred ratio, the two single-line items contribute 10px block size. | tracks `10px 40px` × `10px`; root `50×10`; items `(0,0,10,10)`, `(10,0,40,10)` |
| `column-ratio` | The definite 40px block size and 2:1 ratio yield an 80px inline size; the max-content column accommodates it. | tracks `80px 10px` × `40px`; root `90×40`; items `(0,0,80,40)`, `(80,0,10,40)` |
| `column-control` | Without a ratio, the text max-content inline sizes remain 40px and 10px. | tracks `40px 10px` × `40px`; root `50×40`; items `(0,0,40,40)`, `(40,0,10,40)` |

The CSS Grid 2025 CRD describes automatic grid-item sizing with a preferred aspect ratio in §6.2 and uses item size contributions when resolving intrinsic tracks in §11.5. CSS Sizing 3 defines intrinsic contributions and preferred aspect ratio. The fixture tests the item's ordinary ratio-aware sizing and its effect on the intrinsic track result; it does **not** use the replaced-item transferred-size-suggestion shortcut from Grid §6.6. References: [Grid §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing), [Grid §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [Sizing 3 §2.2](https://www.w3.org/TR/css-sizing-3/#intrinsic-size-contributions), [Sizing 4 aspect-ratio](https://www.w3.org/TR/css-sizing-4/#aspect-ratio).

## Chrome observation

Chrome 156 observed the four independent expected results exactly: row sizes 20px/10px and column tracks 80px+10px/40px+10px. The complete measured JSON is [`chrome-observation.json`](./chrome-observation.json); the viewport screenshot is [`chrome-156.png`](./chrome-156.png). Scene root positions in the raw capture are document coordinates; the table above normalizes item positions to each scene root.

The fixture uses only one ratio value, one Ahem font size, two direct block text items, HTML namespace, horizontal-tb/LTR defaults, zero item edges, max-content columns, normal alignment, and either one auto row or one definite 40px row. It does not establish behavior for general text, arbitrary ratios/fonts, writing modes, constraints, replaced elements, WPT coverage, paint pixels, or product CSS admission. The proposed runtime route still needs to verify the same input through Spinon's cascade, typed projection, Taffy track sizing, and Android/iOS simulator scenes.

## Inputs and hashes

- Fixture: `tests/fixtures/css/c11/grid-direct-text-aspect-ratio.html` — `9a50b61ad019782a16b2c321466cb2b5705c5d74f8934995abc84376d935bcad`
- Font: `tests/fixtures/resources/c15/Ahem.ttf` — `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Screenshot: `chrome-156.png` — `0c236e4b67e4ab4752b7014c439960eeba46b2dc775f341ff41d1ecf20ae183a`
