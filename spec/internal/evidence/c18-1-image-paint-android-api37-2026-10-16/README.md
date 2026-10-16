# C18.1 Android API 37 AVD — 2026-10-16

## 실행 환경

- 대상: Android API 37 ARM64 AVD, `emulator-5554`; `physicalDevice=false`.
- renderer: OpenGL ES / ANGLE SwiftShader.
- fixture route run ID: `636ee4a82a20`, 앱 PID `28792`.
- opt-in APK SHA-256: `5ccbbf60c918a33c985b4a6ca7f68682c453d60bf5bdf590e72ec6f3d112e3cf`; 내부 `libspinon_bootstrap.so` SHA-256: `a99e17063d472ca1b423719e71f9f99809a3be32b6995a65f78434e7d3b11e7f`.
- Android SurfaceView: `210×105` px, CSS viewport `80×40`, scale `2.625×2.625`, generation `2`.

## 결과

PixelCopy는 PNG를 저장했고, 고정 9개 CSS 좌표의 RGBA를 모두 exact 비교했다. 전체 화면 캡처는 `1080×2400` px다. post-PixelCopy status 이후 SurfaceView screen origin은 `(435,1278)` px였으며, 9개 대응 screen pixel과 fixture 바깥 네 색상 anchor도 exact였다. comparator 결과는 `matched`, surface 표본 `9/9`, screenshot 표본 `9/9`, anchor `4/4`다.

| 근거 | 파일 | SHA-256 |
| --- | --- | --- |
| AVD 전체 화면 | [full-screen.png](./full-screen.png) | `64e9d0ce1aa957bbeb067a3aaad4b293b103a8ed6793bb2cc263f9d7d3c86232` |
| SurfaceView PixelCopy | [pixelcopy-surface.png](./pixelcopy-surface.png) | `df72839a24c84c69e28ae11c9a2f810ed2cd9717c29bad5a82d125cabad5969b` |
| 현재 PID 원본 로그 | [logcat.txt](./logcat.txt) | `c6dcb0e87926317b168de3aeb2b282aaa36ca2217139382026b80aa5618ab657` |
| machine-readable mapping | [post-compositor-observation.json](./post-compositor-observation.json) | — |

비교 명령은 `node tools/css-reference/compare-c181-compositor-pixels.mjs spec/internal/evidence/c18-1-image-paint-android-api37-2026-10-16/post-compositor-observation.json`이다. 로그는 PID `28792`만 담아 이번 실행의 draw와 PixelCopy 표본을 확인했다.

이 증거는 지정 AVD의 virtual GPU와 Android PixelCopy/전체 화면 캡처에 한정한다. 실기기, 하드웨어 GPU, 다른 Android API와 제품 image loader는 검사하지 않았다.
