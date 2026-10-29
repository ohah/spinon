# C11.3.3 template-area track-list cycling implementation evidence

This evidence covers the bounded forward fixed-px list-cycle extension for area-created implicit tracks. It does not mark C11.3.3 or C11 complete and does not change the official completion count (`72/213`).

## Oracle and runtime captures

- Chrome 156 precomparison: [fixture, observation, independent expected values](../c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/README.md).
- Android API 37 ARM64 AVD / SwiftShader: [final run](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/README.md), [result JSON](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/result.json), [screen capture](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T183746529Z-702554bb/android-track-list-cycle-screen.png).
- iPhone 17 Pro / iOS 26.2 Simulator: [final run](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/README.md), [result JSON](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/result.json), [screen capture](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T183927099Z-83fc5f15/ios-track-list-cycle-screen.png).

Both runtime runs matched 37 scenes / 118 nodes. Android compared 116 visible frames at maximum error 0 CSS px; the six cycle scenes matched 28/28 PixelCopy samples. iOS compared 116 node frames at maximum error 0 CSS px; the six cycle scenes matched 28/28 Metal drawable samples. The AVD uses SwiftShader; iOS samples are read before compositor submission.

## Review records

- [Implementation/runtime failure review: 20 distinct cases](./implementation-runtime-review-20.md).
- [Implementation PR diff review: 30 distinct cases](./implementation-pr-diff-review-30.md).

## Local gates

- `cargo fmt --all -- --check` — pass.
- Focused `spinon-style`, `spinon-style-to-layout`, `spinon-layout`, and feature-gated `spinon-ffi` tests — pass.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — pass (the changed crates were also checked with `c04-runtime-gpu-fixture-test`).
- C11.3.3 runtime fixture and fixture-module boundary Node tests — pass, 14/14 and 23/23.
- Runtime matrix builder `--check` — pass, 37 scenes / 118 nodes.
- Static archive boundary: default `spinon-ffi` has 0 C11.3.3 fixture exports and no cycle-scene strings; the `c11-grid-metal-surface-pixels-test` positive control has the three expected exports (`new`, `eval`, and surface-pixel request) plus cycle-scene strings. This does not establish APK/IPA or release-bundle exclusion.

The scope excludes numeric implicit-line placement, non-px track lists, negative/backward implicit sizing, WPT conformance, physical-device or hardware-GPU behavior, post-compositor iOS equality, and general CSS Grid support.
