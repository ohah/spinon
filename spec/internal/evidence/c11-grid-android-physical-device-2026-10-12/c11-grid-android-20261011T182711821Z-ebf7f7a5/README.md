# C11.1 Android 실기기 V8→WGPU 검증

- 실행 ID: c11-grid-android-20261011T182711821Z-ebf7f7a5
- 기기: SM-S731N · Android 16/API 36 · serial R5KYB06SMPE
- APK SHA-256: `63d94864fd58d56e972f2e8dc96ff02b60970f02244bf96103037620e8d0a428`
- backend: Samsung Xclipse 940 / Vulkan
- V8 runtime host와 C11.1 scene 평가가 성공했고, 5개 node의 frame을 pinned Chrome 154 기준과 비교했다.
- `SurfaceView`를 `PixelCopy`로 읽은 7/7 표본이 CSS 기대 RGBA와 완전히 일치했다.
- [기기 화면](./device-screen.png) · [surface PixelCopy](./surface-pixelcopy.png) · [원본 로그](./runtime.log) · [비교 JSON](./comparison.json)
- C11.1 fixed-track 한 장면만 검증했다. iOS Metal 직접 readback, 나머지 Chromium scene, 공식 WPT 적합성은 별도 관문이다.
