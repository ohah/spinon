# C11.4 generic direct-text intrinsic runtime evidence

## Scope

This evidence validates the single feature-gated app scene `generic-root-normal`, with two direct `A BC` text items. It does not complete C11.4 Stage A or the full C11.4 suite. The official roadmap remains 72/213 complete, with C11.4 incomplete.

The six-scene Rust collector tests cover a wider HostDocument fixture set. Android/iOS app coverage is deliberately limited to this canonical scene; source/style/fallback rejection controls remain Rust tests. These screenshots show layout boxes and the app surface. Both logs report `runtime_text_count=0` and `glyph_batch_draw_count=0`, so they do not demonstrate glyph painting or pixel equality.

## Build and simulator result

Run ID: `c114-generic-direct-text-20261026T031719Z-708bee0c-pass02`

- The app builds used the clean, pinned V8 source checkout at `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`, commit `7b50b62cb18f28617959e8452e2cd18195b38bcf`. Existing archive SHA-256 values: Android `e0288be05fdedd7b5bed945dc446f7ec7dc11454034d0efc3dfe297d32cd7813`; iOS `7a3e24607ea7b419d3fd858c35c094a566660f9d3112015487fa010d99e99815`. This worktree has no local V8 checkout, so both app builds passed `SPINON_V8_DIR` explicitly; no checkout synchronization was run.
- Android fixture-test Debug APK build passed with `c04_runtime_gpu_fixture_test=1`. APK SHA-256: `d4d36fb0391c905570f13ab90ed13c9b5c16451f662f3a7a558a19bb9e4617cd`.
- iOS Simulator fixture-test Debug app build succeeded with the same fixture route enabled. App executable SHA-256: `4f6fd64fd1f00f56cc5cfef4d9f4b6fcecffea141f68aea694bba6bfb46b2388`.
- Both app artifacts were built from source commit `708bee0cae9b16872ab3a4d5da9d2f67dc8e29bd`. Current Java and Swift matcher SHA-256 values are `c702dd18f3803e1a6a17d81e4ad28c7dc350dcc22ec3daf679e5ade3b79e2346` and `f061c7bed7644729f9679aa010dff95d69efa112ec1f6e79ef25d83578a530ea`.
- Android API 37 ARM64 AVD (`spinon_api37_1_compat16k`) passed `status=0`, `layout=ready`, `boxes=3`, expected node frames and intrinsic/track trace, and `SPINON_C0410_DRAW status=0 presented boxes=3`.
- iPhone 17 Pro / iOS 26.2 Simulator passed the same layout, frame, trace and draw/present assertions.
- The runner stopped the app on both platforms after capture. The Android package process was absent afterward; the iOS Simulator launch services had no remaining SpinonBootstrap process.

## Screenshots

![Android API 37 AVD: generic-root-normal layout and surface](./runs/c114-generic-direct-text-20261026T031719Z-708bee0c-pass02/android-avd/screenshot.png)

Android screenshot: 1080×2400, SHA-256 `4d35f936a25ae96b8483adc518ceeb72d3692b5532a9c6260ea18cc7b739fb23`.

![iPhone 17 Pro Simulator: generic-root-normal layout and surface](./runs/c114-generic-direct-text-20261026T031719Z-708bee0c-pass02/ios-simulator/screenshot.png)

iOS screenshot: 1206×2622, SHA-256 `1905f806d7c8d02045d5e57c219f2154cbd41e9c745cd167415147814b7205f4`.

## Runtime trace values

The app scene resolves the invalid author values `max-width:auto`/`max-height:auto` to valid `none` values. The observed runtime diagnostics report `layoutInputRevision.intrinsicSize=0` for this new HostDocument. The app matchers verify a HostDocument source and nonnegative revision values.

- Intrinsic Text inputs: item nodes 2 and 4, source order 0 and 1, Ahem font registry revision 1, inline min/max 20/40px, block min/max 20/10px, no anonymous text sources.
- Columns content track sizes: `[20, 40]`; row content track size: `[20]`.
- Serialized tracks include collapsed zero-size outer gutters and a 10px gutter between the two columns.
- Columns contributions include node 2 `MinContent=20`, and node 4 `MinContent=20` plus `MaxContent=40`. Repeated identical tuples are accepted.
- Rows contributions are measured at each assigned column width: node 2 contributes 20px for `Minimum`, `MinContent`, and `MaxContent`; node 4 contributes 10px for each. All spans are 1 and `crossesFlexibleTrack=false`.
- Frames: root `(0,0,100,20)`; `generic-normal-a` `(0,0,20,20)`; `generic-normal-b` `(30,0,40,10)`.

These contribution values and track gutters describe Spinon's emitted runtime trace; the saved Chrome DOM observation does not expose Taffy contribution kinds.

## Validation artifacts

- [Full simulator matrix log](./runs/c114-generic-direct-text-20261026T031719Z-708bee0c-pass02/simulator-matrix.log)
- [Android build/runtime logs](./runs/c114-generic-direct-text-20261026T031719Z-708bee0c-pass02/android-avd/)
- [iOS successful build/runtime logs](./runs/c114-generic-direct-text-20261026T031719Z-708bee0c-pass02/ios-simulator/)
- [Focused Rust test](./runs/c114-generic-direct-text-20261026T023012Z-12741e45-pass01/rust-focused-test-final.log): 9 passed
- [Focused FFI source test](./runs/c114-generic-direct-text-20261026T023012Z-12741e45-pass01/ffi-focused-test-final.log): 1 passed
- Original pinned V8 build logs are under the earlier pass01 run's `v8/android/` and `v8/ios/` directories; pass02 reused those pinned archives.

The `android-avd-early/`, `android-avd-corrected-early/`, and `android-avd-refined-early/` directories in pass01 retain diagnostic attempts. Pass02's initial Android build without `SPINON_V8_DIR` is preserved at `android-avd-early/build-missing-v8-dir.log`; it stopped because this feature worktree has no local V8 checkout. The successful pass02 app builds used the pinned external checkout above, and the simulator matrix ran from the committed source revision. Pass01 is retained as earlier evidence; it is not deleted.

The pass02 `source-head.txt` records commit `708bee0cae9b16872ab3a4d5da9d2f67dc8e29bd`. No simulator snapshots or evidence PNGs were deleted.
