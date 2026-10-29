# C11.4 Grid `visibility:collapse` 제한 runtime 결과

이 실행은 [사전 비교](../c11-4-grid-visibility-collapse-precomparison-2026-10-29/README.md)와 [계획](../../../../plan/c11-4-grid-visibility-collapse.md)에 고정한 10개 Grid scene을 현재 작업 트리에서 앱 runtime으로 실행했다. 기준 source HEAD는 `6be4b0c670f776254406778227aa5ce271b74f19`이고, 각 `result.json`의 `sourceDirty`는 이번 runtime source와 capture runner 경로만 기록한다.

## 결과

| 플랫폼 | 환경 | 장면 | 표면 표본 | 결과 |
| --- | --- | ---: | ---: | --- |
| Android | API 37 ARM64 AVD, `sdk_gphone16k_arm64`, Android 17; emulator `emulator-5554` | 10/10 | 14/14 exact PixelCopy | 통과 |
| iOS | iPhone 17 Pro Simulator, iOS 26.2, `iPhone18,1` | 10/10 | 14/14 exact Metal drawable | 통과 |

두 행렬은 `collapse`, `hidden`, `display:none`, visible descendant, visible overflow positive control을 Columns/Rows 양축에서 실행했다. Scene별 computed style/tree, track list, item/child frame, Grid track trace, render box 수, draw/present 및 고정 표면 표본을 확인했다. `display:none`은 축별 `[0,0]` track과 0×0 item을 별도로 확인하며 표면 표본을 요청하지 않는다. 나머지 장면은 각각 14개 표면 표본에서 expected RGB를 정확히 읽었다.

- [Android 실행 결과와 장치 정보](./android-avd/c114-collapse-mvtfme5e-89daa39e/result.json) · [화면](./android-avd/c114-collapse-mvtfme5e-89daa39e/)
- [iOS 실행 결과와 장치 정보](./ios-simulator/c114-collapse-mvtfn06e-14ef5dc2/result.json) · [화면](./ios-simulator/c114-collapse-mvtfn06e-14ef5dc2/)
- 확인한 iOS `collapse-visible-descendant-column` 화면에서는 파란 visible child가 보이고 collapsed red content는 보이지 않는다. `visible-column` 화면에서는 red overflow가 root 경계 밖에 보인다.
- Android는 SwiftShader software AVD다. iOS 표본은 compositor 제출 전 Metal drawable이다. Android/iOS 실기기, hardware GPU, compositor 이후 색은 이 근거로 주장하지 않는다.

## 빌드와 검증

- `cargo fmt --all -- --check` 통과.
- `cargo test --locked -p spinon-style --features c11-4-grid-track-sizing-fixture-test visibility_collapse` 통과 (2 passed).
- `cargo test --locked -p spinon-style-to-render --features c11-4-grid-track-sizing-fixture-test c114_visibility_collapse_keeps_grid_frame_but_omits_paint_and_hit_box` 통과 (1 passed).
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test routes_only_ten_fixed_grid_visibility_collapse_scenes` 통과 (1 passed).
- `cargo check --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test` 통과.
- `cargo clippy --locked -p spinon-style -p spinon-style-to-render -p spinon-runtime -p spinon-ffi --all-targets --features spinon-ffi/c04-runtime-gpu-fixture-test -- -D warnings` 통과.
- Android API 37 ARM64 AVD debug app build 통과. 실행은 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 빌드했다.
- iOS Simulator debug app build 통과. 같은 세 feature flag를 켜고 iPhone 17 Pro / iOS 26.2 Simulator에서 10/10 scene을 실행했다.
- [기본/fixture-test static archive 경계](./artifact-boundary/README.md): 기본 `c04-runtime-gpu` archive에는 C11.4 visibility route marker가 없고, `c04-runtime-gpu-fixture-test` positive control에는 route, symbol 및 scene marker가 있다.

Capture runner는 각 scene을 새 process로 실행하고 마지막에 Android app을 force-stop, iOS app을 terminate했다. 테스트는 고정 opt-in fixture route만 다루며 일반 CSS admission, 다른 Grid profile, Flex/Table collapse, WPT 전체 또는 C11.4/C11 완료를 뜻하지 않는다. 공식 `spec/STATUS.md`의 C11 checkbox와 전체 `72/213`은 그대로다.
