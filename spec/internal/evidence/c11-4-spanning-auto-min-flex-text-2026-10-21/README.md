# C11.4 spanning auto-minimum flex track text fixture

## 고정 oracle과 범위

Chrome 155 고정 oracle의 `spanning-auto-min-flex-tracks`를 앱의 V8 → Stylo → Taffy runtime fixture에 연결했다. 입력은 100×30 CSS px Grid, `repeat(2,minmax(0,1fr))`, column gap 4px, 두 track을 span하는 Ahem 10px `nowrap` 문자열 20자다. 독립 Chrome 관찰은 columns `[48,48]`, root `100×30`, item `100×10`, item `scrollWidth=200`이다. 긴 텍스트가 flex track을 200px까지 늘리지 않는 자동 최소 크기 경계를 확인한다. [저장된 Chrome 관찰](../c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json) · [사전 비교 표와 판정](../c11-4-grid-track-sizing-precomparison-2026-10-19/README.md#구현-전-판정-집합).

이 slice는 한 개의 고정 direct-leaf text 장면이다. 일반 font/script, 여러 item의 span ordering, nested/anonymous item, WPT 또는 C11.4 전체 지원을 뜻하지 않는다. 화면은 item box를 보여주며 글리프 rasterization·text overflow 픽셀·OS compositor 이후 화면은 확인하지 않았다. 공식 C11.4 체크와 완료 집계는 미완료로 둔다.

## 구현 및 집중 검사

- FFI의 정확한 장면 allowlist와 고정 HTML source: [`flex_spanning_text.rs`](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/flex_spanning_text.rs)
- Runtime topology/style 검증, Ahem paragraph 구성: [`flex_span.rs`](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/flex_span.rs)
- Runtime frame·intrinsic input·final columns 회귀: [`c11_grid_tests.rs`](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests.rs)
- 별도 적대적 검사: [구현/runtime 실패 관점 20개](implementation-review-20.md), [PR diff 검토 20개](pr-diff-review-20.md)

통과한 명령:

- `mise exec -- cargo test -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_only_the_fixed_spanning_auto_min_flex_text_scene` — 1 passed
- `mise exec -- cargo test -p spinon-layout chrome155_spanning_auto_min_flex_tracks_match_text_contribution_and_frames` — 1 passed
- `mise exec -- cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_spanning_auto_min_flex_text_keeps_tracks_at_half_width` — 1 passed
- `mise exec -- cargo fmt --all -- --check && git diff --check` — 통과
- `mise exec -- node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs` — 통과

별도 `bun test tools/css-reference/c11-4-auto-repeat-neighbors-simulator.test.mjs` 호출은 존재하지 않는 경로를 지정해 “filters did not match any test files”로 끝났다. 해당 실패는 기능 테스트의 실패가 아니며, 실제 capture runner 스크립트는 위 `node --check` 및 아래 기기 실행으로 확인했다.

## Android 실기기

- 기기: Samsung SM-S731N, Android 16/API 36, ARM64, 1080×2340, density 450, `qemu=0`; Samsung Xclipse 940 / Vulkan renderer.
- 첫 캡처는 `SPINON_C114_GRID_ARGUMENT_ERROR rejected=true`로 실패했다. ADB 연결은 정상이고 당시 local/install APK SHA-256은 서로 같았지만 `b9c803c31ffcfa178388a5b8d134d9372c2d4480162b7d80d5adc2d31a2390e3`인 이전 APK였다. capture runner는 APK를 빌드하지 않고 기존 파일을 설치하므로 새 장면 allowlist를 포함하지 않은 stale artifact가 원인이었다. [기기 정보·첫 설치 hash](android-device-c114-spanning-flex-text-android-device-20261021T000107087Z-00e70fc2/device.json) · [실패 runtime log](android-device-c114-spanning-flex-text-android-device-20261021T000107087Z-00e70fc2/runtime-spanning-auto-min-flex-tracks.log).
- `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android`로 fixture APK를 새로 빌드했다. 빌드 성공 후 같은 실기기 capture를 재실행했다. local/install APK SHA-256은 `d67d89fbdf4158460a4a376e301718d1a9b6520ba1d07277f13cdea34bee8922`로 일치했다.
- 통과 결과: scene 1/1, layout ready, 두 box, root `100×30`, item `100×10`, final columns `48/48`, track trace 6 chunks, WGPU draw/present true, superseded retries 0. [기기·APK 확인](android-device-c114-spanning-flex-text-android-device-20261021T000224674Z-96374cf5/device.json) · [설치 APK hash](android-device-c114-spanning-flex-text-android-device-20261021T000224674Z-96374cf5/installed-apk-verification.json) · [실행 결과](android-device-c114-spanning-flex-text-android-device-20261021T000224674Z-96374cf5/result.json) · [로그](android-device-c114-spanning-flex-text-android-device-20261021T000224674Z-96374cf5/runtime-spanning-auto-min-flex-tracks.log) · [실기기 화면](android-device-c114-spanning-flex-text-android-device-20261021T000224674Z-96374cf5/spanning-auto-min-flex-tracks.png)
- Android 화면 SHA-256: `694b4f0c15a430ad2c7ca64b7f5879bc46f98ecd8f7978ddd0ff07d4dfe204fe`.

## iOS Simulator

iPhone 17 Pro / iOS 26.2 Simulator에서도 같은 scene을 실행했다. iOS Simulator 빌드와 설치가 성공했고 root/item frame, `48/48` tracks, trace 11 chunks, draw/present를 확인했다. [Simulator 결과](ios-simulator-c114-spanning-flex-text-ios-simulator-20261021T000424988Z-24d91db2/result.json) · [설치·실행 로그](ios-simulator-c114-spanning-flex-text-ios-simulator-20261021T000424988Z-24d91db2/launch-spanning-auto-min-flex-tracks.log) · [화면](ios-simulator-c114-spanning-flex-text-ios-simulator-20261021T000424988Z-24d91db2/spanning-auto-min-flex-tracks.png). iOS 화면 SHA-256: `9f901d60df12d2f2a5fb9591fcbd9e59a623ca1c16ebaff8e58614c64284f980`.

## 한계와 판정

Android 실기기와 iOS Simulator는 이 한 장면의 runtime layout, intrinsic input/track trace 및 present 신호를 확인했다. Chrome `scrollWidth=200`은 독립 브라우저 oracle이다. Spinon의 화면에서 글리프가 그려지거나 200px 텍스트의 화면상 overflow가 표시됐다는 근거는 아니다. surface PixelCopy 색 표본, iOS compositor 이후 픽셀, WPT, 전체 C11.4는 검증하지 않았다. 따라서 이 fixture는 후속 slice 근거이며 `spec/STATUS.md`의 C11.4 체크와 완료 수를 올리지 않는다.
