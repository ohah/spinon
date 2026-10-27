# C11.4 generic mixed text and image runtime · 2026-10-28

## Scope and result

The opt-in direct-text/HTML-image adapter accepts four novel HostNode ID scenes: min-content and max-content columns, each in forward and reversed source order. The four existing fixed-ID mixed scenes were rerun as static-route controls. All eight scenes passed on both simulator targets: Android 8/8 and iOS 8/8. The matcher checks item source order and kind, intrinsic values and revisions, Columns/Rows traces, expected frames, and app draw/present. The adapter rejects malformed or out-of-profile inputs and leaves text-only/image-only inputs to their previous routes. The final matrix was built from clean tracked commit `a99e86e4cb4868776d609a8c3f999fdf36821a6c`; both manifests record the same tree, source-file hashes, empty tracked diff, and capture-driver hash.

The screenshots show the three layout boxes and host surface. They do not establish glyph or image texel painting. This is not ordinary CSS admission, a full WPT result, all of C11.4 Stage A, or C11.4 completion. The official count remains `72/213`.

## Simulator evidence

- Android API 37 ARM64, 16 KB AVD `emulator-5554`, Android 17 fingerprint: [accepted run](runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/), [generic mixed scene screenshot](runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/android-direct-min-content-generic-forward.png), [static-route control screenshot](runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/android-direct-min-content-forward.png), [matrix/provenance](runs/c114-direct-mixed-android-avd-20261027T195056521Z-a13fe581/android-simulator.json).
- iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`: [accepted run](runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/), [generic mixed scene screenshot](runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/ios-direct-min-content-generic-forward.png), [static-route control screenshot](runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/ios-direct-min-content-forward.png), [matrix/provenance](runs/c114-direct-mixed-ios-simulator-20261027T195138657Z-f58e345a/ios-simulator.json).

The runner addressed Android only through the selected emulator serial; the attached physical Samsung device was not selected. It terminated the Spinon app after capture and left the simulators running. Runtime/build logs, scene outputs, and app hashes are stored in each accepted run directory.

## Revision validation finding

The first app runs exposed a trace-report check that compared a text sub-layout's revision to the full mixed-layout revision. Text shaping uses the same document, style, and environment revisions but does not depend on the replaced-image intrinsic-size revision. The mixed layout correctly had intrinsic revision 1 while its text sub-layout retained intrinsic revision 0. The trace validator now checks source/style/environment equality for text sub-layouts while continuing to require an exact full revision for intrinsic measurements and grid-item inputs. The regression test is in [ffi-trace-report-tests.log](validation/ffi-trace-report-tests.log); failed diagnostic attempts are retained as separate run directories and are excluded from the 8/8 result.

The earlier passing runs `c114-direct-mixed-android-avd-20261027T194604869Z-018afc34` and `c114-direct-mixed-ios-simulator-20261027T194636806Z-5e80a94e` are also superseded. Their source commit did not track the newly added generic adapter modules, so those manifests cannot prove a clean committed-source build. The runs remain in the evidence archive but are excluded; the accepted runs above use commit `a99e86e4` with those modules committed and tracked.

## Code validation

- Runtime adapter tests: 5/5 passed — [log](validation/runtime-tests.log).
- FFI route tests: 1/1 passed — [log](validation/ffi-tests.log).
- FFI trace-report tests: 6/6 passed — [log](validation/ffi-trace-report-tests.log).
- `cargo fmt --all -- --check`: passed — [log](validation/fmt.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: passed — [log](validation/clippy.log).
- Default/fixture Rust static archive boundary: route absent by default and present under `c04-runtime-gpu-fixture-test` — [result](validation/archive-boundary/result.txt). This does not prove fixture exclusion from release APK/IPA bundles.

The sequential simulator runner is [run-simulator-matrix.sh](run-simulator-matrix.sh); the static archive check is [verify-archive-boundary.sh](verify-archive-boundary.sh). The capture driver hashes current source inputs and build artifacts in each run manifest.

## Completion state

C11.4 remains incomplete. No checkbox was promoted and the official count remains `72/213`. This fixture slice does not establish general CSS text/image admission, glyph/image pixel output, WPT conformance, Android physical-device behavior, or iOS compositor output.

The separate [implementation/runtime review (20 perspectives)](implementation-runtime-review-20.md) and [PR diff review (30 perspectives)](pr-diff-review-30.md) are complete. The merge record will be added here after the reviewed PR is merged.
