# C11.4 anonymous Grid text computed `line-break` runtime evidence

## Scope

This evidence covers the five fixed, opt-in anonymous direct-text Grid scenes in
[`expected-values.json`](expected-values.json). It does not mark C11.4 or C11
complete and does not change the official completion count (`72/213`). The
Chrome 156 observations, CSS references, font provenance and independent
expected-value derivation are recorded in the [precomparison evidence](../c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01/README.md).

The fixture uses `文〜文`, 10px Noto Sans KR, and `lang="ja"` or `lang="en"`.
Japanese `normal` permits three one-glyph lines for min-content (10×30px); the
Japanese `strict` and English `normal` controls use two lines (20×20px).
Max-content uses a 30×10px track. Each scene checks the computed keyword,
language, text layout, intrinsic inputs, Grid frame, final row/column tracks,
revision and runtime trace.

## Simulator results

Both simulator matrices passed 5/5 scenes.

| Target | Build artifact | Installed artifact | Result |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD `emulator-5554` (`sdk_gphone16k_arm64`) | APK SHA-256 `9ea401f5135676b024205749c3425decdd409673f6f1e36f505320cb23f11869` | Same SHA-256 | [matrix](android-avd/matrix.tsv), [simulator metadata](android-avd/simulator.json) |
| iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`) | executable SHA-256 `647f3b1c1490e383ea73b8343d1f8ae262d2736ffabd172461a8a860607ecdc5` | Same SHA-256 | [matrix](ios-simulator/matrix.tsv), [simulator metadata](ios-simulator/simulator.json), [attempt record](ios-simulator/attempts.tsv) |

Every scene log contains a ready layout and complete intrinsic sizing trace:
[Android scene logs](android-avd/) · [iOS scene logs](ios-simulator/).
The iOS capture runner retries once only when the runtime reports the startup
scene was superseded (`status=-12`) and preserves that failed-attempt log.
The opt-in static FFI archive separation is documented in the
[artifact-boundary record](artifact-boundary.md).
The implementation/runtime assessment is recorded in the
[20-perspective review](implementation-runtime-review-20.md).
The separate candidate diff assessment is recorded in the
[30-perspective PR diff review](pr-diff-review-30.md).

Screenshots: [Android](android-avd/representative-screen.png) ·
[iOS](ios-simulator/representative-screen.png). They show the app route and
`layout=ready boxes=1` status over the GPU canvas. The canvas does not show the
fixture glyphs, so these images do not prove text paint, drawable pixel
equality, or post-compositor output. Android used the API 37 AVD software
renderer; iOS used the Simulator Metal path. The screenshots do not establish
hardware renderer output. The scene logs and track-sizing trace are the
evidence for computed text metrics and layout geometry.

## Build and focused checks

- Android simulator build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh`
- iOS simulator build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh`
- `cargo test --locked -p spinon-text-layout c114_line_break` — 2 passed.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test line_break` — 6 passed.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test computed_line_break_scenes` — 1 passed.
- Feature-scoped Clippy passed for `spinon-runtime`, `spinon-text-layout`, and `spinon-ffi` with `--all-targets -- -D warnings`.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs` — 27 passed.
- `cargo fmt --all -- --check`, `git diff --check`, and `node --check tools/css-reference/capture-c11-4-anonymous-grid-text-line-break-simulators.mjs` passed.
- Simulator capture: `node tools/css-reference/capture-c11-4-anonymous-grid-text-line-break-simulators.mjs <android-avd|ios-simulator> <serial|UDID>`.

The matrices cover this fixed text/font/language/profile only. They do not
establish real-device or hardware-GPU behavior, glyph painting, WPT
conformance, general CSS admission, arbitrary language/font inheritance, or
full C11.4/C11 support.
