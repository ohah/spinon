# C11.4 cyclic percentage auto-repeat 행 profile 일반화 실행 근거

이 실행은 [제한 계획](../../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md)의 네 고정 행 장면을 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 실제 V8·Stylo·Taffy·WGPU 앱 경로에서 확인한다. 일반 author CSS admission, WPT 적합성 또는 전체 C11.4 완료를 주장하지 않는다.

## 결과

두 플랫폼 모두 네 scene에서 layout/frame, ordered track trace, draw/present를 통과했다. 각 scene의 root content box는 `24×44px`이고, 아이템 frame은 prefix `(0,0,24,8)`, primary `(0,10,24,24)`, secondary `(0,10,16,16)` CSS px이다. 두 repeated-row item은 같은 행에 겹치며 contribution은 합이 아니라 최댓값 24px이다.

| Scene | Sizing pass / repeat | 최종 Rows | Collapse | suffix 시작 | Android/iOS |
|---|---|---|---|---:|---|
| `cyclic-max-content-auto-fill-prefix-suffix` | intrinsic count 1, basis 없음 → final count 1, basis 44px | `[8,11,8]px` | 없음 | `y=23px` | 각 통과 |
| `cyclic-max-content-auto-fit-prefix-suffix` | intrinsic count 1, basis 없음 → final count 1, basis 44px | `[8,11,8]px` | 없음; 반복 행을 item이 점유 | `y=23px` | 각 통과 |
| `definite-44px-auto-fill-prefix-suffix-control` | definite basis 44px, count 2 | `[8,11,11,8]px` | 없음 | `y=36px` | 각 통과 |
| `definite-44px-auto-fit-prefix-suffix-control` | definite basis 44px, count 2 | `[8,11,0,8]px` | zero-based track index 2 | `y=23px` | 각 통과 |

Android PixelCopy는 scene당 5개 RGBA 표본, 총 **20/20 exact**다. iOS Simulator Metal drawable readback도 scene당 5개, 총 **20/20 exact**다. 색상은 prefix `#2f9e44`, 겹친 secondary `#f08c00`, primary/track 밖 overflow `#e5484d`, root 배경 `#101820`이다. 표본은 track보다 큰 item의 paint 순서와 root 안 overflow를 확인한다. 이 fixture에는 border가 없고 root 바깥 overflow/border 획 표본은 없다.

## 최종 소스와 실행 환경

두 최종 실행은 같은 작업 revision을 사용했다.

- Base commit: `6150d4f289c0e2aabfd2b696748dc3c3be546827`
- Base Git tree: `55b79d5616dfff4381766c83e1625053fd37c57a`
- Android/iOS `simulator.json`의 capture-time tracked-diff SHA-256은 `f0f43809c99079c416fcefa6d26a93098e6d59581d33a0b91b78390fbc312570`로 일치한다. 이 값은 캡처 당시 전체 tracked diff 해시이며 code-only hash는 아니다.
- 현재 PR의 구현 코드만 다시 계산한 staged diff SHA-256은 `6e16c33893a15ad5e083cde731f0883b6f2e2931d92191c1a6de6c388032ac06`이다. 구현 파일 수정 시각이 Android/iOS build와 capture보다 앞서고 이후 source 수정이 없음을 [source revision 검증](./source-revision-verification.json)에 기록했다. capture-time tracked hash에 포함됐던 과거 evidence 경로는 현재 PR index에서 제외했다.
- Android: API 37 ARM64 AVD `emulator-5560`, `sdk_gphone16k_arm64`; 설치 APK SHA-256 `9f045a13d4a4839ca96be09bfdeff80e84ee3384e3fb762984e1334cca77c81d` (설치본 hash와 동일).
- iOS: iPhone 17 Pro, iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`; app executable SHA-256 `99be8687d3f23b70dae8630437ae1dbb7ab71ff16c7ded9891452e9c71425d05` (설치본과 일치; run 폴더의 `installed-app-verification.json`).
- Android AVD는 software renderer다. iOS 표본은 Metal drawable에서 compositor 제출 전 읽었다. 둘 다 실기기 GPU·최종 화면 compositor 결과가 아니다.

실행 파일:

- [계획](../../../../plan/c11-4-cyclic-row-auto-repeat-generalization.md)
- [Chrome 156 사전 비교와 독립 기대값](../c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/README.md)
- [Android 최종 run](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T182325255Z-dd43faa6/result.json)
- [iOS 최종 run](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T182411249Z-0b566316/result.json)
- Android/iOS의 각 run 디렉터리에 `simulator.json`, scene별 runtime/trace log, install log, screenshot이 있다. PNG SHA-256 목록은 각 폴더의 `screenshot-sha256.txt`다.

대표 화면:

- [Android AVD · max-content auto-fill](./android-avd-c114-cyclic-percent-tracks-android-avd-20261025T182325255Z-dd43faa6/cyclic-max-content-auto-fill-prefix-suffix.png)
- [iOS Simulator · max-content auto-fill](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261025T182411249Z-0b566316/cyclic-max-content-auto-fill-prefix-suffix.png)

## 검증과 fixture artifact 경계

- 최종 소스 수정 후 `cargo fmt --all -- --check`, `git diff --check HEAD`, fixture-module boundary (13/13), Layout opt-in 집중 테스트 (7/7), Runtime opt-in 집중 테스트 (2/2), FFI/layout/runtime opt-in all-target Clippy (`-D warnings`)가 통과했다. 해당 원시 로그는 이 폴더의 `*-final.log` 파일이다.
- 전체 workspace 테스트와 기본 workspace Clippy는 앞선 구현 반복에서 통과했다. 마지막 변경은 C11.4 layout gate/test와 플랫폼 fixture trace matcher에 한정되며, 최종 수정 후에는 위 집중 검사와 양 플랫폼 재빌드를 다시 통과했다.
- Android opt-in build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android` 성공.
- iOS opt-in build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 mise exec -- bun run build:ios-sim` 성공.

macOS ARM64 `spinon-ffi` static archive를 Rust 1.96.1 `llvm-nm`과 archive 문자열 검색으로 negative/positive control 비교했다.

| Build | SHA-256 | C11.4 host/eval FFI symbols | 네 신규 scene 문자열 |
|---|---|---:|---:|
| `cargo build --locked --no-default-features -p spinon-ffi --lib` | `5d9fafe4eac1a73576d564654dd4e77efa237332d4991df951bef78c2096e039` | 0 | 0 |
| 위 명령 + `--features c04-runtime-gpu-fixture-test` | `8cc621fd7068a5acec1c7107321b487352cf882d0cf9ec818ec526ea25bc57a3` | 2 (`new`, `eval`) | 4/4 |

각 빌드의 scanner 입력과 결과는 `ffi-*-final-archive-scan.json`에 기록했다. archive 자체는 재현 가능한 로컬 build output이며 PR에 포함하지 않는다.

이 결과는 host macOS ARM64 static archive의 fixture isolation만 입증한다. Android APK/iOS IPA 전체에서 모든 fixture payload가 빠졌다는 증거로 해석하지 않는다.

## 남은 범위

고정 opt-in scene 밖의 일반 CSS 입력, 전체 Grid 조합, WPT, 다른 Android GPU/device, 실기기, iOS compositor 이후 pixel parity는 검증하지 않았다. `X09 Grid 레이아웃` 및 C11.4 상태는 미완료이고 공식 완료 수 `72/213`을 유지한다.
