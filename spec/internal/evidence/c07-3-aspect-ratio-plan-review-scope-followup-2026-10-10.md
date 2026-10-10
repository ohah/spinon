# C07.3 종횡비 계획 범위 재검토

최초 계획 검토 뒤 Chrome·Taffy 차이를 확인해 지원 경계를 축소하고 런타임 연결 계획을 구체화했다. 이 기록은 수정된 계획·내부 계약·고정 Chrome 기준을 서로 다른 실패 관점으로 다시 대조한다.

| # | 실패 관점 | 확인 및 반영 |
| --- | --- | --- |
| 1 | Chrome 한 버전을 CSS 표준 전체의 완전한 oracle로 오해하는가 | 기준은 Chrome 154 회귀 oracle로 제한하고 CSS Sizing Level 4 초안 상태를 계획에 적었다. |
| 2 | C07.3를 CSS 전체 지원으로 표현하는가 | 제한 Block/Flex profile만 대상으로 하고 Grid·table·inline·replaced·intrinsic sizing을 제외했다. |
| 3 | Chrome/Taffy min/max 차이를 정상 범위로 허용하는가 | 네 축과 Flex child의 다섯 차이를 기준에 남기고 해당 조합을 오류로 거부한다. |
| 4 | 충돌한 min/max 속성을 잘못된 property로 보고하는가 | 오류 계약은 node와 실제 non-default min/max property 이름을 포함한다. |
| 5 | 한 요소의 미지원 조합 뒤에 partial frame을 반환하는가 | adapter가 layout 이전에 실패하고 partial output을 만들지 않는 조건을 계약에 넣었다. |
| 6 | `auto <ratio>`를 bare ratio로 축약하는가 | typed computed value의 `auto` flag를 보존하고 현재 Taffy 계약에서 거부한다. |
| 7 | `auto`와 초기값을 불필요하게 ratio 1로 처리하는가 | 둘 다 선호 비율 없음으로 명시하고 기준 fixture에서 빈 block 동작을 각각 고정했다. |
| 8 | zero numerator/denominator를 일반 invalid declaration으로 혼동하는가 | degenerate ratio를 preferred ratio 없음으로 처리하고 Chrome computed 관찰을 별도 보존한다. |
| 9 | finite CSS 입력이 f32 변환에서 underflow/overflow하는가 | 입력 피연산자와 결과 비율을 확인하고 finite positive를 만족하지 못하면 실패한다. |
| 10 | 분자·분모 방향을 역전하는가 | `width / height` 정의와 16/9·4/3 결과를 fixture 및 Taffy mapping에 고정했다. |
| 11 | 양쪽 definite size를 ratio가 덮는가 | Taffy leaf 차이를 사전 측정하고 adapter 보정과 direct regression을 계획에 포함했다. |
| 12 | 보정이 computed source style이나 이웃 노드에 번지는가 | Taffy 호출용 복제 style만 바꾸고 원본 style과 다른 노드는 유지하도록 제한했다. |
| 13 | flex 기본 stretch가 ratio를 언제나 지킨다고 가정하는가 | row/column의 stretch와 start 정렬 관찰을 구분하고 각 결과를 fixture로 유지했다. |
| 14 | box-sizing과 padding·border를 ratio 계산에서 중복 적용하는가 | content-box와 border-box, C07.2 border 입력을 독립 조합으로 관찰한다. |
| 15 | Chrome resolved CSS 문자열과 Stylo computed value를 같은 계층에서 비교하는가 | Typed OM computed 비교와 `getBoundingClientRect()` used geometry 비교를 분리했다. |
| 16 | unsupported reference spacer를 supported 성공으로 세는가 | 다섯 고정 spacer는 위치 안정화만 하고 지원 frame 비교 수에서 제외한다. |
| 17 | DPR별 결과가 다르거나 평균 오차가 실패를 숨기는가 | DPR 1·2 CSS px 결과 일치와 node/field별 최대 절대 오차를 판정한다. |
| 18 | 스타일 변경 뒤 오래된 ratio가 재사용되는가 | StyleRevision 변화, 다음 frame, cache invalidation을 positive test 요구사항으로 명시했다. |
| 19 | 모바일 fixture가 Rust geometry oracle을 대신한다고 주장하는가 | simulator는 실제 V8→Stylo→Taffy→WGPU 장면·로그만 확인하고 자식 frame 비교는 별도 Rust/Chrome 결과로 한정한다. |
| 20 | Android software renderer나 simulator를 실기기·성능 증거로 부풀리는가 | API·OS·backend를 기록하고 hardware GPU·실기기·성능 주장은 제외했다. |

## 계획 조정 결과

- min/max와 aspect-ratio가 함께 있는 노드는 현재 지원에서 제외하고 명시적으로 실패시킨다.
- Taffy leaf의 양축 definite 보정은 독립 layout regression과 Chrome geometry 비교로 확인한다.
- computed CSS와 used geometry oracle을 분리하고, 모바일 실행은 실제 JS fixture 경로의 시각·상태 확인으로만 보고한다.
- C07.3은 내부 계약 `0.1.0`으로 유지한다. `0045`는 문서 ID이며 버전 변경이 아니다.
