# C11.2.1 현재 소스 모바일 재실행 · 2026-10-12

**목적:** Grid 테스트 모듈 분리 뒤에도 C11.2.1 V8→HostDocument→Stylo→Taffy→WGPU 경로와 surface 비교가 유지되는지 Android·iOS Simulator에서 다시 확인한다.
**소스 기준:** `HEAD` `975d3356c9e8837834d4dd633fb3699084fad71a`와 실행 시점의 C11/C12.4 미커밋 작업 트리. 이번 실행 후에는 문서만 추가·갱신했다.

## 실행 결과

| 대상 | 장면 | geometry | 실제 표면 비교 | artifact |
|---|---:|---:|---:|---|
| Android API 37 AVD `sdk_gphone16k_arm64` | 10/10 · 성공 6/6 · 기대 거부 4/4 · 21 node | 최대 오차 0 CSS px, 허용 0.5 | PixelCopy 6/6 scene·20/20 sample exact, 최대 channel 오차 0 | [실행 로그와 캡처](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T122524470Z-d81a5172/README.md) · [비교 JSON](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T122524470Z-d81a5172/comparison.json) |
| iPhone 17 Pro / iOS 26.2 Simulator | 10/10 · 성공 6/6 · 기대 거부 4/4 · 21 node | 최대 오차 0 CSS px, 허용 0.5 | Metal drawable 6/6 scene·20/20 sample exact, 최대 channel 오차 0 | [실행 로그와 캡처](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T122614262Z-d05d416a/README.md) · [비교 JSON](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T122614262Z-d05d416a/comparison.json) |

실행 명령은 `mise exec -- node tools/css-reference/capture-c11-2-1-grid-placement-android-runtime-matrix.mjs emulator-5554`와 `mise exec -- node tools/css-reference/capture-c11-2-1-grid-placement-ios-runtime-matrix.mjs ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`다. 각 실행기가 플랫폼 앱을 현재 소스로 다시 빌드하고, 장면마다 새 앱 process와 HostDocument를 사용했다. 지원 밖 네 장면은 기대 오류·부분 frame 부재·non-empty WGPU 제출 부재로 거부했다. pixel 수집 allowlist 바깥 요청도 거부됐다.

Android AVD의 GLES는 ANGLE/Vulkan/SwiftShader 소프트웨어 경로이므로 하드웨어 GPU나 Android 실기기 증거가 아니다. iOS sample은 compositor 제출 전 Metal drawable texture readback이다. 스크린샷은 화면 확인용이며 surface pixel 비교를 대신하지 않는다. 이 제한 profile 결과는 전체 CSS Grid/WPT 적합성이나 C11.2.1 완료를 뜻하지 않는다.

이번 ADB 조회에는 Android AVD만 있었다. Android 실기기 full-scene PixelCopy와 Spinon WPT runner·적합성은 남아 있으며, 이에 따라 C11.2.1과 C11은 미완료다.
