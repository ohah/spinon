# C11.4 Root HTML `fit-content(px)` implicit-row 사전 비교 · 2026-11-07

## 기준

- Agent Browser 0.31.1의 HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633 CSS px, DPR 1.
- Input: [`root-html-grid-auto-rows-fit-content-cap-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-cap-precomparison.html), SHA-256 `ad7cdb85e930918cb42d699d6f03885dabbc8fe417e16380f7d7afcc7842d68c`.
- Browser 결과는 CSS computed value, used `grid-template-rows`, direct child/payload DOMRect 관측이다. Spinon 구현 결과가 아니다.

## 독립 판정

CSS Grid fit-content track formula는 `max(minimum, min(limit, max-content))`다. `min-height:0` item의 34px nested content와 24px cap은 24px implicit row를 만든다. 40px cap은 34px content를 보존한다. 28px minimum은 24px cap보다 높아 28px row를 만든다. 세 scene의 Grid content box는 40×100px이며 expected rows와 frames는 [`expected-values.json`](./expected-values.json)에 있다. Chrome 관측은 [`chrome-observation.json`](./chrome-observation.json)에 있다.

## 경계

이 비교는 고정 CSS fixture 세 장면의 Chrome oracle다. Spinon cascade, typed projection, layout, app runtime, Android/iOS, 일반 CSS admission 또는 WPT 적합성 증거가 아니다.
