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

Android uses the same native runtime GPU draw path. Its capture adapter obtains a bitmap using `PixelCopy.request(surfaceView, ...)` and samples the requested surface coordinates from that bitmap in `SurfaceViewPixelCapture`. The source path therefore predicts the same clear color, but an API 37 Android Simulator/AVD capture remains required before recording Android runtime evidence.

No fixture-level API to configure a whole-surface backdrop was found in the C11.4 fixture path. This correction changes the fixture's outside-root expected sample only; it does not change the renderer clear behavior or the Chrome HTML oracle. Chrome's body background at the same coordinate remains `[16,24,32,255]`.

## Rerun gate

The app expected values and runner are still awaiting the separate 20-view plan-correction review. Do not rebuild/relaunch C9 or rerun the failed scene until the new plan SHA receives PASS. Preserve the existing iOS app/session while waiting. Android remains unverified until a task-owned API 37 simulator run passes.
