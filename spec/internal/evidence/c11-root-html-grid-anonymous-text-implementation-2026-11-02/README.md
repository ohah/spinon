# C11.1 Root HTML anonymous Grid text 제한 구현 근거

이 증거 묶음은 계획 [C11.1 parsed Root HTML 기본 Grid anonymous text 제한 구현](../../../../plan/c11-root-html-grid-anonymous-text.md)의 한 후보 slice만 다룬다. 코드 diff는 `RuntimeSession::new_runtime_gpu_with_root_html()` 기본 parsed Root HTML 경로에서 direct `AB` text 하나를 anonymous Grid item으로 배치하고 paint한다. C11.1·C11.4·C11은 미완료이며 공식 `spec/STATUS.md` 집계는 변경하지 않았다.

## 기준과 범위

- 기준 입력과 Chrome 156 사전 관측은 [precomparison evidence](../c11-root-html-grid-anonymous-text-precomparison-2026-11-02/README.md)에 있다. `verify-precomparison.mjs` 실행 결과는 PASS다.
- 정규 기대값은 Grid `(0,0,120,70)`, columns `40px 80px`, row `70px`, gap `0`, anonymous item `(0,0,40,70)`, text origin `(0,0)`의 한 줄이다.
- 지원 입력은 `white-space:normal`, monospace 16px/20px, LTR, 기본 text alignment, `#e5484d`로 제한한다. visible element/text 혼합, 지원하지 않는 style, 다른 text 값과 복수 text Grid는 거부한다.
- [CSS Grid §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items)은 white-space-only text sequence를 렌더링하지 않도록 규정한다. 기존 Root HTML 경로도 `white-space:pre`인 white-space-only sequence를 무시하는 회귀를 유지한다.
- 이 변경은 일반 auto-placement, intrinsic contribution, line wrapping, inline descendants, WPT conformance, 실기기 GPU, simulator 간 pixel equality 또는 전체 C11 완료를 주장하지 않는다.

## 실행 검증

| 검증 | 결과 | 원본 |
|---|---:|---|
| Precomparison verifier, HeadlessChrome 156 | PASS | `../c11-root-html-grid-anonymous-text-precomparison-2026-11-02/verification.json` |
| `cargo fmt --all -- --check` | PASS | 터미널 실행, Rust source 전부 formatter 통과 |
| `cargo test --locked -p spinon-runtime c11_1_root_html_default_runtime_` | 7/7 PASS | `runtime-root-html-tests.log` |
| `cargo test --locked -p spinon-runtime root_html` | 54/54 PASS | `runtime-root-html-regressions.log` |
| `cargo test --locked -p spinon-style-to-render` | 71 PASS, 1 ignored | `style-to-render-tests.log` |
| `cargo test --locked -p spinon-style-to-layout root_html` | 2/2 PASS | `style-to-layout-root-html-tests.log` |
| `cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test c11_4_anonymous_text` | 32/32 PASS | `layout-c11-4-anonymous-text-tests.log` |
| C11.4 fixture FFI positive control | 1/1 PASS | `ffi-c11-4-positive-control.log` |
| scoped default all-target Clippy (`layout`, `style-to-layout`, `style-to-render`, `runtime`) | PASS | `clippy-default.log` |
| C11.4 feature runtime library Clippy | PASS | `clippy-c11-4-library.log` |

### 남은 비녹색 게이트

- 전체 `spinon-style-to-layout`은 279개 중 278개 통과했다. 유일한 실패는 변경하지 않은 C07 aspect-ratio 비교 테스트에서 computed style 문자열 `1.7777778 / 1`과 고정 예상값 `1.77778 / 1`이 달라서 발생했다. 실행 원문과 exit code는 `style-to-layout-full-tests.log`, `style-to-layout-full-tests.exit`다.
- C11.4 feature의 runtime all-target Clippy는 변경하지 않은 `runtime_layout_tests/c11_grid_tests/anonymous_text_hyphens.rs` test helper에 `too_many_arguments (11/7)`이 있어 실패했다. 제품 library Clippy와 default scoped Clippy는 통과했다. 로그와 exit code는 `clippy-c11-4-feature.log`, `clippy-c11-4-feature.exit`다.
- 전체 workspace test/Clippy 및 CI 통과를 주장하지 않는다.

## Android API 37 ARM64 AVD

API 37 ARM64 AVD (`emulator-5556`, SwiftShader software render path)에서 기본 parsed Root HTML 앱을 빌드·설치·실행했다. 앱 boot 로그는 `layout=ready boxes=3`, GPU draw/present 로그는 `runtime_text_count=1`, `glyph_batch_draw_count=1`, `root_css_px=411.42856x70`을 보고한다. 결과 화면은 [Android screenshot](./android-avd/c11-root-html-grid-anonymous-text.png)이다.

build/install/launch 로그와 logcat은 `android-avd/`에 있다. 입력 HTML SHA-256은 고정 fixture와 동일하게 `6565f9eb6de1aafc90d5ef4a2fd1a1b7155678d6a8a0ba0dd8f1ef69408a2276`이다. 이 결과는 AVD의 software renderer이며 Android 실기기나 hardware GPU 검증이 아니다.

빌드·테스트 stdout 로그는 원문 출력을 유지하고, Git diff의 끝 공백 경고만 없애기 위해 줄 끝 공백과 마지막의 추가 빈 줄을 정규화했다. iOS 설치 명령은 stdout이 비어 있어 성공 exit code만 별도 기록했다.

## iOS Simulator

iPhone 17 Pro, iOS 26.2 Simulator에서 같은 기본 parsed Root HTML 앱을 빌드·설치·실행했다. boot 로그는 `layout=ready boxes=3`, draw/present 로그는 `runtime_text_count=1`, `glyph_batch_draw_count=1`, `root_css_px=402x70`을 보고한다. 결과 화면은 [iOS screenshot](./ios-simulator/c11-root-html-grid-anonymous-text.png)이다.

build/install/launch 로그, device metadata와 unified log는 `ios-simulator/`에 있다. 이는 iOS Simulator 검증이며 실기기나 compositor 이후 픽셀 동등성을 뜻하지 않는다.

## 코드·런타임 실패 관점 검토

- 실제 변경 코드와 기본 runtime 동작에 대한 새 [구현 실패 관점 검토 20개](./implementation-runtime-review-20.md)를 수행했다.
- 계획 검토와 구별되는 [실제 implementation diff PR 검토 30개](./implementation-pr-diff-review-30.md)를 수행했다.
- pre-build input backup은 로컬 evidence 디렉터리에 보존되어 있지만 원본 앱 bootstrap 파일 변경은 모두 복구했다. PR에는 실제 입력 사본을 복제하지 않고 결과·hash·build/runtime 증거만 포함한다.

## 재현 명령

```sh
node spec/internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/verify-precomparison.mjs
cargo fmt --all -- --check
cargo test --locked -p spinon-runtime c11_1_root_html_default_runtime_
cargo test --locked -p spinon-runtime root_html
cargo test --locked -p spinon-style-to-render
cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test c11_4_anonymous_text
```
