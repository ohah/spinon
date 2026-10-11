# C10.3.5 계획 보정 후 검토

**검토일:** 2026-10-11 · **범위:** C10.3.5 전용 계획, 내부 계약 0056, 고정 Chrome fixture와 모바일 runtime smoke 경계

| # | 독립 실패 질문 | 대조한 조건 | 판정 |
| ---: | --- | --- | --- |
| 1 | 구현 중 상태가 이미 병합된 것으로 오해될 수 있는가 | 계획 상단을 제한 구현·smoke 완료, 최종 검토/PR 진행 중으로 구분했다. | 미병합 상태가 남는다. |
| 2 | 고정 Chrome fixture 크기와 실제 구현 범위가 섞였는가 | Chrome 20 case·70 node와 모바일 subset 3 case·15 node를 따로 적었다. | 전체 browser 기준과 native 범위가 분리된다. |
| 3 | 모바일 fixture가 Chrome 기준과 다른 CSS를 사용하면서 동일 입력이라 주장하는가 | case style은 재사용하고 `background` 색상 shorthand만 제한 runtime profile의 `background-color`로 바꾼다. | 변경이 색상 표현뿐이며 parser 검증이 아님을 명시했다. |
| 4 | 모바일에서 빠진 case가 완료된 것으로 보이는가 | 직접 child, paint 겹침, Block wrapper만 실행한다고 나열했다. | 전체 모바일 행렬은 미실행이라고 남는다. |
| 5 | static-position owner와 containing-block owner를 하나로 합치는가 | direct child, 다른 positioned ancestor, wrapper 조합을 각 단계에서 분리한다. | owner가 다를 때의 좌표 변환 요구가 유지된다. |
| 6 | Block wrapper 후손에게 Flex alignment를 잘못 적용하는가 | wrapper의 Block static-position을 계속 C12.2 규칙으로 둔다. | wrapper의 formatting context를 따로 검증한다. |
| 7 | absolute child가 Flex line와 in-flow frame에 참여하는가 | 추가/삭제 전후 line과 sibling geometry 불변식을 유지한다. | out-of-flow 조건이 완료 gate에 있다. |
| 8 | in-flow `order`가 HostDocument 순서까지 바꾸는가 | 시각 paint order만 order-modified이며 source child 순서를 보존한다고 규정한다. | DOM 유사 조회와 paint 순서가 분리된다. |
| 9 | absolute child의 authored `order`를 Flex item rank로 쓰는가 | absolute child는 Flex item이 아니며 positioned phase에서는 source tree 순서를 따른다. | pinned Chrome/WPT 근거와 구현 테스트가 연결된다. |
| 10 | paint 설명이 stacking context 또는 z-index 지원으로 확대되는가 | 제한 `z-index:auto` phase와 불투명 단색 box로만 범위를 한정한다. | `z-index`, transform, clip, hit-test 미지원이 명시돼 있다. |
| 11 | 중첩 absolute descendant가 부모 Flex order에 섞이거나 두 번 그려지는가 | in-flow subtree와 positioned preorder를 따로 두고 nested descendant 고유성을 검사한다. | 테스트와 완료 조건에서 각각 확인한다. |
| 12 | `display:none` node가 source tree에서 삭제되거나 화면에 남는가 | source identity는 보존하고 layout frame/paint 목록에서 제외하도록 적었다. | document·layout·paint 생명주기 경계가 나뉜다. |
| 13 | 성공 profile이 확장되면서 legacy 경계가 사라지는가 | 여섯 runtime Flex profile과 비-runtime/legacy 거부를 별도로 기록한다. | unsupported profile은 완료 범위로 승격되지 않는다. |
| 14 | 고정 frame 비교가 DPR 좌표를 CSS px 허용치와 혼동하는가 | Chrome DPR 1·2 관측과 모바일 CSS viewport `320×240` 비교를 분리한다. | CSS px `0.5` 허용치가 명확하다. |
| 15 | 모바일 로그의 전체 box 수가 fixture node 수와 혼동되는가 | 15개 fixture node와 runtime root를 더해 16개 frame/box가 된다고 설명한다. | comparator가 root viewport를 별도 검사한다. |
| 16 | Android 에뮬레이터를 실기기로 보고하는가 | SM-S731N, Android 16/API 36, Xclipse 940/Vulkan을 지정했다. | 실제 연결 기기 실행 근거와 맞는다. |
| 17 | iOS Simulator backend를 코드에서만 추정하는가 | 실제 앱 로그의 `backend=Metal`, main-thread surface configuration, presented box를 요구한다. | 런타임 증거가 남는다. |
| 18 | 첫 Android 실패를 지우고 재시도 성공만 남기는가 | `background-position-x` 거부의 초기 로그/캡처를 성공 실행과 별도 보존한다. | 수정 전후가 구분된다. |
| 19 | WPT 파일 경로 인벤토리를 suite pass로 주장하는가 | suite는 실행하지 않았다고 계획/계약/evidence에 반복 표기했다. | 규격 테스트 미실행 경계가 유지된다. |
| 20 | 화면 캡처를 전체 pixel parity나 성능 증거로 확대하는가 | 캡처는 시각 확인, 좌표 비교기는 node geometry, 성능은 별도 미검증이라고 나눈다. | 근거 종류가 서로 대체되지 않는다. |

## 판정

계획 보정 후에도 runtime subset, CSS normalization, Android 실기기/iOS Simulator, Chrome 전체 기준, WPT와 pixel parity의 범위가 서로 충돌하지 않는다. 남은 미검증 항목은 계획·계약·실행 근거에 명시돼 있다. 계획은 전체 Flex/Position 적합성이나 부모 단계 완료를 주장하지 않는다.
