# C08 Block 흐름·기본 페인트 시뮬레이터 근거

## 입력과 빌드

- fixture: `tests/fixtures/css/c08/runtime-block-paint.js`, 고정 ID `C08-block-flow-v1`.
- V8: `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`, revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- Android: API 37 ARM64 emulator `emulator-5554`, model `sdk_gphone16k_arm64`.
- iOS: iPhone 17 Pro Simulator, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 빌드 환경은 `SPINON_ENABLE_C04_RUNTIME_GPU=1`로 기존 C04 WGPU path를 사용했다. 내부·crate 숫자 버전은 `0.1.0` 고정이다.

```sh
mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bash tools/build-android-app.sh
mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 bash tools/build-ios-sim.sh
```

Android emulator에 APK를 설치해 `dev.spinon.bootstrap/.MainActivity --ez spinon_c08_block_paint true`로 실행했다. iOS simulator에는 앱을 설치하고 `dev.spinon.bootstrap --spinon-c08-block-paint`로 실행했다.

## 결과

| 플랫폼 | 실행 결과 | 화면·로그 |
| --- | --- | --- |
| Android API 37 emulator | V8 `status=0`, `layout=ready`, `boxes=7`, root `280×86 CSS px`, WGPU `presented boxes=7` | [화면](c08-block-flow-android-api37-emulator-2026-10-10.png) · [로그](c08-block-flow-android-api37-emulator-2026-10-10.log) |
| iPhone 17 Pro / iOS 26.2 Simulator | V8 `status=0`, `layout=ready`, `boxes=7`, root `280×86 CSS px`, WGPU `presented boxes=7` | [화면](c08-block-flow-ios-26.2-simulator-2026-10-10.png) · [로그](c08-block-flow-ios-26.2-simulator-2026-10-10.log) |

두 화면의 PNG에서 기대한 root, hero, nested hero child, first, second, last RGB palette가 각각 1,000개 이상 표본으로 확인됐다. 투명 group은 scene order를 유지하면서 자체 색을 덮지 않는다. screenshot 픽셀은 표시된 paint 색만 확인하며, 모바일 child별 CSS geometry를 독립 측정한 값은 아니다.

Android `dumpsys SurfaceFlinger`는 `ANGLE` 위의 `SwiftShader Device`와 OpenGL ES 3.1을 표시한다. 이는 emulator software backend 결과다. iOS 앱은 `CAMetalLayer` surface를 통해 WGPU를 제출했다. 두 시뮬레이터 결과를 실제 기기 GPU 성능으로 해석하지 않는다.

## 산출물 해시

| 산출물 | SHA-256 |
| --- | --- |
| Android PNG | `4cc0be6c057964ebba99fc918bbe59b828b049d665cf67ed4c557be8234c3ed7` |
| Android log | `82f5f9bdc11f0bc5fed3bd70ffe63f288d321e1d1524e6fdb3006cd0b4575a99` |
| iOS PNG | `456ef781b3b27ab7653871c8d0ac39450827b3c571c24385a80f78e212cb3890` |
| iOS log | `943f295897b2267c1deb01f45eaad61e0f3c8c6176faa213f3eb3e2167f77ee8` |
