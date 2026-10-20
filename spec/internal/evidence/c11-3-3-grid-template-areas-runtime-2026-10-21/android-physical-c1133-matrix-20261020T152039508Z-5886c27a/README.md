# C11.3.3 Grid template areas Android runtime matrix

- Run: `c1133-matrix-20261020T152039508Z-5886c27a`
- Target: Android 실기기 SM-S731N / Android 16 (API 36); renderer: GLES: Samsung Electronics Co., Ltd., ANGLE ((Samsung Xclipse 940) on Vulkan 1.3.279), OpenGL ES 3.2 Driver version: 24.1.307 git hash: 86f99cfe6317
- 공개 evidence의 ADB serial은 가렸다.
- Runtime matrix: 26 scenes / 65 nodes; 19 success, 5 cascade fallback, 2 expected rejection.
- Chrome DPR 1 geometry: 61 visible node frames compared; maximum error 0 CSS px.
- Basic scene PixelCopy: 2/2 exact; max channel error 0.
- Fixed unknown-area overflow: 3 Chrome geometry nodes; 3/3 exact PixelCopy samples, including the red item outside the grid root.
- Negative controls: pixel samples rejected-before-readback; unknown scene rejected-before-runtime-draw.
- 이 결과는 기록한 단일 실기기·OS·renderer에 한정되며 다른 실기기·driver, WPT 적합성, 일반 CSS Grid 지원을 입증하지 않습니다.

## Files

- `runtime-*.log`: per-scene runtime, geometry, rejection and draw evidence
- `result.json`: aggregate fixture, geometry and negative-control result
- `pixelcopy.png`: basic scene Android SurfaceView PixelCopy
- `unknown-area-overflow-pixelcopy.png`: fixed implicit-track overflow SurfaceView PixelCopy
- Physical-device full-screen capture was omitted; SurfaceView PixelCopy files above contain only the test scene.
