# C11.4 `fit-content-track-sizing.html` zoomed-grid runtime slice

2026-10-29 · 제한된 opt-in fixture 구현과 두 Simulator의 표면 표본 기록이다. 동작 범위는 [내부 계약 0059](../../0059-c11-grid-layout.md)에 고정했다. 이 결과는 일반 CSS `zoom`, 전체 WPT 실행, 전체 C11.4 완료를 뜻하지 않는다.

## 구현 판정

고정 WPT 입력의 root `zoom:2`, `grid-auto-rows:fit-content(40px)`, 400px child를 exact root ID `c114-fit-content-zoomed-grid`와 `c11-4-grid-track-sizing-fixture-test` 경로에 연결했다. Stylo typed values는 zoom을 적용해 root `120×600px`, Grid track limit `80px`, child `800px`로 전달한다. upstream WPT CSSOM의 `gridTemplateRows:40px`은 zoom 미적용 computed-style 값으로 별도 유지한다. 일반 C11.4 author allowlist는 `zoom`을 계속 거부한다.

## Android API 37 ARM64 AVD

- Target: `android-avd`, AVD serial `emulator-5554`, ANGLE/SwiftShader software renderer.
- Run ID: `c114-android-avd-20261029T014400643Z-28ab6d1f-0`.
- Capture directory: [`android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f`](./android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f/).
- Result: scene 1/1, root/item/child frames `120×600`, `120×80`, `120×800` CSS px, row `[80]px`, six row sizing stages, `drawPresented=true`; viewport identity is `320×800` CSS px and revisions match.
- PixelCopy: three yellow overflow samples and one black outside-root control exact (4/4). CSS sample coordinates map to pixels at scale 2.625: `(158,106)`, `(158,263)`, `(158,1602)` are `255,209,102,255`; `(329,1602)` is `0,0,0,255`.
- Screenshot: [fit-content-zoomed-grid-wpt.png](./android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f/fit-content-zoomed-grid-wpt.png), SHA-256 `298262e3f025469cd238827f2b5c5fe0b6fb871ca1b4273eed4a52b3fbe3ffe0`.
- Raw record: [`runtime-fit-content-zoomed-grid-wpt.log`](./android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f/runtime-fit-content-zoomed-grid-wpt.log), [`result.json`](./android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f/result.json), [`simulator.json`](./android-avd-c114-fit-content-zoomed-grid-android-avd-20261029T014400643Z-28ab6d1f/simulator.json).

## iOS 26.2 Simulator

- Target: iPhone 17 Pro, simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- Run ID: `c114-ios-simulator-20261029T014407699Z-670c5b73-0`.
- Capture directory: [`ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73`](./ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73/).
- Result: scene 1/1, root/item/child frames `120×600`, `120×80`, `120×800` CSS px, row `[80]px`, six row sizing stages, `drawPresented=true`; viewport identity is `320×800` CSS px and revisions match.
- Metal readback: compositor 제출 전 drawable `960×2400` at scale 3.0. Three yellow overflow samples and one black outside-root control exact (4/4): `(181,121)`, `(181,301)`, `(181,1831)` are `255,209,102,255`; `(376,1831)` is `0,0,0,255`.
- Screenshot: [fit-content-zoomed-grid-wpt.png](./ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73/fit-content-zoomed-grid-wpt.png), SHA-256 `d5aa47b94a1792f7cb98ebf98e4a5c0123c1a78752d6e2efe58fe0bd26b9c514`.
- Raw record: [`runtime-fit-content-zoomed-grid-wpt.log`](./ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73/runtime-fit-content-zoomed-grid-wpt.log), [`result.json`](./ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73/result.json), [`simulator.json`](./ios-simulator-c114-fit-content-zoomed-grid-ios-simulator-20261029T014407699Z-670c5b73/simulator.json).

두 최신 capture의 `simulator.json`은 공통 checkout HEAD `60ffaf1509902c94a11644331d82280916767a1d`, tree `4d649e1e311c5d7980a3ac108cb289e0e4cd8c8c`, tracked diff SHA-256 `5a6d2feea7c09c2e156f4fe65a7dc56ed4c4c117bb8ebef2f7c3971e9e05905d`, source snapshot SHA-256 `4532333a9548635e8e39d41d2f9601ed4a074aefa9505c476002e7296bbc3c25`를 기록한다.

## 빌드·검사

- Android app build: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android` — 통과.
- iOS Simulator app build: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 bun run build:ios-sim` — 통과.
- Style regression: `cargo test --locked -p spinon-style --features c11-4-grid-track-sizing-fixture-test zoom_is_rejected_by_default_and_fixture_route_preserves_scaled_typed_lengths -- --nocapture` — 1/1.
- Runtime regressions: `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c11_4_fit_content_zoomed_grid -- --nocapture` — 4/4. Includes alternate root zoom `1`, `1.5`, `3`, negative zoom, item/child zoom-owner changes, and stale style/document/tree/environment revisions.
- FFI fixture allowlist: `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_only_the_zoomed_fit_content_grid_scene -- --nocapture` — 1/1.
- Fixture module/capture boundary: `node --test tools/css-reference/fixture-module-boundary.test.mjs` — 20/20. The first rerun found one stale expected scene count (`540`); changed it to `542` and reran the full suite successfully.
- Formatting and whitespace: `cargo fmt --all -- --check` and `git diff --check` — 통과.
- Clippy: `cargo clippy --locked -p spinon-ffi --all-targets --features c04-runtime-gpu-fixture-test -- -D warnings` — 통과.
- FFI static archives: `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu` produced a product archive with no `fit-content-zoomed-grid-wpt`, `c114-fit-content-zoomed-grid`, `fit_content_zoomed_grid` string or matching symbol. `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` is the positive control: it contains the source payload and `spinon_ffi::runtime_gpu::fixtures::c11_4::fit_content_zoomed_grid::runtime_source` symbol. `strings` and Homebrew `llvm-nm` 22.1.8 were used to match Rust 1.96 / LLVM 22 objects. This static archive check does not prove APK/IPA exclusion.
- Final product builds: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android` — `BUILD SUCCESSFUL`; `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 bun run build:ios-sim` — `BUILD SUCCEEDED`.

## 보정된 실패 경로

초기 Android PixelCopy와 iOS sample validator가 zoom child의 표본 y=610을 기본 viewport 높이 240으로 거부했다. 캡처 scene viewport를 `320×800`으로 고정한 뒤 Android의 renderer presentation identity와 iOS의 Metal request-size 조건에서 각각 남아 있던 `240` 가정도 `800`으로 바꿨다. Android 공통 C11.4 parser의 inline `zoom` 거부는 exact fixture root 전용 route를 추가해 해소했다. diff 검토에서 opt-in runtime view constructor의 임의 base URL도 제한해 Chrome-only URL을 고정했다. 최종 Android/iOS capture만 위의 pass로 계산했다. 이전 실패 시도 로그와 screenshots는 이 폴더의 시각별 run directory에 보존한다.

## 한계

Android 결과는 software AVD이며 iOS 표본은 compositor 제출 전 Metal drawable에서 읽었다. 실기기, hardware GPU, compositor 최종 출력, zoom ancestor 조합·zoomed auto-repeat·percentage/calc·text intrinsic sizing, 일반 CSSOM zoom, 전체 WPT runner, C11.4 전체는 검증하지 않았다. 사전 계획 리뷰 20개, 구현/runtime 리뷰 20개, PR diff 리뷰 30개는 완료했다. 커밋·PR·merge는 아직 남아 있다. C11.4 checkbox와 공식 `72/213`은 유지한다.
