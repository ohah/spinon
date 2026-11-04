# C11.1 · parsed Root HTML Grid column-flow 혼합 교차축 배치

**상위:** [C11 CSS Grid 계획](./c11-css-grid.md) · [parsed Root HTML column 흐름](./c11-root-html-grid-column-flow.md) · [행 흐름 혼합 교차축 배치](./c11-root-html-grid-mixed-cross-axis.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 사전 비교와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/README.md) · [기대 frame](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/expected.json) · [Agent Browser capture](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/capture.json) · [Chrome 화면](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/chrome-156-agent-browser.png)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-column-flow-mixed-cross-axis-plan-review-20.md) · **계획 PR 변경 검토:** [30개](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/plan-pr-diff-review-30.md).

## 결손과 목표

기본 parsed Root HTML Grid는 `grid-auto-flow: column`의 fully-auto sparse 배치를 지원한다. 반면 현재 `RootHtmlColumnFlowV1` validator는 네 placement longhand가 모두 `auto`인 경우만 허용한다. 행 흐름에는 row-locked와 column-locked item을 함께 배치하는 `RootHtmlMixedCrossAxisAutoPlacementV1`가 이미 있지만, 열 흐름에서는 두 종류의 부분 고정 배치가 같은 Grid owner에 섞이면 거부한다.

이 제한 slice는 같은 Root HTML Grid 안에서 fully definite, row-locked, column-locked 및 fully-auto item을 열 우선 sparse 알고리즘으로 배치한다. 새 `RootHtmlMixedCrossAxisColumnAutoPlacementV1` placement mode와 owner-scoped selector로 구분한다. 양 종류의 부분 고정 item이 모두 있는 Grid에만 새 mode를 선택한다. 기존 row-flow mixed, column-flow all-auto 및 일반 Root HTML 경로의 admission은 유지한다.

## 규범 기준과 독립 비교

규범 기준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)다. `grid-auto-flow: column`에서는 fully definite item을 먼저 예약하고, column 위치가 definite인 item을 order-modified document order로 각 고정 column의 다음 사용 가능 row에 놓는다. 남은 row-locked와 fully-auto item은 sparse cursor로 처리한다. cursor는 row를 먼저 진행하고 끝에서 다음 column으로 이동하며, row-locked item은 definite row를 유지한다. `visibility:hidden` item은 occupancy에 남고 `display:none` child는 Grid item이 아니다.

독립 셀 점유 계산과 Chrome 156 관측은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/README.md)에 분리했다. 고정 기본 Grid는 4 columns × 3 rows, 40px × 30px tracks, 160px × 90px definite content box와 0 gap이다. span 장면만 여섯 40px columns로 늘려 모든 배치가 explicit grid 안에 있도록 한다. 여덟 positive scenes의 31 participating item frames(27 CSS-visible, 4 `visibility:hidden`)은 Chrome과 독립 기대값이 모두 exact하게 일치한다. implicit-boundary control은 authored grid 밖의 row/column을 각각 요구하며, Chrome에서 5 columns × 4 rows로 확장됨을 확인했다.

## 허용 입력과 거부 경계

- 기존 `RuntimeRootHtmlTextPaintV1` parsed Root HTML route에서 Grid owner 하나씩 판정한다. 하나의 owner 내부 item만 분류하고 sibling/nested Grid의 item, revision 또는 occupancy를 공유하지 않는다.
- `grid-auto-flow: column` sparse만 허용한다. `column dense`, row flow, named area/line, `repeat()`/auto-repeat은 새 mode의 입력이 아니다.
- Columns/Rows는 비어 있지 않은 finite non-negative fixed-px explicit track list다. Root width/height는 definite finite fixed-px이고 explicit tracks의 합과 일치한다. Gap과 box edges는 0이며 방향·writing mode는 LTR/horizontal-tb, alignment는 start, `order`는 0이다.
- `grid-auto-rows`와 `grid-auto-columns`는 computed `auto`여야 한다. 결과가 explicit grid 밖의 implicit row/column을 요구하면 Grid 전체를 frame·trace·draw 없이 원자적으로 거부한다. 명시적 out-of-grid placement를 clamp하거나 일부만 배치하지 않는다.
- `display:none`이 아닌 participating direct Grid item은 fully definite numeric placement, row-locked (definite row / auto column), column-locked (auto row / definite column) 또는 fully auto placement만 사용한다. Mode selector는 `visibility:hidden`을 포함한 모든 participating item을 분류하고, row-locked와 column-locked가 각각 하나 이상일 때만 새 mode를 선택한다. 완전 고정 및 fully-auto item은 이 두 종류와 함께 둘 수 있다.
- 양·음 numeric line은 explicit grid 내부의 track을 가리켜야 한다. 양의 numeric span은 지원한다. 0 span, 범위 밖 line, 한 축의 혼합 endpoint, named line/span은 기존 property/node 오류로 거부한다.
- Direct child는 empty, non-replaced, non-nested block item이다. `display:none` child는 분류·occupancy 전에 제외하고 `visibility:hidden`은 occupancy에 포함한다. Text, replaced/nested item, positioned item, nonzero margin/padding/border, unsupported size, stale revision 및 duplicate/missing NodeId는 거부한다.
- 대상 외 row-only/column-only/auto-only 입력은 새 mode가 가로채지 않는다. 기존 route 결과 또는 기존 fail-closed 오류를 유지한다.

## 구현 순서와 수용 기준

1. Source baseline에서 `RootHtmlColumnFlowV1`이 non-auto placement를 거부하고 row mixed mode가 column flow를 거부하는 조건을 focused tests로 고정한다.
2. 새 `GridPlacementMode`를 추가한다. Selector는 하나의 Root HTML owner의 participating direct items(`display:none` 제외, `visibility:hidden` 포함)와 typed computed placement를 검사한 뒤, column flow이면서 row-locked·column-locked가 각각 있는 경우만 mode를 고른다. authored CSS 문자열, class/tag 이름 또는 scene ID로 mode를 선택하지 않는다.
3. Column-flow validator는 기존 track/style/item 제한을 보존하면서 네 placement class, positive span, explicit negative-line resolution, both-axis occupancy 및 implicit extent rejection을 검사한다. 기존 sparse column normalizer를 재사용하되 `dense`가 켜지지 않았는지 확인한다. Row-flow validator, fully-auto column route 및 Taffy 계산 규칙은 별도 회귀로 보호한다.
4. 고정 Root HTML fixture 여덟 positive scenes—DOM interleave, row/column span, hidden occupancy와 hidden-only lock qualification 2종, `display:none`, reverse DOM grouping, negative line—를 기본 parsed HTML runtime route로 연결한다. 기대 frame은 [expected.json](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/expected.json)의 31 participating item frame(27 CSS-visible, 4 hidden)과 exact 비교한다. Implicit boundary scene은 row와 column 바깥 배치 각각의 정확한 오류 귀속, partial frame/trace/draw 부재를 검증한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 source로 여덟 positive scenes와 negative control을 실행한다. Android PixelCopy와 iOS Metal drawable 표본은 장면별 input/source revision 및 installed artifact hash에 연결한다. Android software renderer와 iOS compositor 제출 전 readback 한계를 구분하고 실기기·hardware GPU 근거로 확대하지 않는다.
6. Focused layout, style-to-layout selector/projection, Root HTML integration, 기존 one-axis/row-mixed/column-all-auto 회귀, `cargo fmt --all -- --check`, 변경 crate scoped Clippy, Android/iOS Simulator builds 및 기본/opt-in artifact boundary를 기록한다. 실행하지 않은 workspace/CI gate를 통과로 보고하지 않는다.
7. 구현/runtime 결과를 별도 실패 관점 20개로 검토한다. 계획 검토와 중복하지 않는 실제 PR diff 적대 검토 30개를 merge 전에 기록한다. 결함을 찾으면 수정 후 영향을 받는 검토 관점을 다시 확인한다.
8. 병합 뒤 내부 계약 0059, `spec/STATUS.md`, C11 plan과 public roadmap에 정확한 merge SHA, scope, 검증, 남은 한계를 동기화한다. PR check rollup이 비었으면 CI 통과라고 표시하지 않는다.

## 완료 판정과 미포함

여덟 positive scenes의 31 participating item frames(27 CSS-visible, 4 hidden)가 독립 기대값·Chrome과 CSS 좌표 exact하게 일치하고, negative control은 implicit extent를 정확히 거부해야 한다. 특히 row-locked 또는 column-locked class의 유일한 대표가 `visibility:hidden`이어도 selector가 해당 Grid owner를 새 mode로 분류해야 한다. 같은 source revision의 Root HTML runtime, Android AVD, iOS Simulator에서 route, owner, revision, item frame, track, trace 및 draw/present 범위를 모두 확인해야 한다. Row-flow mixed와 column-flow fully-auto 결과는 변경 전 기준과 같아야 한다.

일반 author CSS admission, named area/line, dense column flow, implicit track sizing, auto-repeat, nonzero gap, arbitrary track grammar, text/replaced/nested/positioned item, nonzero order, RTL/vertical writing mode, full WPT conformance, 실기기/hardware GPU, iOS compositor 이후 pixels 및 C11.1/C11 완료는 주장하지 않는다. 내부 계약 버전 `0.1.0`, C11 checkbox 및 공식 완료 `72/213`은 변경하지 않는다.
