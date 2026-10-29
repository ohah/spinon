# C11.4 auto-repeat zero fixed maximum runtime evidence

## Scope

This restricted opt-in fixture slice covers a single repeated track `minmax(auto | min-content | max-content, 0px)`. It crosses columns/rows, `auto-fill`/`auto-fit`, and 0px/4px gap for 24 fixed scenes. The repeated axis is definite 16px; a 12×12px item with a 12px minimum is placed in the third repeated track. No author-CSS admission or public API was added.

The independent auto-repeat floor uses 1px when the fixed maximum is zero. The largest positive `n` satisfying `n × 1px + (n − 1) × gap ≤ 16px` is 16 with a zero gap and 4 with a 4px gap. The occupied third track retains the item minimum. `auto-fit` collapses the empty repeated tracks and adjacent gutters; for a 4px gap `auto-fill` places the item at 8px and the item extends 4px outside the 16px root.

## Verification

The 24 scene catalog is compared to the pinned Chrome 156 observations at [the precomparison](../c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/README.md). Layout, FFI routing, and vendored Taffy focused tests passed, as did the 22-test fixture boundary suite, `cargo fmt --all -- --check`, and `git diff --check`.

The final runner requires an explicit scene list for `zero-fixed-maximum`; an unselected run is rejected by the boundary test. The final Android and iOS captures used the exact 24 scene IDs from the fixture JSON:

| Target | Runtime | Scenes | Track trace chunks | Exact surface samples | Screenshots | Installed app hash |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Android | API 37 ARM64 AVD, `emulator-5556`, QEMU | 24/24 | 264 | PixelCopy 54/54 | 24 | matched |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator | 24/24 | 516 | Metal drawable 54/54 | 24 | matched |

The machine-readable result and sanitized source receipts are recorded in `verification.json`, `simulator-source-revisions.json`, and `surface-samples.json`. The two run directories named in `verification.json` contain structured per-scene observations, 24 PNGs each, screenshot hash manifests, and sanitized device/build metadata. The Android AVD is an emulator, not a physical device or hardware-GPU result. iOS Metal samples are read before OS compositor presentation; the captured Simulator screenshots are retained separately and do not change that pixel-sample boundary.

Representative `auto-fill`, 4px-gap overflow captures are `android-avd-c114-zero-fixed-maximum-android-avd-20261029T084323971Z-d7251a9d/columns-auto-fill-auto-max0-gap4.png` and `ios-simulator-c114-zero-fixed-maximum-ios-simulator-20261029T084208665Z-aa4ce7f8/columns-auto-fill-auto-max0-gap4.png`. The implementation PR body embeds both images directly.

## Focused checks

```text
cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test zero_fixed_maximum_auto_repeat_matches_all_chrome_precomparison_scenes -- --nocapture  # 1 passed
cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test zero_fixed_maximum_auto_repeat_catalog_matches_chrome_and_routes_exactly -- --nocapture  # 1 passed
cargo test --manifest-path vendor/taffy/Cargo.toml auto_repeat_count_floors_zero_fixed_maximum_for_intrinsic_minima_on_both_axes -- --nocapture  # 1 passed
node --test tools/css-reference/fixture-module-boundary.test.mjs  # 22 passed
cargo fmt --all -- --check  # passed
git diff --check  # passed
```

The Taffy regression checks both axes and all three intrinsic minima. A 2px maximum is a positive control with counts 8/3 for gaps 0/4; zero maximum counts are 16/4. Existing vendored Taffy unused-variable warnings remain outside this slice.

## Fixture artifact boundary

`artifact-boundary.json` records SHA-256 and byte counts for default and fixture-enabled products. The new scene IDs are absent from the default host static archive, Android APK, and iOS Simulator executable; they are present in each corresponding fixture-test positive control. This is evidence for those inspected artifacts only and is not a claim about every packaging format.

Android and iOS default/fixture Simulator builds succeeded. The initial iOS fixture build exposed a missing explicit Swift `return` in the new frame helper; the helper was corrected, rebuilt, and the corrected fixture app passed all 24 scenes.

## Remaining scope

This is one fixed fixture profile. General author CSS parsing/admission, arbitrary track lists and gap values, indefinite sizing axes, the full auto-repeat matrix, the complete C11.4 parent, all CSS Grid WPTs, a physical Android device, Android hardware GPU, and iOS post-compositor pixel equality remain unverified or unsupported. Keep C11.4 unchecked and the official completion total at `72/213`.
