# C11.4 nested Grid intrinsic contribution simulator 근거

**결과:** 고정된 일반 nested Grid 한 단계 3개 장면을 현재 코드로 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 3/3 실행했다. CSS Grid 독립 계산, Chrome 156 관찰, Rust layout/runtime/FFI trace와 두 앱의 frame·track 값이 일치한다. 이는 opt-in fixture 검증이며 C11.4 완료나 일반 nested Grid 지원을 뜻하지 않는다.

## 기준과 범위

- [구현 전 비교](../c11-4-nested-grid-intrinsic-precomparison-2026-10-21/README.md)는 `HeadlessChrome/156.0.0.0`, CSS Grid Layout Module Level 2 2025 CRD §6.6·§12.5·§12.5.1, 고정 Ahem 10px 입력을 구분해 둔다.
- outer Grid 안에 nested Grid 하나, 그 안에 두 direct text leaf만 둔다. text는 `XX XXXXXXX`와 `XXXXXX`; Ahem resource SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.
- fixture는 `c11-4-grid-track-sizing-fixture-test` 경로에만 있다. layout 알고리즘 수정은 없으며, nested Grid를 text/image leaf input으로 취급하지 않는다.
- 사용자가 ADB에 Samsung 실기기가 나타나지 않은 뒤 simulator를 선택했다. Samsung SM-S731N 실기기는 이번 실행에서 테스트하지 않았다.

## 장면 결과

| scene | outer/nested width | outer columns | inner columns | outer contribution |
| --- | --- | --- | --- | --- |
| `nested-grid-span-two` | `134/134px` | `65/65px` | `70/60px` | nested item `MinContent`, span 2, `134px` |
| `nested-grid-one-track` | `138/134px` | `134/0px` | `70/60px` | nested item `MinContent`, span 1, `134px` |
| `nested-grid-definite-flex-control` | `120/120px` | `58/58px` | `70/60px` | nested item `Minimum`, span 2, `0px` |

Android/iOS 앱 로그에는 전체 node frame, 축별 track, contribution provenance, trace chunk와 draw/present 결과가 보존된다. flex control의 안쪽 빨간 item은 outer Grid가 120px인데도 x=133 CSS px까지 그려지고 x=134는 검정이다. 양쪽 simulator에서 다음 다섯 색을 exact 비교했다.

| CSS sample | Expected RGBA |
| --- | --- |
| x=72 internal gap | `[231,246,237,255]` |
| x=119 inside outer edge | `[229,72,77,255]` |
| x=120 outside outer edge | `[229,72,77,255]` |
| x=133 overflow tail | `[229,72,77,255]` |
| x=134 after overflow | `[0,0,0,255]` |

Android PixelCopy와 iOS Metal drawable 표본은 각각 5/5 exact다. 표본은 캡처한 앱 surface의 색을 확인하며 border stroke를 비교하지 않는다. iOS Metal 값은 OS compositor 제출 전에 읽었다.

## 재실행 artifact

- Android: [API 37 ARM64 AVD 결과](./android-avd-c114-nested-grid-intrinsic-android-avd-20261021T104717622Z-026efc48/result.json), [simulator 정보와 APK hash](./android-avd-c114-nested-grid-intrinsic-android-avd-20261021T104717622Z-026efc48/simulator.json), [runtime 로그](./android-avd-c114-nested-grid-intrinsic-android-avd-20261021T104717622Z-026efc48/runtime-nested-grid-definite-flex-control.log). 로컬 APK와 설치된 `base.apk`의 SHA-256은 모두 `f054f02ab430465cb0efd6fdf754c6f123a54b52c439525d35afd8a5ceb2f8e8`이다.
- iOS: [iPhone 17 Pro / iOS 26.2 결과](./ios-simulator-c114-nested-grid-intrinsic-ios-simulator-20261021T104725303Z-83dd800e/result.json), [simulator 정보와 app hash](./ios-simulator-c114-nested-grid-intrinsic-ios-simulator-20261021T104725303Z-83dd800e/simulator.json), [runtime 로그](./ios-simulator-c114-nested-grid-intrinsic-ios-simulator-20261021T104725303Z-83dd800e/runtime-nested-grid-definite-flex-control.log). app SHA-256은 `d3ce4a937a9c845a8c44d30f3cebe9b8cb101bd44d7dcd5c513651d65836718a`다.
- [Android 화면](./android-avd-c114-nested-grid-intrinsic-android-avd-20261021T104717622Z-026efc48/nested-grid-definite-flex-control.png) · SHA-256 `7c862d98b312c97bd86ecce42ebdf9884418dd293c2cf9ac741ddbeece78de24`.
- [iOS 화면](./ios-simulator-c114-nested-grid-intrinsic-ios-simulator-20261021T104725303Z-83dd800e/nested-grid-definite-flex-control.png) · SHA-256 `a78238a5d9186225bf73c0c3b23cec0d55f09b6103e7786b64f6b63b880af9c7`.
- Build log: [Android fixture build](./android-fixture-build.log) · [iOS Simulator fixture build](./ios-simulator-fixture-build.log).

## 코드·artifact 검사

- Direct layout: `cargo test --locked -p spinon-layout nested_grid_intrinsic_contributions_match_chrome_without_leaf_fallback -- --nocapture` — 1 passed.
- Runtime: `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_nested_grid_items_keep_inner_text_sources_and_parent_contributions -- --nocapture` — 1 passed.
- FFI: `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_fixed_nested_grid_intrinsic_scenes -- --nocapture` — 1 passed.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, fixture-enabled `spinon-runtime`·`spinon-ffi` Clippy, `mise exec -- node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs`, `mise exec -- node tools/css-reference/fixture-module-boundary.test.mjs`가 통과했다. fixture boundary는 6/6이다.
- [기본 FFI archive build](./artifact-default-build.log) 후 `nm`·`strings`에서 C11.4 fixture FFI symbol과 nested scene payload를 찾지 못했다. [fixture feature positive control](./artifact-fixture-build.log)에서는 `_spinon_runtime_gpu_host_{new,eval}_c11_4_grid_track_sizing_fixture`와 scene 문자열이 확인된다. [검사 명령·결과](./artifact-boundary-check.log).
- Android build는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 실행했고 Gradle이 `BUILD SUCCESSFUL`로 끝났다. iOS Simulator도 같은 flag를 사용해 `BUILD SUCCEEDED`로 끝났다.

## 검증 도구에서 고친 실패

첫 AVD 실행은 새 scene ID가 native app scene allowlist에 없어 앱 validator에서 거부됐다. Android/iOS scene allowlist와 frame·box 기대값을 보완한 뒤 재실행했다. 이후 캡처 도구에서 세 검증 오류도 찾아 수정했다.

1. track trace 객체의 ID 속성은 `node`가 아니라 `grid`였으므로 trace를 읽지 못했다. 현재 parser는 field 이름을 바로잡고 outer/nested Grid ID로 trace를 선택한다.
2. trace array 첫 항목을 outer Grid로 가정했지만 nested Grid trace가 먼저 올 수 있었다. 현재 parser는 배열 순서 대신 기대 Grid ID를 사용한다.
3. Android PixelCopy expected RGB map에 새 sample ID가 없어 예상색을 찾지 못했다. 다섯 색을 명시한 뒤 실제 PixelCopy와 iOS Metal readback이 각각 5/5 통과했다.

이 실패들은 fixture/layout 계산 오류가 아니라 app allowlist와 검증 harness 결함이었다. 마지막 결과 JSON과 log는 앱을 재빌드한 다음 캡처한 위 두 run에 해당한다.

## PR diff 검토 후 최종 재실행

최종 diff 검토에서 발견한 text-style validator 누락을 보완하고 양 플랫폼 fixture 앱을 다시 빌드해 확인했다. 수정 후 결과와 해당 APK/app hash, sample log, 화면은 [최종 확인 evidence](../c11-4-nested-grid-intrinsic-final-confirmation-2026-10-21/README.md)에 기록했다. 검토 30개는 [PR diff review](./pr-diff-review-30.md)에 있다.

## 한계

이 evidence는 한 단계 일반 nested Grid와 ASCII/Ahem 입력 세 장면만 다룬다. arbitrary nesting, subgrid, nested anonymous text·image 혼합, 다른 typography, WPT, Android 실기기·hardware GPU, iOS 실기기 및 compositor 이후 픽셀은 검증하지 않는다. C11.4 checkbox와 공식 `72/213` 완료 수, 내부 계약 `0.1.0`은 바뀌지 않는다.
