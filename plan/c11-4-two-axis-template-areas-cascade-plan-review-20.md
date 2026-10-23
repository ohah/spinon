# C11.4 양 축 auto-repeat + areas 계획 cascade 수정 적대 검토 20개

**검토 대상:** [수정된 집중 계획](./c11-4-two-axis-template-areas.md), `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`, `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs`, `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs`.

**수정 이유:** 구현 전 소스 추적에서 style projection 외에 cascade도 같은 조합을 거부하는 것을 확인했다. 코드는 아직 변경하지 않았다. 이번 검토는 기존 20개 계획 검토와 별도로 cascade gate·profile 경계를 다룬다.

| # | 실패 관점 | 수정 계획의 방어선 | 판정 |
| --- | --- | --- | --- |
| 1 | projection 거부만 지워도 cascade가 `ComputedCssGridLayout` 생성을 먼저 막는다. | cascade와 projection 두 guard를 독립 제거 대상으로 문서화했다. | 통과 |
| 2 | cascade 거부를 무조건 제거해 auto-repeat이 비대상 CSS profile로 유출된다. | `allow_auto_repeat`가 true일 때만 양 축 + areas 조합을 허용한다. | 통과 |
| 3 | `allow_auto_repeat`가 여러 profile에서 true라고 잘못 가정한다. | `element.rs` 호출부에서 true가 `RuntimeBlockFlexGridTrackSizingV1` 단 하나인지 확인하고 테스트한다. | 통과 |
| 4 | TrackSizing profile에서 area parsing 자체가 꺼져 있어 computed metadata가 없다. | `allow_integer_repeat` 및 `allow_auto_repeat` 인자를 분리 추적하고 실제 `ComputedCssGridTemplateAreas` 생산을 검사한다. | 통과 |
| 5 | columns auto-repeat은 보존되지만 rows metadata가 누락된다. | cascade 출력에서 양 축 `ComputedCssGridAutoRepeat` 모두를 동시에 단언한다. | 통과 |
| 6 | rows metadata만 보존되고 columns가 누락되거나 교환된다. | scene마다 authored axis mode/count와 projected axis metadata를 따로 비교한다. | 통과 |
| 7 | cascade가 양 축 정보를 보유해도 style projection gate가 계속 거부한다. | projection의 non-target profile gate는 유지하면서 대상 TrackSizing path만 해제한다. | 통과 |
| 8 | projection 성공 뒤 `GridContainerInput`이 areas를 버린다. | projected area rectangle과 line aliases를 검증하고 Taffy style 경계까지 추적한다. | 통과 |
| 9 | 한 축 repeat metadata가 양 축 입력으로 복제된다. | Columns/Rows metadata identity와 used-track outputs를 독립 검사한다. | 통과 |
| 10 | `RuntimeBlockFlexGridTemplateV1`이 두 repeats plus areas를 새로 수용한다. | TemplateV1 cascade 및 projection 거부를 negative control로 보존한다. | 통과 |
| 11 | `RuntimeBlockFlexGridV1` 또는 placement-only profile이 area parsing을 받는다. | profile-specific `allow_integer_repeat`와 `allow_auto_repeat` 호출값을 유지하고 profile tests를 둔다. | 통과 |
| 12 | cascade partial failure가 이전 computed layout revision을 새 조합 결과처럼 게시한다. | 실패 입력에서 computed snapshot/layout revision과 frames가 바뀌지 않는 원자성 regression을 둔다. | 통과 |
| 13 | CSS invalid area/auto-repeat 값이 두 gate 변경으로 다른 오류가 된다. | valid product combination 외 malformed/unsupported values의 기존 구체 진단을 회귀한다. | 통과 |
| 14 | `grid-auto-*` projection이 repeat을 두 축에서 받으며 뜻이 넓어진다. | fixed `16px`/`12px` fixture 이외의 sizing grammar는 계속 reject한다. | 통과 |
| 15 | no-repeat + areas 또는 single-axis repeat + areas가 cascade 수정으로 회귀한다. | 기존 13-scene area matrix를 exact ID regression으로 유지한다. | 통과 |
| 16 | dual-axis repeat without areas가 areas branch에 종속되어 회귀한다. | 기존 12-scene dual-axis matrix를 분리 실행해 exact ID regression으로 유지한다. | 통과 |
| 17 | area보다 repeat count가 큰 경우 계산에서 둘 중 작은 축 값이 선택된다. | 2×2-short-area case와 full columns/rows used lists를 대조한다. | 통과 |
| 18 | area-added track 생성 후 cascade만 통과해 frame 결과를 과장한다. | 9-scene runtime, Taffy track list, item frame 및 draw/present gate를 모두 요구한다. | 통과 |
| 19 | cascade 수정 뒤 fixture symbol이 product artifact에 들어간다. | default archive negative 및 opt-in fixture-test positive control을 각각 검증한다. | 통과 |
| 20 | 추가 계획 수정 검토를 구현 runtime 검토로 중복 계산한다. | 구현 뒤 새 code/runtime 20관점과 구현 PR diff 30관점을 별도로 수행한다. | 통과 |

## 판정

`computed_grid_layout`은 axes를 계산한 뒤 area와 두 repeat이 같이 있으면 reject한다. call site에서 `allow_auto_repeat`는 `RuntimeBlockFlexGridTrackSizingV1`에만 true다. 그러므로 cascade guard는 이 parameter를 기준으로 제한 완화하고, style projection은 해당 profile guard를 그대로 둬야 한다. 이 계획 수정 검토는 구현 승인을 위한 선행 조건이며 코드 실행 결과를 주장하지 않는다.
