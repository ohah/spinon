# C11.2.2 Grid Vec 예약 실패 주입 계획 검토

이 문서는 [C11.2.2 계획](../../../plan/c11-2-2-sparse-row-placement.md)의 남은 Grid Vec 예약 실패 검증 설계를 코드 변경 전에 점검한다. 범위는 명시적으로 실패 처리하는 다섯 `Vec::try_reserve[_exact]` 호출이다. 전역 OOM 복구나 WPT 적합성은 포함하지 않는다.

| # | 공격 관점 | 검토 결과 |
|---:|---|---|
| 1 | 다른 Taffy나 트리 할당을 실패시키고 Grid 검증으로 오인하는가 | failpoint는 지정한 Grid reservation site 직전에서만 allocator에 arm하도록 계획했다. |
| 2 | allocator를 우회한 가짜 `Err`를 반환하는가 | 실제 `Vec::try_reserve[_exact]` 호출의 allocator request를 실패시킨다. allocator 실패 관측이 없으면 테스트 실패다. |
| 3 | failpoint가 전역이라 테스트 병렬 실행을 깨뜨리는가 | 주입 요청과 소비 상태 모두 thread-local one-shot으로 제한한다. |
| 4 | failpoint가 reserve 호출 전에 테스트 harness allocation을 소비하는가 | target-site wrapper가 예약 직전 무장하며, allocator 호출 전 중간 할당을 만들지 않는다. |
| 5 | Vec에 이미 capacity가 있어 allocator를 부르지 않는가 | 각 대상은 계산 중 새로 만든 빈 Vec이며 입력은 양수 용량을 요구한다. 테스트는 실제 allocator 소비를 단언한다. |
| 6 | visible-child item list 예약을 놓치는가 | `visible_children.len() > 0`인 fixture로 별도 site를 지정한다. |
| 7 | row-locked cursor 예약을 놓치는가 | explicit row가 하나 이상인 fixture로 cursor Vec의 positive reserve를 요구한다. |
| 8 | expanded row track 예약을 놓치는가 | implicit row가 실제 생성되는 fixture로 `expanded_rows` 예약을 강제한다. |
| 9 | occupancy row storage 예약을 놓치는가 | definite item 배치로 빈 occupancy row storage를 확장한다. |
| 10 | row interval 예약을 놓치는가 | 새 row의 빈 interval Vec에 definite item interval을 삽입한다. |
| 11 | `alloc`만 가로채고 `realloc`이나 zeroed allocation 경로를 놓치는가 | test allocator는 alloc/realloc/alloc_zeroed를 모두 전달 또는 실패시킨다. |
| 12 | 실패 node가 container와 item 사이에서 잘못 귀속되는가 | 각 호출 위치가 현재 전달하는 node ID를 예상값과 대조한다. |
| 13 | 오류 property가 넓거나 잘못된 값으로 바뀌는가 | 각 site의 `GridAllocationFailure.property`를 개별 검증한다. |
| 14 | 실패 뒤 partial layout/frame이 공개되는가 | `compute` 결과가 정확한 `Err`인지 확인하며 `LayoutOutput` 성공을 허용하지 않는다. |
| 15 | engine 재사용을 다른 인스턴스나 worker로 가장하는가 | 같은 thread에서 동일한 `TaffyLayoutEngine` instance로 후속 계산한다. |
| 16 | 복구 계산이 stale input/revision을 재사용하는가 | 실패와 복구에 같은 고정 입력·revision을 사용하고 성공 frame을 확인한다. |
| 17 | 한 번 실패 후 주입 상태가 남아 복구 계산까지 실패하는가 | one-shot 소비 상태와 후속 계산의 성공을 함께 단언한다. |
| 18 | assertion panic 뒤 다른 test에 failpoint가 누출되는가 | scope guard가 unwind/drop 때 target·allocator 상태를 초기화한다. |
| 19 | production binary/ABI에 test hook이나 custom allocator가 포함되는가 | 주입 hook과 global allocator는 `cfg(test)` 전용으로 둔다. |
| 20 | 결과를 모든 OOM·모바일 allocator·WPT에 과장하는가 | 문서에서 다섯 명시적 Vec 예약만 완료 범위로 두고 tree collections, global OOM, OS allocator 및 WPT를 별도 미완료로 남긴다. |

## 검토 후 고정한 구현 경계

- 테스트는 현재 구현의 다섯 reservation callsite를 하나씩 실제 allocator failure로 밟아야 한다.
- 정확한 오류 귀속, 성공 output 부재, 같은 thread·engine instance의 정상 복구를 모두 확인해야 한다.
- `BTreeMap`·`BTreeSet` 등 infallible allocator 동작을 포함해 전역 OOM 안전성을 주장하지 않는다.
- 구현 후에는 이 문서를 실행 근거와 대조해 결과·차이·남은 범위를 갱신한다.
