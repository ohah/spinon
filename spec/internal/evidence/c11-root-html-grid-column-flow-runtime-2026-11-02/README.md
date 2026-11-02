# C11.1 parsed Root HTML sparse column-flow 후보 runtime 근거

**상태:** 구현 후보. 지정 Rust 검사, Android/iOS Simulator 기본 app-route smoke와 [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md)를 마쳤다. PR diff review, PR 및 merge가 아직 남아 있어 C11.1/C11 완료로 세지 않는다. 공식 집계는 `72/213` 그대로다.

## 구현 전 기준

- 고정 입력: [Root HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-precomparison.html)
- 독립 계산과 Chrome 156 관측: [사전 비교 README](../c11-root-html-grid-column-flow-precomparison-2026-11-02/README.md), [expected.json](../c11-root-html-grid-column-flow-precomparison-2026-11-02/expected.json)
- fixture SHA-256: `a669117f0103975a91bbe3e83e5d963b3927be977e5fdd7a96c7d23deac8ae26`
- App-route marker source: [root-app-marker.js](root-app-marker.js), SHA-256 `8211f52ffba342abccd11c81df356bb47739b7dc3125d0ed01ba2d6d516e603f`
- CSS viewport oracle: `1280×633`; root grid `100×60 CSS px`; explicit columns `[30, 70]`, rows `[20, 40]`; zero-gap baseline
- source-order placement: item A `(0,0,30,20)`, B `(0,20,30,40)`, C `(30,0,70,20)`, D `(30,20,70,40)`

Rust runtime 검사는 위 Chrome observation과 별도로 typed tracks와 semantic color별 item frame을 직접 비교한다. `row-gap:3px; column-gap:5px` control은 각 gutter가 해당 축에서 한 번 적용되는지 검사한다.

## Rust 검사

최종 iterator 검증 변경 뒤 실행했다.

| 검사 | 결과 | 로그 |
|---|---:|---|
| `cargo test --locked -p spinon-layout --lib root_html_column_flow` | 4/4 | [layout.log](rust/layout.log) |
| `cargo test --locked -p spinon-style --lib` | 267/267 | [style-cascade-full.log](rust/style-cascade-full.log) |
| `cargo test --locked -p spinon-style-to-layout --lib root_html_column_flow` | 3/3 | [projection.log](rust/projection.log) |
| `cargo test --locked -p spinon-runtime --lib root_html_grid_column_flow` | 4/4 | [runtime.log](rust/runtime.log) |
| `cargo test --locked -p spinon-runtime --lib root_html_grid` | 37/37 | [runtime_grid_regression.log](rust/runtime_grid_regression.log) |
| Scoped Clippy: layout/style-to-layout/runtime, `--all-targets -- -D warnings` | 통과 | [clippy.log](rust/clippy.log) |
| `cargo fmt --all -- --check` | 통과 | [fmt.log](rust/fmt.log) |

`cargo test --locked -p spinon-style-to-layout --lib` 전체는 **305/306**이다. 유일 실패는 변경 경로와 무관한 기존 `c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs` computed-string 기대값 차이다: 기대 `1.77778 / 1`, 관측 `1.7777778 / 1`. 전체 suite를 통과로 보고하지 않는다. [전체 suite 로그](rust/style-to-layout-full.log)

## Android API 37 ARM64 AVD

- 대상: `emulator-5556`, `spinon_api37_0_applied_txn`; emulator ABI `arm64-v8a`, API 37, SwiftShader software GPU
- 실행: `mise exec -- bun run build:android`; 설치된 패키지 `dev.spinon.bootstrap`; activity `dev.spinon.bootstrap/.MainActivity`
- 최종-source APK SHA-256: `a3ba1d9f8616cd3e8994361a2e6a74fd06dea451ec6a14e9cde54658af1e40d1`
- APK `assets/index.html` SHA-256은 고정 fixture와 동일하다. bundled `assets/root-app.js` SHA-256은 `da5cc944e93b5be89c37ae78670fbd37b172c2d8c3275ebe996c9adf2e08c999`이다.
- 관측: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready boxes=7`, draw/present `boxes=7`; runtime root frame `411.42856×60 CSS px`
- 원본: [build log](android-avd/build.log), [install log](android-avd/install.log), [runtime log](android-avd/runtime.log), [screenshot](android-avd/simulator.png)

## iPhone 17 Pro / iOS 26.2 Simulator

- 대상 UDID: `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`
- 실행: `bash tools/build-ios-sim.sh`; app bundle `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app`; bundle ID `dev.spinon.bootstrap`
- 최종-source Simulator executable SHA-256: `5ec4cb2ede0d6a102183c463c6719a63be9cca690f5444314f76d4350c04b4b6`
- bundled `index.html`은 fixture SHA-256과 동일하다. bundled `root-app.js` SHA-256은 Android APK asset과 동일한 `da5cc944e93b5be89c37ae78670fbd37b172c2d8c3275ebe996c9adf2e08c999`이다.
- 관측: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready boxes=7`, draw/present `boxes=7`; runtime root frame `402×60 CSS px`
- 원본: [build log](ios-simulator/build.log), [runtime log](ios-simulator/runtime.log), [screenshot](ios-simulator/simulator.png)

두 app은 같은 고정 HTML asset과 변환된 root-app marker script를 사용했다. iOS Simulator 로그에는 CoreAnimation app-launch metric 전송 오류 두 줄이 있지만, 제품 runtime marker/layout/draw 상태는 모두 status 0으로 통과했다. Screenshots는 2×2 색상 배치와 화면 표시만 시각적으로 확인한다. 이 검증은 PixelCopy, Metal drawable readback, compositor 후 픽셀 parity, 실기기 또는 hardware GPU를 포함하지 않는다. Chrome oracle viewport와 simulator app viewport는 다르며, 정확한 fixture frame 일치는 Rust runtime 검사가 맡는다.

## 판정 범위

이 결과는 명시 2×2 fixed-px Grid의 sparse column auto-placement 제한 후보만 검증한다. Dense flow, explicit item placement, implicit sizing/tracks, repeat/line names/areas, percentage 또는 fractional explicit tracks, arbitrary item content, 일반 CSS admission, 전체 C11/C11.1 및 WPT 적합성은 포함하지 않는다. Simulator draw/present는 지원 계약이나 전체 기능 완료를 뜻하지 않는다.
