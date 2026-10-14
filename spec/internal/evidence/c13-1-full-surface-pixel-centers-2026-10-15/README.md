# C13.1 full surface pixel-center 비교 · 2026-10-15

Chrome for Testing 155 `300×100` CSS px surface를 Android API 37 ARM64 AVD의 SurfaceView PixelCopy와 iPhone 17 Pro / iOS 26.2 Simulator의 실제 WGPU Metal drawable 표본으로 비교했다.

| 실행 | 고정 환경 | 표본 결과 | 화면/원본 근거 |
|---|---|---|---|
| Android | `emulator-5554`, API 37, ARM64, ANGLE/Vulkan/SwiftShader | 41개 고유 frame, 52/52 PixelCopy, 30,000/30,000 hit identity, 30,000 CSS center 및 120,000 RGBA channel exact; mismatch 0 | [AVD 실행·PixelCopy·진단](../c13-1-android-avd-full-surface-2026-10-15/c13-1-android-avd-20261014T172609127Z-8dbc1491/README.md) |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator | 새 process 3개 각각 41/41 frame, 52/52 fixed query, 30,000/30,000 hit identity, 30,000 CSS center 및 120,000 RGBA channel exact; mismatch 0 | [Metal capture 3회와 manifests](../c13-1-ios-metal-pixel-centers-2026-10-15/c13-1-ios-metal-20261014T171601787Z-572c6b6a/README.md) |

두 표본 비교는 fixture 내부 색에만 한정되지 않는다. `positioned-escape`의 `(270,72)`는 viewport 안이면서 부모의 오른쪽 border/frame x=264 바깥이다. Chrome의 target paint `[227,84,78,255]`가 AVD PixelCopy와 iOS Metal 표본에서 유지됐고, 비교기의 R/G/B/A 각 1-byte mutation을 모두 실패로 검출했다.

Android ADB transport와 앱 실행은 정상이었다. 실행기는 `am start`의 비동기 반환 뒤 PID를 즉시 조회하던 race, JNI·Java 중복 frame log, eval field 순서, PixelCopy SHA/path group index와 comparison size property 위치를 잘못 가정했다. 수정된 실행기는 새 process PID 대기, frame index별 동일 payload, run ID 연결, PixelCopy PNG hash와 전체 비교를 검증했다. 상세는 [Android 실행 근거](../c13-1-android-avd-full-surface-2026-10-15/c13-1-android-avd-20261014T172609127Z-8dbc1491/README.md)에 있다.

![iPhone 17 Pro iOS Simulator](../c13-1-ios-metal-pixel-centers-2026-10-15/c13-1-ios-metal-20261014T171601787Z-572c6b6a/run-1-ios-simulator.png)

![Android API 37 AVD](../c13-1-android-avd-full-surface-2026-10-15/c13-1-android-avd-20261014T172609127Z-8dbc1491/android-api37-avd-screen.png)

![Android API 37 AVD PixelCopy surface](../c13-1-android-avd-full-surface-2026-10-15/c13-1-android-avd-20261014T172609127Z-8dbc1491/android-api37-avd-pixelcopy.png)

## 판정 경계

Metal readback은 compositor submission 이전의 WGPU surface 표본이다. 이 결과는 30,000개 CSS pixel center만 증명한다. 전체 물리 drawable byte, compositor/scanout, 실기기·hardware GPU, 다른 해상도·orientation, DOM event·CSSOM, WPT 또는 C13.1 전체 제품 지원을 증명하지 않는다. 공식 구현 상태는 계속 미완료다.
