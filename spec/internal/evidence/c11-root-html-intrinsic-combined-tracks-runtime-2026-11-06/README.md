# Root HTML intrinsic columns + implicit min-content text rows

## Candidate

- [Implementation/runtime review — 20 perspectives](implementation-runtime-review-20.md) · [PR #1195 actual diff review — 30 perspectives](pr-diff-review-30.md).
- Fixture: `tests/fixtures/css/c11/root-html-grid-intrinsic-combined-tracks-runtime.html`
- Fixture SHA-256: `fc9ff608f630e7a6e6e2f185f8e49da93b696f5ea1b167efca4996082403380e`
- Both simulator apps used the existing C04.12 default Root HTML route. The app's packaged `root-html-default-rendering.html` was temporarily replaced with this fixture for each build, then both source assets were restored and verified at SHA-256 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`.
- Both packaged app assets matched the fixture SHA above.

## Android

- Android 17 / API 37 ARM64 AVD, `sdk_gphone64_arm64`, emulator `emulator-5554`.
- Renderer: ANGLE over Vulkan SwiftShader software rendering. No physical Android device or hardware GPU was used.
- Build: `SPINON_V8_DIR=$PWD/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android`
- APK SHA-256: `e936799e6897dca85172ae47c13ef6997b5034f6be3f5f8da4d8348e1078792d`
- Runtime source log reports the fixture SHA, `layout=ready boxes=7`, viewport `320×240`, root `320×120`, then WGPU draw/present with 7 boxes and 1 text/glyph batch.
- [Screenshot](android-avd-api37/screen.png) (1080×2400), [runtime log](android-avd-api37/runtime.log), [build log](android-avd-api37/build.log).
- Screenshot SHA-256: `81b2604e7c6b0289b4d0b144c9fdc6e80712f4b2e6608024e71946b9e0ea7967`.

## iOS

- iPhone 17 Pro Simulator, iOS 26.2 (`23C54`), simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, model identifier `iPhone18,1`.
- Build: `SPINON_V8_DIR=$PWD/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:ios-sim`
- `SpinonBootstrap` executable SHA-256: `40190537e135404528c5ce2a826ec3fe34f5ad4635d0f8a30aa3082bf9464a37`
- Runtime source log reports the fixture SHA, `layout=ready boxes=7`, viewport `320×240`, root `320×120`, then draw/present with 7 boxes and 1 text/glyph batch.
- [Screenshot](ios-simulator-iphone-17-pro/screen.png) (1206×2622), [runtime log](ios-simulator-iphone-17-pro/runtime.log), [build log](ios-simulator-iphone-17-pro/build.log).
- Screenshot SHA-256: `e189a20df3c11edd45fa9b377601eddc560066ffe094a30c810969bc0acb48ca`.

## Rust validation on the candidate branch

- Focused `spinon-layout` intrinsic-track tests: 12 passed.
- Focused `spinon-style-to-layout` parsed Root HTML tests: 23 passed.
- Focused `spinon-runtime` parsed Root HTML intrinsic auto-row tests: 5 passed.
- The combined runtime candidate's default-session success and atomic out-of-slice negative tests each passed; the expanded style matrix also covers `minmax(10px,max-content)`.
- `cargo fmt --all -- --check` and scoped all-target Clippy for the three changed Rust crates passed.
- The last broad crate runs before the final added runtime negative test were not all green: layout 453 passed / 1 failed; style-to-layout 413 passed / 10 failed; runtime 283 passed / 8 failed / 3 ignored. These results are not presented as candidate baseline comparisons.

Screenshots are visual draw evidence. This run did not perform Android PixelCopy, iOS Metal drawable readback, compositor-pixel comparison, WPT, real-device, or hardware-GPU validation. It is one bounded Root HTML runtime slice and does not complete C11.4/C11 or change the official roadmap count.
