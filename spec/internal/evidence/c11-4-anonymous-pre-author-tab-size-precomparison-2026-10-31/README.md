# C11.4 anonymous Grid `white-space: pre` author `tab-size` precomparison

## Purpose and limits

This record fixes Chrome observations and independent expectations for a possible C11.4 opt-in fixture extension. It covers preserved tabs in one direct anonymous Grid text paragraph with Ahem 10px text and numeric `tab-size` values 1, 2, and 4. It does not claim Spinon support, general CSS admission, WPT conformance, or C11.4/C11 completion.

## Normative basis and independent calculation

CSS Text Level 3 §4.2 defines `tab-size` as an inherited property whose initial value is 8. A numeric value is a multiple of the U+0020 advance. The preserved tab is laid out at the next tab stop relative to the inline content start; a forced segment break resets the position for the next line. The fixed Ahem font has a 10px space advance at 10px font size, so values 1, 2, and 4 produce 10px, 20px, and 40px stop intervals. The independent expected rows in [`expected-values.json`](./expected-values.json) use this arithmetic and do not derive from Chrome CSSOM.

The baseline 8-stop cases and the oracle-only `tab-size:2` rejection are already recorded in the [default-tab plan](../../../../plan/c11-4-anonymous-grid-text-pre-tabs.md). This comparison adds exact-stop, consecutive-tab, LF/CRLF line-reset, trailing-tab, min-content, narrow fixed-track, and additional numeric-value controls.

## Browser observation

Agent Browser loaded the checked-in [fixture](../../../../tests/fixtures/css/c11/anonymous-grid-text-pre-author-tab-size-precomparison.html) in HeadlessChrome 156.0.0.0 at 1280×633 CSS px, DPR 1. It confirmed the Ahem font loaded and recorded computed `tab-size`, Grid used tracks, root frames, and `Range.getClientRects()` in [`chrome-observation.json`](./chrome-observation.json). The screenshot is [`chrome-156-agent-browser.png`](./chrome-156-agent-browser.png). These are browser observations only.

The input fixture SHA-256 is `d3d728c192d1d99ee124ac4e6e809d9968c14907542295ffaa83ff66cbc9ccc5`; the expected-values SHA-256 is `d7f22069f0c3d38a8d4966b2182480010ddffa319899a6cbe85eea49cea4683c`; the raw observation SHA-256 is `b55bff37c17227c8a556bf6ba23aaf38bf8e6bf32fc9b119bf89389ae37019ef`; the screenshot SHA-256 is `05228009f2def6e14f8e0d9803e68303b63ef8b5baa5f822ff6248f29c471a30`. The Ahem resource SHA-256 is `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.

## Observation summary

| Case | `tab-size` | Used columns / rows | Root frame | Text line advance(s) |
|---|---:|---:|---:|---:|
| `size2-two-columns` | 2 | 60px / 10px | 60×10px | 60px |
| `size2-leading` | 2 | 30px / 10px | 30×10px | 30px |
| `size2-exact-stop` | 2 | 70px / 10px | 70×10px | 70px |
| `size2-consecutive` | 2 | 70px / 10px | 70×10px | 70px |
| `size2-reset` | 2 | 60px / 20px | 60×20px | 60px, 30px |
| `size2-crlf-reset` | 2 | 30px / 20px | 30×20px | 20px, 30px |
| `size2-trailing` | 2 | 40px / 10px | 40×10px | 40px |
| `size2-min-content` | 2 | 60px / 10px | 60×10px | 60px |
| `size2-fixed-overflow` | 2 | 25px / 10px | 25×10px | 60px, no soft wrap |
| `size1-leading` | 1 | 20px / 10px | 20×10px | 20px |
| `size1-exact-stop` | 1 | 30px / 10px | 30×10px | 30px |
| `size4-leading` | 4 | 50px / 10px | 50×10px | 50px |
| `size4-exact-stop` | 4 | 90px / 10px | 90×10px | 90px |

No glyph ink or platform renderer pixels were compared. Text range rectangles show browser line layout, not glyph raster output.
