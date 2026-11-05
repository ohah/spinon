# C11.1 Root HTML Grid auto-repeat runtime evidence · 2026-11-05

## 범위와 판정

이 evidence는 `RuntimeRootHtmlTextPaintV1` 기본 Root HTML 경로에 한정된 `RootHtmlAutoRepeatV1` 후보를 기록한다. Android와 iOS는 시뮬레이터에서 같은 Root HTML fixture를 실행했다. 이 결과는 일반 CSS Grid 지원, C11.1/C11 완료, WPT 적합성 또는 하드웨어 GPU 동작을 뜻하지 않는다. 공식 상태 `78/214`와 C11 체크는 그대로 둔다.

## Rust 검증

실행 로그는 이 디렉터리에 보존한다.
텍스트 로그는 명령/시스템 출력의 순서를 유지하면서 행 끝 공백만 정규화했다.

| 명령 | 결과 |
|---|---:|
| `cargo test --locked -p spinon-style-to-layout root_html_auto_repeat -- --nocapture` | 9 passed; [pass 02 log](style-to-layout-focused-pass02.log) |
| `cargo test --locked -p spinon-runtime root_html_grid_auto_repeat -- --nocapture` | 4 passed; 26 unsupported Root HTML inputs fail closed without partial scene; [pass 02 log](runtime-focused-pass02.log) |
| `cargo test --locked -p spinon-layout auto_repeat_neighbor -- --nocapture` | 3 passed; neighboring auto-repeat regression guard; [pass 02 log](layout-focused-pass02.log) |
| `cargo fmt --all -- --check` | passed; [pass 02 log](rustfmt-pass02.log) |
| `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings` | passed; [pass 02 log](clippy-pass02.log) |

Rust suites compare 13 fixed Chrome 156 scenes against independent expected values, exercise one- and two-track patterns, 169/170px repeat count, gutter contribution, the count-only UA 1px floor, finite/overflow guards and Taffy's 10,000-track cap. Runtime cases include rows and columns, `auto-fill`/`auto-fit`, empty/collapsed cases, visible-item capacity and atomic rejection. The 26 negative scenes cover unsupported grammar, container and item styles.

## V8 및 앱 빌드

| 대상 | 명령 | 결과 |
|---|---|---|
| Android ARM64 V8 | `SPINON_V8_JOBS=6 bash tools/v8/build-android-macos.sh` | 2,975/2,975 jobs; [log](android-v8-build-pass02.log) |
| iOS Simulator ARM64 V8 | `SPINON_V8_JOBS=6 bash tools/v8/build-ios-iphonesimulator.sh` | 2,701/2,701 jobs; [log](ios-v8-build-pass01.log) |
| Android app | `mise exec -- bun run build:android` | 통과; [final-source pass 02 log](android-app-build-pass02.log) |
| iOS Simulator app | `mise exec -- bun run build:ios-sim` | 통과; [final-source pass 02 log](ios-app-build-pass02.log) |

V8 revision은 `7b50b62cb18f28617959e8452e2cd18195b38bcf`, depot_tools revision은 `41c9bd890277c2f551499d171d215dfdf5dab97d`다. Build log에는 앱 fixture를 위해 `examples/bootstrap/index.html`을 임시 교체한 과정이 포함된다. 빌드 후 파일을 원래 내용으로 복구했고 확인 SHA-256은 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`다.

## Android API 37 AVD

- 환경: Android 17 / API 37 ARM64 AVD, 모델 `sdk_gphone64_arm64`, emulator `emulator-5554`; 실기기 아님.
- APK: `platforms/android/app/build/outputs/apk/debug/app-debug.apk`.
- Packaged `assets/index.html` SHA-256 equals the native fixture: `4cbd7f40f320d802ddf7ca19151610ea8c2f5d647068ef3efeaae88f147eaaab`.
- 기본 앱 경로가 `source=index.html`로 진입하고 `layout=ready boxes=14`, `status=0 presented boxes=14`, root `411.42856×210 CSS px`를 기록했다.
- 로그: `android-install-pass02.log`, `android-launch-pass02.log`, `android-spinon-log-pass02.txt` (SpinonBootstrap tag만 추린 logcat).
- 캡처: [`android-simulator-pass02.png`](android-simulator-pass02.png), 1080×2400 OS screenshot.

## iOS Simulator

- 환경: iPhone 17 Pro / iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`; 실기기 아님.
- Executable: `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/SpinonBootstrap`.
- Bundled `index.html` SHA-256 equals the same native fixture: `4cbd7f40f320d802ddf7ca19151610ea8c2f5d647068ef3efeaae88f147eaaab`. The app also bundles `root-html-default-rendering.html`, but the captured default route explicitly selects `index.html`.
- 기본 앱 경로가 `source=index.html`로 진입하고 `layout=ready boxes=14`, `status=0 presented boxes=14`, root `402×210 CSS px`를 기록했다.
- 로그: `ios-install-pass02.log`, `ios-launch-pass02.log`, `ios-screenshot-pass02.log`, `ios-system-log-pass02.txt`.
- 캡처: [`ios-simulator-pass02.png`](ios-simulator-pass02.png), 1206×2622 OS screenshot.
- iOS link 단계는 `libspinon_wgpu_r08_spike.a`와 `libspinon_ffi.a` 사이 `RawWindowMetalLayer` duplicate-symbol warning을 기록했지만 build는 성공했고 Simulator route가 정상 표시됐다. 이 별도 linker warning은 이 C11 slice에서 조사하지 않았다.

두 화면은 기본 앱 route에서 세 개의 repeat scene이 보이는지 확인하는 **시각 smoke**다. 화면 캡처는 Android PixelCopy나 iOS Metal drawable readback이 아니며, 하드웨어 렌더러나 compositor 이후 pixel equality를 증명하지 않는다.

## SHA-256

구현/runtime 실패 관점 20개는 [구현 review](implementation-runtime-review-20.md), candidate diff의 20개 공격 관점은 [diff review](pr-diff-review-20.md)에 있다. PR 생성 후 GitHub PR diff가 검토한 candidate diff와 같은지 별도로 확인한다.

앱·fixture·화면·V8 library·bootstrap 원본의 해시는 [`hashes.sha256`](hashes.sha256)에 있다. Android APK와 iOS app에서 실제 실행한 `index.html`의 추출 해시는 [`bundle-input-hashes.txt`](bundle-input-hashes.txt)에 보존한다.
