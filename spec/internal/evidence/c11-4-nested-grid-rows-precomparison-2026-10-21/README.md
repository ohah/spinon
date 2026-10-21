# Nested Grid row-axis intrinsic contribution 사전 비교

## 목적

한 단계 nested Grid가 outer Grid row track에 block-axis intrinsic contribution을 전달하는 세 고정 scene을 만든다. 첫 scene은 nested Grid가 outer row 두 개를 span하고, 두 번째는 한 row만 차지한다. definite-height flex control은 §6.6 automatic minimum 분기를 분리한다. Chrome 관측값과 CSS Grid 독립 계산은 같지만 별도로 기록한다. 이 결과는 Spinon 실행이나 전체 CSS Grid 지원을 뜻하지 않는다.

## 환경과 입력

- 기준 구현: Agent Browser가 연 Google Chrome `156.0.8078.48`, `HeadlessChrome/156.0.0.0`, macOS `26.5.1`, ARM64, DPR 1. 상세 User-Agent Client Hints는 `chrome156-observation.json`에 보존했다.
- fixture: [`grid-track-sizing-c114-nested-grid-rows-precomparison.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-rows-precomparison.html).
- font: 저장소 Ahem TTF. 세 scene 모두 `document.fonts.ready` 뒤 `document.fonts.check('10px Ahem')`가 true였다.
- image: 저장소 `24×16px` PNG. 각 image의 `decode()`가 resolve됐고 `complete=true`, `naturalWidth=24`, `naturalHeight=16`이었다.
- 관측 viewport: `innerWidth=1280`, `innerHeight=633` CSS px. scene geometry는 viewport 폭과 무관한 fixed 98px outer Grid다. 첨부 PNG는 왼쪽 위 `320×190` 화면 영역이다.
- 원본 fixture SHA-256: `09b80963d5e8194f3421a024c20f2db3fbdc75c13acb032622e61344f02b0d9d`.
- Ahem TTF SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- PNG SHA-256: `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`.
- Chrome observation SHA-256: `76f9a09507eb1975626f54f244f1809a924ca627d29c130443ec3218ed064144`.
- Chrome screenshot SHA-256: `58dc1f947f83db934718292d123e89521da3241f0261ab4c601faf44c7dac0af`.

## 독립 계산

규범 근거는 [CSS Grid Layout Module Level 1, 2025 CRD §6.6, §11.1, §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. §11.1 columns pass에서 nested inline size와 text wrapping이 고정된 뒤 rows pass의 contribution을 결정한다. Ahem `XX XXXXXXX`의 longest word는 7×10px=`70px`; 70px column에서 두 줄이 되어 `70×20px`다. PNG image는 `24×16px`다. Inner columns `min-content max-content`와 4px gap은 `70+4+24=98px`; inner min-content rows는 `[20,16]px`이고 gap 포함 block contribution은 40px이다.

- `row-span-two`: §11.5.1의 spanning contribution에서 outer gap 4px을 제외하면 두 min-content row가 분배할 공간은 36px이다. 각 track은 18px이며 root height는 `18+18+4=40px`이다.
- `row-one-track`: 40px nested contribution이 첫 row 하나에 적용되고 둘째 row는 0px이다. root height는 `40+0+4=44px`이다.
- `row-definite-flex-control`: definite root block-size 60px에서 gap 4px을 뺀 56px을 두 flexible rows가 28px씩 받는다. nested item은 두 flexible tracks를 span하므로 §6.6 automatic minimum은 0px이다. nested Grid는 60px로 stretch되고 내부 min-content rows는 `[20,16]px`로 유지된다.

## 결과

| scene | root size / outer tracks | nested frame / inner tracks | text frame | image frame |
| --- | --- | --- | --- | --- |
| `row-span-two` | `98×40`; rows `[18,18]` | `(0,0) 98×40`; columns `[70,24]`, rows `[20,16]` | `(0,0) 70×20` | `(74,24) 24×16` |
| `row-one-track` | `98×44`; rows `[40,0]` | `(0,0) 98×40`; columns `[70,24]`, rows `[20,16]` | `(0,0) 70×20` | `(74,24) 24×16` |
| `row-definite-flex-control` | `98×60`; rows `[28,28]` | `(0,0) 98×60`; columns `[70,24]`, rows `[20,16]` | `(0,0) 70×20` | `(74,24) 24×16` |

Chrome 관측은 위 독립 계산과 일치했다. 관측은 단일 browser build 결과이며 Spinon runtime, WPT, browser 간 적합성 또는 제품 지원 판정으로 확대하지 않는다.

## 산출물

- [Agent Browser 평가 원문](./agent-browser-eval.raw.txt)
- [Chrome 156 JSON](./chrome156-observation.json)
- [Chrome 156 화면](./chrome156-nested-grid-rows.png)
- [행 축 계획](../../../../plan/c11-4-nested-grid-row-contribution.md)
- [계획 실패 관점 20개](../../../../plan/c11-4-nested-grid-rows-plan-review-20.md)
- [계획 PR diff 실패 관점 20개](./plan-pr-diff-review-20.md)
