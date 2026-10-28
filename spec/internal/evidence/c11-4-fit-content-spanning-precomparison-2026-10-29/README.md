# C11.4 fit-content spanning WPT 사전 비교

## 고정 출처와 Chrome 관찰

- WPT revision: `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- Source: [`css/css-grid/grid-definition/fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html), SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- 대상은 원본 [56–58행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L56-L58), assertion은 [114–116행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L114-L116)이다.
- Agent Browser로 `https://wpt.live/css/css-grid/grid-definition/fit-content-track-sizing.html`을 열어 HeadlessChrome 156에서 upstream harness `OK`, 13 tests / 13 Pass를 확인했다. 대상 `gridTemplateRows`는 `40px 40px`, item border box는 60×80 CSS px, root used height는 300 CSS px, item의 computed `min-height`는 `0px`, `overflow`는 `visible`이었다. Viewport는 1280×633 CSS px, DPR은 1이었다. Chrome 결과는 upstream WPT 관찰이며 Spinon 실행 결과가 아니다.
- [대상 assertion 화면](./chrome156-spanning-pass.png)은 Agent Browser viewport 1280×633 PNG다. [상단 harness 캡처](./chrome156-wpt.png)에는 13/13 summary와 첫 viewport에 보이는 assertion들이 담겨 있다.

## 독립 기대값

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.5 spanning items](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items), [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다.

WPT는 300px 높이 Grid에 `grid-auto-rows:fit-content(40px)`를 지정하고, `grid-row:span 2; min-height:0`인 in-flow item 안에 고정 200px block child를 둔다. 두 implicit row의 gap은 0이다. `fit-content(limit)` track은 auto minimum과 max-content maximum을 가지며, growth limit은 `min(limit, max-content contribution)`에서 정해진다. §11.5의 span-2 item 단계는 그 item의 intrinsic contribution을 두 spanned content-sized track에 분배하고, 각 fit-content growth limit을 넘기지 않는다. 따라서 200px content가 각 track에 요구하는 크기는 40px cap에 의해 잘리고 final rows는 `[40,40]px`다.

`align-self:normal`의 stretch로 item border box는 span한 두 row의 80px 합과 같아진다. 중첩 child는 고정 높이 200px을 유지해 item 아래로 120px 보이게 넘치며, Grid root의 300px 높이는 그대로다. 런타임 계획은 inline width를 60px로 고정해 수평축 변동을 없애되 row sizing 입력은 고정 WPT와 동일하게 유지한다.

## 해석 경계

- 고정 WPT source hash는 checkout에서 검증했다. Browser 관찰은 현재 `wpt.live` 실행 결과이며 이 문서는 live response body를 pinned revision과 동일하다고 별도로 주장하지 않는다.
- 화면은 harness 결과 표시만 보여준다. Pixel equality, Spinon paint, clipping, Scroll 동작, compositor 표시를 증명하지 않는다.
- 기계 판독 기대값은 [`expected.json`](./expected.json)이다.
