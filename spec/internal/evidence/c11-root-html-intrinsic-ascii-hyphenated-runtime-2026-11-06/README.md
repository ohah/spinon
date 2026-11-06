# C11.4 parsed Root HTML ASCII hyphenated words runtime · 2026-11-06

## Result

The bounded implementation in [PR #1224](https://github.com/ohah/spinon/pull/1224) was Rebase and merged at 2026-11-06T14:35:05Z (merge SHA `4039ed60327b09128503559fcc45268acbb9637e`). Candidate source commit `3478d9bad89dfc8bb82a063af838b3b9af277756` passed the Android and iOS Simulator app route with the same fixture SHA. The visible `12345678-12` word wraps after U+002D in both screenshots. Root HTML layout reached `layout=ready boxes=8`; the renderer presented eight boxes and two text draws.

This is a candidate verification record. C11.4 and C11 remain incomplete, general CSS admission and WPT are not covered, and the official implementation count remains `78/214`.

## Android

- Device: Android API 37, ARM64 emulator `emulator-5554`.
- Renderer: ANGLE on Vulkan with the SwiftShader software device, as reported by SurfaceFlinger.
- Candidate app used the isolated package ID `dev.spinon.bootstrap.c11hyphen20261106`. The existing `dev.spinon.bootstrap` process remained PID `29922`, and its `MainActivity` was restored after the run.
- The default Root HTML route reported `layout=ready boxes=8`, `presented boxes=8`, and `runtime_text_count=2`.
- SurfaceView PixelCopy checked the existing overflow sample at CSS `(42,19)`, physical `(111,51)`: expected and observed RGBA `[91,168,245,255]`, exact `1/1`.
- [Screenshot](./pass01/android/screenshot.png) · [PixelCopy PNG](./pass01/android/pixelcopy.png) · [App log](./pass01/android/app.log).

## iOS

- Device: iPhone 17 Pro, iOS 26.2 Simulator.
- The default Root HTML route reported `layout=ready boxes=8`, `presented boxes=8`, and `runtime_text_count=2`.
- Metal drawable sampling before compositor submission checked the corresponding CSS `(42,19)` sample at drawable `(127,58)`: expected and observed RGBA `[91,168,245,255]`, exact `1/1`.
- [Screenshot](./pass01/ios/screenshot.png) · [App log](./pass01/ios/app.log).

## Build and code checks

- Style-to-layout C11 Root HTML intrinsic rows: `12/12` passed.
- RuntimeSession default and `c11-4-grid-track-sizing-fixture-test`: `16/16` each passed.
- Text-layout C11.4 line-break tests: `4/4`; existing `hyphens:auto` fixture tests: `3/3`.
- Changed-crate Clippy, fixture-feature RuntimeSession library-only Clippy, `cargo fmt --all -- --check`, `git diff --check`, and the Android/iOS Simulator builds passed.
- Fixture-feature RuntimeSession all-target Clippy still reports two `too_many_arguments` lints in unchanged helpers `anonymous_text_hyphens.rs:117` and `anonymous_text_text_transform.rs:507`. This candidate did not modify either file.
- The runner's `environment.txt` records the candidate HEAD, source manifest, device properties, fixture hashes, and restoration checks. `source-inputs.sha256` binds 26 implementation, fixture, build, and adapter inputs to this capture.

## Scope and cleanup

The screenshots are visual runtime evidence. Android PixelCopy and the iOS Metal drawable sample prove only the single recorded overflow pixel; the iOS sample is before compositor submission. This run does not establish physical-device behavior, Android hardware GPU output, iOS post-compositor pixel parity, WPT results, arbitrary typography, general hyphenation, or full C11.4/C11 support.

The runner removed its isolated Android package, restored the original Android and iOS app assets and Android Gradle file byte-for-byte, and returned the original Android activity with its original PID. The temporary iOS Simulator and the separately created, unbootable Android AVD were deleted after evidence capture. The existing Android emulator was left running.

## Checksums

[`SHA256SUMS`](./pass01/SHA256SUMS) covers all captured pass files except the checksum list itself.
