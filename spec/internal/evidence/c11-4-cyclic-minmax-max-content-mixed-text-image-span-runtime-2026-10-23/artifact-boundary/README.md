# Build, test, and static archive evidence

This directory records the fixture-enabled verification for the C11.4 cyclic minmax mixed text/image span slice. The runtime observations and scope limits are in the [evidence README](../README.md); exact Android and iOS scene results are in their respective successful simulator run directories.

## Static archive feature boundary

Both archives were built for `aarch64-apple-ios-sim` with `cargo build --locked -p spinon-ffi --target aarch64-apple-ios-sim --release --no-default-features`. The product-core archive enabled only `c04-runtime-gpu`; the positive fixture archive enabled `c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test`.

| Build | SHA-256 | Bytes | C11.4 fixture symbols and payload |
| --- | --- | ---: | --- |
| Product core (`c04-runtime-gpu`) | `4f8e2aee3581cd8aaed1672c63557c6f3676a02efeff5585dc1ba0c9b8819e82` | 74,800,232 | Absent |
| Fixture test (fixture-test + Metal pixel-test features) | `0df401411d73222b71cbc2eabb169afd368aa4a6d59f4655a1680cad6c919211` | 77,847,104 | Present |

The boundary inspection used `strings` and LLVM 22.1.8 `llvm-nm --demangle -g --defined-only`. The product-core archive omitted all four scene IDs, both image IDs, the unsupported-fixture marker, `cyclic_minmax_calc_mixed_text_image_span::runtime_source`, and the C11.4 fixture host create/eval symbols. Those markers were present in the positive archive. The archives were hashed and inspected in place, then removed from the ignored Cargo target area after checking that no process held them; the binary archives are not committed.

This proves the static archive feature boundary only. It does not establish fixture exclusion from a final release APK or IPA.

## Build and verification logs

| Evidence | Result |
| --- | --- |
| [`product-core-build.log`](product-core-build.log) | Product-core static archive build |
| [`fixture-test-build.log`](fixture-test-build.log) | Positive static archive build with fixture symbols |
| [`android-fixture-build.log`](android-fixture-build.log) | Android fixture APK build; the successful installed APK hash is in the [runtime evidence](../README.md) |
| [`ios-simulator-fixture-build.log`](ios-simulator-fixture-build.log) | iOS Simulator fixture app build; the successful executable hash is in the [runtime evidence](../README.md) |
| [`style-test.log`](style-test.log) | New max-content layout geometry and track trace test |
| [`style-previous-regression-test.log`](style-previous-regression-test.log) | Existing mixed text/image regression test |
| [`runtime-adapter-test.log`](runtime-adapter-test.log) | Eight-scene runtime adapter allowlist |
| [`ffi-scene-test.log`](ffi-scene-test.log) | Fixture FFI scene/source allowlist |
| [`style-clippy.log`](style-clippy.log), [`runtime-clippy.log`](runtime-clippy.log), [`ffi-clippy.log`](ffi-clippy.log) | Fixture-enabled Clippy for the three affected crates |
| [`workspace-clippy.log`](workspace-clippy.log) | Workspace Clippy |
| [`rustfmt-check.log`](rustfmt-check.log) | `cargo fmt --all -- --check` |
| [`capture-syntax-check.log`](capture-syntax-check.log) | `node --check` for the simulator capture script |
| [`diff-check.log`](diff-check.log) | `git diff --check` |

The successful Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator scene captures are summarized in the [runtime evidence README](../README.md). This evidence does not claim physical Android-device, hardware GPU, iOS post-compositor, WPT, or general CSS Grid coverage.

Captured build and test logs have trailing whitespace and redundant blank lines at end of file normalized so the committed evidence passes `git diff --check`; command output content is otherwise retained.
