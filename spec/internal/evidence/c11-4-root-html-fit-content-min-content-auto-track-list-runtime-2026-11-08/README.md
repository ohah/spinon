# C11.4 fit-content/min-content/auto implicit-row list runtime evidence

## Scope

This candidate extends only the existing parsed Root HTML `RootHtmlFitContentAutoRowsV1` profile for the two three-entry lists `fit-content(<finite non-negative px|%>) min-content auto` and `fit-content(<finite non-negative px|%>) auto min-content`. It does not complete C11.4/C11 or establish general CSS Grid admission, WPT conformance, hardware-GPU behavior, or surface pixel parity. Internal contract 0059 remains at `0.1.0`; official status remains `78/214`.

The fixed default-route HTML fixture is [`root-html-grid-fit-content-min-content-auto-track-list-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-track-list-runtime.html), SHA-256 `18f684f61be1484823919fa2895ae7e0e568592808a4d47d243216517935c08c`. The separate Chrome 157 pixel-cap oracle is recorded at [`precomparison/px-cap`](../c11-4-root-html-fit-content-min-content-auto-track-list-precomparison-2026-11-08/px-cap/README.md); the three percentage-based Chrome scenes are at [`precomparison`](../c11-4-root-html-fit-content-min-content-auto-track-list-precomparison-2026-11-08/README.md).

## Rust verification

The implementation has separate style-projection, layout and parsed `RuntimeSession` checks. They verify both orders, finite px and percentage caps, exact typed order and computed serialization, repeated track-list order, every item/payload frame, auto-only positive-space stretch, visible overflow, unsupported nearby lists, failure atomicity, and same-session recovery.

| Command | Result |
| --- | --- |
| `cargo test --locked -p spinon-layout root_html_fit_content --lib` | 11 passed, 0 failed |
| `cargo test --locked -p spinon-style-to-layout root_html_fit_content --lib` | 5 passed, 0 failed |
| `cargo test --locked -p spinon-runtime parsed_root_html_fit_content --lib` | 11 passed, 0 failed |
| `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings` | passed |
| `cargo fmt --all -- --check` | passed |
| `git diff --check` | passed |

Full test output is preserved in [`layout-tests.log`](./verification/layout-tests.log), [`style-tests.log`](./verification/style-tests.log), and [`runtime-tests.log`](./verification/runtime-tests.log). Scoped Clippy, rustfmt and diff-check results are in [`static-checks.txt`](./verification/static-checks.txt). The separate [implementation/runtime review](./implementation-runtime-review-20.md) checks 20 additional failure paths.

## Android API 37 ARM64 AVD

The built-in Android emulator `emulator-5554` launched the default parsed Root HTML route from the fixture. The exact current app PID log records HTML asset/source size 2183 bytes, `layout=ready boxes=12`, `page_scale=1`, and `SPINON_C0410_DRAW status=0 presented boxes=12`. The final 1080×2400 OS screenshot was captured after those records: [`screenshot-final.png`](./android/final/screenshot-final.png), SHA-256 `21cc19805c6bc530af96a88ca93a8b88626bb8a6422871918534e120d3a81e90`. The earlier empty capture is preserved as [`screenshot.png`](./android/final/screenshot.png); it was taken before the app completed layout and is not the final visual evidence.

This was an AVD software-renderer smoke check, not a PixelCopy surface sample or hardware-GPU result. Exact process-filtered logs are in [`app-current-process.log`](./android/final/app-current-process.log), bound to [`current-app-pid.txt`](./android/final/current-app-pid.txt).

## iOS Simulator

The iPhone 17 Pro, iOS 26.2 Simulator (`97E99AA8-D144-4DA8-B1BF-7F6B34007F8B`) launched the same 2183-byte fixture on the default Root HTML route. Its [current-process log](./ios/final/app-current-process.log) records source bytes, `layout=ready boxes=12`, and `page_scale=1`. The 1206×2622 OS screenshot visibly shows the colored Grid output and overflow: [`screenshot.png`](./ios/final/screenshot.png), SHA-256 `3d0e9f7041c5e2a7f6b6a9341e0bd68fa1ef4670dce97d25b09d667722914953`. The iOS log has no draw-presented marker, so this evidence claims layout readiness and screenshot-visible output only. It is not Metal drawable readback or compositor-after pixel parity.

**Data-continuity incident:** the app data-container path changed across `simctl install`. The previously reported path is now missing; the new path exists. No uninstall, simulator erase, or app-data clear command was run, and the cause of the path change is unknown. Preservation of the old app data was not confirmed. [`data-container-summary.txt`](./ios/final/data-container-summary.txt) records the before/after container IDs and state without a machine-local filesystem path.

## Build and source restoration

The staged app artifacts were built with fixture HTML SHA-256 `18f684f61be1484823919fa2895ae7e0e568592808a4d47d243216517935c08c`. Android APK SHA-256: `9784eb96579a0b40c36e7bb25fe55a48ef2ce2dd964d1a82923be6d63949cc3b`. iOS Simulator executable SHA-256: `04fc96a58a82aa366ec134d33afd7cf746fe7af723e749a2d30f1aaaaaf61153`. The original `examples/bootstrap/index.html` and `root-app.js` contents were restored; their SHA-256 values are `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9` and `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`.

The extra px-cap Chrome run used Agent Browser, then closed that browser session. No `agent-browser` process remained after the capture.
