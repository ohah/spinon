# 계획 실패 관점 검토 20개

**대상:** [`c11-4-auto-repeat-zero-fixed-maximum.md`](../../../../plan/c11-4-auto-repeat-zero-fixed-maximum.md)
**범위:** Chrome oracle 이후 구현 전에 입력·계산·검증·상태 경계를 깨뜨릴 수 있는 서로 다른 실패 경로 20개를 검토했다.

| # | 실패 관점 | 판정 및 근거 |
| ---: | --- | --- |
| 1 | `minmax(inflexible, 0px)`가 fixed-size 문법 범위를 벗어날 수 있음 | 통과. 0px은 음수가 아닌 fixed breadth이며 이번 계획은 해당 existing branch의 실행만 다룬다. |
| 2 | CSS 명세가 1px UA 최소값을 강제한다고 과장할 수 있음 | 통과. 명세는 UA 지정 최소값을 요구하고 1px은 제안값이다. 1px은 Chrome oracle 및 Spinon/Taffy의 명시적 target으로 한정한다. |
| 3 | repeat 수 식에서 첫 track 또는 gap 수를 하나 더 셀 수 있음 | 통과. `n × 1 + (n − 1) × gap ≤ 16`의 가장 큰 양의 정수를 직접 사용한다. |
| 4 | `auto-fit` collapse 뒤 반복 수만 세어 fill과 구별하지 못할 수 있음 | 통과. computed track list의 길이와 item offset을 모두 관찰한다. |
| 5 | definite axis가 없어 percentage/auto sizing에 기대어 oracle이 흔들릴 수 있음 | 통과. 반복 axis는 명시적으로 16px이며 non-repeated axis도 고정한다. |
| 6 | gap 4px에서 기대 count가 소수 반올림에 의존할 수 있음 | 통과. 16px·1px·4px 정수 산술로 4개가 결정된다. |
| 7 | 세 번째 track이 없는 작은 repeat count에서 item이 implicit track으로 이동할 수 있음 | 통과. gap 4px 최소 repeat count가 4이고 item은 line 3에서 시작한다. |
| 8 | item이 자동 최소 크기를 통해 우연히 크기를 얻어 최소값 타입별 차이를 오판할 수 있음 | 통과. 12px item min-size를 명시하고 문서에서 이번 행렬이 intrinsic contribution의 일반 차이를 입증하지 않는다고 제한한다. |
| 9 | occupied track의 12px이 선언된 0px maximum과 충돌한다고 잘못 단언할 수 있음 | 통과. Chrome oracle은 used track serialization을 관찰값으로 기록하며 일반 sizing 결론을 과장하지 않는다. |
| 10 | 빈 `auto-fill` track을 `auto-fit`처럼 접어 offset 비교가 사라질 수 있음 | 통과. 각 mode를 별도 입력으로 만들고 Chrome에서 item offset을 기록한다. |
| 11 | `auto-fit` collapse가 track뿐 아니라 연결 gutter에도 적용되는 것을 놓칠 수 있음 | 통과. gap 4px에서 fill offset 8px과 fit offset 0px을 oracle 및 runtime 기준에 넣었다. |
| 12 | gap 0 대조군이 잘못된 축의 gap을 바꾸거나 직교축에 간격을 만들 수 있음 | 통과. axis별 row/column gap을 직접 지정하고 직교 gap은 항상 0px이다. |
| 13 | row-axis 결과가 column-axis 결과와 다를 수 있음 | 통과. 수평·수직 양축을 각각 12 scene으로 실행하고 축별 item offset을 검사한다. |
| 14 | computed CSSOM serialization을 authored syntax와 동일시할 수 있음 | 통과. JSON은 authored `minimum`/`maximum` 입력과 computed track list를 분리한다. |
| 15 | post-collapse CSSOM track count가 pre-collapse repeat 수를 숨길 수 있음 | 통과. Chrome CSSOM이 16/4개 track을 보존함을 사전 비교했고 runtime trace의 expanded count도 별도 검사한다. |
| 16 | `1px` positive control이 zero-floor 동작을 구분하지 못할 수 있음 | 수정. 계획 검토에서 결함을 발견해 `2px` control과 gap별 예상 8/3 track으로 바꿨다. |
| 17 | direct Taffy count test만으로 Spinon preflight 또는 scene adapter를 입증할 수 있음 | 통과. direct test, typed layout/runtime adapter, 앱 simulator 행렬을 별도 계층으로 둔다. |
| 18 | root 내부 빈 영역, item, 바깥 표본이 서로 겹칠 수 있음 | 통과. 픽셀 표본을 layout 결과를 기준으로 분리하고 scene revision과 함께 비교하는 조건을 둔다. |
| 19 | Android software AVD 또는 iOS pre-submit drawable을 실기기/최종 compositor 증거로 확대할 수 있음 | 통과. 계획에 각각의 한계를 명시한다. |
| 20 | 작은 경계를 C11.4 전체나 공식 완료 수 승격으로 오인할 수 있음 | 통과. C11.4·C11 checkbox와 `72/213`을 유지한다고 계획 첫머리와 종료 조건에 적었다. |

**결과:** 20개 관점을 모두 확인했다. 16번 발견을 계획에 반영했으며, 구현 전에 계획을 다시 읽어 positive control의 최종 2px 기대값을 확인했다.
