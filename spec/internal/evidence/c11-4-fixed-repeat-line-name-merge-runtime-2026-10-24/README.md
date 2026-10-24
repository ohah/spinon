# C11.4 fixed-repeat named-line merge · 구현 후보 실행 근거

## 범위

`repeat(<positive integer>, <1–2 computed nonnegative px tracks>)` line-name metadata를 단일 `auto-fill` 또는 `auto-fit`과 함께 cascade snapshot부터 Taffy named-line placement까지 보존한다. fixed/fixed, fixed/ordinary, fixed/auto 경계를 authored order로 병합하고 auto-repeat 내부 `N+1` groups는 분리한다. 이 변경은 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture profile에만 연결되며 public CSS admission, C11.4 전체 또는 C11 완료를 의미하지 않는다.

## 실행 결과

| 확인 | 결과 |
| --- | --- |
| `spinon-style` fixed-repeat regression | 16/16 passed |
| `spinon-style-to-layout` fixed-repeat named-line integration | 4/4 passed |
| `spinon-ffi` named fixed-repeat scene test | 1/1 passed |
| CSS fixture module boundary | 11/11 passed |
| workspace `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, staged/worktree `git diff --check` | 통과 |
| Android API 37 ARM64 AVD | 3/3 scene, exact root/item frame·track trace·draw/present, PixelCopy sample 12/12 exact |
| iPhone 17 Pro / iOS 26.2 Simulator | 3/3 scene, exact root/item frame·track trace·draw/present, pre-compositor Metal drawable sample 12/12 exact |
| default/opt-in release FFI archive | default marker 0/3와 fixture ABI/Rust symbol 없음; positive control marker 3/3와 두 symbol 존재 |

재실행 가능한 focused check 명령과 원본 로그는 [`focused-checks.json`](focused-checks.json) 및 `*-*.log`에 있다. 기본/positive-control archive를 다시 만드는 scanner와 결과는 [`artifact boundary`](artifact-boundary/README.md)에 기록했다.

## Runtime 결과와 screenshots

세 장면은 columns `auto-fill`, columns `auto-fit`, rows `auto-fill`이다. 각 앱의 root/item frame은 independent expectation과 정확히 같고, auto-fit에서 점유되지 않은 auto-repeat track 다섯 개가 collapse됐다. capture script의 track arrays, app runtime logs, sample 좌표·RGBA 결과와 app/device metadata는 아래 run folders에 있다.

| 장면 | Android AVD | iOS Simulator |
| --- | --- | --- |
| columns auto-fill | [screenshot](android-avd-c114-fixed-repeat-named-lines-android-avd-20261024T124954799Z-38c2483b/auto-repeat-named-fixed-repeat-column-fill.png) | [screenshot](ios-simulator-c114-fixed-repeat-named-lines-ios-simulator-20261024T125412718Z-28ed5da7/auto-repeat-named-fixed-repeat-column-fill.png) |
| columns auto-fit | [screenshot](android-avd-c114-fixed-repeat-named-lines-android-avd-20261024T124954799Z-38c2483b/auto-repeat-named-fixed-repeat-column-fit.png) | [screenshot](ios-simulator-c114-fixed-repeat-named-lines-ios-simulator-20261024T125412718Z-28ed5da7/auto-repeat-named-fixed-repeat-column-fit.png) |
| rows auto-fill | [screenshot](android-avd-c114-fixed-repeat-named-lines-android-avd-20261024T124954799Z-38c2483b/auto-repeat-named-fixed-repeat-row-fill.png) | [screenshot](ios-simulator-c114-fixed-repeat-named-lines-ios-simulator-20261024T125412718Z-28ed5da7/auto-repeat-named-fixed-repeat-row-fill.png) |

- Android: emulator `emulator-5558`, API 37, `arm64-v8a`, `sdk_gphone64_arm64`, APK SHA-256 `73824efd51bcc496e14380b80b5cd94b288d46ad161e654507f72683540c519a`. This is an AVD/software-rendered result, not physical-device or hardware-GPU evidence.
- iOS: iPhone 17 Pro / iOS 26.2 Simulator, app SHA-256 `f692f5accd7b29d0454bf323eccb0a40db7f36c9c233cb45f5ff77d9273e4d88`. Metal readback is before OS compositor output.
- The initial iOS capture used a build without the Metal pixel-test flag and correctly failed before pixel verification. Rebuilding with `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1` produced the successful run above. Xcode also reported existing duplicate `RawWindowMetalLayer`/V8 debug-map warnings; the simulator run completed successfully.
- The verbose Xcode build transcript is preserved as `ios-build.log.gz`; Xcode's generated transcript contains whitespace-only lines and is compressed in the evidence bundle.
- Both runs bind to source commit `626dd9e1f220b41c9e878257a4a5c74e833bd7b6`, tree `35f9bdbb13f2eff7f65be7167ad87a83db950540`, and implementation diff SHA-256 `49bffb910fe7db6ae5447511d7d9eb815dd0b46ab5878bda29e454ea0ef9e378`.

## Review and remaining gates

- [20 implementation failure perspectives](implementation-review-20.md) found no blocking source defect within this slice.
- [The separate 30-perspective PR diff review](pr-diff-review-30.md) found no blocking diff issue. GitHub CI/check state is still empty; merge and post-merge main verification remain.
- Plan and official status remain limited: C11.3.3 template-areas follow-up and broad C11.4 track-sizing scope are open; C11 and official `72/213` remain unchanged.
