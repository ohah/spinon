# C11.1 parsed Root HTML intrinsic track + template-area 실행 근거

## 범위와 판정

기존 기본 `RuntimeRootHtmlTextPaintV1` intrinsic-track 경로에서 `grid-template-areas`와 intrinsic explicit track을 함께 실행하는 제한 slice다. 일반 CSS Grid admission, 새 runtime profile/fixture selector, WPT 적합성 또는 C11 완료를 뜻하지 않는다. 공식 전체 집계 `78/214`, C11·C11.3·C11.3.3·C11.4 체크는 유지한다.

Chrome 156 Agent Browser 사전 비교는 네 장면(min-content columns, intrinsic `minmax()`, intrinsic column span, min/max-content rows)을 관찰했다. 텍스트 폭은 browser/font 관측값이고 area rectangle과 span 구조는 독립 기대값으로 확인한다. 런타임 geometry 허용 오차는 0.5 CSS px다. [사전 비교와 해시](../c11-root-html-intrinsic-areas-precomparison-2026-11-05/README.md)

고정 Rust fixture SHA-256은 `4e6eb57d5eabd58764b03682cc824762db693f5e42c7ba87372143954dbb7fd1`이다. 앱의 기본 `index.html`은 `root-app.js` bootstrap DOM 계약이 있으므로, 시뮬레이터 전용 입력에는 같은 grid 장면과 앱 marker를 함께 둔 [하네스 fixture](fixture-app-harness.html)를 썼다. 원래 bootstrap은 작업 뒤 SHA-256 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`로 복원했다.

## Rust 검증

| 명령 | 결과 | 로그 |
|---|---:|---|
| `cargo test --locked -p spinon-style-to-layout intrinsic_named_area_template -- --nocapture` | 5 passed | [로그](style-area-tests.log) |
| `cargo test --locked -p spinon-runtime root_html_grid_template_areas -- --nocapture` | 15 passed | [로그](runtime-area-tests.log) |
| `cargo test --locked -p spinon-layout root_html_intrinsic -- --nocapture` | 8 passed | [로그](layout-intrinsic-tests.log) |
| `cargo test --locked -p spinon-layout template_area -- --nocapture` | 14 passed | [로그](layout-area-tests.log) |
| `cargo fmt --all -- --check` | 통과 | [로그](rustfmt.log) |
| `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings` | 통과 | [로그](clippy.log) |
| `cargo check --locked -p spinon-ffi --features c04-runtime-gpu` | 통과 | [로그](default-ffi-check.log) |
| `mise exec -- bun test tools/css-reference/fixture-module-boundary.test.mjs` | 33 passed | [로그](fixture-module-boundary.log) |

새 negative는 unsupported `minmax(auto, 40px)` named-area track이 layout을 원자적으로 실패시키고 partial scene을 내지 않는지 확인한다. area 이름 해석·직사각형·dimension, span, intrinsic rows/columns, `minmax()`, cascade fallback과 stale scene도 focused matrix에서 확인했다.

넓은 기존 Root HTML 회귀 전체는 green이 아니다. `spinon-style-to-layout root_html`은 140개 중 131 pass·9 fail이고 [전체 로그](style-to-layout-root-html.log)를 보존했다. 실패는 C11.5 fixed-track alignment 계약 6개, mixed/partial placement 거부 기대 1개, percentage-gap math 조합 2개다. `spinon-runtime root_html_grid`는 95개 중 87 pass·8 fail이며 [전체 로그](runtime-root-html-grid.log)를 보존했다. 실패는 C11.5 fixed-track auto-size/stretch 계약 3개, unsupported automatic-minimum 거부 기대 1개, partial placement 거부 기대 1개, dense-flow 오류 경계 2개, integer-repeat 거부 기대 1개다. 이 broad failure set은 이 slice의 focused 통과 근거에서 제외하며, main 기준 checkout으로 baseline 재실행은 하지 않았다.

## Android API 37 ARM64 AVD

- 기기: `sdk_gphone64_arm64`, `emulator-5554`, API 37; 실기기가 아니다.
- `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 bun run build:android` 성공: [build log](android/build.log).
- APK `assets/index.html`과 앱 하네스 fixture SHA-256은 `cc4c3b4fb7292d2e3765605cf49b44ff5d3167e5155ca3184d4f6c4c1c874d28`로 일치한다. APK hash와 screenshot hash는 [manifest](android/hashes.sha256)에 있다.
- 기본 Root HTML route가 `source=index.html`, `layout=ready boxes=9`, `root_app_marker=ready`, `SPINON_C0410_DRAW status=0 presented boxes=9`를 기록했다: [설치](android/install.log) · [실행](android/launch.log) · [logcat](android/logcat.txt).
- [시뮬레이터 화면](android/simulator-screen.png)은 1080×2400 OS screenshot이다. PixelCopy는 수행하지 않았다.

첫 시도는 app bootstrap 표식이 없는 순수 grid fixture를 넣어 `root-app.js` 검사가 실패했고 화면이 검게 남았다. [실패 화면](android/initial-black-screen.png), [초기 logcat](android/initial-run-logcat.txt), [초기 Android build](android/initial-build.log), [초기 iOS build](ios/initial-build.log)을 보존했다. 하네스 표식을 추가한 입력으로 재빌드한 최종 결과만 위 통과 판정에 썼다.

## iPhone 17 Pro · iOS 26.2 Simulator

- UDID: `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`; 실기기가 아니다.
- `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 bun run build:ios-sim` 성공: [build log](ios/build.log).
- 앱 `index.html`과 app harness fixture SHA-256은 `cc4c3b4fb7292d2e3765605cf49b44ff5d3167e5155ca3184d4f6c4c1c874d28`로 일치한다. executable hash와 screenshot hash는 [manifest](ios/hashes.sha256)에 있다.
- 기본 Root HTML route가 `source=index.html`, `layout=ready boxes=9`, `root_app_marker=ready`, `SPINON_C0410_DRAW status=0 presented boxes=9`를 기록했다: [설치](ios/install.log) · [실행](ios/launch.log) · [unified log](ios/log.txt).
- [시뮬레이터 화면](ios/simulator-screen.png)은 1206×2622 OS screenshot이다. Metal drawable readback은 하지 않았다.

두 앱은 캡처 뒤 force-stop/terminate했고 확인 시 실행 PID가 없었다. `agent-browser` 프로세스도 남아 있지 않았다. Android 화면은 visual smoke일 뿐 surface pixel equality가 아니며, iOS 화면은 compositor 이후 readback 증거가 아니다. Android renderer는 AVD software path다.

## 적대 검토

구현/runtime 실패 관점 20개는 [별도 검토](implementation-runtime-review-20.md), source/test/evidence 변경분에 대한 PR 적대 검토 30개는 [PR diff 검토](pr-diff-review-30.md)에 있다. Chrome 156 비교, fixture 및 앱 입력 해시는 각각 precomparison README, `runtime-fixture.sha256`, `fixture-app-harness.sha256`, 플랫폼별 hash manifest에 보존했다.
