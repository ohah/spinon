# C11.2.1 hidden cascade-context 회귀 구현 검토 · 2026-10-13

변경 대상은 incremental cascade에서 재사용된 context ancestor의 `display:none` 상태가 후손 traversal에 전달되지 않던 오류다. 확인한 핵심 코드는 [`compute.rs`](../../../crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs), 회귀 테스트는 [`runtime_grid_tests.rs`](../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests.rs) 및 [`c11_2_1_grid_placement_tests.rs`](../../../crates/spinon-style-to-layout/src/c11_2_1_grid_placement_tests.rs)다.

| # | 실패를 노린 입력/경계 | 검토 결과 | 근거 |
| --- | --- | --- | --- |
| 1 | 첫 전체 cascade에서 visible root 아래 `display:none` 생성 | 새 flag는 기본 false에서 시작하고 computed display로 전환된다 | 회귀 테스트 |
| 2 | 재계산 경계의 바로 위 ancestor를 context로 재사용 | 재사용된 style에서 숨김 상태를 복원한다 | 수정된 context-node 분기 |
| 3 | context보다 더 위쪽의 일반 ancestor가 재사용됨 | 동일하게 reused-output 분기에서 display를 검사한다 | 수정된 일반 reuse 분기 |
| 4 | hidden ancestor가 여러 층 중첩됨 | `ancestor_hidden || current_display_none`라 하위가 이를 해제하지 못한다 | 코드 경로 점검 |
| 5 | hidden parent 아래 child만 dirty | 자식 work item이 숨김 flag를 받는다 | `incremental_restyle_preserves_hidden_context_for_unsupported_nested_grid` |
| 6 | hidden wrapper 아래 `repeat(auto-fit, 80px)` Grid | unsupported sizing이 hidden subtree의 placement 결과를 만들지 않는다 | 회귀 테스트 + 수정 전 negative control |
| 7 | hidden Grid 안의 item만 dirty | dirty item의 `grid_item_placement`가 없음 | 회귀 테스트 assertion |
| 8 | hidden item에 named line을 둠 | 지원하지 않는 named placement를 visible layout 오류로 승격하지 않는다 | `hidden_grid_item_ignores_unsupported_constraints_and_alignment` |
| 9 | hidden item에 min/max constraint와 alignment를 둠 | hidden item은 zero frame이며 placement projection이 없다 | 같은 style-to-layout 테스트 |
| 10 | 직접 `display:none`인 Grid item | placement 계산에서 제외되고 zero frame을 유지한다 | style-to-layout 회귀 테스트 |
| 11 | visible grid item에 unsupported constraint를 둠 | hidden 전용 예외가 visible 입력을 허용하지 않도록 기존 reject test가 유지된다 | `grid_item_min_max_constraints_remain_outside_the_explicit_placement_profile` |
| 12 | hidden sibling 다음의 visible sibling | 각 자식 tuple은 동일 부모 상태에서 독립적으로 생성된다 | 순회 tuple의 부모별 flag 전달 검토 |
| 13 | hidden parent 아래 자식의 computed display가 `grid` | OR 전파로 조상 숨김 상태가 보존된다 | 코드 경로 점검 |
| 14 | 숨기지 않은 nested Grid | 자식의 computed `display:grid`는 hidden flag를 설정하지 않는다 | nested placement 기존 fixture + 코드 경로 점검 |
| 15 | `visibility:hidden`을 `display:none`과 혼동 | 이 변경은 computed `display`만 확인한다; visibility 동작을 바꾸지 않는다 | 속성 선택 검토, 별도 visibility 계약은 기존 scope |
| 16 | `display:contents` 아래 후손 | `display:none`으로만 판단하므로 subtree를 숨기지 않는다 | 속성 선택 검토 |
| 17 | root 자체가 hidden | root output에서도 computed display를 읽어 후손 tuple에 전달한다 | root 경로 검토 |
| 18 | computed properties에 display key 누락 | `is_some_and`는 이 경우 false; 실제 Stylo computed output에서 display가 항상 제공되는지 별도 기존 profile 테스트로 점검 | 구조 점검; 합성 누락 output은 이번 test에서 만들지 않음 |
| 19 | 재사용된 output을 clone한 뒤 전파 | 소유 snapshot을 수정하지 않고 cloned properties에서 값만 읽는다 | 재사용 clone 경로 검토 |
| 20 | worklist push/pop 순서와 reuse 통계 | 각 tuple에 flag가 추가될 뿐 reverse-child DFS 순서와 counter 증분은 변하지 않는다 | traversal diff 및 전체 회귀 |

## 실제 회귀 증거

- Negative control: 숨김 상태를 복원하는 대입을 제거하자 새 incremental test가 `grid_item_placement.is_none()`에서 실패했다. [원본 로그](c11-2-1-current-source-rerun-2026-10-13/incremental-hidden-context-negative-control.log)
- 수정 후 incremental hidden-context test, hidden item placement test, 전체 suite가 통과했다. [현재 소스 실행 결과](c11-2-1-current-source-rerun-2026-10-13/README.md)
- Android AVD·Android SM-S731N 실기기·iOS Simulator matrix는 같은 작업 소스를 실행했지만 이 특정 incremental dirty-descendant 상태를 모바일 JS fixture로 직접 구동하지 않는다. 회귀 자체의 직접 증거는 Rust test와 negative control이다.

## 남은 한계

20개 관점 중 15·16·18번은 경계 동작의 코드 검토이며 별도 전용 실행 fixture가 아니다. 전역 allocator failure, arbitrary depth, 전체 CSS visibility/layout semantics 또는 WPT 적합성을 이 변경으로 보장하지 않는다.
