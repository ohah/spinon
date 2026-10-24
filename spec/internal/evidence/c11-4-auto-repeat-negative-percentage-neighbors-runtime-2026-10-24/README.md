# C11.4 signed percentage auto-repeat `calc()` neighbor runtime

## 결과

Stylo computed AST의 top-level `Sum`이 정확히 두 항이고, finite signed length 하나와 finite signed percentage 하나로 구성된 경우를 제한 지원한다. 각 항은 직접 값 또는 한 겹의 `Negate`만 허용하며 순서는 둘 다 가능하다. definite positive content-axis basis에서 합산한 최종값은 property의 nonnegative range에 따라 0 이상으로 clamp한다. node/property/calc ID binding, finite 값, representability와 prefix/suffix aggregate 한도는 유지한다.

신규 fixture 9개와 기존 회귀 control 2개를 각 플랫폼에서 모두 통과했다.

| 플랫폼 | 실행 환경 | 장면 | 표면 표본 |
| --- | --- | ---: | ---: |
| Android | API 37 ARM64 AVD `emulator-5558`, software renderer | 11/11 | PixelCopy 22/22 exact |
| iOS | iPhone 17 Pro Simulator, iOS 26.2 | 11/11 | compositor 제출 전 Metal drawable 22/22 exact |

각 장면에서 runtime trace, root/item frame, used track/collapse, draw/present를 검사했다. Android 설치 APK와 빌드 APK SHA-256은 `19e626d64a6a6bd7bf76604f2b17dc85ce07483fd84bef7dddffaa41557f847b`로 같고, iOS 앱 SHA-256은 `3901af4db8fbc021555d1bff86b7d5f8e4e16f8fea97dd593fc00b961ccc821c`다.

## 새 9개 장면

- Column/row prefix·suffix의 positive final: 4개
- Column prefix와 row suffix의 negative final clamp: 2개
- border-box content basis: 1개
- occupied auto-fit prefix item: 1개
- length와 percentage 항이 모두 음수인 final clamp: 1개

모든 장면의 Chrome 156 CSSOM geometry는 독립 CSS Grid 기대값과 1/64 CSS px 이내다. 고정 oracle 검증은 [precomparison README](../c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md)에 있다. [Chrome 156 전체 화면](../c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png)과 선택된 Android/iOS 화면을 함께 보관한다.

## 실행 근거

- Android 최종 11-scene matrix: [result](android-avd-c114-signed-negative-percentage-neighbors-android-avd-20261024T061515084Z-2d9ec1ae/result.json) · [simulator metadata](android-avd-c114-signed-negative-percentage-neighbors-android-avd-20261024T061515084Z-2d9ec1ae/simulator.json) · [capture log](../c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/android-matrix-final.log)
- iOS 최종 11-scene matrix: [result](ios-simulator-c114-signed-negative-percentage-neighbors-ios-simulator-20261024T061454873Z-3c7ad103/result.json) · [simulator metadata](ios-simulator-c114-signed-negative-percentage-neighbors-ios-simulator-20261024T061454873Z-3c7ad103/simulator.json) · [capture log](../c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/ios-matrix-final.log)
- [Android AVD screenshot](selected-screenshots/android-avd-negative-percentage-col-both-negative-final.png) · [iPhone 17 Pro screenshot](selected-screenshots/ios-simulator-negative-percentage-col-both-negative-final.png)
- Focused Rust tests, fixture catalog/boundary checks, `cargo fmt --all -- --check`, Chrome oracle validator, focused Clippy, Android/iOS build logs and static archive scan are in [artifact evidence](../c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/README.md).
- Exact input hashes and tracked diff digest: [source manifest](source-manifest.json).

## 검증 범위

Android 결과는 software-rendered AVD다. iOS 표본은 compositor 제출 전 Metal drawable에 한정된다. 이 검증은 opt-in fixture profile의 9개 scene과 2개 regression control이며, 실기기/hardware GPU, iOS compositor 이후 pixels, WPT, 일반 author CSS admission, 전체 C11.4 또는 CSS Grid 적합성을 증명하지 않는다. 기본/fixture-test FFI static archive 대조는 별도 artifact 근거이며 APK/IPA 전체 fixture 제외 검사는 아니다.

공식 C11.4 checkbox와 완료 집계 `72/213`, 내부 numeric contract version `0.1.0`은 그대로 유지한다.
