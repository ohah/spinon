# C11.4 auto-repeat + template areas 사전 비교 및 최종 simulator run

이 고정 입력은 C11.4의 한 축 auto-repeat과 `grid-template-areas` 결합만 다룬다. 계획과 CSS Grid 규범 기대는 [집중 계획](../../../../plan/c11-4-auto-repeat-template-areas.md)에, Chrome 156 관측과 13개 canonical scene은 [구현 전 비교](../c11-4-auto-repeat-template-area-lines-precomparison-2026-10-23/README.md) 및 [고정 HTML fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-template-areas-c114.html)에 기록했다.

## 최종 실행

최종 코드 변경 후 Android/iOS 앱을 각각 다시 빌드하고 같은 13개 scene을 재실행했다. 두 실행의 `simulator.json`은 base commit, tracked diff hash, 앱 artifact SHA와 simulator 정보를 기록한다. 최종 코드의 새 untracked fixture/module/test 입력은 [source-files-manifest.json](../c11-4-template-area-auto-repeat-runtime-2026-10-23/source-files-manifest.json)에 개별 SHA-256으로 고정했다.

| 대상 | 실행 | 장면 | surface 표본 | 핵심 판정 |
| --- | --- | ---: | ---: | --- |
| Android API 37 ARM64 AVD `emulator-5558` | [final run](./android-avd-c114-template-area-repeat-android-avd-20261023T002240504Z-8c091e78/result.json) | 13/13 | PixelCopy 66/66 | frames, tracks, trace, draw/present, samples 통과 |
| iPhone 17 Pro / iOS 26.2 Simulator | [final run](./ios-simulator-c114-template-area-repeat-ios-simulator-20261023T002340216Z-ea7c6290/result.json) | 13/13 | Metal drawable 66/66 | frames, tracks, trace, draw/present, samples 통과 |

전체 frame·used track을 플랫폼 간 exact 비교했다: [cross-platform-comparison.json](../c11-4-template-area-auto-repeat-runtime-2026-10-23/cross-platform-comparison.json). Android는 AVD software renderer의 PixelCopy를 사용하며 iOS는 OS compositor 제출 전 Metal drawable을 읽는다. 실기기나 compositor 이후 픽셀 증거가 아니다.

기본/product FFI archive에서 fixture module symbol과 scene marker가 없고, `c04-runtime-gpu-fixture-test` archive positive control에서는 둘 다 있었다. [기본 archive](../c11-4-template-area-auto-repeat-runtime-2026-10-23/ffi-product-archive.json) · [positive control](../c11-4-template-area-auto-repeat-runtime-2026-10-23/ffi-fixture-archive.json). 이 검사는 FFI static archive 경계이며 APK/IPA 전체 exclusion은 아니다.

## 화면

| 플랫폼 | area dimension이 expanded template보다 긴 scene |
| --- | --- |
| Android API 37 ARM64 AVD | ![Android C11.4 area-larger scene](./android-avd-c114-template-area-repeat-android-avd-20261023T002240504Z-8c091e78/template-area-repeat-columns-area-larger.png) |
| iPhone 17 Pro / iOS 26.2 Simulator | ![iOS C11.4 area-larger scene](./ios-simulator-c114-template-area-repeat-ios-simulator-20261023T002340216Z-ea7c6290/template-area-repeat-columns-area-larger.png) |

각 run directory에는 해당 platform의 13개 screenshot, scene runtime/launch log, `result.json` 및 `simulator.json`이 들어 있다. [전체 screenshot hash 목록](../c11-4-template-area-auto-repeat-runtime-2026-10-23/screenshot-manifest.json).

## 한계와 관련 문서

Runtime CSS parser의 `visibility:hidden` 제한 때문에 line-collision 시각 probe는 같은 색으로 겹치게 표시했다. 이 run은 visibility cascade를 검증하지 않는다. `area-larger` 화면의 root 바깥 suffix 중심 PixelCopy/Metal 표본은 검사하지만 CSS border stroke 자체는 renderer가 그리지 않아 확인하지 않는다. Auto-fit suffix는 Chrome 156 관측과 달랐던 4px gutter를 규범값으로 판정한다. 일반 CSS admission, WPT, Android hardware GPU, 실기기, iOS compositor 이후 pixels 및 전체 C11.4는 미검증이다. C11.4 checkbox와 공식 `77/213`은 유지한다.

- [구현/runtime 실패 관점 20개](../c11-4-template-area-auto-repeat-runtime-2026-10-23/implementation-runtime-review-20.md)
- [계획 실패 관점 20개](../../../../plan/c11-4-auto-repeat-template-areas-plan-review-20.md)
- [구현 runtime·archive·test 근거](../c11-4-template-area-auto-repeat-runtime-2026-10-23/README.md)
