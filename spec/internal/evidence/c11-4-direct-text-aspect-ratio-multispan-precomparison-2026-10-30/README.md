# C11.4 direct-text aspect-ratio two-track span 사전 비교

## 범위

구현 전에 고정한 Ahem direct-text Grid 입력 9개를 CSS Grid track sizing의 multi-track contribution 규칙으로 독립 계산하고 Agent Browser HeadlessChrome 156에서 관측했다. 이 자료는 oracle 준비 단계다. Spinon 구현, WPT 실행, Android/iOS runtime 또는 화면 일치 근거가 아니다.

## 입력과 관측 방법

- Canonical HTML: [`grid-direct-text-aspect-ratio-multispan-precomparison.html`](../../../../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-multispan-precomparison.html)
- Root: width 100px, `max-content max-content` columns, one `auto` row, zero gap, one HTML Grid item.
- Item: direct text `AAAA`, `grid-column: 1 / span 2`, Ahem 10px/10px, numeric aspect ratio `2 / 1` unless the no-ratio control, auto width/height, zero margin/padding/border.
- Browser: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, macOS UA. `document.fonts.check("10px Ahem")` returned `true`.
- Captured DOM geometry and computed styles: [`chrome-observation-agent-browser.json`](./chrome-observation-agent-browser.json). Rendered view: [`chrome-oracle-agent-browser.png`](./chrome-oracle-agent-browser.png).

## 독립 계산과 브라우저 관측

CSS Grid 2025 CRD §11.5 resolves single-track items before adding the space needed by items spanning multiple tracks. §11.5.1 subtracts the sizes already present in all spanned tracks and distributes the remaining contribution equally among eligible tracks up to their limits. Here both tracks use the same `max-content` sizing function and there is one spanning item, so its inline contribution is split equally. Ahem `AAAA` has a 40px inline size and a 10px line box. A numeric 2:1 ratio maps each supported inline contribution to half its size in the block axis.

| Case | CSS constraints | Independent expected columns / row | Chrome 156 columns / row | item frame | text width / overflow |
|---|---|---:|---:|---:|---:|
| `span-baseline` | none | `[20,20]` / `20` | `[20,20]` / `20` | `40×20` | `40 / 0` |
| `span-min-width-45` | `min-width:45px` | `[22.5,22.5]` / `22.5` | `[22.5,22.5]` / `22.5` | `45×22.5` | `40 / 0` |
| `span-min-width-60` | `min-width:60px` | `[30,30]` / `30` | `[30,30]` / `30` | `60×30` | `40 / 0` |
| `span-min-height-30` | `min-height:30px` | `[30,30]` / `30` | `[30,30]` / `30` | `60×30` | `40 / 0` |
| `span-max-width-30-min-width-0` | `min-width:0;max-width:30px` | `[15,15]` / `15` | `[15,15]` / `15` | `30×15` | `40 / 10` |
| `span-max-height-15-auto-min-control` | `min-height:0;max-height:15px` | `[20,20]` / `15` | `[20,20]` / `15` | `40×15` | `40 / 0` |
| `span-max-height-15-min-width-0` | `min-width:0;min-height:0;max-height:15px` | `[15,15]` / `15` | `[15,15]` / `15` | `30×15` | `40 / 10` |
| `span-max-height-17-75-range` | `min-width:0;min-height:0;max-height:17.75px` | `[17.75,17.75]` / `17.75` | `[17.75,17.75]` / `17.75` | `35.5×17.75` | `40 / 4.5` |
| `span-no-ratio` | `aspect-ratio:auto` | `[20,20]` / `10` | `[20,20]` / `10` | `40×10` | `40 / 0` |

All nine DOM observations match the independently derived track/frame values. The `max-height` pair distinguishes the automatic minimum: without `min-width:0`, the 40px Ahem min-content floor keeps the inline contribution at 40px; with it, the ratio-adjusted 30px contribution distributes as 15px per track. For the latter cases, text width exceeds the item box by 10px or 4.5px and remains visible under the fixture's `overflow:visible` setting.

Re-run the artifact check with `node verify-precomparison.mjs`. It validates the nine case IDs, typed placement, computed ratio/font/constraints, track/row and root-relative item/text geometry, all hashes in this README, and a built-in one-pixel mutation that must be rejected. Final output: `Verified 9/9 Agent Browser geometry cases, artifact hashes, and a rejected geometry mutation.`

Normative source: [CSS Grid Layout 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#extra-space), [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), and [CSS Sizing Level 4 §4](https://www.w3.org/TR/css-sizing-4/#aspect-ratio). The browser values corroborate the calculation; they are not a WPT or product conformance result.

## SHA-256

| Artifact | SHA-256 |
|---|---|
| Canonical HTML | `70a6c4cc299c2acd180528a682272a1eab55ee615855b2b317b13fa212e141af` |
| Ahem TTF | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| Agent Browser observation JSON | `212d83ab5f91719c0d6da63ead2d822a239226d201b52adce6965594c1fbe4e9` |
| Independent expected values | `6be309b135b4a69879c673d79c8d29fa7e116c5e2d1377832fa49bb607bd5e56` |
| Agent Browser screenshot | `4c4429b195f4e18b160384534772a487ea9881f5a430477bc4705c36c5bbee04` |

## 한계

한 개 direct-text item의 두 equal `max-content` column span, numeric 2:1 ratio, 고정 Ahem font와 0..=60px min/max 입력만 비교했다. unequal/fixed/flexible/fit-content tracks, gaps, 더 긴 span, 다른 item과의 경쟁, image/replaced behavior, `auto <ratio>`, nonzero edges, arbitrary CSS/font, WPT execution, Spinon runtime, simulator, 실기기/hardware GPU, iOS compositor 후 pixel은 다루지 않는다.
