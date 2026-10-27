# C11.4 `fit-content()` min-height 구현·런타임 적대 검토

2026-10-28 후보 구현을 서로 다른 실패 경로 20개로 확인했다. 표의 “코드 경계”는 해당 rejection/검증 경로의 코드 검토이고, “실행”은 기록된 focused test 또는 simulator evidence를 뜻한다. 코드 검토만 한 항목을 fault injection 통과로 확대하지 않는다.

| # | 실패 가설 | 확인 경계와 관찰 | 판정 |
|---:|---|---|---|
| 1 | scene ID 접미사·명령문을 붙여 fixture source allowlist를 우회한다 | FFI source는 ID 완전 일치로만 반환한다. allowlist test는 `fit-content-min-height-wpt;throw 1`을 거부한다. | 통과 · 실행 |
| 2 | root ID를 바꿔 비슷한 DOM이 이 adapter를 잘못 선택한다 | Runtime `prepare`는 root id, HTML namespace, `div`, root parent, computed profile을 함께 확인한다. | 통과 · 코드 경계 |
| 3 | 잘못된 namespace/tag 또는 document 외부 root를 그대로 layout한다 | 정확한 namespace/tag 및 `HostParent::Root`가 아니면 `unsupported`를 반환한다. | 통과 · 코드 경계 |
| 4 | Grid에 두 번째 direct item을 추가해 implicit row count가 달라진다 | root child count를 정확히 1로 요구하고 parent 관계도 확인한다. | 통과 · 코드 경계 |
| 5 | item의 ID·tag·parent를 바꿔 다른 node를 측정한다 | item identity와 root의 direct child 소유 관계를 다시 확인한다. | 통과 · 코드 경계 |
| 6 | item의 child를 누락·추가하거나 다른 parent로 옮긴다 | item child count 1, `HostParent::Node(item)`, child identity를 검사한다. | 통과 · 코드 경계 |
| 7 | 예상 밖의 computed style node 또는 중복 style을 붙인다 | style node ID 집합이 세 fixture node와 정확히 일치하고 element 개수 중복도 거부한다. | 통과 · 코드 경계 |
| 8 | Grid-track sizing profile 외의 computed style을 전달한다 | `RuntimeBlockFlexGridTrackSizingV1`과 layout output revision을 함께 고정한다. | 통과 · 코드 경계 |
| 9 | root의 `height`, `min-height`, `grid-auto-rows`, align 또는 explicit rows를 바꾼다 | computed property·typed dimensions·Grid style을 전부 검사한다. | 통과 · 코드 경계 |
| 10 | item의 min-height, placement, margin, padding, border를 바꿔 intrinsic contribution을 바꾼다 | auto dimensions·auto placement·zero spacing·zero border 조건을 확인한다. | 통과 · 코드 경계 |
| 11 | nested child의 폭·높이·spacing 또는 border를 바꿔 10px contribution을 바꾼다 | child computed style은 60×10px와 zero spacing/border만 허용한다. | 통과 · 코드 경계 |
| 12 | typed `GridContainerInput`에서 `fit-content(40px)`를 다른 값이나 비정상 수로 대체한다 | runtime adapter unit test에서 41px limit와 explicit row를 거부한다. layout oracle는 음수·NaN·무한대 fit-content limit를 `InvalidGridTrack`, min-height를 `InvalidStyle`로 거부하는지 확인한다. | 통과 · 실행 |
| 13 | root 외 Grid input을 추가하거나 auto-flow/placement mode를 바꾼다 | 단일 Grid input 및 `Row` / `TrackSizing` / columns·rows·line names를 고정한다. | 통과 · 코드 경계 |
| 14 | style, host document, tree, environment revision 중 하나가 stale하다 | snapshot revision과 `LayoutInputRevision`, runtime key가 모두 일치해야 한다. 두 simulator의 environment-to-draw identity도 일치했다. | 통과 · 실행 + 코드 경계 |
| 15 | layout output에서 root/item/child frame을 누락·중복·추가한다 | 관련 frame 개수와 node별 중복을 거부하며 세 frame을 전부 요구한다. | 통과 · 코드 경계 |
| 16 | root/item/child frame 좌표나 크기가 달라졌는데 성공 처리한다 | exact CSS-px frames `[60×300, 60×10, 60×10]`을 비교했다. 두 simulator 모두 일치했다. | 통과 · 실행 |
| 17 | trace를 다른 Grid가 소유하거나 pass가 빠지거나 추가된다 | root trace 하나, Columns/Rows pass 각 하나를 요구한다. | 통과 · 코드 경계 |
| 18 | row sizing stage가 누락·중복·역순이거나 예상 밖 stage가 추가된다 | oracle, runtime adapter, Android/iOS matcher와 capture가 전체 6개 stage의 순서와 각 단계 row base를 정확히 고정한다. | 통과 · 실행 + 코드 경계 |
| 19 | 초기 0px와 최종 10px을 혼동하거나 stretch에서 row가 늘어난다 | 초기 `Initialize` 0px 이후 sizing stages의 `[10px]`을 검사한다. 두 simulator 모두 10px row를 반환했다. | 통과 · 실행 |
| 20 | 환경 재계산 뒤 다른 revision을 draw하거나 root 밖까지 칠한다 | surface generation/sequence/revisions, presented boxes, child/root-background/outside-root 픽셀을 확인했다. Android·iOS 모두 3/3 정확히 일치했다. | 통과 · 실행 |

실행 근거는 [Android/iOS runtime README](./README.md), 각 simulator `runtime-*.log`와 `result.json`이다. 구현 중 capture harness가 초기 sizing stage를 최종 10px으로 오인하고 iOS presentation log의 필드 순서를 읽지 못하는 두 검증기 결함을 찾아 수정했다. 두 문제는 엔진 layout 결과 결함이 아니었으며, 수정 뒤 두 simulator capture가 모두 통과했다.

추가 artifact 경계 검증에서 제품 `c04-runtime-gpu` archive에는 fixture symbol/scene marker가 없고 fixture-test positive control에는 둘 다 있었다. 이는 앱 번들 전체 크기나 release artifact 검증으로 확대 해석하지 않는다.
