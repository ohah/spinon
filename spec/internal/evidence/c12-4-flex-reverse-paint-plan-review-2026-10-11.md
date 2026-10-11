# C12.4 reverse Flex paint 계획 검토

검토 대상은 [reverse Flex paint 계획](../../../plan/c12-4-flex-reverse-paint-order.md)과 [Chrome 사전 비교](c12-4-flex-reverse-paint-precomparison-2026-10-11.md)다. 검토는 구현에 들어가기 전에 실패 경로·계약 경계·oracle 신뢰성을 공격했다.

| 관점 | 공격 / 빠질 수 있는 실패 | 반영한 계획 보완 |
| --- | --- | --- |
| 1. Oracle 버전 | Chrome 자동 업데이트가 기준 pixel을 바꾸어 실행 간 비교가 섞일 수 있다. | 바이너리 버전·revision·SHA를 고정하고 변경 시 재기준화하도록 했다. |
| 2. 명세 충돌 | Chrome 관측을 표준 규칙처럼 일반화할 수 있다. | W3C §4.3/§5.1과의 차이를 명시하고 Chrome 제한 호환으로 한정했다. |
| 3. 방향 적용 | `row-reverse`만 고쳐 `column-reverse`가 다른 결과로 남을 수 있다. | 4개 방향의 oracle을 요구하고 row·column reverse 모두 계획 범위에 넣었다. |
| 4. `order` 정렬 | paint rank reverse를 `order` 계산 자체에 섞어 일반 row의 order도 망가질 수 있다. | 먼저 기존 stable `(order, source_order)`를 만든 다음 reverse에서만 sequence를 역순 소비한다. |
| 5. tie 안정성 | 동일 `order`에서 stable source tie가 nondeterministic할 수 있다. | 두 order가 같은 조합을 oracle에 포함하고 NodeId rank를 검사한다. |
| 6. layout 부작용 | paint 순서 helper를 layout 입력과 공유해 Taffy 위치가 바뀔 수 있다. | render snapshot 단계만 수정하고 기존 frame과 Taffy 결과 불변을 조건으로 했다. |
| 7. DOM 부작용 | HostDocument 자식 목록 자체를 재정렬하면 JS 유사 API 순서가 바뀐다. | HostDocument·NodeId·source traversal 불변을 계약에 명시했다. |
| 8. out-of-flow 자식 | absolute/fixed가 in-flow reverse에 포함될 수 있다. | direct out-of-flow child는 현재 paint 슬롯·정책대로 보존하도록 했다. |
| 9. display none | 숨긴 subtree가 reverse sequence에서 다시 등장할 수 있다. | 기존 display-none 제외 동작과 negative control을 명시했다. |
| 10. 요소 경계 | text/anonymous node가 item 목록으로 섞이면 인덱스가 어긋날 수 있다. | 현재 요소-only profile 및 미지원 경계를 유지하고 item만 역순 배치한다. |
| 11. 중첩 context | 부모 sequence를 뒤집으며 자손 stacking context가 sibling을 넘을 수 있다. | context atomics와 nested-context regression을 보존 조건에 포함했다. |
| 12. z-index phase | reverse sort를 전체 paint list에 적용해 음수·0·양수 stack phase까지 뒤집을 수 있다. | direct flex item 같은 phase 내부 sequence만 조정하고 phase sorting은 건드리지 않는다. |
| 13. invalid computed value | direction 누락/이상 값이 기본 row로 조용히 대체될 수 있다. | computed snapshot 검증과 구체 오류 전파를 요구했다. |
| 14. locale / writing mode | `direction:rtl`이나 vertical writing mode가 row 방향을 이중 반전시킬 수 있다. | 제한 profile의 현재 입력을 분리하고 광범위한 RTL/vertical 지원을 주장하지 않는다. |
| 15. 플랫폼별 WGPU | Android AVD 결과만으로 iOS Metal도 같다고 주장할 수 있다. | gate를 Android API 37 AVD로 고정하고 iOS readback을 별도 미완료로 남긴다. |
| 16. sample 오탐 | 색상 표본이 box 경계에 있어 rasterization/AA 오차를 paint 오류로 오인할 수 있다. | 표본을 겹침 내부 중앙에 두고 geometry·rank·pixel을 함께 검사한다. |
| 17. 색상 collision | 잘못된 형제 상자가 우연히 같은 색이면 pixel만 통과한다. | NodeId paint rank와 픽셀을 모두 oracle에 대조한다. |
| 18. reserve 실패 | 기존 fallible allocation 처리가 reverse 경로에서 panic/부분 snapshot을 만들 수 있다. | 새로운 allocation을 피하고 오류·부분 결과 불변을 기존 failure path와 함께 검토한다. |
| 19. stale evidence | 과거 PNG/log가 새 실행으로 재사용될 수 있다. | 고유 run ID, fixture/reference hash, case/sample 집합 확인을 통과 조건에 포함했다. |
| 20. 완료 과장 | Android pixel 통과를 전체 CSS·실기기·iOS 완료로 확대 표시할 수 있다. | C12.4 전체 지원 주장 금지, 문서별 상태 동기화, 미실행 플랫폼 분리를 조건화했다. |

결론: 구현에 들어가기 위한 oracle·표준 충돌·범위·실패 조건이 계획에 드러나도록 고쳤다. 구현 후 검토는 실제 diff와 새 runtime evidence를 대상으로 별도로 수행해야 한다.
