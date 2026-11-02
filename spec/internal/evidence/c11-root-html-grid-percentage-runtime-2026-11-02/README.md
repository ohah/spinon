# C11.1 Root HTML Grid percentage-track runtime evidence

## 실행 대상

이 evidence는 PR [#1019](https://github.com/ohah/spinon/pull/1019)로 병합된 제한 구현의 pre-merge source snapshot을 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 기본 Root HTML app route로 실행한 결과다. 작업 시작 기준 checkout HEAD는 `43b0f018562be1dd04ef51582e36118ace5a4105`; 병합 전 candidate 소스의 개별 SHA-256은 [`candidate-source.sha256`](./candidate-source.sha256)에 기록했다. 양쪽 앱에 포장한 HTML SHA-256은 `957686766fd7c11441c4c4d84f755f4eb353b69867a73fd01489f8ffa5523153`이며 고정 입력은 [`root-html-grid-percentage-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-percentage-runtime.html)이다. PR은 2026-11-02T01:42:07Z Rebase and merge됐으며 merge SHA는 `8e549ed55ac6f7e9611a4f7e1e571fca3a1e5684`다.

페이지에는 두 Grid가 있다. 첫 Grid는 definite `240×120px` root에서 columns `[60,120,20]px`, rows `[30,60,10]px`와 fixed-px gaps를 확인한다. 두 번째 Grid는 `200px` root에 120%/10% columns와 8px gap을 두며 두 번째 item의 marker가 x=248px에서 root edge를 넘어 보인다. 이 simulator input은 positive cases만 담는다. 나머지 positive/rejection matrix는 projection 및 runtime Rust tests에서 검증한다. [`root-app-runtime-probe.js`](./root-app-runtime-probe.js)는 parsed document에서 두 main Grid와 item 구조를 확인하며 fixture-specific runtime/FFI entry point를 쓰지 않는다.

## Rust 검증

- `cargo fmt --all -- --check` 통과.
- `cargo test -p spinon-style-to-layout root_html_profile -- --nocapture`: 4/4 통과.
- `cargo test -p spinon-layout percentage_tracks_use_the_definite_grid_content_axes -- --nocapture`: 1/1 통과.
- `cargo test -p spinon-runtime root_html_grid -- --nocapture`: 16/16 통과.
- `cargo clippy --locked -p spinon-style-to-layout -p spinon-layout -p spinon-runtime --all-targets -- -D warnings` 통과.

별도 실패 경로 검토는 [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)에 기록한다.

구현 PR diff의 별도 30개 적대 관점은 [pr-delta-review-30.md](./pr-delta-review-30.md)에 기록한다.

## Android API 37 ARM64 AVD

AVD serial은 `emulator-5556`이고 앱은 `dev.spinon.bootstrap/.MainActivity`의 기본 `index.html` route를 사용했다. Build log의 feature flags에서 `c04_runtime_gpu=1`, fixture-test flags는 0이다. APK의 `assets/index.html`은 source fixture와 byte-identical이다. 앱 로그는 `source=index.html`, `layout=ready boxes=15`, root frame `411.42856×160 CSS px`, `SPINON_C0410_DRAW status=0 presented boxes=15`를 기록한다. Renderer는 ANGLE/Vulkan/SwiftShader 소프트웨어 경로다. [Android screenshot](./android-api37-avd-percentage-overflow.png)에서 보라색 marker가 dark root edge 오른쪽에 보인다.

## iPhone 17 Pro / iOS 26.2 Simulator

Simulator UDID는 `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`다. 앱은 같은 `index.html` fixture를 기본 route로 읽었다. 로그는 `layout=ready boxes=15`, root frame `402×160 CSS px`, `SPINON_C0410_DRAW generation=1 status=0 presented boxes=15`를 기록한다. [iOS screenshot](./ios-26_2-simulator-percentage-overflow.png)에서 같은 overflow marker를 확인했다.

두 앱 프로세스는 캡처 뒤 종료했다. Simulator screenshot은 화면 배치 확인용이며 PixelCopy·Metal drawable 표면 표본이나 픽셀 동등성 근거가 아니다. 이 실행은 physical device, hardware GPU, iOS compositor 이후 pixels, WPT 적합성 또는 전체 CSS Grid 지원을 주장하지 않는다. 병합된 limited slice는 C11.1/C11 완료로 승격되지 않았으며 공식 완료는 `72/213` 그대로다. PR check rollup은 비어 있었고 필수 status check가 설정되지 않아 CI 통과로 세지 않는다.

[`SHA256SUMS.txt`](./SHA256SUMS.txt)는 저장소에 보존한 capture script, source probe, build logs, packaged-asset manifests, runtime logs, screenshots 및 README의 SHA-256을 기록한다. 로그 끝의 공백과 마지막 빈 줄만 capture script에서 정규화했다. APK와 iOS executable은 `build/spinon/artifacts/` 아래 로컬 build outputs이며 evidence manifest에는 포함하지 않는다.
