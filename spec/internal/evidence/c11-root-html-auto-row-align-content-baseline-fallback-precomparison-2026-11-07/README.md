# C11.4 Root HTML `align-content:baseline` 사전 비교

## 재현 조건

- Browser: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0.
- Viewport: 1280×633 CSS px, DPR 1.
- Fixture: `tests/fixtures/css/c11/root-html-auto-row-align-content-baseline-fallback-precomparison.html`.
- SHA-256: `2c3a61b019d1b4ecc708724e932e86329b4cb9245860e23a8e4fc5741c43c675`.
- Grid: horizontal-tb/LTR, 40×40px, one explicit 40px column and 10px row, two implicit auto rows, fixed 40×10px item and two 10px/10px monospace text items.
- Cases: `baseline`, `start`, `safe end`, `end`, `unsafe end`; row-gap `0px` and `calc(120% - 2px)`.

## 판정

CSS Box Alignment의 독립 기대값은 shared baseline alignment context가 없는 top-level Grid에서 `baseline`을 first-baseline safe-start fallback으로 둔다. Chrome computed style은 `baseline`을 보존하고, positive-space rows `[10,10,10]` 및 item y `[0,10,20]`을 내보냈다. Overflow에서 46px row-gap 두 개와 rows `[10,10,10]`에도 y `[0,56,112]`로 start fallback을 유지했다. Positive `safe end` control은 y `[10,20,30]`, overflow `end`/`unsafe end`는 `[-82,-26,30]`이었다. 모든 frame의 x/width/height는 `[0,40,10]`이었다.

Chrome observation과 독립 계산은 10 scene·30 item frames에서 일치했고 최대 좌표 차이는 0 CSS px이다. CSSOM computed `first baseline`의 canonical spelling `baseline`도 확인했다. Chrome 157에서 `last baseline`은 이 입력의 computed `align-content`로 남지 않고 `normal`이 됐기 때문에 `last baseline`은 후보에 포함하지 않았다.

## 근거와 한계

- [독립 기대값](./expected-values.json)
- [Chrome 관측](./chrome-observation.json)
- [고정 fixture](../../../../tests/fixtures/css/c11/root-html-auto-row-align-content-baseline-fallback-precomparison.html)
- 규범: [CSS Box Alignment Level 3 §4.2](https://www.w3.org/TR/css-align-3/#baseline-values), [§5.4](https://www.w3.org/TR/css-align-3/#baseline-align-content).

이 비교는 top-level Root HTML fixed profile의 safe-start fallback만 다룬다. Nested Grid/flex baseline-sharing group, `last baseline`, 일반 CSS admission, WPT, Android/iOS 실행, 실기기 또는 GPU 표면 픽셀을 검증하지 않았다.
