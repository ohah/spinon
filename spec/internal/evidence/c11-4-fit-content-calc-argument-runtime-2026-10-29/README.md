# C11.4 definite `calc()` `fit-content()` implicit-row runtime slice

이 자료는 pinned WPT [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html)의 `calculatedArgument` assertion 하나를 C11.4 opt-in 앱 fixture 경로로 연결한다. Source revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`, SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. Chrome 156 upstream baseline 및 CSS Grid 2025 CRD 독립 계산은 [사전 비교](../c11-4-fit-content-calc-argument-precomparison-2026-10-29/README.md)에 있다.

## 입력과 계산

- Scene: `fit-content-calc-percentage-row-wpt`. Root content box는 `60×300px`, computed style은 `grid-auto-rows:fit-content(calc(10% + 5px))`다.
- Definite 300px block-size basis에서 `10% + 5px = 35px`; item은 `min-height:0`, nested child는 `30×400px`이다. Expected row/item height는 35px이고 child overflow는 400px까지 유지한다.
- Runtime은 computed typed calc AST, projected `GridTrackLengthPercentage::Calc(LayoutCalcId)`, registry의 같은 ID·root node·`GridTemplateRows`·`Sum([Percentage(0.1), LengthPx(5.0)])`를 각각 검사한다. Registry가 빠지거나 ID/owner/property/expression이 달라지면 거부한다. Six-stage Rows trace는 base `[0,0,0,35,35,35]px`, contribution Minimum 0px / MinContent 400px / MaxContent 400px를 고정한다.
- 기존 `fit-content-percentage-row-wpt` 회귀 scene도 같은 adapter로 보존하며 row 150px을 유지한다. 지원은 두 고정 fixture scene에 한정되고 일반 CSS admission을 넓히지 않는다.

## Android API 37 ARM64 AVD

최종 모듈 분리 뒤 `emulator-5554` (`sdk_gphone16k_arm64`, API 37, arm64-v8a)에서 기존 percentage와 새 calc scene 모두 2/2 통과했다. Calc scene은 root/item/child frame `60×300 / 60×35 / 30×400px`, row base 35px, trace chunk 4, draw presented, Android PixelCopy 표본 4/4 exact를 기록했다. Surface 표본은 row 안 child `#ffd166`, row 안 item `#e5484d`, row 아래 root 배경 `#17212b`, root 밖 child `#ffd166`이다. AVD는 software renderer이며 Android 실기기나 hardware GPU 결과가 아니다.

- [최종 캡처 결과 JSON](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/android-avd-c114-fit-content-percentage-argument-android-avd-20261028T235305228Z-29498015/result.json) · [simulator/source/APK hash](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/android-avd-c114-fit-content-percentage-argument-android-avd-20261028T235305228Z-29498015/simulator.json) · [calc runtime trace/PixelCopy](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/android-avd-c114-fit-content-percentage-argument-android-avd-20261028T235305228Z-29498015/runtime-fit-content-calc-percentage-row-wpt.log)
- APK SHA-256: `9d60de714a7783d03f486c2ebc5633ed6147f0609daa0429a1954473b1016872`; installed APK hash matches.
- Source capture base commit `53e0461a748c9754b13879d6ea4ba3bef9aab18a`; dirty tracked diff SHA-256 `332c3ff6d586cdc01445bf10b0716bc4f2e57213a0e7b40e13df2e5107883122`; source snapshot SHA-256 `07c3ea3c58f5f5f2f05e7586654af4af2553360854483fba212c164d4cebbc6a`.

![Android API 37 AVD calc fit-content scene](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/android-avd-c114-fit-content-percentage-argument-android-avd-20261028T233558662Z-a2bdea72/fit-content-calc-percentage-row-wpt.png)

## iPhone 17 Pro · iOS 26.2 Simulator

최종 모듈 분리 뒤 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 기존 percentage와 새 calc scene 모두 2/2 통과했다. Calc scene은 동일한 35px row와 `30×400px` child frame, trace chunk 8, draw presented, Metal drawable 표본 4/4 exact를 기록했다. 표본은 OS compositor에 제출하기 전 drawable에서 읽었다.

- [최종 캡처 결과 JSON](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T235319650Z-40e4efaa/result.json) · [simulator/source/app hash](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T235319650Z-40e4efaa/simulator.json) · [calc runtime trace/Metal samples](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T235319650Z-40e4efaa/runtime-fit-content-calc-percentage-row-wpt.log)
- App SHA-256: `d8e9c7acca9283658d1c0fa8d7c001567aedd011b883ffbcadf06ee2d72fcf20`; installed app hash matches.
- Source capture base commit and dirty diff/snapshot hashes match the Android capture above.

![iPhone 17 Pro iOS Simulator calc fit-content scene](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T233608884Z-7a0217e5/fit-content-calc-percentage-row-wpt.png)

## Build, tests, and artifact boundary

- Runtime focused tests 4/4: [log](./checks/runtime-focused-final.log). FFI exact-route test 1/1: [log](./checks/ffi-focused-final.log).
- Fixture module boundary 2/2: [log](./checks/fixture-module-boundary-final.log). Capture script syntax: [log](./checks/capture-script-syntax-final.log). `cargo fmt --all -- --check` and `git diff --check`: [format](./checks/fmt-final.log), [diff](./checks/diff-check-final.log).
- Runtime and FFI all-target Clippy with `-D warnings`: [runtime](./checks/runtime-clippy-final.log), [FFI](./checks/ffi-clippy-final.log).
- Android and iOS opt-in app builds succeeded after the module split: [Android](./checks/android-build-final.log), [iOS](./checks/ios-build-final.log). iOS emitted existing V8 duplicate debug-map timestamp and AppIntents metadata warnings; build succeeded. [Android simulator capture](./checks/android-capture-final.json) and [iOS simulator capture](./checks/ios-capture-final.json) point to the complete run directories above.
- Default macOS ARM64 `spinon-ffi` static archive does not contain the new calc scene marker; `c04-runtime-gpu-fixture-test` archive contains it as a positive control. Default archive SHA-256 is `be9c0a6e0075fc9cce57e28f050aae7cddf0247d6a29a77200127d3b2725ccc2`; final fixture archive SHA-256 is `36c3cc936174b1aea0424d7f6aafa5e5e6bc24296049e41f01ec6eb6d61ac4c9`. [Default build](./artifact-boundary/default-build-final.log) · [default marker](./artifact-boundary/default-marker-check-final.txt) · [default hash](./artifact-boundary/default-archive-final.sha256) · [fixture build](./artifact-boundary/fixture-build-final.log) · [fixture marker](./artifact-boundary/fixture-marker-check-final.txt) · [fixture hash](./artifact-boundary/fixture-archive-final.sha256). This archive check does not prove APK/IPA exclusion.

## Corrective runs and limits

The first calc AVD capture correctly exposed a native matcher defect: the Rust report had the expected ID 0, node 1, property, and expression, while Android's ID parser removed too few closing parentheses. The parser was fixed and the combined AVD rerun passed. The rejected run is preserved at [failed AVD capture](../c11-4-fit-content-percentage-argument-runtime-2026-10-29/android-avd-c114-fit-content-percentage-argument-android-avd-20261028T233535674Z-8e389dbe/runtime-fit-content-calc-percentage-row-wpt.log). During compilation, a new observation argument was also briefly attached to a neighboring fixture call; the Rust compiler identified it and the call sites were corrected before the passing build and tests. The first opt-in archive marker check also produced a false negative because `rg -q` exited early under `pipefail`; the initial output is retained in [false-negative record](./artifact-boundary/fixture-marker-check-initial-false-negative.txt). Rechecking with a full-stream `rg` confirmed the default negative and fixture positive controls.

This is one pinned WPT assertion exercised through an opt-in fixture. It is not a Spinon WPT harness result, general author CSS support, arbitrary Grid tree support, full C11.4 completion, Android physical-device/hardware GPU validation, or post-compositor iOS pixel verification. C11.4 remains unchecked and the official completion count remains `72/213`.
