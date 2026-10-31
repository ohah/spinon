# C11.4 anonymous Grid text computed `letter-spacing` runtime

**Result:** The fixed 16-scene opt-in profile passed on Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator. This is a bounded runtime candidate; C11.4, C11, public CSS support, and the official `72/213` count remain unchanged.

## Scope and oracle

The scenes use Ahem 10px/400, horizontal-tb, LTR, `white-space:normal`, `word-break:normal`, and anonymous direct Grid text containing either `XX` or `XX XX`. They cover computed `normal`, authored `0px`, and finite absolute values from -1px through 2px. Independently derived CSS Text/CSS Sizing values and the Chrome 156 comparison are in the [precomparison](../c11-4-anonymous-grid-text-letter-spacing-precomparison-2026-10-31/README.md) and [expected values](../c11-4-anonymous-grid-text-letter-spacing-precomparison-2026-10-31/expected-values.json).

Stylo serializes the authored `0px` scene as computed `normal`; the fixture verifies that serialization and uses the equivalent 0 CSS px input. The observation is limited to this Stylo snapshot path and does not describe general CSSOM serialization. Chrome 156 differs from the independent expected values on several positive and negative spacing cases; this run does not claim Chrome parity.

## Platform runs

| Target | Result | Installed artifact |
| --- | ---: | --- |
| Android API 37 ARM64 AVD (`emulator-5554`, `sdk_gphone16k_arm64`) | 16/16 scenes | APK SHA-256 matches: `940cdd5bd7dbe5e6bc35da3624d88907c889087cf9e7b0aacc0d72e51dc8fecd` |
| iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`) | 16/16 scenes | App executable SHA-256 matches: `0fc966b1c8790c7d2a49e8654f5f088cafb06fbd4a45a0b77ea3017aed1a5963` |

For every scene, the capture runner checked the runtime summary, HostDocument revision, computed paragraph `letterSpacing`, text layout and line advances, intrinsic min/max contributions, item frame, and final Columns/Rows track sizes. Each per-scene log preserves the chunked `grid_track_sizing_diagnostics` trace. See the Android [matrix](android-avd/matrix.tsv) and [simulator metadata](android-avd/simulator.json), plus the iOS [matrix](ios-simulator/matrix.tsv) and [simulator metadata](ios-simulator/simulator.json).

The [Android screenshot](android-avd/representative-screen.png) and [iOS screenshot](ios-simulator/representative-screen.png) show the fixture route and `layout=ready` status, but their renderer area is black. They are run-context captures, not glyph-pixel evidence. This run did not perform Android PixelCopy or an iOS drawable readback, and it does not establish compositor output, physical-device behavior, or hardware-GPU behavior. The Android AVD uses a software-rendered emulator path.

## Reproduction

```sh
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android
node tools/css-reference/capture-c11-4-computed-letter-spacing-simulators.mjs android-avd emulator-5554

mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:ios-sim
node tools/css-reference/capture-c11-4-computed-letter-spacing-simulators.mjs ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB
```

The runner stopped the Android app after its matrix and terminated the iOS app after its matrix. The iOS Simulator itself was left booted.

## Remaining boundaries

The opt-in profile does not add general CSS admission, arbitrary fonts or scripts, inline spans, bidi, vertical writing, relative or calculated values, WPT conformance, or full C11.4/C11 support. The default-versus-fixture static archive boundary passed; see the [archive evidence](artifact-boundary/README.md). Focused Rust, FFI, fixture-boundary, simulator checks, and `cargo check --locked --workspace --all-targets` passed. The [20-point implementation/runtime review](implementation-runtime-review-20.md) and [30-point PR diff review](pr-diff-review-30.md) are recorded. `cargo fmt --all -- --check` passed. Targeted Clippy passed after allowing only `clippy::suspicious-else-formatting` at unchanged `runtime_layout.rs:960`; strict `-D warnings` without that allow stopped on this existing formatting lint.
