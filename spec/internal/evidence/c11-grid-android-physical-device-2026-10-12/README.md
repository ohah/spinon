# C11.1 Android 실기기 최신 검증

## 2026-10-12 실기기 재연결 후 현재 소스 행렬

- 실행 ID: `c11-grid-android-matrix-20261011T214743478Z-072dc5a1`
- 기기: Samsung SM-S731N · Android 16/API 36 · `ro.kernel.qemu=0`
- 그래픽: WGPU Vulkan · Samsung Xclipse 940 · SurfaceView
- APK SHA-256: `bc588a8af11d7ed887767fece32a890ac63915b98de07736d8c6b384438555e9`
- 고정 Chrome 154 기준 10/10 scene·40/40 data node가 통과했고, 모든 frame 좌표가 ±0.5 CSS px 기준을 만족했다.
- `fixed-tracks-gap`의 SurfaceView PixelCopy는 7/7 RGBA 표본이 exact였다. 나머지 9개 scene은 geometry와 화면 캡처만 확인했다.
- [비교 JSON](./c11-grid-android-matrix-20261011T214743478Z-072dc5a1/comparison.json) · [PixelCopy 화면](./c11-grid-android-matrix-20261011T214743478Z-072dc5a1/fixed-tracks-gap-surface-pixelcopy.png) · [전체 실행 로그·화면](./c11-grid-android-matrix-20261011T214743478Z-072dc5a1/)

- 기기: Samsung SM-S731N · Android 16/API 36 · `ro.kernel.qemu=0`
- 그래픽: WGPU Vulkan · Samsung Xclipse 940 · `SurfaceView`
- Android APK SHA-256: `bc588a8af11d7ed887767fece32a890ac63915b98de07736d8c6b384438555e9`
- 기준: 고정 Chrome 154.0.8037.98 reference · fixture/reference hash는 각 실행 JSON에 보존했다.

| 실행 | 결과 | 근거 |
| --- | --- | --- |
| C11.1 geometry 행렬 | 10 scene·40 node 전체 통과, 각 좌표 오차 0.5 CSS px 이하 | [실행 요약](./c11-grid-android-matrix-20261011T210223559Z-780ee094/README.md) · [비교 JSON](./c11-grid-android-matrix-20261011T210223559Z-780ee094/comparison.json) |
| 고정 장면 표면 표본 | `fixed-tracks-gap` 7/7 RGBA exact | [전용 PixelCopy 실행](./c11-grid-android-20261011T210208642Z-0592dff3/README.md) · [비교 JSON](./c11-grid-android-20261011T210208642Z-0592dff3/comparison.json) · [PixelCopy PNG](./c11-grid-android-matrix-20261011T210223559Z-780ee094/fixed-tracks-gap-surface-pixelcopy.png) |

두 검사는 별도 앱 실행이다. 10-scene 행렬은 geometry와 화면 PNG를 보존하지만 모든 장면의 pixel equality를 측정하지 않았다. 고정 장면 PixelCopy는 화면 최종 compositor 출력이 아니라 Android `SurfaceView` 표면을 읽었다. 이 근거는 다른 Android 기기·release 성능·전체 CSS·Spinon WPT 적합성을 뜻하지 않는다.
