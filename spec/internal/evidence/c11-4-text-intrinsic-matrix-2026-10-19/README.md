# C11.4 Ahem 텍스트 고유 크기 행렬 — 2026-10-19

## 결과

고정된 Ahem direct-leaf text 10개 scene을 Chrome 155, Android API 37 ARM64 AVD, iPhone 17 Pro / iOS 26.2 Simulator에서 다시 비교했다. 두 앱의 검증 로그에서 root와 item의 `x/y/width/height`, node 순서와 paint 순서가 Chrome과 10/10 scene에서 일치했다. native app assertion은 min/max-content width·height와 `fontRegistryRevision=1`도 확인한다. Android와 iOS의 Columns·Rows sizing trace는 chunk를 복원해 비교했으며, 10/10이 의미상 일치했다. 원본 trace는 8/10이 byte 단위로 같고, `auto-text-max-content`와 `min-content-text-nowrap`는 `AutomaticMinimum` Rust `Debug` 문자열의 공백만 다르다. 숫자·track stage·contribution 값은 모두 일치한다. 최신 재실행 ID, frame, intrinsic input, trace와 SHA-256은 [cross-platform-comparison.json](./cross-platform-comparison.json)에 기록했다.

`min-content-text-break-all`에서는 root 높이 40 CSS px보다 아래인 y=45 CSS px에서도 item 색이 그려지는지 확인했다. Android PixelCopy (`c114-android-jsoninput-min-content-text-break-all-20261019`)와 최신 iOS Simulator Metal drawable (`c114-ios-jsoninput-min-content-text-break-all-20261019`)에서 item 내부, root 아래의 item, root 배경 control 표본을 각각 확인했고 두 표면 모두 3/3 표본이 예상 RGBA와 정확히 일치했다. Metal 표본은 OS compositor 이후 화면을 읽은 결과가 아니다.

이 실행은 opt-in app fixture route의 제한된 text contribution matrix다. 공개 CSS 지원이나 C11.4 전체 완료를 뜻하지 않는다.

## 비교 입력과 기준

Chrome oracle는 저장소의 [고정 precomparison HTML](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html)을 Agent Browser 0.31.1과 Chrome 155.0.8059.85에서 실행했다. 브라우저는 HeadlessChrome 155, DPR 1, viewport 1280×633이었다. 기본 oracle 결과는 [chrome155-oracle.json](./chrome155-oracle.json)에 보존했다.

앱 fixture는 `align-items:start`를 지정하고 `align-content`를 생략한다. 앱 runtime의 computed `align-content`는 `normal`이다. 원본 Chrome precomparison의 `.grid`는 `align-content:start`를 지정하므로, runtime-equivalent Chrome 관찰에서는 10개 text root에 `align-content:normal`과 `align-items:start`를 중요한 author 선언으로 적용한 뒤 값을 수집했다. 이 조정 결과가 [chrome155-runtime-profile-oracle.json](./chrome155-runtime-profile-oracle.json)이다. 따라서 서로 다른 computed alignment를 둔 채 결과를 비교하지 않았다.

| scene | Chrome 열 / item | 텍스트 고유 입력 (min / max) | Android / iOS frame |
| --- | --- | --- | --- |
| `auto-text-max-content` | 100px / 100×10 | 70×20 / 100×10 | 120×40 root, 100×10 item |
| `min-content-text-normal` | 70px / 70×20 | 70×20 / 100×10 | 120×40 root, 70×20 item |
| `max-content-text-normal` | 100px / 100×10 | 70×20 / 100×10 | 120×40 root, 100×10 item |
| `fit-content-text-minimum` | 70px / 70×20 | 70×20 / 100×10 | 120×40 root, 70×20 item |
| `fit-content-calc-limit` | 15px / 15×80 | 10×80 / 80×10 | 100×120 root, 15×80 item |
| `min-content-text-edge` | 80px / 80×20 | 70×20 / 100×10 | 120×50 root, 80×20 item |
| `min-content-text-min-width` | 90px / 90×20 | 70×20 / 100×10 | 120×50 root, 90×20 item |
| `min-content-text-max-width` | 40px / 40×20 | 70×20 / 100×10 | 120×50 root, 40×20 item |
| `min-content-text-break-all` | 10px / 10×80 | 10×80 / 80×10 | 120×40 root, 10×80 item |
| `min-content-text-nowrap` | 100px / 100×10 | 100×10 / 100×10 | 120×40 root, 100×10 item |

앱 summary는 native 경로가 root/item frame 전체 좌표와 text intrinsic 입력을 확인한 뒤에만 기록한다. scene별 runtime 로그에는 chunk 원문 trace와 node frame 기록이 있다. 별도 `environment-*-verified.log`에는 비교에 사용한 node frame 결과를 보존했다.

## Simulator 실행

- Android: AVD `spinon_api37_1_compat16k`, `emulator-5554`, Android API 37, ARM64 `arm64-v8a`. renderer는 Android emulator OpenGL ES와 ANGLE/SwiftShader를 사용했다. 실기기 또는 hardware GPU 결과는 아니다.
- iOS: iPhone 17 Pro Simulator, iOS 26.2, Xcode 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. 앱은 Simulator Metal drawable을 사용했다. OS compositor 출력 이후 표면은 읽지 않았다.
- 실기기는 연결하거나 사용하지 않았다.
- 빌드 로그: [Android](./android-avd/build.log), [iOS](./ios-simulator/build.log). 높이·revision assertion이 포함된 현재 소스로 두 simulator 앱 빌드를 다시 성공시켰다. Android Gradle은 기존 compile SDK 37.2 / AGP 8.13.2 호환성 경고를 냈고, iOS Xcode는 V8 archive 중복 timestamp debug-map 경고와 매번 실행되는 준비 script 안내를 출력했다.
- Rust workspace test는 1,113 passed · 4 ignored · 0 failed이고, workspace all-target Clippy `-D warnings`와 `cargo fmt --all -- --check`도 통과했다. C11.4 focused runtime test 1/1, FFI test 5/5, surface-pixel feature Clippy와 Android/iOS fixture 앱 빌드도 통과했다. 명령 로그는 [Rust 회귀 결과](./workspace-test.log), [workspace Clippy](./workspace-clippy.log), [rustfmt](./rustfmt-check.log), [runtime focused test](./runtime-c114-focused-test.log), [FFI focused tests](./ffi-c114-focused-tests.log), [FFI feature Clippy](./ffi-feature-clippy.log)에 있다.

| scene | Android 실행 로그 / 화면 | iOS 실행 로그 / 화면 |
| --- | --- | --- |
| `auto-text-max-content` | [검증 로그](./android-avd/runtime-auto-text-max-content-verified.log) · [화면](./android-avd/screenshot-auto-text-max-content.png) | [검증 로그](./ios-simulator/runtime-auto-text-max-content-verified.log) · [화면](./ios-simulator/screenshot-auto-text-max-content.png) |
| `min-content-text-normal` | [검증 로그](./android-avd/runtime-min-content-text-normal-verified.log) · [화면](./android-avd/screenshot-min-content-text-normal.png) | [검증 로그](./ios-simulator/runtime-min-content-text-normal-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-normal.png) |
| `max-content-text-normal` | [검증 로그](./android-avd/runtime-max-content-text-normal-verified.log) · [화면](./android-avd/screenshot-max-content-text-normal.png) | [검증 로그](./ios-simulator/runtime-max-content-text-normal-verified.log) · [화면](./ios-simulator/screenshot-max-content-text-normal.png) |
| `fit-content-text-minimum` | [검증 로그](./android-avd/runtime-fit-content-text-minimum-verified.log) · [화면](./android-avd/screenshot-fit-content-text-minimum.png) | [검증 로그](./ios-simulator/runtime-fit-content-text-minimum-verified.log) · [화면](./ios-simulator/screenshot-fit-content-text-minimum.png) |
| `fit-content-calc-limit` | [검증 로그](./android-avd/runtime-fit-content-calc-limit-verified.log) · [화면](./android-avd/screenshot-fit-content-calc-limit.png) | [검증 로그](./ios-simulator/runtime-fit-content-calc-limit-verified.log) · [화면](./ios-simulator/screenshot-fit-content-calc-limit.png) |
| `min-content-text-edge` | [검증 로그](./android-avd/runtime-min-content-text-edge-verified.log) · [화면](./android-avd/screenshot-min-content-text-edge.png) | [검증 로그](./ios-simulator/runtime-min-content-text-edge-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-edge.png) |
| `min-content-text-min-width` | [검증 로그](./android-avd/runtime-min-content-text-min-width-verified.log) · [화면](./android-avd/screenshot-min-content-text-min-width.png) | [검증 로그](./ios-simulator/runtime-min-content-text-min-width-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-min-width.png) |
| `min-content-text-max-width` | [검증 로그](./android-avd/runtime-min-content-text-max-width-verified.log) · [화면](./android-avd/screenshot-min-content-text-max-width.png) | [검증 로그](./ios-simulator/runtime-min-content-text-max-width-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-max-width.png) |
| `min-content-text-break-all` | [검증 로그](./android-avd/runtime-min-content-text-break-all-verified.log) · [화면](./android-avd/screenshot-min-content-text-break-all.png) | [검증 로그](./ios-simulator/runtime-min-content-text-break-all-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-break-all.png) |
| `min-content-text-nowrap` | [검증 로그](./android-avd/runtime-min-content-text-nowrap-verified.log) · [화면](./android-avd/screenshot-min-content-text-nowrap.png) | [검증 로그](./ios-simulator/runtime-min-content-text-nowrap-verified.log) · [화면](./ios-simulator/screenshot-min-content-text-nowrap.png) |

화면 캡처는 앱 shell과 box geometry를 보여준다. C11.4 fixture renderer가 text glyph를 그리지 않으므로 text ink나 typography를 확인한 자료는 아니다. overflow 표본 로그와 화면은 [Android PixelCopy 로그](./android-avd/pixel-matrix-min-content-text-break-all.log), [Android 캡처](./android-avd/pixel-matrix-min-content-text-break-all.png), [iOS 최신 Metal·frame 로그](./ios-simulator/metal-pixel-min-content-text-break-all.log), [iOS Simulator 화면](./ios-simulator/screenshot-min-content-text-break-all.png)에서 확인할 수 있다.

## 제품·fixture artifact 경계

`spinon-ffi` release staticlib를 `--no-default-features`로 빌드했을 때 C11.4 host/pixel symbol과 fixture payload가 없었다. `c11-grid-metal-surface-pixels-test`를 켠 release artifact에는 두 symbol과 C11.4 fixture payload가 있었다. Rust 1.96 toolchain의 LLVM 22 `llvm-nm`과 전체 입력을 읽는 `strings` 검사로 대조했다. Xcode `nm`은 LLVM 22 attribute kind를 인식하지 못해 첫 검사에서 제외했다. [기본 빌드](./artifact-boundary/default-build.log) · [기본 부재 결과](./artifact-boundary/default-symbol-check.log) · [fixture 빌드](./artifact-boundary/fixture-build.log) · [fixture 포함 결과](./artifact-boundary/fixture-symbol-check.log) · [초기 Xcode nm 실패 기록](./artifact-boundary/initial-xcode-nm-failure.log).

## 한계

- 이 scene set은 고정 Ahem ASCII text와 지정한 Grid track/style 조합만 다룬다. 일반 font, script, line-breaking, writing mode, anonymous/nested text item, aggregate descendant measurement와 image intrinsic contribution은 다루지 않는다.
- `fit-content(calc(10% + 5px))`, inline padding/border, min/max width는 각 고정 scene만 확인했다. 전체 CSS contribution, automatic-minimum, span, cyclic percentage, auto-repeat 및 placement 조합은 별도 행렬이 필요하다.
- Chrome geometry 및 runtime frame 검사는 수치 결과다. PixelCopy와 Metal drawable은 고정 overflow scene의 표본 3개만 확인했다. border stroke, text glyph paint, OS compositor 이후 표시, 실기기는 확인하지 않았다.
- WPT와 전체 C11.4 적합성은 실행하지 않았다. C11.4와 상위 C11은 미완료로 유지한다.

## 적대 검토

- [구현 실패 관점 20개](./implementation-review-20.md)
- [PR diff 실패 관점 30개](./pr-diff-review-30.md)

초기 앱 시도에서 이 runtime profile이 `align-content:start`를 거부하는 것을 확인했다. 해당 미지원 선언은 앱 fixture 입력에서 제외하고 computed `normal`을 확인했다. 이전 거부 로그는 [첫 style 검증](./android-avd/failure-align-content-style.log)과 [computed-value 거부](./android-avd/failure-align-content-unsupported.log)에 보존했다.

## SHA-256

실행 로그, Chrome oracle, frame/trace 비교 JSON, PNG 캡처와 두 review 문서를 포함한 evidence 파일의 해시는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 기록했다. README와 hash manifest 자체는 재귀 해시를 피하려고 manifest에서 제외한다.
