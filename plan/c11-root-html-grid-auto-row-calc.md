# C11.1 parsed Root HTML `grid-auto-rows: calc(px + %)`

**상태:** 계획 PR [#1101](https://github.com/ohah/spinon/pull/1101)은 `2026-11-03T02:27:46Z` Rebase and merge됐다. 제한 구현 후보는 focused Rust 검증, scoped Clippy, Android API 37 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행과 구현/runtime 20개 및 PR diff 30개 검토를 마쳤다. 구현 PR·병합 및 그 뒤 문서/로드맵 동기화는 남아 있으며 이 slice는 C11.1/C11 상위 완료와 공식 `72/213`을 바꾸지 않는다.

**상위와 인접 작업:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML implicit row list](./c11-root-html-grid-auto-row-list.md) · [definite percentage row list](./c11-root-html-grid-auto-row-percentage-list.md) · [Root HTML `grid-auto-columns` calc](./c11-root-html-grid-column-flow-calc-auto-columns.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 기준:** [Chrome 156 관측·독립 기대값·고정 fixture](../spec/internal/evidence/c11-root-html-grid-auto-row-calc-precomparison-2026-11-03/README.md) · [계획 실패 관점 20개](./c11-root-html-grid-auto-row-calc-plan-review-20.md) · [계획 PR diff 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-auto-row-calc-precomparison-2026-11-03/plan-pr-diff-review-20.md)

## 문제와 목표

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` route는 `grid-auto-rows`의 직접 px, percentage, fraction scalar/list와 목록 순환을 지원한다. `grid-auto-columns`에는 `calc(px + %)`가 제한적으로 연결됐지만 row track projection, `RootHtmlImplicitRowsV1` validator 및 implicit-row aggregate extent 검증에는 같은 계산식 경로가 없다. 이 계획은 기존 row-list 동작을 유지하며 definite block-size가 있는 단일 Root HTML Grid에서 두 항의 `px + percentage` 합을 implicit row track으로 연결한다.

## 규범 기준과 고정 비교

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.2.1, 7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에 따라 row percentage는 grid container inner block size를 기준으로 resolve되고, 여러 `grid-auto-rows` 값은 첫 implicit row부터 순환한다. [CSS Values and Units Level 4, 2024-03-12 WD §10.9](https://www.w3.org/TR/2024/WD-css-values-4-20240312/)의 math range 처리를 적용한다. Chrome 156은 관측자이며 독립 판정은 고정 입력 산술과 명세다.

고정 fixture의 main scene은 content size `50×100px`, explicit rows `[10,20]px`, row-gap `3px`, 여덟 개의 `10×10px` 빈 direct child, `grid-auto-rows: calc(10% + 2px) 25% 15px`를 사용한다. 두 explicit row 뒤에 implicit track 여섯 개가 생기며 예상 used rows는 `[10,20,12,25,15,12,25,15]px`, row starts는 `[0,13,36,51,79,97,112,140]px`다.

| 경계 | 독립 기대값 | 실패 시 의미 |
|---|---:|---|
| 음수 결과 | `100px` basis에서 `calc(1px - 10%)` → `0px` | non-negative track range 위반 또는 부호 손실 |
| 100% 초과 | `calc(120% + 1px)` → `121px` | 잘못된 clamp 또는 overflow 소실 |
| definite zero | `0px` basis에서 `calc(10% + 5px)` → `5px` | zero basis를 indefinite로 처리 |
| gutter 분리 | `15px` track + `10px` row-gap → 다음 row start `25px` | gap을 percentage basis에 섞음 |
| 순환 | `[12,25,15]`가 첫 implicit row에서 반복 | explicit prefix 포함·순서 바꿈·주기 오류 |
| flexible neighbor | `calc(10% + 5px) 1fr` | fixed calc와 기존 fraction implicit row의 remaining-space 분배 회귀 |

Chrome 156 관측과 독립 기대값, tolerances, 제한은 precomparison evidence에 저장한다. Browser CSS serialization은 normative expected values를 대신하지 않는다.

## 지원 범위와 fail-closed 경계

- 기본 parsed Root HTML route의 단일 non-nested, visible-overflow, sparse row-flow Grid에만 적용한다. 새 profile, 일반 CSS admission, FFI/API 또는 fixture 전용 제품 route를 추가하지 않는다.
- 기존 auto-row scalar/list의 direct finite non-negative px, percentage, fraction 지원을 그대로 유지한다. 새 calc 값은 non-empty `grid-auto-rows` list 안의 typed `Calc(Sum(...))` 중 유한 signed px 항 하나와 유한 signed percentage 항 하나를 가진 두 항 식만 통과시킨다. 항 순서는 보존하고 단항 `Negate`는 각 직접 항에만 허용한다.
- computed style이 length-only math를 direct `LengthPx`로 normalize하면 기존 scalar/list 경로를 유지한다. 이 새 경계는 typed calc tree로 남는 length-percentage 수식에만 적용한다.
- calc와 별도 track인 기존 direct `fr` entry의 혼합 목록도 계속 지원한다. 이 행렬은 calc fixed track과 `fr` implicit track의 remaining-space 분배를 검증한다. `fr`은 calc expression의 항이 아니다.
- calc percentage가 있는 Grid는 finite non-negative direct CSS-pixel `height`가 definite여야 한다. definite `0px`는 유효하다. math-backed height, `min-height`, `max-height`, `aspect-ratio`로 basis가 달라질 수 있는 경우는 거부한다. 기존 px/%/fr list의 admission은 이번 변경으로 좁히지 않는다.
- projection과 layout은 같은 Grid node 및 `GridTemplateRows` property에 묶인 math record를 확인해야 한다. 누락·중복 ID, 다른 owner/property, 잘못된 track이나 math tree, 비유한 항·basis·resolved value, 개별 또는 누적 extent overflow는 layout/frame publication 전에 거부한다. 기존 CSS layout math 수치 ceiling `33,554,428 CSS px`와 `GRID_MAX_TRACKS` cap을 유지한다.
- negative calc result는 row track의 non-negative range로 clamp해 `0px` used track이 된다. `100%` 초과 결과는 container 아래로 overflow할 수 있으며 보존한다. Row-gap은 percentage basis와 별도로 더한다.
- explicit rows, item placement, hidden-item occupancy, current gap subset, source/revision identity와 failure atomicity는 그대로 유지한다. `grid-template-areas`, implicit columns, column/dense flow, authored placement, nested Grid, intrinsic item sizing, auto-repeat, `minmax()`, `fit-content()`, `fr`를 포함한 calc, `min()`/`max()`/`clamp()`, 곱셈·나눗셈, 2항 초과 수식은 이 slice에서 받지 않는다.
- CSS syntax-invalid 선언 cascade 진단과 syntax-valid but unsupported computed math는 별도 실패 경로로 확인한다. 실패 또는 quota/할당 오류 뒤 partial frame/track/draw를 내지 않고 후속 valid 요청이 성공해야 한다.

## 구현 및 검증 관문

1. 계획 PR에는 구현 전 fixture, Chrome observation, 독립 기대값, 이 계획의 서로 다른 20개 실패 관점 검토와 별도의 계획 PR diff 20관점 검토를 포함한다. 해당 PR에는 사용자의 목표에 따라 20개의 서로 다른 변경 실패 관점을 별도 기록으로 둔다.
2. Style-to-layout은 Root HTML profile의 `grid-auto-rows`에만 허용 calc tree를 전달하고, 다른 profile의 기존 경계는 유지한다. Projected math ID는 `GridTemplateRows`에 결속한다. `root_html_has_supported_implicit_rows`와 area/alignment gate의 목록 분류도 같은 제한식을 인식하도록 한다.
3. Layout core는 `RootHtmlImplicitRowsV1`에서 calc-bearing row list를 허용하되 direct row cases의 표현을 바꾸지 않는다. Definite height, node/property-bound math, track count, each expanded row, row gap과 누적 f32 extent를 검증한 뒤에만 Taffy가 frame을 만들게 한다. Track-list cycle은 explicit row 이후 첫 implicit track에서 시작한다.
4. Rust matrix는 주 목록, mixed px/%/calc, calc+fr remaining-space distribution, scalar calc track list, existing px/%/fr controls, negative clamp, 100% 초과 overflow, zero basis, row-gap independence, list cycling, unsupported math AST, missing/duplicate/wrong ID, wrong owner/property, invalid/non-finite values, height/min/max/aspect-ratio/math-backed basis, empty/oversized list, track/aggregate extent overflow, syntax-invalid cascade와 valid-but-unsupported input을 분리해서 다룬다. Reject path는 partial layout을 발행하지 않고 후속 valid request 복구를 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 parsed Root HTML source hash로 기존 default Root HTML runtime route를 확인한다. Build/source/input hash, computed style, used row sizes, node frames, layout-ready, draw/present와 화면 캡처를 따로 저장한다. PixelCopy 또는 Metal drawable readback을 실제 수집하지 않으면 surface pixel equality를 주장하지 않는다.
6. 영향받은 Rust focused suites, affected crate Clippy, `cargo fmt --all -- --check`, diff checks 및 Android/iOS simulator builds를 실행한다. 실행하지 않은 test, CI 또는 platform 결과는 통과로 기록하지 않는다. 기본 artifact에서 새로운 fixture FFI/ABI가 생기지 않았음을 확인한다.
7. 구현/runtime은 계획표와 다른 20개 실패 관점으로 review하고, 구현 PR 변경분은 다시 독립된 20관점으로 검토한다. 결함을 발견하면 수정하고 영향받은 관점을 재검증한다.
8. 구현 PR 병합 뒤 계약 0059 `0.1.0`, C11 plan, `spec/STATUS.md`, 공개 roadmap과 승인된 Tailnet preview를 실제 merge 상태에 맞춘다. 상위 항목의 완료 조건을 모두 충족하기 전에는 C11.1/C11 checkbox와 공식 완료 수를 승격하지 않는다. Agent Browser를 사용하면 세션과 해당 Chrome 프로세스를 종료하고 종료 상태를 확인한다.

## 제외 범위

일반 CSS Grid grammar/admission, 모든 CSS math tree, intrinsic/indefinite/cyclic sizing, WPT conformance, named areas/lines, row `minmax()`·`fit-content()`·auto-repeat, nested/content-driven Grid, Android 실기기·hardware GPU, iOS compositor 이후 pixel equality와 C11 상위 완료는 이 계획에 포함하지 않는다.
