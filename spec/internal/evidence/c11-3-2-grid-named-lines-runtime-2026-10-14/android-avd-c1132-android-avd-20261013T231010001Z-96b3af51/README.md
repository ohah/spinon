# C11.3.2 Android AVD 재실행 실패 기록 · 2026-10-14

- 대상: Android API 37.0 ARM64 AVD `emulator-5554`
- 실패 단계: `:app:prepareSpinonBootstrap`, 앱 source 빌드 전
- 원인: PR 전용 checkout에 `build/v8-source/v8`가 없고 `SPINON_V8_DIR` override 없이 실행해 고정 V8 revision 사전 조건을 만족하지 못했다.
- 결과: 제품 runtime, geometry, PixelCopy 또는 fixture를 실행하지 않았다. 이 run은 제품 실패 증거가 아니다.
- 원본 로그: [build.log](./build.log), [failure.txt](./failure.txt), [device.json](./device.json)
- 재시도: 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`의 clean V8 checkout을 `SPINON_V8_DIR`로 지정했다. [통과한 최신 AVD 실행](../android-avd-c1132-android-avd-20261013T231257017Z-496b3f85/README.md)
