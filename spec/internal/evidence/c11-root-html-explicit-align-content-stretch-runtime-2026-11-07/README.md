# C11.4 parsed Root HTML explicit `align-content:stretch` runtime 근거

## 변경 범위

[병합된 C11.4 두 implicit auto 행 경로](../../../../plan/c11-root-html-multiple-auto-row-stretch.md)에 author가 `align-content:stretch`를 명시한 동일 shape를 추가한다. 이 slice는 기존 `RootHtmlAutoRowStretchV1` 후보의 computed value gate와 그 경로 전용 layout validation만 넓힌다. 공개 CSS parser·runtime selector·layout profile ID·FFI·API는 추가하지 않는다. Legacy `RootHtmlIntrinsicTrackSizingV1`에서는 `grid-auto-rows:auto`와 `align-content:stretch` 조합을 계속 거부한다.

- 사전 비교/독립 계산: [HeadlessChrome 157 결과와 expected.json](../c11-root-html-explicit-align-content-stretch-precomparison-2026-11-07/README.md)
- 규범 근거: [CSS Grid Layout Module Level 1 §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). `normal`과 `stretch`는 남은 양의 definite free space를 `auto` max track 사이에 분배한다.
- Fixture: [root-html-multiple-auto-row-align-content-stretch-precomparison.html](../../../../tests/fixtures/css/c11/root-html-multiple-auto-row-align-content-stretch-precomparison.html), SHA-256 `44a3423cd3be20935b2436d7f9f3701e580e6d39205b445b5b2a8158ca3e7308`.
- 고정 shape는 content box 40×40px, explicit column 40px, explicit row 10px, 두 direct text `A`/`B`가 만든 implicit `auto` rows 두 개다. 독립 계산과 Chrome 관측은 used rows `[10,15,15]px`, item frames `[0,0,40,10]`, `[0,10,40,15]`, `[0,25,40,15]`다. 기존 `normal` control은 같은 geometry를 내고 `start` control은 rows `[10,10,10]px`를 유지한다.

## Rust 검증

모든 focused tests는 통과했다.

- `cargo test --locked -p spinon-layout root_html_two_implicit_auto_rows_stretch_equally_for_normal_and_stretch`: 1/1. `normal`과 `stretch`의 같은 tracks·frames.
- `cargo test --locked -p spinon-layout legacy_root_html_intrinsic_track_profile_still_rejects_stretch_content_alignment`: 1/1. 기존 intrinsic profile의 rejection 보존.
- `cargo test --locked -p spinon-style-to-layout root_html_auto_row_stretch_candidate_requires_the_fixed_single_track_grid`: 1/1.
- `cargo test --locked -p spinon-runtime parsed_root_html_explicit_align_content_stretch_matches_normal -- --nocapture`: 1/1. 기본 `RuntimeSession` projection, NodeId/source order, geometry, text draw와 normal 대조.
- `cargo test --locked -p spinon-runtime parsed_root_html_multiple_auto_row_stretch -- --nocapture`: 2/2. 여섯 다른 computed alignment/변형 shape의 atomic rejection 및 같은 session recovery.
- `cargo test --locked -p spinon-runtime parsed_root_html_stretches_two_implicit_auto_rows_equally -- --nocapture`: 1/1. 직전 merged `normal` 경로 회귀.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_explicit_align_content_stretch_matches_normal -- --nocapture`: 1/1. 두 contribution 및 `StretchAutoTracks` 전후 trace를 opt-in으로 대조.
- `cargo build --locked --release -p spinon-runtime --lib`: 통과. `target/release/deps/libspinon_runtime-e547016af4e3eeda.rlib` SHA-256 `a1ab45d11262bab8c18b5335376d8d2d6fc229159fe95ad106c2681f107d7256`; binary strings에서 새 fixture title marker는 부재했다. fixture-enabled test executable은 positive control로 같은 marker를 포함한다.
- `cargo fmt --all -- --check`: 통과.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 수정 후 통과. 첫 실행에서 두 신규 boolean allowlist에 `nonminimal_bool`가 발견되어 허용 조건을 묶고 재검증했다.

## Android API 37 ARM64 AVD

- `emulator-5554`, `sdk_gphone64_arm64`, Android 17/API 37, `arm64-v8a`에서 실행했다. 실기기는 쓰지 않았다.
- Android 기본 Root HTML route는 `layout=ready`, 6 boxes, `runtime_text_count=2`, WGPU draw/present를 기록했다. [runtime log](./pass01/android/runtime.log)와 [build log](./pass01/android/build.log).
- APK의 fixture SHA-256은 `44a3423cd3be20935b2436d7f9f3701e580e6d39205b445b5b2a8158ca3e7308`; APK SHA-256은 `3dda465a4987da3e7fcf2949ed66281b9283fbbc98c30d73baf954f04e98f22c`다.
- [AVD screenshot](./pass01/android/simulator.png), 1080×2400, SHA-256 `421c4ae8918adc11e3aa10dbb0e03756d1fcf4a5eee5fb2a95f00439c3f2d3b3`.

## iOS Simulator

- iPhone 17 Pro, iOS 26.2, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`에서 실행했다.
- 기본 Root HTML route는 `layout=ready`, 6 boxes, `runtime_text_count=2`, Metal drawable 제출을 기록했다. [runtime log](./pass01/ios/runtime.log)와 [build log](./pass01/ios/build.log).
- App fixture SHA-256은 Android/source와 같은 `44a3423cd3be20935b2436d7f9f3701e580e6d39205b445b5b2a8158ca3e7308`; executable SHA-256은 `7016426f08bb26036e2e457966cdcbfa970089afbe9ff1f2057b05979150ef37`다.
- [iOS Simulator screenshot](./pass01/ios/simulator.png), 1206×2622, SHA-256 `10623713f71a951a897860c2c69348ecdecce6d59f3eb531b564c734fe23892c`.
- iOS app linking은 기존 `RawWindowMetalLayer` duplicate-symbol 경고를 냈으나 빌드·실행은 완료됐다. 이번 diff에서 해당 링크 경계는 변경하지 않았다.

두 build log는 원본 출력에서 줄 끝 공백과 마지막 빈 줄만 제거해 저장했다. 이 편집은 빌드 값/상태를 바꾸지 않는다.

두 화면은 실제 앱 route에서 Grid와 두 text item이 제출된 시각적 smoke evidence다. 정확한 track·frame은 Rust assertions와 opt-in trace가 판정하며 screenshot만으로 pixel parity를 주장하지 않는다. Android는 emulator의 ANGLE/SwiftShader 경로다. iOS evidence는 compositor 이후 surface readback이 아니다. Physical device, hardware GPU, WPT, arbitrary CSS admission과 전체 C11.4는 이 slice에서 검증하지 않았다.

## 복원

[실행 기록](./pass01/environment.txt)의 `asset_and_simulator_restoration=PASS`에서 bootstrap source, 기존 Android/iOS 설치 앱과 기존 빌드 artifact hash를 실행 전 값과 대조했다. 기존 앱을 재설치했으며 대상 앱 프로세스는 종료했다. 실제 입력 화면은 [Android](./pass01/android/simulator.png)와 [iOS](./pass01/ios/simulator.png)다.

C11.4와 C11 상위 항목은 완료 처리하지 않으며 공식 `78/214` 및 내부 계약 `0.1.0`은 유지한다.

추가 검토 기록: [구현/runtime 실패 경로 20개](./implementation-runtime-review-20.md), [구현 PR diff 실패 경로 30개](./pr-diff-review-30.md).
