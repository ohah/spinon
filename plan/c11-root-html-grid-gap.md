# C11.1 · parsed Root HTML 기본 Grid의 고정 px gap

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML 기본 Grid 경로](./c11-root-html-grid-runtime.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 현재 후보는 Root HTML CSS allowlist와 typed layout admission까지 구현했다. Root HTML 회귀 18/18, computed gap admission 단위 검사 1/1, CSS-invalid gap 진단 단위 검사 1/1, Chrome 156 사전 비교 verifier 1/1, Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 실행, 구현/runtime 실패 관점 검토 20개가 통과했다. [실행 근거](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/README.md) · [구현/runtime 검토](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/implementation-runtime-review-20.md). PR 생성, 별도 PR diff 적대 검토 30개, GitHub check 확인 및 rebase merge는 남아 있다. 이 제한 조각은 C11.1, C11 또는 공개 CSS Grid 지원 완료로 표시되지 않으며 공식 `72/213`을 바꾸지 않는다.

**구현 전 입력:** [HTML fixture](../tests/fixtures/css/c11/root-html-grid-gap-author-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/expected.json) · [Agent Browser Chrome 관측](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/chrome-observation.json) · [화면](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/chrome-156-grid-gap-precomparison.png) · [기계 검증기](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/verify-precomparison.mjs)

**계획 실패 관점:** [구현 전 20개 검토](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/plan-review-20.md)

## 결손과 목표

`RuntimeSession::new_runtime_gpu_with_root_html()`은 기본 Root HTML Grid 경로에서 computed `row-gap`·`column-gap`이 `normal` 또는 0px일 때만 layout에 도달한다. 기존 typed layout/Taffy Grid 입력은 gap을 보유하지만 Root HTML CSS property allowlist와 Grid admission이 authored gap을 거부한다. 이 계획은 같은 parsed document/style revision 경로에서 명시적인 non-negative px gap만 통과시킨다.

CSS Grid gutter는 인접한 행·열 track 사이에만 존재하며 track 크기 합에 별도의 공간으로 반영된다. [`gap` shorthand](https://www.w3.org/TR/css-gaps-1/#gap-shorthand)에서 첫 값은 `row-gap`, 두 번째 값은 `column-gap`이다. 이 조각은 shorthand의 두 px 값과 동등한 두 longhand를 고정 입력으로 비교한다.

## 독립 기대값과 Chrome oracle

고정 장면은 130×65px definite Grid, columns `[40, 80]px`, rows `[30, 30]px`, `gap: 5px 10px`, 기본 row auto-placement, 고정 크기 직접 자식 네 개다. 독립 계산은 다음과 같다.

- 폭: `40 + 10 + 80 = 130px`.
- 높이: `30 + 5 + 30 = 65px`.
- source order item frame: `(0,0,40,30)`, `(50,0,80,30)`, `(0,35,40,30)`, `(50,35,80,30)` CSS px.
- row/column gap은 track의 바깥이나 첫/마지막 track에 더해지지 않는다.

Chrome `HeadlessChrome/156.0.0.0` 관측은 computed rows/columns/gap, root/item rect가 이 기대값과 같음을 확인했다. Chrome은 교차 oracle이며 독립 기대값을 대신하지 않는다. 기계 검증기는 주요 computed 값과 frame을 정확 비교하고 browser 버전이 다른 관측은 거부한다.

## 제한 범위

- 기존 기본 `new_runtime_gpu_with_root_html` entry, `RuntimeRootHtmlTextPaintV1` profile, fixed non-negative px explicit tracks, 기본 row auto-placement, implicit track 없는 직접 blockified 빈 item만 대상으로 한다.
- CSS `gap`, `row-gap`, `column-gap`이 Stylo에서 계산된 non-negative finite `LengthPx`일 때만 허용한다. `normal`과 0px은 기존 동작으로 0을 유지한다.
- `gap: 5px 10px`처럼 row/column 값을 분리하고 root가 두 축 모두 definite인 고정 입력에서 track/frame을 대조한다. 동등 longhand control과 source selector/ID 이름을 바꾼 positive control을 추가한다.
- typed `LayoutGap`을 같은 style snapshot에서 읽어 Taffy 계산에 한 번 전달한다. computed string을 다시 파싱하거나 item frame에 gap을 사후 보정하지 않는다.
- valid percentage, `calc()`/math-backed gap, cyclic/indefinite root, negative computed value, implicit tracks, spanning, non-default auto-flow, nested Grid와 그 밖의 Root HTML runtime 제외 문법은 계속 scene publication 전에 거부한다. CSS에서 유효하지 않은 음수 선언은 cascade가 버리는 규칙과 runtime의 computed-value admission을 구분한다.
- 이 admission은 `RuntimeRootHtmlTextPaintV1` Root HTML 경로에만 적용한다. 기존 fixture Grid profile의 입력 허용 범위는 이 계획으로 넓히지 않는다.
- same-document/style/layout revision 검증과 zero-gap 기존 Root HTML, text/color-scheme, explicit Grid fixture 회귀를 유지한다.

## 완료 관문

1. `verify-precomparison.mjs`가 고정 fixture, expected JSON, Agent Browser Chrome 관측을 검증한다.
2. Root HTML positive runtime scene의 root frame, two axis track/gap, four item frames, source order와 same snapshot key가 기대값에 일치한다. Shorthand/longhand, changed ids/selectors, zero-gap controls를 실행한다.
3. percentage와 `calc()` gap inputs는 `column-gap`/`row-gap`을 식별하는 `unsupported_computed_value` diagnostic으로 scene·partial frame을 내지 않고 거부한다. CSS-invalid negative shorthand는 CSS cascade가 declaration 전체를 버리고 zero-gap scene을 유지한다. 기존 nonzero edge/margin/border/overflow 및 explicit placement rejection도 유지한다.
4. Rust focused test, relevant workspace regressions, rustfmt/Clippy를 통과한다.
5. Android API 37 ARM64 AVD/SwiftShader 및 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 parsed source를 제품 Root HTML entry로 실행한다. source/build hashes와 runtime log·화면을 저장한다. 화면 캡처는 draw/present까지만 입증한다. PixelCopy/Metal drawable readback을 별도 실행하지 않으면 표면 pixel equality를 주장하지 않는다. Android 실기기는 사용하지 않는다.
6. 내부 계약 0059의 Root HTML 제한 profile을 갱신하되 숫자 버전 `0.1.0`을 유지한다. C11/C11.1 체크박스와 공식 `72/213`은 미완료로 둔다.
7. 구현 후 독립적인 runtime/code failure 관점 20개 검토, PR diff의 서로 다른 적대 검토 30개를 마치고 발견 결함은 수정 후 재검토한다. PR check run이 없으면 CI 성공으로 쓰지 않는다.

## 완료로 주장하지 않는 것

이 조각은 percentage/calc gaps, indefinite percentage resolution, track sizing 일반화, named lines/areas, auto-repeat, placement/span, Grid alignment, nested Grid, general CSS admission, Spinon WPT 적합성, 실기기/hardware GPU, surface pixel equality, 전체 C11.1/C11.3/C11.4/C11 지원을 검증하지 않는다.
