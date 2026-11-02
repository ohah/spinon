# C11.1 percentage implicit columns runtime evidence

This evidence runs the precomparison fixture through Spinon's parsed Root HTML route on Android and iOS simulators. It is a bounded runtime check for definite-width percentage `grid-auto-columns`; it does not complete C11.1 or the C11 parent.

## Input and reference

- Fixture: [`root-html-grid-column-flow-percent-auto-columns-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-percent-auto-columns-precomparison.html)
- Independent expectations and Chrome 156 comparison: [precomparison evidence](../c11-root-html-grid-column-flow-percent-auto-columns-precomparison-2026-11-02/README.md)
- Fixture SHA-256: `b6f8eb8bf5f631a0cce483b5d389addcf8bb0126fc0a2f69c3d4f1b0de22d06a`
- Both app routes construct `RuntimeGpuHost::new_with_root_html` from this fixture and evaluate `void 0` through the app host.

## Android simulator

- Android API 37 ARM64 AVD `emulator-5556`, ANGLE/Vulkan with SwiftShader.
- Build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-android.sh` and the same environment for `tools/build-android-app.sh`.
- APK SHA-256: `c43f2f876ee146fabe4c0e5cd39205616c52038acd1fd619d33ee4a75230836d`.
- Runtime log reports `layout=ready`, `boxes=15`, `status=0`, and `presented boxes=15`.
- The 240×63 CSS px grid produced column starts `[0,35,80,109,150,179]`, row starts `[0,23]`, and twelve 10×10 CSS px items, matching `expected.json`.
- Capture: [AVD screenshot](./android/avd-screen.png), [runtime log](./android/avd-runtime.log).

## iOS simulator

- iPhone 17 Pro, iOS 26.2 Simulator, Metal.
- Build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C11_1_ROOT_HTML_PERCENT_COLUMNS_FIXTURE_TEST=1 tools/build-ios-sim.sh` (`BUILD SUCCEEDED`).
- App executable SHA-256: `2dd2c173ac7159e9d0550cf070595ef342dcbe9770ddcd1ddde06457f436cb63`.
- Runtime log reports `layout=ready`, `boxes=15`, `status=0`, and `presented boxes=15`.
- The grid node is 240×63 CSS px; its column starts are `[0,35,80,109,150,179]`, row starts are `[0,23]`, and twelve items are 10×10 CSS px. The grid and item frames match the precomparison geometry; the two outer Root HTML wrapper frames follow the simulator viewport and differ by platform.
- Capture: [iPhone simulator screenshot](./ios/iphone17pro-screen.png), [runtime log](./ios/iphone17pro-runtime.log).

## Fixture artifact boundary

- `cargo build -p spinon-ffi --lib` (default features): the C11.1 fixture symbol and fixture title payload were absent from `target/debug/libspinon_ffi.a`.
- `cargo build -p spinon-ffi --lib --features c11-1-root-html-percent-columns-fixture-test`: the `_spinon_runtime_gpu_host_new_c11_1_root_html_percent_columns_fixture` symbol and fixture title payload were present.
- The archive check used Rust 1.96's bundled `llvm-nm` and `strings`. This establishes the Rust static-library feature boundary, not the size or contents of a release app bundle.
- Focused tests passed: layout `root_html_column_flow` (15), style-to-layout `root_html_grid` (28), style `root_html_profile_` (9), runtime percentage/default-route and fixed-px cases (2), and runtime unsupported-shape rejection (1). Full command output: [`focused-tests.log`](./focused-tests.log).
- Scoped Clippy passed with `-D warnings` for the affected Rust packages on default features and for `spinon-ffi` with `c11-1-root-html-percent-columns-fixture-test`. `cargo fmt --all -- --check` and shell syntax checks for all three modified build scripts passed.

## Limits

These are simulator app-route and rendered-screen checks. The simulator surface viewport differs by platform (Android log: 121.90476×91.42857 CSS px; iOS log: 106.666664×80 CSS px), while the fixture's authored grid width is 240 CSS px. The grid tracks and item frames match the oracle on both platforms. Captures do not establish physical-device GPU behavior or pixel-for-pixel drawable readback.
