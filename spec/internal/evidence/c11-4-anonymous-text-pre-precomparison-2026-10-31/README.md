# C11.4 anonymous Grid text `white-space:pre` 사전 비교

## 환경과 입력

- Agent Browser 0.31.1, HeadlessChrome 156.0.8078.76: `Mozilla/5.0 ... HeadlessChrome/156.0.0.0 ...`; UA Client Hints reported the full version.
- `document.fonts.check("10px Ahem")` returned `true`.
- Fixture: [`anonymous-grid-text-pre-precomparison.html`](../../../../tests/fixtures/css/c11/anonymous-grid-text-pre-precomparison.html).
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- Fixture SHA-256 at observation time: `197cec44aa9b80d8b5e9af2170c8e1efbbe2b43456a9d1ed8c8c664fb8dd7353`.
- Browser screenshot: [`chrome-156.png`](./chrome-156.png).

## 관찰값

| Scene | Computed mode | Grid track | Grid root | Text Range line boxes |
|---|---|---:|---:|---|
| normal-max-content | collapse / wrap | 50px | 50×10px | 50×10px |
| pre-max-content | preserve / nowrap | 60px | 60×10px | 60×10px |
| normal-min-content | collapse / wrap | 20px | 20×20px | 20×10px × 2 |
| pre-min-content | preserve / nowrap | 50px | 50×10px | 50×10px × 1 |
| pre-forced-break | preserve / nowrap | 20px | 20×20px | two 20×10px lines; Range also reports a zero-width LF rect |
| normal-fixed-width | collapse / wrap | 25px | 25×20px | 20×10px × 2 |
| pre-fixed-width | preserve / nowrap | 25px | 25×10px | 50×10px × 1 |

The results are browser observations for the exact fixture. The 50px pre min-content and nowrap overflow follow the preserved-space/no-soft-wrap semantics of CSS Text; this artifact alone does not assert general browser conformance or Spinon behavior.

## 범위

This precomparison was performed before implementation. It covers direct anonymous Grid text, Ahem 10px, horizontal writing mode, the listed ASCII strings, min/max-content tracks and one fixed-width overflow control. It does not cover tabs, arbitrary text/fonts, general CSS admission, WPT, mobile, physical devices or the rest of C11.4.
