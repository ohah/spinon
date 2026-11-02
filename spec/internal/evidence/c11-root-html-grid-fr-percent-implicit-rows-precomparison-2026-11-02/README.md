# Root HTML explicit `fr`·percentage track과 fixed implicit row 사전 비교

## 목적과 기준

기존 Root HTML profile은 이름 없는 direct `px`·`fr`·percentage explicit tracks를 허용하고, 별도 후속 PR #1024는 단일 non-negative px `grid-auto-rows`로 implicit rows를 허용한다. 그러나 이 두 기능을 결합하면 projection과 layout validation이 explicit `fr`·percentage tracks를 거부한다. 이 자료는 CSS Grid의 explicit track sizing과 implicit row placement를 결합하는 네 개의 고정 입력에 대해 독립 산술 기대값을 정하고 Chrome을 교차 관측자로 사용한다.

표준 기준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)의 explicit track sizing, flexible tracks, percentage sizing, implicit track sizing과 auto-placement 알고리즘이다. 정규 기대값은 고정 root 크기와 track 함수에서 직접 계산했다. Chrome 결과가 규범 계산을 대체하지 않는다.

## 재현 자료

- HTML source: [`oracle.html`](oracle.html), SHA-256 `de9446c778deb6b5bd73697a25306491313f9945a94c04e47ddd9621589375e6`
- Chrome 관측: Agent Browser가 연 `HeadlessChrome/156`에서 CSSOM used tracks와 root-relative item frames를 읽었다. 원본 결과는 [`chrome-observation.json`](chrome-observation.json)에 보존한다.
- 화면: [Chrome 156 Agent Browser screenshot](chrome156-agent-browser.png), SHA-256 `63b203ae125ee9acc40aeae4c0f14b08c3edc54826f0ca4967991670f42dd0b5`
- 기계 판독 기대값: [`expected.json`](expected.json)

## 독립 기대값과 관측 결과

| 장면 | root | explicit tracks | implicit row | item frame (root-relative) |
|---|---:|---|---:|---|
| Fraction columns | 120×35 | Columns `[40,80]`; Rows `[20]` | `[15]` | `(0,0,10,10)`, `(40,0,10,10)`, `(0,20,10,10)` |
| Percentage columns | 120×40 | Columns `[30,90]`; gap 5px; Rows `[20]` | `[15]` with 5px row gap | `(0,0,10,10)`, `(35,0,100,10)`, `(0,25,10,10)` |
| Fraction rows | 120×60 | Columns `[120]`; Rows `[15,30]` | `[15]` | `(0,0,10,10)`, `(0,15,10,10)`, `(0,45,10,10)` |
| Percentage rows | 120×80 | Columns `[120]`; Rows `[20,20]` | `[15]` | `(0,0,10,10)`, `(0,20,10,10)`, `(0,40,10,10)` |

Chrome CSSOM reported the same used track sizes in all four scenes: columns `40px 80px`, `30px 90px`, or `120px`; rows `20px 15px`, `15px 30px 15px`, or `20px 20px 15px` as applicable. All item frames matched the independent root-relative calculations exactly. In the percentage-column case, the 100px second item starts at x=35px and extends 15px beyond the 120px root edge; the frame remains unclamped under visible overflow. The percentage-row scene has 25px unused block space; fixed explicit and implicit tracks are not stretched by the default normal alignment.

## 범위와 한계

이 comparison은 고정된 definite root dimensions, no-area Grid, 기본 row flow, auto implicit columns, 한 개의 fixed-px implicit-row size, 세 개의 empty fixed-size direct block items, 0px 또는 5px gap과 zero box edges만 다룬다. Chrome 계산은 implementation/runtime 검증이 아니며 일반 CSS admission, nested Grid, text/image contribution, named areas, `minmax()`, intrinsic sizing, 임의 implicit track lists, 표면 pixel parity를 검증하지 않는다.
