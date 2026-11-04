# C11.1 parsed Root HTML 혼합 fully definite·one-axis auto placement

**상태:** 계획 PR #1146에 이어 구현 PR [#1147](https://github.com/ohah/spinon/pull/1147)이 `2026-11-04T06:01:15Z` Rebase and merge되어 merge SHA `c3b3c3d0ec8f841562b0a724ff596d65b0a3b83a`에 포함됐다. 제한 구현 후보의 placement layout module 9/9, Root HTML integration module 8/8, selector 3/3, scoped Clippy/rustfmt가 통과했다. Android API 37 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 각각 19/19 표본이 exact로 일치했다. 행 고정 부분 item이 실제 fully definite blocker를 건너뛰는 표본을 추가해 앞선 18-sample 실행을 대체했다. 구현/runtime 20개, 구현 diff 20개, 실제 PR diff 적대 검토 30개가 각각 통과했다. PR check은 보고되지 않았고 main required status checks도 없어 CI 통과로 세지 않는다. C11.1/C11과 공식 완료 `72/213`은 유지한다. [runtime evidence](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/implementation-runtime-review-20.md) · [구현 diff 검토 20개](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/implementation-pr-diff-review-20.md) · [PR diff 적대 검토 30개](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/pr-diff-review-30.md)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.1 one-axis auto-placement](./c11-root-html-grid-one-axis-auto-placement.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**비교 기준:** [Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-root-html-grid-mixed-placement-precomparison-2026-11-04/README.md)

**계획 실패 관점:** [독립 20개 검토](./c11-root-html-grid-mixed-explicit-one-axis-plan-review-20.md)

**계획 PR 변경 검토:** [실제 diff 실패 관점 20개](./c11-root-html-grid-mixed-explicit-one-axis-plan-pr-diff-review-20.md)

**구현/runtime 실패 관점:** [20개 검토](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/implementation-runtime-review-20.md)

**구현 diff 검토:** [20개 관점](../spec/internal/evidence/c11-root-html-grid-mixed-placement-runtime-2026-11-04/implementation-pr-diff-review-20.md)

## 결손과 목표

기존 `RootHtmlOneAxisAutoPlacementV1`은 visible direct item 전체가 같은 한 축만 definite하고 다른 축은 모두 `auto`인 Grid만 허용한다. Fully definite item은 같은 Grid의 occupancy를 먼저 예약해야 하지만 현재 selector와 validator가 이를 거부한다. `normalize_sparse_grid_container`는 already fully definite item을 먼저 occupancy에 넣고 definite-axis auto-placement를 뒤에 수행하지만, 이 동작을 Root HTML 기본 route로 노출하는 별도 bounded contract는 없다.

새 `RootHtmlMixedExplicitOneAxisAutoPlacementV1` mode로 한 Grid의 visible items 중 fully definite numeric item과 한 방향의 one-axis numeric item만 함께 허용한다. Layout normalizer는 existing sparse reservation order를 재사용하되 새 mode validator가 admission과 rejection boundary를 다시 확인한다. `RootHtmlOneAxisAutoPlacementV1` 기존 계약은 바꾸지 않는다.

## 제한 계약

- 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`의 no-area `grid-auto-flow: row` owner 하나에 한정한다. sibling Grid owner의 placement state는 공유하지 않는다.
- visible participating direct items 중 fully definite numeric item이 하나 이상, 그리고 row-locked 또는 column-locked one-axis item이 하나 이상 있어야 한다. Partial items는 모두 같은 definite axis를 써야 하며 definite 축은 numeric line/span, 반대 축 양 끝은 모두 `auto`여야 한다. Fully definite item은 두 축 모두 numeric line/span이어야 한다.
- Explicit columns/rows는 비어 있지 않은 finite non-negative fixed-px 목록이고, container 크기는 finite definite CSS px다. Gap, margin, padding, border는 0이다. 추가 implicit row/column extent가 필요하면 전체 Grid 결과를 거부한다.
- Item은 static empty non-replaced direct block이며 auto 또는 finite non-negative fixed-px size를 쓴다. Source order와 paint order는 보존한다. Fully definite items는 CSS Grid 규칙처럼 partial auto-placement보다 먼저 occupancy를 예약한다.
- `display:none`은 selector와 occupancy에서 제외한다. `visibility:hidden`은 occupancy를 차지한다.
- Named line/span/area, all-auto item, mixed row-locked와 column-locked partial items, partial item의 양 축 placement, dense/column flow, repeat/auto-repeat, nested Grid, text, image/replaced item, positioned item, nonzero order, unsupported style, implicit extent 및 track cap 초과는 fail-closed한다.
- Reject 시 partial frame, expanded track 또는 draw record를 publish하지 않는다. 다른 Grid owner의 결과와 revision을 오염시키지 않는다.

## 독립 oracle와 판정

고정 fixture는 40/50/30px columns와 20/30/40px rows를 사용한다. CSS Grid §8.5에 따라 fully definite item들을 먼저 occupancy에 등록한 후 같은-axis partial items를 DOM source order로 배치해 expected frames를 계산했다. Partial item의 definite 축에는 span을 허용하고 반대 축은 양 끝 모두 `auto`로 둔다. Chrome 156 Agent Browser의 9 scenes·20 items·19 visible frames 관측은 독립값과 최대 0 CSS px 차이였다. 화면·hash·도구 한계는 precomparison evidence에 기록한다.

Accepted scenes는 row/column definite blocker, row/column span, hidden fully definite blocker 및 display-none control이다. Mixed partial axes와 implicit row extent는 Chrome에서 유효한 layout이라도 제품 후보에서는 거부한다. all-auto는 기존 route control이다. 제품 acceptance는 accepted frame과 exact typed disposition, rejection 시 atomic output 없음으로 판정한다. Geometry tolerance는 0.5 CSS px, authored track values tolerance는 0.01 CSS px다.

## 구현 순서와 검증

1. 계획 PR 전에 Chrome fixture, 독립 expected JSON, browser observation, screenshot hash와 verifier를 고정한다. 계획 문서를 서로 다른 실패 관점 20개로 검토하고 계획 PR 실제 diff를 별도 20개 관점으로 검토한다.
2. Layout에 새 placement mode와 admission validator를 추가한다. Existing `RootHtmlOneAxisAutoPlacementV1` rejection behavior를 유지하며 sparse normalizer의 definite-item reservation pass가 partial placement보다 먼저 실행되는 것을 tests로 고정한다.
3. Style projection에 혼합 selector를 추가한다. Visible item 집합의 typed placement만 읽고 owner별로 mode를 고른다. All-auto·다른 partial axis·named endpoint·areas·dense/column flow를 새 route로 보내지 않는다. `display:none`·`visibility:hidden`을 서로 다르게 처리한다.
4. Runtime fixture는 accepted mixed-placement 6 scenes와 all-auto·fully-definite existing-route controls를 포함한다. 혼합 partial axis와 implicit extent의 negative controls는 focused Rust tests 및 Chrome oracle에서 별도로 확인한다. 현재 `RuntimeRootHtmlTextPaintV1`은 이 unsupported computed placement를 만나면 Grid owner 단위가 아니라 문서 전체를 layout 전에 거부하므로 negative owner를 accepted scene과 한 mobile runtime fixture에 합치면 surface draw까지 진행할 수 없다. Android/iOS surface에서는 accepted scene의 geometry 결과를 19개 표본으로 확인한다. 행 blocker 표본은 뒤에서 온 fully definite item이 같은 행의 열을 선점하고 다음 sparse item이 세 번째 열로 이동하는 것을 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 fixture hash와 final source를 실행한다. Android PixelCopy는 19/19, iOS Metal drawable은 19/19 exact다. iOS readback은 compositor 제출 전이며 두 결과 모두 시뮬레이터 근거다. Android AVD 결과를 실기기/hardware GPU 적합성으로 일반화하지 않는다.
6. Scoped Rust tests, fixture/reference/boundary checks, relevant Clippy, `cargo fmt --all -- --check`, default vs opt-in artifact boundary를 확인한다. 계획 검토와 겹치지 않는 구현/runtime 실패 관점 20개 및 실제 구현 diff 20개를 각각 완료했다. 결함을 고친 뒤 관련 관점과 simulator 표본을 재실행했다.
7. 구현 PR을 만든 뒤 실제 PR diff를 프로젝트 규칙에 따라 별도 실패 관점 30개로 검토한다. PR에는 변경 ID, contract/evidence, 남은 한계를 기록하고 Android/iOS 화면을 첨부할 수 있으면 Agent Browser로 이미지 로드를 확인한다. Merge 후 `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, C11 plan과 Tailnet roadmap을 증거에 맞춰 갱신한다. C11.1/C11 및 공식 `72/213` 상태는 승격 조건을 만족하기 전까지 그대로 둔다.

## 완료로 주장하지 않을 사항

이 bounded mode는 일반 CSS admission, named lines/areas, all-auto와 mixed partial-axis placement, implicit track sizing, full C11.1/C11, public CSS support, WPT conformance, Android physical-device/hardware GPU, iOS compositor-after pixel equality를 주장하지 않는다.
