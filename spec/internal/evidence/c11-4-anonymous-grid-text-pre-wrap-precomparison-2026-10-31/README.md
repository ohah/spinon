# C11.4 anonymous Grid text `white-space:pre-wrap` precomparison

## 판정 범위

이 자료는 C11.4 fixture 계획을 위한 Chrome 관측과 독립 CSS/Ahem 기대값이다. 아직 Spinon 구현, WPT harness 실행 또는 CSS 지원 판정이 아니다. 고정 HTML은 [fixture source](https://github.com/ohah/spinon/blob/plan/c11-4-anonymous-grid-text-pre-wrap/tests/fixtures/css/c11/anonymous-grid-text-pre-wrap-precomparison.html)이고 Ahem font SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.

## 규범과 독립 계산

[CSS Text Level 3, 2026-08-14 Candidate Recommendation Draft, §3](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#white-space-property)는 `pre-wrap`을 공백 보존과 wrapping 허용으로 정의한다. 보존된 공백이 줄 끝에 남으면 hang 처리하며, `break-spaces`의 줄 끝 동작과는 구별한다. Grid item anonymous block 생성은 [CSS Grid Layout Level 1 §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items), max-/min-content track 계산은 §11.5.1을 따른다. §6은 공백으로만 이루어진 direct text sequence를 렌더링하지 않으며 Grid item으로 만들지 않는다고 명시한다. 공백-only text 장면과 block 앞 공백 대조는 max-content track 생성 및 다음 block의 auto-placement cell 소비 여부를 확인한다.

Ahem 10px에서 `X` glyph와 U+0020 각각의 advance는 10 CSS px다. 따라서 `XX  XX`의 normal max-content는 공백 두 개가 하나로 접혀 50px이고, pre-wrap max-content는 두 공백을 보존해 60px이다. 두 단어 사이에는 wrap opportunity가 있으므로 두 모드의 min-content column은 각각 20px이다. pre-wrap에서 line-end preserved space가 hanging되는 25px soft-wrap 장면과 LF forced break 장면은 별도로 둔다.

| 장면 | Columns | Rows | Root | 관측 목적 |
| --- | ---: | ---: | ---: | --- |
| `normal-max-content` | `[50]px` | `[10]px` | `50×10` | 두 source space가 하나로 접힘 |
| `pre-wrap-max-content` | `[60]px` | `[10]px` | `60×10` | 두 source space가 모두 보존됨 |
| `pre-wrap-comment-split-max-content` | `[60]px` | `[10]px` | `60×10` | comment가 두 Text node를 나누지만 anonymous item은 하나이며 두 공백 보존 |
| `pre-wrap-wrap` | `[25]px` | `[20]px` | `25×20` | soft wrap, line-end hanging space, 두 줄 |
| `pre-wrap-forced-break-max-content` | `[20]px` | `[20]px` | `20×20` | LF를 forced break로 보존 |
| `normal-min-content` | `[20]px` | `[20]px` | `20×20` | normal min-content control |
| `pre-wrap-min-content` | `[20]px` | `[20]px` | `20×20` | preserved spaces는 hanging하며 min-content를 키우지 않음 |
| `pre-wrap-whitespace-only-item` | `[0]px` | `[0]px` | `0×0` | 공백-only sequence는 익명 Grid item을 만들지 않음 |
| `pre-wrap-leading-spaces-max-content` | `[40]px` | `[10]px` | `40×10` | 앞의 두 공백 보존, 20px text + 20px spaces |
| `pre-wrap-trailing-spaces-max-content` | `[40]px` | `[10]px` | `40×10` | max-content line에서 뒤의 두 공백 보존 |
| `pre-wrap-tab-max-content` | `[100]px` | `[10]px` | `100×10` | computed `tab-size:8`, tab stop x=80까지 이동 |
| `pre-wrap-whitespace-run-before-block` | `[20,20]px` | `[10]px` | `40×10` | 앞 공백이 cell을 차지하지 않아 marker가 첫 cell `(0,0)`에 배치됨 |

규범 계산은 [expected-values.json](https://github.com/ohah/spinon/blob/plan/c11-4-anonymous-grid-text-pre-wrap/spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/expected-values.json), Agent Browser 관측은 [chrome-observation-agent-browser.json](https://github.com/ohah/spinon/blob/plan/c11-4-anonymous-grid-text-pre-wrap/spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/chrome-observation-agent-browser.json)에 분리했다. 화면은 [HeadlessChrome 156](https://github.com/ohah/spinon/blob/plan/c11-4-anonymous-grid-text-pre-wrap/spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/chrome-156-agent-browser.png) 캡처다.

## 브라우저 관측과 한계

Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0에서 viewport `1280×633` CSS px, DPR 1로 실행했다. Ahem font load status가 `loaded`이고 `document.fonts.check('10px Ahem')`이 true인지 확인했다. 각 scene의 root-relative Range fragments도 관측 JSON에 기록했다. Range는 text fragment 위치이며 glyph raster/pixels를 뜻하지 않는다.

Chrome의 computed `grid-template-columns`, `grid-template-rows`, root frame은 독립 기대값과 모두 일치했다. `pre-wrap-wrap`에서 separator Range rectangle이 root 폭을 넘어가는 값은 fragment 관측으로만 기록하며 glyph ink 또는 GPU overflow sample로 해석하지 않는다. 공백-only sequence 두 개에는 Range fragment가 없고, marker child는 첫 auto-placement cell `(0,0)`에서 관찰됐다. C11.4 implementation은 이 모든 track/root 값과 source revision을 별도 검사해야 한다.

최종 12개 scene의 관측, source hash와 screenshot hash는 Agent Browser JSON에 기록했다.

이 자료는 WPT source, Spinon WPT runner, Android/iOS simulator, physical device, GPU surface 또는 general CSS author admission을 검증하지 않는다.
