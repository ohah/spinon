# C11.1 parsed Root HTML named-area + fractional-track 실행 근거

2026-11-02 기준. 이 자료는 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 고정된 named-area map과 두 definite 축의 direct `1fr 2fr`를 조합한 제한 구현 후보다. C11 전체, 일반 CSS Grid 지원, WPT 적합성 완료를 뜻하지 않는다. 공식 상태 `72/213`과 C11 상위 checkbox는 바꾸지 않는다.

## 입력과 독립 기대값

- 입력: [`root-html-grid-fr-named-areas-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-fr-named-areas-runtime.html)
- Chrome 156 Agent Browser 사전 관측과 기준 자료: [precomparison README](../c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/README.md), [expected values](../c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/expected.json), [화면](../c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/chrome-agent-browser.png)
- 표준 계산은 `120×70px` definite root에서 열 `[40,80]`, 행 `[70/3,140/3]` CSS px다. 예상 semantic frame은 header `[0,0,120,70/3]`, side `[0,70/3,40,140/3]`, content `[40,70/3,80,140/3]`다. Chrome DOMRect는 1/64 CSS px 단위로 양자화되므로 Rust geometry 비교 허용치는 독립값 기준 0.02 CSS px다.
- 재배치 control은 CSS selector/id를 바꾸고 child source order를 재정렬한 뒤 동일 area frame을 확인한다.

## Rust 검증

최종 구현 조건 정리와 Clippy 수정 후 다음 순서로 통과했다.

| 명령 | 결과 |
|---|---:|
| `cargo test -p spinon-runtime --lib root_html_grid` | 22 passed |
| `cargo test -p spinon-runtime --lib root_html` | 65 passed |
| `cargo test -p spinon-style-to-layout --lib root_html_named_area` | 5 passed |
| `cargo test -p spinon-style-to-layout --lib c11_3_3` | 26 passed |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --locked -p spinon-runtime -p spinon-style-to-layout -p spinon-style --all-targets -- -D warnings` | passed |

Fractional named-area rejection controls cover wrong ratio/count, mixed px/fr and percentage tracks, indefinite row axis, min/max/aspect-ratio constraints, both nonzero gaps and intrinsic `minmax()`. Every case fails without a completed scene. The `minmax()` parser/cascade path reports `cascade_failed`; the test intentionally asserts fail-closed behavior and no partial output rather than imposing a projection-stage diagnostic. Other named-area rejection controls remain in the existing Root HTML suite.

During validation, Clippy found one non-minimal boolean in the production allowlist; it was simplified and the scoped Clippy and tests passed again. An initial negative assertion also expected `unsupported_computed_value` for intrinsic `minmax()`, while the parser correctly rejects it earlier at cascade; the test was narrowed to the stable failure/no-partial-scene contract. No implementation defect remained from these findings.

## Android API 37 AVD

- Emulator `emulator-5556`, API 37 / Android 17, `arm64-v8a`; renderer ANGLE → Vulkan → SwiftShader software device.
- Final-source app build: `pass03/android-avd/build.log` (`BUILD SUCCESSFUL`). Android Gradle Plugin 8.13.2 warns that compile SDK 37.2 is newer than its tested 36.1 ceiling; the build still succeeded. Install, launch and runtime: `pass03/android-avd/install.log`, `pass03/android-avd/launch.log`, `pass03/android-avd/runtime.log`.
- The default Root HTML path reported `layout=ready`, `boxes=6`, `document_revision=2`; WGPU draw/present reported six boxes and root CSS height `70px` at viewport `411.42856×841.52374 CSS px`.
- [Final-source pass03 screenshot](pass03/android-avd/c11-root-html-grid-fr-named-areas.png) is visual smoke evidence. No Android `PixelCopy` surface readback was performed. This AVD software renderer is not physical-device or hardware-GPU evidence.

## iOS Simulator

- iPhone 17 Pro Simulator, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`; Metal simulator GPU.
- Final-source app build: `pass03/ios-simulator/build.log` (`BUILD SUCCEEDED`). The build contains duplicate `RawWindowMetalLayer` linker/debug-map warnings and an App Intents metadata warning; they did not prevent the simulator build. Install, launch and runtime: `pass03/ios-simulator/install.log`, `pass03/ios-simulator/launch.log`, `pass03/ios-simulator/unified-log.txt`.
- Human-readable Xcode logs have trailing whitespace and terminal blank lines normalized for `git diff --check`; byte-exact originals are retained as adjacent `*.log.raw.gz` files and verified by `*.log.raw.sha256` sidecars.
- The default Root HTML path reported `layout=ready`, `boxes=6`, `document_revision=2`; Metal draw/present reported six boxes at viewport `402×778 CSS px` and root CSS height `70px`.
- [Final-source pass03 screenshot](pass03/ios-simulator/c11-root-html-grid-fr-named-areas.png) is visual smoke evidence. No Metal drawable readback was performed; this does not establish compositor pixels or physical iPhone behavior.

## Harness correction and input hashes

The first simulator launches reached the fixture document but the stock `root-app.js` boot assertion rejected it because that assertion expects the stock demo DOM. The iOS log records `script-or-layout-failed`; this was a harness mismatch, not a Grid layout failure. The first-pass build, launch and screenshot artifacts remain in the parent `android-avd/` and `ios-simulator/` directories. For pass02, `index.html` was temporarily set to the fixture and `root-app.js` to a minimal readiness marker, following the repository's established C11 simulator-evidence pattern. Both tracked bootstrap files were restored and their restored hashes match the originals below.

The source trees used for pass02 and the final-source pass03 were the same. `pass02-source-hashes.txt` and `pass03/input-hashes-before-build.txt` record:

- fixture / packaged `index.html`: `903450133a04eba9acd51cf27f1a7ae897e0dda4933bdfdd4486cef566786d94`
- original and restored `examples/bootstrap/index.html`: `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`
- original and restored `examples/bootstrap/root-app.js`: `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`
- temporary readiness-marker `root-app.js`: `c89ca5f95c8b1d5356781580664c2fffc87b3394ec3d2f0c1f62341525355fc8`
- Android and iOS builds: passed.

For pass03 the APK's `assets/index.html` and iOS app bundle's `index.html` were both extracted/hashed as `903450133a04eba9acd51cf27f1a7ae897e0dda4933bdfdd4486cef566786d94`, equal to the fixture hash. [`implementation-source-hashes.txt`](pass03/implementation-source-hashes.txt) binds the base commit, modified production validator source and fixture used for this pass. `pass03/input-hashes-after-restore.txt` confirms the source bootstrap files still match their original hashes after the final build. The same final-source pass03 run reports Android API 37 AVD `layout=ready`/6 boxes and iOS 26.2 Simulator `layout=ready`/6 boxes; see the platform runtime logs above.

No fixture FFI, scene ID, public API, or contract-version change was added. The HTML fixture is consumed by Rust unit tests and the simulator evidence build; screenshots are not pixel-readback proofs.
