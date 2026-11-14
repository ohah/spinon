# C11.4 두-fit row-span mixed minimum 계획 · 계획 실패 관점 20개

검토 대상은 [집중 계획](./c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum.md)과 [Chrome 157 사전 비교·독립 verifier](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md)다. 코드를 구현했다고 가정하지 않고, 계획에서 누락·과장·잘못된 pass 조건이 생기는 별도 실패 경로를 확인했다.

| # | 실패 관점 | 계획에서 막는 방법 | 판정 |
|---:|---|---|---|
| 1 | 기존 style projection의 넓은 admission을 새 기능 완료처럼 서술 | 이미 `0|auto`를 받아들인 사실과 전용 RuntimeSession 증거가 없는 상태를 분리 | 통과 |
| 2 | Chrome 결과를 Spinon RuntimeSession 결과로 확대 | oracle 92,160 scene과 미실행 product gate를 계획 첫머리·완료 기준에 분리 | 통과 |
| 3 | 92,160 scene 산술 오류 | 480 lists × 2 alignment × 2 order × (32+16 masks)를 명시하고 verifier count와 연결 | 통과 |
| 4 | 480 unique track list를 scene count로 혼동 | 15 positions × 8 ordered cap assignment × 4 tail order를 고정 | 통과 |
| 5 | percentage cap을 definite basis 없이 px로 해석 | `40×220px` definite Grid와 220px percentage basis를 고정 | 통과 |
| 6 | 표본 cap을 전체 cap 지원 범위로 오해 | 직접 유한 non-negative px/% 표본이며 allowlist 전체가 아님을 명시 | 통과 |
| 7 | 네 tail order를 전체 list permutation으로 확대 | 고정 네 order 이외 list grammar를 열지 않도록 범위를 제한 | 통과 |
| 8 | track multiset에서 fit/min/max/auto 개수가 달라짐 | `fit×2 + min×2 + max×1 + auto×1`을 입력·검증에서 고정 | 통과 |
| 9 | row-span topology에서 explicit/implicit row와 item 수 혼합 | 여섯 implicit rows, span2Six 다섯 item, span2Span3Six 네 item을 구분 | 통과 |
| 10 | reverse DOM order에서 mask가 다른 item에 적용 | bit를 semantic item ID에 귀속하고 forward/reverse pair를 전부 비교 | 통과 |
| 11 | explicit-row control까지 auto mask에 포함 | empty 10px explicit control을 항상 `min-height:0`으로 고정 | 통과 |
| 12 | `min-height:auto` 선언을 곧 eligible automatic minimum으로 계산 | 덮는 track type·scrollable overflow·intrinsic trace를 함께 검사 | 통과 |
| 13 | zero-cap min-content floor가 사라지거나 cap 이하라고 가정 | cap·floor·item minimum contribution을 별도로 기록하고 control을 유지 | 통과 |
| 14 | payload overflow를 Grid item frame 또는 전체 Grid overflow와 혼동 | payload, line rect, item frame, track sum, Grid edge를 각각 측정 | 통과 |
| 15 | 320 visible-overflow scene을 전부 버그 또는 0건으로 단정 | zero-cap, `overflow:visible` 조건을 유지하고 320개를 독립 boundary control로 검증 | 통과 |
| 16 | source-order 회귀가 일부 mask에 숨어 있음 | 46,080 pair 모두 semantic item/payload/text-line/row geometry를 비교 | 통과 |
| 17 | `align-content:stretch`가 non-auto row도 변경 | 46,080 alignment pair에서 auto row singleton과 나머지 행을 따로 판정 | 통과 |
| 18 | float rounding 때문에 관측 오차를 제품 의미 차이로 오판 | span-row 합·frame tolerance와 1/64px quantization 및 trace 예외를 분리 | 통과 |
| 19 | reject 뒤 stale trace나 partially published frame이 재사용 | owner/revision/order/stage 오류를 fail-closed로 만들고 same-session recovery를 확인 | 통과 |
| 20 | 검증 편의로 분모·시뮬레이터·공식 상태를 과장 | RuntimeSession full matrix, 플랫폼 smoke의 한계, 작업 소유 자원 종료, `78/214` 및 C11.4 미완료를 완료 조건에 고정 | 통과 |

**결과:** 서로 다른 실패 경로 20개를 확인했다. 계획은 style admission과 실제 동작 증거를 구별하고, 기존 two-fit profile을 넓히지 않으며, overflow 장면을 제거하지 않는다. Chrome oracle만으로는 계획 완료가 아니다. 제품 RuntimeSession 비교, 회귀, simulator 및 별도 implementation/runtime 검토가 남는다.
