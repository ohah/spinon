# C11.4 definite percentage auto-repeat maximum 구현 진행 · 2026-10-23

## 판정

구현 후보는 작업 브랜치에 있으며 Android·iOS simulator runtime 28/28씩을 통과했다. 구현/runtime 실패 관점 20개와 별도 PR diff 실패 관점 30개 검토를 완료했고, 두 검토에서 발견한 세부 입력 coverage 공백은 회귀 테스트로 보완했다. PR은 아직 생성하지 않았다. C11.4 체크와 공식 완료 수 `72/213`은 변경하지 않는다.

## 구현 범위

- Cascade는 auto-repeat `minmax()`의 maximum으로 nonnegative percentage와 제한 calc를 허용한다.
- Layout projection은 definite positive content-axis basis와 typed node/property calc binding을 확인한다. calc는 finite positive length와 finite nonnegative percentage의 두 항 합만 허용한다.
- columns/rows, `auto-fill`/`auto-fit`, direct percentage, max-below-min, zero maximum, calc와 fixed control을 포함한 28 scene을 opt-in app fixture와 Android/iOS route에 등록했다.
- 계획·기준·독립 기대값은 [집중 계획](../../../../plan/c11-4-auto-repeat-percentage-maximum.md)과 계획에 연결된 Chrome 156 evidence에 있다.

## 통과한 로컬 검사

- `cargo test --locked -p spinon-layout auto_repeat`: 41 passed([로그](./layout-auto-repeat-pass-review20.log)).
- `cargo test --locked -p spinon-style auto_repeat_percentage_maximum`: 2 passed([로그](./style-auto-repeat-percentage-maximum-pass-review20.log)).
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test percentage_maximum_catalog_is_exact_and_routed`: 1 passed([로그](./ffi-percentage-maximum-catalog-pass-review20.log)).
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 7 passed([로그](./fixture-module-boundary-pass-review20.log)).
- `mise exec -- cargo test --locked --workspace`: 구현 review 보완 후 재실행 통과([로그](./workspace-tests-pass-review20.log)). 첫 실행은 새로 허용한 percentage maximum을 계속 거부하도록 남아 있던 cascade 기대값 1건에서 실패했다. 해당 기대를 수정한 뒤 전체 workspace 재실행이 통과했다. 초기 로그도 [workspace-tests-pass-current.log](./workspace-tests-pass-current.log)에 남아 있다.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과([로그](./workspace-clippy-pass-review20.log)).
- `mise exec -- cargo fmt --all -- --check`와 `git diff --check`: 통과.
- 기본 `c04-runtime-gpu` archive와 opt-in `c04-runtime-gpu-fixture-test` positive control을 빌드해 fixture marker/payload의 부재·존재를 비교했다([기본](./product-archive-build.log) · [fixture](./fixture-archive-build.log) · [기계 판정 JSON](./product-archive-boundary.json), [positive control](./fixture-archive-boundary.json)). 이 결과는 FFI static archive에만 한정하며 APK/IPA 전체 제외를 뜻하지 않는다.

## Simulator runtime 결과와 미완료 확인

- iPhone 17 Pro / iOS 26.2 Simulator에서 새 expected root/item/node frames를 연결한 앱으로 28/28 scene의 track trace, frame, draw/present가 통과했다. 78/78 선택된 Metal drawable RGBA 표본은 exact match이며 compositor 제출 전 결과다. [결과 JSON](./ios-simulator-c114-percentage-maximum-ios-simulator-20261023T122917798Z-9678b290/result.json) · [iOS Simulator 화면](./ios-simulator-c114-percentage-maximum-ios-simulator-20261023T122917798Z-9678b290/columns-percent-max-fill.png)
- Android API 37 ARM64 AVD `emulator-5570`에서 28/28 scene의 expected node frames, used tracks, trace chunks, draw/present가 통과했고 78/78 PixelCopy RGBA 표본이 exact였다. 앱 renderer는 ANGLE/SwiftShader software path다. local/installed APK SHA-256은 `3d421afc506db85c7dfd703f64c24902c333658c14f64a58b374e76ffe680ee6`으로 일치했다. [Android build log](./android-build-route-color-fix.log) · [결과 JSON](./android-avd-c114-percentage-maximum-android-avd-20261023T130834339Z-da0fb0d3/result.json) · [AVD metadata](./android-avd-c114-percentage-maximum-android-avd-20261023T130834339Z-da0fb0d3/simulator.json) · [Android AVD 화면](./android-avd-c114-percentage-maximum-android-avd-20261023T130834339Z-da0fb0d3/columns-percent-max-fill.png)
- [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)와 [구현 PR diff 실패 관점 30개](./pr-diff-review-30.md) 검토를 완료했다. 두 검토 기록에서 확인한 회귀 coverage를 보완하고 테스트를 재실행했다. PR은 아직 생성하지 않았다. 기본·opt-in static archive boundary는 확인했으나 Android hardware GPU, 실기기, iOS compositor 이후 pixel은 검증하지 않았다.

## 공식 상태와 범위

이 후보는 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture slice다. 일반 CSS 지원, WPT, 실기기, Android hardware GPU, iOS compositor 이후 pixel 결과를 주장하지 않는다. 내부 계약 `0.1.0`, C11.4 체크와 공식 `72/213`은 그대로 유지한다.
