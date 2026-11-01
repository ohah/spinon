# C11.4 anonymous Grid text `hyphens` runtime 후보

2026-11-01에 고정 Ahem anonymous direct-text Grid의 computed `hyphens` 21-scene 후보를 최신 앱 소스로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행했다. 독립 Chrome 156 관찰과 기대 계산은 [사전 비교](../c11-4-anonymous-grid-text-hyphens-precomparison-2026-11-01/README.md)에 있다.

각 플랫폼에서 positive 19/19, 기대 거부 2/2가 통과했다. 전체 per-scene 판정은 [Android matrix](./android-avd/matrix.tsv)와 [iOS matrix](./ios-simulator/matrix.tsv), 원본 진단 log는 각 platform directory에 있다. Matrix는 `none|manual|initial`, soft hyphen 없음/1개/2개, visible U+002D, min-/max-content와 고정 30px column, 조상 상속, `auto`/잘못된 value 거부를 다룬다.

Positive scene validator는 computed style, exact Grid/text owner, HostDocument generation/document/render-tree revision, 원문 source와 UTF-16 mapping, intrinsic min/max 및 block contribution, line 수/advance/height, item frame과 최종 Columns/Rows track을 확인한다. Reject scene은 `unsupported_c114_hyphens_value`, `computed hyphens` property, 원인 node와 frame 미발행을 확인한다. 실행 결과는 geometry/trace 근거이며 text glyph 모양의 pixel 비교가 아니다.

## Android API 37 ARM64 AVD

- 장치: `emulator-5556`, QEMU 1, API 37, `arm64-v8a`, `sdk_gphone64_arm64`.
- APK와 설치 APK SHA-256은 모두 `785a252476208d8161eb96bda1c4c8f7959f392f16c43369b15635dfac967d7f`다([simulator.json](./android-avd/simulator.json)).
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh` 성공.
- 실행: `node tools/css-reference/capture-c11-4-anonymous-grid-text-hyphens-simulators.mjs android-avd emulator-5556` — 21/21 예상 결과 통과.
- AVD는 ANGLE/Vulkan/SwiftShader software path이며 실기기 GPU 검증이 아니다.

## iOS Simulator

- 장치: iPhone 17 Pro (`iPhone18,1`), iOS 26.2, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`.
- 앱 executable과 설치 executable SHA-256은 모두 `9638b30b0bfebc481f0a098dce07b1ac02b4a90c247510e4fd5af867e9ed2bcf`다([simulator.json](./ios-simulator/simulator.json)).
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh` 성공.
- 실행: `node tools/css-reference/capture-c11-4-anonymous-grid-text-hyphens-simulators.mjs ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB` — 21/21 예상 결과 통과.
- 캡처는 Simulator 실행 근거다. 실제 기기 또는 iOS compositor 이후 픽셀을 검사하지 않았다.

두 capture runner는 `finally`에서 각 앱을 종료했다. [대표 화면](./android-avd/representative-screen.png)과 iOS 대응 화면은 root/Grid frame만 보여주며 glyph rasterization 증거가 아니다.

## 집중 검사

| 검사 | 결과 |
|---|---|
| `cargo test -p spinon-text-layout c114_hyphens -- --nocapture` | 4/4 |
| `cargo test -p spinon-style --features c11-4-grid-track-sizing-fixture-test hyphens -- --nocapture` | 7/7 |
| `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test anonymous_grid_hyphens -- --nocapture` | 2/2 |
| `cargo test -p spinon-ffi --features c04-runtime-gpu-fixture-test computed_hyphens_generates_the_fixed_direct_text_matrix -- --nocapture` | 1/1 |
| `node tools/css-reference/fixture-module-boundary.test.mjs` | 3/3 |
| fixture-enabled FFI Clippy, `cargo fmt --all -- --check`, `git diff --check` | 통과 |
| 기본 `cargo check -p spinon-runtime`, `cargo check -p spinon-ffi` | 통과 |
| `cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test` | unit 394 passed/3 ignored, integration 19 passed, 0 failed; 기존 C11.4 전체 runtime 회귀 포함 |

기본 macOS ARM64 및 fixture-enabled release FFI archive에서 scene payload와 generic fixture host export를 비교한 결과는 [artifact boundary](./artifact-boundary/README.md)에 있다. 그 archive 결과는 APK/IPA 전체 bundle에서 fixture 코드가 제외됨을 증명하지 않는다.

실제 구현/runtime 실패 경로 20개와 PR 변경 diff의 별도 실패 관점 30개를 검토했다([구현/runtime 검토](./implementation-runtime-review-20.md), [PR diff 검토](./pr-diff-review-30.md)).

## 범위 밖

이 제한 profile은 일반 author CSS admission, 임의 font·language·script·inline run, 자동 사전 기반 `hyphens:auto`, glyph paint, WPT, 실기기/hardware GPU, iOS compositor 이후 출력, 전체 C11.4 또는 C11을 지원한다고 선언하지 않는다. C11.4/C11 checkbox와 공식 완료 `72/213`은 미완료로 유지한다.
