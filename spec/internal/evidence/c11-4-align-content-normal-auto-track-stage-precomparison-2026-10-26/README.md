# C11.4 · `align-content: normal` 자동 트랙 늘림 사전 비교

## 목적과 기준

이 입력은 기존 §11.8 stretch-stage Rust fixture에 `align-content: normal` positive를 추가하기 위한 test-only oracle다. 규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이며, 관련 축의 content-distribution 값이 `normal` 또는 `stretch`이면 남은 양의 definite free space를 `auto` max track sizing function을 가진 행에 균등 배분한다.

기대값은 Taffy 출력이 아니라 규범과 고정 fixture 입력에서 계산했다. 사용한 입력과 결과는 [`input.json`](./input.json)과 [`expected.json`](./expected.json)에 저장했다.

## 입력

- Root 및 viewport: definite `20×100px`, `RootSizingPolicy::Match`, border/padding `0`.
- Grid: 한 개의 `20px` column, rows `[auto, auto, 5px]`, `row-gap:10px`, `column-gap:0`.
- Alignment: `align-content:normal`, `justify-content:flex-start` (`LayoutJustifyContent::FlexStart`, 기존 `input` helper와 동일).
- Items: DOM 순서대로 한 행에 하나씩 놓인 direct block child 세 개, definite heights `[10,20,5]px`, width `20px`.
- Stage trace: `GridPlacementMode::TrackSizing`.

이는 기존 `grid_stretch_auto_tracks_oracle_tests.rs` 입력과 동일한 geometry다. 바뀌는 입력은 `LayoutAlignContent::Normal`뿐이다.

## 독립 계산

§11.8 이전 rows는 item의 definite heights `[10,20,5]px`다. 두 내부 row gutter 각각 `10px`이며 root block size는 definite `100px`이므로 남는 definite free space는 `100 - 35 - 20 = 45px`다. 첫 두 행은 `auto` max track sizing function을 가져 대상이 되고, 마지막 `5px` fixed track은 대상이 아니다. 따라서 각 auto row는 `22.5px` 성장해 최종 rows `[32.5,42.5,5]px`를 만든다. 남은 row-axis free space는 `0px`이다.

Direct children은 각자의 고정 height를 유지한다. 최종 row starts에 따라 기대 frame은 `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`이다. Root frame은 `(0,0,20,100)`이다.

예상 stage 상태는 `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`에서 rows `[10,20,5]px`, 이어지는 단일 `StretchAutoTracks { applied:true }`에서 `[32.5,42.5,5]px`이다. Stage 순서는 해당 값을 비교하기 전에 별도로 확인한다.

## 비교 대조와 한계

병합된 `align-content:stretch` positive 입력은 같은 규범 기대값을 사용하고, `align-content:start` control은 stage 미적용 및 rows `[10,20,5]px`를 유지한다. 새 `normal` 입력은 이 구현 결과를 oracle로 가져오지 않고 위 독립 계산과 직접 비교한다.

현재 Rust style projection은 `LayoutAlignContent::Normal`을 Taffy `AlignContent::STRETCH`로 보낸다. 이 사전 비교는 그 경로의 단위 검증을 제안할 뿐이며, 일반 author CSS cascade, 플랫폼 runtime, browser rendering, WPT 또는 전체 CSS Grid 적합성을 주장하지 않는다. 계획 검토가 PASS되기 전까지 구현은 시작하지 않는다.
