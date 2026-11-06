# C11.4 intrinsic columns + multiple text rows runtime evidence

Candidate source commit: `64502bd9a797ba7adcff539a85d79195777fd84b`. The checked fixture SHA-256 is `ef0cff8d4e53a162cc7a5dfdb0c8c5ec0d7b7f3942ba9985edc4f1f7101e33e6`; its Chrome 156 oracle and independently derived expectations are recorded in [precomparison](../c11-root-html-intrinsic-combined-multiple-text-precomparison-2026-11-06/README.md). The default Root HTML route runs the same fixture for four explicit column modes and both text source orders.

## Rust verification

- `cargo test --locked -p spinon-style-to-layout c11_1_root_html_intrinsic_auto_rows_text_tests --lib -- --nocapture`: 7/7 passed, including four column modes, reversed text order, contribution identity, invalid second item, and the combined 10,000/10,001 implicit-row boundary.
- `cargo test --locked -p spinon-runtime session::tests::root_html_grid_intrinsic_auto_rows --lib -- --nocapture`: 11/11 passed. Default `RuntimeSession` publication, box/text identity, source order, draw-command order, revisions, overflow, and atomic negative controls are covered.
- `cargo test --locked -p spinon-layout root_html_intrinsic --lib -- --nocapture`: 12/12 passed.
- `cargo fmt --all -- --check` and `cargo clippy --locked --workspace --all-targets -- -D warnings` passed.

Full package suites expose deterministic failures already present on base commit `3d1d4cab0c75b0905bd6952e3faee59a72dc863d`: current `spinon-style-to-layout` is 422 passed / 10 failed, `spinon-runtime` is 288 passed / 8 failed / 3 ignored, and `spinon-layout` is 453 passed / 1 failed. The eight runtime failures and ten style-to-layout failures were individually rerun after temporarily restoring the affected production/test source files from the base commit; each failed there as well. Their failure points are existing C11.5 fixed-track auto-size/stretch assertions and one C07 aspect-ratio oracle assertion. They are recorded as baseline failures, not passed checks or changes repaired by this slice.

## Android and iOS simulator smoke

The fixture was installed through the default Root HTML application route. Both native C11 fixture flags were off. Android used API 37 ARM64 AVD `emulator-5554`; iOS used iPhone 17 Pro Simulator on iOS 26.2 (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`). No physical Android target was used.

- Android reported `layout=ready`, `boxes=8`, `runtime_text_count=2`, and `presented boxes=8`; installed `assets/index.html` matches the fixture SHA. Screenshot: [1080×2400](android/screenshot.png).
- iOS reported `layout=ready`, `boxes=8`, and a completed default-app evaluation; installed HTML and JS assets match their recorded hashes. Screenshot: [1206×2622](ios/screenshot.png).
- `root-app.js` checked the parsed DOM and item order. Screenshots show the narrow min-content column, both text items, and content extending below the fixed 40px Grid.
- The screenshots are visual smoke only. No PixelCopy, Metal drawable readback, compositor pixel comparison, physical-device test, WPT run, or CI result is claimed.

The runner and original bootstrap copies are preserved beside the platform logs. The script restored the original bootstrap assets byte-for-byte after both app runs (`cmp` passed). Xcode's build log had trailing spaces and multiple final blank lines; only that whitespace was normalized so the evidence passes `git diff --check`.

## Scope boundary

This admits only the existing default parsed Root HTML text route for the bounded combined shape. C11.4 and C11 remain incomplete; official completion stays at 78/214. The change adds no public API, fixture FFI, arbitrary text/font/markup support, general CSS Grid admission, or completion status update.
