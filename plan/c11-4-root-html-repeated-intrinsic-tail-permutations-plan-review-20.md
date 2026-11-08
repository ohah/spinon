# C11.4 repeated intrinsic tail permutations 계획 실패 관점 검토 20개

검토 대상은 [`c11-4-root-html-repeated-intrinsic-tail-permutations.md`](./c11-4-root-html-repeated-intrinsic-tail-permutations.md)와 독립 Chrome 사전 비교다. 이는 구현 전 계획 검토이며 구현/runtime 검토나 PR diff 검토에 재사용하지 않는다.

| # | 독립 실패 관점 | 판정과 반영 |
| --- | --- | --- |
| 1 | `grid-auto-rows`가 repeated track-size 목록을 문법상 허용하지 않는 경우 | 통과. CSS Grid 2 §7.6의 value grammar는 `<track-size>+`이고 여러 값은 필요한 implicit tracks에 반복 적용된다. |
| 2 | 후행 multiset에서 고유 순서 개수를 잘못 계산한 경우 | 통과. `5!/(2!·2!)=30`이며 생성기가 30 unique suffix를 assert한다. |
| 3 | 기존 #1284 order가 30-order candidate에서 누락되는 경우 | 통과. 기존 suffix `min max auto min max`는 candidate multiset의 한 순서다. |
| 4 | `fit-content()`가 첫 위치에서 이동하여 overflow topology가 달라지는 경우 | 통과. fit-content는 모든 order에서 첫 member로 고정하고 위치 이동은 범위 밖으로 남긴다. |
| 5 | explicit row가 자동 목록 cycle에 섞이는 경우 | 통과. 첫 explicit row는 별도 fixed 10px이며 implicit item 1이 list index 0을 사용한다. |
| 6 | 두 번째 list cycle의 시작점이 하나 밀리는 경우 | 통과. 8개 implicit items는 sequence index `0..5,0..1`을 덮고, expected/Chrome 모두 9개 used rows를 확인한다. |
| 7 | 60px payload의 반복 fit-content row가 cap보다 커지는 경우 | 통과. row 7은 20px로 clamp되고 payload는 60px 및 visible overflow를 유지한다. |
| 8 | `20%` cap이 definite 100px basis가 아닌 다른 basis를 쓰는 경우 | 통과. container height 100px은 definite이며 Chrome은 `fit-content(20%)` scene에서 20px used cap을 관찰했다. |
| 9 | `20px`와 `20%`가 동일 고정 입력에서 다른 geometry를 만드는 경우 | 통과. px/% 60 scene의 track 및 box 값이 independent model과 일치했다. |
| 10 | `auto` track이 두 번 나오는 suffix에서 free-space 분배 개수를 하나로 가정하는 경우 | 통과. 자동 행은 suffix 첫 위치일 때 row 2와 row 8에 나타나며 CSS Grid 2 §12.8의 equal distribution model을 적용한다. |
| 11 | `auto` track이 한 번 나오는 24개 order에서 남은 4px이 엉뚱한 track으로 들어가는 경우 | 통과. 이 24개 order에서는 유일한 auto track만 4px을 받고 모든 row sizes가 Chrome과 exact match다. |
| 12 | negative free space가 stretch 계산에 섞이는 경우 | 통과. 모든 scene 독립 계산에서 cap 적용 후 total은 96px이며 free space는 양수 4px이다. |
| 13 | `%` cap의 computed serialization이 px cap으로 조용히 정규화되는 경우 | 통과. 20% 입력의 computed `grid-auto-rows` order/type은 `fit-content(20%)`로 유지되고 used geometry는 20px이다. |
| 14 | 반복되는 min/max track의 순서가 computed style에서 정렬·합쳐지는 경우 | 통과. 120개 computed track-list serialization이 각각의 expected member sequence와 일치했다. |
| 15 | fixed-px payload 비교를 임의 intrinsic contribution 검증으로 과장하는 경우 | 통과. README와 계획은 min/max intrinsic contribution이 이 fixture에서 payload height와 같고 arbitrary content sizing은 미검증이라고 제한한다. |
| 16 | sibling, 추가 item, 다른 explicit shape가 고정 topology에 섞이는 경우 | 통과. 후보 admission은 single Grid, explicit row 하나, exact eight payload item 및 fixed container shape만 허용한다. |
| 17 | 잘못된 duplicate 수 또는 여섯-member가 아닌 목록이 들어오는 경우 | 통과. 허용 multiplicity는 정확히 `fit-content×1`, `min-content×2`, `max-content×2`, `auto×1`이다. |
| 18 | 실패 revision이 partial layout/render를 publish하거나 후속 revision이 복구되지 않는 경우 | 통과. 구현 계획은 property/node error, atomic rejection 및 same-session recovery를 필수 gate로 명시했다. |
| 19 | Chrome observation을 독립 expected value로 재사용하는 경우 | 통과. `expected.json`은 fixed-payload track formula로 생성하고 browser output은 별도 verifier input으로만 썼다. |
| 20 | bounded slice가 일반 CSS, WPT 또는 C11.4 완료로 게시되는 경우 | 통과. 공개 admission/WPT/full intrinsic contribution 범위를 제외하고 C11.4/C11 체크 및 `78/214`를 유지한다. |

## 결론

계획은 30개 후행 순서를 제한된 profile로 다루고 Chrome 157 사전 비교 120/120, 9,480 coordinate/track checks와 독립 기대값 0 CSS px 차이를 연결한다. 계획 누락은 발견되지 않았다. 계획·비교가 분리되어 있으며 아직 구현은 시작하지 않았다.
