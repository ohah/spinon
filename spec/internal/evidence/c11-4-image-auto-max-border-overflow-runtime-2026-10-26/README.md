# C11.4 image auto/max border overflow runtime evidence

이 evidence는 iOS Simulator에서 수행한 최초 capture를 기록한다. 해당 capture 당시 앱 fixture의 root 바깥 표본 기대값은 Chrome body background `[16,24,32,255]`로 잘못 설정되어 있었고, 따라서 run은 최종 pixel gate에서 실패했다. 이 실패를 숨기거나 성공으로 재분류하지 않는다. Geometry, track trace, item marker 및 두 border 표본은 통과했다.

## iOS 최초 capture

- Device: task-owned iPhone 17 Pro clone, iOS 26.2 Simulator, UDID `C9CD3A32-14FB-4013-A16D-4A029370DA3F`.
- Run evidence: [`ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T055257971Z-2e01772c`](./ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T055257971Z-2e01772c/).
- App surface: 960×720 drawable over a 320×240 CSS viewport, scale 3×, origin `(0,0)`.
- `image-auto-max-mixed-tracks` control scene passed and its screenshot was saved.
- Border-overflow scene returned root `12×40`, item `(1,1,24,16)`, columns `[24,17]`, row `[16]`, and `image_draw_count=0`.
- Item marker at CSS center `(12.5,8.5)` was `[229,72,77,255]`; left and right border samples were `[34,197,94,255]`.
- Root-exterior CSS center `(26.5,8.5)` returned `[0,0,0,255]`. The runner expected `[16,24,32,255]`, so the recorded result is `matched_samples=3/4` and `status=mismatch`.
- The original log is preserved at [`runtime-image-auto-max-mixed-tracks-border-overflow.log`](./ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T055257971Z-2e01772c/runtime-image-auto-max-mixed-tracks-border-overflow.log). No screenshot was emitted for the failed scene because the runner saves it only after the pixel gate passes.

## Surface-clear source path

`crates/spinon-ffi/src/runtime_gpu/renderer.rs::draw` selects opaque black `[0,0,0,255]` whenever `root_html_default_render` is false, then passes that color to `draw_if_with_srgb_clear_color`. The C11.4 native fixture uses this path. The root HTML default-render path can use `root_html_canvas_color`; that is a different mode and does not set the C11.4 fixture's whole-surface backdrop.

Android uses the same native runtime GPU draw path. Its capture adapter obtains a bitmap using `PixelCopy.request(surfaceView, ...)` and samples the requested surface coordinates from that bitmap in `SurfaceViewPixelCapture`. The API 37 AVD rerun below observed the same opaque-black clear sample exactly; this is simulator evidence and does not establish physical-device behavior.

No fixture-level API to configure a whole-surface backdrop was found in the C11.4 fixture path. This correction changes the fixture's outside-root expected sample only; it does not change the renderer clear behavior or the Chrome HTML oracle. Chrome's body background at the same coordinate remains `[16,24,32,255]`.

## Corrected plan gate

Root independently passed the plan correction at SHA-1 `30922642f0bd8c6763742457699222f20010723e`. The iOS, Android and runner app expectations use opaque black `[0,0,0,255]` for the outside-root surface sample; the Chrome body sample remains `[16,24,32,255]`. The plan correction review is [plan-correction-review-20.md](../c11-4-image-auto-max-border-overflow-plan-correction-review-2026-10-26/plan-correction-review-20.md), its source-only implementation check is [surface-clear-correction-review-20.md](../c11-4-image-auto-max-border-overflow-implementation-review-2026-10-26/surface-clear-correction-review-20.md), and the final Android adapter/runtime review is [android-pixelcopy-scene-allowlist-review-20.md](../c11-4-image-auto-max-border-overflow-implementation-review-2026-10-26/android-pixelcopy-scene-allowlist-review-20.md). Earlier pending-runtime statements in the precomparison and initial implementation reviews are time-bounded to those review snapshots.

## Corrected iOS Simulator rerun

- Build: `mise exec -- bun run build:ios-sim` with `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`, and the existing V8 checkout pinned at `7b50b62cb18f28617959e8452e2cd18195b38bcf`. The build passed; its log is [`ios-build-correction-final-20261026.log`](./ios-build-correction-final-20261026.log). Xcode reported existing Swift deprecation/duplicate-symbol warnings, but exited successfully.
- App executable SHA-256: `5ba470a7664714c792318ade97225ed7f1a6ce898d94210c5ee59573d245ab87`.
- Device: iPhone 17 Pro simulator model `iPhone18,1`, iOS 26.2, UDID `C9CD3A32-14FB-4013-A16D-4A029370DA3F`.
- Run evidence: [`ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T061945593Z-ec40c8ba`](./ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T061945593Z-ec40c8ba/).
- Both the `image-auto-max-mixed-tracks` control and `image-auto-max-mixed-tracks-border-overflow` scene passed. The latter returned root `12×40`, item `(1,1,24,16)`, columns `[24,17]`, row `[16]`, and `image_draw_count=0`.
- Metal drawable samples matched 4/4 exactly: item marker `[229,72,77,255]`, outside-root surface clear `[0,0,0,255]`, and left/right borders `[34,197,94,255]`. This readback occurs before compositor submission.
- The captured iPhone screenshot is [`image-auto-max-mixed-tracks-border-overflow.png`](./ios-simulator-c114-image-auto-max-border-overflow-ios-simulator-20261026T061945593Z-ec40c8ba/image-auto-max-mixed-tracks-border-overflow.png), SHA-256 `70c9cb5de7307db194a2e4938063e696c6e20c9f2cbb1e72dfda8ab0def63ada`.

## Android API 37 AVD rerun

- Build: `mise exec -- bun run build:android` with `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`, `CARGO_BUILD_JOBS=1`, and the V8 checkout pinned at `7b50b62cb18f28617959e8452e2cd18195b38bcf`. The build passed; its log is [`android-build-pixel-matrix-fix-20261026.log`](./android-build-pixel-matrix-fix-20261026.log). Gradle emitted the existing compile SDK 37.2 compatibility and Java deprecation warnings.
- Device: C11.4 Pixel 6 AVD `spinon_c114_cyclic_image_span_20261023`, API 37.0, ARM64, emulator serial `emulator-5556`. It cold-booted because its saved `default_boot` snapshot could not be loaded; the run used `-no-snapshot-save`.
- The first Android attempt passed the control scene but the border-overflow scene stopped at `scene-index-mismatch`: the Android PixelCopy adapter's scene allowlist lacked the two new C11.4 image scenes. The adapter now explicitly permits those scene IDs. The failed attempt remains in [`android-avd-c114-image-auto-max-border-overflow-android-avd-20261026T063019632Z-3316b3e4`](./android-avd-c114-image-auto-max-border-overflow-android-avd-20261026T063019632Z-3316b3e4/); it is not counted as a pass.
- APK and installed APK SHA-256: `f2ceeaa7cfc56bb732b4eca6d6cc6aa9dd4f8d73a2ee7fcb86c915d0ec5c3b8a`.
- Corrected run evidence: [`android-avd-c114-image-auto-max-border-overflow-android-avd-20261026T063238848Z-970016a2`](./android-avd-c114-image-auto-max-border-overflow-android-avd-20261026T063238848Z-970016a2/). The control and border-overflow scenes both passed, with six track-trace chunks and `drawPresented=true` for each.
- Border-overflow geometry was root `12×40`, item `(1,1,24,16)`, columns `[24,17]`, and row `[16]`; `image_draw_count=0`. PixelCopy matched 4/4 exact samples: item `[229,72,77,255]`, outside-root clear `[0,0,0,255]`, and both borders `[34,197,94,255]`.
- The captured Android screenshot is [`image-auto-max-mixed-tracks-border-overflow.png`](./android-avd-c114-image-auto-max-border-overflow-android-avd-20261026T063238848Z-970016a2/image-auto-max-mixed-tracks-border-overflow.png), SHA-256 `02203d1eb3377c9504491eb4917917580bb16e09de80ef2ba77614e39415a3f5`.
- PR #652 embeds both platform screenshots using the pushed branch's raw image URLs. Agent Browser loaded them at their native dimensions and displayed them in the PR body; [render/close verification](./pr-652-agent-browser-verification.json) and [page screenshot](./pr-652-agent-browser.png) are preserved.

The original iOS `3/4` mismatch remains preserved as the oracle correction history. Corrected runtime captures now pass on both iOS Simulator and Android API 37 AVD; these results do not establish physical-device behavior. The official C11.4 status and `72/213` count remain unchanged.
