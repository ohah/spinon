# C13.1 compact 13-case surface matrix 실행 근거 · 2026-10-14

## 범위

Chrome 155에 고정한 C13.1 compact fixture를 Android API 37 ARM64 AVD의 300×100 CSS-px runtime surface와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행했다. 이 근거는 C13.1의 fixture 전용 slice다. 52개의 고정 PixelCopy 표본과 PNG를 검사했으며 300×100 전체 pixel byte 비교, 제품 CSS 지원, DOM event dispatch, WPT 또는 C13.1 완료를 뜻하지 않는다.

## Android API 37 AVD

- AVD `spinon_api37_1_compat16k`, serial `emulator-5554`, API 37 / Android 17, `ro.kernel.qemu=1`, `arm64-v8a`다. 기기 표시 영역은 1080×2400 px / 420 dpi다.
- `SurfaceFlinger`는 ANGLE 2.1.1 / Vulkan 1.3.0 / SwiftShader를 보고했다. 실기기나 hardware GPU 결과가 아니다.
- fixture runtime에서 41/41 node frame, 52/52 fixed hit identity와 52/52 PixelCopy RGBA가 Chrome oracle과 일치했다. PixelCopy는 788×263 px, scale x `2.6266667`, y `2.63`이었다. 표면 크기·density 비율 검증은 통과했다.
- [PixelCopy surface PNG](android-api37-avd-pixelcopy.png), SHA-256 `d585f3c4b06558e292c7d2553edd18636bf75da64edcf9c1971ee665a7ca59d9` (788×263).
- [전체 앱 화면](android-api37-avd-screen.png), SHA-256 `8cb809e9ffccf9fe83b40ad72088d2948ed95e586258c842339360d2340448df` (1080×2400). 화면 상태는 `status=0 layout=ready boxes=41`이다.
- [Android runtime log](android-api37-avd-runtime.log)는 앱 PID 10092의 successful run, frame/hit report, 52 PixelCopy 표본, PNG hash를 보존한다. [route collision negative control](android-avd-route-collision-negative-control.log)은 서로 다른 C13.1 route 두 개를 동시에 지정했을 때 `multiple-fixture-routes`로 거부된 결과다.

## ADB 및 runtime 실패 원인

ADB transport에는 문제가 없었다. `adb devices -l`은 `emulator-5554`를 `device`로 보고했고 API 37 ARM64 AVD에 앱을 설치·실행했다. 최초 앱 실행 실패는 Android Gradle/build shell이 예전 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST`만 읽고, 공개 실행 명령이 사용한 `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST`를 전달하지 않아 `SPINON_C131_DISABLED`로 종료한 flag 이름 불일치였다. build script와 Gradle은 canonical 이름을 우선 사용하고 이전 이름을 호환 alias로 받으며, 값이 서로 다르면 조기에 거부한다.

호스트의 기본 PATH에는 `emulator` 명령이 없었지만 Android SDK의 `emulator/emulator` 파일은 있었고 AVD는 이미 실행 중이었다. 이는 AVD 관리 명령의 PATH 문제였으며 `adb` 연결·app install·runtime eval 실패 원인은 아니었다.

Flag를 맞춘 다음 실제 runtime에서는 host document에 `document.body`가 없어서 `appendChild` 호출이 실패했다. fixture가 `document.body || document`에 연결하도록 수정했다. 다음 실행에서는 compact route의 300×100 viewport가 공통 environment 적용에서 320×240으로 바뀌었다. compact route 전용 viewport 값을 environment 경로에서도 고정했다. 이후에도 52개 중 7개 색이 달라, parent background가 정적 in-flow child보다 늦게 paint되는 renderer 순서 문제를 재현했다. auto positioned parent의 단순 정적 subtree paint 순서를 overflow 값과 분리해 고치고 visible, clip, hidden, mixed `clip/auto` Rust 회귀를 추가했다. 현재 52/52 PixelCopy가 exact match다. 단계별 ADB 로그는 [실패 진단 로그](android-avd-failure-diagnosis.log)에 남겼다.

## iOS Simulator

- iPhone 17 Pro / iOS 26.2 Simulator, arm64 simulator build에서 동일한 source를 실행했다.
- 41/41 frame과 52/52 Chrome hit identity가 일치했다. [Unified log](ios-26-2-simulator-runtime.log)는 geometry·hit summary를 포함한다.
- [Simulator 전체 화면](ios-26-2-simulator-screen.png), SHA-256 `9790a2a937587fc21666ce0104f9ca2a3d3ac283b82221cbf84a8161afe224c5` (1206×2622). 화면은 Metal surface 결과를 보여 주지만 Metal drawable readback, compositor 제출 완료 또는 scanout을 증명하지 않는다.
- 최종 linked app executable SHA-256 `9aad17a1bf0238dfbe6674b7a5a2c588cca766ee0977b42e8deccb5398c7243a`.

## Fixture feature 경계와 검사

- [feature isolation](feature-isolation.txt)은 matching Rust 1.96.1 / LLVM 22.1.2 `llvm-nm`과 `strings`로 기본 FFI static archive에서 새 C ABI symbol과 compact payload가 빠지고, opt-in positive control에는 들어가는 것을 확인했다. Apple 시스템 `nm`은 LLVM 22 object attribute를 읽지 못해 사용하지 않았다. 이 검사는 host static archive에 한정되며 최종 앱 artifact 크기나 제품 빌드 전체를 보증하지 않는다.
- `node --test tools/css-reference/c13-1-android-build-flags.test.mjs tools/css-reference/c13-1-compact-surface-matrix.test.mjs`: 3/3 통과.
- `cargo test --locked -p spinon-style-to-render`: 54 통과, 0 실패, 1 ignored.
- `cargo test --locked -p spinon-ffi --features c13-1-overflow-clip-fixture-test runtime_gpu::fixtures::c13_1_compact_surface_matrix::tests -- --nocapture`: 2/2 통과.
- `cargo test --locked --workspace`: 775 통과, 0 실패, 4 ignored. [workspace log](workspace-test.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`: 통과.
- Canonical C13.1 flag로 Android build와 iOS Simulator build가 성공했다. [Android build log](android-build.log), [iOS build log](ios-simulator-build.log). iOS static archive link에는 `spinon_wgpu_r08_spike.a`와 `libspinon_ffi.a`의 duplicate `raw_window_metal` symbols 및 debug-map 경고가 있었지만 Xcode build는 성공했다.
- Android의 서로 다른 canonical/legacy flag 값은 [negative control log](android-conflicting-build-flags-negative-control.log)에서 exit 2로 거부됐다. AVD에서 C13.1 compact/overflow route 동시 실행은 [route collision log](android-avd-route-collision-negative-control.log)에서 host 생성 전에 거부됐다.
- 공개 저장소에 보존하는 build/test log에서 홈·임시 worktree·외부 V8 source 경로와 owner 이름은 placeholder로 익명화했다. 실행 결과, flag 값, 오류와 성공 상태는 유지했다.

## 고정 oracle과 남은 범위

[Chrome 155 사전 비교](README.md)는 13 frame·41 node·52개의 element identity와 실제 screenshot RGBA oracle이다. Android는 전체 SurfaceView PNG와 52개 표본을 확인했다. iOS는 frame·hit query와 Simulator 화면을 확인했으며 drawable readback은 없다. 전체 surface byte equality, 더 넓은 frame 밖 paint/hit/event controls, scroll/CSSOM 상태, border-radius/transform, float parity, DOM event dispatch, WPT 실행과 C13.1 완료 gate는 남아 있다.
