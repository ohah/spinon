# C11.4 generic direct-text runtime matrix · 2026-10-27

## Scope and result

The six fixed generic direct-text intrinsic scenes passed the native app matcher on both target simulators: Android API 37 ARM64 AVD, 6/6, and iPhone 17 Pro / iOS 26.2 Simulator, 6/6. The scenes cover normal wrapping, nowrap, break-all, different content, collapsible source whitespace, and item-specific style after DOM reorder. Every scene checks the typed item input, raw and normalized text, layout/style/revision tuple, line advances, Columns and Rows contributions, item order and frames, and app draw/present report.

This is runtime evidence for the opt-in direct-text input path only. It does not establish ordinary CSS text admission, glyph painting, full C11.4 Stage A, C11.4 completion, or C11 completion. The captured screens show layout boxes and the host surface. Runtime logs explicitly report runtime_text_count=0 and glyph_batch_draw_count=0.

## Simulator evidence

The accepted six-scene run is [pass02](runs/c114-direct-text-matrix-20261027T031021Z-pass02/). It stores the source patch/hash, APK and iOS app bundle hashes, simulator identifiers, per-scene launch/runtime/termination logs, 6/6 matrix tables, test and lint logs, archive-boundary result, and one screenshot per platform.

- Android API 37 ARM64 AVD: [representative normal scene screenshot](runs/c114-direct-text-matrix-20261027T031021Z-pass02/android-avd/screenshot.png) · [matrix](runs/c114-direct-text-matrix-20261027T031021Z-pass02/android-avd/matrix.tsv)
- iPhone 17 Pro / iOS 26.2 Simulator: [representative normal scene screenshot](runs/c114-direct-text-matrix-20261027T031021Z-pass02/ios-simulator/screenshot.png) · [matrix](runs/c114-direct-text-matrix-20261027T031021Z-pass02/ios-simulator/matrix.tsv)

The Android screenshots were captured from emulator-5554, AVD spinon_api37_1_compat16k, API 37, ABI arm64-v8a. The iOS screenshots were captured from UDID 9DCCE836-7659-4914-B6C1-13F54BFFA9DB, model identifier iPhone18,1. The runner terminated the Spinon app after each scene and left both simulators running.

The earlier pass01 run is retained separately. Its first three Android controls passed, then the harness-generated run ID for content-variation exceeded the app's 80-character argument limit and was rejected before layout. The runner now uses short per-platform scene-index IDs; pass02 completed all twelve launches. No product source changed between those two passes.

## Code validation

- Opt-in Android app build: successful.
- Opt-in iOS Simulator app build: successful. Xcode emitted duplicate RawWindowMetalLayer linker-symbol warnings; the build completed.
- Generic direct-text FFI scene test: 1/1 passed.
- Generic collector runtime test: 1/1 passed.
- FFI trace-report tests: 5/5 passed.
- cargo fmt --all -- --check: passed.
- cargo clippy --locked --workspace --all-targets -- -D warnings: passed.
- Rust static archive boundary: the new route string is absent from the default spinon-ffi archive and present in the opt-in c04-runtime-gpu-fixture-test archive. This does not establish release APK or IPA fixture exclusion. The first positive-control probe used rg -q with a piped strings producer under pipefail, which caused a false negative when rg closed the pipe early; the corrected full-consuming probe passed and its result is in validation/archive-boundary/result.txt.

Reports: [implementation/runtime review, 20 perspectives](implementation-runtime-review-20.md) · [PR diff review, 20 perspectives](pr-diff-review-20.md).

## Reproduce the simulator matrix

Build both opt-in apps with SPINON_ENABLE_C04_RUNTIME_GPU=1 and SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1, then run:

    spec/internal/evidence/c11-4-generic-direct-text-runtime-matrix-2026-10-27/run-simulator-matrix.sh
