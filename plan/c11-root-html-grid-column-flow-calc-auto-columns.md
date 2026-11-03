# C11.1 · parsed Root HTML `calc()` implicit columns

**상태:** 제한 구현 후보를 현재 checkout에서 검증 중이다. 아직 병합되지 않았고 공식 완료가 아니다. C11.1/C11 체크와 공식 완료 `72/213`은 바꾸지 않는다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [parsed Root HTML column-flow](./c11-root-html-grid-column-flow.md) · [percentage implicit columns](./c11-root-html-grid-column-flow-percent-implicit-columns.md) · [fraction implicit columns](./c11-root-html-grid-column-flow-fr-implicit-columns.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 기준:** [Chrome 156 관측, 독립 계산, range controls](../spec/internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-precomparison-2026-11-03/README.md) · [고정 fixture](../tests/fixtures/css/c11/root-html-grid-column-flow-calc-auto-columns-precomparison.html) · [계획 실패 관점 20개](./c11-root-html-grid-column-flow-calc-auto-columns-plan-review-20.md) · [계획 PR diff 검토 30개](../spec/internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-precomparison-2026-11-03/plan-pr-diff-review-30.md)

## 문제와 목표

`RootHtmlColumnFlowV1`은 parsed Root HTML `grid-auto-flow:column`에서 direct finite non-negative px, percentage, fraction implicit columns를 지원한다. 기존 percentage/fr 계획은 CSS math track을 명시적으로 범위 밖에 둔다. `GridTrackSize::Calc`와 node/property-bound math projection 및 Taffy calc 전달 경로는 있지만 Root HTML column-flow admission·track validator·extent 검사에서 해당 track을 거부한다. 이 계획은 definite-width Root HTML 경로에서 제한된 `<length-percentage>` sum 계산식을 `grid-auto-columns` scalar/list에 연결한다.

## 규범 범위와 고정 동작

- [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.2.1, 7.6, 7.7, 11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에서 `<track-breadth>`는 `[0,∞]` 범위 `<length-percentage>`를 포함하고, `grid-auto-columns` track-size 목록은 필요한 implicit track에 앞에서부터 반복된다. §11.1은 definite grid size가 알려진 뒤 track percentage를 grid container 크기에 resolve한다.
- [CSS Values and Units Level 4, 2024-03-12 WD §§10.9, 10.12](https://www.w3.org/TR/2024/WD-css-values-4-20240312/)에 따라 계산식은 해당 타입을 받는 문맥에서 사용 가능하고 top-level 결과는 target range로 clamp한다. 따라서 계산 결과가 음수인 사례는 used track `0px`가 기대값이다.
- [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-precomparison-2026-11-03/README.md)는 oracle 관측이다. 판정의 독립 출처는 위 명세와 산술이며 browser serialization은 기대값을 대체하지 않는다.

고정 주 장면은 content width `240px`, content height `63px`, explicit columns `[30,40]px`, rows `[20,40]px`, column/row gaps `5/3px`, 12개의 고정 `10×10px` direct empty `div`, `grid-auto-flow:column`이다. `grid-auto-columns` pattern은 `calc(10% + 5px) calc(2% + 2px)`이고, definite width를 기준으로 네 implicit columns에 두 번 순환 적용한다.

| 검사 | 독립 기대값 | 확인할 경계 |
|---|---|---|
| 주 장면 | used columns `[30,40,29,6.8,29,6.8]px`; starts `[0,35,80,114,125.8,159.8]px`; rows `[20,40]px`, starts `[0,23]px` | 2항 calc, percentage basis, 목록 순환, gutter를 basis에서 빼지 않음 |
| negative range | width 240px, `calc(1px - 10%)` → implicit tracks `0px` | CSS non-negative track range clamping |
| over-100 | width 240px, `calc(120% + 1px)` → implicit tracks `289px` | container보다 큰 track 보존 |
| zero basis | width 0px, `calc(10% + 5px)` → implicit tracks `5px` | definite zero percentage basis |
| mixed list | `10px calc(10% + 5px) 15%` → `[10,29,36,10]px` repeated | direct px/percentage와 calc 순서 유지 |
| fraction mix | `calc(10% + 5px) 1fr` → implicit `[29,43.5,29,43.5]px` | calc track, existing direct fr와 remaining-space distribution |

Chrome direct used-value positions는 `1/64px` 단위로 양자화된다. Geometry 대조 허용 오차는 `0.02 CSS px`이며 raw observation은 이 양자화값을 수정 없이 유지한다.

## 구현 중 확인한 computed-style 경계

고정 길이만 계산하는 CSS math는 Stylo computed style에서 `Calc` tree가 아니라 direct `LengthPx`로 정규화될 수 있다. 예를 들어 `grid-auto-columns:calc(2px * 3)`은 computed `6px`가 되어 기존 direct-px 경로를 탄다. 계산 결과가 직접 length-percentage track으로 정규화되지 않고 typed `Calc`로 남은 경우에만 아래의 두 항 signed `Sum` 제한을 적용한다.

Root HTML inline style에서 문법 오류인 `grid-auto-columns:calc(10% + )`는 computed initial `auto`로 무시되지 않고 cascade `CascadeDiagnostic`으로 반환된다. 유효한 CSS지만 이 profile이 받지 않는 `min(10%, 20px)`은 typed `Calc`로 보존된 뒤 style projection에서 거부된다. 구현 검증은 두 오류 경로를 분리하며 syntax-invalid CSS의 browser-compatible fallback을 주장하지 않는다.

## 제한 지원 계약

- 대상은 기본 parsed Root HTML route의 하나의 non-nested sparse column-flow Grid뿐이다. 새로운 CSS admission, runtime profile, fixture FFI/ABI는 추가하지 않는다.
- 기존 direct px, percentage, fraction track과 목록 순환을 유지한다. 새 calc 항목은 computed typed `Calc(Sum(...))`가 유한한 px 항 하나와 유한한 percentage 항 하나를 갖는 형태만 허용한다. 각 항은 값 자체 또는 단항 negate로 부호를 보존하고, 두 항의 순서는 모두 받는다. `calc(1px - 10%)` 관측에 따라 부호가 붙은 항을 처리하고, 사용 결과는 `grid-auto-columns`의 non-negative 범위로 clamp한다. 곱셈·나눗셈·min/max/clamp 및 다른 tree shape는 이 slice에서 fail-closed한다.
- percentage를 포함하는 calc가 있으므로 Grid content width는 definite finite non-negative direct CSS-pixel `width`여야 한다. `min-width`, `max-width`, `aspect-ratio`, math-backed/indefinite width는 거부한다. definite `0px`는 유효하다.
- 계산식 record는 동일 Grid node와 `GridTemplateColumns` math binding에 묶이고, ID 누락/중복, 소유자/property 불일치, 잘못된 dimension, 비유한 입력·extent는 layout 전에 거부한다. 기존 math evaluator가 적용하는 `33,554,428 CSS px` 수치 ceiling을 명시하고, cumulative extent는 별도의 finite-f32 검사로 보호한다.
- explicit rows/columns는 기존 finite non-negative direct px subset이고, gap은 기존 fixed px subset이다. Area, explicit repeat/named line, implicit rows, row/dense flow, authored placement, nested Grid, item content/intrinsic sizing, 비-visible overflow, nonzero box edge는 기존처럼 제외한다.

## 구현 및 검증 관문

1. 본 계획 PR 전에 계획 실패 관점 20개와 계획 PR 변경분 실패 관점 30개를 서로 다른 항목으로 기록한다. 계획 PR 병합 뒤 최신 `main`에서 구현 브랜치를 만들며 worktree를 추가하지 않고 현재 checkout을 순차 사용한다.
2. Style projection은 허용된 `Calc(Sum(length, percentage))`만 Root HTML `grid-auto-columns`로 통과시킨다. 일반 computed-style admission과 다른 Grid profile은 넓히지 않는다.
3. Layout placement validator, shared track validator, Root HTML container-style/extent gate가 같은 calc track subset과 node/property-bound math record를 확인한다. Expanded implicit track verifier는 explicit prefix, exact count, 순환된 pattern type/value를 보존한다.
4. Rust matrix는 주 장면, mixed list, calc+fr, scalar, negative range, over-100%, zero basis, existing px/%/fr controls, empty/over-limit/unsupported/non-finite list, CSS syntax-invalid declaration cascade diagnostic과 typed-but-unsupported math tree, wrong/missing/duplicate math ID, wrong node/property, invalid/indefinite/min/max/aspect-ratio/math width, max track quota와 aggregate extent overflow를 확인한다. 거부 입력은 partial track/frame을 발행하지 않고 뒤이은 valid layout이 정상 복구되어야 한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 parsed Root HTML source hash로 기존 C11.1 column-flow fixture-test route를 실행한다. 이 route는 기존 `RuntimeGpuHost::new_with_root_html`을 호출하며 새 fixture ABI/API를 추가하지 않는다. Layout frame, draw/present, screenshot, native surface pixel을 서로 구별해 기록한다. PixelCopy/Metal readback이 없으면 pixel equality를 주장하지 않는다.
6. 변경 crate focused Rust tests, 해당 crate Clippy, `cargo fmt --all -- --check`, `git diff --check`를 실행한다. 기본 앱에는 새 fixture FFI/ABI가 없어야 한다. PR check이 비어 있으면 CI 통과로 기록하지 않는다.
7. 구현/runtime에서 계획 검토와 다른 실패 관점 20개를 검토한다. 현재 구현 후보의 별도 [PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-root-html-grid-column-flow-calc-auto-columns-runtime-2026-11-03/implementation-pr-diff-review-30.md)를 기록했다. 결함 수정 뒤 영향받은 관점과 검증을 반복하고, PR 전 최종 diff에 변경이 생기면 해당 관점을 다시 확인한다.
8. 구현이 병합된 뒤 [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md), [C11 상태 대장](../spec/STATUS.md), 이 계획 및 공개 Tailnet 로드맵을 실제 병합 상태와 맞춘다. C11/C11.1과 공식 완료 `72/213`은 별도 상위 완료 관문 없이는 체크하지 않는다. Agent Browser를 쓴 경우 세션과 daemon을 닫고 프로세스가 없는지 확인한다.

## 범위 밖

이 slice에서 고정한 2항 calc 이외의 AST, `min()`/`max()`/`clamp()`/곱셈·나눗셈, `fr` 계산식, 2항을 넘는 복합 수식, auto/intrinsic/minmax/fit-content sizing, CSS-wide/general author admission, WPT conformance, 다른 root profile, hardware GPU, Android 실기기 또는 iOS compositor 이후 pixel equality는 포함하지 않는다.
