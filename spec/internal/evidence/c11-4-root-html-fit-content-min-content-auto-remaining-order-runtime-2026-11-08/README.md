# C11.4 Root HTML `fit-content()` / `min-content` / `auto` implicit-row order runtime

이 근거는 `grid-auto-rows`에서 기존 두 순서에 더해 나머지 네 순서를 기본 parsed Root HTML route에 연결한 제한 구현 후보를 기록한다. 일반 CSS admission이나 C11.4/C11 전체 지원 완료를 뜻하지 않는다.

## 비교 기준과 독립 기대값

구현 전 기준은 [계획](../../../../plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.md)과 [Chrome 157 사전 비교](../c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08/README.md)다. 네 순서 각각에서 `fit-content(20px)`와 `fit-content(20%)`를 비교해 총 8 scene의 Grid used rows, item frames, payload frames를 고정했다. definite `40×100px` Grid의 percentage cap은 20px이다. 독립 산술은 track base size에서 남은 free space를 `auto` track에 배분하며 Chrome 관측값을 기대값으로 재사용하지 않는다.

Simulator와 일치하는 고정 Root HTML fixture는 [`root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html), SHA-256 `2f108e91fb3f7485df0aded8e4eb7942f7718063f63437eb05ec064532d9471e`, 2,180 bytes다. Chrome 157 Agent Browser 관측 JSON은 [`chrome-runtime-oracle.json`](chrome-runtime-oracle.json)에 보존했다. `auto fit-content(20%) min-content` 장면의 used rows는 `[10,44,20,10,16]px`, Grid item frames는 `[[0,0,40,10],[0,10,40,44],[0,54,40,20],[0,74,40,10],[0,84,40,16]]`, nested payload frames는 `[[0,10,40,34],[0,54,40,34],[0,74,40,10],[0,84,40,6]]`이다. 이 값은 계획의 독립 산술과 일치한다.

## 구현과 Rust 검증

Style projection, Root HTML layout admission, sparse auto-placement track validation에 기존 exact 두 순서와 신규 네 순서를 함께 허용했다. 나머지 arbitrary track list와 invalid cap은 거부한다. layout·projection·RuntimeSession 테스트는 typed order와 frame, px/% cap, invalid revision의 atomic rejection, 동일 세션 recovery를 검사한다.

실행한 검증:

- `cargo fmt --all -- --check` — 통과
- `cargo test --locked -p spinon-layout grid_root_html_fit_content_auto_rows_tests` — 8 passed
- `cargo test --locked -p spinon-style-to-layout root_html_fit_content` — 5 passed
- `cargo test --locked -p spinon-runtime root_html_grid_fit_content_auto_rows` — 9 passed
- `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings` — 통과

별도 [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md)와 [PR 변경 검토 30개](pr-diff-review-30.md)를 수행했다.

## Android 및 iOS Simulator

두 앱은 같은 2,180-byte HTML hash를 Bootstrap 앱의 기본 `index.html` route에서 읽었다. fixture 전용 Cargo/FFI flag는 켜지 않았다. Bootstrap의 원본 `index.html`과 `root-app.js`를 기록된 hash로 복구했다.

| 플랫폼 | 장치 | 확인값 | 화면 |
| --- | --- | --- | --- |
| Android | `emulator-5554`, API 37, `arm64-v8a`, 1080×2400 AVD | 앱 프로세스 PID 16144, `layout=ready boxes=12`, `SPINON_C0410_DRAW status=0 presented boxes=12`; present 로그를 본 뒤 캡처 | [최종 screenshot](android/final/screenshot.png) |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator, UDID `97E99AA8-D144-4DA8-B1BF-7F6B34007F8B` | 앱의 현재 프로세스 로그에서 `layout=ready boxes=12`, environment ready를 확인 | [screenshot](ios/final/screenshot.png) |

이 PNG들은 OS 화면 캡처를 통한 시각 확인이다. Android PixelCopy, iOS Metal drawable readback, compositor 이후 pixel parity를 측정하지 않았다. Android 첫 캡처는 draw/present 전이라 빈 화면이었고 [`screenshot-before-draw.png`](android/final/screenshot-before-draw.png)로 보존했다. 캡처 스크립트를 현재 앱 PID의 `presented boxes=12`를 확인하도록 보완한 뒤 최종 화면을 다시 캡처했다. 두 이미지의 hash와 전체 build artifact hash는 [`final-capture.sha256`](final-capture.sha256)에 기록했다.

화면을 기록한 뒤 auto-placement validation의 중복 exact-pattern match를 기존 Root HTML matcher 호출로 바꾸는 소스 중복 제거를 했다. track sizing/placement 동작은 바뀌지 않았고 변경된 최종 source로 focused Rust tests, scoped Clippy, rustfmt를 다시 통과했다. Simulator 앱은 재설치하지 않았다. 직전 iOS `simctl install`에서 app-data container path가 예상 밖으로 교체된 사실을 확인했기 때문이다. 따라서 이 화면들은 중복 제거 전 동등한 allowlist 구현의 runtime smoke이며, 최종 helper refactor가 들어간 재빌드 바이너리의 runtime 캡처는 아니다.

## 실행 보존 파일과 한계

- [`build-simulators.sh`](build-simulators.sh), [`install-capture-simulators.sh`](install-capture-simulators.sh), [`capture-android-current-process.sh`](capture-android-current-process.sh)는 빌드·설치·캡처 경로를 기록한다.
- Android/iOS build log, device metadata, app PID별 runtime log, installed fixture hash와 screenshot은 각각 [`android/`](android/) 및 [`ios/`](ios/)에 있다.
- iOS `simctl install` 전후 `get_app_container` 경로가 `AF0E8FA7-70D0-44B0-9786-6844987417ED`에서 `A710A909-02B3-47AD-A591-D487A4A75BFE`로 바뀌었다. 전 경로는 이후 존재하지 않고 후 경로는 존재한다. 스크립트는 `erase`, `uninstall`, app-data `clear`를 실행하지 않았다. 경로 교체 원인은 확인하지 못했으므로 이전 앱 data 보존 결과를 주장하지 않는다.
- Android emulator는 SwiftShader 기반 가상 GPU다. 물리 Android 기기, hardware GPU, WPT 적합성, 임의 DOM/CSS 및 모든 C11.4 동작은 검증하지 않았다.

공식 `spec/STATUS.md`의 C11.4/C11 완료 체크는 이 slice로 바꾸지 않는다. 내부 계약 숫자 버전 `0.1.0`도 유지한다.
