# C11.4 nested Grid depth-two 구현 적대 검토 20개

계획 검토와 별도로 runtime adapter, projection, track trace, simulator harness, artifact 설정을 공격 관점에서 확인했다. 각 항목은 서로 다른 입력 불변식 또는 실패 경계를 다룬다. 검토 중 빠진 leaf-to-inner contribution assertion과 root ownership/`data-node` 확인을 보강하고, 관련 검사를 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 |
| --- | --- | --- |
| 1 | 비등록 `c114-nested-depth-*` scene이 기존 일반 fixture로 흘러가 일부 layout을 내는가 | FFI allowlist와 runtime adapter가 등록 ID 네 개 외 nested-depth prefix를 거부한다. FFI unit test에서 depth-three와 미등록 ID가 `None`임을 확인했다. |
| 2 | 다른 namespace/tag/style profile의 root가 같은 adapter를 탈 수 있는가 | HTML namespace, `div`, `RuntimeBlockFlexGridTrackSizingV1`를 확인한다. 다른 profile의 intrinsic Grid width는 projection regression에서 거부된다. |
| 3 | scene root가 다른 node 밑에 있거나 render marker가 빠진 경우를 허용하는가 | 초기 검토에서 root parent와 `data-node` 표식 확인이 빠져 있음을 찾았다. root ownership=`HostParent::Root`와 빈 `data-node` 검사를 추가했고 Android/iOS 실행을 다시 통과했다. |
| 4 | depth-one 입력에 중간 Grid가 끼거나 depth-two owner chain이 섞이는가 | depth별 예상 ID와 `root → middle → inner → leaf` identity를 순서대로 확인한다. |
| 5 | Grid나 leaf에 추가 sibling/descendant가 있어도 통과하는가 | 각 Grid는 자식 하나, leaf item은 direct text node 하나만 허용한다. |
| 6 | middle/inner Grid 또는 leaf가 잘못된 parent에 매달리는가 | snapshot parent를 각 단계에서 exact handle과 비교하고, node marker를 각 expected element에서 검사한다. |
| 7 | inline text가 다른 element로 감싸져 item identity를 유지하지 못하는가 | leaf는 정확한 HTML `div` ID이며 innermost Grid의 직접 자식이어야 한다. |
| 8 | text node가 달라지거나 여러 text node로 분리되는가 | 단일 direct text node의 값이 `XX XXXXXXX`와 exact match해야 한다. |
| 9 | font/line-wrap 속성이 바뀌어도 같은 70/100px 수치로 판정하는가 | font family/size/weight/style, line-height, white-space, wrapping, direction, break, spacing 관련 값과 auto dimensions/zero edges를 확인한다. |
| 10 | 시스템 font fallback이 Ahem 수치를 우연히 맞추는가 | `TextLayoutEngine::new_without_system_fonts()`에 저장소 Ahem만 등록하며 intrinsic source의 font registry revision 1을 검사한다. |
| 11 | width constraint와 Columns track 함수가 서로 다른 min/max mode를 쓰는가 | 각 Grid에서 `width`와 유일 Columns track이 같은 `MinContent` 또는 `MaxContent`여야 한다. |
| 12 | intrinsic block-size가 inline-axis fixture에 섞이는가 | Grid height/Rows track은 `auto`; intrinsic min/max/fit-content height 조합은 projection 허용 조건에 들지 않는다. |
| 13 | padding, margin, border, gap 또는 sizing mode가 기대값에 더해지는가 | `border-box`, min/max/flex-basis defaults, zero margin/padding/border와 0 row/column gap을 adapter에서 고정한다. |
| 14 | auto-repeat, template areas, implicit tracks, line names, 다른 auto-flow가 결과를 바꾸는가 | 한 track list 외 auto-repeat/areas/line names와 non-default auto track/flow 구성을 fail closed한다. |
| 15 | 명시 placement가 child contribution 순서나 span을 바꾸는가 | 각 Grid item의 row/column start/end가 모두 `Auto`여야 하며 parent span 1 contribution을 확인한다. |
| 16 | 중간 Grid trace 누락 또는 일부 축만 계산된 상태를 성공으로 보는가 | 각 Grid마다 trace 하나와 Columns/Rows 각각 한 pass, final stage, 활성 track 하나를 요구한다. |
| 17 | gutter/collapsed track을 본 track으로 세거나 예상 폭과 다른 값을 숨기는가 | gutter/collapsed 항목은 제외한 뒤 실제 활성 track 개수를 1로 assert하고 base size를 1/64px 허용치로 비교한다. |
| 18 | leaf intrinsic source가 맞아도 leaf-to-inner 또는 중간 parent 전달이 끊기는가 | 초기 검토에서 중간 Grid 사이만 assertion되고 마지막 text item contribution이 빠진 것을 찾았다. direct test와 simulator verifier가 모든 Grid 단계에서 직계 child Grid 또는 leaf text의 kind/span/value를 검사하도록 수정했다. |
| 19 | text intrinsic min/max metrics 또는 source order/font revision이 바뀌는가 | min `70×20`, max `100×10`, source order 0, exact inner-grid owner, text source kind, revision 1을 확인한다. |
| 20 | target 별 실행 인자가 달라도 결과를 통과시키거나 fixture가 제품 archive에 들어가는가 | Android/iOS allowlist, root frame, node order/count, full trace chunk, draw/present를 검사했다. 기본 `c04-runtime-gpu` archive에는 네 scene ID가 없고 fixture-test positive control에는 네 ID가 모두 있다. |

재검증은 [runtime focused test](./runtime-focused-test-final.log), [projection regression](./projection-focused-test-final.log), [FFI allowlist test](./ffi-focused-test-final.log), [workspace Clippy](./workspace-clippy-final.log), [fixture-feature Clippy](./fixture-clippy-final.log), [Android AVD/iOS Simulator 실행](./README.md)에서 확인할 수 있다. 검증 범위는 fixed fixture에 한정되며 이 검토로 C11.4 전체를 완료 처리하지 않는다.
