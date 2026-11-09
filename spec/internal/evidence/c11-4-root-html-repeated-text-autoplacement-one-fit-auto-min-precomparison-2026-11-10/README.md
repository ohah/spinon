# C11.4 반복 텍스트 one-fit 자동 배치 `min-height:auto` 사전 비교

**상태:** Chrome 157 사전 비교 및 독립 관계 검증 완료 · 제한 계획 단계 · 2026-11-10

## 고정 입력과 환경

- Oracle fixture: [`oracle-fixture.html`](./oracle-fixture.html)
- Fixture SHA-256: `c504105b3926c3f668678d8a5386c75b9fb7eea9b723fe66696179669d5c4854`
- Browser: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, MacIntel, DPR 1, viewport 1280×900
- Grid: definite `40×220px`, `40px` explicit column 하나, `10px` explicit row 하나, gap 0, row auto-flow, LTR, visible overflow
- Children: 첫 explicit row의 빈 control 한 개는 `min-height:0`; 뒤의 auto-placement text item 여덟 개는 모두 computed `min-height:auto`다. Payload 줄 수는 `[2,2,2,3,2,2,3,1]`이다.
- `grid-auto-rows`: `fit-content(cap)` 한 개, `min-content` 두 개, `max-content` 두 개, `auto` 한 개. Fit-content 위치 1–6, 나머지 다섯 member의 고유 순서 30개, cap `20px`, `20%`, `0%`, `120%`, 정렬 `start|stretch`를 비교했다.
- 기존 [one-fit `min-height:0` 기준](../c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/README.md)과 같은 authored input 순서로 장면을 짝지었다. 후보와 기준은 각 1,440 scene이다.

## 관측과 독립 판정

고정 matrix는 `6 positions × 30 tail orders × 4 caps × 2 alignments = 1,440` scenes이며 고유 authored list는 720개다. 전체 [Chrome 관측](./chrome-matrix-observation.ndjson)은 metadata 한 줄과 장면별 한 줄을 보존한다. [수집기](./capture-chrome.mjs)는 Agent Browser의 브라우저 DOM에서 computed style, Grid row, item/payload frame과 text line rectangles를 읽는다. [독립 검증기](./verify-chrome-matrix.py)는 고정 fixture·collector hash, 기존 `min-height:0` 기준, multiplicity와 CSS Grid sizing 관계를 별도로 검사한다.

W3C [CSS Grid §6.6 automatic minimum size](https://www.w3.org/TR/css-grid-1/#min-size-auto)는 scrollable overflow가 아닌 Grid item이 해당 축의 auto minimum track을 span하면 content-based automatic minimum을 사용한다고 규정한다. Fit-content track의 automatic minimum은 authored cap에 의해 내용 최소값 아래로 clamp되지 않는다. 이번 표본은 `overflow:visible`, definite block basis, one-fit list, 고정 text/profile 안에서 이 관계를 확인한다.

검증 결과:

- 1,440/1,440 scene, fit 위치별 240 scene, cap별 360 scene, 30 tail order 및 start/stretch 각 720 scene을 확인했다.
- 모든 장면에서 여섯-member authored/computed 순서, control `min-height:0`, 여덟 text item의 computed `min-height:auto`, 10px explicit row, 8개 implicit row의 cycle, item/payload geometry와 줄 수를 확인했다.
- 1,920 fit-content cycle occurrence 중 기존 `min-height:0` 결과가 content floor보다 작았던 600 paired occurrence(정렬 pair별 중복 포함)는 자동 최소 크기에서 floor까지 회복했다. 모든 fit row는 cap `0%`와 `20px`를 포함해 text content floor를 유지했다. Content floor 위반은 0개다.
- `start`의 row 합은 720 list 모두 180px다. `stretch`는 40px 남은 공간을 auto row occurrence에만 나누어 220px를 채웠고 6,480개 row delta 관계에서 non-auto row 변화는 0px였다.
- auto-minimum 장면의 visible payload overflow는 0/1,440 scene이다. 이 입력은 definite 220px grid 안에 모든 text를 수용하며, overflow 동작을 입증하지 않는다.
- 독립 검증 결과는 [matrix-verification.json](./matrix-verification.json)이다. [표본 화면](./chrome157-one-fit-auto-min-sample.png)은 `fit-content(0%)`의 visual reference이며 geometry 판정은 JSON 행렬과 verifier가 담당한다. [Agent Browser 수명 기록](./agent-browser-lifecycle.json)은 세션이 정상 종료됐고 Terminal Browser·Simulator를 사용하지 않았음을 기록한다.

이 자료는 Chrome 기준 관측이며 Spinon RuntimeSession, 네이티브 렌더링, public CSS admission, WPT 적합성 또는 C11.4/C11 완료를 입증하지 않는다. 구현은 계획 검토·계획 PR 병합 뒤에만 시작한다.
