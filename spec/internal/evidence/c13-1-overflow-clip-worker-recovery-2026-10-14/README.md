# C13.1 clip ancestry 실패 뒤 runtime worker 복구 · 2026-10-14

## 구현 전 기준과 판정

기존 snapshot builder 검사는 `FaultInjectingReserve`를 같은 builder에 다시 전달해 snapshot 생성이 재시도 가능한지만 확인한다. CSS runtime worker의 요청 게시, 오류 상태, worker 생존 및 후속 요청 처리는 아직 관찰하지 않았다. 이번 검증은 아래 세 입력 조건과 판정 기준을 구현 전에 고정한다.

- 입력은 세 element의 단일 HostDocument tree다. root는 고정 크기 relative block, child는 고정 크기 `overflow:clip` ancestor, grandchild는 positioned box다. 실제 C13.1 profile의 cascade·Taffy layout·render snapshot 조립 경로를 사용한다.
- 먼저 같은 runtime worker에서 fault-free 요청을 성공시켜 이전 completed snapshot을 만든다. 다음 environment revision 요청의 세 번째 `overflow clip ancestry` 성장 할당 시점에만 결정적으로 `AllocationFailed`를 반환한다. 이 occurrence는 root, clip ancestor 다음의 positioned target box ancestry를 조립하는 시점이다.
- 실패 요청은 cascade `Ready`, layout `Failed`, `render_allocation_failed`, property `overflow clip ancestry`를 반환하고 `completed=null`로 이전 성공 snapshot을 제거해야 한다. 부분 frame/box 배열을 게시하거나 coordinator가 이후 environment revision을 접수하지 못하거나 worker가 종료되면 실패다.
- 같은 coordinator의 다음 environment revision 요청은 새 worker를 만들지 않고 같은 CSS runtime worker에서 완료되어야 한다. layout은 `Ready`이고 render snapshot은 완전해야 하며, 동일 revision의 fault-free 직접 계산과 snapshot 전체가 같아야 한다.
- 이 검증은 allocator 전역 OOM, Android/iOS platform allocator, paint surface, 실기기를 시험하지 않는다. 예약 한 지점만 feature-gated deterministic injector로 대체한다.

## 실행 결과

`mise exec -- cargo test --locked -p spinon-runtime --features c13-1-overflow-clip-worker-test clip_ancestry_allocation_failure_does_not_kill_runtime_worker -- --nocapture`로 단일 실패 경로를 실행했다. 먼저 environment revision 0의 전체 snapshot을 정상 게시한 뒤 revision 1에서 failure injection, revision 2에서 복구를 수행했다. [전용 runtime feature 전체 테스트](worker-feature-cargo-test.log)도 통과했다.

- revision 0 baseline은 snapshot을 게시했다. revision 1은 runtime cascade `Ready`, layout `Failed`, error code `render_allocation_failed`, property `overflow clip ancestry`, `completed=null`로 끝났고 이전 baseline key도 유지되지 않았다. worker는 계속 사용 가능했다.
- environment revision 2 요청이 기존 worker thread에서 `Ready`로 완료됐다. 세 box를 가진 render snapshot 전체가 같은 revision의 fault-free 직접 계산 결과와 `PartialEq`로 같았다.
- 기본 workspace test는 성공했고 `spinon-runtime` unit suite는 139 passed·3 ignored였다. 전용 feature suite는 140 passed·3 ignored였고, 실패 복구 test 1개도 포함한다. [workspace test](workspace-cargo-test.log)와 [전용 runtime feature test](worker-feature-cargo-test.log)에 원본 출력을 보존했다.
- [workspace Clippy](workspace-clippy.log), [전용 feature Clippy](worker-feature-clippy.log), [rustfmt](rustfmt.log), [diff check](git-diff-check.log)가 통과했다. 기본 구성과 `c13-1-overflow-clip-worker-test` opt-in 구성을 각각 검증했다.
- injector, helper API와 runtime test module은 `c13-1-overflow-clip-worker-test` opt-in feature에 격리했다. 기본 feature workspace test/clippy와 feature-on runtime test/clippy를 각각 통과했다. feature는 FFI나 Android 앱 route에 연결되지 않는다.

결과는 결정적으로 지정한 Rust `Vec::try_reserve` 경로만 다룬다. 이 검증은 OS allocator 전역 OOM, 실제 memory pressure, Android/iOS 앱 worker·surface 복구를 보장하지 않는다. C13.1의 전체 matrix, clip/float formatting 대조, fail-closed radius/transform, iOS Simulator, WPT 분류는 계속 남아 있다.
