# C11.1 parsed Root HTML 혼합 교차축 자동 배치 구현 근거

이 기록은 `RuntimeRootHtmlTextPaintV1` 기본 앱 경로에 추가한 제한 placement candidate의 근거다. C11.1/C11 완료, 일반 CSS Grid 지원 또는 WPT 적합성을 선언하지 않는다. 이 실행에서 사용한 Android와 iOS는 모두 Simulator/AVD다.

## 기능 범위

같은 no-area, `grid-auto-flow: row` sparse Grid owner의 visible direct items에 row-locked와 column-locked numeric placement가 함께 있을 때 새 `RootHtmlMixedCrossAxisAutoPlacementV1` mode를 선택한다. Fully definite items가 먼저 점유하고, row-locked items가 다음 placement phase에서 배치된다. 남은 column-locked와 fully-auto items는 `order: 0`인 source order대로 sparse row cursor를 사용한다. `display:none` subtree는 후보에서 빠지고 `visibility:hidden` item은 occupancy를 차지한다. 기존 one-axis/all-auto routes는 selector control로 유지한다.

고정 입력은 4×3 explicit fixed-px grid, definite `160×120 CSS px` root, zero gap/edge, empty static direct block items, LTR다. `dense`/column flow, named line/area, non-positive span, unsupported item styles와 implicit extent가 필요한 placement는 이 slice가 지원하지 않는다.

- 계획과 Chrome 156 기준: [구현 계획](../../../../plan/c11-root-html-grid-mixed-cross-axis.md) · [사전 비교 및 독립 기대값](../c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/README.md)
- fixture: [`root-html-grid-mixed-cross-axis-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-mixed-cross-axis-runtime.html) · [readiness script](../../../../tests/fixtures/css/c11/root-html-grid-mixed-cross-axis-runtime.js)
- fixture SHA-256: `d5861bfbb7705a2fc7d752ddc40cc66a04fa7e5523e0f585ea88f69b8a1928ad`
- readiness script SHA-256: `79c90eb831dcbe3afed3414bbe477216e4f85c3af85b9fd8daa97a347340dda4`
- run ID: `c11xca26-20261104`; base source HEAD: `2b7bb7bc2f70dcff6be782a82a881cb6dcfb5351`; [candidate source/artifact hashes](./source-and-artifact-manifest.txt)

Chrome 156 Agent Browser 사전 비교는 다섯 positive scene의 22 visible item frame과 implicit-row rejection boundary를 고정했다. independent expected frames와 Chrome 관측 frame의 최대 geometry 차이는 0 CSS px다. 이는 Spinon 실행 결과가 아니다. Agent Browser의 fixture 실행은 다섯 grid와 authored direct items의 frame을 대조했고 별도 screenshot을 저장했다.

## Rust 검증

- `cargo test --locked -p spinon-layout root_html_mixed_cross_axis -- --nocapture`: 4/4 통과
- `cargo test --locked -p spinon-layout root_html_one_axis_placement -- --nocapture`: 13/13 통과
- `cargo test --locked -p spinon-style-to-layout c11_1_root_html_one_axis_grid_placement -- --nocapture`: 10/10 통과
- `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`: 통과
- `cargo clippy --locked -p spinon-style-to-layout --all-targets -- -D warnings`: 통과
- `cargo fmt --all -- --check`: 통과
- `mise exec -- bun test tools/css-reference/c11-ios-build-flags.test.mjs`: 최종 14/14 통과. 출력 플래그를 추가한 뒤 기존 expectation이 오래되어 첫 실행 13/14였고, 누락된 두 기본-off flags를 expectation에 맞춘 후 재실행했다.
- 원본 log: [mixed layout](./layout-mixed-cross-axis-test.log) · [one-axis regression](./layout-one-axis-placement-test.log) · [style/layout integration](./style-root-html-placement-test.log) · [layout Clippy](./clippy-layout.log) · [style/layout Clippy](./clippy-style-to-layout.log) · [format](./rustfmt.log) · [platform boundary test](./platform-source-boundary-test-final.log)

이 검증은 지정된 layout/style crates와 focused selector/normalizer 경로에 한정한다. 전체 workspace suite 결과로 확대하지 않는다.

## Android API 37 AVD

`emulator-5554`, API 37, `sdk_gphone64_arm64`에서 실제 `SurfaceView` PixelCopy를 실행했다. 로그에서 `layout=ready boxes=27`, root frame `411.42856×600 CSS px`와 draw/present를 확인했다. Surface는 `1080×2209`, viewport는 `411.42856×841.52374 CSS px`, density는 2.625다. 설치된 `base.apk`의 SHA-256은 빌드 APK와 같다. 고정 표본 26/26의 기대 RGBA가 PixelCopy readback과 exact로 일치했다.

- [AVD 화면](./android/avd-screenshot.png)
- [PixelCopy surface PNG](./android/pixelcopy-surface.png)
- [PixelCopy 로그](./android/pixelcopy-log.txt)
- [build/install/run/hash 기록](./android/) · [default flag-off DEX scan](./android/default-marker-scan.txt) · [opt-in DEX marker scan](./android/opt-in-marker-scan.txt)

AVD renderer는 ANGLE/Vulkan/SwiftShader software path다. 이 결과는 Android 실기기나 hardware GPU 검증이 아니다.

## iOS 26.2 Simulator

iPhone 17 Pro Simulator에서 `layout=ready boxes=27`, root frame `402×600 CSS px`, draw/present를 확인했다. Metal drawable은 `1206×2334`, scale은 3이다. 설치된 앱 executable hash는 빌드 product와 같다. drawable readback의 고정 표본 26/26이 exact로 일치했다.

- [최종 Simulator 화면](./ios/ios-screenshot-final.png)
- [최종 Metal drawable 로그](./ios/metal-log-final.txt) · [flag-off executable scan](./ios/default-marker-scan.txt) · [opt-in executable marker scan](./ios/opt-in-marker-scan.txt)
- [기본 flag-off build log](./ios/default-boundary-build-final.log) · [opt-in build/launch/hash 기록](./ios/)

Metal 표본은 OS compositor 제출 전이다. iOS 실기기와 compositor 이후 pixel parity는 확인하지 않았다.

## 발견한 fixture/build 경계

초기 fixture의 `border: 0`은 computed `border-top-color`가 현재 제한 Root HTML style profile에서 지원되지 않아 layout 전 거부됐다. 해당 선언과 geometry에 영향을 주지 않는 no-op declarations를 fixture에서 빼고, Agent Browser에서 같은 0 CSS px geometry를 다시 확인한 뒤 Android/iOS 앱 경로를 재실행했다. 원래 실패는 [`android/pre-fix-fixture-failure.log`](./android/pre-fix-fixture-failure.log)에 보존했다. 첫 Android APK의 asset hash가 default bootstrap을 가리킨 문제를 확인해 fixture asset을 반영한 opt-in APK로 다시 만들었다. 수정 후 설치 APK와 build APK hash가 같고, APK/iOS bundle의 HTML·JS hashes는 fixture와 일치한다. iOS 재실행 첫 pass는 fixture 원본 filenames를 app bundle에 복사해 default 3-box page를 띄웠다. App entry asset 이름인 `index.html`/`root-app.js`로 복사한 뒤 다시 실행했고 `layout=ready boxes=27`, Metal 표본 26/26 exact를 확인했다. 이 실패는 [`metal-log-final-pre-fix-assets.txt`](./ios/metal-log-final-pre-fix-assets.txt)에 보존했다.

PixelCopy adapter는 Android의 `c11_grid_metal_surface_pixels_fixture_test` source set에 있고, iOS surface adapter는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST_1` compile guard 아래에 있다. Flag-off Android APK DEX에서는 adapter class/run marker가 없고 flag-off iOS Simulator executable도 C11 mixed-cross-axis marker를 포함하지 않는다. 최초 iOS default scan에서 scene selector의 argument prefix 하나가 guard 밖에 남은 것을 발견해 `#if` 안으로 옮겼다. 수정 전 결과는 [`default-marker-scan-pre-fix.txt`](./ios/default-marker-scan-pre-fix.txt), 수정 후 flag-off scan과 opt-in positive-control scan은 각각 platform directory에 있다. Final opt-in build/runtime에서 marker와 26-sample matched summary를 다시 확인했다.

## 한계와 현재 상태

일반 CSS admission, CSS Grid WPT 적합성, implicit track sizing, named lines/areas, dense/column flow, text/replaced/nested/positioned items, order-modified placement, Android 실기기, hardware GPU 및 iOS compositor 이후 pixels는 범위 밖이다. 공식 구현 상태는 [spec/STATUS.md](../../../STATUS.md)에서 관리하며 C11.1/C11 상위 체크와 전체 완료 `72/213`은 그대로 유지한다.

구현 PR [#1152](https://github.com/ohah/spinon/pull/1152)는 2026-11-04T17:59:44+09:00 Rebase and merge됐고 merge SHA는 `6fe4980533f3da1e7250ff00fdb4d21e541a3a55`다. PR의 실제 변경 diff에 대한 별도 30개 검토는 [pr-diff-review-30.md](./pr-diff-review-30.md)에 있다. GitHub check rollup은 비어 있고 main required status checks가 설정되지 않아 CI 통과로 주장하지 않는다.
