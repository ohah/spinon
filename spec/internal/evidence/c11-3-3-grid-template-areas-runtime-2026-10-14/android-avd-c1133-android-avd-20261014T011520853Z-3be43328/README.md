# C11.3.3 Grid template area 모바일 실행

- 실행 ID: `c1133-android-avd-20261014T011520853Z-3be43328`
- 대상: `android-avd`
- Chrome 기준: Chrome/154.0.8037.92 · DPR 1 · `basic-rectangles`
- runtime: 성공 장면 1개 · 기대 거부 장면 1개 · node 4개
- 성공 geometry: root 160×120 CSS px, item 120×40 CSS px at (40, 40)
- 표면 표본: 2/2 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- 표면 비교는 Android API 37 AVD의 SurfaceView PixelCopy다. SwiftShader software renderer 결과이며 실기기·hardware GPU 검증을 대체하지 않는다.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `pixelcopy.png`: PixelCopy 표면 PNG
- `android-screen.png`: Android 화면
- `result.json`: fixture·oracle·앱 hash와 실행 결과

