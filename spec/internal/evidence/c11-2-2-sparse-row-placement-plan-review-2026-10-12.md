# C11.2.2 계획 실패 관점 검토

**대상:** [행 우선 sparse 자동 배치와 암시 행 계획](../../../plan/c11-2-2-sparse-row-placement.md)  
**기준:** CSS Grid Level 1 2025-03-26 CRD §7.5–7.7·§8.3–8.5, Chrome 154 고정 환경, Taffy 0.14.0 소스  
**검토 범위:** 계획의 지원 경계·순서·비교 기준만 검토했다. 구현/실행 증거는 아니다.

| # | 공격 관점 | 반영한 방어·결과 |
| ---: | --- | --- |
| 1 | 이전 all-auto/all-explicit gate를 조금 완화하다 기존 profile 결과가 바뀐다. | 별도 `RuntimeBlockFlexGridAutoPlacementV1`과 전용 FFI fixture를 둔다. 기존 두 profile은 그대로 유지한다. |
| 2 | Grid 한 개의 placement mode를 형제·중첩 container가 공유한다. | 보이는 직계 child만 container별로 분류하고 nested Grid는 독립 판정한다. |
| 3 | `display:none` 자식이 occupancy를 차지하거나 지원 밖 값을 만들어 unrelated container를 실패시킨다. | 숨겨진 subtree는 분류·occupancy에서 제외하고 전용 fixture로 확인한다. |
| 4 | out-of-flow 자식이 auto-placement occupancy에 들어간다. | in-flow child만 대상으로 하고 positioned child는 기존 fail-closed 계약을 유지한다. |
| 5 | CSS `order`가 auto-placement 순서를 바꾸는데 DOM 순서를 사용한다. | `order != 0`은 거부해 지원 subset에서 order-modified order와 DOM order가 같도록 고정한다. |
| 6 | definite item이 auto item보다 늦게 occupancy에 등록되어 auto item이 겹친다. | definite placement를 먼저 처리하는 단계 순서를 계약과 fixture에 넣는다. |
| 7 | definite-row/auto-column item이 일반 row-major item보다 늦거나 일찍 배치된다. | row-locked 단계와 그 이후 remaining-item 단계의 순서를 분리한다. |
| 8 | definite-row item이 explicit columns를 넘으면서 implicit column을 조용히 만든다. | explicit column 안에서 못 찾으면 container 전체를 거부한다. |
| 9 | definite-column/auto-row item이 열을 고정하지 않고 일반 item처럼 배치된다. | cursor 열을 definite column으로 설정하고 빈 row를 검색하는 별도 scene을 요구한다. |
| 10 | sparse 알고리즘이 이전 hole을 다시 채워 dense 결과를 낸다. | cursor가 앞으로만 이동하는 장면과 `dense` 명시 거부 negative를 둔다. |
| 11 | item span이 occupied cell 검사를 무시해 겹치거나 잘못된 행에서 배치된다. | row/column span의 전체 직사각형을 occupancy 검사하고 span별 overlap fixture를 둔다. |
| 12 | `grid-row/column` conflict handling을 placement 계산 전에 생략한다. | line 역순 교환, 동일 line end 제거, 두 span 중 end 제거를 CSS 단계에서 normalize하고 Chrome 장면에 고정한다. |
| 13 | 음수 numeric line 기준이 implicit row 개수로 이동한다. | 음수 line을 기존 C11.2.1과 같이 explicit track 수 기준으로 계산하며 암시행 생성 전후 결과를 비교한다. |
| 14 | 0 line/span, i32/i16 overflow가 Taffy clamp와 wrap으로 성공한다. | typed conversion과 checked arithmetic을 Taffy 호출 전에 수행하고 경계 값을 negative test로 둔다. |
| 15 | auto-placement가 explicit rows 초과 시 implicit rows를 만드는 대신 잘리거나 반복 루프에 빠지고, track 상한을 explicit 개수에만 적용해 내부 총량을 넘긴다. | 이번 profile은 negative implicit을 만들지 않으므로 Taffy `MAX_GRID_TRACKS`에 맞춰 `explicit + positive implicit ≤ 10,000`을 각 축에서 checked arithmetic으로 검사하고 상한에서 전체 거부한다. Taffy loop를 제한 fixture와 hostile stress로 대조한다. |
| 16 | 기본 `grid-auto-rows:auto` 또는 복수 pattern이 px 한 track과 동일 취급된다. | 실제 implicit row가 필요할 때 single non-negative px만 허용하고 default auto/복수/%/fr를 거부한다. implicit row가 없는 장면의 초기값은 미사용 값으로만 취급한다. |
| 17 | 암시 열/negative implicit row를 허용하면서 여기서 명시하지 않은 축 track sizing에 진입한다. | 이번 단계는 positive implicit rows만 생성한다. 그 외 track 생성이 필요한 입력은 전부 fail-closed한다. |
| 18 | intrinsic/text 크기가 implicit track 또는 auto item 크기에 영향을 주는데 빈 box fixture로 전체 지원을 주장한다. | 모든 auto-placement item의 width/height를 fixed로 제한하고 text/replaced/intrinsic sizing을 명시 제외한다. |
| 19 | unsupported item 하나의 실패 뒤 partial frame이나 stale revision이 새 결과로 보인다. | Grid 계산 성공 후 revision 단위로 publish하고, 실패 후 기존 snapshot 보존 및 같은 worker 유효 입력 복구를 완료 관문으로 둔다. |
| 20 | Chrome WPT/AVD/Simulator 결과를 Spinon WPT 적합성·hardware GPU·전체 CSS 지원으로 부풀린다. | pinned Chrome, Spinon fixture, 공식 WPT 실행, Android AVD/실기기, iOS Simulator, surface readback을 별도 artifact와 상태로 기록한다. |

## 검토 후 계획 수정

검토 중 초기 초안의 `grid-auto-rows:auto` 지원 여부가 고정 item만으로 입증되지 않았고, implicit columns가 자동 생성될 수 있는 경우가 명확히 분리되지 않은 점을 찾아 수정했다. 최종 재검토에서는 Taffy track 상한을 explicit 개수에만 적용할 여지가 있어 explicit+implicit 총량으로 제한하도록 보완했다. 현재 계획은 암시 행이 필요할 때 단일 non-negative px만 허용하고, implicit columns 및 negative implicit rows는 전체 계산을 거부한다. 별도 실패 관점 표는 계획 검토이며 구현 검토나 20회 실행을 대신하지 않는다.

## 구현 전 잔여 조건

계획은 구현 가능한 범위와 실패 동작을 고정했다. 구현 전 Chrome 154 reference·WPT 원문 hash를 새 fixture로 고정해야 하며, 그 결과가 이 계약과 충돌하면 구현 전에 계획과 이 검토를 다시 갱신한다.
