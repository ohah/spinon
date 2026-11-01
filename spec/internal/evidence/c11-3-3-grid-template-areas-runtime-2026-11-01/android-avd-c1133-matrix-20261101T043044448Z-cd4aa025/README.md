# C11.3.3 Grid template areas Android runtime matrix

- Run: `c1133-matrix-20261101T043044448Z-cd4aa025`
- Target: Android API 37 AVD with SwiftShader
- Source commit: `0096e39d59988abb60768129a5cdada0ec077497`; Android Java SHA-256: `a115ff8e84d8e320de1cbc8bb4a7cf5e158ee64462704b2eaff8f0883a31596a`; APK SHA-256: `ce82bd8b32a78514ba2230422e577772b530ea9c03002239d8d38d7266928f2d`.
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
