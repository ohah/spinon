# C11.4 parsed Root HTML auto implicit-row stretch 사전 비교

## 고정 조건

- 규범 기준: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.6, §11.4–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).
- 기준 페이지: [`root-html-grid-auto-row-stretch-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-stretch-precomparison.html), SHA-256 `8a30b3d0a65f8d3edbfdc79af27d565055b22e30454f7ee687958fd5b96a40d8`. 입력에는 visible Grid owner가 하나만 있다. Chrome 대조 장면은 Agent Browser가 같은 DOM에서 `grid-auto-rows`와 `align-content`를 CSSOM으로 바꾸어 순서대로 관측한다. 앱 검증은 변형 전 기본 `auto`/`normal` 상태의 동일 source hash를 쓴다.
- Agent Browser CLI 0.31.1에서 HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1을 관측했다. Chrome 바이너리 revision은 노출되지 않아 버전 관측으로만 기록한다.
- 문서 방향은 LTR, writing mode는 `horizontal-tb`다. Grid content box는 40×40 CSS px이고 explicit columns `[40px]`, explicit rows `[10px]`, gap 0이다. 첫 item은 40×10 CSS px의 빈 direct child, 두 번째 item은 `A A A A A A` text 하나를 가진 direct child다. Chrome의 computed style은 `align-items:normal`, text item `align-self:auto`, `font:10px/10px monospace`, `white-space:normal`, `direction:ltr`, `writing-mode:horizontal-tb`다.

## 독립 기대값

텍스트는 40px inline size에서 10px/10px monospace 두 줄이므로 block-axis min-content contribution은 20px이다. 기본 scene의 implicit `auto` row base size는 20px이다. explicit 10px row와 implicit base 20px를 뺀 definite positive free space는 10px이다. §11.8의 `align-content:normal` 조건에서 auto max track sizing function을 가진 단일 implicit row가 이 10px을 받아 최종 30px이 된다. auto-sized item의 default `align-self:normal`은 30px row를 채운다.

| 장면 | Chrome used rows | direct child frame (Grid 좌표) | text lines |
|---|---|---|---:|
| auto / normal | `10px 30px` | `[0,0,40,10]`, `[0,10,40,30]` | 2 |
| auto / start control | `10px 20px` | `[0,0,40,10]`, `[0,10,40,20]` | 2 |
| min-content control | `10px 20px` | `[0,0,40,10]`, `[0,10,40,20]` | 2 |
| max-content control | `10px 20px` | `[0,0,40,10]`, `[0,10,40,20]` | 2 |
| fixed 10px control | `10px 10px` | `[0,0,40,10]`, `[0,10,40,10]` | 2, text가 10px 넘침 |

`align-content:start` 대조군은 auto row가 20px에 머문다. 이는 기본 scene의 추가 10px이 item text measurement가 아니라 §11.8 auto-track stretch에서 온다는 것을 분리한다. 고정 row 대조군은 text의 두 줄 크기와 track sizing을 분리한다.

## 관측물과 한계

- [독립 기대값 및 Chrome 관측 JSON](./expected.json)
- [Chrome fixture 화면](./chrome/fixture.png), 1280×633 PNG, SHA-256 `0ac56beafef7882ab1bbfc7596b4162b885d5e7255a96787700613c7695bc47f`
- Chrome 관측은 CSS Grid CRD 산술로 도출한 기대값의 교차 확인이다. 이 결과는 Spinon runtime, WPT 적합성, 일반 CSS author admission 또는 앱 pixel equality를 증명하지 않는다.
- Android API 37 AVD와 iOS Simulator 기본 Root HTML 앱 실행 자료는 [runtime evidence](../c11-root-html-grid-auto-row-stretch-runtime-2026-11-07/README.md)에 구분해 기록한다. 앱 로그의 layout-ready와 screenshot은 전체 geometry·surface readback 또는 compositor 뒤 화면을 증명하지 않는다.
