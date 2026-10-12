# C11 CSS Grid 계획 실패 관점 검토 · 2026-10-11

**대상:** [C11 Grid 계획](../../../plan/c11-css-grid.md)과 [내부 계약 0059](../0059-c11-grid-layout.md). 기능 코드는 아직 변경하지 않았다. 각 관점은 구현 전 계획의 반례·경계·증거 조건을 확인한다.

| # | 공격 관점 | 실패할 수 있는 경로 | 계획 반영 |
| --- | --- | --- | --- |
| 1 | C09/C10 미완료 때문에 Grid를 잘못 선행/차단하는가 | C09.4·C10.3·C10.4는 남았지만 각각 intrinsic/text 선행 작업에 묶여 있다. | C11은 definite-size empty item으로 시작하고 C09/C10 상위는 완료 처리하지 않는다. |
| 2 | X09를 별도 구현으로 세어 중복 범위가 생기는가 | C11 Grid 구현과 X09 Grid 검증이 서로를 선행조건으로 요구하면 순환한다. | C11을 구현 소유자로 고정하고 X09를 C11 결과의 교차 검증 추적으로 분리한다. |
| 3 | X16 전체 적합성 묶음이 첫 Grid 구현을 막는가 | 전 요소·API 자동화가 끝날 때까지 Grid를 못 시작할 수 있다. | C11.1을 X16보다 독립 진행하고 fixture를 X16에 편입한다. |
| 4 | Taffy의 기능 목록을 웹 적합성으로 오해하는가 | Taffy가 Grid API를 제공해도 Stylo·CSS 해석·모바일 출력은 다를 수 있다. | Chrome oracle, runtime 앱, surface 표본이 모두 있어야 각 단계 통과다. |
| 5 | custom calc tree를 우회하는가 | high-level Taffy tree로 교체하면 CSS math owner·revision·positioning 경계가 사라진다. | custom tree에 Grid trait와 알고리즘 dispatch를 추가한다. |
| 6 | CSS 문자열을 Taffy parser로 재파싱하는가 | Stylo computed 값과 다른 cascade/parser 결과가 섞일 수 있다. | Stylo typed value에서 typed engine-neutral 입력으로만 변환한다. |
| 7 | `fr`가 free space나 gap을 잘못 계산하는가 | gap을 제외하기 전에 분배하거나 확정 크기 축에 적용한다. | C11.1 oracle에 fixed/percentage/`fr`·gap·definite basis를 독립 case로 둔다. |
| 8 | percentage track이 indefinite 축에서 조용히 계산되는가 | 부모 auto size가 없는 경우 percentage basis를 0 또는 viewport로 바꿀 수 있다. | indefinite axis는 오류이며 0/default/viewport 치환을 금지한다. |
| 9 | intrinsic contribution 부재가 감춰지는가 | empty sizing은 맞아도 텍스트·이미지의 min/max-content가 빠진다. | C14/C15 전까지 텍스트/replaced item을 분리해 실패시킨다. |
| 10 | auto-placement가 source order를 바꾸는가 | 시각 위치를 위해 HostDocument child vector를 재정렬할 수 있다. | layout placement와 DOM/source order를 분리하고 C11.1은 `order:0`만 허용한다. |
| 11 | explicit item이 auto placement 코드에 섞이는가 | line/span placement 일부만 해석돼 다음 item 위치가 잘못될 수 있다. | C11.1 placement 선언은 fail-closed, 전체 placement 알고리즘은 C11.2 소유다. |
| 12 | implicit track 확장으로 폭발하는가 | 큰 음수·양수 line, span, auto-repeat이 무제한 할당을 유발한다. | checked index·`try_reserve`·원자 실패를 요구하고 임의 cap은 근거 없이는 추가하지 않는다. |
| 13 | repeat·named line 확장 순서가 바뀌는가 | repeat 복제와 같은 이름 line 중복이 잘못 펼쳐질 수 있다. | C11.3에 repeat/line-name inventory와 경계별 oracle를 별도 둔다. |
| 14 | `grid-template-areas` 파서 복구가 비직사각형 영역을 성공시킨다 | invalid area matrix가 partial track으로 들어갈 수 있다. | C11.3에서 유효성·invalid diagnostic·computed serialization을 검증한다. |
| 15 | Grid alignment가 Flex alignment와 혼동되는가 | `normal`, `stretch`, baseline 및 safe/unsafe 의미가 다를 수 있다. | Grid 축별 alignment를 독립 property/case로 정의하고 C10 mapping을 가정하지 않는다. |
| 16 | nested Block/Flex/Grid mixed tree의 algorithm 선택이 깨지는가 | 부모가 Grid지만 자식 Flex 또는 반대인 노드를 Block으로 통과시킬 수 있다. | custom tree에서 각 node의 display별 algorithm을 dispatch하고 nested mixed tests를 둔다. |
| 17 | `display:none` subtree가 track을 차지하는가 | hidden child가 auto placement나 intrinsic sizing에 잔존할 수 있다. | hidden subtree 제외와 새 revision 재계산을 각 단계에서 검사한다. |
| 18 | absolute Grid item의 containing block을 잘못 지원으로 본다 | Block absolute owner를 grid-area containing block에 재사용할 수 있다. | C12 Grid-specific owner 계약 전 positioned item을 거부한다. |
| 19 | writing mode·RTL·overflow가 implicit 성공 경로로 새는가 | 물리 행/열 축이나 scroll/clip 의미를 LTR/viewport 기본값으로 대체할 수 있다. | C17/C13 owner를 명시하고 해당 교차 입력은 준비되기 전 실패시킨다. |
| 20 | 좁은 비교로 전체 Grid·GPU 적합성을 주장하는가 | geometry pass를 WPT·pixel·hardware GPU pass로 과장할 수 있다. | 각 단계의 geometry·surface 표본·Chrome oracle·WPT runner·실기기 경계를 분리 보고한다. |

**검토 결과:** 계획은 C11.1 구현을 시작하기 위한 고정 oracle, 제한 computed profile, typed Taffy 경계, fail-closed 입력, Android/iOS simulator gates와 별도 후속 의존성을 정의한다. 위 경계를 계약·단계표에 반영했다. 기능 구현 뒤에는 이 20개를 재사용하지 않고 새 구현 코드·실행 결과를 대상으로 별도 20개 검토를 수행한다.
