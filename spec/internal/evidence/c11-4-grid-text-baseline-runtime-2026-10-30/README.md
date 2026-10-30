# C11.4 direct-text first/last baseline simulator evidence

**판정:** 고정 Ahem Grid fixture의 first-baseline 회귀와 multiline last-baseline 다섯 scene을 기존 opt-in runtime 경로에서 확인했다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각 9/9 scene이 통과했다. 이는 제한 fixture 증거이며 C11.4 또는 C11 완료 판정이 아니다.

## 입력과 구현 경계

- 고정 CSS fixture와 독립 규범/Chrome 기대값은 [사전 비교](../c11-4-grid-text-last-baseline-precomparison-2026-10-30/README.md)에 기록했다. 새 last-baseline profile은 두 direct Ahem text item, `25px 40px` explicit columns, 한 auto row, `horizontal-tb`만 허용한다.
- 장면은 first-baseline 회귀 4개와 last-baseline/controls 5개다. `align-items` 상속, 양 item의 `align-self`, DOM source order 역전, one-line first control, multiline start control을 포함한다.
- Rust adapter와 vendored Taffy의 변경은 fixture feature에서만 연결한다. 일반 CSS projection은 first/last baseline을 계속 거부한다. 새 public API나 내부 계약 버전 변경은 없다.
- Build: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 bun run build:android` 및 `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 bun run build:ios-sim`가 각각 성공했다.

## Android API 37 ARM64 AVD

- 실행 manifest: [simulator.json](./android-avd/c114-text-baseline-mvu9nh3u-ddfba41b/simulator.json) · [9 scene 결과](./android-avd/c114-text-baseline-mvu9nh3u-ddfba41b/result.json).
- `emulator-5554`, QEMU API 37, `arm64-v8a`, `sdk_gphone16k_arm64`, 1080×2400 / density 420.
- 설치 APK와 로컬 APK의 SHA-256이 모두 `bc94e85031a352afdaad2727a55df45ca3bfce8dc36e8fb7f7918dd2c5c69ca6`이다. renderer log는 ANGLE/Vulkan/SwiftShader를 나타낸다.
- 각 scene에서 3 box frame, Columns/Rows track trace, DOM node frame과 3개 PixelCopy sample을 확인했다. PixelCopy 표본은 총 27/27 RGBA exact다.
- 대표 화면: [multiline last baseline](./android-avd/c114-text-baseline-mvu9nh3u-ddfba41b/grid-text-last-baseline-align-items-multiline.png) · [first-baseline 회귀](./android-avd/c114-text-baseline-mvu9nh3u-ddfba41b/grid-text-first-baseline-align-items.png).

## iOS Simulator

- 실행 manifest: [simulator.json](./ios-simulator/c114-text-baseline-mvu9pccg-f329c361/simulator.json) · [9 scene 결과](./ios-simulator/c114-text-baseline-mvu9pccg-f329c361/result.json).
- iPhone 17 Pro (`iPhone18,1`), iOS 26.2, 960×720 drawable.
- 빌드 executable과 설치 executable SHA-256이 모두 `31f2fd1e8224fc64b72efb41cb0dc91edbf329bee8d41cc4d09fd518ca2e7243`이다.
- 각 scene에서 3 box frame, Columns/Rows track trace, DOM node frame과 3개 Metal drawable sample을 확인했다. drawable 표본은 총 27/27 RGBA exact이며 compositor 제출 전 결과다.
- 대표 화면: [multiline last baseline](./ios-simulator/c114-text-baseline-mvu9pccg-f329c361/grid-text-last-baseline-align-items-multiline.png) · [first-baseline 회귀](./ios-simulator/c114-text-baseline-mvu9pccg-f329c361/grid-text-first-baseline-align-items.png).

## 실패와 재실행

첫 Android 실행은 [초기 run](./android-avd/c114-text-baseline-mvu9ml7d-9b75513a/)에서 source-order 역전 scene에 실패했다. frame geometry 자체는 기대값과 일치했지만 native 검증기가 node 순서가 바뀐 뒤에도 frame index 1/2를 이전 node 문자열에 붙여 비교했다. Android와 Swift 검증기의 index/node 매핑을 고친 후 두 플랫폼 전체 9 scene을 다시 실행했으며 위 최종 결과가 통과했다. 초기 실패 파일은 진단 근거로 보존했다.

## 확인 범위와 한계

- screenshots와 surface samples는 색이 칠해진 layout box와 gap을 검증한다. 이 runtime은 `runtime_text_count=0`, `glyph_batch_draw_count=0`을 보고하므로 글리프 raster 또는 화면에서 보이는 글자 모양을 검증하지 않는다. baseline 수치는 runtime text layout trace의 line metrics에서 검사했다.
- Android 결과는 emulator의 SwiftShader software renderer이며 Android hardware GPU나 실기기 결과가 아니다. iOS 표본은 Metal drawable에서 compositor 제출 전에 읽었다.
- 일반 author CSS admission, WPT conformance, 임의 font·폭, rich inline text, multi-row span, vertical writing mode, Flex last-baseline, CSS border paint, hardware GPU 및 C11.4 전체는 이 실행의 범위가 아니다.
- Capture script의 `finally` 경로는 Android 앱을 force-stop하고 iOS target 앱을 종료한다. 실행 후 target 앱 프로세스는 남아 있지 않았다. Agent Browser는 이 runtime capture에서 실행하지 않았다.

## Focused verification

최종 source에서 다시 실행:

- `cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test c15_text_layout_tests::grid_text_baseline` — 11 passed.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test runtime_layout_tests::c11_grid_tests::grid_text_baseline` — 2 passed.
- `cargo test --locked -p spinon-style-to-layout --features c11-4-grid-track-sizing-fixture-test c11_4_track_sizing_areas_tests::first_baseline_admission` — 2 passed; ordinary CSS baseline admission remains closed.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test runtime_gpu::fixtures::c11_4::grid_text_baseline` — 1 passed.
- `bun test tools/css-reference/fixture-module-boundary.test.mjs` — 23 passed.
- `bun test tools/css-reference/c11-ios-build-flags.test.mjs` — 13 passed.
- Feature-enabled Clippy for `spinon-layout`, `spinon-runtime`, and `spinon-style-to-layout` — passed with `-D warnings`.
- `cargo fmt --all -- --check`, `git diff --check`, and capture-script `node --check` — passed.
- Debug `spinon-ffi` product and fixture-test static archives exclude/include the host symbol and all nine scene IDs as expected; exact hashes and probe scope are in [artifact boundary](./artifact-boundary/README.md).

These checks cover the C11.4 candidate's focused code and local debug archive. They do not establish full workspace CI or built APK/IPA exclusion.
