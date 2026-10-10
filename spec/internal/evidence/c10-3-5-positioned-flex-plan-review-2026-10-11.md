# C10.3.5 위치 지정 Flex 자식 계획 실패 관점 검토

**검토일:** 2026-10-11 · **범위:** 전용 계획, 기존 C12.2/C10.3 경계, 고정 CSS Flexbox 기준과 pinned Chrome/WPT 후보 · **성격:** 구현 전 계획 검토. 기능 구현이나 runtime 통과의 증거가 아니다.

계획과 현재 코드의 source parent, Flex formatting context, containing block owner, layout child set, paint preorder를 대조했다. 아래 20개는 서로 다른 실패 경로를 묻고, 발견한 경계 누락은 계획에 반영했다.

| # | 실패 경로 질문 | 대조 결과와 판정 | 계획 반영 |
| ---: | --- | --- | --- |
| 1 | Flex containing-block owner만 보고 후손에 Flex static-position을 잘못 적용하는가 | `positioning.rs`는 `flex_source_parent || flex_owner`를 같은 거부로 묶는다. Block wrapper 후손은 직접 Flex child와 다른 CSS formatting owner를 가진다. | direct child와 Block wrapper descendant를 다른 성공/오류 case로 분리했다. |
| 2 | source parent가 Flex인 child의 actual containing block이 다른 ancestor일 때 좌표가 섞이는가 | static-position parent와 nearest positioned owner는 독립 관계일 수 있다. | 두 owner를 따로 저장하고 ancestor frame 기반 좌표 변환을 요구한다. |
| 3 | containing block이 Flex이면 source tree를 그 아래로 재부모화하는가 | C12.2는 source tree와 calculation owner를 분리한다. | HostDocument parent/order/NodeId를 변경하지 않는 불변식을 명시했다. |
| 4 | absolute child가 Flex line, gap 또는 wrapping에 참가하는가 | Flexbox §4.1은 out-of-flow child를 Flex layout에서 제외한다. | child 추가/삭제 전후 line·gap·형제 frame 불변을 필수로 두었다. |
| 5 | 정적 위치를 실제 Flex line에서 이전/다음 형제 위치로 추정하는가 | 현재 CSS Flexbox 기준은 child를 used size 고정인 유일한 Flex item처럼 계산한다. | content-edge/static-position rectangle과 single-item main-axis 계산을 명시했다. |
| 6 | cross-axis static rectangle을 padding/border edge로 잘못 잡는가 | §4.1은 container content edges를 사용한다. C12.2 CB geometry는 padding edge owner 기준이다. | static-position rectangle과 actual CB padding edge를 분리해 관찰하도록 했다. |
| 7 | auto margin을 실제 Flex free-space 분배처럼 확장하는가 | §4.1 static-position 계산에서 child auto margins는 0이다. | auto margin 계산을 static-position 전용 규칙으로 기록하고 실제 absolute margin 동작과 분리했다. |
| 8 | inset 한 쪽만 auto인데 양쪽 auto 정렬을 강제로 적용하는가 | static position은 해당 축의 양쪽 inset이 auto인 축에서만 사용한다. | x/y inset 조합을 독립 fixture 축으로 만들었다. |
| 9 | authored inset·computed inset·used offset·final frame을 같은 값으로 기록하는가 | cascade 값과 static-position에서 파생된 used 값은 다른 단계다. | 네 값을 별도 observation으로 보존하도록 했다. |
| 10 | `align-self:auto`를 고정 값으로 잘못 해석하는가 | alignment keyword가 computed/used behavior에서 달라질 수 있다. | `auto`, `stretch`, baseline/safety 변형을 pinned Chrome으로 관찰하고 미지원 값은 명시 오류로 둔다. |
| 11 | `justify-content` 분배 값의 0/음수 free-space 동작을 임의 추정하는가 | single-item 고정 크기 알고리즘에서도 분배·overflow 경계가 있다. | 모든 값을 직접 reference로 캡처하고 unsupported declaration fallback도 비교한다. |
| 12 | `flex-direction: row-reverse`를 paint order 변경과 합치는가 | main-axis 배치와 order-modified paint rank는 다른 규칙이다. | 네 방향에서 geometry와 paint rank를 별도 기록한다. |
| 13 | `flex-wrap` 또는 `align-content`가 absolute child를 Flex line으로 만들어내는가 | absolute child는 line collection 밖에 있고 static rectangle은 content box 기준이다. | nowrap/wrap·한 줄/다중 줄 조합에서 line 및 frame 불변을 확인한다. |
| 14 | absolute child의 authored `order`를 paint rank로 사용하거나 0 rank를 무시하는가 | Flexbox는 absolute child를 Flex item과 paint 순서 비교 시 `order:0`으로 취급한다. | direct absolute child의 effective paint rank 0과 authored order 독립성을 명시했다. |
| 15 | equal-rank item에서 source order tie를 잃거나 absolute끼리 재정렬되는가 | order-modified order는 안정된 시각 순서가 필요하다. | 음수/0/양수 item·absolute sibling을 겹치고 equal-rank tie를 검사한다. |
| 16 | nested Flex descendants를 전역 order 숫자 하나로 섞는가 | 각 Flex container는 자기 direct item 순서를 소유한다. | paint subtree를 부모 item rank 아래에서 연속 유지한다. |
| 17 | `display:none` child를 line/paint에 남기거나 HostDocument에서 삭제하는가 | 숨은 box는 layout/paint에서 제외되지만 source node와 lifetime은 별개다. | source tree 보존과 layout/paint 제외를 별도 확인한다. |
| 18 | ancestor effect, `z-index`, clip, scroll, hit-test까지 암묵 지원했다고 주장하는가 | stacking 및 input 경로는 C12.4/C13/S05/C22 소유다. | 이 단계 범위 밖과 불투명 box 캡처 경계를 분명히 했다. |
| 19 | stale owner/style/revision 또는 좌표 변환 일부 실패가 혼합 frame을 내는가 | 이전 positioning 구현은 입력 오류를 layout commit 전에 실패시키도록 설계했다. | 오류의 NodeId/property/revision 및 frame 원자성을 완료 gate에 포함했다. |
| 20 | Chrome fixture 일부나 simulator smoke를 전체 Flex/WPT/실기기 동등성으로 확대하는가 | 각 비교 모델은 fixture geometry, WPT path, runtime smoke와 서로 다른 증거다. | DPR 1/2·node별 허용오차, WPT 미실행 표기, Android 실기기와 iOS Simulator 한계를 분리했다. |

## 검토 결과

- 계획의 핵심 경계는 source parent, Flex static-position formatting owner, absolute containing-block owner로 나눴다. 실제 구현이 좌표계나 CSS box parent를 구분할 수 없는 조합은 성공시키지 않는다.
- 현재 code guard가 containing-block owner가 Flex라는 이유만으로 Block wrapper 후손을 거부하는 점은 C10.3.5 사전 Chrome 비교와 구현 fixture에서 직접 판정해야 한다. WPT/Chrome 결과 전에는 guard를 제거하지 않는다.
- fixed-size element box만 우선 비교한다. text, intrinsic sizing, Grid, writing mode, stacking과 hit-test는 이 PR의 완료 범위가 아니다.
- 구현 전 Chromium fixture/reference와 WPT pinned path inventory를 만든다. WPT 파일 존재 확인과 suite 실행은 분리하고, 이 검토는 어느 쪽도 구현 통과로 계산하지 않는다.
