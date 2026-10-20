# C04.12 root HTML page scale simulator pixel analysis

The native app route rendered the same root HTML fixture through the Android WGPU surface and iOS Metal drawable. The fixture source SHA-256 is `3516b552a76777632679ca1554ccceb14c1ac52548f7ddd264a67689fd744829` on all three bundled copies. The scale cases replace only the viewport meta value; the resulting app input hashes match the Chrome oracle record.

## Surface and environment

| Platform | Simulator | Screenshot | CSS surface | Physical surface | DPR |
| --- | --- | ---: | ---: | ---: | ---: |
| Android | API 37 ARM64 AVD `spinon_api37_1_compat16k`, density 420 | `1080×2400` | `320×240` | `840×630` | `2.625` |
| iOS | iPhone 17 Pro Max, iOS 26.2 Simulator | `1320×2868` | `320×240` | `960×720` | `3` |

The measured `#112233` surface bounds are Android screenshot coordinates `(120,1011)-(960,1641)` and iOS `(180,1074)-(1140,1794)`. Each bound is exactly the expected physical surface size. At a sample point corresponding to CSS `(10,10)`, all six screenshots contain exact RGB `(17,34,51)`, confirming the Canvas clear color is present inside the surface.

## Scale and pixel bounds

Bounds are half-open physical pixel coordinates relative to the measured surface origin. `page-scale-probe` is the magenta CSS box `(100,50)` size `20×10`; `surface-edge-probe` is the green box `(200,60)` size `20×10`.

| Platform | Case | Layout viewport / visual viewport | Physical scale | Magenta observed | Green observed |
| --- | --- | ---: | ---: | ---: | ---: |
| Android | `initial-scale=0.5` | `640×480 / 640×480` | `1.3125` | `[131,157)×[66,79)` | `[262,289)×[79,92)` |
| Android | `initial-scale=2` | `320×240 / 160×120` | `5.25` | `[525,630)×[262,315)` | clipped outside surface |
| Android | `width=500, initial-scale=1` | `500×375 / 320×240` | `2.625` | `[262,315)×[131,157)` | `[525,577)×[157,184)` |
| iOS | `initial-scale=0.5` | `640×480 / 640×480` | `1.5` | `[150,180)×[75,90)` | `[300,330)×[90,105)` |
| iOS | `initial-scale=2` | `320×240 / 160×120` | `6` | `[600,720)×[300,360)` | clipped outside surface |
| iOS | `width=500, initial-scale=1` | `500×375 / 320×240` | `3` | `[300,360)×[150,180)` | `[600,660)×[180,210)` |

The integer iOS bounds match exactly. Android fractional edge coordinates match the expected pixel-center coverage. At scale 2, the green control begins beyond the physical surface width, so its absence confirms clipping at the surface edge.

## HTML input hashes and captures

| Input | SHA-256 | Android capture | iOS capture |
| --- | --- | --- | --- |
| Default app HTML | `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85` | [default.png](./android/default.png) | [default.png](./ios/default.png) |
| `initial-scale=0.5` | `de90f7a36cd0ceae2869204601632c1662bf49ed438fb2d06679a814870c2fab` | [scale-half.png](./android/scale-half.png) | [scale-half.png](./ios/scale-half.png) |
| `initial-scale=2` | `fe7368388fbeb38a69d4fabce218568735cee890bbcb890340c8c183f75246bc` | [scale-double.png](./android/scale-double.png) | [scale-double.png](./ios/scale-double.png) |
| `width=500, initial-scale=1` | `028471f1dbd807679ef455b21ccf4b5fc832e9131a1d2e84a33790059f8db097` | [scale-fixed-width.png](./android/scale-fixed-width.png) | [scale-fixed-width.png](./ios/scale-fixed-width.png) |

Per-process app logs and launch PIDs are saved beside each capture. All six final environment reports have `status=0`, `environment_revision=0`, the expected DPR and page scale, and a draw report for the full physical surface.

Each non-default iOS case also records one initial `status=-12` result. The view reports its first drawable size, increments the surface generation, and configures the Metal surface while the first environment request is still pending. That request is correctly rejected as stale; the current request then reports `status=0`, and its draw uses the expected environment. The original per-process logs retain both the cancellation and successful replacement.

## Evidence limits

This measures the simulator's captured app surface. It does not establish physical-device behavior, Android hardware GPU behavior, iOS compositor-submitted pixels, input coordinate remapping, pinch/pan, or general HTML/CSS parity.

## Build and focused Rust checks

| Command | Result |
| --- | --- |
| `mise exec -- cargo fmt --all -- --check` | pass |
| `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test root_html_page_scale` | 5 passed, 0 failed |
| `mise exec -- cargo test --locked -p spinon-render page_scale` | 1 passed, 0 failed |
| `mise exec -- cargo test --locked -p spinon-render-wgpu page_scale` | 6 passed, 0 failed; includes image UV, glyph clip, border/opacity, and physical-edge readback cases |
| `mise exec -- cargo build --locked -p spinon-ffi --features c04-runtime-gpu --lib` | pass with product feature; fixture-test features are off in app builds |
| `mise exec -- cargo clippy --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib -- -D warnings` | pass; removed a needless explicit lifetime from an existing C11.4 helper exposed by this Clippy version |
| `mise exec -- bun run build:android` | `BUILD SUCCESSFUL`; API 37.2 compatibility advisory from Android Gradle Plugin 8.13.2, which was tested through 36.1 |
| `mise exec -- bun run build:ios-sim` | `BUILD SUCCEEDED`; existing duplicate `RawWindowMetalLayer` linker and V8 debug-map warnings remain |

The Android and iOS app builds set `c04_runtime_gpu=1` and `c04_runtime_gpu_fixture_test=0`. The screenshot runs used only the Android API 37 AVD and iOS 26.2 Simulator; no physical device was attached.
