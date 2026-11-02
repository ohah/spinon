# C11.1 parsed Root HTML column-flow implicit columns 실행 근거

**상태:** 제한 구현 PR [#1059](https://github.com/ohah/spinon/pull/1059)은 `2026-11-02T13:03:56Z` Rebase and merge됐고 merge SHA는 `16c62916b4a72de6a44a9cbd3bec021d0454541e`다. 구현/runtime 관점 20개와 PR diff 관점 30개 검토 및 앱 경로 smoke 근거를 보존한다. GitHub check run은 보고되지 않았다. C11.1/C11 완료나 공개 CSS 지원으로 승격하지 않으며 공식 완료 집계는 `72/213`이다. [구현 검토 20개](implementation-review-20.md) · [diff 검토 30개](pr-diff-review-30.md) · [PR 화면/상태 검증](pr1059-agent-browser-verification.json)

## 입력과 산출물

- 고정 HTML: [fixture](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-implicit-columns-precomparison.html), SHA-256 `432991cb1edf2a48186339431a5a4504441b0e787449baa4f8ee2703011df87a`.
- 앱 marker source: [root-app-marker.js](root-app-marker.js), SHA-256 `eadc101e28bcebaa3bc52a4f7605a65e2439644dd4b06cad1139b64845c7fee9`.
- Android APK와 iOS `.app`에 든 `index.html`은 fixture와 동일한 hash를 가진다. 두 앱의 bundled `root-app.js` hash는 `41170b50d5d2e55503423776f61bfa3e74147e3d510d4663518816058dced8be`다.
- APK SHA-256: `cd6e387ce94a60e8d30b0248baa3a73242e271384fe4de2233afc9f9522fad86`.
- iOS Simulator executable SHA-256: `dc29b3ff8061fa9f8d9f4007eb64e58812173628bfd0947478998cc7f762f661`.
- 빌드 스크립트는 fixture와 marker를 bootstrap 입력으로 잠시 복사한 뒤 종료 trap에서 원본을 복구했다. 복구 후 `examples/bootstrap/index.html`과 `root-app.js`는 보관본과 각각 일치하며 원본 SHA-256은 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다. 재현 순서는 [run-simulator-builds.sh](run-simulator-builds.sh), 각 플랫폼 build log는 아래 경로에 있다.
- 빌드 당시 기준 HEAD와 변경된 코드·fixture의 hash 목록은 [source-revision.json](source-revision.json)에 기록했다. 두 simulator에 설치된 실제 APK/app 번들 및 자산 hash 비교는 [installed-artifacts.json](installed-artifacts.json)에 있다.
- 설치 산출물, asset, Chrome 비교, runtime marker, build 성공과 원본 bootstrap 복구를 다시 대조한 요약은 [verification-summary.json](verification-summary.json)이다.
- `*.log.gz`는 도구 출력의 공백을 바꾸지 않은 lossless gzip 복사본이다. 대응하는 원본 `*.log` 파일도 로컬 evidence 디렉터리에 보존한다.

## Rust 검사

| 검사 | 결과 | 원본 로그 |
|---|---:|---|
| `cargo fmt --all -- --check` | 통과 | [fmt](rust/fmt-final.log.gz) |
| `cargo test --locked -p spinon-layout root_html_column_flow --lib` | 10/10 | [layout](rust/layout-final.log.gz) |
| `cargo test --locked -p spinon-style root_html_profile --lib` | 8/8 | [cascade](rust/style-final.log.gz) |
| fixed auto-column list allocation failure/recovery | 1/1 | [allocation](rust/style-allocation-final.log.gz) |
| `cargo test --locked -p spinon-style-to-layout root_html_column_flow --lib` | 6/6 | [projection](rust/projection-final.log.gz) |
| `cargo test --locked -p spinon-style-to-layout root_html --lib` | 32/32 | [Root HTML projection](rust/projection-root-html-final.log.gz) |
| `cargo test --locked -p spinon-runtime c11_1_root_html_column_flow --lib` | 1/1 | [runtime candidate](rust/runtime-primary-final.log.gz) |
| `cargo test --locked -p spinon-runtime root_html_grid_column_flow --lib` | 5/5 | [runtime column-flow](rust/runtime-column-flow-final.log.gz) |
| `cargo test --locked -p spinon-runtime root_html_grid --lib` | 38/38 | [Grid regression](rust/runtime-grid-final.log.gz) |
| Scoped Clippy, four affected crates, all targets, `-D warnings` | 통과 | [Clippy](rust/clippy-final.log.gz) |

전체 `spinon-style-to-layout` library suite는 **308 통과, 1 실패**다. 유일 실패는 기존 C07 aspect-ratio computed serialization 기대값 차이로, 기대 `1.77778 / 1`, 실제 `1.7777778 / 1`이다. 전체 suite green으로 보고하지 않는다. [전체 suite 원본](rust/style-to-layout-full-final.log.gz)

## Android API 37 ARM64 AVD

- 대상: `emulator-5556`, model `sdk_gphone64_arm64`, Android API 37 / release 17, ARM64 AVD. SurfaceFlinger는 ANGLE/Vulkan/SwiftShader software renderer를 보고했다.
- APK: `platforms/android/app/build/outputs/apk/debug/app-debug.apk`; `install.log`에서 설치 성공을 확인했고 설치된 APK hash가 빌드 산출물과 일치한다.
- 실행 패키지/activity: `dev.spinon.bootstrap`, `dev.spinon.bootstrap/.MainActivity`.
- 로그: [build](android-avd/build.log.gz), [install](android-avd/install.log.gz), [launch](android-avd/launch.log.gz), [runtime](android-avd/runtime.log.gz), [screenshot](android-avd/simulator.png).
- 관측: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready`, `layout=ready boxes=15`; `SPINON_C0410_DRAW status=0 presented boxes=15`. page-root frame은 viewport와 같은 `411.42856×841.52374` CSS px이다.
- screenshot SHA-256: `5ad6e8035026a0f82d1732d4fc54df0969b12154e8fbb054049ddf7b05f966b5`.

## iPhone 17 Pro / iOS 26.2 Simulator

- 대상 UDID: `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2 (23C54). 기존 C11 baseline이 실행 중인 다른 iPhone 17 Pro Simulator는 그대로 두었다.
- 앱: `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app`, bundle ID `dev.spinon.bootstrap`; 설치된 bundle의 HTML/JS/executable hash를 별도로 확인했다.
- 로그: [build](ios-simulator/build.log.gz), [bundle ID](ios-simulator/bundle-id.log.gz), [install](ios-simulator/install.log.gz), [launch](ios-simulator/launch.log.gz), [runtime](ios-simulator/runtime.log.gz), [screenshot](ios-simulator/simulator.png).
- 관측: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass root_app_marker=ready`, `layout=ready boxes=15`; `SPINON_C0410_DRAW status=0 presented boxes=15`. page-root frame은 viewport와 같은 `402×778` CSS px이다.
- screenshot SHA-256: `3b4e24410e8d968b2ee76ba9b728aaab6607b3ccdf5277f84a6a3ed8ea69ab6a`.

## 확인 범위와 한계

두 앱은 explicit columns `[30,40]`, rows `[20,40]`, `grid-auto-columns:[15,25]`, column gap `5px`, row gap `3px`, visible direct item 12개의 같은 fixture를 사용했다. Agent Browser Chrome 156 oracle의 columns `[30,40,15,25,15,25]`, rows `[20,40]` 및 12 frame은 [독립 기대값](../c11-root-html-grid-column-flow-implicit-columns-precomparison-2026-11-02/expected.json)과 정확히 일치한다. Rust runtime test는 item frame과 색상 소유 관계를 검사한다.

Simulator log는 동일 앱 route에서 JS marker, layout-ready scene 및 WGPU draw/present를 확인한다. screenshot은 보이는 6×2 color-grid 배열을 시각적으로 확인한다. PixelCopy, Metal drawable readback, OS compositor 이후 pixel equality, 실기기, hardware GPU, 성능, 일반 CSS admission, WPT 적합성은 이 실행으로 검증하지 않는다.
