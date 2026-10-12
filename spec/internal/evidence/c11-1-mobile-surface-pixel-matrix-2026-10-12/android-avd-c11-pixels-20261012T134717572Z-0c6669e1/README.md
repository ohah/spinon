# C11.1 모바일 표면 픽셀 비교

- 상태: 10/10 scene · 36/36 sample · RGBA exact · 최대 채널 오차 0
- 실행 ID: c11-pixels-20261012T134717572Z-0c6669e1
- 대상: {"serial":"emulator-5554","model":"sdk_gphone16k_arm64","apiLevel":37,"qemu":true,"renderer":"GLES: Google (Google Inc. (Google)), Android Emulator OpenGL ES Translator (ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (LLVM 10.0.0) (0x0000C0DE)), SwiftShader driver-5.0.0)), OpenGL ES 3.1 (OpenGL ES 3.1.0 (ANGLE 2.1.1 git hash: fbf66f49c7cc))"}
- renderer: Android SurfaceView PixelCopy; AVD software/virtual GPU path
- oracle: Chrome Chrome/154.0.8037.98 · chromium-darwin-arm64-Chrome-154.0.8037.98-d9996da8123a-83fc71ce41b4-ca4062baab2b · DPR 1
- Android는 PixelCopy PNG 자체의 표본 픽셀·SHA-256과 로그를 교차 확인했다. iOS는 Metal drawable readback 로그를 고정 Chrome 표본과 비교했다.
- 이 결과는 고정 C11.1 10 scene의 GPU surface 표본만 다룬다. 최종 화면 합성, 실기기 GPU, Spinon WPT 적합성, 전체 CSS 지원을 의미하지 않는다.
