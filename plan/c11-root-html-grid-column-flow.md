# C11.1 · parsed Root HTML column 흐름 자동 배치

**상태:** 구현 후보 코드, 지정 플랫폼 smoke 검증과 구현/runtime 적대 리뷰 20개를 마쳤다. `spinon-layout` 4/4, `spinon-style-to-layout` 3/3, runtime 4/4 및 Root HTML Grid 회귀 37/37이 통과했고, scoped Clippy와 `cargo fmt --all -- --check`도 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 app route에서 동일 fixture asset hash, `layout=ready`, 7-box draw/present를 확인했다. 전체 `spinon-style-to-layout` suite는 기존 C07 aspect-ratio serialization 기대값 불일치 1건으로 305/306이며 green으로 세지 않는다. [구현/runtime 적대 리뷰](../spec/internal/evidence/c11-root-html-grid-column-flow-runtime-2026-11-02/implementation-runtime-review-20.md)와 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-column-flow-runtime-2026-11-02/README.md)를 남겼다. PR diff 30관점 리뷰와 PR/merge gate가 남아 있다. C11.1과 C11은 미완료이고 공식 집계 `72/213`은 유지한다.

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 통합](./c11-root-html-grid-runtime.md) · [상태 대장](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [고정 HTML과 Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-column-flow-precomparison-2026-11-02/README.md) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-column-flow-precomparison-2026-11-02/expected.json)

## 현재 결손과 코드 경계

기본 Root HTML runtime은 `RuntimeRootHtmlTextPaintV1` style profile을 사용한다. Stylo cascade는 computed `grid-auto-flow: column`을 보존하고 typed projection도 `GridAutoFlow::Column`으로 매핑한다. 그러나 [Root HTML Grid track 검증](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)은 기본 row flow 외 값을 거부한다. 이어지는 [layout 검증](../crates/spinon-layout/src/grid.rs)은 기본 `Existing` placement mode에서 column flow를 거부한다.

Layout core의 `DenseAutoPlacement` 경로는 sparse column auto-placement의 계산기를 이미 포함하지만, 그 profile은 Root HTML의 제한 계약을 나타내지 않는다. 따라서 구현 시 일반 dense profile을 무조건 재사용하지 않고, 명시 셀에 한정한 별도 `RootHtmlColumnFlowV1` mode를 추가해 scope를 검사한다. 이 mode는 기존 Root HTML entry에서만 선택하고 fixture-only runtime constructor에 연결하지 않는다.

## 기준 규칙과 독립 기대값

기준 규칙은 [CSS Grid Layout Module Level 2 §7.7](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property) 및 [§8.5](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)다. 참조 시점 문서는 2025-03-26 W3C Candidate Recommendation Draft이며 최종 Recommendation으로 다루지 않는다. `column`은 sparse auto-placement가 각 column 안에서 row를 채우고 다음 column으로 이동하게 한다. 고정 fixture에는 명시 columns `[30px,70px]`, rows `[20px,40px]`, `grid-auto-flow:column`, 네 개의 fully-auto item이 있다. item의 fixed size는 할당된 cell과 같고 gap 및 box edges는 0이다. 독립 결과는 다음과 같다.

| item source order | row | column | expected frame (CSS px) |
|---:|---:|---:|---|
| A | 1 | 1 | `(0, 0, 30, 20)` |
| B | 2 | 1 | `(0, 20, 30, 40)` |
| C | 1 | 2 | `(30, 0, 70, 20)` |
| D | 2 | 2 | `(30, 20, 70, 40)` |

셀 수와 item 수가 모두 4이므로 양의 implicit row/column은 필요하지 않다. 독립 덧셈으로 두 축 track positions를 구했고, Chrome 156 `getBoundingClientRect()` 관측을 교차 확인으로 남겼다. Chrome 관측값을 구현 정답의 유일한 근거로 사용하지 않는다.

## 제안하는 최소 런타임 계약

1. 기존 `RuntimeSession::new_runtime_gpu_with_root_html` 경로에서만 기본 Root HTML style/layout/paint revision을 이어 준다. node id, class 이름, tag, scene 이름으로 feature를 선택하지 않는다.
2. CSS `grid-auto-flow: column`만 새로 허용한다. `row`는 기존 동작을 유지하고, `row dense`, `column dense`는 계속 fail-closed한다.
3. columns와 rows는 Stylo typed snapshot에서 각각 하나 이상의 finite non-negative `LengthPx` track으로 계산되어야 한다. 절대 단위 `calc()`가 cascade에서 `LengthPx`로 계산되면 같은 값 계약으로 받는다. percentage/`fr`, line name, fixed/auto `repeat()`, `grid-template-areas`, 중첩 Grid는 이 slice에서 함께 열지 않는다. Fixed-repeat group count는 typed snapshot에서 보존해 Root HTML projection 전에 거부한다.
4. `grid-auto-rows`와 `grid-auto-columns`는 computed `auto`여야 한다. 모든 visible direct Grid item의 계산된 네 placement longhand는 `auto`, `order`는 0이어야 한다. authored line/span/area placement를 무시하거나 auto 배치로 낮추지 않는다.
5. 이 slice의 visible item은 기존 Root HTML block-item 제한을 따르는 direct empty `div`와 fixed finite width/height를 사용한다. 기존 hidden subtree 규칙대로 `display:none` item은 visible capacity와 scene에서 제외하고, non-whitespace 직접 text, nested item, image/intrinsic contribution 및 unsupported display는 fail-closed한다.
6. logical-to-physical 매핑이 정해지지 않은 `direction:rtl` 또는 non-horizontal writing mode는 받지 않는다. 기존 fixed-px gap 규칙은 유지하며, precomparison 기준에서는 초기 gap 0을 사용한다.
7. visible item 수는 `columns × rows` 이하로 제한한다. span/placement가 모두 auto인 조건과 결합해 implicit tracks 없이 명시 셀만 사용한다. 초과 item은 layout/scene publication 전에 거부한다.
8. Typed Root HTML projection은 computed flow `Column`, repeat group count 0, `auto` implicit sizing, 이름 없는 finite explicit tracks, no-area shape 및 explicit capacity를 확인한 뒤 `LayoutInput`으로 변환한다. `RootHtmlColumnFlowV1` layout contract는 변환된 explicit track 값, flow, implicit sizing, item shape와 explicit capacity를 다시 검증한다. layout placement normalizer 결과가 명시 track 범위를 벗어나면 결과를 publish하지 않는다.

## 구현 수락 기준

- [고정 fixture](../tests/fixtures/css/c11/root-html-grid-column-flow-precomparison.html)의 Chrome 156 사전 기준과 같은 `1280×633` viewport에서 root frame, typed explicit track values, semantic item frame을 각각 `0.02 CSS px` 허용오차로 확인한다.
- fixed-px `row-gap:3px; column-gap:5px` 조합 control도 두어 expected origins `(0,0)`, `(0,23)`, `(35,0)`, `(35,23)`을 확인한다. 기존 px gap 계산이 column auto-placement 순서와 결합될 때도 각 축의 gutter를 한 번만 반영해야 한다.
- 실제 Root HTML runtime test에서 기본 column order, row-flow 회귀 control, source-order 변형, 다른 root tag/selector/HostNode ID를 함께 검사한다. 모든 assertion은 color 또는 DOM semantic mapping으로 frame을 찾고 고정 NodeId에 의존하지 않는다.
- rejection tests는 `column dense`, `row dense`, 5번째 item에 의한 implicit column, authored numeric/named/span placement, non-auto `grid-auto-columns`/`grid-auto-rows`, named-area·fixed repeat 조합, RTL/vertical writing mode와 unsupported item content를 각각 확인한다. 실패 때 partial layout 또는 render scene이 남지 않아야 한다.
- `spinon-style-to-layout`, `spinon-layout`, `spinon-runtime`의 변경 관련 테스트와 기존 Root HTML Grid 회귀를 실행한다. `cargo fmt`, scoped Clippy와 제품 feature 빌드를 확인한다. 실행하지 않은 항목은 통과로 보고하지 않는다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 기본 app route에서 같은 HTML source hash, `layout=ready`, box count와 draw/present를 확인한다. Screenshot은 시각 확인만 보조하며 PixelCopy, Metal drawable readback, compositor 이후 pixel parity로 승격하지 않는다.
- 구현 전에 [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)의 숫자 버전을 `0.1.0`으로 유지하면서 이 제한 mode의 입력, 오류 및 미지원 조합을 추가한다. C11.1/C11 checkbox와 공식 완료 집계는 올리지 않는다.

## 계획 단계 검토와 변경 게이트

- 이 문서는 구현 전에 서로 다른 실패 관점 20개로 검토한다: [계획 검토](./c11-root-html-grid-column-flow-plan-review-20.md).
- 계획 PR은 계획·fixture·oracle·문서 diff를 대상으로 별도 30개 적대 관점 검토를 받아야 한다. 이 검토는 계획 20개를 대체하지 않는다.
- 구현 PR은 실제 변경 코드와 runtime 결과를 대상으로 새 실패 관점 20개를 거친 뒤, PR diff 자체의 서로 다른 30개 관점 검토를 완료해야 한다.
- 구현 중 이 계약의 좁은 경계가 깨지면 계획과 oracle을 먼저 고치고 해당 실패 관점을 다시 확인한다. 임의로 일반 CSS 지원이나 implicit sizing까지 넓히지 않는다.

## 범위 밖

전체 C11/C11.1 완료, 일반 CSS admission, explicit placement, dense placement, implicit track sizing, named lines/areas 조합, intrinsic Grid sizing, CSS Grid 전체 WPT 적합성, Android 실기기/hardware GPU 및 iOS 실기기 pixel evidence를 주장하지 않는다.
