# C04.6 CSS margin → Taffy 실행 근거

**구현 계약:** [0027 C04.6 Flex margin adapter](../0027-c04-flex-margin-layout.md) · **계획:** [C04.6 margin 구현 계획](../../../plan/c04-margin-layout.md) · **상태:** 구현 검증 중

## 구현 결과

새 `FlexMarginV1` profile과 `compute_flex_margin_style_layout` 내부 API를 추가했다. 제한 Flex CSS cascade의 computed physical margin 네 값을 `LayoutStyle.margin`으로 변환하고 Taffy에 전달한다. CSS margin은 음수 유한 px를 허용하고, padding·gap의 기존 유효성 규칙은 바꾸지 않는다. 기존 profile은 margin 선언을 계속 거부한다.

`margin`, `margin-inline`, `margin-block`, 네 physical 및 네 logical margin longhand를 Stylo가 계산한 물리 edge로 투영한다. `horizontal-tb`만 대상으로 하며 `writing-mode` 선언은 거부한다. `%`, `auto`, 비-px computed value, 선택 root의 nonzero margin, 미지원 CSS·diagnostic은 부분 layout 없이 오류가 된다.

고정 Chromium `154.0.8037.98` 기준 10개 기본 case와 3개 logical shorthand case를 각각 immutable input/reference로 추가했다. 성공 판정은 computed margin 문자열 정확 일치 및 각 표시 box 좌표의 case별 최대 절대 오차 `0.5 CSS px` 이하이다. `display:none` 요소 자체는 layout box가 없으므로 그 요소의 DOMRect 원점은 비교 대상에서 제외하고 뒤의 표시 item 좌표는 비교한다.

구현 도중 발견한 결함은 계획·코드에 반영했다.

- Stylo가 logical margin shorthand/longhand를 logical 이름으로 내보내므로 새 profile에서만 명시 allowlist에 넣고 computed physical edge를 전달한다.
- 선택 root margin은 Taffy의 viewport-root 입력으로 정확히 해석할 containing block이 없어 nonzero 값을 오류로 거부한다.
- `display:none` 요소의 Chromium `getBoundingClientRect()` viewport 원점은 layout frame으로 비교하지 않되, 해당 요소의 margin이 뒤의 표시 sibling에 미치는 영향은 검사한다.

## 구현 적대 검토 · 20개 실패 관점

| # | 실패 관점 | 근거·판정 |
|---:|---|---|
| 1 | 새 `LayoutStyle.margin`의 기본값이 기존 배치를 바꿈 | 네 방향 default 0을 코드에서 확인했고 전체 workspace 회귀가 통과했다. |
| 2 | 음수 margin을 padding처럼 거부함 | 음수 CSS margin fixture와 `negative_margin_reaches_taffy_and_non_finite_margin_is_rejected`의 `x - 7px` 결과가 통과했다. |
| 3 | NaN 또는 양·음의 무한대가 Taffy에 들어감 | margin finite 검사와 NaN·±Inf 거부 단위 테스트가 통과했다. |
| 4 | 물리 위·오른쪽·아래·왼쪽 값을 서로 바꿈 | 비대칭 네 방향 longhand의 computed values 및 표시 frame 비교가 통과했다. |
| 5 | 한 값 `margin` shorthand가 네 방향으로 확장되지 않음 | `shorthand-one-value`의 네 computed values와 표시 frame 비교가 통과했다. |
| 6 | 두 값 shorthand의 vertical/horizontal 순서가 뒤바뀜 | `shorthand-two-values`의 네 computed values와 표시 frame 비교가 통과했다. |
| 7 | 세 값 shorthand가 left/right 대칭 규칙을 놓침 | `shorthand-three-values`의 computed values와 표시 frame 비교가 통과했다. |
| 8 | 네 값 shorthand가 clockwise 순서를 놓침 | `shorthand-four-values`의 네 computed values와 표시 frame 비교가 통과했다. |
| 9 | logical inline margin의 LTR 시작/끝 매핑이 틀림 | `margin-inline` LTR shorthand와 logical longhand reference 비교가 통과했다. |
| 10 | RTL inline start/end가 물리 좌우로 반전되지 않음 | 기본 fixture와 보충 fixture의 RTL shorthand/longhand 비교가 통과했다. |
| 11 | logical block shorthand가 top/bottom을 뒤바꿈 | `margin-block` horizontal-tb computed values 및 frame 비교가 통과했다. |
| 12 | vertical writing mode를 수평으로 잘못 처리함 | `writing-mode: vertical-rl`이 cascade 오류로 닫히는 테스트가 통과했다. |
| 13 | percentage margin을 viewport 기준 px로 잘못 취급함 | `10%`가 unsupported computed value로 반환되는 테스트가 통과했다. |
| 14 | `auto` margin을 0 또는 임의 분배로 취급함 | `auto`가 unsupported computed value로 반환되는 테스트가 통과했다. |
| 15 | `calc()`에서 일부 수치만 추출해 적용함 | `calc(10% + 2px)` 전체가 거부되는 테스트가 통과했다. |
| 16 | gap과 margin이 중복 또는 누락 적용됨 | column gap과 양쪽 margin을 함께 둔 Chromium geometry 비교가 통과했다. |
| 17 | `display:none` margin이 숨겨진 item을 차지하거나 뒤 item을 밀어냄 | hidden 자체 DOMRect는 제외하고 뒤의 표시 item frame을 비교한 결과가 통과했다. |
| 18 | 루트 margin이 Taffy에서 무시되어 성공처럼 보임 | nonzero root margin을 `UnsupportedRootMargin`으로 반환하는 테스트가 통과했다. |
| 19 | inline style·parse diagnostic·allowlist 밖 선언에서 부분 layout을 반환함 | inline style, 잘못된 선언, `writing-mode`, `padding` 입력 각각의 fail-closed 테스트가 통과했다. |
| 20 | 신규 profile이 이전 API 범위를 넓히거나 revision을 잃음 | 이전 profile의 margin 거부와 style/environment/viewport revision 보존 테스트가 통과했다. |

별도 계획 적대 검토는 [C04.6 계획 검토](c04-margin-layout-plan-review-2026-10-09.md)에 둔다. 표는 테스트 이름만 나열한 완료 선언이 아니며 각 행의 실제 실행 상태와 전체 명령 결과를 아래 기록에 연결한다.

## 실행 환경과 결과

아래 결과는 구현이 고정된 뒤 실행해 날짜·도구 버전·명령·통과 수·실패 원인을 기록한다. 앱 runtime, 실제 화면, GPU, Android/iOS 기기 동작은 이 내부 CSS→layout 기능의 검증 범위가 아니다.

| 항목 | 결과 |
|---|---|
| 소스·fixture SHA-256 | 실행 후 기록 |
| Chromium reference 환경 | Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS `26.5.1` arm64, locale `en-US`, timezone `UTC`, viewport `301×80 CSS px`, scale `1` |
| 소스·fixture SHA-256 | 기본 input `1f0309565aa5ea8a5e7ddc291ae92125153ee6b52e791fcef7c8a1b6145dc071`; logical input `4dc070935fafc6c743b3b41a5902b6b98cdf4536eccaad42ff65dc9663517c36`; 기본 reference `05fbf530b0da4fd69b2a3fae17defd9c15f428e210ed85d4e263f0736b59ed7d`; logical reference `b9548257297697ddb176616ae693b8be5e0a6411dd1d0962153c41474c61dc80` |
| Rust workspace tests | `mise exec -- cargo test --locked --workspace` · 187 passed, 0 failed |
| Rust Clippy | `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` · 통과 |
| Rust formatting | 변경된 Rust 파일 전체에 `rustfmt --check --edition 2024 --config skip_children=true` 통과. `cargo fmt --all -- --check`는 HEAD에도 그대로 있는 `cascade/ua_baseline_tests.rs` 포맷 차이로 실패하며 해당 파일은 변경하지 않았다. |
| CSS reference self-tests | `mise exec -- bun run test:css-reference` · 18 passed, 0 failed |
| Android/iOS target compile | `cargo check --locked --workspace --target aarch64-linux-android` 및 `--target aarch64-apple-ios-sim` 모두 통과. 앱 실행과 구분 |
| 앱 화면·GPU·제품 runtime | 미검증; 이 구현 범위에 포함하지 않음 |

## 남은 경계

이는 C04 내부 제한 Flex profile의 새 기능이다. 제품 runtime의 stylesheet owner·root·viewport·media 전달, UA profile과 결합, 일반 CSS, Block margin collapse, percentage/auto margin, CSSOM, text layout, GPU 및 Android/iOS 화면에는 아직 연결하지 않았다. C04 전체 완료를 뜻하지 않는다.
