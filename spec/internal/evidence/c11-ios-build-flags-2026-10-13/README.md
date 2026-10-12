# C11 iOS Grid 표면 픽셀 빌드 flag 검증 · 2026-10-13

이 증거는 내부 Grid 표면 표본 검증 flag가 기본 비활성화되고, 값과 선행 feature를 검증하며, simulator/native 빌드 경로를 통과하는지 기록한다.

## 검사 결과

- 새 iOS build flag test: 6/6 통과 — [log](./build-flags-test.log)
- iOS simulator runtime matrix: 10 scene, 성공 6/6·기대 거부 4/4, 21 node, geometry 오차 0 CSS px, Metal 표본 20/20 exact — [실행 요약](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T191045812Z-79554a35/README.md)
- 실제 build log는 simulator의 `c04_runtime_gpu=1`, `c11_grid_metal_surface_pixels=1`과 native Cargo feature·C++ define을 모두 기록한다.
- C11/C12.4 readback feature 공존: 39 passed·2 ignored·0 failed — [log](./ffi-feature-coactivation.log)
- 저장소 전체: Bun example 2/2, CSS reference 348/348, benchmark/tool tests 29/29, Rust workspace 625 passed·4 ignored·0 failed, Release depth 1/1 — [log](./full-test.log)
- `cargo fmt --all -- --check`: 통과 — [log](./rustfmt.log)
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과 — [log](./clippy.log)

첫 전체 실행에서 기존 fake ADB test가 한 차례 실패 표시 후 종료되지 않은 기록은 [별도 log](./first-full-test-attempt.log)에 보존했다. 해당 benchmark test group 단독 실행과 후속 전체 실행은 모두 통과했고, 최초 실패 원인은 확정하지 않았다.

새 20개 관점 구현 검토: [검토 표](../c11-ios-build-flags-implementation-review-2026-10-13.md). CSS Grid 지원 범위와 미구현 항목은 [공식 상태 대장](../../STATUS.md)을 따른다.
