# C13.1 worker recovery PR 변경 관점 검토 30개 · 2026-10-14

이 검토는 PR #158에 추가한 worker recovery diff의 독립 검토다. 기존 30개 PR 검토를 대체하지 않고 새 코드·상태 문서 변경의 경계를 확인한다. 근거는 [worker recovery 실행](c13-1-overflow-clip-worker-recovery-2026-10-14/README.md), [기본 workspace log](c13-1-overflow-clip-worker-recovery-2026-10-14/workspace-cargo-test.log), [feature-on log](c13-1-overflow-clip-worker-recovery-2026-10-14/worker-feature-cargo-test.log)다.

| # | PR 변경 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 새 feature의 기본 활성화 | Cargo default feature set에는 포함되지 않으며 기본 workspace test가 통과했다. |
| 2 | runtime feature forwarding 대상 | `spinon-style-to-render/c13-1-overflow-clip-worker-test`만 전달한다. 다른 workspace crate feature는 바뀌지 않았다. |
| 3 | fault helper의 crate export | 재현 helper re-export는 같은 feature로 cfg 처리했다. 기본 빌드 public surface에 노출하지 않는다. |
| 4 | fault target 설정 오타 | public helper는 arbitrary stage 문자열을 받지 않고 ancestry stage 세 번째 occurrence로 고정했다. |
| 5 | fallback production builder 선택 | 일반 `compute_request_with_profile`은 `build_runtime_render_snapshot`을 builder로 전달한다. |
| 6 | builder abstraction의 제품 경로 비용 | dyn trait object 대신 generic `FnMut` 경로를 써서 기본 builder는 monomorphize 가능하다. |
| 7 | 비-paint layout의 snapshot 호출 | `runtime_paint_enabled=false`면 builder를 호출하지 않는 기존 분기가 유지된다. |
| 8 | 빈 document 처리 | root_count 0 분기는 기존 empty snapshot helper를 쓰며 clip injector에 진입하지 않는다. |
| 9 | 다중 root 처리 | 다중 root 검증은 builder invocation 전에 실패하므로 partial root scene을 만들지 않는다. |
| 10 | cascade validation 순서 | unsupported CSS/profile 진단과 layout 실패가 먼저 반환되며 test hook은 실제 scene assembly에만 적용된다. |
| 11 | `StyleRenderError` mapping | `AllocationFailed`는 `render_allocation_failed`와 stage property로 변환된다. |
| 12 | `RuntimeCalculation` 부분 상태 | layout 오류는 cascade 완료 데이터와 분리되어 publish되고 render completed 값은 설정되지 않는다. |
| 13 | stale result guard | worker의 current requested key 검사와 `request_latest` pending 전환을 바꾸지 않았다. |
| 14 | 이전 layout completion 제거 | revision 1 오류 때 revision 0 snapshot이 `completed`에 남지 않는 것을 test가 검사한다. |
| 15 | 실패 뒤 다음 요청 queue | revision 2 environment 변경이 accepted되고 완료되는 것을 같은 coordinator에서 확인했다. |
| 16 | 요청 coalescing의 가짜 성공 | 각 revision 완료를 기다린 뒤 다음 요청을 넣어 pending supersede를 피한다. |
| 17 | worker identity | 동일 thread ID 비교와 두 번째 요청 성공을 함께 사용한다. 새 coordinator 생성은 없다. |
| 18 | worker availability flag의 거짓 양성 | flag 확인 외에도 후속 요청의 실제 계산 완료를 요구한다. |
| 19 | injected computation의 일회성 | atomic swap으로 실패를 단 한 번 소비한다. 이후 요청은 production C13.1 builder 경로다. |
| 20 | 직전 baseline의 가치 | 먼저 성공 revision을 만들고 나서 failure를 주입해 stale completion 경계를 시험한다. |
| 21 | oracle request key | recovery 직접 계산은 실제 revision 2와 동일한 document generation, viewport, environment revision을 사용한다. |
| 22 | snapshot 깊은 비교 | box count뿐 아니라 full `RuntimeRenderSnapshot` equality를 확인한다. |
| 23 | clip ancestry 출력 누락 | 세 box 전체 equality와 box count 3으로 root·clip·target 누락을 탐지한다. |
| 24 | profile bleed | 테스트 computer에서 C13.1 profile을 지정하고 다른 runtime constructor는 수정하지 않았다. |
| 25 | test module 책임 경계 | 새 runtime test는 별도 `c13_1_overflow_clip_worker_tests.rs`에 두고 기존 424줄 파일을 유지했다. |
| 26 | 앱 fixture artifact 영향 | 새 feature를 Android Cargo/JNI/BuildConfig route에 연결하지 않았다. |
| 27 | FFI ABI 영향 | C header와 exported FFI symbol 변경이 없다. 이 helper는 Rust test feature 경로다. |
| 28 | physical-device 가정 | 이번 host worker 실행은 ADB 대상에 의존하지 않는다. 앞선 API 37 ARM64 AVD 결과와 합산하지 않는다. |
| 29 | 공식 진행 상태와 계획 정합 | worker recovery만 완료로 갱신하고 C13/C13.1 전체 완료 체크는 바꾸지 않았다. |
| 30 | 잔여 gate 누락 | 전체 oracle, paint/hit breadth, float/flow-root, radius/transform, iOS Simulator, WPT를 남은 범위로 계속 명시한다. |

이 30개는 새 worker-recovery diff에 대한 PR 관점 검토다. 실제 전역 memory exhaustion과 Android/iOS 앱 worker에서의 allocator 복구를 통과했다고 해석하지 않는다.
