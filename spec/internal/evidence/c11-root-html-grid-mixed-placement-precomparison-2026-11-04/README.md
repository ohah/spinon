# C11 parsed Root HTML 혼합 고정·한 축 자동 배치 사전 비교

## 기준 입력과 환경

- Fixture: [`root-html-grid-mixed-explicit-one-axis-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-mixed-explicit-one-axis-precomparison.html)
- 독립 기대값: [`expected.json`](./expected.json)
- Chrome 관측: [`chrome156-observation.json`](./chrome156-observation.json)
- Agent Browser 화면: [`chrome156.png`](./chrome156.png)
- 검증 스크립트: [`verify_precomparison.py`](./verify_precomparison.py)
- 규범: [CSS Grid Layout Level 1 §8.5](https://www.w3.org/TR/css-grid-1/#auto-placement-algo)

Agent Browser 0.31.1의 HeadlessChrome 156, macOS arm64에서 viewport 1280×633 CSS px, DPR 1로 관찰했다. 기대값은 40/50/30px columns와 20/30/40px rows, CSS Grid의 fully definite 선점 단계 및 source-order sparse placement로 독립 계산했다. Chrome 관측은 그 기대값을 교차 확인한다.

Fixture SHA-256: `e1edb6a892d501a2e327be80d34e40f773256ddbe56336cf857df1972ed7cb79`

Chrome observation SHA-256: `27c87f6640deeda8618dcaab84a7206e6044ccfd82c27b96c768c14dd0dd8199`

Screenshot SHA-256: `fb4abe03f979c881d5c5fb3d1344c6ce46532b5ef38388058befa97839f53201`

## 관찰 결과와 후보 범위

9 scenes·20 items·19 visible frames에서 독립 기대값과 Chrome DOMRect가 최대 0 CSS px 차이로 일치한다. Fully definite item은 DOM 순서상 뒤에 있어도 먼저 occupancy를 예약하고, 같은 definite axis를 가진 partial item은 그 cell/span을 피해 sparse source order로 배치된다. `visibility:hidden`은 occupancy에 참여하고 `display:none`은 제외한다. Oracle에서 동작하는 mixed row/column scene과 implicit-track scene은 제품 후보에서 기대 거부한다. all-auto scene은 기존 route의 회귀 control이다.

Chrome reference 결과는 제품 runtime·WPT 적합성·Android/iOS 결과가 아니다. 제품 후보는 no-area, row-flow, finite fixed-px explicit tracks, zero gap/edge인 기존 Root HTML 제한 profile에만 추가한다. Named placement, mixed partial axes, all-auto, implicit track 생성, nested/text/replaced item은 기존 경계를 유지한다.
