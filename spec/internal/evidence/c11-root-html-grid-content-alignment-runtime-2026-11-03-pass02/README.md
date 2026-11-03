# C11 Root HTML Grid 콘텐츠 정렬 실행 근거

## 판정 범위

2026-11-03 checkout `e7fe2f9ab`에서 제한된 parsed Root HTML Grid `align-content`·`justify-content` 후보를 검증했다. 일반 CSS 지원, C11.1/C11 완료 또는 공식 완료 항목 증가는 판정하지 않는다. 공식 집계는 `72/213` 그대로다.

고정 fixture는 [`root-html-grid-content-alignment.html`](../../../../tests/fixtures/css/c11/root-html-grid-content-alignment.html)이며 SHA-256은 `95d57100a0f24d353911934922ba7832f6461cd39586e76b8bb92fc1a7f2e132`다. 독립 CSS Box Alignment 산식과 Chrome 156 Agent Browser 관찰은 [사전 비교 근거](../c11-root-html-grid-content-alignment-precomparison-2026-11-03/README.md)에 있다.

## Rust 검증

- `cargo test --locked -p spinon-layout --lib content_alignment_tests -- --nocapture`: 3/3 통과.
- `cargo test --locked -p spinon-style -p spinon-style-to-layout --lib c11_1_root_html_content_alignment -- --nocapture`: cascade 1/1, projection 4/4 통과.
- `cargo test --locked -p spinon-runtime c11_1_root_html_fixed_grid_content_alignment_reaches_default_runtime -- --nocapture`: 기본 Root HTML runtime 1/1 통과.
- `cargo fmt --all -- --check`: 통과.
- `cargo clippy --locked -p spinon-layout -p spinon-style -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`: 통과.
- 원본 명령 출력은 [`runtime-test.log`](runtime-test.log)에 둔다.

## 플랫폼 앱 실행

두 빌드 모두 `SPINON_ENABLE_C04_RUNTIME_GPU=1`이며 C11 전용 GPU fixture flag 없이 기본 앱 경로를 사용했다. 앱 fixture를 위해 임시 교체했던 `examples/bootstrap/index.html`과 `root-app.js`는 캡처 뒤 원래 SHA로 복구했다.

### Android

- Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`, serial `emulator-5556`; 실기기는 사용하지 않았다.
- API `37`, ABI `arm64-v8a`, display `1080×2400`, density `420 dpi`.
- 실행 로그 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass ... layout=ready boxes=7`을 기록했다. 화면에서 2×2 색상 item 배치를 확인했다.
- [Android 화면](android/android-api37-avd-content-alignment.png) · [앱 로그](android/app.log) · [빌드 로그](android/build.log).
- APK SHA-256: `ade24a2a5f2070d61291a28e6448ff56694e4ebe450da26b4bf363c4e560c141`.

초기 AVD 시작은 crash reporter의 숨은 동의 대화상자에서 멈췄다. emulator 공식 옵션 `-crash-report-mode disabled`를 사용한 재시작 뒤 ADB 등록, cold boot, 앱 실행이 성공했다. 재시작 진단 로그를 보존했으며 AVD 데이터나 crash database는 삭제하지 않았다.

### iOS

- iPhone 17 Pro / iOS 26.2 Simulator `AC268F2F-76B5-4B8E-90A1-1D35D9CD36ED`.
- 실제 앱 로그에서 `layout=ready`, `boxes=7`, Metal renderer와 draw 제출을 확인했다.
- [최종 iOS 화면](ios/iphone-17-pro-ios-26.2-content-alignment-final.png) · [최종 앱 로그](ios/app-final.log) · [빌드 로그](ios/build.log).
- Simulator executable SHA-256: `fe14ba4ce0080f8c6d8062e1c13914758946d939beb44eb19321f32403421d50`.

첫 iOS 실행의 cascade 오류 화면과 로그도 보존했다. 원인은 fixture의 `background` shorthand 확장이 지원되지 않은 computed property를 만든 점이었다. fixture를 `background-color` longhand로 바꾼 뒤 양 플랫폼 실행과 runtime test가 통과했다. iOS build는 성공했으나 기존 `RawWindowMetalLayer` duplicate-symbol linker warning이 남았다.

두 플랫폼 screenshot은 앱의 시각적 smoke evidence다. Android PixelCopy, iOS Metal surface readback, compositor 이후 픽셀 비교를 수행하지 않았다. Metal/하드웨어 GPU, 실기기, WPT 및 전체 CSS Grid 적합성의 근거로 확대하지 않는다.

## Agent Browser와 정리

Chrome 기준 실행은 Agent Browser `HeadlessChrome/156`에서 했다. 사용자 요청에 따라 terminal-browser를 사용하지 않았다. Agent Browser session을 닫았고 관련 browser daemon도 종료했다. Android 테스트 AVD와 이번에 만든 전용 iOS Simulator만 종료했으며 기존에 켜져 있던 다른 시뮬레이터는 건드리지 않았다.

## 실패·범위 기록

- 초기에 Android QEMU watchdog 종료가 있었으나 근본 원인은 headless crash reporter 동의 대기였다. 비활성 옵션 뒤 같은 API 37 AVD에서 앱이 실행됐다.
- 초기 iOS cascade 실패는 앱 renderer 문제가 아니라 fixture shorthand 지원 경계였다. 실패 screenshot/log는 보존했다.
- 이 근거는 후보 구현만 다룬다. C11 parent checkbox, 내부 계약 숫자 버전 `0.1.0`, 공식 완료 `72/213`을 바꾸지 않는다.
