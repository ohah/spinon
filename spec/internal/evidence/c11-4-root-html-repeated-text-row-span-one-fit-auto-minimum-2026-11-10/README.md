# C11.4 one-fit row-span automatic minimum · 2026-11-10

This bounded slice adds `min-height:auto` for the existing one-fit repeated-text row-span profile with three implicit rows. It does not complete C11.4 or C11 and does not change the official `78/214` count.

## App visual smoke

The existing `root-html-default-render` app route loaded the same baseline/candidate Root HTML on Android and iOS. The grid uses one `fit-content(0%)` track, the `span2-before-span3` overlap topology, a fixed explicit-row control with `min-height:0`, and direct monospace text. Light mode uses `align-content:start`; dark mode uses `align-content:stretch` and the page's `prefers-color-scheme` styles.

| Platform | Baseline | Candidate |
| --- | --- | --- |
| Android API 37.1 ARM64 `spinon_api37_1_compat16k` AVD | [light](./android-baseline-light.png) · [dark](./android-baseline-dark.png) | [light](./android-candidate-light.png) · [dark](./android-candidate-dark.png) |
| iPhone 17 Pro, iOS 26.2 Simulator | [light](./ios-baseline-light.png) · [dark](./ios-baseline-dark.png) | [light](./ios-candidate-light.png) · [dark](./ios-candidate-dark.png) |

The four runtime logs per platform record the loaded HTML SHA, `layout=ready`, and a successful `SPINON_C0410_DRAW ... presented`. Dark-mode logs record `dark=true`. Android used the emulator's ANGLE/SwiftShader renderer. These are simulator visual-smoke captures; they are not physical-device tests, PixelCopy equality checks, Metal drawable readback, or proof of pixel equality after the OS compositor.

The temporary app-resource substitutions have been restored. Both source assets and both final build outputs again contain the original default HTML SHA `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`.

The captured Xcode build logs had trailing spaces in echoed search-path commands and an extra blank line at EOF. Those whitespace-only differences were normalized before commit; command output and build diagnostics were otherwise retained.

| Variant | Root HTML SHA-256 | Android APK SHA-256 | iOS app executable SHA-256 |
| --- | --- | --- | --- |
| baseline (`min-height:0`) | `cd4b9e524ae4d552cb6d044f134ae35bd05a04a1ab1f36277e3ec27951f48b58` | `a227dfe02ce39a0061062b0e55d4a9e91e9949215eacac64fa68e6e0a1232615` | `fa998813020428313c5b884970e9047d59b9fdfc928c3fde00fb957af5570369` |
| candidate (`min-height:auto`) | `2f53d679d701de52af92b7b6c9b66456c78b287d947ff1fede27cbe6286a3395` | `7599f966b9ca05f76efc17d7faa14eb78bbb1c3faa2cd292a977f03930555474` | `81989ea3fce00cf725f8fcef8ec4789bf5bdb0bf45ef9535dbe00bccd5b57089` |

The HTML SHA is identical between Android and iOS for each variant and matches the runtime source log. The screenshots use the default product-feature build, not a fixture-test FFI adapter.

Android and iOS builds succeeded. The Android toolchain warned that AGP 8.13.2 is tested through compile SDK 36.1 while this AVD uses 37.1/37.2 tooling; it also ignored a mismatched `ANDROID_NDK_HOME` and selected the pinned SDK NDK `27.1.12297006`. Xcode emitted duplicate debug-map object warnings from the pinned V8 archive. None prevented build or route startup.

## Runtime and layout verification

| Check | Result |
| --- | --- |
| `parsed_root_html_one_fit_row_span_auto_minimum_matches_all_2880_chrome_pairs` | PASS · 2,880 paired scenes / 5,760 layouts · 236.40s |
| `parsed_root_html_one_fit_row_span_auto_minimum_rejects` | PASS · mixed minima, explicit control, fit-position boundary, atomic rejection/recovery |
| `parsed_root_html_one_fit_zero_cap_preserves_overlapping_span_minimums` | PASS · zero-cap overlapping-span regression |
| one-fit auto-placement `parsed_root_html_one_fit_auto_minimum` | PASS · 1,440 scenes plus controls/rejections · 70.68s |
| two-fit row-span `parsed_root_html_two_fit_content` | PASS · 3 tests including 3,840 scenes and the two-cap min-content floor control · 181.89s |
| two-fit auto-placement `parsed_root_html_two_fit` | PASS · both 2,880-scene matrices and controls/rejections; a broader initial run also exposed the old two-fit floor control, which passed after the scope fix |
| later fit positions `parsed_root_html_later_fit_content_positions` | PASS · 1,920 and 4,320 scenes, including rejection checks · 223.95s |
| repeated intrinsic text `parsed_root_html_repeated_intrinsic_text_matches_all_810_chrome_scenes` | PASS · 810 scenes · 38.46s |
| original row-span `parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes` | PASS · 960 scenes · 39.52s |
| `cargo fmt --all -- --check` | PASS |
| Clippy on changed crates/all targets | PASS with `-D clippy::too_many_arguments` retained and the two existing anonymous-text helper warnings explicitly allowed |

The final candidate rerun after adding Runtime glyph-line assertions passed the same 2,880 pairs / 5,760 layouts in 248.75s. The automatic-minimum tests are gated behind `c11-4-grid-track-sizing-fixture-test`, so this long matrix does not run in default runtime test builds.

Additional final-candidate checks:

- Vendored Taffy track-sizing unit tests: 4/4 passed, including new minimum contribution growth beyond a fit-content cap and maximum contribution cap preservation. Taffy emitted six existing unused-mut/unused-variable warnings in unrelated `compute/grid/mod.rs` code.
- Layout nearest-rounding and track-trace tests: 8/8 passed, including the nearest/cap tolerance boundary and the rounded-down case that needs no growth allowance.
- Runtime invalid `fitPosition` fixture rejection: 1/1 passed. Absent `fitPosition` remains compatible with the older fixed fixture corpus; malformed present values fail closed.
- Runtime zero-cap overlapping-span trace and automatic-minimum contribution trace: 1/1 each passed.
- Existing direct-text row-span matrix after the source-generator change: 960/960 scenes passed in 41.94s.
- Runtime candidate matrix compares actual glyph baseline line counts with Chrome line rectangles as well as frames and visible bottom overflow.
- Implementation/runtime review: [20 distinct failure perspectives](./implementation-runtime-review-20.md). PR diff review: [30 distinct failure perspectives](./pr-diff-review-30.md).

Reproduction commands and final results:

```sh
cargo test --manifest-path vendor/taffy/Cargo.toml compute::grid::track_sizing::tests -- --nocapture
# 4 passed; six existing unused_mut/unused_variables warnings from compute/grid/mod.rs

cargo test --locked -p spinon-layout blink_ -- --nocapture
# 9 passed

cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_one_fit_row_span_auto_minimum -- --nocapture
# 3 passed, including 2,880 pairs / 5,760 layouts

cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test malformed_fit_position_in_fixture_fails_closed -- --nocapture
# 1 passed

cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_one_fit_zero_cap_preserves_overlapping_span_minimums -- --nocapture
# 1 passed

cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_one_fit_row_span_trace_keeps_both_span_auto_minimums_above_zero_cap -- --nocapture
# 1 passed

cargo test --locked -p spinon-runtime parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes -- --nocapture
# 1 passed, 960 scenes

cargo fmt --all -- --check
# passed

cargo clippy --locked --all-targets -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --features spinon-runtime/c11-4-grid-track-sizing-fixture-test -- -D warnings -A clippy::too_many_arguments
# passed; the pre-existing too_many_arguments lint is the only allowed lint
```

Two broader package runs remain non-green on other contracts:

- `cargo test --locked -p spinon-layout`: 524 passed, 1 failed. The failure is `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order` in the unchanged HostDocument path; an isolated run also failed identically.
- `cargo test --locked -p spinon-style-to-layout`: 455 passed, 10 failed. The failures are in the existing C07.3 computed-ratio serialization and C11.1/C11.5 fixed-grid self-alignment/placement/percentage-gap fixture paths. An isolated `root_html_fixed_grid_projects_distribution_values_to_taffy` reproduces `UnsupportedGridContainerStyle { property: "C11.5 fixed-track auto-size/stretch contract" }`; neither that validator nor the failing test was changed here.
- Strict all-target Clippy without an allow stops on two existing `too_many_arguments` helpers in `anonymous_text_hyphens.rs` and `anonymous_text_text_transform.rs`. With only that lint allowed, the changed crates pass `-D warnings`.

## Cleanup

The Android AVD and iPhone 17 Pro Simulator were both shut down after capture. ADB has no connected device; no task-owned emulator, Simulator app, Spinon app, or Gradle daemon remains. There is no S03-named simulator profile. The iPhone 17 Pro and Android AVD are shared profiles reused by adjacent Spinon slices, so their profile data was preserved. No Agent Browser was started for this visual smoke. The native app-resource files were restored, and the restored Android APK and iOS `.app` were rebuilt with the original default HTML asset. After the standalone Taffy unit tests completed, `cargo clean --manifest-path vendor/taffy/Cargo.toml` removed their task-created build output (667 files, 719.0 MiB).
