# C11.2.1 명시 Grid 배치 계획 · 실패 관점 검토

**검토일:** 2026-10-12  
**상태:** 계획 조정 반영 · 구현 전  
**대상:** [`plan/c11-2-1-explicit-grid-placement.md`](../../../plan/c11-2-1-explicit-grid-placement.md)  
**규범 비교:** [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/), §8.3–8.5  
**계산기 버전:** 저장소에서 고정한 Taffy `0.14.0`; 결과 자체는 규범 oracle로 취급하지 않음

| # | 공격 관점 | 계획에 반영한 방어·판정 |
| ---: | --- | --- |
| 1 | 음수 line을 0-based 인덱스로 그대로 넘겨 explicit grid가 아닌 implicit 쪽에 놓일 수 있다. | explicit track 개수 기준으로 먼저 정규화하고 CSS positive/negative 방향을 fixture에서 비교한다. |
| 2 | `grid-row: 0 / 2`가 Taffy의 Auto 변환을 타서 정상 배치처럼 보일 수 있다. | line 0을 adapter 경계에서 거부한다. |
| 3 | 2,147,483,647 같은 computed integer가 `i16`으로 잘리거나 wrap될 수 있다. | checked conversion 후 오류를 내고 Taffy에는 검증값만 전달한다. |
| 4 | `span 0`·음수 span이 track 1개 또는 Auto로 바뀔 수 있다. | positive span만 허용하고 0/음수·범위 초과를 별도 오류 case로 둔다. |
| 5 | span을 단순 line 인덱스로 취급하면 end line이 하나 밀릴 수 있다. | line+span과 span+line 양 방향의 정규화 결과 및 실제 rect를 oracle로 고정한다. |
| 6 | `grid-row: 4 / 2`의 line swap 규칙을 빠뜨리면 Taffy와 Chrome이 다를 수 있다. | 역순 line 쌍을 CSS conflict handling으로 swap한 뒤 비교한다. |
| 7 | 동일 start/end line에서 CSS가 end를 제거하고 auto placement로 넘어가는 것을 잘못 처리할 수 있다. | 이번 slice에서 그 자동 placement로 조용히 전환하지 않고 명시적으로 거부한다. |
| 8 | 보이는 item 하나의 끝 line이 track 개수 밖이면 Taffy가 implicit track을 생성해 성공처럼 출력할 수 있다. | track 개수+1 경계를 넘는 placement는 입력 전에 거부한다. |
| 9 | 두 span 또는 span만 있는 축은 충돌 해결·auto placement가 필요하다. | 양 끝을 definite로 만들지 못하는 span 조합을 거부하고 다음 단계로 분리한다. |
| 10 | line만 있는 축과 반대 축 Auto가 섞이면 CSS auto-placement cursor 규칙이 개입한다. | 두 축 모두 fully definite인 all-explicit 집합 또는 기존 all-auto 집합만 허용한다. |
| 11 | WPT 후보 경로나 revision이 느슨하면 이후 upstream 변경에 따라 같은 계획이 다른 case를 검증할 수 있다. | 구현 전에 고정 revision에서 실제 후보를 고르고 source hash·Chrome 실행을 기록한다. 적절한 WPT가 없으면 자체 fixture와 미선정 이유를 남긴다. |
| 12 | `grid-row`·`grid-column` shorthand 또는 `grid-area`가 raw source string만 보고 누락될 수 있다. | raw shorthand 파서를 추가하지 않고 Stylo가 계산한 네 longhand 값으로 cascade/전개를 확인한다. 한 축이라도 `auto`로 남는 `grid-row:2`, `grid-area:2 / 4`는 아직 거부한다. |
| 13 | shorthand와 longhand declaration 순서·cascade priority가 잘못 합쳐질 수 있다. | 같은 property 조합을 바꾼 oracle fixture에서 computed longhand 값을 대조한다. |
| 14 | `foo 2`, `span foo`, named line이 numeric line처럼 변환될 수 있다. | ident가 붙은 computed GridLine은 거부하고 이름 조회는 C11.3으로 둔다. |
| 15 | 비 Grid element의 기본 `auto` 값이나 숨겨진 child의 stale placement가 전체 계산을 막을 수 있다. | 실제 보이는 Grid child에 한해 placement를 투영·검증하며 `display:none` subtree는 제외한다. |
| 16 | child NodeId가 다른 container에 붙었거나 parent map이 stale하면 다른 Grid의 배치가 적용될 수 있다. | 동일 snapshot/root와 직접 Grid-parent identity를 검증하고 orphan/mismatched child negative case를 둔다. |
| 17 | overlap geometry를 중복 셀 오류로 취급하거나 source node 배열을 재정렬할 수 있다. | overlap을 CSS상 유효 입력으로 두고 DOM/source order 불변 및 frame 독립성을 검사한다. |
| 18 | 겹친 item의 색상 가림 또는 hit-test 순서를 geometry 통과로 오인할 수 있다. | geometry, draw order, hit-test를 별도 관찰하고 C12 stacking semantics를 범위에 넣지 않는다. |
| 19 | 새 배치 profile의 exhaustive match나 FFI 선택에서 paint·platform 경계가 빠지거나, `order`·position·intrinsic size가 암묵적으로 들어올 수 있다. | profile 이름을 `RuntimeBlockFlexGridPlacementV1`로 고정하고 전체 match·paint·FFI 경로를 검토한다. order 0·static과 auto/fixed item dimension만 허용하고 intrinsic sizing은 제외한다. |
| 20 | 일부 item만 변환된 채 Taffy가 실행되거나 새 오류 뒤 이전 revision 결과를 재사용할 수 있다. | container 단위 사전 분류·변환을 먼저 끝낸 뒤 계산하고 실패 시 전체 snapshot을 publish하지 않는다. |

## 남은 판정 경계

계획은 이름 line·implicit grid·혼합 auto placement의 지원을 약속하지 않는다. `grid-template-areas`와 line 기반 shorthand, 동일 line 충돌 처리의 자동 배치 결과는 여전히 후속 범위다. 모바일 surface readback이 수집되지 않으면 WGPU present를 pixel parity로 승격하지 않는다. 계획 자체에는 미해결 구현 계약이 남지 않았으며, 구현 중 Stylo의 typed value와 Taffy placement 정규화가 계획에 맞지 않으면 코드보다 계약을 먼저 다시 검토한다.
