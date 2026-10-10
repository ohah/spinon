# C10.3 Flex 순서·정렬 계획 · 실패 관점 검토

검토 대상은 [C10.3 전용 계획](../../../plan/c10-3-flex-order-alignment.md), [C10 상위 계획](../../../plan/c10-flexbox.md), [공식 상태 대장](../../STATUS.md), 현행 Stylo→typed layout→Taffy 경로와 잠긴 Taffy 0.14.0 source다. 이 검토는 계획 PR 단계의 검토이며 기능 구현, test, simulator/device 실행을 완료했다는 뜻이 아니다.

| # | 독립 관점 | 공격 질문과 가능한 실패 | 계획의 통제 |
|---:|---|---|---|
| 1 | 로드맵 선행·fixture 출처 | C10.1/10.2 line 기준과 C12/C15 의존성을 숨기거나, 임의로 만든 fixture만으로 지원 grammar를 닫지 않는가? | 선행·교차 gate를 명시하고 지원 값마다 pinned WPT 경로를 inventory에 대응한다. |
| 2 | 하위 완료 과장 | C10.3의 일부 값만 지원하고 상위 Flexbox가 완료된 것으로 표시하지 않는가? | 다섯 하위 상태와 C12/C15 gate를 따로 두고 모두 끝나기 전 C10.3/C10 미완료를 유지한다. |
| 3 | reverse 축 의미 | `row-reverse`를 row의 시작점 이동이 아니라 child vector 역순으로 구현하지 않는가? | geometry, 계산 순서, DOM 원본 순서를 별도 관찰하고 reverse 전용 case를 둔다. |
| 4 | column 방향 투영 | column 역전에서 main axis를 y가 아닌 x로 반전하거나 세로 gap을 빠뜨리지 않는가? | row/column paired frame과 gap case를 요구한다. |
| 5 | wrap-reverse 라인 쌓기 | `wrap-reverse`에서 item 수집 순서까지 반전하거나 main-axis flow를 바꾸지 않는가? | item 수집과 cross-axis line stacking을 별도 기록한다. |
| 6 | shorthand 전개 | `flex-flow` reverse 조합과 뒤따르는 longhand override가 cascade 순서를 잃지 않는가? | shorthand computed longhand와 winner 값을 fixture에 포함한다. |
| 7 | `order` sort 안정성 | 동일 order 값이 nondeterministic sort로 frame마다 흔들리지 않는가? | equal-order는 source order를 유지하는 stable tie 조건으로 고정한다. |
| 8 | order 정수 경계 | CSS `order`를 `u32`로 직접 캐스팅해 음수를 wrap하거나 큰 값을 절단하지 않는가? | Stylo 타입과 Chromium 경계를 먼저 조사하고 내부 paint rank와 CSS 값 표현을 분리한다. |
| 9 | line break와 order | item frame만 재정렬하고 line collection은 source order로 수행해 줄바꿈이 틀리지 않는가? | order 단계 완료 조건에 line collection과 C10.2 분배 적용을 모두 포함한다. |
| 10 | DOM identity 보존 | 계산용 sorted children이 HostDocument 자식 목록이나 JS `children` 결과를 바꾸지 않는가? | 원본 child vector 불변식과 NodeId-keyed frame을 유지한다. |
| 11 | 세 가지 order 값 혼동 | Taffy node 초기 `Layout::order`, Flex 내부 출력 order, runtime scene `paint_order`(현재 HostDocument preorder)를 CSS `order`로 섞지 않는가? | 계산 child 순서와 runtime paint list를 별도로 다루며, 원본 tree 순서를 보존한다. |
| 12 | nested paint subtree | 한 container의 item을 전역 정렬해 다른 nested container의 자손을 부모 밖으로 이동시키지 않는가? | 형제 flex item만 정렬하고 paint list를 subtree 단위로 재귀 생성한다. |
| 13 | 접근성·탐색 순서 | CSS `order`가 DOM, speech, sequential focus order까지 바꿨다고 잘못 주장하지 않는가? | source traversal은 원본 order, native accessibility는 미구현 경계로 명시한다. |
| 14 | 비-item 범위 | `display:none`, anonymous text item, absolute child를 일반 Flex item처럼 line layout하지 않는가? absolute child가 paint에서는 `order:0`으로 취급되는 규칙을 놓치지 않는가? | visible in-flow element만 초기 fixture에 두고 text는 C15로, absolute child의 `order:0` paint와 static-position alignment는 C10.3.5/C12로 격리한다. |
| 15 | 입력 이벤트 구현 경계 | 아직 없는 RuntimeRenderSnapshot hit-test/S05 입력 경로를 구현 완료처럼 가정하지 않는가? | 이 계획에서는 frame·paint 항목의 NodeId 연결까지만 확인하고 pointer target은 S05로 남긴다. |
| 16 | `align-self`의 computed/used 구분과 auto margin 우선순위 | computed `auto`를 부모 alignment로 덮어 쓰거나 cross-axis auto margin이 있는데도 `align-self` 값이 위치를 바꾸지 않는가? | computed `auto` 보존과 부모 alignment에 따른 used geometry를 따로 기록하고 auto margin precedence를 paired case로 둔다. |
| 17 | stretch 크기 제한 | `stretch`가 fixed cross-size, margin, min/max, padding/border 제약을 무시하지 않는가? | 자동/명시 cross-size와 box constraints를 나눠 geometry로 비교한다. |
| 18 | align-content 분기 | `wrap`이 우연히 한 line만 만든 경우를 `nowrap`과 같다고 처리하거나 `space-between` single-line fallback을 놓치지 않는가? | nowrap과 single-actual-line wrap을 따로 두고, wrap의 `space-between`은 flex-start fallback 좌표를 확인한다. |
| 19 | gap/free-space 상호작용 | align-content 분배에서 cross gap을 두 번 더하거나 빼고 wrap-reverse의 cross-start를 무시하지 않는가? | 0/양수/음수 free-space, gap, wrap-reverse 조합을 명시한다. |
| 20 | baseline·텍스트 선행 의존성 | empty-box synthetic baseline을 text baseline과 동일하다고 홍보하거나 line content-baseline을 빠뜨리지 않는가? | item first/last baseline, line baseline-content alignment, Flex container baseline을 분리하고 text metric은 C15 전까지 미완료로 둔다. |

## 검토에서 반영한 계획 수정

- 기존 C10.3 한 줄의 “방향·그리기 순서·접근성 순서를 함께 정한다”는 표현은 `order`가 CSS 시각 순서만 변경한다는 표준 계약과 혼동될 수 있었다. source traversal/접근성 순서는 원본 순서를 유지하고, 아직 없는 네이티브 접근성 구현을 별도 경계로 정정했다.
- 코드를 따라가 보니 `CalcLayoutTree`는 Taffy 노드를 postorder로 만들고 `Layout::order`를 초기화하지만, Flex algorithm은 일부 자식의 출력 order를 다시 지정한다. Runtime renderer는 이 값 대신 HostDocument preorder로 `paint_order`를 부여한다. 최초 문구가 이 값들을 하나의 렌더 순서처럼 설명할 위험이 있어 실제 세 경로를 구분하도록 고쳤다.
- `align-content`는 실제 line 하나만 있는 경우라는 조건으로 계획하면 틀릴 수 있다. `nowrap` single-line container와 `wrap` multi-line container의 한 개짜리 실제 line을 구분하도록 수정했다.
- `order` integer를 Rust 부호 없는 타입으로 바로 옮기거나 `u32`로 정렬할 위험을 추가했다. Stylo computed representation을 조사하고 stable rank를 별도 할당하도록 계획했다.
- `flex-flow` shorthand 토큰 순서가 바뀌어도 같은 computed longhand가 나오는지, `order` 소수 선언이 무시되는지 확인할 최소 입력이 부족했다. 두 경우를 seed case와 cascade 범위에 추가했다.
- `order`의 잘못된 소수 declaration을 정수처럼 조용히 읽으면 시각 순서가 잘못된다. 앞선 유효 declaration을 보존하는 item과 초기값으로 남는 item을 함께 두고 computed value와 geometry를 확인하도록 구체화했다.
- `align-self:auto`의 computed value를 부모 `align-items` 값으로 치환하면 CSS computed-style 결과와 used alignment를 혼동한다. computed `auto` 보존과 실제 geometry 계산을 분리하도록 보강했다.
- 공통 관찰 항목에 아직 구현되지 않은 runtime pointer target이 포함되어 있었다. C10.3은 frame·paint-list까지만 관찰하고 S05 입력 연결 시 target을 다루도록 경계를 바로잡았다.
- 여섯 runtime Flex profile을 단지 개수로만 적으면 구현자가 대상을 임의로 고를 수 있었다. 실제 profile 이름을 열거하고 정렬 seed case에 한 줄·두 줄 분배의 손계산 좌표를 추가했다.
- Flexbox 2025 CRD에서 `order`는 in-flow flex item의 line/layout 순서에 적용되며, absolute flex child는 flex item과의 paint 비교에서 `order:0`으로 취급된다. 이를 일반 item sort와 혼동하지 않게 C10.3.5/C12 교차 gate를 추가했다.
- Flexbox의 값 표만 보면 `align-content:normal`, `space-evenly`와 확장 baseline 문법이 빠질 수 있다. 최신 고정 CSS Box Alignment 2026-10-08 Working Draft를 함께 참조하고, Chromium oracle가 표준 초안 자체를 대신하지 않도록 초안 상태를 명시했다.
- 구현 fixture만으로 특정 shorthand·baseline grammar의 coverage가 비어도 알아채지 못할 수 있다. 각 지원 value와 interaction을 pinned upstream WPT path에 연결하고 profile 밖 case의 제외 이유를 inventory에 남기도록 보강했다.
- `align-content:space-between`은 한 개의 실제 line이 있는 `wrap`에서 `flex-start`로 돌아간다. single-line wrap seed의 예상 위치에 이를 추가했다.
- baseline 계획이 item alignment와 Flex container baseline만 다루고 line content-baseline 및 first/last 선택을 놓쳤다. empty box baseline 입력에 `align-content` first/last line case를 넣고 C15 text dependency를 유지했다.
- `align-self:stretch`와 cross-axis auto margin·고정 크기·min/max의 precedence, `align-content:stretch`가 item stretch로 연결되는 연쇄를 독립 완료 조건으로 보강했다.
- baseline은 C15 실제 글꼴 metric 없이 text 호환을 주장할 수 없다. empty box synthetic baseline과 text baseline을 나누고 C15까지 상태를 닫지 않도록 수정했다.
- 기존 C10.1/C10.2 및 다른 runtime profile에 새 CSS 동작이 새어가는 역회귀, stale revision, NodeId 매핑 오류와 S05 미구현 경계를 계획에 추가했다.
- 최신으로 바뀔 수 있는 W3C URL만 두지 않고 2025-10-14 Candidate Recommendation Draft snapshot을 직접 고정했으며, 구현 시작 시 표준 갱신 여부를 다시 검토하게 했다.

## 최종 재검토

표의 20개 질문을 계획 본문과 맞춰 다시 확인했다. 고정 Chromium oracle/threshold, C10.1/10.2 선행 관계, 명시 profile 목록/cascade 경로, in-flow/positioned child 경계, CSS tree와 layout/paint 순서 분리, S05 hit-test 미구현 경계, computed/used alignment, single-line `wrap` fallback, first/last item·line baseline 및 C15/C12 증거 gate가 문서에 남아 있다. 이 문서에서는 테스트·빌드·실행을 하지 않았다. 기능 구현 PR은 별도의 새 20개 관점 검토와 실행 근거를 가져야 한다.
