# C11.1 · parsed Root HTML column-flow implicit columns

**상태:** 구현 전 계획. 기존 [column-flow 제한 slice](./c11-root-html-grid-column-flow.md)는 explicit cell만 사용하고 implicit track을 거부한다. 이 후속 계획은 fixed-px `grid-auto-columns`를 써서 positive implicit columns를 생성하는 좁은 기본 Root HTML runtime 경로를 추가한다. 일반 CSS Grid 지원이나 C11 완료를 뜻하지 않는다.

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 통합](./c11-root-html-grid-runtime.md) · [상태 대장](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**선행 구현:** [PR #1056](https://github.com/ohah/spinon/pull/1056) · [기존 column-flow 계획](./c11-root-html-grid-column-flow.md) · [기존 실행 근거](../spec/internal/evidence/c11-root-html-grid-column-flow-runtime-2026-11-02/README.md)

**구현 전 비교:** [고정 HTML, 독립 기대값, Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-precomparison-2026-11-02/README.md)

## 목적과 경계

현재 `RootHtmlColumnFlowV1`은 sparse `grid-auto-flow: column`을 허용하지만 implicit sizing은 `auto`만 받고, visible item 수가 explicit `columns × rows`를 넘으면 거부한다. 그 결과 column 흐름의 다음 열을 채울 때 필요한 implicit columns는 만들 수 없다.

이 작업은 다음 조합만 추가한다.

- explicit rows와 columns: 비어 있지 않은 finite non-negative direct px track 목록
- `grid-auto-flow: column` sparse placement
- `grid-auto-columns`: 단일 finite non-negative px 크기 또는 그런 크기만 포함한 비어 있지 않은 typed track 목록
- `grid-auto-rows: auto`
- placement longhand 네 개가 모두 `auto`이고 `order: 0`인 direct empty HTML `div` child
- item의 고정 finite non-negative px width와 height
- `direction:ltr`, `writing-mode:horizontal-tb`, 단일 non-nested Grid container

기존 fixed-px gap, hidden-item, stale-revision 및 scene publication 동작을 보존한다. `row`, dense flow, implicit rows, line/span/area placement, named line/area template, repeat, percentage, `fr`, `minmax()`, `fit-content()`, math-backed track, nonempty/nested item, CSS-wide 또는 auto item sizing은 이 profile에서 새로 열지 않는다.

## 규범 원본과 사전 비교

[CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)은 `grid-auto-columns`에 여러 크기가 있으면 해당 패턴을 반복하도록 정의한다. [§7.7](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property)에서 `column`은 각 열을 위에서 아래로 채운 뒤 다음 열로 이동한다. [§8.5](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)의 자동 배치 결과를 독립 산술과 Chrome 156 observation으로 교차 확인한다. 사전 비교는 [evidence README](../spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-precomparison-2026-11-02/README.md)에 있다.

Fixture는 explicit columns `[30px,40px]`, rows `[20px,40px]`, auto columns `[15px,25px]`, column gap `5px`, row gap `3px`, 그리고 네 열에 배치될 12개 fixed-size item을 사용한다. 예상 열은 `[30,40,15,25,15,25]`, 행은 `[20,40]`이다. explicit grid 밖의 4개 열에는 `[15,25]`가 두 번 순환한다. 기대 frame은 각 item index에 대해 다음과 같다.

| source order | expected frame (CSS px) |
|---:|---|
| 0 | `(0, 0, 30, 20)` |
| 1 | `(0, 23, 30, 40)` |
| 2 | `(35, 0, 40, 20)` |
| 3 | `(35, 23, 40, 40)` |
| 4 | `(80, 0, 15, 20)` |
| 5 | `(80, 23, 15, 40)` |
| 6 | `(100, 0, 25, 20)` |
| 7 | `(100, 23, 25, 40)` |
| 8 | `(130, 0, 15, 20)` |
| 9 | `(130, 23, 15, 40)` |
| 10 | `(150, 0, 25, 20)` |
| 11 | `(150, 23, 25, 40)` |

Root frame is `(0,0,175,63)`. The expected JSON is authored independently from the captured `getBoundingClientRect()` values. Chrome output only checks the oracle; it is not Spinon evidence.

## 구현 경계

1. `crates/spinon-style-to-layout/src/projection/style_values/root_html_column_flow.rs`에서 `grid-auto-columns`의 single px 및 typed px list를 검사한다. 비어 있는 목록, 비유한/음수 값, 10,000 track 상한 초과, 미지원 track variant는 해당 CSS property와 source node를 포함해 fail-closed한다.
2. `grid_values.rs`의 typed projection에서 `RootHtmlColumnFlowV1` 경로가 이미 검증된 auto-column 값만 `GridContainerInput.auto_columns`로 전달하게 한다. 다른 Root HTML 및 제품 경로의 admission은 넓히지 않는다.
3. `crates/spinon-layout/src/grid_auto_placement/dense_column.rs`의 fixed-px auto-track expansion이 이 제한 profile에서만 활성화되도록 한다. 일반 `DenseAutoPlacement`에는 새 admission을 주지 않는다.
4. `crates/spinon-layout/src/grid_placement/root_html_column_flow.rs`에서 visible item 수가 explicit cell capacity를 넘는 경우도 허용하되, 정상화된 placement가 explicit row 범위 안에 있고, positive implicit column 수가 auto-placement가 필요로 한 최소 개수와 정확히 일치하는지 확인한다. implicit row, negative track, explicit placement 유입은 거부한다.
5. total column count와 expansion 수를 checked arithmetic으로 계산하고 각 축 10,000 track quota를 넘기기 전에 거부한다. 큰 item 수나 auto-column 목록을 대상으로 expansion 전 거부와 bounded allocation을 단위 검사한다.
6. 배치 검증이 실패하면 기존의 Root HTML revision gate와 scene publication 흐름을 통해 partial layout/scene이 노출되지 않음을 확인한다.

## 검증 기준

- Chrome 156 고정 fixture의 root frame 및 12개 semantic item frame을 `0.02 CSS px` 이내로 일치시킨다. implicit column list의 반복, column-major source order, column/row gap을 각각 확인한다.
- Layout tests는 1-column, 1-item baseline, explicit capacity 안의 기존 동작, 정확히 한 개/여러 개 implicit column, 두 번 이상 auto track-list 순환, hidden item 제외 및 row flow 회귀를 확인한다.
- Style projection과 runtime tests는 px scalar/list 긍정 사례와 percentage, `fr`, `auto`, `minmax()`, `fit-content()`, math, empty/oversized list, negative/nonfinite synthetic typed value를 거부한다. `column dense`, implicit row, explicit placement, wrong direction/writing mode, nested Grid, item order, nonempty/non-`div`/auto-sized item도 기존 fail-closed 계약을 유지한다.
- 10,000-track 경계에서 마지막 유효 입력과 첫 초과 입력을 검사하고, 실패 시 layout 결과·revision·render scene이 publish되지 않는지 확인한다.
- auto-column expansion과 item-placement reserve 실패를 주입해 fallible error 전파와 scene 미발행을 확인한다.
- Rust focused tests, 관련 Root HTML regression, `cargo fmt --all -- --check`, scoped Clippy 및 default product feature build를 실행한다. 기존 무관 실패가 있으면 실패로 분리 기록한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 기본 app route에서 같은 HTML fixture hash, `layout=ready`, root/item frame 및 draw/present를 확인한다. screenshot은 시각 확인으로만 사용하며 PixelCopy, Metal drawable readback, 실기기 GPU 또는 compositor 이후 pixels로 확대 해석하지 않는다.
- [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)에 입력, 진단, track cap, atomicity와 지원 밖 경계를 추가하되 숫자 버전 `0.1.0`은 유지한다. C11.1/C11 checkbox 및 공식 완료 집계는 이 제한 slice로 바꾸지 않는다.

## 검토와 병합 게이트

- 구현 전 계획을 서로 다른 실패 관점 20개로 검토했다: [계획 검토](./c11-root-html-grid-column-flow-implicit-columns-review-20.md).
- 계획 PR 변경분은 계획 검토와 별도로 서로 다른 30개 관점에서 검토했다: [계획 PR diff 검토](./c11-root-html-grid-column-flow-implicit-columns-plan-pr-diff-review-30.md).
- 구현 뒤에는 구현/runtime 실패 관점 20개와 구현 PR diff 관점 30개를 새로 기록한다. 계획 검토 결과를 재사용하지 않는다.
- PR은 프로젝트 PR 양식과 문서 라벨 기준을 따르고 Rebase and merge로 병합한다. GitHub check run이 없는 경우 CI 통과로 표시하지 않는다.
