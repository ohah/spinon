# C10.3.5 위치 지정 Flex 자식 플랫폼 실행 근거

**실행일:** 2026-10-11 · **내부 계약:** 0056 / `0.1.0` · **기준:** Chrome `154.0.8037.98`

## 고정 비교 범위

- Chrome reference: 20개 case, 70개 고유 node, viewport `320×240 CSS px`, DPR 1·2.
- Android/iOS runtime: 동일 inventory에서 3개 case, 15개 fixture node. 앱 root를 포함해 각 실행은 16개 frame/box를 냈다.
- 모바일 fixture는 `direct-row-center-end`, `paint-order`, `block-wrapper-static-position`이다. runtime paint profile이 허용하지 않는 `background` shorthand는 동일 색의 `background-color`로 바꿔 전달했다. 이 실행은 shorthand 파싱을 검증하지 않는다.
- 비교기는 source 순서의 NodeId와 frame을 고정 Chrome 관측값에 연결하고 `0.5 CSS px` 허용 오차를 적용한다. 정적 실행 결과를 두 플랫폼에서 각각 비교했다.

## Android 실기기

- Samsung SM-S731N, Android 16 / API 36, Samsung Xclipse 940, wgpu Vulkan.
- Debug APK 빌드 및 실기기 설치·실행이 성공했다. 로그에 `layout=ready boxes=16`, `backend=Vulkan`, `presented boxes=16`이 있다.
- 15/15 fixture node가 Chrome과 일치했고 최대 절대 frame 오차는 `0 CSS px`다.
- [전체 Logcat](android-physical.log) · [기계 비교 결과](android-comparison.json) · [화면 캡처](android-physical.png)

## iOS Simulator

- iPhone 17 Pro / iOS 26.2 Simulator, wgpu Metal.
- Simulator 앱 빌드·설치·실행이 성공했다. 앱 로그에 `backend=Metal`, main-thread surface configuration, `presented boxes=16`이 있다.
- 15/15 fixture node가 Chrome과 일치했고 최대 절대 frame 오차는 `0 CSS px`다.
- [전체 앱 로그](ios-simulator.log) · [기계 비교 결과](ios-comparison.json) · [화면 캡처](ios-simulator.png)

## 확인 과정에서 수정한 부분

첫 Android 실행은 runtime profile이 확장한 `background` shorthand의 `background-position-x`를 거부했다. 제한 paint profile에서 필요한 단색만 유지하도록 모바일 fixture 생성 단계에서 `background-color`로 정규화한 뒤 다시 빌드했다. 첫 실패는 [원본 로그](android-initial-failure.log)와 [캡처](android-initial-failure.png)에 보존했다. 실패를 숨기거나 성공 실행과 합산하지 않았다.

iOS unified log에는 C++ renderer report가 나타나지 않아 Swift host에서 surface 준비 및 renderer 초기화 결과를 명시적으로 기록했다. 두 번째 실행 로그는 Metal backend를 직접 보인다.

## 실행 명령

```sh
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android
adb -s R5KYB06SMPE install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s R5KYB06SMPE shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c1035_positioned_flex true

mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:ios-sim
xcrun simctl install booted build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app
xcrun simctl launch --terminate-running-process booted dev.spinon.bootstrap --spinon-c1035-positioned-flex

mise exec -- node tools/css-reference/compare-c10-3-5-positioned-flex-runtime.mjs \
  android-physical spec/internal/evidence/c10-3-5-positioned-flex-2026-10-11/android-physical.log
mise exec -- node tools/css-reference/compare-c10-3-5-positioned-flex-runtime.mjs \
  ios-simulator spec/internal/evidence/c10-3-5-positioned-flex-2026-10-11/ios-simulator.log
```

## 남은 검증 경계

WPT suite는 실행하지 않았다. Android/iOS에서는 20개 전체 Chrome case를 돌리지 않았다. 캡처 이미지는 기기 표면의 시각 확인이며 Chrome과 pixel-by-pixel 비교가 아니다. iOS 실기기·Android 성능·GPU hardware 전체의 동등성은 이 근거에서 주장하지 않는다. Android Gradle 빌드에는 compile SDK 37.2 / Android Gradle Plugin 8.13.2 조합 경고가 있었지만 빌드는 성공했다. iOS 빌드는 성공했고 고정 V8 archive의 중복 timestamp debug-map 경고가 남았다.
