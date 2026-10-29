# C11.4 Grid item visibility:collapse 계획 · 독립 실패 관점 20개

검토 대상은 [계획](./c11-4-grid-visibility-collapse.md), [10-scene CSS fixture](../tests/fixtures/css/c11/grid-visibility-collapse-c114-precomparison.html), [Chrome observation과 독립 기대값](../spec/internal/evidence/c11-4-grid-visibility-collapse-precomparison-2026-10-29/README.md), 현재 C11.4 cascade 거부 테스트 및 기존 hidden runtime fixture다. 구현 전 계획·oracle·범위만 검토했다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | visibility:collapse를 모든 formatting context에서 동일하게 처리할 수 있음 | CSS Display Level 3 §4의 일반 fallback을 확인하고 table/flex의 context-specific 규칙을 분리했다. 통과. |
| 2 | Grid에 없는 특수 collapse algorithm을 임의로 추정할 수 있음 | CSS Grid Level 1 2025 CRD의 item sizing 절차에 collapse 전용 algorithm이 없음을 확인했다. hidden-equivalence는 spec 조합에서 도출한 inference로 표시했다. 통과. |
| 3 | Flexbox strut 및 relayout 동작이 공유 경로에 섞일 수 있음 | 계획은 Grid-only route를 요구하고 Flexbox 행렬을 이번 계약에서 제외한다. 통과. |
| 4 | computed value를 hidden으로 덮어 써 cascade 결과를 잃을 수 있음 | snapshot의 computed collapse 보존과 Grid layout/render boundary에서의 사용 의미를 분리했다. 통과. |
| 5 | collapse item을 display:none처럼 formatting context에서 제거할 수 있음 | hidden 대조군은 [50,50] track과 100px item을 유지하고, 별도 none 대조군은 [0,0]과 0px item을 요구한다. 통과. |
| 6 | spanning intrinsic contribution을 두 track에 잘못 분배할 수 있음 | 100px max-content contribution, 두 minmax(auto,max-content) track 및 80px definite axis를 고정해 각 track 50px를 독립 계산했다. 통과. |
| 7 | Columns만 구현되고 Rows의 sizing property가 빠질 수 있음 | 동일 contribution과 root basis를 축별로 바꿔 각 5개씩, visible positive control을 포함해 총 10개 scene으로 고정했다. 통과. |
| 8 | row visible-descendant가 추가 contribution을 만들어 track 기대가 달라질 수 있음 | visible child 8px와 뒤따르는 flow content 92px를 합쳐 총 100px로 정규화했다. 통과. |
| 9 | collapse 부모가 명시적 visible descendant까지 숨길 수 있음 | 양축 visible-descendant scene에서 computed visible, 8×8px frame 및 descendant hit target을 각각 요구한다. 통과. |
| 10 | hidden/collapse hit-test가 보이지 않는 item을 반환할 수 있음 | 같은 좌표의 underlying scene root를 기대값으로 두고 visible descendant는 별도 target으로 구분했다. 통과. |
| 11 | layout은 맞지만 collapse item의 red content가 paint될 수 있음 | 구현 acceptance에 Android PixelCopy와 iOS Metal drawable의 background/red/blue 표본을 두고, item이 root 밖으로 넘치는 경계도 검사한다. 통과. |
| 12 | 현재 hidden-only route가 collapse를 우연히 허용하거나 다른 node에 허용할 수 있음 | 기존 runtime helper는 column-visibility-collapse를 unknown으로 거부한다. 계획은 direct target item과 computed owner/property를 다시 검사하고 정확한 새 ID만 허용한다. 통과. |
| 13 | 새 값이 일반 author CSS admission으로 번질 수 있음 | 범위를 opt-in C11.4 fixture profile로 한정하고 generic admission 및 다른 Grid profile 확장을 금지했다. 통과. |
| 14 | snapshot, source order 또는 layout revision이 섞인 결과를 노출할 수 있음 | 계획은 source/tree/style identity와 revision 불일치, 누락·중복 node를 부분 결과 없이 거부한다. 통과. |
| 15 | visible child의 값만 확인하고 자식 paint/hit 경계를 잘못 숨길 수 있음 | layout frame, computed visibility, hit target, surface color를 별도 계층에서 검사하도록 했다. 통과. |
| 16 | fixture route가 기본 product artifact에 포함될 수 있음 | default와 opt-in fixture artifact에서 route marker/symbol을 부정·긍정 대조한다. 통과. |
| 17 | Android와 iOS 결과가 같은 장면·revision에서 나온 것인지 확인하지 않을 수 있음 | 각 simulator의 scene ID, source revision, computed value, tracks, frame, trace 및 draw/present를 scene별로 고정한다. 통과. |
| 18 | Chrome 기준 결과를 Spinon 구현 또는 WPT 통과로 잘못 부를 수 있음 | 사전 비교 README는 Agent Browser를 oracle로만 구분하고 구현/WPT 결과 주장을 금지한다. 통과. |
| 19 | document flow에서 각 scene의 절대 y 위치가 달라 oracle 검증이 깨질 수 있음 | 최초 검증은 절대 좌표 y=0을 기대해 hidden-column에서 실패했다. item/child를 root/item 기준 상대 좌표로 비교하고 fixture·screenshot SHA-256과 고정 scene 순서를 검사하도록 고쳤다. 수정 검증은 10/10 scene과 pixel 14/14 통과했다. |
| 20 | 이 제한 slice를 C11.4 또는 C11 전체 완료로 승격할 수 있음 | 계획은 WPT, 일반 CSS admission, Flex/table collapse, 실기기/hardware GPU, 전체 C11.4를 제외하고 공식 72/213 및 C11 checkbox를 유지한다. 통과. |

**결과:** 계획 검토 20/20 완료, 미해결 계획 차단 항목 0개. 사전 비교 validator의 절대좌표 가정은 발견 후 수정했고, 수정본은 10/10 scene·14개 screenshot sample·입력 hash·화면 hash를 확인했다. Chrome observation은 구현 근거가 아니다. 구현 전 별도 계획 PR과 별도 PR-diff 20관점 검토가 남아 있다.
