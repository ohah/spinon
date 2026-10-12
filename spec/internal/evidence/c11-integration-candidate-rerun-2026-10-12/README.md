# C11 통합 후보 모바일 재실행 · 2026-10-12

**검증 대상:** 임시 후보 index에서 만든 tree `3638b7327ae0074d84d65d27908dc10093fe999b`. 현재 `main`이나 병합된 코드의 결과가 아니다.

## 검증 결과

| 범위 | Android API 37 AVD | iPhone 17 Pro / iOS 26.2 Simulator |
|---|---|---|
| C11.1 geometry runtime | 후보 Android 실기기가 ADB에 없어 미실행 | 10 scene · 40 data node · pinned Chrome 154 기준 ±0.5 CSS px 통과 |
| C11.1 GPU 표면 픽셀 | 10 scene · 36/36 PixelCopy sample exact · 최대 채널 오차 0 | 10 scene · 36/36 Metal drawable sample exact · 최대 채널 오차 0 |
| C11.2.1 explicit placement runtime | 10 scene · 성공 6 · 기대 거부 4 · 21 node · 최대 geometry 오차 0 CSS px | 10 scene · 성공 6 · 기대 거부 4 · 21 node · 최대 geometry 오차 0 CSS px |
| C11.2.1 성공 장면 표면 픽셀 | PixelCopy 6 scene · 20/20 sample exact · 최대 채널 오차 0 | Metal drawable 6 scene · 20/20 sample exact · 최대 채널 오차 0 |

각 runtime scene은 새 앱 process와 HostDocument로 실행했다. C11.2.1의 기대 거부 네 입력은 진단을 반환했고 partial frame 및 non-empty WGPU 제출을 만들지 않았다. Pixel capture 허용 목록 밖 요청도 readback 전에 거부했다.

## 빌드·검사

- `mise exec -- bun run test`: JS 2, CSS reference 297, Android 도구 29, Rust 615 통과·4 ignored. Release depth test 별도 1/1 통과.
- `mise exec -- cargo fmt --all -- --check`: 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과.
- Android C11.2.1 AVD와 iOS Simulator 앱 빌드: 통과.

## 근거

- [C11.1 iOS geometry runtime와 캡처](../c11-grid-ios-simulator-2026-10-12/c11-grid-ios-matrix-20261012T134514150Z-2b1302b0/README.md)
- [C11.1 Android AVD 표면 비교](../c11-1-mobile-surface-pixel-matrix-2026-10-12/android-avd-c11-pixels-20261012T134717572Z-0c6669e1/README.md)
- [C11.1 iOS 표면 비교](../c11-1-mobile-surface-pixel-matrix-2026-10-12/ios-simulator-c11-pixels-20261012T134755249Z-21004660/README.md)
- [C11.2.1 Android runtime·PixelCopy](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T133451214Z-bbb225b3/README.md)
- [C11.2.1 iOS runtime·Metal](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T133856335Z-0b6d2280/README.md)

## 한계

Android AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 경로로, 하드웨어 GPU나 실기기 성능 근거가 아니다. 이번 `adb devices -l`에는 `emulator-5554`만 있었다. iOS는 Simulator이며 실기기 검증이 아니다. 표면 비교는 지정 fixture의 GPU 표면에서 읽었고 OS compositor 최종 출력의 색 처리를 뜻하지 않는다. Spinon WPT runner·적합성, Android 실기기 행렬, C11 전체와 전체 CSS 지원은 미완료다.
