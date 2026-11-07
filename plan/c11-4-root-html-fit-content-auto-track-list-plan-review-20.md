# C11.4 Root HTML fit-content 목록 `auto` track 계획 검토 20개

대상: [계획](./c11-4-root-html-fit-content-auto-track-list.md)과 [고정 precomparison fixture](../tests/fixtures/css/c11/root-html-grid-fit-content-auto-track-list-precomparison.html). 구현 이전에 규범 문법·독립 sizing 계산·현재 profile gate·runtime publication 경계를 서로 다른 실패 관점으로 검토한다.

| # | 실패 관점 | 확인 및 방어 조건 | 판정 |
|---:|---|---|---|
| 1 | 유효한 `auto`를 전체 Root HTML Grid admission으로 과장한다 | exact 기존 fit-content profile 및 mixed list에만 추가한다. | 계획 고정 |
| 2 | 사전 비교 fixture가 의도한 `grid-auto-rows` 대신 initial `auto`를 관측한다 | 수정 capture의 세 computed values가 각각 authored pattern과 일치한다. | PASS |
| 3 | list order가 바뀌거나 첫 implicit row가 두 번째 항목부터 시작한다 | 세 pattern과 네 implicit row의 order를 계산한다. | fixture 고정 |
| 4 | 네 번째 implicit row에서 modulo cycle이 어긋난다 | expected row 4가 첫 pattern member를 재적용한다. | fixture 고정 |
| 5 | 20% cap basis를 grid height나 border-box에서 잘못 얻는다 | root는 border-box와 content-box가 같은 40×100px, zero edges다. | 계획 고정 |
| 6 | `auto`가 leftover free space를 받아 start alignment를 잘못 stretch한다 | `align-content:start`, 100px definite height와 row 합계를 함께 검사한다. | fixture 고정 |
| 7 | `auto` max-content contribution을 payload CSS height와 구분하지 않는다 | auto item은 min-height 0이고 nested block의 definite height가 contribution source다. | 계획 고정 |
| 8 | automatic minimum이 fit-content zero cap을 0px 위로 올린다 | zero-cap scene의 item min-height 0 및 auto minimum 0을 고정한다. | 계획 고정 |
| 9 | fixed `8px` member가 Auto로 취급되거나 stretch eligibility를 바꾼다 | 세 번째 member와 used 8px row를 모든 관련 scene에서 대조한다. | fixture 고정 |
| 10 | zero-sized fit-content row에서 item/payload positions가 충돌한다 | zero scene은 y origin과 following auto/fixed rows를 각각 확인한다. | 계획 고정 |
| 11 | item order가 explicit row를 건너뛰어 row count를 바꾼다 | explicit 10px item을 DOM 첫 자식으로 고정하고 이어 네 auto item을 둔다. | fixture 고정 |
| 12 | percentage track이 indefinite block-axis basis를 사용한다 | Grid block-size 100px를 definite로 고정한다. | 계획 고정 |
| 13 | margin, padding, border, gap이 contribution 또는 basis에 섞인다 | 모든 geometry-affecting edge와 gap을 zero로 고정한다. | fixture 고정 |
| 14 | child overflow가 Grid에서 clip되어 zero cap 경계가 감춰진다 | root와 items는 visible overflow이며 payload frame도 별도 관측한다. | 계획 고정 |
| 15 | 문법상 유효하지만 미지원인 `minmax`, intrinsic track, `fr`, `calc` 또는 fit-content 없는 auto list가 허용된다 | allowlist는 px, %, `fit-content(px|%)`, `auto`로 닫고 fit-content 존재 및 negative tests를 요구한다. | 계획 고정 |
| 16 | non-finite/negative length 또는 empty pattern이 Taffy까지 전달된다 | projection과 layout에서 기존 finite/non-negative 및 list-count guard를 유지한다. | 계획 고정 |
| 17 | 다른 placement profile이 같은 validator 완화의 영향을 받는다 | typed admission은 `RuntimeRootHtmlTextPaintV1` exact candidate에 한정한다. | 계획 고정 |
| 18 | 실패한 revision의 일부 frames 또는 draw snapshot이 남는다 | invalid style/shape atomic rejection과 same-session valid recovery를 검증한다. | 수용 기준 |
| 19 | 계획 fixture와 Chrome output을 독립 CSS Grid 기대값으로 혼동한다 | used rows를 별도 규칙으로 계산하고 Chrome 결과는 관측 JSON으로만 기록한다. | 계획 고정 |
| 20 | 하나의 root profile 통과를 C11.4 완료나 공식 count 증가로 기록한다 | C11.4/C11 unchecked와 `78/214`를 유지하고 simulator smoke를 표면 parity로 부르지 않는다. | 계획 고정 |

**판정:** 20개 계획 실패 경로를 구현 전에 검토했다. Browser 관측을 수집한 뒤 fixture/expected-values를 확정하며, 이 검토는 구현/runtime 20회 및 PR diff 30회와 별도다.
