# C11.4 `grid-layout-auto-tracks` 구현/runtime 실패 관점 검토 20개

고정 WPT slice의 Rust layout/runtime adapter, FFI fixture, Android/iOS matcher와 최종 simulator evidence를 서로 다른 실패 경계에서 검토했다. 계획 검토 표의 항목을 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 허용 장면 ID 뒤에 임의 코드를 붙여 source allowlist를 통과한다 | FFI source unit test가 suffix가 붙은 scene ID를 거부한다 | PASS |
| 2 | 다른 scene ID 요청이 이 fixture source로 fallback된다 | 정해진 scene ID에서만 source가 생성되고 그 외에는 `None`이다 | PASS |
| 3 | 같은 ID를 가진 비-HTML 또는 다른 tag가 adapter에 들어온다 | HTML namespace, `div`, runtime profile, root parent를 먼저 검사한다 | PASS |
| 4 | root의 direct item 순서가 바뀌어 기대 frames가 우연히 맞는다 | direct child count와 A/B/C identity·순서를 고정한다 | PASS |
| 5 | B의 50px content가 다른 parent에 붙거나 sibling이 추가된다 | B child count, ownership, element identity를 검사한다 | PASS |
| 6 | A/B content/C가 다른 텍스트 또는 여러 text node를 사용한다 | 각 owner의 단일 NBSP text node와 parent를 검사한다 | PASS |
| 7 | host system font에 따라 intrinsic contribution이 달라진다 | system font 없이 Ahem fixture font를 등록하고 10px metrics로 측정한다 | PASS |
| 8 | `grid-template` 또는 auto-flow가 다른 계산 입력으로 넘어온다 | computed root style 및 typed `GridContainerInput`을 각각 검사한다 | PASS |
| 9 | item placement, width, edge가 fixture CSS와 달라진다 | A/B/C placement와 text block dimensions·spacing·border를 검사한다 | PASS |
| 10 | 다른 explicit Grid container가 진단 대상에서 누락된다 | observation 시 Grid input이 하나이며 root에만 있는지 검사한다 | PASS |
| 11 | 누락/중복/추가 fixture frame이 5개 결과처럼 보인다 | runtime validator는 관련 frame 수와 고유 node 집합을 모두 요구한다 | PASS |
| 12 | 이전 document/style/environment revision의 completion이 섞인다 | style snapshot, layout source revision과 completion key의 generation·revision을 대조한다 | PASS |
| 13 | trace에 다른 Grid 또는 추가 axis pass가 들어온다 | root trace 1개와 Columns/Rows pass 각 1개만 허용한다 | PASS |
| 14 | track stage 순서가 다르거나 마지막 stretch stage가 빠진다 | intrinsic complete, maximize, flex expansion 순서와 terminal stretch stage를 확인한다 | PASS |
| 15 | gutter/collapsed sentinel을 CSS track으로 세거나 implicit column 크기가 틀린다 | visible track 두 개의 stage bases `[100,50]` / `[10,10]`을 대조한다 | PASS |
| 16 | Minimum contribution이 중복됐지만 같은 값 하나만 발견해 통과한다 | 검토 중 발견한 허용 틈을 고쳤다. Rust·Android·iOS 모두 정확한 Minimum entry 개수와 기대 item/value 집합을 요구한다 | PASS |
| 17 | Android report가 다른 profile/revision/frame을 성공으로 표시한다 | Android matcher가 computed properties, node IDs, frames, revisions, track traces를 검증한다 | PASS |
| 18 | Android 표본 ID/좌표를 바꾸거나 root 경계의 off-by-one을 놓친다 | PixelCopy 입력은 125/149 content와 x150 outside-root를 exact ID·좌표로 제한한다 | PASS |
| 19 | iOS drawable을 compositor 이후 화면으로 오인하거나 sample mismatch를 놓친다 | run-scoped Metal sample 로그가 RGB/A exact 3/3이며 README에 pre-submit 한계를 명시한다 | PASS |
| 20 | 시뮬레이터 캡처가 splash 또는 이전 source revision의 결과다 | 최종 matcher 보정 후 앱을 재빌드·재설치해 새 run ID로 실행하고 최종 screenshot/log를 다시 확인했다 | PASS |

## 판정

구현/runtime 검토 PASS 20/20, PARTIAL 0, FAIL 0. 16번에서 확인한 중복 contribution 허용 가능성은 플랫폼 matcher와 Rust runtime validator를 강화한 뒤 최종 Rust 검사 및 Android/iOS 빌드·실행으로 재확인했다.
