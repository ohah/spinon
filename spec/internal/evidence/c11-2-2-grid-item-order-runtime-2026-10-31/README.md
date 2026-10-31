# C11.2.2 sparse row `order` runtime candidate · 2026-10-31

## Scope

This is a bounded extension of the existing `RuntimeBlockFlexGridAutoPlacementV1` test route. Two fixed scenes admit nonzero computed `order` only when the visible direct children are all fully auto-placed. Row sparse placement sorts by signed order and preserves source order for ties. Existing C11.2.2's 16-scene runtime matrix is unchanged.

The Chrome 156 Agent Browser precomparison, source HTML, observed computed values, rectangles, and input hashes are in [precomparison evidence](../c11-2-2-grid-item-order-precomparison-2026-10-31/README.md). Runtime fixture and surface-pixel matrices are separate versioned JSON files under `tests/fixtures/css/c11/`.

## Results

- Rust layout, style projection, FFI route, and matrix checks passed for signed order, stable ties, source DOM order preservation, `i32` boundaries, and unsupported dense/partial-placement controls.
- Android 17/API 37 ARM64 AVD: [run](./android-avd/c1122-order-a-mvvuthqz-298dcdbd/README.md), 2/2 scenes, 11 frame comparisons, maximum geometry error 0 CSS px, 7/7 PixelCopy samples exact. Renderer: ANGLE/Vulkan/SwiftShader.
- iPhone 17 Pro/iOS 26.2 Simulator: [run](./ios-simulator/c1122-order-i-mvvutu0t-0faf4978/README.md), 2/2 scenes, 11 frame comparisons, maximum geometry error 0 CSS px, 7/7 Metal drawable samples exact. Readback is before OS compositor submission.
- Node matrix generator check and tests passed 2/2. Workspace Clippy, `cargo fmt --all -- --check`, and `git diff --check` passed after one Clippy style correction.
- `cargo test --workspace --locked` ran 265 tests successfully in `spinon-style-to-layout` before one unrelated existing C07.3 serialization expectation failed: actual `1.7777778 / 1`, pinned expected `1.77778 / 1`. Re-running that exact test reproduced the same mismatch. The changed files do not include the C07.3 test. The workspace rerun excluding only that exact case passed 1,431 tests with 4 ignored, 0 failed; [full filtered log](./workspace-test-excluding-c07-3-baseline.log) and [failure observation](./workspace-test-c07-3-baseline-failure.md) are retained.
- Default `c04-runtime-gpu` static archive had neither new C11.2.2 fixture symbol; `c04-runtime-gpu-fixture-test` positive-control archive contained the new host creation/eval symbols. [Archive inspection](./artifact-symbol-boundary.md).

## First Android attempt and correction

The first AVD attempt is retained at [initial failure](./android-avd/c1122-order-a-mvvuqz8l-f000139b/). The activity launched but the runner timed out waiting for runtime/draw/PixelCopy markers. Source tracing found the new scene IDs missing from the platform C11.2.2 scene and pixel admission allowlists. Added both IDs to the Android allowlists and corresponding iOS allowlists, then rebuilt and reran both simulator matrices successfully. No simulator app process from these capture runs remains active.

## Review

- [Implementation/runtime failure review · 20 perspectives](./implementation-review-20.md)
- [PR diff review · 30 perspectives](./pr-delta-review-30.md)

## Limits

The scenes use non-overlapping items. They do not verify overlapping paint/stacking behavior, accessibility or keyboard order, arbitrary author CSS, the full WPT suite, physical devices, hardware GPU behavior, or OS-composited iOS pixels. This candidate does not complete C11.2.2/C11 or change the official completion count `72/213`.
