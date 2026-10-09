# C04.7 CSS media 환경 cascade 실행·구현 검토 근거

## 변경 내용

- `CssViewport`에 `CssMediaEnvironment`를 포함하고 색상 scheme, primary pointer/hover, 전체 pointer capability를 Stylo 0.22 `Device`에 명시적으로 전달한다.
- C04.6 `FlexMarginV1`의 allowlist는 유지하고 별도 `FlexMediaEnvironmentV1`와 `compute_flex_media_environment_cascade`를 추가한다.
- 허용 query는 `prefers-color-scheme: light|dark`, `pointer`·`any-pointer: none|coarse|fine`, `hover`·`any-hover: none|hover`의 feature/value 쌍이다. `screen`·`all`, `and`·`not`, comma list를 허용한다.
- viewport media 환경 검증, 쿼리 구문·feature/value 검증, 지원 밖 at-rule·feature 거부를 추가했다. Taffy adapter는 새 profile을 명시적으로 지원하지 않는다.
- desktop/mobile × light/dark의 Chromium CDP reference와 Rust 비교 fixture를 추가했다.

## Chromium 기준

- Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- macOS `26.5.1` build `25F80`, arm64, Node `v24.20.0`
- viewport `390×844` CSS px, DPR `3`, locale `en-US`, timezone `UTC`
- mobile 입력은 CDP touch emulation으로 설정한 뒤 실제 `matchMedia()` 결과가 coarse/no-hover인지 확인했다.
- 네 case 각각 12개 `matchMedia()`와 12개 CSS probe의 computed `display`를 수집했다. 총 48개 media match와 48개 computed-style 값이 fixture 입력과 일치했다. 이 검증은 geometry나 렌더된 픽셀을 비교하지 않는다.
- mixed pointer와 pointer 없음은 이 Chromium emulation으로 측정하지 않았다. 이 입력은 Stylo 0.22 mapping/invariant 테스트에만 포함한다.
- reference ID: `chromium-Chrome-154.0.8037.98-25ad954b5ef3-631b0a7d28c7-e01557a07457-ccffd5c5fe77`
- 고정 reference 파일은 캡처 스크립트에서 존재 여부를 확인하고 exclusive create로 기록해 덮어쓰지 않는다. `bun run test:css-reference`가 입력·HTML·CSS·capture 도구·Chromium executable hash와 provenance를 확인한다.

## 구현 후 적대적 검토 — 20개 독립 실패 관점

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | C04.7 추가가 C04.6 `FlexMarginV1`에서 허용하던 at-rule 경계를 넓히는가? | 기존 profile은 `@media`를 계속 거부하고 신규 profile에서만 허용한다. 회귀 테스트 통과. |
| 2 | 새 profile이 margin 외 author property까지 실수로 허용하는가? | C04.6과 같은 `FLEX_MARGIN_AUTHOR_PROPERTIES` allowlist를 사용한다. workspace 회귀 테스트 통과. |
| 3 | 잘못된 viewport와 media environment가 동시에 주어졌을 때 오류 순서가 흔들리는가? | `InvalidViewport`가 먼저 반환되는 테스트 통과. |
| 4 | primary가 `none`인데 primary hover가 true일 수 있는가? | 모순 입력을 `InvalidMediaEnvironment`로 거부한다. |
| 5 | primary coarse가 전체 capability에 빠질 수 있는가? | 거부 테스트 통과. |
| 6 | primary fine이 전체 capability에 빠질 수 있는가? | 거부 테스트 통과. |
| 7 | primary hover를 지원하면서 전체 hover가 false일 수 있는가? | 거부 테스트 통과. |
| 8 | 전체 hover만 true이고 coarse/fine 장치가 없을 수 있는가? | 거부 테스트 통과. |
| 9 | 여러 장치 조합에서 coarse와 fine을 함께 표현할 수 있는가? | 혼합 capability 입력을 허용하고 primary와 `any-pointer` 계산을 따로 확인했다. |
| 10 | primary 포인터가 없고 보조 장치만 있는 값을 한 종류로 뭉개는가? | primary `pointer:none`과 `any-pointer:fine`을 각각 보존하는 mapping 테스트 통과. |
| 11 | light scheme에서 dark query가 함께 맞는가? | desktop/mobile light case 모두 12 media 결과와 computed-style 비교 통과. |
| 12 | dark scheme 입력이 실제 Stylo Device까지 전달되는가? | desktop/mobile dark case 모두 Chromium reference와 computed-style 비교 통과. |
| 13 | 모바일 환경을 Android/iOS 기본값으로 추정해 desktop 값이 유출되는가? | target 기본값에 기대지 않고 명시 입력을 전달한다. CDP mobile touch 결과를 실행 중 확인했다. 실제 모바일 OS 수집은 미검증·미구현으로 남긴다. |
| 14 | 값 없이 쓴 `(prefers-color-scheme)`가 Stylo 복구 뒤 지원 query로 통과하는가? | 최초 적대 테스트에서 통과 결함을 찾았다. feature/value 쌍 검증을 추가했고 현재 거부 테스트 통과. |
| 15 | 다른 feature의 허용 값을 잘못 끼워 넣어도 받아들이는가? | `(pointer: light)`, `(prefers-color-scheme: fine)`, `(hover: coarse)`, 다중 값을 거부한다. |
| 16 | viewport 크기·reduced motion·미지 feature·print·`or`·`@supports`·`@import`·알 수 없는 at-rule이 허용되는가? | AST에 남지 않고 Stylo parse diagnostic으로만 남는 at-rule까지 `UnsupportedAuthorCss`로 거부한다. 각 입력 테스트 통과. |
| 17 | 지원한다고 문서화한 `screen`·`all`·`and`·`not`·comma 조합이 실제 평가되는가? | 참/거짓 조합 여섯 가지의 computed `display` 테스트 통과. |
| 18 | 일반 CSS 진단이 media/at-rule 오류로 오인되거나, parse recovery가 unsupported at-rule을 누락하는가? | `@import`가 AST에서 빠지고 진단만 남는 경로를 찾아 invalid media/at-rule 진단 prefix를 검사한다. 일반 declaration 진단은 보존하는 테스트 통과. |
| 19 | environment revision, style revision, 입력 media environment가 결과에서 사라지거나 섞이는가? | snapshot에 입력 전체와 `EnvironmentRevision`·`StyleRevision`을 보존하는 테스트 통과. |
| 20 | 계산 profile을 Taffy·제품 runtime·OS API·GPU가 이미 지원한다고 과장하는가? | layout adapter는 새 profile을 명시적으로 거부한다. contract·status·fixture 문서가 OS 수집, 자동 재계산, runtime·layout·GPU 연결을 미구현으로 표시한다. |

적대 테스트에서 발견한 세 결함은 bare boolean media feature의 통과, 지원 feature와 맞지 않는 값의 통과, `@import`처럼 AST에는 없고 진단에만 남는 at-rule의 누락이었다. feature/value pair와 media/at-rule 진단 식별을 보강했다. 일반 declaration 진단을 잘못 거부하지 않는 negative control도 추가했으며, 수정 후 관련 관점과 전체 검사를 다시 실행했다.

## 실행 결과

- `mise exec -- cargo test --locked --workspace` — 통과, unit/integration test 213개, doctest 0개.
- `mise exec -- cargo test --locked -p spinon-style media_environment -- --nocapture` — 통과, C04.7 전용 테스트 7개.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `mise exec -- cargo fmt --all -- --check` — 통과.
- `mise exec -- bun run test:css-reference` — 통과, CSS reference 검증 19개.
- `git diff --check` — 통과.
- `mise exec -- cargo check --locked --workspace --target aarch64-apple-ios-sim` — 통과.
- `mise exec -- cargo check --locked --workspace --target aarch64-linux-android` — 통과.

두 target 결과는 Rust workspace compile만 의미한다. iOS/Android 앱 빌드·실행이나 OS media 환경 수집을 검증한 것은 아니다.

## 남은 범위

OS의 색상·입력 환경 수집, 환경 변경 알림, environment revision 발급자, stale 계산 취소, 자동 재계산, 모바일 runtime 연결, 일반 layout/Taffy 연결, GPU 프레임·픽셀 비교는 구현·검증하지 않았다. mixed/no-pointer 입력에 대한 Chromium 실측도 없다. 이 PR은 내부 Rust API의 제한 CSS cascade 결과까지만 다룬다.
