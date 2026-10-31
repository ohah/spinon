# C11.4 anonymous Grid text computed `line-break` 사전 비교

## 목적과 경계

Chrome 156의 직접-text Grid 입력에서 computed `line-break`와 일본어 `lang`이 min-content track sizing에 미치는 효과를 관찰했다. 이 문서는 구현 후보의 oracle 기록이다. Spinon 동작, 공개 CSS 지원, WPT 적합성이나 C11.4 완료를 증명하지 않는다.

## 고정 환경

- Oracle: Agent Browser가 연 headless Chrome 156 (`Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/156.0.0.0 Safari/537.36`).
- Font: `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf`, SHA-256 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`; `document.fonts.check()` returned true for `文〜文` at 10px.
- Font table: the variable font has only a `wght` axis (100–900, default 100). `fontTools` instantiated at 400 reports U+6587 `文` (`uni2F42`) and U+301C `〜` (`uni301C`) at 1000/1000 em each; each advances 10 CSS px at 10px/400. Both codepoints exist in the font cmap and HVAR data was applied when instantiating.
- Text: `文〜文`, one direct text node; `word-break:normal`, normal white-space wrapping, 10px font and 10px line-height.
- Grid: 120px definite width, zero edges/gap, one `min-content` or `max-content` column and one auto row. This matches the existing anonymous-text fixture's outer layout inputs.
- Runtime-style capture: [Chrome 156 scene image](chrome156-runtime-style-grid.png); [machine-readable observed values](chrome-observation.json).
- Intrinsic sizing control: [shrink-to-fit comparison image](chrome156-line-break-grid.png).

## 규범과 독립 기대값

[CSS Text Level 3 §5.2](https://www.w3.org/TR/css-text-3/#line-break-property) says a break before U+301C is allowed for `line-break:normal` when the writing system is Chinese or Japanese, and otherwise forbidden. `strict` uses the more restrictive set. Japanese `normal` can therefore break before the wave dash; Japanese `strict` and English `normal` cannot. A break before the trailing ideograph remains available. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes) defines the min-content contribution from the longest unbreakable segment; [CSS Grid Level 1 §11](https://www.w3.org/TR/css-grid-1/#algo-track-sizing) applies that contribution to the `min-content` track.

The pinned font's advances make the independent geometry direct: each of the three glyphs is 10px; normal Japanese can break into three one-glyph segments; strict Japanese and normal English have a longest two-glyph segment. At 10px line-height, the grid's auto row follows the resulting line count.

| `lang` | computed `line-break` | column sizing | Chrome column track | Chrome row track | Grid root frame |
| --- | --- | --- | ---: | ---: | ---: |
| `ja` | `normal` | `min-content` | 10px | 30px | 120×30px |
| `ja` | `strict` | `min-content` | 20px | 20px | 120×20px |
| `en` | `normal` | `min-content` | 20px | 20px | 120×20px |
| `ja` | `normal` | `max-content` | 30px | 10px | 120×10px |

Chrome reported computed `grid-template-columns` as `10px`, `20px`, `20px`, and `30px`; computed `grid-template-rows` were `30px`, `20px`, `20px`, and `10px`. Each Grid root had exactly one direct Text child (`nodeType 3`) and the pinned font was loaded. The first screenshot shows the exact 120px fixture width and auto-row profile; the second is a shrink-to-fit cross-check.

## 재현 및 한계

A loopback-only Python HTTP server served a temporary page so Chrome could load the pinned repository font. Agent Browser opened that page, waited for the font, read computed tracks and frames, and saved the screenshots. Both browser sessions and the temporary server were closed after capture. The temporary inputs are under `/tmp/spinon-c114-line-break-oracle-20261101/`; they are not repository sources.

This matrix covers one Japanese punctuation distinction, one English negative control, one font, horizontal writing, one direct text node and two intrinsic track keywords. It does not cover `auto`, `loose`, `anywhere`, other language inheritance, nested elements, arbitrary fonts, general author CSS admission, WPT, Spinon glyph paint or whole C11.4/C11.
