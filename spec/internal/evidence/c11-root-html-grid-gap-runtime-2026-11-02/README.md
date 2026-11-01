# C11.1 parsed Root HTML Grid fixed-px gap 사전 비교

## 고정 입력과 independent expectation

[`tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html)는 definite 130×65px grid, fixed columns `[40,80]px`, rows `[30,30]px, `gap:5px 10px`, row-major placement 네 장면이다. Independent expectation은 `40+10+80=130`, `30+5+30=65`로 track/gap extents를 각각 계산하고 source-order item frame 네 개를 [`expected.json`](./expected.json)에 고정한다.

## Agent Browser 관측

- CLI: `agent-browser` 0.31.1, Headless Chrome 156 (`HeadlessChrome/156.0.0.0`)
- viewport: 390×844 CSS px
- 결과: root `(0,0,130,65)`, columns `40px 80px`, rows `30px 30px`, `column-gap:10px`, `row-gap:5px`; 네 item frame은 expected JSON과 exact match
- capture: [`chrome-156-grid-gap-precomparison.png`](./chrome-156-grid-gap-precomparison.png)
- `agent-browser console --clear` 및 `agent-browser errors --clear`에서 page error가 보고되지 않았다.
- 기계 대조: `node spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/verify-precomparison.mjs`

Chrome 관측은 CSS oracle 비교이며 Spinon 결과나 WPT 통과가 아니다. 기대값은 definite px tracks, gap sizes와 row-major placement에서 독립 계산한다. 기준 정의는 [CSS Grid Layout Module Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters) 및 [CSS Gaps Module Level 1 §2.1](https://www.w3.org/TR/css-gaps-1/#row-column-gutters)이다.

## 구현 및 플랫폼 실행 결과

- 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로가 fixture 전용 FFI 없이 고정 HTML을 읽었다. probe는 V8 document에서 `#gap-grid`와 class/source order가 맞는 직접 자식 네 개를 확인했다.
- `cargo test --locked -p spinon-runtime root_html_ -- --nocapture`: 18/18 통과. 이 안에 C11.1의 shorthand·longhand, selector/ID 변경, zero gap, CSS-invalid 음수 shorthand/inline 선언, percentage·`calc()` 거부와 partial-scene 비공개 대조가 있다.
- `cargo test --locked -p spinon-style root_html_gap_recovery_ignores_only_invalid_negative_fixed_px_declarations -- --nocapture`: 1/1 통과. 잘못된 음수 fixed-px gap 선언만 무시하고, 다른 parser 진단은 fail-closed로 남는다. 대소문자 변형도 포함한다.
- `cargo test --locked -p spinon-style-to-layout root_html_grid_gap_admits_only_finite_nonnegative_px_or_normal -- --nocapture`: 1/1 통과. `normal`·유한 non-negative px는 허용하고 음수·NaN·무한대·percentage·auto·unsupported는 거부한다.
- `cargo fmt --all -- --check`, 관련 세 crate Clippy와 아래 구현 전 Chrome oracle verifier 통과. Clippy 출력의 `runtime_grid_tests.rs` dead-code 경고 2개는 변경 밖의 기존 test helper 경고다.
- 최종 후보의 위 명령과 결과는 [validation-pass03.log](./validation-pass03.log)에 보존했다.

플랫폼 app은 현재 candidate 소스, C04 runtime GPU on, fixture-test flags off로 다시 빌드했다. AVD의 첫 자동 입력 실행은 `monkey`가 no-physical-keys 환경에서 exit `-5`를 반환했다. 해당 로그를 보존하고, 최종 pass03은 명시적 `MainActivity` 실행으로 검증했다.

| 대상 | 런타임 근거 | 화면 |
| --- | --- | --- |
| Android API 37 ARM64 `spinon_api37_0_applied_txn` AVD, ANGLE/Vulkan/SwiftShader | [빌드·설치 artifact·layout/present 로그](./platform-runtime-observation.json) · [원본 app 로그](./android-api37-avd-gap-runtime-pass03.log) | [AVD 화면](./android-api37-avd-grid-gap-current-source-pass03.png) |
| iPhone 17 Pro, iOS 26.2 Simulator, Metal Simulator GPU | [빌드·실행 artifact·layout/present 로그](./platform-runtime-observation.json) · [원본 app 로그](./ios-26_2-simulator-gap-runtime-pass03.log) | [iOS 화면](./ios-26_2-simulator-grid-gap-current-source-pass03.png) |

양 플랫폼에서 기본 Root HTML route가 `layout=ready`, root frame 높이 65 CSS px, `boxes=7`을 보고했고 draw/present log가 7개 box를 표시했다. Android viewport는 `411.42856×841.52374` CSS px, DPR 2.625이며 iOS viewport는 `402×778` CSS px, DPR 3이다. 설치 app의 SHA-256, source manifest, feature flags와 실행 환경은 [platform-runtime-observation.json](./platform-runtime-observation.json)에 묶었다. 소스 파일 manifest는 [candidate-source.sha256](./candidate-source.sha256)다.

screenshots는 ADB display capture와 `simctl io screenshot`이다. GPU surface readback이나 compositor 뒤의 pixel equality를 주장하지 않는다. 정확한 track·gap·item frame 비교는 Rust integration test의 검증이며, simulator log는 기본 HTML runtime이 성공적으로 layout하고 7개 box를 draw/present한 근거다. Android는 software-rendered AVD이지 Samsung 실기기가 아니고, iOS는 Simulator다. 실행 후 두 앱 프로세스를 종료하고 임시 변경한 example/generated bootstrap source asset을 원래 파일로 복구했다.

구현 뒤의 별도 [실패 관점 검토 20개](./implementation-runtime-review-20.md)는 CSS-invalid 음수 shorthand에서 주석 토큰이 cascade를 실패시키는 재현을 포함한다. 진단 parser를 보완하고 같은 runtime 입력을 다시 통과시켰다.

이 구현 조각과 실행 결과는 C11.1 전체, C11, 공개 CSS Grid 적합성의 완료 선언이 아니다. 공식 완료는 `72/213`으로 유지한다.
