# C11 · parsed Root HTML Grid auto-repeat 제한 구현 계획 · 2026-11-05

**상태:** 계획 및 사전 비교 후보. 구현 전이다. · **대상 경로:** `RuntimeRootHtmlTextPaintV1`의 기본 parsed Root HTML Grid 경로. · **완료 판정:** 이 제한 후보만으로 C11.1, C11.4, C11 또는 공개 CSS 지원을 완료 처리하지 않는다. 현재 공식 집계 `78/214`를 유지한다.

## 문제와 코드 경계

Root HTML 내부 계약은 양의 정수 `repeat()`을 제한 지원하지만 `auto-fill`/`auto-fit`은 거부한다([0059 C11 내부 계약](../spec/internal/0059-c11-grid-layout.md)). 현재 `grid_values::project_grid_layout_with_route`는 auto-repeat을 `RuntimeBlockFlexGridTrackSizingV1`에서만 허용하고, `validate_root_html_grid_tracks`도 Root HTML profile에서 모든 auto-repeat을 거부한다. 공용 `spinon-layout` adapter는 이미 typed `GridAutoRepeat`을 Taffy repetition으로 변환한다. Root HTML의 item 용량 검사는 authored `columns.len() × rows.len()`을 사용하므로, 확장된 repeat count를 반영하는 제한 검사가 추가되어야 한다.

## 제안하는 지원 계약

Root HTML 기본 경로에서 다음 조건을 모두 만족하는 한 축의 auto-repeat만 연결한다.

- `auto-fill` 또는 `auto-fit` 하나만 사용한다. Columns와 rows에 동시에 쓰거나 같은 축에 둘 이상 쓰는 입력은 거부한다.
- 반복 fragment가 그 축의 track list 전체이며, pattern은 1개 또는 2개의 `minmax(<finite positive px>, 1fr)` track이다. Prefix/suffix track, integer `repeat()` 혼합, named line, `grid-template-areas`는 거부한다.
- 반복 축의 computed width/height는 finite positive CSS px로 definite이고, 해당 축의 min-size는 `auto`, max-size는 `none`, aspect ratio는 `auto`다. Percentage/calc/intrinsic track 최소값과 percentage gap은 거부한다.
- gap은 0 이상 finite CSS px다. 축의 padding·border·margin은 0이며 track-count basis와 content size가 다르지 않다.
- 다른 축은 이름 없는 fixed px track 하나다. Default row auto-placement만 사용한다. Column/dense flow, explicit line placement, implicit track 생성은 거부한다.
- 각 expanded axis는 `GRID_MAX_TRACKS`를 넘지 않는다. 계산한 expanded explicit cell capacity보다 grid item 수가 크면 배치 전에 거부한다. `display:none` 자식은 item 용량과 `auto-fit` 점유에서 제외한다.
- `auto-fit`의 빈 track 및 인접 gutter collapse, visible item overflow와 track sizing은 기존 Taffy 경로가 처리한다. Spinon은 이 알고리즘을 새로 구현하지 않는다.
- 지원 밖 값, 자원 상한 초과, 산술 불능 및 allocation 오류에서는 부분 layout scene을 publish하지 않는다.

Expanded repeat count 사전 검사는 위 grammar로 한정한다. Repeat pattern의 고정 px 최소값, fixed px gap, definite content axis만 입력으로 사용하며, CSS Grid §7.2.3.2에 따른 track당 UA 1px count floor(반복 횟수 산정에만 적용), Taffy의 최소 1회 repetition과 `GRID_MAX_TRACKS` clamp를 포함한다. Count 계산에만 floor를 적용하고 실제 track sizing은 authored minimum을 유지한다. 경계·산술 결과가 Taffy와 다르지 않음을 Rust adapter/runtime matrix로 확인한다. 이 subset 밖의 count 계산은 거부한다.

## 사전 비교와 독립 기준

[Agent Browser Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/README.md)는 auto-fill/auto-fit, columns/rows, empty·hidden item, 169/170px count boundary, track당 UA 1px count floor, visible overflow를 포함한 13개 장면이다. [정정된 규범 기대값](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/normative-expected.json), [원시 관찰](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/chromium-observation.json), [비교 결과](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/comparison.json), [화면](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/chromium-agent-browser.png)을 구현 전에 고정했다. 최초 hand-derived fr 계산의 오류도 원본 파일과 함께 남겨 corrected oracle의 산출 경위를 보인다.

## 구현 순서

1. Root HTML route 전용 validator에서 지원 grammar, definite axis, gap 및 zero-edge 조건을 한 번 검증한다. 기존 C11.4 track-sizing profile의 허용 범위는 넓히지 않는다.
2. Root HTML profile의 `GridAutoRepeat` projection gate를 해당 validator가 승인한 입력에만 연다. 공용 Taffy converter와 auto-fit/auto-fill engine 동작을 재사용한다.
3. expanded repeat count와 expanded explicit capacity를 overflow-safe하게 계산하고 `validate_root_html_grid_item_capacity`에 반영한다. 최대 tracks, overflow, hidden child와 no-implicit-track 경계를 회귀 검사한다.
4. CSS cascade→typed snapshot→Root HTML style/layout runtime 경로의 positive 및 fail-closed tests를 추가한다. Count 경계 169/170, 양 축 개별 route, 1-track/2-track pattern, auto-fill/auto-fit의 빈·점유·hidden 경우, over-cap 및 item-capacity control을 포함한다.
5. Chrome 사전 fixture와 같은 author HTML을 기본 Root HTML 경로에서 실행하고 결과 frames/tracks를 독립 기대값과 비교한다. 기존 integer-repeat, fixed tracks, percentage/fr/fit-content 경로도 scoped 회귀 확인한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML runtime을 실행한다. Root/item geometry, expanded track evidence, empty-track collapse, overflow와 screenshot을 기록한다. Pixel 표본을 얻으면 그 표본/표면 시점만 명시하며 simulator 결과를 실기기 증거로 표현하지 않는다.
7. 기본 artifact에 fixture 전용 adapter가 새로 유입되지 않았는지 확인한다. 이 변경은 일반 HTML parser/API나 fixture-only C11.4 entry point에 의존하지 않는다.
8. 구현/runtime을 서로 다른 20개 실패 관점으로 검토하고, 별도 PR diff를 30개 관점으로 검토한다. PR은 현재 project rule대로 Rebase and merge를 사용하고 check rollup이 비어 있으면 CI 성공으로 기록하지 않는다.
9. 병합 후 `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, `plan/c11-css-grid.md`의 실제 구현 범위와 증거 링크만 갱신한다. 전체 C11/C11.1/C11.4의 미완료 상태와 `78/214` 집계는 완료 조건이 충족되지 않는 한 유지한다.

## Pass criteria

- 13개 Chrome 사전 비교 장면에서 expected repeat count, track sizes, root/item frames, hidden/empty occupancy, UA count floor와 overflow가 독립 기대값과 일치한다.
- Root HTML runtime이 지원 subset을 성공시키고, 계획한 negative controls는 부분 frame/scene 없이 명시적으로 거부한다.
- Expanded explicit capacity 검사에서 실제 반복 수와 item 수의 관계가 정확하고, implicit row/column으로 조용히 넘기지 않는다.
- Android AVD와 iOS Simulator의 동일 source candidate에서 geometry가 독립 기준과 일치하고 실행·화면 artifact가 PR에서 열 수 있다.
- 기존 scoped Root HTML Grid 경로가 통과하고 Rust format/lint 게이트가 통과한다. CI가 보고되지 않은 경우 그 사실을 그대로 기록한다.

## PR 분리

이 계획과 고정 사전 비교를 먼저 PR로 검토·병합한다. 구현은 그 뒤 별도 순차 브랜치에서 진행하고, 구현/runtime 검토와 PR diff 검토는 계획 리뷰와 별개로 새로 수행한다.
