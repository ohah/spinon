# C11.4 anonymous Grid text `font-size: 0` runtime evidence

**Scope:** opt-in C11.4 fixture only. This run does not mark C11.4 or C11 complete and does not change the official `72/213` count.

## Build and simulator runs

- Android: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android` succeeded. The APK was installed onto the Android API 37 ARM64 AVD (`emulator-5556`, `sdk_gphone64_arm64`, QEMU). Built and installed APK SHA-256: `ecfe76ba1be793cacead8811d4b459aad0c565e2f6067bf0057a24ec6f8106c4`.
- iOS: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:ios-sim` succeeded. The app was installed onto the iPhone 17 Pro / iOS 26.2 Simulator (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`, `iPhone18,1`). Built and installed executable SHA-256: `281a2acd50c0ace3fea3efdbd27fd795210cfb8ecc22d7f5d79628e3994a0444`.
- `tools/css-reference/capture-c11-4-font-size-zero-simulators.mjs` launched the same four opt-in scenes on both targets. Each run required `status=0`, `layout=ready`, the exact expected box count and root frame, matching intrinsic/text layout traces, finite line geometry, and exact final column/row track sizes. The Android AVD app was force-stopped and the iOS Simulator app terminated by the runner after capture.
- Android matrix: [matrix.tsv](./android-avd/matrix.tsv). iOS matrix: [matrix.tsv](./ios-simulator/matrix.tsv). Per-scene raw log chunks, summary and trace JSON are in the matching target directory.

| Scene | Root | Boxes | Columns | Rows | Android | iOS |
|---|---:|---:|---:|---:|---|---|
| `font-size-0-min-content` | 100×10 | 1 | 0 | 10 | pass | pass |
| `font-size-0-max-content` | 100×10 | 1 | 0 | 10 | pass | pass |
| `font-size-0-text-before-box` | 100×13 | 2 | 5 | 10, 3 | pass | pass |
| `font-size-0-whitespace-only` | 100×0 | 0 | none | 0 | pass | pass |

For the first three scenes, the anonymous text intrinsic input reports min/max inline contribution `0`, min/max block contribution `10`, and a single line with `0` advance under Ahem `font-size: 0px; line-height: 10px`. In the text-before-box control, the stretched Grid item is 5×10, while its zero-advance text layout remains 0px wide; the following block occupies `(0,10)` at 5×3. Whitespace-only input produces no Grid item frame, intrinsic input, or paragraph layout.

## Screen evidence and limits

- Android API 37 AVD screen: [representative-screen.png](./android-avd/representative-screen.png)
- iPhone 17 Pro / iOS 26.2 Simulator screen: [representative-screen.png](./ios-simulator/representative-screen.png)

The captures show the runtime demo and its tiny red 5×3 block in the text-before-box control, with `layout=ready boxes=2`. They are simulator screen captures, not surface readback or a glyph-pixel comparison. Zero-size glyph rasterization and general CSS admission were not tested. Android ran on an AVD software graphics path; no physical device or hardware GPU was used. No CI result is claimed.

## Static archive fixture boundary

Both release archives were built for `aarch64-apple-darwin` from the current candidate source. `strings` found none of the four `font-size-0-*` scene IDs in the default `spinon-ffi` archive; the positive-control archive built with `c04-runtime-gpu-fixture-test` contains all four IDs. This is a static archive string scan, not a complete APK/IPA or symbol-stripping proof.

| Archive | SHA-256 | Scene IDs found |
|---|---|---|
| Default `spinon-ffi` | `2a522eb8807f443f3f291e083d78c098a55ac5c8da3cdd1ff5ec16e8906fc0b9` | 0/4 |
| `spinon-ffi` + `c04-runtime-gpu-fixture-test` | `f8db0af8607a949f6b0947fc09322dd144ee985929e350d4cdecf222666159ec` | 4/4 |

The all-target runtime Clippy command is blocked by an existing `too_many_arguments` lint in `runtime_layout_tests/c11_grid_tests/anonymous_text_hyphens.rs`; the scoped runtime library Clippy command passes. The issue is outside this change and was not modified.

The broader feature-enabled runtime library suite reported 425 passed, 4 failed, and 3 ignored. The failing cases are `root_html_grid::c11_1_root_html_default_runtime_rejects_out_of_scope_author_grid_inputs`, both `root_html_grid_anonymous_text` cases (`keeps_adjacent_text_sources_in_one_grid_item`, `renders_one_anonymous_grid_text_item`), and `root_html_grid_implicit_rows::c11_1_root_html_default_runtime_rejects_unsupported_implicit_row_combinations_atomically`. They exercise Root HTML profiles outside the new C11.4 scene allowlist. The first anonymous-text failure was rerun with the previous frame-count expression restored and still reported `root_html_default_render_owner_frame_missing`; the candidate does not claim the full runtime suite is green. Without the fixture feature, runtime library tests reported 219 passed, 2 failed, and 3 ignored; the two failures are the out-of-scope Grid and implicit-row cases above.

## Checksums

`SHA256SUMS` contains hashes for the captured logs, matrices, simulator metadata and screenshots. Archive hashes above are recorded here; the build outputs remain in the ignored `build/` directory.
