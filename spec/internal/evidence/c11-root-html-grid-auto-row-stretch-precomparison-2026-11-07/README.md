# C11.4 parsed Root HTML auto implicit-row stretch 사전 비교

## 고정 조건

- 규범 기준: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.6, §11.4–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).
- 기준 페이지: [`root-html-grid-auto-row-stretch-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-stretch-precomparison.html), SHA-256 `b566cf4ebaad8ffea05458939bf1c9b05ac5f4d3cfa840852ee1c03993063a87`.
- Agent Browser CLI 0.31.1에서 HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1을 관측했다. Chrome 바이너리 revision은 노출되지 않아 버전 관측으로만 기록한다.
- 문서 방향은 LTR, writing mode는 `horizontal-tb`다. 각 Grid content box는 40×40 CSS px이고 explicit columns `[40px]`, explicit rows `[10px]`, gap 0이다. 첫 item은 10px 높이의 빈 direct child, 두 번째 item은 `A A A A A A` text 하나를 가진 direct child다.

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
- [Chrome fixture 화면](./chrome/fixture.png), SHA-256 `0ab2f319c39e797a159c8df338c47173d8ecbf73a9dc2814109f9f1c5111919f`
- Chrome 관측은 CSS Grid CRD 산술로 도출한 기대값의 교차 확인이다. 이 결과는 Spinon runtime, WPT 적합성, 일반 CSS author admission 또는 앱 pixel equality를 증명하지 않는다.
- 향후 Android API 37 AVD와 iOS Simulator 앱 로그는 geometry/draw/present 검증으로 구분한다. 표면 readback 또는 compositor 뒤의 화면 검증은 실제 수행한 경우에만 별도 기록한다.
