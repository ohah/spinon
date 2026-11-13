# C11.4 three-fit row-span mixed minimum 계획 검토 20개

대상: [three-fit row-span item별 mixed `min-height:0|auto` 계획](./c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.md)와 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md).

제품 코드를 바꾸기 전에 서로 다른 실패 관점으로 계획의 입력 경계, oracle 분모, 독립 판정, 플랫폼 검증과 완료 주장을 대조했다. 구현 결과를 검증한 기록이 아니다.

| # | 적대적 관점 | 확인 결과와 판정 |
|---:|---|---|
| 1 | 제한 fixture를 일반 CSS Grid 지원으로 넓혀 읽을 수 있는가 | 계획은 정확한 six-member three-fit track multiset과 두 row-span topology만 허용하고 public CSS/API·C11.4 완료를 제외한다. 상위 지원을 주장하지 않는다. |
| 2 | track list 길이 또는 tail multiplicity의 잘못된 입력이 positive admission에 섞이는가 | 6개 implicit row와 `min-content`·`max-content`·`auto` 각 1개를 요구한다. 다른 multiplicity와 list length는 negative case다. |
| 3 | 일부 fit-position set만 반복해 전체 지원처럼 보일 수 있는가 | 6개 row 중 3개를 고르는 20개 position set을 모두 관측하고 verifier가 signature 720개를 확인한다. |
| 4 | 각 fit cap이 자기 authored 위치와 분리될 수 있는가 | `fitPositions`와 cap index를 함께 검사하며 direct `0px`, `10px`, `20%`의 여섯 순열을 전부 확인한다. |
| 5 | intrinsic tail 순서가 정규화되어 CSS 입력 순서 차이를 감출 수 있는가 | 세 tail value의 여섯 순서를 독립 생성·검증한다. |
| 6 | percentage basis가 border box 222px로 잘못 계산되는가 | fixture는 42×222 border box와 `box-sizing:border-box`를 사용해 content box와 percentage basis를 40×220px로 유지한다. computed rows와 uniform endpoint를 oracle에 대조한다. |
| 7 | mask bit를 DOM 순서로 해석해 reverse topology에서 다른 item에 적용할 수 있는가 | bit 순서는 semantic ID 정렬로 고정하고, forward/reverse topology마다 같은 semantic item mask를 검증한다. |
| 8 | topology별 mask가 일부 누락되거나 초과될 수 있는가 | span2-five-item은 32개, span2/span3-four-item은 16개 mask를 모두 확인하고 네 방향별 scene 분모도 독립 검증한다. |
| 9 | 명시 row의 empty control이 mask에 포함돼 `min-height:auto`가 되는가 | empty control은 모든 scene에서 mask 밖이며 computed `min-height:0px`와 10px frame을 검사한다. |
| 10 | `min-height:auto`를 모든 item에서 동일하게 처리해 eligible span 조건을 무시하는가 | §6.6 조건에 따라 item별 overflow·covered track·flexibility를 판정하고 eligible item만 content-floor assertion을 받는다. |
| 11 | `overflow:auto|scroll|hidden`에서 automatic minimum 조건을 잘못 적용하는가 | Positive fixture는 computed visible overflow로 고정한다. 다른 overflow mode는 이번 positive profile 밖이고 거부 회귀 대상이다. |
| 12 | span 안의 flexible track이 있어도 minimum mask를 허용하는가 | Positive multiset에는 flexible track이 없으며 flexible/extra track은 negative admission이다. |
| 13 | explicit row를 implicit row로 세어 7개 used row를 놓치는가 | 관측은 explicit row 1개와 implicit row 6개를 합한 7개를 요구하며 각 span은 실제 covered row sum으로 계산한다. |
| 14 | Grid border와 span frame 좌표를 혼동해 1px 오차를 숨기는가 | border box와 content box를 별도로 기록하고 item geometry는 grid border origin 기준으로 측정한다. Uniform endpoint 비교는 border 안쪽 좌표를 1px 정규화한다. |
| 15 | source-order reversal이 geometry 대신 DOM index를 비교하는가 | pair key는 track list·alignment·mask·topology family를 쓰고 semantic ID별 row/item frame을 비교한다. |
| 16 | `align-content:stretch`가 `fit-content()` track까지 늘어도 통과하는가 | 매 alignment pair에서 non-auto row 변화는 0.01px 이하여야 하고 남는 공간은 auto maximum row에만 배분된다. |
| 17 | line wrapping 변경이 automatic-minimum floor 비교를 무효화하는가 | text payload, line-count, line-height와 frame을 고정하고 각 record에서 payload frame 및 line rectangles를 보존한다. |
| 18 | visible overflow를 항상 0으로 기대해 유효한 외부 paint를 거부하는가 | Chrome에서 관측한 1,344 전체 및 1,236 mixed-mask overflow scene을 별도 분모로 유지하고 20px 최대 초과를 기록한다. 화면 sentinel은 D의 10px 초과를 보인다. |
| 19 | 새 mask 구현이 기존 all-zero/all-auto endpoint를 바꿔도 숨겨지는가 | 기존 three-fit 11,520-scene Chrome observation과 모든 endpoint row/item/payload geometry를 대조한다. 관측 최대 차이는 0px다. RuntimeSession에서도 같은 endpoint 회귀를 다시 실행한다. |
| 20 | 작은 RuntimeSession sample을 전체 138,240 실행으로 과장하거나 failure 뒤 stale output을 허용하는가 | 36 list × 192 scenes = 6,912 sample로 분모를 분리하고 fail-closed/atomic recovery 및 same-session valid revision 회복을 별도 gate로 요구한다. |

결론: 계획은 구현 전 비교, bounded admission, failure recovery, visual overflow와 completion-reporting 경계를 명시한다. 위 검토는 계획의 설계 검토이며 구현/runtime 검토나 PR 변경분 30개 검토를 대체하지 않는다.
