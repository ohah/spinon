# C11.1 parsed Root HTML `grid-auto-rows` fractional track list 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [fixed-px implicit row list](./c11-root-html-grid-auto-row-list.md) · [percentage implicit row list](./c11-root-html-grid-auto-row-percentage-list.md) · [explicit fractional tracks](./c11-root-html-grid-fr-track.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**계획 상태:** Chrome 156 사전 비교 7 scene과 독립 계산은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03/README.md)에 고정했고, 계획 PR [#1069](https://github.com/ohah/spinon/pull/1069)은 병합됐다. `grid-auto-rows` direct px/%/fr implicit-row 목록 구현 후보는 cascade·projection·layout·runtime 집중 Rust 테스트와 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 layout-ready/draw-present까지 확인했다. [실행 evidence](../spec/internal/evidence/c11-root-html-grid-auto-row-fr-simulators-2026-11-03/README.md)에 범위, 해시, 캡처와 한계를 기록한다. 넓은 style-to-layout Root HTML 회귀는 기존 auto-column invalid-fraction 검사 한 건이 실패해 green이 아니다. 구현/runtime 실패 관점 20개와 구현 PR diff 실패 관점 30개 검토는 완료했다. 구현 PR [#1070](https://github.com/ohah/spinon/pull/1070)은 2026-11-02T16:37:30Z Rebase and merge됐고 merge SHA는 `f40cb3b93d2555dafecd352bd35fb349b6bf7a64`다. PR check rollup과 main required status checks는 비어 있다. Tailnet roadmap sync는 이어서 반영한다. C11.1/C11 완료와 공식 `72/213`은 바꾸지 않는다.

## 문제와 목표

기본 Root HTML Grid는 `grid-auto-rows`의 direct px scalar 및 px/% 목록을 순환할 수 있다. CSS typed values와 Grid 입력은 direct fraction 값을 이미 표현하고 explicit Root HTML rows도 definite height에서 `fr`을 허용하지만, projection과 `RootHtmlImplicitRowsV1`은 implicit row list에서 `fr`을 거부한다. 따라서 explicit fraction sizing과 sparse row auto-placement를 각각 지원하는 경로를 함께 쓸 수 없다.

기존 `RuntimeSession::new_runtime_gpu_with_root_html()` 기본 parsed HTML 경로에서 `grid-auto-rows` direct finite non-negative px/%/fr 목록을 허용한다. 첫 implicit row부터 목록 순환, definite root block size 기준 percentage, definite grid block size에서의 fractional free-space allocation, CSS Grid item automatic minimum, visible overflow를 기존 Taffy Grid 계산에 연결한다. 새 sizing algorithm, public API, fixture-only route, 일반 CSS admission은 추가하지 않는다.

규범 기준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 flexible track sizing 알고리즘이다. Chrome 156은 독립 observation oracle이며, 정답은 고정 CSS geometry로 계산해 브라우저 관측 전에 fixture와 함께 기록한다.

## 범위

- `RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML route의 no-area Grid만 대상으로 한다. 기존 `RootHtmlImplicitRowsV1`, row-sparse auto-placement, `grid-auto-flow: row`, `grid-auto-columns: auto`, direct fixed-size visible HTML block children 및 `order: 0` 제한을 유지한다.
- 기존 단일 fixed-px `GridAutoRows::LengthPx` 표현은 바꾸지 않는다. computed non-empty `TrackList`에 direct finite non-negative `LengthPx`, `Percentage`, `Fraction`을 허용하고, 값의 종류와 authored 순서를 보존해 첫 implicit row부터 반복한다. 목록·expanded row 수는 기존 `GRID_MAX_TRACKS` 한도를 넘지 않는다.
- `%` 또는 `fr` 값이 있는 목록은 finite non-negative definite root height를 요구한다. `min-height`·`max-height`는 auto, `aspect-ratio`는 미설정이어야 한다. Math-backed/indefinite height는 거부한다. definite `0px`는 유효하며 그에 따른 used flex track은 0px이 될 수 있다.
- Percentage는 definite root content-box height를 기준으로 하며 row-gap을 basis에서 빼지 않는다. Fixed explicit/implicit track과 gutter를 제외한 flexible free space를 factor에 따라 `fr` track에 분배한다. `0fr`, 합계가 1보다 작은 factor와 fixed-size item minimum을 CSS Grid 알고리즘에 따라 보존한다. `fr`/percentage가 container 또는 item을 root 밖으로 밀어도 frame을 clamp하지 않는다.
- Fixed px gap, source order, display-none child 제외, explicit rows, ID/class 이름 비의존성, revision ownership 및 기존 실패 원자성/resource cap을 유지한다. Syntax-invalid CSS cascade fallback과 syntax-valid하지만 definite-axis 조건을 만족하지 않는 computed value 거부를 구분한다.

## 사전 수치 oracle

입력 SHA-256, independent expected values, Chrome computed value/used rows/DOMRect, user agent, DPR 및 screenshot은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03/README.md)에 고정했다. Geometry 허용 오차는 0.02 CSS px다.

| Scene | Root height | `grid-template-rows` | `grid-auto-rows` | Used rows (px) | Item row starts (px) | 확인점 |
|---|---:|---:|---|---|---|---|
| `cycle` | 100 | `20px` | `1fr 2fr` | `[20,10,20,10,20]` | `[0,25,40,65,80]` | 네 implicit rows에서 factor 목록 순환, 전체 definite space 사용 |
| `mixed` | 120 | `20px` | `15px 25% 1fr` | `[20,15,30,20,15]` | `[0,25,45,80,105]` | px/%를 먼저 계산하고 남은 20px을 `1fr`에 배분 |
| `overflow` | 100 | `20px` | `120% 1fr` | `[20,120,0]` | `[0,25,150]` | 120% clamp 없음, 남은 flex size 0, root 아래 overflow 보존 |
| `zero` | 0 | `0px` | `1fr` | `[0,0]` | `[0,0]` | definite zero basis |
| `zero-factor` | 40 | `10px` | `0fr 1fr` | `[10,10,5,0]` | `[0,15,30,40]` | `0fr` automatic minimum과 cyclic list |
| `subunit` | 100 | `20px` | `0.25fr` | `[20,17.5,17.5]` | `[0,25,47.5]` | subunit flex factor가 남는 공간을 보존 |
| `minimum` | 30 | `20px` | `1fr` | `[20,40]` | `[0,25]` | 40px fixed item automatic minimum이 root를 넘어 확장 |

모든 장면에서 item은 fixed-size empty block이며 template columns, row/column gap, source order 및 item box도 fixture에 고정했다. Raw observation은 expected JSON과 따로 보관했다.

## 계속 거부할 입력

`auto`, `minmax()`, `fit-content()`, intrinsic track breadth, `calc()`/math-backed track, auto-repeat, named line/area와 implicit row 결합, implicit columns, `grid-auto-flow: column|dense`, authored line placement/span, nonzero `order`, nested Grid, anonymous text/image/intrinsic item, percentage gap, indefinite `%`/`fr` basis, min/max-height/aspect-ratio override, nonzero box edges, clipping/scroll overflow, stale revision 및 resource quota 초과는 범위 밖이다. 기존 Root HTML profile에 없는 입력을 초기값이나 부분 layout으로 대체하지 않는다.

## 구현 및 검증 관문

1. 계획 PR은 [precomparison fixture와 고정 expected values](../spec/internal/evidence/c11-root-html-grid-auto-row-fr-precomparison-2026-11-03/expected.json)를 포함하고 계획 실패 관점 20개와 계획 PR 실제 diff의 별도 30관점 검토를 기록한다.
2. Projection에서 Root HTML direct auto-row validator와 `project_auto_rows()`를 갱신한다. px 단일 scalar 의미를 보존하고, direct finite non-negative px/%/fr 목록만 통과시킨다. Named-area/template-area profile과 C11.4 profile은 바꾸지 않는다.
3. `spinon-layout`의 `RootHtmlImplicitRowsV1` validator·container-style definite-height check·expanded row validation을 갱신한다. Percentage resolved extent, fractional factors, gaps 및 track count를 검사한 뒤 placement/layout에 넘긴다. Invalid factor와 f32 extent overflow는 scene/frame publication 전에 node/property가 있는 오류로 반환한다.
4. Layout/projection tests에서 Chrome 7 scene의 exact used rows/item frames, cyclic pattern, first implicit index, px/%/fr composition, definite-zero, zero/subunit factor, fixed-item automatic minimum, overflow, invalid/non-finite/negative factor, indefinite/min/max/aspect-ratio axis, named-area/column-flow 거부, resource ceiling 및 partial-output 부재를 검사한다. 기존 px/% lists와 explicit fr tracks의 회귀도 확인한다.
5. Default Root HTML runtime integration은 committed 단일 입력 fixture로 same-source cascade → typed projection → Grid layout → draw/present를 확인한다. Revision, computed tracks, projected auto rows, used track frames, node ownership, source reorder 및 frame/root overflow를 별도 검증한다. Precomparison의 여러 root fixture를 제품 runtime 입력으로 재사용하지 않는다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 app route에서 같은 committed fixture/hash를 실행한다. Draw/present와 screenshot은 visual smoke evidence다. PixelCopy/Metal surface readback은 실제 취득한 경우에만 분리 보고하고, 실기기·hardware GPU·post-compositor equality로 확대 해석하지 않는다.
7. 변경 경로의 focused cascade, style-to-layout projection, layout Grid, Root HTML runtime suites, 관련 crate scoped Clippy, `cargo fmt --all -- --check` 및 diff check를 실행한다. 미실행 gate와 GitHub check 부재를 성공으로 적지 않는다. Default artifact와 opt-in fixture-test artifact 경계를 바꾸지 않는다.
8. 구현/runtime failure 관점 20개는 계획 review와 다른 항목으로 기록한다. 구현 PR diff는 프로젝트 규칙에 따라 별도의 30관점으로 검토한다. 결함 수정 후 영향 관점과 검사를 다시 확인한다.
9. Implementation merge 후 contract 0059 `0.1.0`, C11 plan, `spec/STATUS.md`, public roadmap, Tailnet preview 및 browser/process cleanup을 최신 사실과 맞춘다. 제한 slice를 기록하되 C11.1/C11 checkbox와 공식 `72/213`은 바꾸지 않는다.

## 완료로 주장하지 않을 사항

이 제한 slice는 arbitrary CSS Grid support, auto/intrinsic/minmax track sizing, general CSS admission, WPT runner/conformance, named-line/area + implicit track interaction, dense/column flow, intrinsic text/image contribution, physical-device/hardware-GPU validation 또는 C11.1/C11 completion을 주장하지 않는다.
