# C11.1 parsed Root HTML Grid explicit `fr`·percentage track + fixed implicit row

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML explicit `fr` track](./c11-root-html-grid-fr-track.md) · [Root HTML percentage track](./c11-root-html-grid-percentage-tracks.md) · [Root HTML fixed implicit row](./c11-root-html-grid-implicit-rows.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**계획 상태:** Chrome 156 기준 네 scene과 독립 계산을 [사전 비교 자료](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/README.md)에 고정했다. 이 계획은 기존 Root HTML 지원인 direct `fr`·percentage explicit tracks와 PR #1024의 단일 fixed-px `grid-auto-rows`를 함께 쓰는 조합만 다룬다. 구현 전 계획 실패 관점 20개 검토는 [계획 검토](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/plan-review-20.md)에 기록한다. 계획 PR을 먼저 병합하고 이 기준을 만족하는 구현 PR을 별도로 제출한다. 이 계획만으로 지원 상태나 공식 완료 집계는 바뀌지 않는다.

**구현 결과:** 계획 PR [#1026](https://github.com/ohah/spinon/pull/1026)은 2026-11-02T03:21:01Z Rebase and merge됐고 merge SHA는 `3f7bfa71ae67a1f61cb8a5154f0cb74137bad9fc`다. 구현 PR [#1027](https://github.com/ohah/spinon/pull/1027)은 2026-11-02T03:48:24Z Rebase and merge됐고 merge SHA는 `732b0257146464de55ddede8a0477726bc5a380b`다. 구현은 direct finite non-negative explicit px/percentage/fr tracks를 기존 fixed implicit-row contract와 연결했다. 구현/runtime 실패 관점 20개와 PR #1027 diff 실패 관점 30개, layout 74개·style projection 12개·Root HTML runtime 20개, scoped Clippy·rustfmt·diff check, Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 경로 확인은 [실행 근거](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-runtime-2026-11-02/README.md)에 기록했다. PR check rollup은 비어 있다. Surface readback과 Tailnet 동기화는 이 문서 갱신 뒤 별도로 확인한다. 공식 `72/213` 및 C11.1/C11 체크 상태는 바꾸지 않는다.

## 문제와 목표

[Root HTML fixed implicit-row slice](./c11-root-html-grid-implicit-rows.md)는 no-area 기본 Root HTML Grid에서 단일 finite non-negative px `grid-auto-rows`를 허용하지만 explicit tracks를 fixed px로 제한했다. [Root HTML `fr` slice](./c11-root-html-grid-fr-track.md)와 [percentage slice](./c11-root-html-grid-percentage-tracks.md)는 definite axis의 explicit tracks로 각각 finite non-negative `fr`와 percentage를 허용하지만, `grid-auto-rows:auto`만 허용한다. 결과적으로 각자 허용된 track 기능이 같은 유효한 Grid에서 조합되지 않는다.

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 computed `grid-auto-rows`가 한 개의 finite non-negative px 길이일 때, 이름 없는 direct `px`·`fr`·percentage explicit tracks를 둘 중 한 definite root axis에 한정해 허용한다. CSS computed values → typed `GridContainerInput` → Grid layout → Root HTML runtime scene 전체에서 같은 root/document/style/layout revision을 유지한다. Grid algorithm은 기존 Taffy track sizing과 row-sparse auto-placement를 사용하며, 새 sizing algorithm·public API·fixture-only route는 추가하지 않는다.

표준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)의 track sizing, flexible track, percentage sizing, implicit track sizing 및 auto-placement 규칙을 기준으로 한다. Chrome은 independent observation이며 기대값은 고정 root size와 CSS track 함수에서 계산한다.

## 독립 비교 기준

HTML, source hash, Chrome observation, screenshot과 수치 기대값은 [사전 비교 자료](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/README.md)와 [expected JSON](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/expected.json)에 고정했다.

| Scene | Definite root | Explicit tracks | Implicit rows | 핵심 item frame (root-relative) |
|---|---:|---|---|---|
| `fraction-columns` | 120×35px | columns `1fr 2fr` → `[40,80]`; rows `[20]` | `[15]` | `(40,0,10,10)`, `(0,20,10,10)` |
| `percentage-columns` | 120×40px | columns `25% 75%` → `[30,90]`; gap 5px; rows `[20]` | `[15]`, row-gap 5px | second item `(35,0,100,10)`; root right overflow 15px |
| `fraction-rows` | 120×60px | columns `[120]`; rows `1fr 2fr` → `[15,30]` | `[15]` | item y positions `[0,15,45]` |
| `percentage-rows` | 120×80px | columns `[120]`; rows `25% 25%` → `[20,20]` | `[15]` | item y positions `[0,20,40]`; remaining block space 25px |

각 scene에는 empty fixed 10×10px direct block children 세 개가 있어 기본 row placement가 마지막 child를 한 implicit row에 놓는다. Percentage는 definite root content-box axis를 basis로 한다. Row `fr` share는 definite 60px block size에서 fixed 15px implicit row를 제외한 45px를 `1:2`로 나눈다. Percentage row scene의 고정 tracks는 root 끝까지 늘어나지 않는다. Gap은 고정 px subset을 유지하며 percentage-column overflow case는 item frame을 root 경계에서 clamp하지 않아야 한다.

## 입력 범위

- 기본 parsed Root HTML entry와 no-area Grid만 대상으로 한다. `RuntimeRootHtmlTextPaintV1`와 기존 `RootHtmlImplicitRowsV1` placement contract를 유지하고 C11.3.3/C11.4 fixture profile을 사용하지 않는다.
- `grid-auto-flow:row`, `grid-auto-columns:auto`, 단일 finite non-negative `grid-auto-rows:<length-px>`와 visible direct children의 기본 auto placement·`order:0` 조건을 그대로 요구한다.
- `grid-template-columns`와 `grid-template-rows`는 비어 있지 않은 이름 없는 direct finite non-negative `px`·`fr`·percentage tracks만 허용한다. `%`와 `fr`은 해당 axis의 computed root content-box 크기가 definite CSS-pixel 값일 때만 허용한다. 계산 불가·비유한·음수 values는 reject한다.
- Root width/height는 fixed definite CSS px이고, 기존 허용 범위의 box sizing, zero margin/padding/border, fixed non-negative px gap, static positioning, visible overflow 및 min/max/aspect-ratio 조건을 유지한다.
- Children은 HTML namespace의 empty fixed-size direct block이며 natural/intrinsic sizing 없이 배치한다. `display:none` children은 placement occupancy에서 제외한다. CSSOM item order와 document source order의 기존 관계를 보존한다.

## 계속 거부할 입력

Named lines, named areas, authored numeric/named placement, auto-repeat, `minmax()`/`fit-content()`/`auto` explicit breadth, implicit `grid-auto-rows` list·percentage·`fr`·intrinsic sizing, non-auto `grid-auto-columns`, dense/column flow, explicit/implicit columns, nonzero `order`, nested Grid, direct text/image/intrinsic contribution, nonzero box edges, non-visible overflow, indefinite percentage/`fr` axis, min/max/aspect-ratio override, stale revisions와 resource-limit 초과는 계속 거부한다. Syntax-invalid declaration이 Stylo cascade에서 떨어지는 경우와 syntax-valid하지만 이 profile 밖인 computed value 거부를 구분한다.

지원하지 않는 조합은 scene을 publish하기 전에 구체 property·node 진단으로 거부한다. 실패한 layout, stale revision, allocation/resource cap은 partial frame이나 이전 revision의 cached scene을 반환하지 않는다. 10,000-track ceiling은 placement/track expansion 전에 적용한다.

## 구현 및 검증 관문

1. Projection에서 `grid-auto-rows`가 fixed px일 때의 기존 explicit-track px-only 조건을 넓히되 direct `fr`·percentage 외에 intrinsic sizing이나 line metadata를 허용하지 않는다. `ComputedCssGridTrackSize`와 `GridTrackSize`에서 finite/non-negative 검사 및 same-source typed conversion을 확인한다.
2. Layout `RootHtmlImplicitRowsV1` contract가 direct px·fraction·percentage tracks만 받도록 조정한다. `validate_tracks`의 다른 `TrackSizing`/template profile 허용 규칙은 변경하지 않고, percent/fraction definite-axis validation과 row-space subtraction이 C11.4 algorithm에서 유지되는지 확인한다.
3. focused style-to-layout projection tests에서 양 축의 px·fr·percentage 혼합, finite boundaries, fixed implicit row preservation 및 named-area 거부를 검사한다. Layout tests에서 exact used tracks, row-sparse placement, root/item frames와 failure atomicity를 검사한다.
4. Root HTML integration matrix는 네 oracle scene과 reordered/renamed controls를 기본 HTML entry에서 실행한다. ID/class rename은 각 source-order item의 geometry를 바꾸지 않고, source reorder는 CSS auto-placement에 따라 frame ownership을 바꿔야 한다. 모든 case에서 document/style/environment/layout revision, computed track values, typed tracks, implicit row, exact root-relative item frames와 draw/present를 검사한다.
5. Positive overflow scene의 second item은 x=35px에서 시작해 root right edge x=120px보다 15px 바깥으로 끝난다. Runtime frame으로 clamp 부재를 확인하고 가능한 경우 Android PixelCopy 및 iOS Metal drawable에서 root 밖 item interior sample을 별도 검사한다. Android renderer와 iOS readback 시점은 명시하고 screenshot만으로 pixel equality를 주장하지 않는다.
6. Negative matrix는 indefinite `%`/`fr` axis, negative/NaN/infinite breadth, intrinsic/minmax/auto-repeat/named lines/areas, multiple/percentage implicit row list, wrong auto-flow/auto-columns, authored placement/order, nested/mixed/intrinsic item, box edges/overflow, stale revision, over-limit rows와 partial-output absence를 확인한다. Invalid declaration fallback controls는 valid-but-unsupported value rejection과 구분한다.
7. 기존 Root HTML px/fr/percentage/gap/area/anonymous text/implicit row suites와 C11.3.3 regression을 실행한다. Fixture-only FFI/API/feature는 추가하지 않고 default/product archive에 신규 fixture marker나 payload가 없음을 확인한다.
8. `cargo fmt --all -- --check`, focused Rust integration suites, 관련 crate Clippy와 Android API 37 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 한다. 실행하지 않은 검사·플랫폼은 통과로 표시하지 않는다. GitHub PR checks가 없으면 CI 통과로 기록하지 않는다.
9. 구현/runtime 변경은 계획과 다른 실패 관점 20개로 검토한다. 구현 PR diff는 프로젝트 규칙에 따라 별도 30개 관점으로 검토하고, 변경한 결함 관점을 다시 확인한다. 계획 PR에는 이미 별도 계획 검토 20개 및 PR diff 검토 30개를 수행한다.
10. 구현 병합 뒤 내부 계약 0059, `spec/STATUS.md`, C11 plan, public roadmap 및 Tailnet preview를 실제 지원 범위만 반영해 동기화한다. C11/C11.1/C11.3.3/C11.4 checkbox와 공식 `72/213`은 그대로 둔다.

## 완료로 주장하지 않을 사항

이 slice는 일반 CSS Grid conformance나 arbitrary explicit/implicit track grammar, intrinsic sizing, named area/line + implicit row 조합, dense/column placement, text/image item contribution, WPT runner, physical Android/hardware GPU, post-compositor iOS pixels, 또는 C11.1/C11.3.3/C11.4/C11 전체 완료를 주장하지 않는다.

## 실패 관점 검토

- 계획 검토 20개: [plan review](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/plan-review-20.md).
- 이 계획 PR의 변경 diff 30관점 검토: [plan PR diff review](../spec/internal/evidence/c11-root-html-grid-fr-percent-implicit-rows-precomparison-2026-11-02/plan-pr-diff-review-30.md).
