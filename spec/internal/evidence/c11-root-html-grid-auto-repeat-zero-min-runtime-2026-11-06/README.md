# C11.1 parsed Root HTML Grid auto-repeat · zero minimum 실행 근거

**상태:** 구현 후보 검증 완료, PR 전. C11.1/C11 완료 체크와 공식 집계 `78/214`는 유지한다. 공개 CSS 지원으로 선언하지 않는다.

## 동작 범위

기존 parsed Root HTML `RootHtmlAutoRepeatV1` 제한에서 finite non-negative fixed-px minimum을 받는다. auto-fill/auto-fit, columns/rows, 1–2개 `minmax(px, 1fr)` 반복 track, definite positive axis size와 finite non-negative px gap만 포함한다. `max(minimum, 1px)`는 반복 횟수를 셀 때만 사용하며 실제 minimum은 0px이다. count는 `max(1, floor((A + G) / (sum(max(min_i, 1px)) + P × G)))`로 계산한다.

일반 authored CSS admission, arbitrary repeat pattern, implicit tracks, named lines/areas, nested Grid, WPT 전체, 새 공개 API/FFI, C11.1/C11 완료는 범위 밖이다.

## Fixture·Chrome oracle

고정 Root HTML fixture는 [source](../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-zero-min-runtime.html)이며 SHA-256은 `a10c8192343bde89e8bcb0f59d07d52a1a45e385888b76cf9d717a6aada418c4`다. Agent Browser Chrome `HeadlessChrome/157.0.0.0`에서 10 case를 실행했다. 단일·혼합 pattern, fractional gap, auto-fill/auto-fit, empty fit, rows를 포함해 독립 count 식과 resolved track 개수 10/10이 일치했다. 첫 item frame과 실제 track 크기도 각 case에서 저장했다: [관찰 JSON](./chrome157-runtime-observation.json). 전체 independent expected와 계획용 사전 비교는 [Chrome 157 precomparison](../c11-root-html-grid-auto-repeat-zero-min-precomparison-2026-11-06/README.md)에 있다.

## Rust 검증

| 범위 | 명령의 filter | 결과 |
|---|---|---:|
| Stylo cascade auto-repeat | `cargo test --locked -p spinon-style --lib auto_repeat` | 46 passed |
| projection count/grammar | `cargo test --locked -p spinon-style-to-layout --lib root_html_auto_repeat` | 11 passed |
| layout auto-repeat·quota regressions | `cargo test --locked -p spinon-layout --lib auto_repeat` | 74 passed |
| parsed Root HTML runtime | `cargo test --locked -p spinon-runtime --lib root_html_grid_auto_repeat` | 6 passed |
| C11.4 style-to-layout area/track-sizing regression | `cargo test --locked -p spinon-style-to-layout --lib c11_4_track_sizing_areas_tests` | 22 passed |
| C11.4 intrinsic-row runtime regression | `cargo test --locked -p spinon-runtime --lib root_html_grid_intrinsic_auto_rows` | 15 passed |

Workspace Clippy `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, Android/iOS shell `bash -n`, Xcode project `plutil -lint`, `git diff --check`도 통과했다. 원본 로그는 [검증 파일 목록](#검증-로그)에 둔다. 한 package 전체 테스트 suite를 통과했다고 확대 해석하지 않는다.

## Android API 37 ARM64 AVD

- 기본 artifact는 C04 runtime GPU on, C11.1 fixture flag off로 빌드했다. `BUILD SUCCESSFUL`; bootstrap generated copy와 APK의 `assets/root-html-grid-auto-repeat-zero-min-runtime.html`가 모두 없다. APK SHA-256: `40565688f78686888f652f1b7473c3eeed8b3bf2083a927222a1eab958730de3`. 새 설치 route log는 fixture-test disabled marker를 남긴다: [기본 route](./android-default-final-route.log).
- Opt-in artifact는 fixture flag on으로 빌드했다. APK SHA-256: `078171cde7757b419fcd09885a2e4dcee9ec394b23cad94a9f7cc551cef74a4f`. APK의 HTML asset은 test source와 동일한 SHA-256 `a10c8192…418c4`다.
- Android Emulator API 37 / Android 17 / `arm64-v8a`에서 fixture source hash, session ready, `layout=ready boxes=21`, status 0 draw/present를 확인했다. Renderer는 ANGLE/SwiftShader software path다: [최종 log](./android-avd-final-runtime.log), [화면](./android-avd-final.png, 1080×2400).

## iOS Simulator

- iPhone 17 Pro / iOS 26.2 Simulator에서 C04 runtime GPU on, fixture flag off 빌드를 설치하고, 앱을 제거 후 다시 설치해 fixture 비포함 route를 확인했다: [기본 route](./ios-default-route-clean-install.log). default build 시 app bundle과 `platforms/ios/Resources` 양쪽에서 fixture file이 없었다.
- fixture flag on build의 app bundle HTML SHA-256은 test source와 같은 `a10c8192…418c4`다. Simulator runtime은 source hash, session ready, `layout=ready boxes=21`, status 0 Metal draw/present를 기록했다: [최종 log](./ios-simulator-after-fix-runtime.log), [화면](./ios-simulator-after-fix.png, 1206×2622).
- Xcode build log에는 V8 static library object의 duplicate timestamp debug-map 경고가 있다. 두 빌드는 `BUILD SUCCEEDED`로 끝났다.

양쪽 build script는 fixture flag가 `0|1`이 아니거나 fixture flag `1`에 C04 runtime GPU `0`을 조합하면 exit 2로 거부한다: [Android](./android-invalid-flag.log), [Android prerequisite](./android-c04-prerequisite.log), [iOS](./ios-invalid-flag.log), [iOS prerequisite](./ios-c04-prerequisite.log).

### Artifact SHA-256

| Artifact | SHA-256 |
|---|---|
| Runtime HTML source / Android APK asset / iOS bundle resource | `a10c8192343bde89e8bcb0f59d07d52a1a45e385888b76cf9d717a6aada418c4` |
| Android default APK | `40565688f78686888f652f1b7473c3eeed8b3bf2083a927222a1eab958730de3` |
| Android opt-in APK | `078171cde7757b419fcd09885a2e4dcee9ec394b23cad94a9f7cc551cef74a4f` |
| iOS opt-in Simulator executable | `bbe767c67ba39398ce5792f352520e30b3e23ce9c9b0953c4822914f71786e5a` |
| Chrome runtime observation JSON | `11191e7b8f2fca4f779d1937d909280b335c77177930f6c39a8c0edfdee8b5b4` |
| Android AVD screenshot | `5803bfe7d3cd79bb85659a6c5a599bf5630274b861ce39629d1e4c5d2bc778e4` |
| iOS Simulator screenshot | `06ad1f2fea24f696446a951eb00e27560c6eef39d210d3811788e71880f18573` |

Source fixture, browser observation, simulator screenshots, and four route logs are pinned in [SHA256SUMS.txt](./SHA256SUMS.txt); all manifest entries passed `shasum -a 256 -c`.

두 simulator screenshot은 OS 화면 캡처이며 PixelCopy 또는 Metal drawable readback이 아니다. 따라서 실제 track geometry는 Rust/Chrome 검증으로 보고하고, 앱 화면은 fixture source와 parser/layout/draw 경로를 확인하는 visual smoke로만 취급한다. 실기기·Android hardware GPU·iOS compositor 이후 pixel parity는 검증하지 않았다.

## 구현 실패 관점 검토

서로 다른 실패 관점 20개는 [구현/runtime 검토](./implementation-runtime-review-20.md)에 기록했다. 검토 중 찾은 stale Android fixture cleanup과 Gradle import 오류는 수정하고 빌드를 다시 실행했다. PR의 실제 diff에는 별도 30개 적대 검토를 추가한다.

## 검증 로그

- [Stylo auto-repeat 46](./style-root-html-auto-repeat-regression.log)
- [projection auto-repeat 11](./projection-root-html-auto-repeat-final.log)
- [layout auto-repeat 74](./layout-auto-repeat-final.log)
- [runtime auto-repeat 6](./runtime-root-html-auto-repeat-final.log)
- [C11.4 style/track sizing 22](./style-c11-4-track-sizing-regression.log)
- [C11.4 intrinsic-row runtime 15](./runtime-root-html-intrinsic-auto-rows-regression.log)
- [workspace Clippy](./clippy-workspace-final.log) · [rustfmt](./rustfmt-final.log)
- [Android default build](./android-default-final-build.log) · [Android opt-in build](./android-fixture-final-build.log)
- [iOS default build](./ios-default-after-fix-build.log) · [iOS opt-in build](./ios-fixture-after-fix-build.log)

Xcode/test log files retain their output; only trailing whitespace and redundant final blank lines were removed to satisfy `git diff --check`.
