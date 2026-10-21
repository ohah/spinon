# C11.4 depth-three nested Grid runtime 검증

## 소스와 빌드

Simulator manifest는 소스 revision을 `b8bdcb502b42f364b6bd5129238e3737c3b2836c`, base tree를 `2b6e218ab4ca5495d56967b64313c341e34aee66`, tracked diff SHA-256을 `ace539c61e72d60f7b23282e265e7a15e2e4c3857978aa7ded8fe48e1fcaa68a`로 기록한다. 변경된 여섯 source 파일 목록과 실행 artifact hash는 각 [Android manifest](./android-avd-c114-nested-grid-depth-android-avd-20261021T163602344Z-da5183ae/simulator.json) 및 [iOS manifest](./ios-simulator-c114-nested-grid-depth-ios-simulator-20261021T163623798Z-23de29cd/simulator.json)에 있다.

- Android: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android`; API 37 ARM64 AVD `emulator-5558`; local APK와 설치된 `base.apk` SHA-256 모두 `49b4455c2bea2c0ed250985b43193dcdbf52d0074f1d1e833b95ec60f379e7ba`.
- iOS: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim`; iPhone 17 Pro, iOS 26.2 Simulator; app executable SHA-256 `ca02b009d151ef99035fd17389137ea00fb1e4ddd563c0a24b71c567f8ad0aea`.
- 두 빌드 모두 C04 runtime GPU와 fixture-test를 켰고 다른 작업별 fixture flag는 껐다.

## 실행 결과

기존 depth-one/two 8개 control과 신규 scene 2개를 순서대로 실행했다. 두 플랫폼 모두 10/10 scene에서 runtime summary, 기대 box 수, 각 node frame, Columns/Rows trace와 draw/present gate를 통과했다.

| 새 scene | 각 Grid 및 text frame | Columns / Rows track | Grid trace | Android / iOS draw |
| --- | --- | --- | ---: | --- |
| `nested-depth-three-min-content-two-axis` | 70×20px | `[70]` / `[20]` | 4 | 통과 / 통과 |
| `nested-depth-three-max-content-two-axis` | 100×10px | `[100]` / `[10]` | 4 | 통과 / 통과 |

각 실행은 root/middle/branch/inner Grid node 1–4와 Ahem text item node 5를 기록했다. Android 결과에는 [전체 10-scene result](./android-avd-c114-nested-grid-depth-android-avd-20261021T163602344Z-da5183ae/result.json), [설치 artifact 비교](./android-avd-c114-nested-grid-depth-android-avd-20261021T163602344Z-da5183ae/installed-apk-verification.json), [min-content 화면](./android-avd-c114-nested-grid-depth-android-avd-20261021T163602344Z-da5183ae/nested-depth-three-min-content-two-axis.png), [max-content 화면](./android-avd-c114-nested-grid-depth-android-avd-20261021T163602344Z-da5183ae/nested-depth-three-max-content-two-axis.png)이 있다. iOS 결과에는 [전체 10-scene result](./ios-simulator-c114-nested-grid-depth-ios-simulator-20261021T163623798Z-23de29cd/result.json), [min-content 화면](./ios-simulator-c114-nested-grid-depth-ios-simulator-20261021T163623798Z-23de29cd/nested-depth-three-min-content-two-axis.png), [max-content 화면](./ios-simulator-c114-nested-grid-depth-ios-simulator-20261021T163623798Z-23de29cd/nested-depth-three-max-content-two-axis.png)이 있다.

화면 캡처는 앱의 Grid box와 runtime 상태를 보여 주는 보조 확인이다. 글리프 paint, 색상별 track pixel parity, iOS compositor 이후 표면, WPT, Android hardware GPU 또는 실기기를 판정하지 않는다.

## 음수 입력과 경계

`mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_nested_grid_depth -- --nocapture`는 depth-three 정상 matrix와 아래 fail-closed 입력 4개를 통과했다.

- 미등록 depth-four root ID
- 잘못된 branch identity
- branch의 min/max intrinsic keyword 불일치
- root의 추가 direct child

`mise exec -- cargo test --locked -p spinon-ffi --features c11-grid-metal-surface-pixels-test emits_only_ten_fixed_nested_grid_depth_scenes -- --nocapture`는 fixed 10-ID FFI scene 목록과 미등록 ID 거부를 통과했다.

## Fixture artifact 경계

- 기본 `cargo build --locked -p spinon-ffi --lib` archive는 새 scene ID/payload를 포함하지 않는다. SHA-256: `100139f90ce7f21baf46a9b371185fa743ff62f9f15f57cd68a44b0dd8e5db49`.
- `--features c11-grid-metal-surface-pixels-test` archive에는 두 depth-three ID가 포함된다. SHA-256: `e072706a9c6c0034778b7594c96f4fd1e2d86d427ff3c418de6bcb4294cb67db`.
- source module boundary test는 6/6 통과했다.

## 재실행 기록

초기 Android capture는 fixture-test APK를 재빌드하지 않고 이전 8-scene APK를 설치해 신규 ID argument gate에서 멈췄다. 현재 source로 fixture-test APK를 다시 빌드한 후 같은 API 37 AVD에서 10/10을 통과했고, 로컬/설치 APK hash도 일치한다.
