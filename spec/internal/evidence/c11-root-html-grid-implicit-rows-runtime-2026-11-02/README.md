# C11.1 Root HTML Grid fixed implicit rows runtime 후보

## 범위

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 입력에서 no-area Grid의 단일 finite non-negative px `grid-auto-rows`를 제한 허용한다. `grid-auto-flow: row`, `grid-auto-columns: auto`, 이름 없는 fixed-px explicit rows/columns, fixed-px gap, auto placement와 order 0, 빈 fixed-size block item만 처리한다. fixture-only FFI나 scene-ID 분기는 추가하지 않았다.

CSS 기준 geometry는 [Chrome 156 사전 비교와 독립 기대값](../c11-root-html-grid-implicit-rows-precomparison-2026-11-02/README.md) 및 [`chrome156-expected.json`](../c11-root-html-grid-implicit-rows-precomparison-2026-11-02/chrome156-expected.json)에 고정했다. Rust integration test는 같은 기본 Root HTML entry에서 다음 frame을 exact 비교한다.

- 한 implicit row: root `100×40`, rows `[20,15]`, 세 번째 item `(0,25,40,15)`.
- 0px implicit row: root `100×25`, rows `[20,0]`, 세 번째 item `(0,25,40,0)`.
- 두 implicit rows: root `100×60`, rows `[20,15,15]`, 다섯째·여섯째 item `(0,45,40,15)` 및 `(45,45,55,15)`.
- hidden child 생략, 기본/명시 `row` 동일 배치, class/id 변경 무관성, DOM 순서 변경 반영.
- unsupported flow/sizing/tracks/areas/placement/order/alignment/overflow/text/nesting은 실패하며 partial scene을 발행하지 않는다. 10,001개 implicit row 필요 입력은 확장 track 할당 전에 거부한다.

## Rust 검증

[`validation-pass04.log`](./validation-pass04.log)에서 다음 최종 소스 검증을 확인했다.

- `cargo fmt --all -- --check`, `git diff --check`: 통과.
- `cargo test --locked -p spinon-layout grid_ -- --nocapture`: 74/74, 자원 상한 test 포함.
- `cargo test --locked -p spinon-style-to-layout root_html_grid -- --nocapture`: 10/10.
- `cargo test --locked -p spinon-style-to-layout c11_3_3_grid_template_areas_tests -- --nocapture`: 26/26; stale document revision guard: 1/1.
- `cargo test --locked -p spinon-runtime root_html_ -- --nocapture`: 34/34.
- 세 변경 crate all-target Clippy `-D warnings`: 통과.

## Android 및 iOS Simulator

두 앱은 기본 `index.html` route에서 동일한 [HTML source](./root-html-grid-implicit-rows-runtime.html)와 [V8 DOM probe](./root-app-runtime-probe.js)를 실행했다. Android/iOS bootstrap 예제와 기존 생성 bootstrap 파일은 백업 후 원본 hash로 복구했다([복구 hash](./restored-bootstrap-sha256.txt)). fixture-test flags는 모두 꺼져 있었다.

| 대상 | 실행 결과 | 화면 및 원본 로그 |
| --- | --- | --- |
| Android API 37 ARM64 `spinon_api37_0_applied_txn` AVD, ANGLE/Vulkan/SwiftShader | `layout=ready`, viewport `411.42856×841.52374`, root `411.42856×40`, 6 box draw/present. 설치 APK와 빌드 APK SHA-256이 같다. | [화면](./android-api37-avd.png) · [앱 marker 필터 로그](./android-api37-avd-runtime-summary.log) · [빌드](./android-build.log) |
| iPhone 17 Pro, iOS 26.2 Simulator, Metal Simulator GPU | `layout=ready`, viewport `402×778`, root `402×40`, 6 box draw/present. 설치 executable과 빌드 executable SHA-256이 같다. | [화면](./ios-26_2-simulator.png) · [실행 로그](./ios-simulator-runtime.log) · [요약](./ios-simulator-runtime-summary.log) · [빌드 요약](./ios-simulator-build-summary.log) |

실행 직전 시뮬레이터 log를 보존한 뒤 Android logcat만 초기화해 현재 앱 실행을 분리했다. Android launch는 `adb shell am start -n dev.spinon.bootstrap/.MainActivity`, iOS launch는 `xcrun simctl launch <UDID> dev.spinon.bootstrap`이다. 두 앱 프로세스는 캡처 후 종료했다. device, viewport, build feature flags와 artifact hash는 [`platform-runtime-observation.json`](./platform-runtime-observation.json)에 있다. 빌드 로그에는 이번 변경과 무관한 기존 iOS archive duplicate-symbol/dSYM 경고가 있으며 build는 성공했다.

screenshots는 ADB display capture와 `simctl io screenshot`의 시각 자료다. GPU 표면 readback이나 compositor 이후 pixel equality를 주장하지 않는다. exact track과 item frame은 Rust integration test로 확인했고, simulator log는 기본 HTML 경로의 layout-ready 및 draw/present를 확인한다. 이 검증은 실기기, hardware GPU, WPT 또는 일반 CSS Grid 지원 검증이 아니다.

## 상태

구현 후보이며 [구현/runtime 실패 관점 검토 20개](./implementation-runtime-review-20.md)는 통과했다. PR diff 검토와 PR 병합은 남아 있다. Root HTML의 제한된 fixed-px implicit-row 입력 하나를 연결한 것으로 C11.1, C11.3.3, C11.4, C11 전체 완료를 뜻하지 않는다. 공식 완료 `72/213`을 유지한다.
