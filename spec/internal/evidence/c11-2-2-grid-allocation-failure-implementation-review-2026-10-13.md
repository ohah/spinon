# C11.2.2 Grid Vec 예약 실패 구현 공격 검토

이 검토는 [allocator failure 주입 계획](./c11-2-2-grid-allocation-failure-plan-review-2026-10-13.md)과 현재 Rust 변경, 실제 실패 로그, Rust workspace 회귀 및 Android/iOS 현재 소스 실행을 대조한다. 전체 OOM·Spinon WPT·전체 Grid 지원을 검토 대상으로 가장하지 않는다.

| # | 공격 관점 | 결과 |
|---:|---|---|
| 1 | custom allocator가 test binary에서 실제 global allocator로 등록되지 않았다 | `spinon-layout` unit test binary에만 `#[global_allocator]`를 등록하고 targeted test를 실행했다. |
| 2 | `GridAllocationFailure`를 직접 반환해 allocator 경로를 건너뛴다 | allocator에서 null allocation을 돌려 `Vec::try_reserve[_exact]` 자체가 오류를 반환했으며, 소비 관측값을 각 경우 확인했다. |
| 3 | capacity가 남아 할당 요청이 발생하지 않는데 실패 처리만 주장한다 | 대상 Vec는 매 계산에서 새로 만든 빈 Vec다. 다섯 site의 실제 allocator 소비를 각각 단언한다. |
| 4 | item list보다 앞선 `collect` 할당을 Grid failpoint로 오인한다 | site는 item Vec 예약 helper 직전에서만 arm되므로 `visible_children` 준비 할당은 주입 범위 밖이다. |
| 5 | 한 예약의 실패 신호가 다른 예약에서 소비된다 | `GridReservationSite`를 별도 지정해 다섯 개 각 지점에서 실제 실패를 확인했다. |
| 6 | visible item 목록 reserve가 빠졌거나 잘못된 fixture를 쓴다 | visible child 1개 이상인 입력에서 node 1의 item-list reservation failure를 확인했다. |
| 7 | row-locked cursor reserve를 실행하지 않는다 | explicit row가 있는 입력에서 node 1의 cursor reserve가 실패했다. |
| 8 | implicit row 확장 없이 expanded track 실패를 주장한다 | 두 번째 auto item이 양의 implicit row를 만들며, node 1의 `grid-template-rows` reserve가 실패했다. |
| 9 | occupancy 행 확장과 interval insertion을 같은 경로로 뭉뚱그린다 | definite item fixture로 두 site를 각각 실패시켰고 node 2의 오류를 확인했다. |
| 10 | `alloc`만 막고 Vec가 사용하는 다른 allocator entry point를 놓친다 | test allocator는 `alloc`, `alloc_zeroed`, `realloc` 모두를 fail-once 검사하고 비대상 호출은 `System`으로 위임한다. |
| 11 | error property가 호출 위치와 다르다 | 다섯 site 각각 `grid-placement-occupancy` 또는 `grid-template-rows`를 정확히 검사한다. |
| 12 | 오류가 container/item의 잘못된 NodeId에 붙는다 | container reservation의 node 1과 occupancy item의 node 2를 비교했다. |
| 13 | 실패한 계산이 일부 frame을 반환한다 | 각 실패 결과를 `Err(GridAllocationFailure)`로 검사해 `LayoutOutput` 성공을 배제했다. |
| 14 | allocator failpoint가 두 번째 예약까지 연속 실패시킨다 | allocator failure와 site 선택은 one-shot이다. 각 case 뒤 guard가 상태를 초기화한다. |
| 15 | 오류 뒤 같은 엔진 계산이 성공해도 다른 worker에서 실행된다 | 모든 fail/recovery case가 한 spawned worker thread의 동일 engine instance에서 순차 실행된다. |
| 16 | recovery가 입력·revision을 바꾸어 poisoned worker를 감춘다 | 실패와 성공 후속 계산은 동일한 fixture와 revision을 사용하고 frame을 확인한다. |
| 17 | panic이 다음 병렬 테스트에 TLS 주입 상태를 남긴다 | RAII guard의 `Drop`이 reservation TLS와 allocator TLS를 reset한다. 다른 test thread의 allocator state는 TLS로 분리된다. |
| 18 | test hook이 production code·ABI에 남거나 경로 비용을 키운다 | failpoint과 allocator만 `cfg(test)`다. Grid 전용 helper 추출은 production `try_reserve` 동작을 그대로 호출한다. |
| 19 | Grid adapter 입력 구조체 변경이 compile/lint/test 회귀를 만든다 | 인자 수 Clippy 경고를 Grid 전용 `GridTaffyStyleInput`으로 정리했다. layout Clippy `-D warnings`, workspace test 640 passed·4 ignored가 통과했다. 초기 borrow-check 오류도 길이 계산을 borrow 전에 분리해 고쳤다. |
| 20 | desktop fault test를 모바일 표면 검증 또는 WPT 적합성으로 과장한다 | 변경 후 Android API 37 AVD와 iOS Simulator 각 16 scene·69 node, 11 성공·5 기대 거부, geometry 0, 표면 51 sample exact를 재실행했다. AVD는 ANGLE/SwiftShader이며 iOS는 compositor 전 readback이고 allocator 주입은 Rust unit worker 결과로 한정한다. |

## 판정과 열린 경계

다섯 명시적 Vec reservation 오류의 오류 귀속·출력 원자성·동일 Rust worker thread 회복을 확인했다. `BTreeMap`·`BTreeSet`·기타 infallible allocation의 전역 OOM 안전성, 플랫폼 OS allocator fault injection, Spinon WPT 적합성, 전체 CSS Grid, iOS compositor 이후 출력은 확인하지 않았다. 종합 실행 근거는 [allocator failure 결과](./c11-2-2-grid-allocation-failure-2026-10-13/README.md)에 있다.
