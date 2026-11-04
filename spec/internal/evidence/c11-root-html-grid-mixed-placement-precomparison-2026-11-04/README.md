# C11 parsed Root HTML 혼합 고정·한 축 자동 배치 사전 비교

## 기준 입력과 환경

- Fixture: [`root-html-grid-mixed-explicit-one-axis-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-mixed-explicit-one-axis-precomparison.html)
- 독립 기대값: [`expected.json`](./expected.json)
- Chrome 관측: [`chrome156-observation.json`](./chrome156-observation.json)
- Agent Browser 화면: [`chrome156.png`](./chrome156.png)
- 검증 스크립트: [`verify_precomparison.py`](./verify_precomparison.py)
- 규범: [CSS Grid Layout Level 1 §8.5](https://www.w3.org/TR/css-grid-1/#auto-placement-algo)

Agent Browser 0.31.1의 HeadlessChrome 156, macOS arm64에서 viewport 1280×633 CSS px, DPR 1로 관찰했다. 기대값은 40/50/30px columns와 20/30/40px rows, CSS Grid의 fully definite 선점 단계 및 source-order sparse placement로 독립 계산했다. Chrome 관측은 그 기대값을 교차 확인한다.

Fixture SHA-256: `cf1f516150cd7f72efdc7f7a612e61840210dd21205dcc57cf269e15440f6c88`

Chrome observation SHA-256: `32acf59e8479007611ef9a0540df13e95208e6abf3a59f9c7841cd3233ffc142`

Screenshot SHA-256: `781f7ad9ecb942ba7f66ef1abd7ee64d3e7028b438185add64a69199622f5117`

2026-11-04 Agent Browser 재실행도 9 scenes·19 visible frames에서 최대 0 CSS px 차이로 통과했다. 재수집한 [`JSON`](./chrome156-agent-browser-rerun-2026-11-04.json)과 [`화면`](./chrome156-rerun.png)은 기존 observation 및 screenshot과 각각 동일한 SHA-256이다. 검증 뒤 해당 Agent Browser 세션과 프로세스를 종료했다.

## 구현 전 범위 정정

초기 사전 fixture에는 partial item의 자동 축 endpoint에 `span`을 둔 장면이 있었다. 기존 one-axis route 계약은 반대 축의 양 endpoint가 모두 `auto`여야 하므로 그 장면은 계획 범위 밖이었다. 제품 코드를 구현하기 전에 span을 definite 축으로 옮기고 사전 기대값·관측·화면을 다시 수집했다. 위 hash와 결과는 수정된 fixture만 가리킨다.

## 관찰 결과와 후보 범위

9 scenes·20 items·19 visible frames에서 독립 기대값과 Chrome DOMRect가 최대 0 CSS px 차이로 일치한다. Fully definite item은 DOM 순서상 뒤에 있어도 먼저 occupancy를 예약하고, 같은 definite axis를 가진 partial item은 그 cell/span을 피해 sparse source order로 배치된다. Partial item's definite axis는 line/span을 사용하고 반대 축 양 endpoint는 모두 `auto`다. `visibility:hidden`은 occupancy에 참여하고 `display:none`은 제외한다. Oracle에서 동작하는 mixed row/column scene과 implicit-track scene은 제품 후보에서 기대 거부한다. all-auto scene은 기존 route의 회귀 control이다.

Chrome reference 결과는 제품 runtime·WPT 적합성·Android/iOS 결과가 아니다. 제품 후보는 no-area, row-flow, finite fixed-px explicit tracks, zero gap/edge인 기존 Root HTML 제한 profile에만 추가한다. Named placement, mixed partial axes, all-auto, implicit track 생성, nested/text/replaced item은 기존 경계를 유지한다.
