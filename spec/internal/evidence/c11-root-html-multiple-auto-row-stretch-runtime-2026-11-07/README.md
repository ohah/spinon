# C11.4 parsed Root HTML 두 implicit auto 행 stretch 구현 근거

## 대상

- 계획과 독립 기대값: [구현 계획](../../../../plan/c11-root-html-multiple-auto-row-stretch.md), [HeadlessChrome 157 사전 비교](../c11-root-html-multiple-auto-row-stretch-precomparison-2026-11-07/README.md)
- 규범 근거: [CSS Grid Layout Module Level 1 §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). `align-content: normal`에서 양의 definite free space를 auto max track 두 개에 균등 분배한다.
- fixture: [root-html-multiple-auto-row-stretch-runtime.html](../../../../tests/fixtures/css/c11/root-html-multiple-auto-row-stretch-runtime.html), SHA-256 `05b781873d7fa88b73f19f4dde4acb069f1f44bdbbb204c6613576653917d061`.
- 승인 모양은 40×40px parsed Root HTML Grid, explicit row 10px, fixed empty item 하나, direct text `A`와 `B`다. Used row sizes `[10,15,15]px`; item frames `[0,0,40,10]`, `[0,10,40,15]`, `[0,25,40,15]`다.
- 기존 single implicit auto-row (`A A A A A A`, `[10,30]px`) 경로는 유지했다. 두-item legacy text grammar는 바꾸지 않았고, 세-item shape만 정확히 `A` 다음 `B`로 제한했다.

## Rust 검증

실제 코드는 projection admission을 두/세 visible direct item에 한정하고 기존 track sizing을 재사용한다. 두 text contribution과 `StretchAutoTracks` 이전/이후 trace는 opt-in test feature에서 검사한다.

- `cargo test --locked -p spinon-runtime --lib root_html_grid_intrinsic_auto_rows -- --nocapture`: 22/22 통과.
- `cargo test --locked -p spinon-runtime --lib --features c11-4-grid-track-sizing-fixture-test root_html_grid_intrinsic_auto_rows -- --nocapture`: 22/22 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_intrinsic -- --nocapture`: 20/20 통과.
- `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-style-to-render -p spinon-runtime --all-targets -- -D warnings`: 통과.
- `cargo fmt --all -- --check`: 통과.
- `cargo build --locked --release -p spinon-runtime --lib`: 통과. `libspinon_runtime-e547016af4e3eeda.rlib` SHA-256 `02cb9bbb5e89df6e2229f6ac6e66a95e53c4f7592dc76ba7eb783b8433e26692`; 새 fixture title/path marker는 artifact에서 검색되지 않았다.

## Android API 37 ARM64 AVD

- 기기 `emulator-5554`, Android 17/API 37, ARM64, emulator ANGLE/Vulkan SwiftShader다. 실기기를 사용하지 않았다.
- 앱은 기본 `index.html` Root HTML route에서 `layout=ready`, `boxes=6`, `runtime_text_count=2`로 draw/present했다. 전체 근거는 [runtime log](./pass01/android/runtime.log), [build log](./pass01/android/build.log), [device properties](./pass01/android/device-properties.txt)다.
- APK의 `assets/index.html` SHA-256은 fixture와 같다. APK SHA-256 `db03340cee44deb3649964c305e1f1e628ad09b90ffd0258938d4165e23af26d`.
- [Simulator screenshot](./pass01/android/simulator.png), SHA-256 `93567da8554d8aa3fe6d2137725add50afb5cb190a962c6776bf033344c070b7`.

## iOS Simulator

- iPhone 17 Pro (`iPhone18,1`), iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`다.
- 앱은 기본 Root HTML route에서 `layout=ready`, `boxes=6`, `runtime_text_count=2`로 draw/present했다. 전체 근거는 [runtime log](./pass01/ios/runtime.log), [build log](./pass01/ios/build.log), [device list](./pass01/ios/device-list.txt)다.
- 앱 bundle의 `index.html` SHA-256은 Android 및 source fixture와 같다. 실행 파일 SHA-256 `c57e0fad6a7d0fdd087eb140b3ed00ba33a87d234aef6bf180c0a22e20f46970`.
- [Simulator screenshot](./pass01/ios/simulator.png), SHA-256 `8097ea231b87cdfc709c2fa7fecc18129ee74b19ffa7039e9ee88ad429994f03`.

두 화면은 Grid와 두 text item의 시각적 실행 확인이다. 정확한 frame·track 값은 RuntimeSession assertion과 opt-in trace에서 확인했다. 스크린샷은 surface pixel equality, iOS compositor parity, 물리 기기 또는 hardware GPU 증거가 아니다.

## 복원 및 경계

- 빌드 전 `examples/bootstrap/index.html`과 `root-app.js`, 설치된 Android APK, 설치된 iOS app executable hash를 보관했다. 테스트 종료 뒤 기존 앱을 재설치하고 두 앱 실행을 종료했다. 복원된 Android APK와 iOS executable hash가 백업 hash와 일치한다. [environment 및 복원 기록](./pass01/environment.txt).
- Bootstrap source byte hashes는 원본으로 돌아왔고 두 파일에 Git diff가 없다. 빌드 산출물도 실행 전의 APK·iOS app으로 복원했다.
- Agent Browser는 사용하지 않았다. 종료 확인 시 `agent-browser session list`는 `No active sessions`였고 Agent Browser 프로세스도 남아 있지 않았다.
- C11.4/C11 공식 완료 상태는 이 코드 PR에서 바꾸지 않는다. 문서 상태와 public roadmap 및 사용자가 정한 Tailnet preview는 구현 PR 병합 뒤 별도 문서 변경으로 맞춘다. 공식 카운트 `78/214`를 이 bounded slice로 올리지 않는다.

상세 adversarial review: [구현/runtime 20개](./implementation-review-20.md), [실제 PR diff 30개](./implementation-pr-diff-review-30.md). Source/fixture checksum 목록은 [source-inputs.sha256](./source-inputs.sha256)에 있다.
