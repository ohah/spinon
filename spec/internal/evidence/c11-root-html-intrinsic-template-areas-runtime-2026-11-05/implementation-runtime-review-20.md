# C11.1 intrinsic track + template-area 구현/runtime 실패 관점 검토 20개

**대상:** `root_html_intrinsic_named_area.rs`, `root_html_named_area.rs`, `grid_values.rs`, `grid.rs`, `root_html_one_axis_placement.rs`와 고정 Root HTML runtime matrix.

| # | 공격 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | 같은 snapshot에 area template을 가진 visible Grid가 둘 이상 있음 | candidate validator가 area owner 하나와 전체 Grid container 하나를 요구한다. 추가 owner는 거부된다. |
| 2 | nested Grid 또는 다른 Grid item owner가 map 검증을 우회함 | 각 item의 `grid_item_owners`를 확인하고 전체 owner item 집합과 직계 child 집합을 비교한다. |
| 3 | Grid 아래 non-whitespace anonymous text가 임의 auto item이 됨 | ASCII whitespace만 무시한다. 다른 텍스트는 `grid-item` 오류로 막는다. |
| 4 | `display:none` direct child가 area slot으로 남거나 hidden scene을 바꿈 | hidden direct child는 `display` 오류로 거부한다. |
| 5 | SVG/foreign namespace 또는 `div` 아닌 요소가 통과함 | HTML namespace와 local name `div`를 모두 확인한다. |
| 6 | nested element child가 텍스트 전용 direct item 제한을 우회함 | item child가 모두 text node인지 검사하고 nested element를 거부한다. |
| 7 | area마다 item이 없거나 area보다 direct item이 더 많음 | 누락 및 초과 cardinality를 각각 검사하고 partial frame을 허용하지 않는다. |
| 8 | 두 item이 한 이름을 차지하고 다른 area가 비어 있음 | `used_areas` bitmap 중복/누락을 거부한다. |
| 9 | placement 네 edge 중 일부만 이름이 있거나 endpoint 이름이 다름 | 네 computed endpoint가 동일한 `NamedLine`이어야 한다. 그 밖의 placement는 거부한다. |
| 10 | 알 수 없는 named area가 implicit line으로 처리됨 | 정렬된 map에 `binary_search`가 실패하면 node/property 오류다. unresolved area runtime test가 이를 확인한다. |
| 11 | malformed CSS area가 cascade에서 빠진 뒤 named-line fallback으로 성공함 | no-area explicit/one-axis fallback에서 named-line/span을 차단한다. 잘못된 선언·unresolved area atomic tests가 통과한다. |
| 12 | `minmax(auto, 40px)`가 기존 unsupported auto-minimum 범위를 우회함 | intrinsic named-area template validator가 fixed/percentage maximum의 auto-minimum 조합을 거부한다. runtime negative로 partial scene 부재를 확인했다. |
| 13 | area 행/열 수가 explicit track 개수와 달라 track을 암묵적으로 늘림 | style projection과 layout validator가 intrinsic mode에서 두 축 dimension을 각각 exact 비교한다. |
| 14 | zero/empty map, 10,000 초과 축, area count가 cell 수를 초과함 | non-empty dimension, `GRID_MAX_TRACKS`, cell capacity를 allocation 전에 검사한다. |
| 15 | 중복/빈/NUL area 이름 또는 zero/out-of-range rectangle | name set과 half-open row/column bounds 검사가 거부한다. 단위 행렬에서 duplicate와 dimension failure를 확인했다. |
| 16 | 한 행에서 부분 겹침, multi-row 겹침 또는 겹치는 동일 시작 column | sweep-line이 row 끝 이벤트를 시작 이벤트보다 먼저 처리하고 active interval 이웃을 검사한다. overlap unit test 통과. |
| 17 | 인접한 두 rectangle이 같은 경계에서 잘못 겹침으로 분류됨 | row end를 row start 전에 제거하며 strict `<`/`>` interval 비교를 사용한다. 인접 경계는 허용되고 실제 rectangle 겹침은 거부된다. |
| 18 | 거대 이름/metadata 또는 allocator 실패가 overflow·panic으로 변함 | `checked_mul`/`checked_add`, 1 MiB quota, `try_reserve`를 사용한다. oversized metadata 단위 테스트 통과. |
| 19 | indefinite/zero 크기, min/max override, non-finite gap으로 sizing 값이 퍼짐 | positive finite px 축, 기본 min/max, definite 축 basis의 gap validator를 요구한다. out-of-scope percentage/size controls가 원자 거부된다. |
| 20 | 문서 revision 변경 뒤 이전 area scene이 새 요청에 재사용됨 | runtime의 environment change 후 stale scene 비노출 테스트가 통과했고 app draw marker는 최종 generation 2에서 status 0이다. |

## 실행 결과

- runtime named-area positive/negative matrix: 15/15; [로그](runtime-area-tests.log)
- style-to-layout map/rectangle/quota tests: 5/5; [로그](style-area-tests.log)
- layout intrinsic-track tests: 8/8; [로그](layout-intrinsic-tests.log)
- layout template-area tests: 14/14; [로그](layout-area-tests.log)
- scoped Clippy와 rustfmt 통과; [Clippy](clippy.log) · [rustfmt](rustfmt.log)
- Android API 37 AVD 및 iOS 26.2 Simulator에서 기본 route layout/draw/present 성공; 화면은 visual smoke이며 pixel readback은 하지 않았다.

넓은 Root HTML crate 회귀에는 별도 미해결 실패가 남아 있다. 상세 테스트 이름과 로그는 [runtime evidence](README.md)에 기록했고 focused matrix 통과 수에 합산하지 않았다.
