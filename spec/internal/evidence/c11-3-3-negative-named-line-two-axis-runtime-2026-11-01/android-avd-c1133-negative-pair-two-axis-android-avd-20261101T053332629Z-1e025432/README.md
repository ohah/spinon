# C11.3.3 양 축 negative leading named-line pair 모바일 실행

- 실행 ID: `c1133-negative-pair-two-axis-android-avd-20261101T053332629Z-1e025432`
- 대상: `android-avd`
- Chrome 기준: HeadlessChrome/156.0.0.0 · DPR 1 · `negative-named-line-pair-two-axis-matrix`
- runtime: 성공 scene 1개 · element frame 13개
- 성공 geometry: root 320×240 CSS px, item 18×12 CSS px at (200, 120)
- 표면 표본: 6/6 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- scene allowlist negative control: rejected-before-runtime-draw
- 표면 비교는 Android API 37 AVD의 SurfaceView PixelCopy다. SwiftShader software renderer 결과이며 실기기·hardware GPU 검증을 대체하지 않는다.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `pixelcopy.png`: PixelCopy 표면 PNG
- `android-screen.png`: Android 화면
- `result.json`: fixture·oracle·앱 hash와 실행 결과
