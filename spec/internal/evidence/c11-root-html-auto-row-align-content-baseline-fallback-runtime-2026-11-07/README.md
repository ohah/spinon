# C11.4 `align-content:baseline` Runtime·Simulator 근거

## 검증 범위

기본 parsed Root HTML route에서 baseline fallback의 positive·overflow 장면을 실행했다. Simulator 앱에는 대응 fixture HTML을 그대로 넣고 fixture SHA, APK/app bundle HTML SHA, runtime `layout=ready`, 6 boxes, 두 text draw 및 present marker를 확인했다. 실행 도구 [run-simulators.sh](run-simulators.sh)는 기존 설치 앱, APK/iOS app build, bootstrap HTML/JS를 먼저 보관하고 종료 때 복원 hash를 대조한다.

| Pass | 장면 | Fixture SHA-256 | Android AVD y offsets | iOS Simulator y offsets |
|---|---|---|---|---|
| pass03 | positive space, `row-gap:0%` | `c4518320b24963c3e23f73305bd9ab7dc81d609812a531d09f9c905ff2ca76f0` | `[0,9.9048,19.8095]` CSS px | `[0,10,20]` CSS px |
| pass04 | overflow, `row-gap:calc(120% - 2px)` | `96692ba2e9519674f9166ba822fd46e1da4d3bba9c8bce9e9b7229f93a1217e4` | `[0,56,112]` CSS px | `[0,56,112]` CSS px |

최종 pass03/pass04의 `candidate_implementation_source_diff_sha256`은 둘 다 `caf3c039c0cd8b1e37cc4a7f80ecab640415110dbf6228fbd2f22e8a71fd5650`으로 같다. Positive Android color bounds의 약 `0.19px` 양자화 차이는 스크린샷 DPR 2.625 픽셀 격자에 따른 값이며 분석 허용치는 1 CSS px다. Overflow 장면은 두 플랫폼 모두 exact CSS y offsets다. 분석 파일은 [pass03](pass03/screenshot-pixel-analysis.json), [pass04](pass04/screenshot-pixel-analysis.json)에 있다. 초기 pass01/pass02 캡처도 보존했고 최종 PR 화면은 hash가 일치하는 pass03/pass04를 쓴다.

## Simulator 환경과 캡처

- Android: `emulator-5554`, API 37, ARM64 AVD, DPR 2.625. Renderer log는 ANGLE/SwiftShader를 기록한다.
- iOS: iPhone 17 Pro, iOS 26.2 Simulator, DPR 3.0, `Apple iOS simulator GPU` Metal renderer.
- PNG 크기: Android `1080×2400`, iOS `1206×2622`.
- Positive screens: [Android](pass03/android/simulator.png) · [iOS](pass03/ios/simulator.png).
- Overflow screens: [Android](pass04/android/simulator.png) · [iOS](pass04/ios/simulator.png).
- 각 pass에서 app route log는 `layout=ready boxes=6`, `runtime_text_count=2`, `presented`를 확인한다. APK와 iOS bundle의 `index.html` SHA는 각 pass fixture SHA와 exact 일치한다.
- 최종 pass03/pass04 모두 설치 APK, 설치 iOS executable, 기존 build APK/app 및 bootstrap 입력이 종료 후 원래 hash로 복원됐고 `asset_and_simulator_restoration=PASS`다.

화면은 Simulator OS screenshot의 색 경계를 재는 visual evidence다. Android PixelCopy, Metal drawable readback, iOS compositor 이후 pixel parity, physical device, hardware GPU나 WPT를 입증하지 않는다. Positive scene의 unused 10px 영역 때문에 Grid 배경의 노출 top edge를 root origin으로 쓰지 않고 fixed 첫 item의 top edge를 origin으로 계산했다.

## Rust 검증

- `cargo test --locked -p spinon-layout root_html_auto_row_`: 20 passed.
- `cargo test --locked -p spinon-style-to-layout root_html_auto_row_`: 11 passed.
- `cargo test --locked -p spinon-runtime session::tests::root_html_grid_intrinsic_auto_rows`: 41 passed.
- `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- 실패 관점별 코드는 [구현/runtime 검토 20개](implementation-runtime-review-20.md)에 있다.

## 원본

[pass03 device/build/runtime/hash/screenshot 로그](pass03/environment.txt) · [pass04 device/build/runtime/hash/screenshot 로그](pass04/environment.txt) · [초기 캡처 pass01](pass01/environment.txt) · [초기 캡처 pass02](pass02/environment.txt) · [screenshot analyzer source](analyze_screenshot_pixels.py) · [bootstrap root app](root-app.js).

Xcode build transcript 원본은 tool-generated trailing whitespace를 포함하므로 pass01–04 모두 lossless gzip으로 보관했다: [pass01](pass01/ios/build.log.gz) · [pass02](pass02/ios/build.log.gz) · [pass03](pass03/ios/build.log.gz) · [pass04](pass04/ios/build.log.gz). 각 압축 로그의 byte equality와 source hash는 같은 폴더의 `build-log-sha256.txt` sidecar에 있다.
