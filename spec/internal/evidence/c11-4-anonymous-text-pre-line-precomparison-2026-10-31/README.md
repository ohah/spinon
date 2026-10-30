# C11.4 anonymous Grid text `white-space:pre-line` 사전 비교

이 기록은 C11.4 anonymous Grid text의 `white-space:pre-line` 제한 fixture slice에 앞선 Chrome 관찰과 별도 규범 계산을 고정한다. 이 관찰은 Spinon 실행 결과가 아니다.

## 입력과 실행 환경

- 입력: [`chrome-precomparison.html`](./chrome-precomparison.html), SHA-256 `e0d4b9f17f9790faeb7b7b34699e4d90c9c6a69b8c1d67663d5720cf0aa69e1a`
- 폰트: [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Browser: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, macOS ARM, viewport 1280×633, DPR 1
- Browser flow: `AGENT_BROWSER_ALLOW_FILE_ACCESS=true agent-browser --session spinon-c11-4-preline-20261031 open file:///Users/yoonhb/Documents/workspace/spinon/spec/internal/evidence/c11-4-anonymous-text-pre-line-precomparison-2026-10-31/chrome-precomparison.html`, then wait for `document.fonts.ready`, read `window.__C114_PRE_LINE_OBSERVATION__`, and save a screenshot
- 폰트 준비: `document.fonts.check('10px Ahem') == true`
- 화면: [`chrome-agent-browser.png`](./chrome-agent-browser.png), 1280×633, SHA-256 `07f68b4ac8adb6cd069e0eb2d85d7f6385f1afdcc2e71b62d9d62f5d4656678a`
- 구조화된 관찰: [`chrome156-observation.json`](./chrome156-observation.json)
- Agent Browser 세션을 닫고 전용 백그라운드 프로세스 종료를 확인했다. terminal-browser는 사용하지 않았다.

## 독립 규범 계산과 관찰값

두 장면은 같은 direct text sequence `XX\nXX`, Ahem 10px/10px, `width:20px`, `grid-template-columns:max-content`, auto row, 0px gap/edges를 사용한다. 각 `X`는 10px이고 각 줄의 `XX`는 20px이다.

CSS Text Level 3 §3은 `pre-line`이 공백 문자를 접고 줄바꿈을 강제 line break로 보존한다고 정한다. `normal`은 해당 segment break를 접어 공백으로 만들고 wrapping을 허용한다. max-content sizing은 soft wrap opportunity를 취하지 않는 크기이며 Grid의 `max-content` track은 그 안 item의 max-content contribution을 반영한다. 따라서 별도 기대값은 `normal`에서 `20 + 10 + 20 = 50px` 한 줄, `pre-line`에서 보존된 강제 줄바꿈 기준 20px track과 20px 높이 두 줄이다. 이는 Chrome 수치를 그대로 기대값으로 역산하지 않은 계산이다.

| 장면 | 계산된 CSS 값 | 규범 계산 | Chrome 156 관찰 |
| --- | --- | --- | --- |
| `normal` | `collapse / wrap` | max-content track 50px, row 10px; 20px root 밖으로 text item 30px overflow | columns `50px`, rows `10px`, root `20×10px`, Range `50×10px` 한 조각 |
| `pre-line` | `preserve-breaks / wrap` | max-content track 20px, row 20px; 20px root 안에 두 줄 | columns `20px`, rows `20px`, root `20×20px`, 두 glyph-line fragment 각 `20×10px` 및 LF의 0×10px Range rect |

Range는 inline text fragment 관찰이며 anonymous Grid item box나 glyph ink 표본이 아니다. 개별 newline Range가 0px 폭 rect를 만드는 동작은 raw observation에 보존했으며 줄 수를 세는 glyph fragment에 포함하지 않는다. Browser x/y는 section의 문서 좌표이므로 acceptance는 local track/frame 크기와 fragment count에 둔다.

## 출처

- [CSS Text Level 3 §3 white-space](https://www.w3.org/TR/css-text-3/#white-space-property) 및 [§4.1.3 segment-break transformation](https://www.w3.org/TR/css-text-3/#line-break-transform)
- [CSS Grid Layout Level 1 §6 Grid items](https://www.w3.org/TR/css-grid-1/#grid-items), [§7.2.1 Track sizes](https://www.w3.org/TR/css-grid-1/#track-sizes), [§11.5.1 Resolve intrinsic track sizes](https://www.w3.org/TR/css-grid-1/#algo-content)
- [CSS Box Sizing Level 3 §2.2 intrinsic size contributions](https://www.w3.org/TR/css-sizing-3/)
