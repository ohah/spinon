# C11.2.1 분리 통합 후보 재실행 · 2026-10-12

**검증 대상:** 임시 후보 index를 만든 통합 후보 tree `3638b7327ae0074d84d65d27908dc10093fe999b`. 현재 `main`이나 병합된 코드의 검증 결과가 아니다.

## 결과

| 대상 | 행렬 | geometry | GPU 표면 표본 |
|---|---|---|---|
| Android 17 / API 37 AVD `sdk_gphone16k_arm64` | 10 scene · 성공 6 · 기대 거부 4 · 21 node | 최대 오차 0 CSS px (허용 0.5) | PixelCopy 6 scene · 20/20 표본 exact · 최대 채널 오차 0 |
| iPhone 17 Pro / iOS 26.2 Simulator | 10 scene · 성공 6 · 기대 거부 4 · 21 node | 최대 오차 0 CSS px (허용 0.5) | Metal drawable 6 scene · 20/20 표본 exact · 최대 채널 오차 0 |

각 scene은 별도 앱 process와 HostDocument로 실행했다. 기대 거부 입력 4개는 진단을 반환하고 frame 및 non-empty WGPU 제출을 만들지 않았다. 허용 목록 밖의 pixel capture 요청도 readback 전에 거부했다.

## 근거

- Android app SHA-256: `020c7cbccbe0fef0579e071f1227d113b1937f851d6c87d959b2867ce4c155b5`
- iOS app SHA-256: `9b55761d7e94527330956c5712114ea07dcaeab9e53f7e0be6373bfa4fe1dd39`
- 공통 inventory SHA-256: `b2801e708e4c1ad540e119dcdba04d7c215e4029d5ec5627683e15ede8ef368b`
- 공통 HTML SHA-256: `e9b080bbc65901b17f7dead60eb8071df8ecbefee2ea62f1318776848e75f78d`
- Chrome 154 reference SHA-256: `5bbbd041e2e59fda32c684f7290f61bff41ab6dda9ab9f77ce2fb36f0e0ee034`
- [Android 실행 로그·캡처·비교 JSON](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T133451214Z-bbb225b3/README.md)
- [iOS 실행 로그·캡처·비교 JSON](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T133856335Z-0b6d2280/README.md)

## 한계

Android AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 경로이며 Android 실기기나 하드웨어 GPU 근거가 아니다. iOS 결과는 Simulator drawable을 compositor 제출 전에 읽은 것이다. Android 실기기 전체 pixel 행렬, Spinon WPT runner·적합성, C11 전체 및 C12.4 완료는 검증하지 않았다. 이 결과는 제한된 C11.2.1 fixture 행렬에만 해당한다.
