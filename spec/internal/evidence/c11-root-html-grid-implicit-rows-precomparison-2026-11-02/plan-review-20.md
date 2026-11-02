# C11.1 Root HTML implicit row 계획 실패 관점 검토 20개

## 판정 범위

이 검토는 `plan/c11-root-html-grid-implicit-rows.md`의 구현 전 범위·검증 계획을 코드 경계와 대조한다. 제품 코드는 변경하지 않았고 테스트도 실행하지 않았다. `PASS`는 위험을 계획의 명시적 제한 또는 수락 조건으로 다뤘다는 뜻이며 구현 통과를 뜻하지 않는다.

## 검토 결과

| # | 실패 관점 | 계획의 방어 또는 확인 기준 | 판정 |
|---:|---|---|---|
| 1 | Root cascade가 `grid-auto-rows: 15px`를 지원 타입으로 전달하지 않아 CSS 단계에서 빠짐 | 현재 cascade가 단일 유한 non-negative px breadth를 `ComputedCssGridAutoRows::LengthPx`로 계산하는 코드를 확인했고, 계획은 회귀 검사를 요구한다. | PASS |
| 2 | CSS-invalid 음수 선언이 valid-but-unsupported computed value와 같은 오류로 처리됨 | 계획은 cascade에서 탈락한 문법 오류와 projection/layout 거부를 구분한다. | PASS |
| 3 | `grid-auto-rows`의 복수 fixed list가 단일 px 값처럼 실수로 수용됨 | 허용 타입을 단일 `LengthPx`로 고정하고 복수 list negative control을 둔다. | PASS |
| 4 | `minmax()`, intrinsic breadth 또는 percentage를 px처럼 좁혀 받아들임 | computed fixed-px 외의 `TrackList`, percentage, fr, minmax, fit-content를 제외하고 negative control로 확인한다. | PASS |
| 5 | `grid-auto-flow:column` 또는 dense 흐름이 row sparse 알고리즘으로 조용히 처리됨 | 기본 `row`만 허용하며 `column`, `row dense`, `column dense` 모두 거부하는 검사를 둔다. | PASS |
| 6 | `grid-auto-columns` 값이 implicit column 생성으로 번짐 | computed `auto`만 허용하고 별도 implicit-column negative control을 둔다. | PASS |
| 7 | named-area template의 missing rows가 C11.3.3의 area expansion과 혼합됨 | named area가 있는 Root HTML은 새 mode에서 계속 거부하고 기존 TemplateGrid 경로는 회귀로 분리한다. | PASS |
| 8 | 숫자 또는 이름 기반 authored placement가 implicit row 기능을 넓힘 | 새 mode는 모든 visible item에 기본 auto placement만 요구하고 authored placement negative control을 둔다. | PASS |
| 9 | CSS `order`가 DOM 순서를 바꾸지만 “기본 DOM row flow” 결과로 기록됨 | Root mode에서 `order:0`만 허용하고 비기본 order를 거부한다. | PASS |
| 10 | `display:none` child가 빈 cell을 차지하거나 implicit row 수를 늘림 | hidden subtree는 auto-placement occupancy에서 제외하고 positive control로 검증한다. | PASS |
| 11 | direct text, nested child 또는 image의 intrinsic contribution이 fixed row 크기와 섞임 | 빈 direct block item만 허용하고 text·nested·image/intrinsic child를 negative control로 둔다. | PASS |
| 12 | 기존 Root HTML fr/percentage explicit sizing과 새 implicit-row mode가 검증 없이 결합됨 | 새 mode는 explicit fixed-px tracks/gap만 받고 fr/percentage 조합을 negative control로 둔다. | PASS |
| 13 | `grid-auto-rows:0px`에서 row가 사라지거나 배치 cursor가 다음 행으로 이동하지 않음 | zero-height implicit row를 track·item 위치의 positive boundary control로 고정한다. | PASS |
| 14 | row gap이 explicit/implicit 경계나 0px track에 두 번 적용되거나 빠짐 | `20 + 5 + 0 = 25`, 한 15px row의 40px, 두 row의 60px를 독립 산술과 Chrome geometry로 확인한다. | PASS |
| 15 | item 수가 늘 때 implicit row 확장이 10,000 track을 초과하거나 overflow됨 | 기존 `GRID_MAX_TRACKS`와 sparse normalizer의 checked end 검사를 유지하고 초과 count를 거부한다. | PASS |
| 16 | 검증 뒤 Taffy track 확장 또는 allocation 중 실패해 일부 frame이 공개됨 | 오류/부분 output 부재, reservation failure, revision identity를 runtime gate로 확인한다. | PASS |
| 17 | stale document/style/layout revision에서 이전 placement와 새 track이 섞임 | 기존 revision guard를 유지하고 stale revision negative control을 둔다. | PASS |
| 18 | Root profile의 새 지원이 fixture constructor나 NodeId/class 선택 분기로만 동작함 | 기본 `new_runtime_gpu_with_root_html()` 경로와 ID/class rename control만 사용하며 fixture FFI dispatch를 추가하지 않는다. | PASS |
| 19 | Root HTML 기본 grid, named area, text, gap, fr, percentage 후속 또는 C11.3.3 regression이 깨짐 | 기존 각 regression과 C11.3.3 placement tests를 독립 실행하도록 수락 조건에 둔다. | PASS |
| 20 | Chrome geometry, simulator draw/present 또는 screenshot이 native surface pixel/WPT 전체 통과로 과장됨 | oracle, geometry, draw/present, 선택적 surface readback과 WPT 한계를 분리 기록하고 C11 공식 상태는 올리지 않는다. | PASS |

## 남은 구현 검증

이 검토는 plan review만 마쳤다. 구현 후에는 별도 20개 구현/runtime 실패 관점, Android API 37 ARM64 AVD와 iOS 26.2 Simulator 실행, 그리고 PR 변경 전용 검토가 필요하다. Chrome 156 사전 비교만으로는 앱 동작을 증명하지 않는다.
