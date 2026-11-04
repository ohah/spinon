# C11.1 parsed Root HTML 혼합 placement simulator runtime

이 문서는 fully definite Grid item과 같은 definite 축의 sparse auto-placement item을 섞는 제한 구현 후보의 앱 경로 근거다. C11.1/C11 완료나 일반 CSS Grid 지원을 의미하지 않는다.

## 입력과 기준

- Root HTML fixture: [`root-html-grid-mixed-explicit-one-axis-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-mixed-explicit-one-axis-runtime.html)
- JavaScript readiness marker: [`root-html-grid-mixed-explicit-one-axis-runtime.js`](../../../../tests/fixtures/css/c11/root-html-grid-mixed-explicit-one-axis-runtime.js)
- fixture SHA-256: `28c018c16becbca46bdd85329dc14a5720d86f024fc6744e100bf6a48742f3dc`
- readiness marker SHA-256: `0dcd892383013684a64524e5c1cf4cb7e072b2a6c797a17e1dea7397a998518f`
- Chrome 156 비교: [독립 기대값·9 scene 관측](../c11-root-html-grid-mixed-placement-precomparison-2026-11-04/README.md)
- 최종 런 ID: `c11mix19-20261104`

앱 fixture는 accepted mixed-placement scene 6개에 all-auto와 fully-definite 기존 경로 control을 더한다. 혼합 partial axis와 implicit extent 거부는 Chrome oracle과 focused Rust tests에서 검사한다. 현재 Root HTML 런타임은 지원 밖 computed placement를 만나면 개별 Grid owner 대신 문서 전체를 layout 전에 거부하므로, 거부 장면을 accepted scene들과 같은 모바일 surface 행렬에 넣을 수 없다.

구현 검토에서 첫 18-sample 실행의 행 고정 장면은 fully definite item을 같은 행의 blocker cell에 두지 않아 row-direction skip을 검증하지 못한다는 점을 발견했다. 이전 입력은 [`pass18-input`](./pass18-input/) 아래 보존했다. blocker를 1행 2열로 옮기고 PixelCopy/Metal에 해당 색상 표본을 추가했다. 최종 실행은 뒤에 온 fully definite item의 선점과 다음 row-locked item의 세 번째 열 배치를 확인한다. 최종 판정에는 19-sample 런 ID만 사용한다.

## 빌드와 실행

Android/iOS 앱 빌드는 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`로 만든 격리된 도구 경로 `build/c11-v8-rebuild/v8-source/v8`을 사용했다. 기존 `build/depot_tools`는 덮어쓰지 않았다. Android와 iOS용 V8 및 앱 빌드 로그를 이 디렉터리에 보존했다.

Android API 37 AVD에서는 `layout=ready`, `draw=presented` 후 SurfaceView PixelCopy의 좌표·RGBA 표본 19/19가 exact로 일치했다. iPhone 17 Pro/iOS 26.2 Simulator에서는 Metal drawable 표본 19/19가 exact로 일치했다.

| 플랫폼 | 관측 | 근거 |
|---|---|---|
| Android API 37 AVD | PixelCopy 19/19 exact, surface `1080×2209`, CSS viewport `411.42856×841.52374` | [최종 logcat](./android-avd-logcat-pass19.txt) · [실제 PixelCopy PNG](./android-avd-pixelcopy-pass19.png) · [시뮬레이터 화면](./android-avd-screen-pass19.png) |
| iPhone 17 Pro / iOS 26.2 Simulator | Metal drawable 19/19 exact, drawable `1206×2334`, scale `3×` | [최종 unified log](./ios-simulator-log-final.txt) · [시뮬레이터 화면](./ios-simulator-screen-final.png) |

iOS 표본은 OS compositor 제출 전 Metal drawable readback이다. 두 결과 모두 시뮬레이터에서 나온 것이며 실기기나 hardware GPU 근거로 해석하지 않는다. 앱 화면 캡처와 drawable/PixelCopy readback은 서로 다른 관측이다.

## 저장된 실행 기록

- 최종 앱 빌드: `android-fixture-apk-build-final.log`, `ios-app-build-final.log`
- 기본/fixture opt-in 산출물: [Android 기본 APK scan](./android-default-apk-scan.json) · [Android opt-in APK scan](./android-optin-apk-scan.json) · [iOS 기본 app scan](./ios-default-app-scan.json) · [iOS opt-in app scan](./ios-optin-app-scan.json). 기본 Android APK에는 새 Java adapter/run/sample strings가 없고 opt-in APK에는 모두 있다. 기본 iOS app에는 새 run/scene/sample strings가 없고 opt-in app에는 모두 있다.
- 최종 Rust 검증: placement layout 9/9, Root HTML integration 8/8, selector 3/3, scoped Clippy와 rustfmt 통과. 상세 log: [`layout-focused-final.log`](./layout-focused-final.log) · [`style-runtime-final.log`](./style-runtime-final.log) · [`selector-tests-final.log`](./selector-tests-final.log) · [`clippy-final.log`](./clippy-final.log).
- 제한 구현/runtime 검토 20개, 실제 구현 diff 검토 20개와 [PR #1147 실제 diff 적대 검토 30개](./pr-diff-review-30.md)를 별도 기록했다. PR check은 보고되지 않았다.
- 고정 V8 빌드: `v8-android-build.log`, `v8-ios-simulator-build.log`
- 이전 18-sample 실행: `android-avd-logcat-pass02.txt`, `android-avd-pixelcopy-pass18.png`, `android-avd-pixelcopy-screen-pass18.png`, `ios-simulator-log-pass18.txt`, `ios-simulator-screen-pass18.png`
- 초기 시도와 수정 원인: [간단 logcat](./android-avd-logcat.txt) · [재시도 logcat](./android-avd-logcat-pass01.txt); `border:0` shorthand가 `border-top-color` unsupported computed value를 만들었고, 같은 문서에 negative control을 포함하면 `grid-area` fail-fast가 전체 문서를 거부했다. 둘 다 final fixture 입력에서는 제거·분리했다.
- fixture 자산 보존: `android-assets-before/`, `ios-assets-before/`는 fixture 복사 전 기본 앱 자산, `pass18-input/`은 blocker 수정 전 HTML/JS다.

Android PixelCopy PNG SHA-256: `3dfb8a3d7050b2e4dcf92d3dc76e2ab35a631cbce023ba78293bdf957c39382a`
iOS 최종 화면 PNG SHA-256: `a3159ff8683642a3585b120b52f2d23fa09953fa5002f4b7a2b92042a9d03b11`

## 미검증 범위

이 결과는 Root HTML 제한 profile, 지원되는 explicit fixed-px tracks와 해당 placement 집합에만 적용한다. 전체 CSS Grid, Spinon WPT 적합성, mixed row/column partial items, implicit track sizing, Android 실기기, iOS compositor 이후 pixel parity 및 C11.1/C11 완료는 검증하지 않았다. 공식 상태 `72/213`은 변하지 않는다.
