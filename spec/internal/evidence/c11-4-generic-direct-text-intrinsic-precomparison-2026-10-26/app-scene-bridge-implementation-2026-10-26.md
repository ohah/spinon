# C11.4 generic direct text app-scene bridge 구현 기록

## 상태와 범위

이 기록의 첫 버전은 `generic-root-normal`을 C11.4 fixture-test 앱 경로에 연결한 소스/검사 결과였다. 당시 Android/iOS build·Simulator 실행은 하지 않았다. 현재 두 플랫폼 build/runtime 후속 결과는 [runtime evidence](../c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/README.md)에 기록한다.

## 연결한 canonical scene

- Scene/root ID: `generic-root-normal`
- Item ID와 source order: `generic-normal-a` / `0`, `generic-normal-b` / `1`
- 두 item의 direct text: `A BC`
- Intrinsic input: 각각 `Text`, min/max inline `20/40px`, min/max block `20/10px`, font registry revision `1`; anonymous `textSources`는 비어 있음
- 최종 trace: content Columns `[20,40]px`, Rows `[20]px`; serialized trace에는 양끝 collapsed gutter와 가운데 10px Columns gutter가 포함된다. Columns contributions는 node `2` MinContent 20px, node `4` MinContent 20px 및 MaxContent 40px이며 Rows contributions는 definite assigned column width에서 node `2`가 각각 20px, node `4`가 각각 10px이다. 반복 동일 tuple을 허용하고 다른 item/kind/value는 거부한다.
- Frame: root `(0,0,100,20)`, 첫 item `(0,0,20,20)`, 두 번째 item `(30,0,40,10)`; app matcher는 `boxes=3`과 node ID mapping도 고정한다.

## Feature 경계

- FFI scene source는 `spinon-ffi`의 `runtime_gpu::fixtures`에서만 노출되며 해당 모듈은 `c04-runtime-gpu-fixture-test` feature 아래 compile된다.
- Android scene allowlist는 `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST`가 켜졌을 때만 이 ID를 허용한다.
- iOS scene allowlist는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1` compile condition 아래에서만 이 ID를 가진다.
- 이 fixture는 제품 CSS admission이나 glyph paint 지원을 추가하지 않는다. 현재 Android/iOS screenshots와 `runtime_text_count=0`, `glyph_batch_draw_count=0`은 layout/draw surface evidence이며 glyph pixels를 검증하지 않는다.

## 실행한 검사

- `cargo fmt --all -- --check`: 통과
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_only_the_canonical_generic_direct_text_scene`: 1/1 통과
- `git diff --check`: 통과

## 2026-10-26 Android/iOS build 및 Simulator 후속

- Android `mise exec -- bun run build:android`, fixture-test flags enabled: `BUILD SUCCESSFUL`; [build log](../c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/runs/c114-generic-direct-text-20261026T023012Z-12741e45-pass01/android-avd/build.log).
- iOS `mise exec -- bun run build:ios-sim`, fixture-test flags enabled: `BUILD SUCCEEDED`; [successful pass02 log](../c11-4-generic-direct-text-intrinsic-runtime-2026-10-26/runs/c114-generic-direct-text-20261026T023012Z-12741e45-pass01/ios-simulator/build-pass02.log). The first build exposed a Swift optional-binding error; it was corrected before the successful build.
- Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator each emitted `status=0 layout=ready boxes=3`, expected root/item node frames, intrinsic inputs, contribution/track traces, and `status=0 presented boxes=3`. Full evidence and screenshots are in the linked runtime report.
- `runtime_text_count=0` and `glyph_batch_draw_count=0` on both renderers; this confirms layout and surface presentation and does not claim glyph paint.
- App/runtime independent implementation review and separate PR diff review remain pending in this record's update. Official C11.4 remains `72/213`, incomplete.
