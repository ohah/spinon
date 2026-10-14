# C13.1 runtime worker recovery 구현 실패 관점 검토 20개 · 2026-10-14

검토 대상은 clip ancestry reserve 실패를 실제 `RuntimeUaCascadeCoordinator` worker에서 처리한 뒤 같은 worker로 다음 revision을 완료하는 변경이다. 판정 기준은 [사전 등록 기록](c13-1-overflow-clip-worker-recovery-2026-10-14/README.md), 실행 결과는 [feature-on runtime test](c13-1-overflow-clip-worker-recovery-2026-10-14/worker-feature-cargo-test.log)와 [workspace test](c13-1-overflow-clip-worker-recovery-2026-10-14/workspace-cargo-test.log)에 둔다.

| # | 실패 관점 | 적대 조건과 확인 결과 |
| --- | --- | --- |
| 1 | Host tree가 clip descendant를 포함하지 않음 | 세 노드를 root→clip→positioned target로 연결했다. worker test에서 C13.1 profile 계산이 성공 기준에 도달한다. |
| 2 | legacy 또는 stacking profile만 실행 | 테스트 computer가 `BlockFlexOverflowClip` profile을 고정한다. 오류 stage는 실제 C13.1 snapshot builder에서 온다. |
| 3 | root lineage에서 조기 실패 | 세 번째 `overflow clip ancestry` 성장 예약에만 실패시킨다. root와 clip box baseline이 이미 정상 생성된 revision을 먼저 게시한다. |
| 4 | stage occurrence가 전체 reserve 호출 수와 혼동 | selector는 실제 capacity 성장이 필요한 시도만 세고 stage 이름별 occurrence를 비교한다. occurrence 3을 고정 helper로 한정했다. |
| 5 | `Vec`가 이미 가진 capacity에 실패를 잘못 주입 | `FaultInjectingReserve`는 추가 원소가 spare capacity를 넘을 때만 시도를 기록한다. 기존 no-growth 경계 테스트도 유지한다. |
| 6 | 다른 stage가 실패했다고 오인 | 반환 error의 property가 `overflow clip ancestry`와 정확히 일치한다. 다른 stage라면 worker 상태 검증에서 실패한다. |
| 7 | allocation error가 panic으로 worker를 종료 | 결과는 `render_allocation_failed` layout 오류로 게시되고 `worker_available`이 true로 유지된다. |
| 8 | cascade 성공까지 실패로 덮음 | 실패 revision에서 cascade는 `Ready`이며 completed cascade 결과가 보존된다. layout 오류와 cascade 오류 상태를 구분한다. |
| 9 | 일부 frame/box가 성공 snapshot으로 노출 | 실패 결과의 `layout_completed`는 `None`이다. partial render snapshot이나 frame 배열을 게시하지 않는다. |
| 10 | 직전 유효 snapshot이 실패 revision의 stale 결과로 남음 | 먼저 revision 0 baseline을 게시한 뒤 revision 1에 실패시킨다. 실패 시 completed가 비고 requested key는 baseline key와 다르다. |
| 11 | 실패 뒤 coordinator가 모든 요청을 막음 | 실패 후 `set_environment`가 revision 2를 발급한다. coordinator의 worker availability 확인도 true다. |
| 12 | 다른 worker가 성공을 대신 처리 | coordinator의 `JoinHandle` thread ID를 시작 전·복구 후 비교해 동일하다. |
| 13 | 주입이 매 요청 반복되어 복구하지 못함 | test 전용 `AtomicBool`을 실패 처리 때 false로 소비한다. revision 2는 기본 fault-free C13.1 builder를 사용한다. |
| 14 | recovery snapshot에 target descendant 누락 | 복구 결과는 정확히 세 render box를 포함한다. |
| 15 | 오류 후 clip metadata가 부분 손상 | 복구 snapshot 전체가 같은 key·viewport의 fault-free 직접 계산 결과와 `PartialEq`로 같다. |
| 16 | 이전 environment의 결과를 새 key로 게시 | revision 2 key와 392 CSS px viewport를 직접 계산 입력과 비교해 snapshot equality로 확인한다. |
| 17 | failure path가 Android/ADB 상태에 의존 | 이 경로는 Rust worker의 결정적 reserve injector만 실행한다. 실제 기기나 ADB를 사용하지 않았고 그 범위로 주장하지 않는다. |
| 18 | 기본 빌드에 failure injector가 유입 | helper와 모듈은 `c13-1-overflow-clip-worker-test` feature로 gated다. 기본 workspace test와 Clippy도 별도로 통과했다. |
| 19 | injector를 FFI/app에서 호출 가능 | feature forwarding은 runtime→style-to-render 테스트 경로만 추가한다. C header, JNI, app route, C13 fixture ABI 변경은 없다. |
| 20 | 제한 결과를 C13.1 완료로 과장 | 상태 대장과 내부 계약은 전체 matrix, float/flow-root, fail-closed radius/transform, iOS Simulator, WPT가 남았다고 유지한다. |

전용 feature 전체는 140 passed·3 ignored, workspace는 전체 crate test 성공이다. workspace/feature Clippy, rustfmt, diff check도 통과했다. 전역 allocator OOM과 앱 플랫폼 worker/surface 복구는 이 검토의 PASS 범위가 아니다.
