# C11.2.2 Grid Vec 예약 실패 주입 결과

## 결과

`spinon-layout` 테스트 전용 global allocator가 실제 할당 요청을 실패시켰다. 다섯 Grid 예약 지점 모두 allocator failure 관측값이 `true`였고, 정확한 `GridAllocationFailure`를 반환했다. 실패 호출은 `LayoutOutput`을 반환하지 않았으며, 같은 worker thread와 같은 `TaffyLayoutEngine` instance에서 이어서 실행한 동일 입력 계산은 정상 frame을 반환했다.

| 예약 지점 | 귀속 node | `GridAllocationFailure.property` | 실제 allocator 실패 |
|---|---:|---|---|
| 보이는 자식 배치 목록 | 1 | `grid-placement-occupancy` | 확인 |
| row-locked cursor | 1 | `grid-placement-occupancy` | 확인 |
| 확장된 행 track | 1 | `grid-template-rows` | 확인 |
| occupancy 행 목록 | 2 | `grid-placement-occupancy` | 확인 |
| 행별 interval 목록 | 2 | `grid-placement-occupancy` | 확인 |

## 검증 명령과 로그

- 집중 주입: `cargo test -p spinon-layout actual_grid_vec_reservation_failures_return_errors_and_recover_on_the_same_worker -- --nocapture` · 1/1 통과, 다섯 실패 지점별 관측 로그는 [unit-test.log](./unit-test.log)에 있다.
- layout crate 전체: 109 unit test와 기존 allocator integration test 2개 통과 · [package-test.log](./package-test.log).
- Rust 전체 workspace: 640 통과, 4 ignored, 0 실패 · [workspace-test.log](./workspace-test.log).
- `cargo clippy -p spinon-layout --all-targets -- -D warnings` 통과 · [clippy.log](./clippy.log).
- `cargo fmt --all -- --check`와 `git diff --check` 통과 · [rustfmt-check.log](./rustfmt-check.log), [diff-check.log](./diff-check.log).
- 계획의 20개 공격 관점은 [계획 검토](../c11-2-2-grid-allocation-failure-plan-review-2026-10-13.md), 구현의 20개 공격 관점은 [구현 검토](../c11-2-2-grid-allocation-failure-implementation-review-2026-10-13.md)에 기록했다.

## 변경 뒤 모바일 현재 소스 행렬

최종 C11.2.2 PR 후보 source로 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator runtime matrix를 실행했다. 각 실행은 16 scene·69 node, 성공 11·기대 거부 5, geometry 최대 오차 0 CSS px였다. Android AVD PixelCopy와 iOS Simulator Metal drawable은 각각 11 scene·51 sample에서 최대 RGBA channel error 0이다. Android 결과는 ANGLE/Vulkan/SwiftShader AVD이며 iOS 표면 readback은 compositor 제출 전이다.

- [Android AVD 실행](../c11-2-2-sparse-row-placement-android-runtime-matrix-2026-10-13/c1122-avd-mv61a1sd-32a80873/README.md)
- [iOS Simulator 실행](../c11-2-2-sparse-row-placement-ios-simulator-runtime-matrix-2026-10-13/c1122-i-mv61i5ir-95971d40/README.md)

## 한계

실패 주입은 `spinon-layout`의 다섯 명시적 `Vec::try_reserve[_exact]` 지점에 한정된다. `BTreeMap`·`BTreeSet` 등의 infallible allocation, 전체 프로세스 OOM 복구, Android/iOS 시스템 allocator에 직접 오류를 주입한 결과를 뜻하지 않는다. WPT runner 적합성, 전체 CSS Grid 지원, iOS OS compositor 이후 화면도 증명하지 않는다.

보존한 console log는 판독성과 저장소 공백 검사를 위해 줄 끝 공백만 제거했다.
