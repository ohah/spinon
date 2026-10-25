# C11.4 fixed-repeat `minmax(px, fr)` + auto-repeat 실행 근거

**최종 source 증거 · 2026-10-25:** FFI fixture test가 Chrome 156의 raw root/item rect를 직접 대조하며, 그 파일 hash를 포함한 26개 입력 SHA가 Android·iOS 최종 캡처 및 archive run에서 모두 일치한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 6/6 장면을 다시 실행했다.

## 판정 범위

이 후보는 opt-in `RuntimeBlockFlexGridTrackSizingV1`에서 한 track list 안에 하나의 `auto-fill` 또는 `auto-fit`과 하나 이상의 integer fixed-repeat을 둔 경우, fixed-repeat pattern의 1–2개 track을 finite nonnegative `LengthPx` 또는 `MinMax { min: LengthPx, max: Fraction }`으로 확장한다. 일반 authored neighbor의 `minmax(px, fr)`는 허용하지 않는다. Cascade가 fixed-repeat 출처 track index를 보존하고 style-to-layout이 그 provenance가 있는 track에만 새 shape를 허용한다.

fixed minimum은 auto-repeat 개수 계산에 쓰고, 보존된 flex maximum은 최종 track sizing에서 사용한다. `0fr`와 1 미만 flex factor를 유지한다. Cascade와 layout은 Taffy의 `f32` flex-factor 합계가 비유한 값이 되지 않도록 expanded-axis 상한을 호출 전에 검사한다. 지원 밖 breadth와 음수·비유한 typed value는 node/property 맥락을 유지해 fail-closed한다.

규범 입력은 [CSS Grid Level 1 2025 CRD §7.2.3.1–7.2.3.2, §7.2.4, §11.7–§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에 고정했다. 독립 계산, Chrome 156 관측, 6-scene 입력과 허용 오차는 [사전 비교](../c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.md), [`spec-derived-expectations.json`](../c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json), [`chrome156-observation.json`](../c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/chrome156-observation.json)에 있다.

## 최종 source snapshot 실행

`sourceRevision.commit`은 계획 PR #614가 병합된 `d8a9fd96d7de6d7234aca705bde1bff91a3133df`이며, 두 simulator metadata는 실행 시점 tracked diff SHA `9a79a11be6cc3cd88f6fcb5d49db10eebf66c4fe4407563edbb88cba5aa0a391`와 26개 구현·입력 파일의 개별 SHA-256을 기록한다. 두 플랫폼의 manifest를 현재 파일과 재계산해 각각 26/26 일치했다. Android 설치 APK SHA는 build APK와 같고, iOS app executable SHA는 simulator metadata와 일치한다. 실행 뒤 추가된 evidence index/log는 앱 입력이 아니다.

| 실행 | 환경 | 결과 | 증거 |
| --- | --- | --- | --- |
| Android | API 37 ARM64 AVD `emulator-5560` (`sdk_gphone16k_arm64`) | 6/6 scene의 node frame, track/collapse, trace, draw/present 통과. superseded retry 0회. APK SHA-256 `558ba30ef578ae24272061b046fe4ec213ff538abf7491c4b927500b2408a2c1`; 설치본 일치. | [`simulator.json`](./android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T235142067Z-4887fd16/simulator.json) · [`result.json`](./android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T235142067Z-4887fd16/result.json) · [화면](./android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T235142067Z-4887fd16/fixed-repeat-minmax-flex-columns-fill.png) |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator, Xcode 26.2 SDK | 6/6 scene의 node frame, track/collapse, trace, draw/present 통과. superseded retry 0회. App SHA-256 `15dc61e151fb8544ad309f04a3dbd37410298ac7113388bfd073cf13d469075e`. | [`simulator.json`](./ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/simulator.json) · [`result.json`](./ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/result.json) · [화면](./ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/fixed-repeat-minmax-flex-columns-fill.png) |

Android build command는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android`이며 결과는 `BUILD SUCCESSFUL`이다. iOS는 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:ios-sim`으로 빌드했다. 캡처는 같은 6개 scene을 각 플랫폼에 설치된 최종 fixture-test app에서 실행했다. final-source precomparison 입력 SHA·APK/app hash·scene 결과는 각 `simulator.json` 및 `result.json`이 원본이다.

모든 frame 좌표는 FFI fixture test에서 independent expected tracks, item grid-line 범위, root 크기와 gutter로 산출한 full rect 네 좌표와 연결한다. 각 결과는 고정된 Chrome 156 observation의 root 크기와 item rect 네 좌표에도 1/64 CSS px 허용치로 직접 대조하며 scene ID·item index·순서를 검사한다. Simulator runner는 node ID·순서·x/y/width/height 및 track/collapse 결과를 비교하고 fractional runtime geometry에 0.02 CSS px까지 허용한다. `auto-fit` collapsed track은 정확히 0을 요구한다. 최신 AVD trace chunk 수는 scene 순서대로 9/9/6/6/15/8, iOS는 17/17/12/12/29/15이다.

이 matrix는 **레이아웃 계산, draw/present 제출, 화면 캡처**를 확인한다. 새 scene은 pixel sampling을 요청하지 않으므로 Android PixelCopy 색상 일치나 iOS Metal drawable RGBA exact를 주장하지 않는다. Android 결과는 AVD이며 실기기 Samsung이나 hardware GPU 검증이 아니다. iOS 결과는 Simulator이며 OS compositor 이후 출력 검증이 아니다.

첫 실행 전에는 Android/Swift app adapter의 helper 중괄호 누락 때문에 앱 빌드가 실패했다. 두 파일의 brace를 바로잡고, 오래된 generic fixed-repeat 기대값(200×80)을 새 200×120 scene 크기로 분기한 다음 양쪽 빌드와 6-scene matrix를 다시 실행했다. 해당 실패는 final-source 결과에 포함하지 않았다. provenance manifest 도입 전 성공 실행은 [`android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T232500742Z-6e455bd1`](./android-avd-c114-fixed-repeat-minmax-flex-android-avd-20261024T232500742Z-6e455bd1/) 및 [`ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T232541310Z-5ebdcb2d`](./ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T232541310Z-5ebdcb2d/)에 보존돼 있으며 final-source 판정에는 쓰지 않는다.

## Rust·fixture 검증

- `cargo fmt --all -- --check` 통과.
- `cargo test --locked --workspace --all-targets` 통과.
- `cargo test --locked -p spinon-style fixed_repeat_ --lib`: 27/27 통과.
- `cargo test --locked -p spinon-style-to-layout track_sizing_projection_ --lib`: 6/6 통과.
- `cargo test --locked -p spinon-layout flex_factor_bound --lib`: 4/4 통과.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test anonymous_grid_fixture_rejects_typography_and_track_profile_mutations --lib`: 1/1 통과.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test fixed_repeat_minmax_flex_matrix_matches_precomparison_and_routes --lib`: 1/1 통과. 이 test는 six scene의 full rect 네 좌표를 independent track/frame 계산 및 고정 Chrome 156 root/item rect 네 좌표와 각각 1/64 CSS px 허용치로 대조한다.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 12/12 통과; `node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs` 통과.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과. 전체 출력은 [`workspace-clippy.log`](./workspace-clippy.log).
- `git diff --check` 통과.
- Workspace-wide Clippy: 최종 source snapshot에서 `-D warnings` 통과.

## FFI artifact 경계

최종 source snapshot에서 product와 fixture-test positive control을 각각 locked release로 빌드·스캔했다. product `c04-runtime-gpu` archive는 75,186,128 bytes / SHA-256 `9384a0d07444ecb98a6c0e69d76a5358292d7d4a95d52a743e6fc68cb4bf6d95`이며 새 scene marker 0/6, C11.4 host ABI 문자열 0/2, 정의 심볼 0/2, 고유 CSS payload 없음이다. positive-control `c04-runtime-gpu-fixture-test`는 78,357,536 bytes / SHA-256 `c48e6742f311befa4349e51a709cfa757683c390f29ce9216b9cc38df0f8d853`이며 scene marker 6/6, host ABI 문자열·정의 심볼 각 2/2, `0.5fr/0fr` CSS payload를 포함한다.

세부 build/`strings`/`llvm-nm` logs와 26개 앱 source hash 대조는 [artifact-boundary README](./artifact-boundary/README.md) 및 [최신 final-source run](./artifact-boundary/final-source/runs/final-artifact-boundary-20261024T235409Z-94df3151/)에 있다. 이 검사는 Rust static archive feature 경계만 판정하고 Android APK나 iOS IPA 전체에서 fixture code가 빠진다는 증거는 아니다.

## 적대 검토와 공식 상태

구현·runtime failure-view review는 [20개 검토](./implementation-review-20.md)에 기록한다. 별도 PR diff 30-view review는 [검토 결과](./pr-diff-review-30.md)에 기록했으며 제품 코드 차단 결함은 없었다. 검토에서 찾은 공개 roadmap simulator 화면 링크를 final-source run으로 수정했다.

이 limited fixture candidate는 일반 CSS admission, CSS Grid WPT 적합성, 실기기/hardware GPU, 최종 APK/IPA 전체 fixture exclusion, iOS compositor 이후 pixels 또는 전체 C11.4 완료를 뜻하지 않는다. 공식 C11 체크박스와 완료 수 `72/213`은 유지한다. 내부 계약 숫자 버전도 `0.1.0`으로 유지한다.
