# C11.1 parsed Root HTML Grid 제한 `grid-auto-rows` implicit row

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [percentage tracks](./c11-root-html-grid-percentage-tracks.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**계획 검토:** [구현 전 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-implicit-rows-precomparison-2026-11-02/plan-review-20.md)

**구현/runtime 검토:** [실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/implementation-runtime-review-20.md)

**구현 PR diff 검토:** [별도 실패 관점 30개](../spec/internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/pr-diff-review-30.md)

**현재 상태:** 단일 fixed-px implicit row의 제한 구현을 기본 Root HTML runtime 경로에 연결한 구현 PR [#1024](https://github.com/ohah/spinon/pull/1024)은 2026-11-02T03:01:28Z에 Rebase and merge됐고 merge SHA는 `90723452724656a985c91428eaabc75976d8ad45`다. Rust Root HTML/layout 회귀, 자원 상한, 관련 C11.3.3 검사, Clippy와 rustfmt 및 Android API 37 AVD/iOS 26.2 Simulator 실행을 마쳤다. 별도의 구현/runtime 실패 관점 20개 및 구현 PR diff 30개 검토도 통과했다. [실행 근거](../spec/internal/evidence/c11-root-html-grid-implicit-rows-runtime-2026-11-02/README.md)에 exact frame matrix, source/build hash, platform log, screenshots와 한계를 기록한다. PR check rollup은 비어 있어 CI 통과로 주장하지 않는다. 이 제한 구현은 C11.1/C11 또는 공식 완료 `72/213`을 승격하지 않는다.

## 목표와 규범 근거

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 `grid-auto-flow:row`로 explicit row 바깥에 배치된 Grid item을 기존 Grid layout 경로가 수용하게 한다. 이번 제한 profile은 computed `grid-auto-rows`가 단일 finite non-negative CSS-pixel track일 때만 허용한다. `grid-auto-columns`는 초기값 `auto`, auto-flow는 기본 row만 허용한다. Root HTML projection은 기존 `GridPlacementMode::Existing`을 사용해 non-auto implicit row를 다시 거부하므로, layout core에 root 전용 `RootHtmlImplicitRowsV1` mode를 추가한다. 이 mode는 기존 sparse row placement 알고리즘을 재사용하되 모든 visible direct item의 placement와 `order`가 기본값이고, item은 빈 fixed-size block이어야 한다. named-area/placement fixture mode를 재사용하지 않아 C11.3.3 허용 범위가 Root HTML 경로로 새지 않게 한다.

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.5–7.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-grids)은 explicit grid 바깥에 놓인 item이 implicit track을 만들며 `grid-auto-rows`가 그 크기를 지정한다고 규정한다. §7.6은 복수 auto-track size의 반복 규칙을 정의하지만 이번 profile은 단일 고정 px 값으로 제한한다. §7.7의 초기 `grid-auto-flow:row`는 row를 차례로 채우며 필요한 row를 추가한다. §8.5의 placement algorithm은 item의 DOM order와 점유 cell을 기준으로 배치를 수행한다. 해당 W3C snapshot은 Candidate Recommendation Draft이며 작업 중 명세로 인용한다.

## 고정 fixture와 독립 기대값

Chrome 기준과 fixture hash는 [사전 비교 근거](../spec/internal/evidence/c11-root-html-grid-implicit-rows-precomparison-2026-11-02/README.md)와 [expected JSON](../spec/internal/evidence/c11-root-html-grid-implicit-rows-precomparison-2026-11-02/chrome156-expected.json)에 기록한다. fixture는 40px/55px explicit columns, 20px explicit row, 5px gaps와 세 개 또는 여섯 개의 empty direct block item을 사용한다. Item은 자기 cell 크기와 같은 fixed `px` width/height를 가져 intrinsic sizing을 배제한다. `grid-auto-rows`는 15px positive case, 0px boundary와 단일 15px size의 반복 사용을 비교한다.

- 세 item은 explicit row 뒤에 implicit row 하나를 만든다. Root는 `100×40px`, used rows는 `[20,15]px`이고 세 번째 item은 local frame `(0,25,40,15)`다.
- 0px control도 implicit row와 placement를 보존한다. Root는 `100×25px`, used rows는 `[20,0]px`, 세 번째 item은 `(0,25,40,0)`이다.
- 여섯 item은 implicit row 두 개를 만든다. Root는 `100×60px`, used rows는 `[20,15,15]px`다. 마지막 두 item은 각각 `(0,45,40,15)`와 `(45,45,55,15)`다.
- item position은 DOM source order와 two-column row flow에서 산출한다. HostNode ID, class 이름 또는 scene ID가 성공 경로를 선택하지 않는다.

## 제한 scope

- 기본 parsed Root HTML runtime entry만 확장한다. C11.4 opt-in constructor, fixture scene registry, ID 기반 adapter dispatch 또는 새 public CSS coverage claim은 추가하지 않는다.
- `grid-auto-flow:row`와 `grid-auto-columns:auto`를 유지한다. `grid-auto-rows`는 단일 finite non-negative `LengthPx` computed track 하나만 받는다. 기존 projection/layout이 생성된 implicit row track을 Grid item count에 따라 반복하게 한다.
- 이 mode에서 explicit columns/rows는 이름 없는 finite non-negative direct px track만 받으며, explicit auto-repeat과 named lines는 거부한다. gap은 기존 finite non-negative px 범위만 받는다. 기존 Root HTML의 `fr`/percentage explicit track 지원은 `grid-auto-rows:auto` 경로에서 유지하고 이 implicit-row mode와의 조합은 후속으로 남긴다. 이번 positive oracle는 columns `[40px,55px]`, row `[20px]`, 5px gaps다.
- Grid는 HTML namespace container이며 visible in-flow 직접 자식만 auto-placement에 참여한다. visible item은 빈 block이고 fixed `px` width/height를 가진다. `display:none` 자식은 CSS처럼 placement에서 빠진다. nonzero `order`, margins, padding, borders, nested Grid, text/image intrinsic contribution, named areas, authored placement 및 non-default alignment/overflow는 이 slice에서 계속 거부한다.
- 생성되는 implicit row 수와 전체 track 수는 기존 Grid track/resource cap을 넘지 않아야 한다. 초과 입력은 layout 전에 property/node 진단으로 거부하고 partial frame을 publish하지 않는다.

## 계속 거부할 입력

`grid-auto-flow:column|dense`, non-auto `grid-auto-columns`, 복수 `grid-auto-rows` list, percentage/fr/minmax/fit-content/auto-repeat implicit sizing, explicit fr/percentage tracks와 non-auto `grid-auto-rows`의 조합, named areas와 implicit sizing의 결합, implicit columns, explicit placement, nested/mixed content, non-static positioning, non-visible overflow/clipping, unsupported visibility, stale source/style revision 및 resource limit 초과는 이 slice에서 지원하지 않는다. CSS syntax-invalid 선언이 cascade에서 떨어지는 경우와 valid-but-unsupported computed value가 layout adapter에 거부되는 경우를 구분한다.

## 구현 수락과 검증 관문

1. Cascade 계약은 변경하지 않는다. 현재 `RuntimeRootHtmlTextPaintV1` 경로는 단일 finite non-negative px breadth를 이미 `ComputedCssGridAutoRows::LengthPx`로 만든다. `grid_values.rs` projection만 해당 값을 좁게 수용하고, Root HTML named-area template과 함께 쓰는 경우는 계속 거부한다. no-area 기본 row-flow 경로에서만 `RootHtmlImplicitRowsV1` placement mode를 고른다. Root mode는 `grid-auto-flow:row`, `grid-auto-columns:auto`, named-line-free fixed-px explicit tracks와 fixed-px gaps, 모든 visible direct item의 auto placement와 `order:0`, fixed-size empty block item, 단일 non-negative px implicit row를 검사하고 GRID_MAX_TRACKS 한도를 배치 전에 유지한다. `display:none` direct child는 placement occupancy에서 제외한다. `spinon-layout/src/grid_placement.rs`는 Root mode에서 기존 sparse normalizer만 선택하며 `grid.rs`의 mode별 admission, item shape, implicit track 요구 조건을 같은 mode로 묶어 검사한다. 여러 auto-row track, intrinsic/list 값 및 explicit fr/percentage track 조합은 cascade 또는 projection에서 기존처럼 거부한다. Revision 및 failure atomicity 경계는 유지한다.
2. Root HTML Rust integration tests에서 one-row/zero-row/two-row cases, default-vs-explicit `row` flow, ID/class rename과 item DOM order 변화, exact rows/columns/root/item frames를 Chrome geometry와 CSS 0.02px 이내로 대조한다. Taffy final used tracks가 CSS Grid expected sequence와 같은지 확인한다.
3. Negative controls는 `column|dense`, non-auto `grid-auto-columns`, 복수 auto-row list, implicit percentage/fr/minmax, fixed implicit row와 조합한 explicit fr/percentage tracks, named-area interaction, numeric/named authored placement, nonzero `order`, extra implicit column, text/nested/otherwise unsupported child shape, over-limit count, stale revision, layout error와 partial-output absence를 검사한다. 별도 positive control은 `display:none` direct child가 occupancy나 row count에 포함되지 않음을 확인한다.
4. Existing Root HTML block/text/background/color-scheme/viewport, fixed px/fr/percentage tracks, gap, named area와 anonymous text regressions 및 C11.3.3 placement tests를 분리해 실행한다. Fixture-only FFI symbols/payload를 추가하지 않고 production path에서 fixture ID dependency가 없는지 확인한다.
5. 같은 HTML source를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML entry에서 실행한다. 설치 app/source hashes, root/item frames, tracks, revision, `layout=ready`와 draw/present를 기록하고 screenshot을 PR 본문에 연결한다. PixelCopy/Metal readback을 할 경우 해당 표면만 확인했다고 구분하며 screenshot을 pixel proof로 부르지 않는다.
6. `cargo fmt --all -- --check`, focused Rust/runtime tests, 관련 crate Clippy, Android/iOS simulator builds를 실행한다. 미실행 gate는 통과로 보고하지 않는다. GitHub status check이 비어 있으면 CI 통과로 표시하지 않는다.
7. 구현/runtime 변경을 계획과 독립된 실패 관점 20개로 검토하고, plan PR diff와 구현 PR diff도 각각 별도 20개 관점으로 검토한다. 실패를 수정하면 해당 관점을 재검증한다.
8. 구현 PR 병합 뒤 내부 계약 0059, `spec/STATUS.md`, C11 plan과 공개 roadmap을 제한 범위만 반영한다. C11.1/C11/C11.4 checkbox 및 공식 완료 `72/213`은 바꾸지 않는다.

## 완료로 주장하지 않을 사항

이 slice는 arbitrary auto-track list, implicit columns, auto-placement의 모든 형태, `grid-auto-flow:column|dense`, named areas와 implicit track sizing의 모든 결합, intrinsic text/image sizing, nested Grid, WPT runner 또는 일반 CSS Grid support를 완료로 표시하지 않는다. Root HTML base route에 이 제한을 연결해도 C11.1/C11.3.3/C11.4 또는 C11 전체가 완료되는 것은 아니다.
