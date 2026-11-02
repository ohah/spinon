# C11.1 · parsed Root HTML percentage implicit columns

**상태:** 제한 구현 PR [#1063](https://github.com/ohah/spinon/pull/1063)이 `2026-11-02T14:18:53Z` Rebase and merge됐다(merge SHA `246babd793b27c2ca9e8edc5d81b8726c7479816`). definite-width Root HTML sparse column-flow의 direct percentage `grid-auto-columns` scalar/list가 구현됐다. 이는 C11.1/C11 완료나 공식 `72/213` 승격이 아니며, 이 계획의 제외 범위는 그대로 남는다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [기존 sparse column-flow](./c11-root-html-grid-column-flow.md) · [fixed-px implicit columns](./c11-root-html-grid-column-flow-implicit-columns.md) · [definite percentage implicit rows](./c11-root-html-grid-auto-row-percentage-list.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 기준:** [Chrome 156 관측과 독립 계산](../spec/internal/evidence/c11-root-html-grid-column-flow-percent-auto-columns-precomparison-2026-11-02/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-column-flow-percent-auto-columns-precomparison.html)

## 문제와 동작 범위

현재 `RootHtmlColumnFlowV1`은 implicit column을 만들 때 finite non-negative direct `px` scalar/list만 허용한다. 같은 parsed Root HTML column-flow에서 percentage auto-column을 사용하면 typed cascade 뒤에 거부된다. 이 계획은 definite width를 가진 container에서 direct percentage를 `grid-auto-columns` scalar/list의 일부로 해석한다.

- 기본 parsed Root HTML runtime route에서 `grid-auto-flow:column` sparse placement와 기존 fixed-px implicit-column 동작을 유지한다.
- `grid-auto-columns`는 finite non-negative direct `px`와 percentage를 한 비어 있지 않은 scalar/list로 받는다. 목록 순서는 첫 implicit column부터 반복한다. 기존 px 단일 값 표현은 유지한다.
- percentage basis는 Grid container의 definite content-box width다. definite zero width는 유효한 0px basis다. Percentage 계산은 row/column gap을 빼지 않으며 100%를 초과해도 clamp하지 않고 overflow를 보존한다.
- percentage를 포함하면 root width는 finite non-negative direct CSS-pixel `width`여야 한다. `min-width`, `max-width`, `aspect-ratio`, math-backed width가 basis를 바꾸는 경우와 indefinite/percentage width는 거부한다. CSS syntax-invalid 선언의 cascade fallback과 typed-but-unsupported 입력은 구분한다.
- 기존 fixed-px explicit rows/columns, fixed-px gaps, `grid-auto-rows:auto`, direct empty HTML `div` items, item의 fixed px width/height, `direction:ltr`, `writing-mode:horizontal-tb`, 단일 non-nested Grid, sparse flow, hidden-item 제외 및 revision/publication gate를 유지한다.
- C11.1/C11 checkboxes와 공식 완료 수는 바꾸지 않는다. 일반 CSS admission, row/dense flow, implicit rows, authored placement, intrinsic/`fr`/`minmax()`/`fit-content()`/math tracks, nested Grid, WPT 적합성은 이 slice에 포함하지 않는다.

## 규범 기준과 oracle

[CSS Grid Layout Module Level 2, 2025-03-26 CRD §5.2, §7.6, §7.7, §8.5, §12](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/)를 규범 출처로 삼는다. §7.6은 여러 `grid-auto-columns` track size가 필요한 implicit track에 반복 적용됨을 정한다. definite container size가 주어진 뒤 percentage track size를 container 크기에 resolve한다. Chrome 156은 독립 관측자이며 규범 기대값의 출처가 아니다.

고정 fixture의 content width는 240px이고 border/padding이 0이다. explicit columns는 `[30,40]px`, rows는 `[20,40]px`, column/row gap은 5/3px, source-order item은 12개, item size는 10×10px다. 따라서 column-flow는 총 6개 열을 필요로 하고 네 개 implicit column을 만든다.

| Case | `grid-auto-columns` | Expected used columns | Expected column starts | 목적 |
|---|---|---|---|---|
| `cycle` | `10% 15%` | `[30,40,24,36,24,36]px` | `[0,35,80,109,150,179]px` | 240px basis, 두 번의 목록 순환, gap이 basis에서 빠지지 않는지 확인 |
| `scalar` | `10%` | `[30,40,24,24,24,24]px` | `[0,35,80,109,138,167]px` | 단일 percentage track 반복 |
| `mixed` | `10px 15%` | `[30,40,10,36,10,36]px` | `[0,35,80,95,136,151]px` | px/percentage 목록 순서와 순환 |
| `zero` | `0%` | `[30,40,0,0,0,0]px` | `[0,35,80,85,90,95]px` | zero track과 item overflow |
| `over-100` | `120%` | `[30,40,288,288,288,288]px` | `[0,35,80,373,666,959]px` | 100% 초과 track을 clamp하지 않고 보존 |
| `zero-basis` | `50%`, root width `0px` | `[30,40,0,0,0,0]px` | `[0,35,80,85,90,95]px` | definite zero basis |

독립 기대값은 `percentage × 240px`, explicit track width 및 gap의 합으로 계산한다. Browser `getComputedStyle().gridTemplateColumns`, `gridAutoColumns`, root `DOMRect`, 각 semantic item `DOMRect`는 [원본 관측 JSON](../spec/internal/evidence/c11-root-html-grid-column-flow-percent-auto-columns-precomparison-2026-11-02/chrome156-observation.json)에서 독립 기대값과 대조한다. 허용 geometry 오차는 0.02 CSS px다.

## 구현 및 판정

1. `root_html_column_flow.rs` candidate validation은 finite non-negative direct px/percentage `grid-auto-columns`만 허용한다. 10,000-track 상한, 빈 목록과 기타 track variant의 fail-closed 동작을 보존한다.
2. Root HTML percentage-axis validation은 `grid-auto-flow:column`의 percentage auto-column 목록을 인식하고 definite finite non-negative direct px width, min/max/aspect-ratio/math 경계와 각 resolved percentage의 f32 범위를 검사한다. 다른 Grid mode와 CSS properties의 admission은 넓히지 않는다.
3. `GridPlacementMode::RootHtmlColumnFlowV1`의 auto-track expansion은 percentage pattern을 직접 보존해 Taffy로 전달한다. expanded-column verifier는 explicit prefix, exact implicit count 및 반복 track type/value를 확인한다. 다른 placement mode의 `FixedPxList` 제한은 그대로 둔다.
4. Layout tests는 `px`, scalar percentage, 혼합 px/% list, 2회 이상 list cycling, 0%, 120%, definite zero basis, hidden-item/source-order 및 기존 px 회귀를 확인한다. indefinite/min/max/aspect-ratio/math-backed width, negative/non-finite synthetic percentage, resolved f32 overflow, empty/unsupported list, track quota와 aggregate extent overflow는 거부한다.
5. Style projection tests는 cascade typed values, percentage property/node 진단, Root HTML mode 분리를 검사한다. CSS syntax-invalid declaration fallback과 syntax-valid but profile-unsupported input을 별도로 확인한다.
6. Runtime tests는 실제 parsed Root HTML 기본 route에서 same-source fixture hash, style/layout revision identity, computed auto-column list, root/item frame, exact expanded-column trace, `layout=ready`와 draw/present를 확인한다. 실패 입력 또는 allocation fault 이후 partial scene이 공개되지 않고 다음 유효 입력이 회복되는지도 확인한다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 committed fixture를 기본 app route로 실행한다. 플랫폼별 build/runtime hash와 화면을 보존한다. Screenshot은 시각 확인이며 PixelCopy/Metal surface readback, 실기기 GPU, iOS compositor 이후 pixel equality로 확대 해석하지 않는다.
8. 관련 focused Rust suites, scoped Clippy, `cargo fmt --all -- --check`, 기본 제품 기능 build를 실행한다. 기존 변경 외 실패와 GitHub check 부재는 성공으로 보고하지 않는다. opt-in fixture route를 수정했다면 기본 artifact의 fixture ABI/payload exclusion도 확인한다.

## 적대 검토와 후속

- 구현 전 계획의 서로 다른 실패 경로 20개는 [계획 검토](./c11-root-html-grid-column-flow-percent-implicit-columns-percentage-plan-review-20.md)에 둔다. 이 리뷰는 feature implementation/runtime 리뷰로 재사용하지 않는다.
- 구현 후에는 새로운 implementation/runtime 실패 관점 20개를 기록한다. 구현 PR diff는 프로젝트 규칙에 따라 별도 30개 관점으로 검토한다.
- 구현 PR이 병합되면 status ledger, public roadmap, contract 0059 `0.1.0` 및 Tailnet preview를 후속 문서 변경으로 맞춘다. 어느 bounded slice도 C11.1/C11 전체 완료나 공식 `72/213` 변경을 주장하지 않는다.


## 병합 결과와 남은 범위

PR #1063은 fixed-px implicit columns 뒤에 finite non-negative direct percentage 및 px/% 혼합 목록을 Root HTML `RootHtmlColumnFlowV1`에 연결했다. Chrome 156 독립 기대값, layout 15/15, style-to-layout 28/28, style cascade 9/9, runtime 3/3, affected-package Clippy와 rustfmt를 확인했다. 기본 FFI archive에는 task symbol/title이 없고 fixture-test positive control에는 둘 다 있다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator는 parsed Root HTML host route에서 같은 240×63 grid 및 12 item frame을 출력했다. 상세 hash·로그·화면과 simulator viewport 차이는 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-column-flow-percent-auto-columns-runtime-2026-11-02/README.md)에 둔다.

일반 CSS admission, dense/row flow, intrinsic tracks, authored placement, nested Grid, WPT 적합성, 실기기/hardware GPU와 drawable pixel readback은 검증하지 않았다. PR checks는 보고되지 않았고 main required checks도 설정되어 있지 않다. 이 계획 slice는 C11.1/C11 checkbox를 완료하지 않으며 내부 계약 숫자 버전 `0.1.0`과 공식 `72/213`을 유지한다.
