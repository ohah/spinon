# 빌드·표적 검사 기록

2026-10-22 source base `8c7c412708ec4cb8bb78faf9366bb96c529c8938`에서 실행했다.

- `mise exec -- cargo fmt --all -- --check` — 통과.
- `mise exec -- cargo test --locked -p spinon-style-to-layout --features c11-4-grid-track-sizing-fixture-test cyclic_minmax_calc -- --nocapture` — 3 passed.
- `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test exposes_fixed_cyclic_calc_track_scenes_by_function -- --nocapture` — 1 passed.
- `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test minimum_calc -- --nocapture` — 2 passed.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs`와 `git diff --check` — 통과.
- Android opt-in build `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android` — 성공. Android Gradle Plugin 8.13.2의 compile SDK 37.2 사전 검증 범위 경고가 있었지만 APK가 생성됐다.
- iOS opt-in build `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 mise exec -- bun run build:ios-sim` — Xcode `** BUILD SUCCEEDED **`. V8 archive debug-map 경고와 AppIntents metadata 생략 경고가 있었으며 Simulator build는 완료됐다.
- Android API 37 AVD final capture — 4/4 scene, PixelCopy 28/28 exact.
- iPhone 17 Pro / iOS 26.2 Simulator final capture — 4/4 scene, Metal drawable 28/28 exact.

이 결과에는 workspace `cargo test` 전체, workspace 외 타깃 또는 Android 실기기 테스트가 포함되지 않는다.
