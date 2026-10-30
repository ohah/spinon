# C11.4 anonymous Grid text `white-space:nowrap` Chrome 사전 비교

## 범위

Agent Browser의 고정 입력을 HeadlessChrome 156에서 실행해 normal control과 `white-space:nowrap` Grid intrinsic 결과를 기록했다. 이 자료는 구현 전 oracle이며 Spinon 동작이나 CSS 적합성을 검증한 결과가 아니다.

## 재현 입력

- HTML: [`grid-track-sizing-c114-anonymous-text-nowrap-precomparison.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-nowrap-precomparison.html)
- HTML SHA-256: `93b9650f612fb977d68afd0d94e921673331be05b74a8f33431f08eef9ccf29a`
- Ahem asset: [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf)
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Browser: `HeadlessChrome/156.0.0.0` user agent, macOS `MacIntel`, viewport `1280×633`, DPR `1`
- Font readiness: `document.fonts.check('10px Ahem') == true`
- Normative references: [CSS Grid Layout Level 1 §6, §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items) · [CSS Text Level 3 `white-space`](https://www.w3.org/TR/css-text-3/#white-space-property)

## 관측값과 기대값

| 장면 | `grid-template-columns` | `grid-template-rows` | Grid border/content box | Text Range rects |
| --- | --- | --- | --- | --- |
| `anonymous-normal-min-content` | `20px` | `20px` | `20×20px` | 두 개, 각 `20×10px`; 두 번째 줄은 첫 줄보다 10px 아래 |
| `anonymous-nowrap-min-content` | `50px` | `10px` | `20×10px` | 하나, `50×10px`; content box를 30px 넘음 |

두 scene 모두 Ahem `10px/10px`, `XX XX`, `min-content` 단일 column, 20px root width, 0px gap/padding/border를 사용한다. `normal`에서는 공백에서 wrap해 min-content가 20px이다. `nowrap`은 soft wrap을 금지하므로 min-content와 max-content가 모두 50px이다. Root width는 20px로 고정되어 track의 30px overflow가 분리된다.

## 파일

- 전체 관측 JSON: [`chrome156-observation.json`](./chrome156-observation.json)
- Chrome 화면: [`chrome156-anonymous-text-nowrap.png`](./chrome156-anonymous-text-nowrap.png), SHA-256 `1b24f9e58edeaf4128fb3f6d364b92b13df485cf75e976f43085b07c8632f5b0`
- 계획 검토: [실패 관점 20개](../../../../plan/c11-4-anonymous-grid-text-nowrap-plan-review-20.md) · [계획 PR diff 관점 30개](./plan-pr-diff-review-30.md)

## 한계

- Page-level `x/y`는 각 `<section>` 배치 좌표다. 계획된 acceptance는 grid-local width/height, computed track, fragment 수와 relative line offset을 사용한다.
- Range fragment는 glyph ink, anonymous Grid item의 별도 DOM box, 앱 surface pixel 또는 OS compositor output이 아니다.
- 이 비교는 Spinon code, Android/iOS Simulator, WPT, 일반 white-space cascade를 실행하지 않았다.
