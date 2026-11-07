# C11.4 Root HTML `fit-content(percentage)` implicit-row 사전 비교

## 범위

기본 Root HTML Grid의 definite 40×100px content box에서 singleton `grid-auto-rows:fit-content(percentage)`가 implicit track을 제한하는지 비교한다. Chrome 값은 oracle 관측으로 분리하고 기대 행 크기는 CSS Grid fit-content sizing 공식과 고정 content contribution으로 독립 계산했다.

## 환경과 원본

- 브라우저: Agent Browser HeadlessChrome 157.0.0.0, macOS arm64 (`navigator.platform=MacIntel`)
- Viewport: 1280×633 CSS px, DPR 1
- 입력: [고정 precomparison fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-rows-fit-content-percentage-precomparison.html)
- 화면: [전체 페이지 PNG](./chrome157-agent-browser-full.png), 1280×845 pixels
- 기계 판독 browser 관측값: [chrome-observation.json](./chrome-observation.json)
- 공식으로 독립 계산한 기대값: [expected-values.json](./expected-values.json)

## 결과

Chrome computed `grid-auto-rows`는 각 direct percentage를 보존했다. 일곱 장면의 사용 track list는 CSS px 기준 `[10,0,6]`, `[10,20,6]`, `[10,24,6]`, `[10,34,6]`, `[10,34,6]`, `[10,34,6]`, `[10,28,6]`였다. 측정된 정수 CSS px 좌표에서 Grid, item, nested payload, probe geometry가 독립 기대값과 정확히 일치했다.

Percentage basis는 Grid의 definite한 내부 block 크기 100px이다. Nested block 34px이 max-content 높이를 제공한다. Zero-minimum 장면에서 상한은 0px이며, minimum-floor 장면에서는 item의 28px minimum이 20px 상한보다 우선한다. 두 번째 implicit probe 행은 반복된 auto-row sizing function을 따르며 6px으로 유지된다.

## Limits

이 자료는 일곱 고정 장면의 browser 기준이며 Spinon 출력이나 일반 CSS Grid 적합성 결과가 아니다. Definite하지 않거나 cyclic한 Grid height, border/padding/gap, mixed track list, area/auto-repeat 조합, 임의 HTML, WPT, mobile runtime 동작은 확인하지 않았다. 캡처 뒤 Agent Browser session을 닫고 browser process가 비활성임을 확인했다.
