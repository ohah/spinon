# C09.3 `flow-root` 구현 계획의 실패 관점 검토

- 대상 계획: [`plan/c09-block-formatting.md`의 C09.3 실행 계획](../../../plan/c09-block-formatting.md)
- 비교 모델: [C09 Chromium 사전 비교](c09-block-formatting-precomparison-2026-10-10.md), 고정 reference v2의 C09.3 4개 case·16개 node
- oracle: Chromium `154.0.8037.98`, viewport `320×240` CSS px, DPR 1·2, field별 최대 절대 오차 `0.5 CSS px`
- 계획 SHA-256: `cf5e226809efe449612a871db7929e5418642eaaf1c7d0cba9e2aa08d04c7cde`
- 범위: 코드 변경 전 계획만 검토. 구현 결과나 플랫폼 동작의 검증을 대신하지 않음.

| 번호 | 독립 실패 관점 | 공격 질문과 확인 | 계획 반영 |
| --- | --- | --- | --- |
| 1 | layout 모델 정보 손실 | `flow-root`가 `block`으로 정규화되어 부모 BFC에 참여하는가? | `LayoutDisplay::FlowRoot` typed variant를 추가하고 값 보존을 확인한다. |
| 2 | 잘못된 profile 확장 | 기존 Flex 또는 C08 Block paint에서도 우연히 `flow-root`가 통과하는가? | C09 `RuntimeBlockFormattingV1`만 허용하고 나머지 profile의 실패 테스트를 둔다. |
| 3 | Taffy feature 차이 | `FlowRoot` API가 현재 고정 의존성에 없거나 feature에서 빠져 빌드가 깨지는가? | lockfile의 Taffy `0.14.0`, `block_layout` feature, vendor source를 구현 전 확인한다. |
| 4 | custom tree dispatch 누락 | 실제 Spinon resolver가 `Display::FlowRoot`를 일반 Block처럼 호출하는가? | 자식이 있는 FlowRoot에 block algorithm과 새 context(`None`)를 전달하는 분기를 지정한다. |
| 5 | 빈 FlowRoot 경로 | 자식이 없는 FlowRoot에서 지원하지 않는 display 오류 또는 크기 변화가 생기는가? | no-child 노드는 기존 leaf path를 통과시키고 별도 테스트한다. |
| 6 | 내부 top margin 격리 | 첫 자식의 top margin이 FlowRoot 상단으로 빠져나오는가? | `c093-flow-root-internal`의 자식 frame과 부모 높이를 node별 비교한다. |
| 7 | 내부 bottom margin 격리 | 마지막 자식의 bottom margin이 FlowRoot 바깥에서 collapse하는가? | 같은 case에서 auto-height FlowRoot가 자식 margin까지 포함하는 geometry를 비교한다. |
| 8 | 부모 첫 자식 경계 | FlowRoot의 own top margin이 비-BFC 부모의 top margin과 전혀 collapse하지 않는가? | `c093-flow-root-parent-margin`에서 부모와 FlowRoot frame을 각각 확인한다. |
| 9 | 부모 마지막 자식 경계 | FlowRoot의 own bottom margin이 auto-height 부모 끝에서 사라지는가? | `c093-flow-root-parent-last-child`의 parent height와 다음 형제 y를 확인한다. |
| 10 | 이전 형제 경계 | 앞선 Block의 bottom margin과 FlowRoot top margin을 합산하는가? | sibling case에서 앞선 box, FlowRoot, 후속 box의 y를 함께 비교한다. |
| 11 | 다음 형제 경계 | FlowRoot bottom margin과 다음 Block top margin을 합산하는가? | 같은 sibling case의 후속 box y를 별도 assertion으로 비교한다. |
| 12 | 일반 Block 회귀 | flow-root가 아닌 일반 Block까지 독립 context가 되어 기존 C09.2 결과를 바꾸는가? | 기존 16개 C09.2 case suite를 수정 없이 재실행한다. |
| 13 | HostRoot 혼동 | root margin 제한이나 viewport containing-block 합성이 CSS 부모 관계를 만들어내는가? | 기존 root margin/fragment 경계를 유지하고 C09.3 중첩 관계만 사용한다. |
| 14 | `display:none` 상호작용 | 숨겨진 subtree가 BFC·margin strut·frame 결과에 참여하는가? | C09.2 hidden case 회귀를 유지하고 C09.3 변환 후 hidden frames도 기존 0 frame 규칙을 따른다. |
| 15 | cascade 값 누락 | Stylo가 computed `flow-root`를 반환해도 stylesheet 사전 검사가 먼저 거부하는가? | inline 및 nested author stylesheet scanner에서 `flow-root`를 허용하고 지원 밖 값을 계속 차단한다. |
| 16 | scanner 과허용 | `flow-root` 허용 변경이 `inline flow-root`, `grid`, `var()`까지 허용하는가? | 차단 키워드·함수·다중 키워드 negative tests를 유지한다. |
| 17 | CSSOM/UA 범위 과장 | 이번 동작이 전체 CSS Display 값·UA/default styles 지원으로 발표되는가? | 내부 C09 profile에 한정하고 공개 CSS 지원 완료나 다른 BFC 생성 조건을 주장하지 않는다. |
| 18 | DPR 단위 혼동 | DPR 2의 좌표가 device px로 비교되어 값이 두 배로 보이는가? | oracle·runtime 모두 CSS px을 사용하고 DPR 1/2 결과 동일성을 assert한다. |
| 19 | renderer 경로 착시 | Android/iOS에서 box count만 보여 실제 layout report 없이 성공 처리하는가? | V8 fixture report의 `layout=ready`·node frame을 수집하고 캡처는 보조 자료로만 쓴다. |
| 20 | 플랫폼·하드웨어 주장 확대 | simulator 결과를 실기기나 GPU 성능 결과로 일반화하는가? | API 37 emulator와 iOS 26.2 Simulator로 범위를 한정하고 실기기·성능은 미검증으로 남긴다. |

## 계획 수정 사항

- 기존 계획은 C09.3 기능 범위와 4개 oracle case를 선언했지만, pinned Taffy의 `FlowRoot` enum이 Spinon의 사용자 정의 `CalcLayoutTree` dispatcher에 연결되어 있지 않다는 구현 경로를 지정하지 않았다. 별도 BFC context를 만드는 구체적인 adapter 작업을 계획에 추가했다.
- 기존 source scanner가 `flow-root`를 미지원값으로 거부하므로, inline과 nested stylesheet 허용 범위 및 다른 profile fail-closed 테스트를 계획에 명시했다.
- C09.3 화면 검증이 geometry 판정을 대신하지 않도록 Rust/Chromium 수치 비교와 V8→WGPU smoke를 구분했다.

검토 결론: 위 수정 후 구현 진입을 막는 계획 공백은 남지 않았다. 남은 불확실성은 실제 pinned Taffy 경로가 기존 Chromium 관찰값과 일치하는지이며, 이는 구현 후 독립 20개 검토 및 테스트로 판정한다.
