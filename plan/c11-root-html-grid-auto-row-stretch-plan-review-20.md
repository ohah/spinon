# C11.4 Root HTML `grid-auto-rows:auto` stretch 계획 실패 관점 검토 20개

대상: [`c11-root-html-grid-auto-row-stretch.md`](./c11-root-html-grid-auto-row-stretch.md)와 [고정 사전 비교](../spec/internal/evidence/c11-root-html-grid-auto-row-stretch-precomparison-2026-11-07/README.md). 계획을 구현하기 전에 서로 다른 실패 조건을 점검했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | definite inner block-size 40px 대신 border-box·padding 크기를 free-space 계산에 쓰는가? | PASS — padding/border 0, content box 40px을 입력으로 고정했다. |
| 2 | explicit 10px row를 남은 공간에서 빠뜨리는가? | PASS — 40−10−20=10px 산술과 control rows를 고정했다. |
| 3 | text min-content contribution을 grid-auto row 자체 크기와 혼동하는가? | PASS — 초기 base 20px과 §11.8 뒤의 30px을 별도 값으로 검증한다. |
| 4 | CSS Grid §11.8의 분배 조건 없이 모든 intrinsic row를 늘리는가? | PASS — computed `align-content:normal`과 `auto` max function을 모두 검사한다. |
| 5 | `align-content:normal`을 `stretch`와 동일한 선언 문자열이라고 가정하는가? | PASS — computed value는 `normal` 그대로 보존하고 규범의 Grid 동작만 적용한다. |
| 6 | `align-content:start` control을 제품 지원 범위에 몰래 포함하는가? | PASS — oracle control이고 구현 profile은 normal만 허용한다. |
| 7 | text item의 `align-self:normal`을 행의 content distribution과 같은 단계로 취급하는가? | PASS — track list와 item frame을 함께 확인해 track 30px/item 30px을 각각 검증한다. |
| 8 | 고정 10px 첫 item이 explicit row에 놓이지 않고 implicit row로 이동하는가? | PASS — item source order와 첫 explicit row geometry를 고정한다. |
| 9 | text를 가진 두 번째 item이 row 2가 아니라 explicit row를 겹쳐 쓰는가? | PASS — row-flow auto placement와 scene frame을 검증한다. |
| 10 | text line height·font가 달라져 기여가 20px이 아니게 되는가? | PASS — monospace 10px/10px 및 두 줄 line rectangles를 고정한다. |
| 11 | `white-space` 접기나 token grammar를 새로 확장하는가? | PASS — 기존 ASCII token/whitespace gate를 그대로 재사용한다. |
| 12 | generic Root HTML 외의 runtime profile이 이 성공 경로에 들어오는가? | PASS — 기본 parsed Root HTML route와 정확한 Grid owner를 요구한다. |
| 13 | 두 개 이상의 Grid owner 또는 다중 visible item이 sizing을 바꾸는가? | PASS — 단일 owner, 고정 대조 item 1개, text item 1개만 받는다. |
| 14 | hidden, detached, stale-revision item이 contribution에 섞이는가? | PASS — 기존 ownership·visibility·revision 검증을 유지하고 negative control에 둔다. |
| 15 | 추가 row/column, dense flow, named area, placement, repeat을 잘못 받아들이는가? | PASS — 해당 topology와 grammar는 fail-closed 경계로 적었다. |
| 16 | track/quota 상한 또는 arithmetic overflow에서 일부 geometry가 게시되는가? | PASS — 기존 10,000-track limit, checked arithmetic, atomic output을 acceptance에 포함했다. |
| 17 | layout trace가 잘려도 track geometry가 성공한 것처럼 보이는가? | PASS — trace presence와 final geometry를 별도 검사하며 생략/오류 계약을 분리한다. |
| 18 | browser screenshot을 Spinon 결과나 pixel parity 근거로 과장하는가? | PASS — precomparison과 simulator/runtime evidence의 증명 범위를 분리했다. |
| 19 | Android AVD/iOS Simulator 결과를 실기기·GPU·compositor 결과로 확대하는가? | PASS — platform/software/compositor 제한을 계획에 명시했다. |
| 20 | 이 제한 slice를 C11.4/C11 완료 또는 공식 집계 증가로 처리하는가? | PASS — checkboxes와 완료 집계를 변경하지 않도록 명시했다. |

결론: 기준 입력에서 auto-track stretch를 item sizing과 분리해 판정할 수 있고, 지원 밖 조합과 공식 완료 상태의 경계가 계획에 고정되어 있다. 계획 PR 실제 diff는 이 기록과 별개로 독립 실패 관점 30개로 검토한다.
