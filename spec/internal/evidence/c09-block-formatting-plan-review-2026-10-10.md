# C09 Block formatting 계획의 독립 실패 관점 검토

검토 대상은 [C09 계획](../../../plan/c09-block-formatting.md), [내부 계약 초안 0047](../0047-c09-block-formatting.md), 현재 C06/C07 계약, 고정 Taffy 0.14.0 dependency다. 이는 계획 검토이며 C09 구현·기능 검증을 뜻하지 않는다.

| # | 실패 관점 | 반례·위험 | 검토 결과와 반영 |
| ---: | --- | --- | --- |
| 1 | C08 병합 전에 코드 착수 | C08의 기본 block-flow가 아직 main에 없는데 C09가 그 코드를 전제로 함 | C08.1 PR #105 병합을 runtime 구현 gate로 명시했다. 계획만 먼저 준비한다. |
| 2 | HostRoot를 CSS box로 오인 | native surface 또는 HostRoot padding이 CSS margin에 합산됨 | HostRoot와 CSS box를 분리하고 viewport containing block은 DOM·paint node가 아닌 보조 입력으로 정의했다. |
| 3 | CSS root와 fragment root 혼동 | 실제 CSS `:root` 특례를 HostRoot 직속 Element에 적용하거나 브라우저 `html`/`body` margin이 비교값에 섞임 | C04.8 계약의 독립 fragment 의미를 확인했다. HostRoot Element 하나를 viewport 크기의 `flow-root` wrapper 안 일반 Block으로 참조하고 wrapper는 0 margin/padding/border로 고정한다. 실제 CSS root margin propagation과 여러 HostRoot는 지원 범위에서 제외했다. |
| 4 | containing block의 content/border edge 혼동 | child percentage 또는 auto width에 부모 border-box 폭을 사용 | parent content box와 C06 percentage/C07 box model 교차 검증을 적었다. |
| 5 | `auto` width 방정식 불완전 | 좌우 margin만 검사하거나 모든 값 지정 상태의 LTR over-constraint에서 우측 margin 규칙을 놓침 | auto width/margins, over-constrained LTR 우측 margin, C07 padding·border·min/max를 별도 비교하도록 보완했다. |
| 6 | RTL 과잉 주장 | LTR over-constrained width 해법을 RTL에도 적용 | C09.1은 LTR로 제한하고 direction/RTL은 C17 owner로 보냈다. |
| 7 | C06 단위 의미 중복 구현 | `%` margin을 C09에서 다른 containing block basis로 환산 | 단위 환산은 C06 계약을 재사용하고 C09는 관계 geometry만 통합 검증한다. |
| 8 | C07 box sizing 회귀 | border/padding을 adapter와 Taffy에서 중복 가산 | C07.1/.2 교차 regression을 요구하고 새 환산식을 만들지 않게 했다. |
| 9 | adjacent pair만 시험 | 세 개 이상 margin collapse가 순서별 합산으로 잘못 계산 | 다중 margin strut을 독립 case로 포함했다. |
| 10 | negative margin 취급 | 음수 두 개, positive+negative, 양수 없는 음수를 모두 0으로 clamp | signed strut 최대/차감 규칙과 음수 전용 입력을 분리했다. |
| 11 | parent-child collapse 조건 | padding, border, height, min-height를 하나의 일반 규칙으로 처리 | 각 면의 used edge와 used size/child 조건을 조합해 Chromium fixture별로 판정하게 했다. |
| 12 | declared border-width를 barrier로 사용 | `border-width:8px; border-style:none`이 collapse를 잘못 막음 | 선언 문자열이 아닌 C07.2 used border width를 입력으로 명시했다. |
| 13 | border color 누락을 layout 실패로 오인 | geometry용 border 폭을 paint 지원으로 확대 | C09는 collapse barrier와 layout만 다루고 선 페인트는 C22로 분리했다. |
| 14 | flow-root 외부 margin 손실 | 내부 child만 보고 flow-root box 자체와 비-BFC 부모 첫/마지막 자식 및 상위 BFC 형제 사이 외부 margin을 모두 차단 | 내부 child collapse와 flow-root box의 부모·형제 관계를 별도 중첩 fixture로 검사하도록 보완했다. |
| 15 | formatting context 범위 누락 | flow-root만 구현하고 overflow/position/float/Flex/Grid도 지원했다고 주장 | 각 생성 조건을 owning roadmap task에 매핑하고 C09 profile에서는 fail closed한다. |
| 16 | clearance가 일반 margin으로 변환 | float/clear 입력이 성공한 Block으로 조용히 계산됨 | clearance fixture를 C26까지 보류하고 float/clear는 지원 profile 오류로 둔다. |
| 17 | positioned containing block 조기 구현 | absolute/fixed 요소가 normal-flow parent content box를 잘못 사용 | position 기반 containing block은 C12로 이관했다. |
| 18 | shrink-to-fit 식을 완전 규격으로 간주 | CSS2.1 식만 구현하거나 `width:auto`인 absolute를 모두 shrink-to-fit으로 오인 | float/inline-block을 각각 C26/C15로 두고 absolute는 §10.3.7 cases 1·3으로 한정했다. C14/C15 intrinsic 측정과 사용처별 Chromium geometry를 필수화했다. |
| 19 | `fit-content`와 shrink-to-fit 동일시 | 계산에 필요한 intrinsic width가 없는 상태에서 Taffy value만 매핑 | CSS property 의미를 구분하고 수치 대조 전에는 C09.4를 미구현으로 유지한다. |
| 20 | 앱 검증을 화면 한 장으로 대체 | box count/screenshot 통과를 자식 layout 정확도라고 보거나 `display:none` frame이 누락/이전 값으로 남음 | 두 simulator에서 V8 frame별 CSS geometry·revision을 같은 Chrome reference에 대조하고, C04.9 계약에 맞춰 숨김 subtree preorder frame은 모두 0인지 검증하도록 했다. |

## 반영 후 상태

- 위 20개는 각각 다른 semantic boundary 또는 검증 실패 경로를 다룬다. 발견한 설계 취약점은 계획과 0047 계약에 반영했다.
- C04.8의 HostRoot 직속 Element별 독립 fragment cascade scope와 `:root`/`documentElement` 비해당 규칙을 원 계약 0029에서 확인하고 root geometry 비교 harness를 이에 맞췄다.
- C04.9의 DOM preorder frame 목록 및 `display:none` node/subtree의 0 frame 규칙을 원 계약 0030에서 확인해 계획·0047에 유지했다. LTR over-constraint와 absolute shrink-to-fit의 조건부 §10.3.7 cases도 계획 범위에 추가했다.
- 계획의 수치 비교는 고정 Chrome 154.0.8037.98, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport 320×240 CSS px, DPR 1·2, node별 0.5 CSS px 상한으로 고정했다.
- Taffy의 `flow-root`, optional `float_layout`, 그리고 Spinon에서 해당 feature가 비활성인 상태를 local locked source와 공식 API 문서에서 대조했다. 이 점은 구현 가능성 증거일 뿐 CSS 동등성 증거는 아니다.
- 계획 hash: `67755958e01137e92550744057aedea34943ae6d2596fe8bb426f2b596d1a1ab` (`plan/c09-block-formatting.md`, SHA-256).
- 계획 검토와 C09 구현 검토는 별도다. 이 표는 아직 구현되지 않은 코드를 통과 처리하지 않는다.
