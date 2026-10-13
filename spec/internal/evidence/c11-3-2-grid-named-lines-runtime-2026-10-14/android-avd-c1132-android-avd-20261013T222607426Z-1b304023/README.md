# C11.3.2 Grid named line 모바일 실행

- 실행 ID: `c1132-android-avd-20261013T222607426Z-1b304023`
- 대상: `android-avd`
- AVD: `spinon_api37_0_applied_txn` · Android 37.0 ARM64 · emulator 37.2.12.0 · serial `emulator-5554`
- 렌더러: ANGLE/Vulkan/SwiftShader · density 2.625 · SurfaceView 840×630 px
- AVD 시작: `$HOME/Library/Android/sdk/emulator/emulator -avd spinon_api37_0_applied_txn -port 5554 -no-snapshot-save -no-boot-anim -no-audio -gpu swiftshader_indirect`
- 캡처: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run css:capture:c11-3-2-grid-named-lines:android-avd`
- Chrome 기준: Chrome/154.0.8037.92 · DPR 1 · `positive-occurrence`
- runtime: 성공 장면 1개 · 기대 거부 장면 1개 · node 4개
- 성공 geometry: root 160×120 CSS px, item 80×40 CSS px at (80, 0)
- 표면 표본: 2/2 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- 표면 비교는 Android API 37 AVD의 SurfaceView PixelCopy다. 소프트웨어 렌더러 결과이며 실기기 검증을 대체하지 않는다.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `build.log`, `install.log`, `launch.log`: 최신 PR source 빌드·설치·실행 로그
- `device.json`: API/QEMU/fingerprint/renderer 메타데이터
- `pixelcopy.png`: PixelCopy 표면 PNG
- `android-screen.png`: Android 화면
- `result.json`: fixture·oracle·앱 hash와 실행 결과
