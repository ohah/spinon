# C11.4 parsed Root HTML 두 implicit auto-row stretch 사전 비교

## 고정 조건

- 규범 기준: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.6, 7.6.1, 8.5, 11.3, 11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).
- Fixture: [root-html-multiple-auto-row-stretch-precomparison.html](../../../../tests/fixtures/css/c11/root-html-multiple-auto-row-stretch-precomparison.html), SHA-256 `3dafe4a9f6c0b1ba05bd0391e0d32bd40cd4d1dbdd3dafa60b1f8e67a1063e55`. Visible Grid owner는 하나다. Content box는 40×40px이고 explicit column은 40px 하나, explicit row는 10px 하나, gap은 0이다. Direct child는 source order대로 fixed empty item, 한 줄 `A` item, 한 줄 `B` item이다. Text item 둘 다 10px/10px monospace다.
- Agent Browser 0.31.1에서 HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1을 관측했다. Controls는 같은 DOM에서 CSSOM 값을 바꾸어 실행했으며 source fixture 기본 장면은 auto/normal 두-row 상태다.

## 독립 기대값

Explicit fixed row base는 10px이다. 두 implicit `auto` rows는 각 text의 10px minimum contribution을 받는다. 따라서 초기 bases `[10,10,10]` 이후 definite 40px block axis에 positive free space 10px이 남는다. CSS Grid §11.8에 따라 두 auto max track에 각 5px을 균등 분배하므로 기대 used rows는 `[10,15,15]px`이고 두 text frame은 `[0,10,40,15]`, `[0,25,40,15]`다.

`align-content:start`, `grid-auto-rows:min-content`, fixed `10px` controls는 각각 `[10,10,10]px`를 유지한다. 한 implicit auto row control의 두 줄 text는 기존 `[10,30]px` 결과를 유지한다. HeadlessChrome 157은 네 control 결과를 정확히 관측했으며 세부 값은 [expected.json](./expected.json)에 있다.

## 관측과 한계

- [Chrome fixture screenshot](./chrome/fixture.png), 1280×633 PNG, SHA-256 `e717e0bfc1e5de8872a88a2d755b5664ad36e99219a675d8929c9058331714bf`. 기본 fixture 화면만 담는다.
- [Agent Browser lifecycle](./agent-browser-lifecycle.json)에 사용한 session과 종료 상태가 있다. 다른 session은 닫지 않았다.
- 이 oracle은 구현 전 비교 근거다. Spinon runtime, WPT, 일반 CSS admission, native app 동작 또는 pixel equality를 입증하지 않는다.
- 제한 구현 계획은 [C11.4 두 auto-row stretch](../../../../plan/c11-root-html-multiple-auto-row-stretch.md)다.
