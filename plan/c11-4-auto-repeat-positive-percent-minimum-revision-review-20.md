# C11.4 positive percentage auto-repeat minimum 계획 수정 실패 관점 20개

첫 계획의 중단 조건이 발동한 후, 새 typed-value 범위와 비교 기준을 implementation 전에 검토했다. 이전 계획 리뷰 20개와 합치지 않는다.

| # | 수정 계획 실패 관점 | 판정 기준 |
|---:|---|---|
| 1 | computed CSS가 authored calc와 direct percent를 구별한다고 가정한다. | 실제 `Percentage(0.1)` normalization log를 기준으로 두 authored form을 같은 typed scope로 묶는다. |
| 2 | 정규화 발견을 무시하고 Calc-only feature로 계속 구현한다. | 이전 계획을 superseded로 표기하고 새 계획 PR을 구현 전에 통과시킨다. |
| 3 | direct percentage가 새 범위에 포함된다는 사실을 숨긴다. | `minmax(10%,1fr)`를 명시적으로 허용 입력·Chrome scene·test matrix에 둔다. |
| 4 | zero percentage를 positive minimum과 혼동한다. | 새 admission은 finite `p > 0`; zero percentage는 거부하고 별도 0-size floor를 구현하지 않는다. |
| 5 | 음수, NaN 또는 overflow하는 percentage를 허용한다. | typed validation과 `percentage×positive content basis` finite 검사를 통과 조건으로 둔다. |
| 6 | percentage를 max 함수나 bare track에도 무심코 허용한다. | 새 gate는 `minmax(Percentage, fixed px | flex)`에만 적용한다. |
| 7 | border-box 124px를 percentage basis로 쓴다. | content-box CSS width/height 120px와 border 2px를 분리해 content basis를 고정한다. |
| 8 | border가 있어 track/item 시작 좌표를 0으로 비교한다. | root border edge와 item content origin 2px offset을 모든 frame 기대값에 둔다. |
| 9 | auto-fill의 반복 수에서 border/gap을 제외하는가? | 120px content, gap 0, 12px minimum에서 정확히 10개인지 검사한다. |
| 10 | auto-fit에서 collapse를 반복 수와 혼합한다. | 먼저 10개를 만들고 한 점유 track 외 9개를 placement 뒤 0으로 접는다. |
| 11 | rows 퍼센트를 inline basis로 resolve한다. | row min은 definite inner block 120px의 10% 및 150%를 사용한다. |
| 12 | 150% minimum에서 repeat count를 0으로 허용한다. | 아무 양의 repetition도 fit하지 않아도 CSS 규칙에 따라 1개 track을 둔다. |
| 13 | 180px track overflow가 root에 가려져 확인되지 않는다. | 120px content/124px border box와 180px item의 58px outer-border overflow를 측정한다. |
| 14 | calc, direct percent, fixed control이 서로 다른 frame을 낸다. | 10% 12-scene matrix와 overflow 4-scene matrix 각각 exact equality를 요구한다. |
| 15 | test root는 app runtime root와 다른 box-sizing을 사용한다. | content-box, 2px border, zero padding을 browser와 app fixture에 함께 적용한다. |
| 16 | zero basis에서 Taffy zero divisor 경로에 도달한다. | positive explicit content basis 사전 검사에서 typed `InvalidGridTrackList`로 거부한다. |
| 17 | indefinite/cyclic input을 positive fixed basis로 오인한다. | 명시적인 positive fixed dimension 이외는 새로운 percentage gate에서 통과하지 않는다. |
| 18 | Chrome screenshot을 pixel parity나 Spinon runtime으로 과장한다. | screenshot은 Chrome 관측으로만 기록하고 app simulator·surface evidence를 분리한다. |
| 19 | 실패했던 AST probe를 제품 테스트 실패처럼 보고한다. | 로그에 assertion의 잘못된 기대값과 actual `Percentage(0.1)`을 그대로 남긴다. |
| 20 | 계획 변경으로 C11.4 완료 상태/계약 버전이 이동한다. | checkbox, `72/213`, 내부 계약 0059 `0.1.0`을 유지한다. |

새 scope는 실패한 authored-calc 가정을 고쳤으며 일반 percentage CSS 지원이나 C11.4 완료를 주장하지 않는다.
