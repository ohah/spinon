# C11.3.3 Grid template areas Android runtime matrix

- Run: `c1133-matrix-20261031T075025965Z-f6357ddc`
- Target: Android API 37 AVD with SwiftShader
- Source commit: `7bcc6a6641a2d95e607a6f805578a19a5fa84f0a`; Android Java SHA-256: `a88f0caa630310a5c89354ce5567d49d9d859b42ecd054adcc26e93946abef9c`; APK SHA-256: `16fb68df6a6517eade346629a0534050e0cb3c2997fd2deae2294b70a9a5984b`.
- Runtime matrix: 39 scenes / 126 nodes; 34 success, 5 cascade fallback, 0 expected rejection.
- Chrome DPR 1 geometry: 126 visible node frames compared; maximum error 0 CSS px.
- Basic scene PixelCopy: 2/2 exact; max channel error 0.
- Track-list cycling PixelCopy: 6 scenes / 28 RGBA samples exact; max channel error 0.
- Typed auto-track PixelCopy: 2 scenes / 7 RGBA samples exact; max channel error 0.
- Fixed unknown-area overflow: 3 Chrome geometry nodes; 3/3 exact PixelCopy samples, including the red item outside the grid root.
- Leading named implicit lines: 7 scenes / 14 visible nodes; maximum geometry error 0 CSS px; 7/7 PixelCopy samples exact.
- Negative controls: pixel samples rejected-before-readback; unknown scene rejected-before-runtime-draw.
- AVD evidence does not establish physical-device or hardware-GPU behavior, WPT conformance, or general CSS Grid support.

## Files

- `runtime-*.log`: per-scene runtime, geometry, rejection and draw evidence
- `result.json`: aggregate fixture, geometry and negative-control result
- `pixelcopy.png`: basic scene Android SurfaceView PixelCopy
- `pixelcopy-*.png`: track-list cycling Android SurfaceView PixelCopy samples
- `pixelcopy-typed-area-*.png`: typed auto-track Android SurfaceView PixelCopy samples
- `typed-area-*-screen.png`: typed auto-track AVD screen captures
- `unknown-area-overflow-pixelcopy.png`: fixed implicit-track overflow SurfaceView PixelCopy
- `pixelcopy-leading-*.png`: named implicit line SurfaceView PixelCopy samples
- `android-track-list-cycle-screen.png`: both-axis cycling AVD screen capture
- `android-screen.png`: final AVD screen capture
