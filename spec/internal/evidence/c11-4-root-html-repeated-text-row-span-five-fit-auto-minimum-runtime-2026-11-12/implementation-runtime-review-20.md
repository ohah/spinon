# C11.4 five-fit row-span 구현·runtime 실패 관점 검토 20개

검토일: 2026-11-12

검토 기준: `feat/c11-4-five-fit-row-span-20261111`의 로컬 구현 변경과 이 문서에 연결된 Chrome 157 RuntimeSession matrix·Android/iOS 기본 route smoke. 계획 검토 20개와 분리한 구현/runtime 검토다.

| # | 실패 관점 | 코드·실행 대조와 판정 |
|---:|---|---|
| 1 | style projection만 허용하거나 layout admission만 허용해 경로별 지원 범위가 달라지는 경우 | 두 계층에 별도 five-fit row-span predicate를 두고 같은 cap multiset·singleton tail을 검사한다. projection/layout profile 회귀와 RuntimeSession matrix를 통과했다. |
| 2 | cap의 authored 위치가 바뀔 때 track의 상한이 다른 위치로 이동하는 경우 | 허용 cap을 `GridTrackSize`별로 추적하고 위치 정보는 입력 track 순서에 남긴다. 6개 fit 위치와 cap order를 포함한 2,160 track list가 matrix에 들어 있다. |
| 3 | 같은 cap을 중복하고 다른 cap 하나를 빼도 다섯 개라는 이유로 승인되는 경우 | `caps_seen` 중복 검사를 하고 다섯 항목이 각각 한 번인지 확인한다. 중복·누락 negative profile tests를 통과했다. |
| 4 | singleton tail이 없거나 두 개 이상이어도 지원하는 경우 | 6개 member에서 다섯 cap 외 `auto|min-content|max-content`가 정확히 하나여야 한다. 누락·복수·잘못된 tail tests가 이를 확인한다. |
| 5 | 퍼센트 cap의 basis가 불확정이거나 잘못된 축의 크기를 쓰는 경우 | percentage는 definite `grid_height`와 곱하고 비유한·음수 결과는 LayoutUnit 변환에서 거부한다. 고정 220px grid의 Chrome matrix가 `%` cap 전체 순열을 대조한다. |
| 6 | `calc()`를 제한 profile 밖에서 조용히 계산하거나 잘못 정규화하는 경우 | sizing simulator는 unresolved `Calc`를 거부한다. 사전 비교·계획 범위의 px/percentage typed cap 외 입력은 두 admission test 계층에서 거부한다. |
| 7 | row list의 explicit/implicit 경계가 한 track 밀리는 경우 | explicit `10px` row만 non-intrinsic이고, 초기 visible track이 정확히 7개인지 검사한다. matrix의 34,560 scene에서 used rows와 item frame을 Chrome에 비교했다. |
| 8 | 다른 grid 또는 중복 trace를 잘못 선택하는 경우 | grid `NodeId`로 trace를 걸러 정확히 하나만 허용한다. 누락·중복이면 simulation을 거부한다. |
| 9 | rows 이외 axis 또는 이전 sizing pass를 사용하는 경우 | row axis 중 sequence가 가장 큰 pass를 선택하고 initialize stage가 없으면 거부한다. 이후 각 intrinsic span batch에서 trace checkpoint를 확인한다. |
| 10 | non-zero gutter를 일반 row처럼 계산하거나 gutter 공간을 숨기는 경우 | visible track에서 gutter를 제외하되 gutter의 base/growth limit이 0.01px를 넘으면 거부한다. 이 경계는 1/64px replay 전에 확인된다. |
| 11 | row placement line의 1-based/0-based 변환이나 span 끝을 잘못 계산하는 경우 | start line에서 checked subtraction을 사용하고 `Span`·`Line`만 해석한다. 빈 범위, track 밖 범위, 기대 span 불일치는 모두 거부한다. |
| 12 | flex track을 가로지르는 intrinsic contribution을 이 algorithm으로 처리하는 경우 | `crosses_flexible_track`이면 거부한다. 이 제한에서 flex 분배를 일반 track 성장으로 오인하지 않는다. |
| 13 | min/content/max contribution 단계 순서가 바뀌어 결과가 달라지는 경우 | `Minimum → ContentBasedMinimum → MaxContentMinimum → IntrinsicMaximum → MaxContentMaximum` 순서를 고정한다. 각 batch 뒤 Taffy stage와 비교한다. |
| 14 | 사라진 item ID 또는 다른 span의 contribution을 합산하는 경우 | contribution의 item을 입력 map에서 찾아 row range와 예상 span을 대조한다. 어느 하나라도 불일치하면 전체 simulation을 거부한다. |
| 15 | fit-content growth limit을 base-size growth에도 적용해 자동 최소가 잘리는 경우 | base-size phase와 growth-limit phase의 potential을 분리하고, min contribution으로 base가 cap을 넘은 경우 limit을 base까지 올린다. 34,560 candidate scenes의 payload-floor 위반은 0건이다. |
| 16 | zero 증가 단계가 기록되지 않아 무한 growth limit이 그대로 남는 경우 | 영향받은 track은 증가량 0도 계획하고 commit한다. singleton `max-content` tail의 세 track-list 회귀와 `zero-growth-limit-after-spanning-minimum` 테스트가 통과했다. |
| 17 | `min-content` 또는 `max-content` singleton tail을 `auto`로 취급하는 경우 | 각 tail의 min/max sizing function을 별도로 만든다. 세 `max-content` 배치 경계가 RuntimeSession regression으로 고정되어 통과했다. |
| 18 | intrinsic sizing 후 남은 무한 growth limit이 row 결과에 그대로 노출되는 경우 | sizing 완료 때 무한 limit을 현재 base size로 확정하고 resolved limit까지 행을 최대화한다. 최종 pre-alignment trace와 결과를 대조한다. |
| 19 | `align-content:stretch`가 fit cap row까지 늘리거나 나머지 1/64px를 잘못된 row에 주는 경우 | free space는 auto row에만 배분하고 나머지는 뒤쪽 auto row에 준다. 사전 비교 matrix는 `start|stretch`와 tail별 결과를 포함하며 모든 candidate floor가 보존됐다. |
| 20 | trace 불일치 뒤 부분 frame을 게시하거나 실패 후 같은 세션이 복구되지 않는 경우 | simulation 결과는 모든 stage·최종 track 대조가 끝난 뒤에만 반환한다. runtime atomic rejection/recovery 2개와 trace 7개, 전체 Chrome matrix 4개 청크가 통과했다. |

## 판정

위 20개 관점에서 별도 수정이 필요한 미해결 결함은 발견하지 못했다. 마지막 Chrome 157 matrix 실행은 4/4 청크, 34,560/34,560 scene 통과이며 완료 시간은 549.79초다. RuntimeSession matrix는 일반 CSS Grid나 앱 표면 pixel parity의 증거가 아니다. C11.4/C11 완료 체크와 공식 `78/214`는 유지한다.
