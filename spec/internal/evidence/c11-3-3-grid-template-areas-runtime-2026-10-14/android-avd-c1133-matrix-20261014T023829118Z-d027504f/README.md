# C11.3.3 Grid template areas Android runtime matrix

- Run: `c1133-matrix-20261014T023829118Z-d027504f`
- Target: Android API 37 AVD with SwiftShader
- Runtime matrix: 26 scenes / 65 nodes; 19 success, 5 cascade fallback, 2 expected rejection.
- Chrome DPR 1 geometry: 61 visible node frames compared; maximum error 0 CSS px.
- Basic scene PixelCopy: 2/2 exact; max channel error 0.
- Negative controls: pixel samples rejected-before-readback; unknown scene rejected-before-runtime-draw.
- AVD evidence does not establish physical-device or hardware-GPU behavior, WPT conformance, or general CSS Grid support.

## Files

- `runtime-*.log`: per-scene runtime, geometry, rejection and draw evidence
- `result.json`: aggregate fixture, geometry and negative-control result
- `pixelcopy.png`: basic scene Android SurfaceView PixelCopy
- `android-screen.png`: final AVD screen capture

