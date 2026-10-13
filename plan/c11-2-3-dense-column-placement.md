# C11.2.3 · dense·column 자동 배치와 양의 암시 열

**상태:** 제한 runtime profile, Android 실기기/iOS Simulator geometry 및 표면 pixel gate 완료 · Chrome 기준 25 scene·107 node · 성공 19·기대 거부 6 · 양 플랫폼 geometry 최대 오차 0 CSS px · 표면 99/99 표본 exact
**상위:** [C11 CSS Grid](c11-css-grid.md) · [C11.2.2 sparse 행 배치](c11-2-2-sparse-row-placement.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)
**계획 실패 관점 검토:** [20개 계획 검토](../spec/internal/evidence/c11-2-3-dense-column-placement-plan-review-2026-10-13.md)
**계획 기준 엔진:** 현재 저장소의 고정 Chromium 154 profile · 계산 후보 Taffy `0.14.0`
**계약 버전:** `0059`와 crate 숫자 버전을 `0.1.0`으로 유지한다.

## 목표와 경계

C11.2.2의 `grid-auto-flow: row` sparse auto-placement에 이어 다음 세 computed 값을 별도 내부 runtime profile에서 다룬다.

- `grid-auto-flow: row dense`
- `grid-auto-flow: column`
- `grid-auto-flow: column dense`

CSS 기본값인 `row` sparse 동작은 이 profile에 포함해 회귀를 확인하되, 기존 C11.2.2 profile의 성공 의미와 FFI 진입점은 변경하지 않는다. dense는 앞서 생긴 빈 칸을 다음 항목이 채울 수 있고 시각 위치가 source order와 달라질 수 있다. 그래도 DOM child 배열·accessibility order·paint order를 배치 결과만으로 재정렬하지 않는다.

이번 범위는 양의 방향으로 생성되는 implicit row/column에 한정한다. `grid-auto-rows`와 `grid-auto-columns`는 각각 초기값 `auto` 또는 단일 비음수 고정 `px` 길이만 허용한다. 해당 축에서 implicit track을 만들지 않는 경우에만 `auto`를 허용한다. 반복 sizing pattern과 다른 track 함수는 거부한다. 각 축의 explicit+positive implicit track 합은 기존 Taffy 경계를 넘기 전에 검사하고, 범위를 넘으면 layout 전체를 실패시킨다.

## 지원 계약

1. Stylo의 computed typed `grid-auto-flow`·`grid-auto-rows`·`grid-auto-columns`를 사용한다. 선언 문자열을 별도로 파싱하지 않는다. CSS shorthand `grid`의 cascade 해석은 기존 Stylo 경로 결과만 사용한다.
2. 배치 순서는 CSS Grid §8.5의 단계에 맞춘다: definite 두 축 item, definite row에 잠긴 item(row flow; column flow는 전치), implicit grid의 flow-secondary track 수 결정, 남은 auto item 순서다. definite position item은 order-modified document order로 처리하지만 이번 profile은 `order != 0`을 거부하므로 입력 child 순서와 같다. 각 단계의 점유 영역과 오류는 container 단위로 원자적이다.
3. row flow에서는 row에 잠긴 item을 처리한 뒤 모든 item의 definite column과 auto column span을 보고 implicit column track 수를 먼저 결정한다. column flow는 행과 열을 바꾼다. sparse는 이전 cursor 이후부터 검색하고, dense는 각 남은 auto-placement 검색을 implicit grid 시작점부터 수행한다. Chrome에서 관측한 각 item의 정규화 line·cell·rect가 기준이다.
4. definite secondary-axis item은 고정된 secondary 위치에서 primary 축의 빈 범위를 찾는다. definite primary/secondary 조합, 양의 span, span 충돌과 wrap은 flow별 Chrome fixture로 고정한다. CSS conflict resolution은 C11.2.1에서 이미 정한 규칙을 재사용한다.
5. 두 축 모두 auto인 item은 해당 flow의 primary 축을 먼저 전진하고 secondary 축 경계에 닿으면 다음 secondary track으로 이동한다. `dense`는 다음 item 검색 전에 시작점으로 돌아간다. 이로 인해 뒤 item이 앞 item보다 먼저 칠해지지 않도록 source/paint order를 별도 확인한다.
6. explicit line 범위 밖의 지정, 음수 implicit track, 이름 기반 line/area, `order != 0`, text/replaced/intrinsic item, indefinite container axis는 거부한다. 이 작업은 C11.2.2의 제한 track·gap·item-size 입력을 넓히지 않는다.
7. positive implicit row/column의 computed sizing은 단일 고정 `px` 값만 사용한다. `auto` sizing은 새 implicit track이 필요 없는 fixture에만 허용한다. 여러 값이 반복되는 pattern, `%`, `fr`, `minmax()`, `fit-content()`는 추정치나 Taffy 기본값으로 대체하지 않고 오류를 반환한다.
8. nested Grid container는 각 container의 computed flow와 occupancy로 독립 계산한다. `display:none` subtree와 기존 규칙상 in-flow가 아닌 child는 auto-placement 대상에 넣지 않는다.
9. 내부 전용 profile ID는 `RuntimeBlockFlexGridAutoPlacementDenseV1`로 제안한다. 이름, serialized fixture schema 및 FFI fixture entry는 사전 비교 fixture를 고정하기 전에 코드·계약 문서와 함께 확정한다. 사용자 공개 API나 CSS 지원 선언은 만들지 않는다.

## 구현 전 비교 모델

구현 전에 고정 Chromium 154 binary·revision·OS·viewport·DPR·locale·font·media profile을 기록하고, C11.2.2의 reference를 덮어쓰지 않는 새 C11.2.3 inventory와 SHA-256을 만든다. scene별 preorder NodeId와 parent, computed placement/flow/auto-track 값, explicit·implicit track 수, item rect와 예상 지원/거부를 저장한다.

고정 기준은 [C11.2.3 inventory v6](../tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json), [HTML fixture v6](../tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html), [Chrome 154 reference v7](../tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json)다. 25 scene·107 node 중 21개는 구현 후보, 4개는 oracle-only다. row·column dense와 동일 source order sparse 대조 scene을 각각 고정했고, 음수 implicit line과 indefinite axis를 분리했다. 10,000·10,001 track 경계는 `span` 자동 배치 후보로 기록했다. Chrome은 각각 10,000·10,001 computed column을 만들지만, 이는 Spinon의 허용 범위를 정하지 않는다. 런타임은 10,000 상한 초과를 layout 입력 검증 단계에서 거부해야 한다. 상세 환경·계산 좌표·한계는 [사전 비교 근거](../spec/internal/evidence/c11-2-3-dense-column-placement-precomparison-2026-10-13/README.md)를 따른다. pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 세 후보를 Chrome 154 runner에서 실행해 3/3 test·36/36 subtest PASS를 확인했다. 이는 Chrome oracle 결과이며 Spinon 적합성 근거가 아니다. 규범 알고리즘은 [CSS Grid Layout Module Level 2 §8.5](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)이며 Chrome 결과와 함께 판정한다.

최소 20 scene을 포함한다.

| 사례 | 기준에서 고정할 동작 |
| --- | --- |
| 기본 sparse negative control | 기존 C11.2.2 row 결과와 같고 앞선 빈 칸을 되채우지 않는다. |
| row dense 기본 구멍 | span item 뒤에 생긴 구멍을 다음 작은 item이 채운다. |
| row dense span 충돌 | 후보 위치 일부만 겹칠 때 다음 빈 직사각형으로 이동한다. |
| row sparse 대조 | dense와 같은 source children에서 cursor가 되감기지 않는 차이를 확인한다. |
| row dense definite-row item | 고정 row 안에서 definite row/auto column 충돌을 검색한다. |
| row dense definite-column item | 고정 column을 유지하며 primary row cursor를 검색한다. |
| row dense implicit row | 양의 implicit row 생성 및 단일 `grid-auto-rows` 적용을 확인한다. |
| column sparse explicit fill | row를 먼저 채운 뒤 다음 column으로 넘어간다. |
| column sparse implicit column | explicit rows가 부족할 때 양의 implicit column을 만든다. |
| column sparse definite-row item | 고정 row에서 column-major placement 경계를 확인한다. |
| column sparse definite-column item | 고정 column에서 primary row를 검색한다. |
| column dense hole fill | column 방향 dense 검색이 이전 빈 영역을 채운다. |
| column dense span | 두 축 span이 있을 때 column-major collision skip을 확인한다. |
| column dense implicit column | dense 재검색과 양의 implicit column 확장을 함께 확인한다. |
| definite item과 dense auto item | 선점한 definite item이 dense 후보를 계속 막는다. |
| hidden item | hidden child가 점유 cell과 DOM identity를 만들지 않는다. |
| nested mixed flow | 내부·외부 Grid가 서로 다른 auto-flow를 사용한다. |
| implicit track sizing | `grid-auto-rows`/`grid-auto-columns` 고정 `px`와 `auto` 미사용 조건을 구분한다. |
| unsupported declaration | 복수 auto-track pattern 등 unsupported 값을 구체 오류로 거부한다. |
| track limit·implicit boundary | limit 직전과 초과, 음수 implicit 요구를 구분해 고정한다. |

기준 scene의 각 node frame은 최대 절대 오차 `0.5 CSS px` 이내로 판정한다. 예상 거부 scene은 오류 종류·node·property·값이 맞아야 하며 partial output이나 양수 draw가 없어야 한다. dense/column behavior와 연관된 고정 WPT 후보 세 개의 원문 hash 및 Chrome runner 결과를 [사전 비교 근거](../spec/internal/evidence/c11-2-3-dense-column-placement-precomparison-2026-10-13/README.md)에 연결했다. 적합성 판정은 향후 Spinon 구현 결과에서 별도로 수행한다.

## 구현과 검증 관문

| 관문 | 통과 조건 | 실패 처리 |
| --- | --- | --- |
| Typed cascade | 네 flow 값과 auto-track 길이의 computed 결과가 Chromium fixture와 일치한다. | raw declaration parsing이나 cascade 누락을 고친다. |
| Placement | definite-first, secondary-locked, sparse/dense cursor, 두 flow 방향, cell occupancy가 기준과 일치한다. | Taffy 결과를 사후 보정해 차이를 숨기지 않고 최소 fixture로 축소한다. |
| Implicit tracks | 생성 축, 개수, 단일 auto-track sizing이 Chrome과 일치하고 상한은 Taffy 호출 전에 거부한다. | clamp·overflow·반복 pattern 성공으로 오인하면 실패다. |
| 실패 원자성 | invalid declaration·span·track limit·할당 실패는 부분 frame 없이 실패하고, 같은 worker에서 다음 정상 layout이 복구된다. 구현 뒤 실제 Vec 예약 지점 여섯 곳(`VisibleItemList`, `RowLockedCursor`, `ExpandedRows`, `ExpandedColumns`, `OccupancyRows`, `OccupancyIntervals`)을 inventory화해 각 호출에 allocator failure를 주입했다. 각 정확한 오류와 동일 worker 복구를 확인했다. [실행 로그와 한계](../spec/internal/evidence/c11-2-3-grid-allocation-failure-2026-10-13.md)를 따른다. | 테스트하지 않은 allocator site나 recovery를 완료로 표시하지 않는다. |
| Rust·binding | layout/style-to-layout/runtime/FFI 검사와 C11.2.2 regression을 통과한다. | 이전 profile의 성공 의미를 바꾸지 않는다. |
| Android | SM-S731N 실기기에서 25 scene·107 node, 성공 19·기대 거부 6의 geometry·draw contract를 비교한다. | PixelCopy는 별도 행렬이므로 이번 결과에는 포함하지 않는다. |
| iOS | iPhone 17 Pro/iOS 26.2 Simulator에서 같은 bundle·scene geometry·draw contract를 비교한다. | Simulator 결과를 실기기나 Metal surface pixel readback으로 승격하지 않는다. |
| 표면 픽셀 | 별도 [C11.2.3 모바일 표면 픽셀 계획](c11-2-3-mobile-surface-pixels.md)에 따라 Android 실기기 PixelCopy와 iOS Simulator drawable readback을 Chrome DPR 1과 exact 비교했다. 각 플랫폼 성공 19 scene·99 표본이 모두 일치했고 거부 6 scene은 readback을 요청하지 않았다. | 고정 제한 profile의 pixel gate를 통과했다. 전체 CSS Grid·WPT·iOS 실기기·compositor 이후 픽셀은 별도 범위다. |
| WPT·지원표 | 후보별 Chrome oracle과 Spinon 실행 결과를 분리하고, 실제 구현된 declaration·오류 경계를 API/지원 명세에 연결한다. | Chrome runner PASS를 Spinon 적합성으로 부르지 않는다. |
| 저장소 hygiene | workspace test, CSS reference suite, Clippy `-D warnings`, `cargo fmt --all -- --check`, diff 검사를 기록한다. | 실행 로그가 없는 검증은 통과로 쓰지 않는다. |

WGPU surface 비교는 Android `PixelCopy`와 iOS Metal drawable에서 scene별 interior sample을 모은다. iOS drawable 결과는 OS compositor 제출 전 표본이다. 이 작업에서 성능 우위, 전체 Grid/CSS 지원, 전체 WPT 적합성을 주장하지 않는다. 공개 지원 표의 현재 상태는 구현이 아닌 명세 계약을 기준으로 갱신한다.

구현 변경, 고정 fixture, Rust 검사 및 두 플랫폼 실행 결과는 [구현 실패 관점 검토](../spec/internal/evidence/c11-2-3-dense-column-placement-implementation-review-2026-10-13.md), [표면 pixel 구현 검토](../spec/internal/evidence/c11-2-3-mobile-surface-pixels-implementation-review-2026-10-13.md), [최신 Android 실행](../spec/internal/evidence/c11-2-3-dense-column-placement-android-physical-runtime-matrix-2026-10-13/c1123-a-mv5yld3e-4ded3cc6/README.md), [최신 iOS 실행](../spec/internal/evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv5ymb0x-10eed1fb/README.md)에 기록한다. 계획 검토와 구현 검토는 분리한다.

## 실행 결과와 남은 경계

2026-10-13 수정된 현재 소스에서 Android SM-S731N 실기기와 iPhone 17 Pro/iOS 26.2 Simulator가 각각 25 scene·107 node를 실행했다. 성공 19 scene은 Chrome 154 DPR 1 기준과 geometry 최대 오차 0 CSS px, 표면 99/99 RGBA exact였고, 6개 거부 scene은 오류 code·NodeId·property와 완성 frame 및 non-empty draw 부재를 확인했다. 새 run은 Android density가 아닌 실제 PixelCopy 크기/viewport 비율을 사용한다. 두 실행 모두 scene마다 새 앱 process를 사용했다. [최신 Android 결과](../spec/internal/evidence/c11-2-3-dense-column-placement-android-physical-runtime-matrix-2026-10-13/c1123-a-mv5yld3e-4ded3cc6/README.md) · [최신 iOS 결과](../spec/internal/evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv5ymb0x-10eed1fb/README.md) · [표면 구현 검토](../spec/internal/evidence/c11-2-3-mobile-surface-pixels-implementation-review-2026-10-13.md). 여섯 Vec 예약 실패 지점은 오류 반환 및 동일 worker 복구를 확인했다. 전역 allocator OOM 복구는 보장하지 않는다. iOS 실기기, 전체 WPT/Spinon 적합성, 전체 CSS Grid 지원, compositor 이후 픽셀은 미완료다.

## 이번 범위에서 제외

negative implicit tracks, explicit line에서 implicit track 생성, named lines/areas, `repeat()`·auto-repeat, 반복 `grid-auto-*` track pattern, `auto`·`%`·`fr`·`minmax()`·`fit-content()` implicit sizing, nonzero `order`, intrinsic/text/replaced item, auto minimum size, baseline/alignment 확장, positioned child, writing mode·RTL, scroll/overflow, subgrid, masonry, full WPT runner 및 성능 우위 주장은 범위 밖이다. 다음 단계는 해당 선행 계약이 마련된 뒤 별도 계획으로 나눈다.
