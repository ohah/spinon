# C11.4 positive implicit track sizing 계획 적대 검토 20개

**대상:** [`c11-4-positive-implicit-track-sizing.md`](../../../../plan/c11-4-positive-implicit-track-sizing.md)와 이에 연결된 Chrome 155 precomparison 입력·기대값.

이 표는 구현을 검사하지 않는다. 각 항목은 다른 실패 경로 또는 경계 하나를 확인하고, 계획의 acceptance test·오류·범위 제한에 대응한다. 구현 뒤 code/runtime review와 계획 PR diff review는 별도로 수행한다.

| # | 실패 관점 | 계획에서 요구한 확인 |
| ---: | --- | --- |
| 1 | `grid-auto-rows/columns` computed list가 첫 값만 보존하거나 authored order를 바꾸는가 | snapshot에서 전체 순서·길이·axis를 보존하고 min/max list matrix를 정확 비교한다. |
| 2 | pattern index가 explicit track 수 기준 한 칸 앞서거나 늦게 시작하는가 | positive implicit 첫 track에 list[0]을 적용하고 explicit prefix를 그대로 둔다. |
| 3 | 3번째 이상 implicit track에서 modulo 순환이 끊기거나 list가 한 번만 복사되는가 | `[min-content,max-content]` 3-track과 `[10px,max-content]` 3-row를 확인한다. |
| 4 | columns와 rows가 같은 list buffer 또는 cursor를 공유해 한 축 결과가 다른 축을 덮는가 | 서로 다른 axis list를 동시에 쓰는 matrix와 per-axis trace를 비교한다. |
| 5 | default `auto` implicit track이 0px, 빈 track 또는 Unsupported로 바뀌는가 | Chrome row/column auto-placement controls와 native scene의 nonzero track를 검사한다. |
| 6 | min-content item의 longest unbreakable run 또는 Ahem revision이 무시되는가 | 70px min-content word case와 intrinsic source/layout revision을 확인한다. |
| 7 | max-content sizing이 line break를 유지해 70px로 줄어드는가 | 100px unwrapped Ahem case를 row/column output과 비교한다. |
| 8 | percentage basis가 inline size로 고정되어 row/column에서 잘못 계산되는가 | 축별 definite basis를 분리하고 projection/layout trace에 해당 axis를 연결한다. |
| 9 | `fr` implicit track이 explicit track/gap을 뺀 나머지 공간을 잘못 배분하는가 | 160px definite column에서 24px prefix 이후 136px 기대값을 고정한다. |
| 10 | `minmax()`나 `fit-content()`가 parse 단계에서 다른 track function으로 축약되는가 | 40px max limit과 70px min-content floor case를 독립 관찰한다. |
| 11 | `calc()`가 percentage와 absolute 성분을 버리거나 잘못된 basis를 쓰는가 | 160px × 10% + 5px = 21px case 및 기존 math projector를 사용한다. |
| 12 | explicit positive line placement와 auto-placement가 서로 다른 확장 경로에서 track sizing을 누락하는가 | 두 생성 원인을 둘 다 포함하고 expansion route별 Rust tests를 요구한다. |
| 13 | row-flow와 column-flow에서 implicit count가 반대 축에 추가되는가 | row/column cases, root/item geometry와 per-axis track traces를 각각 비교한다. |
| 14 | item이 explicit/implicit 경계를 span할 때 list 시작 위치나 occupied track이 바뀌는가 | cross-boundary span, occupied tracks, placement range를 runtime matrix에 추가한다. |
| 15 | `display:none`, ancestor-hidden 또는 empty child가 implicit track을 만들거나 pattern cursor를 이동시키는가 | hidden/empty item controls에서 track count와 next visible index를 확인한다. |
| 16 | negative line이 negative implicit track을 만들 때 forward cycle로 잘못 처리되는가 | negative implicit path를 이번 범위 밖으로 고정하고 명시 오류·partial-output 금지를 확인한다. |
| 17 | 10,000 track 상한 또는 `usize` arithmetic overflow가 truncate/wrap된 track list로 성공 처리되는가 | checked arithmetic, per-axis cap, exact cap and cap+1 rejection controls를 요구한다. |
| 18 | 목록이 비었거나 unsupported value가 있을 때 기본값 `auto`로 복구하는가 | unsupported property/node/value 진단과 fail-closed tests를 요구한다. |
| 19 | list/expanded-track reservation failure 뒤 일부 `LayoutOutput`이 반환되거나 이전 result가 남는가 | fallible reservation injection, atomic output, same-worker recovery를 확인한다. |
| 20 | 기존 C11.2.2/2.3 또는 C11.3 profile의 fixed px semantics가 함께 확장되는가 | 기존 profiles의 accepted/rejected inputs와 artifact boundary를 별도 회귀한다. |

## 계획 수정 사항

이 검토에서 놓친 경계가 없도록 계획에 negative implicit track fail-closed, cross-boundary span, hidden item, 10,000/cap+1, allocation failure, existing-profile isolation 및 border 바깥 native surface sample을 명시했다. 기능 구현·app execution·artifact proof·CI 상태는 아직 수행하지 않았다.
