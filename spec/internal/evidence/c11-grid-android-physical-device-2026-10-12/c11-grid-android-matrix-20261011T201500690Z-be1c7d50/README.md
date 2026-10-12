# C11.1 Android 실기기 10-scene runtime 비교

- 실행 ID: c11-grid-android-matrix-20261011T201500690Z-be1c7d50
- 기기: samsung SM-S731N · Android 16/API 36
- 그래픽: GLES: Samsung Electronics Co., Ltd., ANGLE ((Samsung Xclipse 940) on Vulkan 1.3.279), OpenGL ES 3.2 Driver version: 24.1.307 git hash: 86f99cfe6317
- APK SHA-256: `f1bef1b61acde8ae3a6a202bc453414fd19855d090ed7a7ed7b3646010964b35`
- 고정 Chrome 154 reference의 10개 scene·40개 node를 scene별 새 앱 프로세스에서 실행했다.
- 모든 node의 x/y/width/height를 CSS px 기준 ±0.5 이내로 비교했다.
- `fixed-tracks-gap`의 Android `SurfaceView` PixelCopy는 7/7 표본 RGBA가 exact였다.
- 다른 9개 scene은 frame geometry와 device screenshot만 수집했으며, 전체 scene pixel equality를 주장하지 않는다.
- [상세 비교](./comparison.json) · 실행별 `*-runtime.log`, 화면 PNG, build/install 로그를 같은 디렉터리에 보존했다.
- iOS Simulator, WPT, hardware GPU 외 기기군 및 전체 CSS 지원은 별도 관문이다.
