# C11.4 auto-fit named-line implicit fallback 계획: 독립 실패 관점 20개

계획 PR에 앞서 구현과 분리해 검토했다. 각 관점은 별도 실패 경로를 가리키며, 해결책은 고정 비교 fixture와 acceptance rule에 반영했다.

| # | 실패 관점 | 계획의 방지 기준 |
|---:|---|---|
| 1 | repeat count를 collapse 후 track 수로 세어 named line index가 바뀐다. | 125px definite axis에서 placement 전 3회 반복을 고정하고 post-collapse size를 별도 비교한다. |
| 2 | fallback이 implicit line을 만들면 이를 occupied repeated track으로 오인해 auto-fit collapse가 달라진다. | absent/insufficient fallback scene에 in-flow item 하나만 두고 §7.2.3.2 순서로 기대값을 계산한다. |
| 3 | 모든 repeat가 빈 상황에서 전체 auto-fit track은 collapse될 수 없다고 가정한다. | fallback item이 repeat 바깥에 있는 양·음수 사례를 별도 둔다. |
| 4 | 양수 absent-name fallback이 explicit end 뒤 필요한 implicit track 개수를 잘못 만든다. | `5 ghost / 6 ghost`의 track list와 `(100,0) 20×10` frame을 함께 고정한다. |
| 5 | 양수 existing-name occurrence 부족을 absent-name case와 동일하다고 가정한다. | `5 cell / 6 cell`을 별도 scene으로 두고 `(20,0)` 및 implicit track 수를 판정한다. |
| 6 | 음수 occurrence를 leading explicit grid line이 아니라 collapsed used geometry 끝에서 센다. | `-5 ghost / -4 ghost`와 negative track list/frame을 독립 비교한다. |
| 7 | 음수 `cell` 부족 occurrence에 leading implicit track을 덜 만들거나 더 만든다. | `-5 cell / -4 cell`을 absent-name negative와 따로 고정한다. |
| 8 | forward named span이 explicit repeat 영역 바깥 방향에 있는 implicit line을 잘못 센다. | line 4에서 `span ghost`를 두고 §8.3의 탐색 방향과 zero-width collapse 후 frame을 확인한다. |
| 9 | backward span이 explicit 영역 반대편 implicit line까지 센다. | `span ghost / 1`을 따로 두고 leading 20px track 및 root-relative geometry를 판정한다. |
| 10 | repeat line-group 경계의 중복 `[cell]` 이름이 병합되어 occurrence 개수가 변한다. | Chrome의 named line groups를 포함한 전체 computed track serialization을 exact 비교한다. |
| 11 | adjacent collapsed track을 하나로 합쳐 track identity가 사라진다. | 크기만 합산하지 않고 각 0px track과 line group 순서를 보존해 비교한다. |
| 12 | 한 번이라도 track을 점유한 in-flow item이 있는데 그 track까지 collapse된다. | `occupied-repeat` control에서 정확히 한 40px track이 남는지 확인한다. |
| 13 | item이 두 repeat를 가로지르는데 한쪽만 occupied로 인식된다. | `span-occupied-repeat` control은 `[0,40,40]` track sizes와 80px frame을 요구한다. |
| 14 | columns 동작만 맞고 rows의 auto-fit fallback은 다른 경로에서 실패한다. | 양수·음수 row-axis scene을 각각 포함한다. |
| 15 | row 시험의 column 축도 implicit sizing으로 바뀌어 오차 원인을 가린다. | 각 row scene은 고정 20px column을 cross-axis control로 둔다. |
| 16 | border/padding/gap이 125px inner axis를 바꿔 repeat count가 3이 아니게 된다. | fixture는 `box-sizing:border-box`, border/padding 0, gap 0, axis 125px로 잠근다. |
| 17 | item content/min-size가 track sizing에 끼어 fixed 20px implicit size를 오염시킨다. | 빈 item, `min-width:0`, 고정 높이, fixed `grid-auto-*` size를 사용한다. |
| 18 | fully specified placement인데 automatic placement가 추가 row/column을 만든다. | scene마다 단일 in-flow item에 start와 end를 모두 지정한다. |
| 19 | computed CSSOM placement 문자열을 resolved internal line index의 증거로 오해한다. | 판정은 exact tracks/groups와 root-relative frame이며 computed placement는 원본 관찰만 보존한다. |
| 20 | 제한 fixture 통과를 공개 CSS 지원 또는 전체 C11.4 완료로 확대한다. | opt-in fixture, 미지원 grammar, simulator 범위와 `77/213` 미완료 상태를 문서에 고정한다. |

## 판정

각 관점에 재현 입력과 검출 근거가 계획에 연결되어 있다. Chrome fixture 관측은 W3C 독립 기대값과 분리 저장했다. 구현을 시작하기 전 이 표를 재사용하지 않고 새 runtime review 20개를 작성한다.
