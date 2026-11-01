# C11.1 parsed Root HTML Grid fixed-px gap 사전 비교

## 고정 입력과 independent expectation

[`tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html)는 definite 130×65px grid, fixed columns `[40,80]px`, rows `[30,30]px, `gap:5px 10px`, row-major placement 네 장면이다. Independent expectation은 `40+10+80=130`, `30+5+30=65`로 track/gap extents를 각각 계산하고 source-order item frame 네 개를 [`expected.json`](./expected.json)에 고정한다.

## Agent Browser 관측

- CLI: `agent-browser` 0.31.1, Headless Chrome 156 (`HeadlessChrome/156.0.0.0`)
- viewport: 390×844 CSS px
- 결과: root `(0,0,130,65)`, columns `40px 80px`, rows `30px 30px`, `column-gap:10px`, `row-gap:5px`; 네 item frame은 expected JSON과 exact match
- capture: [`chrome-156-grid-gap-precomparison.png`](./chrome-156-grid-gap-precomparison.png)
- `agent-browser console --clear` 및 `agent-browser errors --clear`에서 page error가 보고되지 않았다.
- 기계 대조: `node spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/verify-precomparison.mjs`

Chrome 관측은 CSS oracle 비교이며 Spinon 결과나 WPT 통과가 아니다. 기대값은 definite px tracks, gap sizes와 row-major placement에서 독립 계산한다. 기준 정의는 [CSS Grid Layout Module Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters) 및 [CSS Gaps Module Level 1 §2.1](https://www.w3.org/TR/css-gaps-1/#row-column-gutters)이다.

## 구현 결과

구현 전이다. Root HTML 기본 entry 연결, Android/iOS Simulator, runtime failure review와 artifact 경계 결과를 구현 뒤 아래에 추가한다. 이 사전 비교는 C11.1 또는 C11 완료 선언이 아니며 공식 `72/213`을 바꾸지 않는다.
