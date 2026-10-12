# C11 Grid 모듈 분리 구현 검토 · 2026-10-12

**범위:** 기존 Grid 입력 검증·Taffy 변환·custom-tree dispatch 코드를 작은 책임 단위로 옮긴 변경만 검토한다. Grid 동작 범위와 앱 런타임의 기능 검토는 [layout-core](./c11-layout-core-implementation-review-2026-10-12.md), [Stylo adapter](./c11-style-grid-projection-implementation-review-2026-10-12.md), [모바일 runtime](./c11-grid-mobile-runtime-implementation-review-2026-10-12.md)의 별도 검토를 따른다.

| 관점 | 실패 가능성 | 확인 결과 |
| --- | --- | --- |
| 1. Rust module 경로 | `grid_tests.rs`의 경로 속성이 실제 부모 모듈과 다를 수 있다 | `grid.rs`의 `#[path = "grid_tests.rs"] mod tests`로 `grid::tests` 아래에 연결되고 빌드된다. |
| 2. 중첩 테스트 wrapper | 파일을 옮기며 테스트가 이중 중첩되거나 모듈 밖으로 빠질 수 있다 | 테스트 함수가 부모 Grid 모듈의 하위 테스트 모듈에 한 번만 포함된다. |
| 3. 테스트 전용 컴파일 | 테스트 코드가 일반 build에 들어가거나 테스트 import가 일반 코드에 새어 나올 수 있다 | 테스트 모듈 선언에 `#[cfg(test)]`가 남아 있다. |
| 4. 부모 이름 접근 | `super::*`가 기존 Grid 타입·상수를 더 이상 가져오지 못할 수 있다 | `cargo test --locked -p spinon-layout` 99/99 통과로 이름 해석을 확인했다. |
| 5. 누락 테스트 | 분리 중 기존 Grid 회귀가 잘려 나갈 수 있다 | 전체 suite의 Rust 총계가 618 통과·4 ignored이며, Grid 전용 layout 테스트 99개가 모두 실행됐다. |
| 6. 중복 테스트 | 복사 과정에서 동일 테스트가 두 번 등록될 수 있다 | 테스트 이름과 결과를 확인했고 같은 항목의 중복 등록은 없다. |
| 7. private 경계 | trait 구현 이동이 외부에 불필요한 API를 노출할 수 있다 | `CalcLayoutTree`와 Grid helper는 crate 내부 모듈에 머문다. |
| 8. trait impl 누락 | `LayoutGridContainer` 구현이 모듈 이동 중 제거될 수 있다 | `calc_tree/grid.rs`에서 구현되고 Grid test가 통과한다. |
| 9. trait impl 중복 | 기존 구현과 새 구현이 동시에 컴파일될 수 있다 | `calc_tree.rs`에는 중복 구현이 없고 workspace가 통과한다. |
| 10. Grid dispatch | `compute_grid_layout` 호출이나 Block/Flex 분기가 이동 중 달라질 수 있다 | dispatch는 기존 `calc_tree.rs`에 남고 Grid 분기 테스트가 통과한다. |
| 11. style 변환 호출 | Grid 변환 helper 이동으로 container/item 입력을 빠뜨릴 수 있다 | `grid_container`와 정규화된 placement를 한 변환 경로에 전달한다. |
| 12. 변환 의미 변경 | helper 이동 중 auto-flow·line·track 변환이 바뀔 수 있다 | 변환 구현을 `taffy_style/grid.rs`로 옮기고 profile fixture가 통과한다. |
| 13. 오류 전파 | helper가 구체 오류 대신 기본값이나 panic을 반환할 수 있다 | 기존 `Result`와 `LayoutError` 경로를 유지한다. |
| 14. 배정 크기 실패 | `try_reserve` 오류 처리가 추출 과정에서 누락될 수 있다 | track 변환의 fallible reserve 및 오류 반환을 유지한다. |
| 15. 부분 프레임 | 변환 실패가 partial layout 결과를 노출할 수 있다 | layout-core 전체 회귀에서 기존 검증·계산 경계가 통과한다. |
| 16. `CompactTaffyStyle` 연결 | 모듈 추출이 스타일 크기 최적화와 충돌할 수 있다 | core tree·Taffy trait가 모두 compact projection을 사용하며 Clippy가 통과한다. |
| 17. Debug depth 과장 | 테스트 모듈 분리 뒤 Debug suite가 1,024-depth를 실행했다고 오인할 수 있다 | 현재 Debug test는 의도적으로 ignored이며, run-08은 Release depth 1/1을 별도로 기록한다. |
| 18. mobile 증거 과장 | host suite를 모바일 앱 검증으로 해석할 수 있다 | run-08은 앱 행렬을 다시 실행하지 않았다고 계획·상태·Tailnet 페이지에 명시했다. |
| 19. 파일 책임·크기 | 책임이 다시 한 파일에 집중되거나 작은 불필요 파일이 늘 수 있다 | `grid.rs` 191, `grid_tests.rs` 313, `calc_tree.rs` 491, `calc_tree/grid.rs` 16, `taffy_style.rs` 484, `taffy_style/grid.rs` 62줄이다. |
| 20. 통합 회귀 | package 단위만 통과하고 workspace 의존 경로가 깨질 수 있다 | `mise exec -- bun run test`, locked Clippy, rustfmt, `git diff --check`를 현재 소스에서 다시 통과했다. [run-08 전체 근거](./c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-08/README.md). |

## 남은 경계

run-08은 저장소 host suite만 실행했다. Android 실기기·iOS Simulator 앱 행렬, GPU surface, WPT runner, 현재 Debug worker의 1,024-depth 실행을 새로 검증한 결과가 아니다. Debug depth test를 제외하고 Release에서 실행하는 이유와 결과는 [현재 소스 근거](./c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-08/README.md)에 구분했다.
