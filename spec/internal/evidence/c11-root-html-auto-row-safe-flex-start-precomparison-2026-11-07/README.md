# C11.4 `align-content:safe|unsafe flex-start` 사전 비교

## 범위와 입력

기본 parsed Root HTML의 기존 `RootHtmlAutoRowStretchV1` 고정 입력에서 computed `safe flex-start`와 `unsafe flex-start`를 `start` control과 대조했다. 재현 HTML은 기존 canonical fixture와 같은 40×40px Grid, 40px column, 10px explicit row, 두 direct-text implicit `auto` rows, fixed empty item과 `A`/`B` text items를 사용한다([원본 fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html)).

Agent Browser 0.31.1 / HeadlessChrome 157.0.0.0, 1280×720 CSS px viewport, DPR 1에서 관측했다. `row-gap:0px`는 positive-free-space control이다. Overflow input은 definite 40px block-size basis에서 `calc(120% - 2px)`를 46px로 resolve해 10px row 세 개와 두 gutter의 합이 122px가 된다. [Chrome observation](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/chrome-observed.json), [독립 기대값](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/expected.json), [비교기](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/compare.py), [비교 결과](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/comparison.json), [Chrome 화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/chrome-reference.png)을 보존한다.

## 판정

CSS Box Alignment §4.1은 `flex-start`가 flex formatting context 밖에서 `start`처럼 동작한다고 규정한다. §4.4의 `safe`는 overflow 시 `flex-start`로 fallback하고 `unsafe`는 지정한 정렬을 유지한다. 따라서 이 Grid profile에서는 두 modifier 모두 positive와 overflow 양쪽에서 start-side placement를 내야 하며 computed string은 `safe flex-start`와 `unsafe flex-start`로 각각 보존돼야 한다. 독립 계산과 Chrome 관측은 6/6 scene·72/72 item-frame coordinates에서 일치했고 최대 좌표 오차는 0 CSS px(허용치 1/64 CSS px)였다. Track rows는 전 장면 `[10,10,10]px`다. 규범 참조는 [CSS Box Alignment Level 3 §4.1](https://www.w3.org/TR/css-align-3/#positional-values), [§4.4](https://www.w3.org/TR/css-align-3/#overflow-values)이며 Working Draft이고 overflow-position 값은 at-risk다.

이 자료는 browser oracle과 계획 입력만 고정한다. Spinon 구현, app runtime, WPT 적합성, mobile/physical device, hardware GPU, PixelCopy/Metal readback 또는 C11.4/C11 완료를 주장하지 않는다. 브라우저 캡처 후 Agent Browser를 닫았고 후속 프로세스 확인에서 브라우저 프로세스는 남지 않았다.

## SHA-256

| Artifact | SHA-256 |
|---|---|
| `flex-start-reference.html` | `e4df8de6c64469cc52003d88e85a51b3d7e96d926724857686f8c7fc61eca845` |
| `chrome-observed.json` | `da4afe0543f9f1c14f928a37705c7ef7287c2a76f2a579a1be88629db9fad1d3` |
| `expected.json` | `5e896d20d097dd10b871476839550b77287239d3572dd970e9c5dc0a18230eb5` |
| `comparison.json` | `50a7ecef715821fadf0e39bb2dece72c8c5a8003384735bb0438e00993ff3a9d` |
| `chrome-reference.png` | `dc3a3966b50fe2f76054c7ad11d9bf02c055de85ee969b969093d6ad4c598811` |
