# C13.3 iOS recognizer event-pairing 계측 계획 검토 20개

구현 전에 계획의 독립 실패 경로와 판정 경계를 점검했다.

1. **기존 action window 결과를 같은 event의 인과 증거로 승격하나?** — event/action ID 연결을 금지하고 단일 `sendEvent` 호출의 pre/post만 같은 event row에 묶는다.
2. **event row sequence가 action ID를 흉내 내나?** — sequence는 app 내부 순서이며 XCUITest action 식별자로 사용하지 않는다.
3. **`super.sendEvent` 호출이 빠지거나 두 번 호출될 수 있나?** — override 코드의 단일 forwarding 경로를 검사하고 direct-touch 및 scroll positive control을 유지한다.
4. **state sample이 `super` 호출 전후가 아니라 다른 runloop 시점에서 오나?** — pre/post sample은 같은 main-thread override stack에서 읽으며 async sample은 별도 계측으로 섞지 않는다.
5. **UIKit event delivery가 main thread가 아닌데 gesture state를 읽나?** — 각 row에 thread invariant를 기록하고 main-thread 외 state read는 launch를 무효화한다.
6. **mode별 recognizer가 등록되기 전 startup event를 무효 표본으로 세나?** — setup 완료 시 capture-ready를 내고 scroll mode의 3개 또는 direct-touch mode의 1개 weak reference를 검사한다. ready 전 event는 scored ledger에서 제외한다.
7. **view가 사라진 뒤 stale recognizer reference를 읽나?** — weak reference 해제와 active audit lifetime을 확인하고 deallocated recognizer를 무효로 센다.
8. **기존 mask, delegate 또는 touch type 설정을 바꿔 oracle을 변화시키나?** — 세 recognizer의 구성과 XCUITest 입력을 그대로 복사하고 source/binary hash를 묶는다.
9. **같은 `.changed` 상태의 반복 callback이나 callback 없는 state 전이를 잘못 무효화하나?** — state snapshot과 callback count를 독립 관측으로 기록하고, 둘 사이에 1:1 invariant를 요구하지 않는다.
10. **`possible` 상태 유지가 UIKit 내부 거부 원인이라고 해석되나?** — 결과를 관찰된 state delta로만 표현하고 정확한 internal recognition 단계는 미확정으로 둔다.
11. **callback이 별도 event dispatch에서 늦게 도달하나?** — per-event row와 기존 2초 action window를 함께 보존하고 late callback은 같은 event로 재귀속하지 않는다.
12. **scroll event 이외의 후속 event가 pan을 시작하나?** — `.scroll` 외 event type도 같은 bounded ledger에 남기고 event type별로 분리한다.
13. **nested `sendEvent`가 ledger 행의 저장 순서를 뒤집거나 refresh가 dispatch를 재진입하나?** — 진입 sequence에서 slot을 예약하고 `super` 뒤 같은 slot을 채우며 export는 sequence 순으로 정렬한다. UI/accessibility refresh는 stack 반환 뒤 다음 main-queue turn으로 예약한다.
14. **logging 비용이 인식 결과를 바꾸나?** — hot path에서 string formatting·파일 I/O·wait를 하지 않고 bounded memory row를 후처리한다.
15. **ledger capacity 초과가 조용히 event를 버리나?** — overflow flag를 고정하고 overflow launch는 전체 무효 처리한다.
16. **전역 sequence가 process 재시작 뒤 이전 run과 섞이나?** — fresh app process마다 counter를 0에서 초기화하고 launch ID와 hash를 함께 기록한다.
17. **state 표기 정수 변환이 enum case를 오분류하나?** — 알려진 모든 `UIGestureRecognizer.State`를 명시 매핑하고 unknown raw value를 보존한다.
18. **direct-touch와 indirect scroll이 같은 success count에 합쳐지나?** — direct-touch positive control을 별도 행렬·summary로 유지한다.
19. **정상적인 zero callback delta를 parser가 실패로 취급하나?** — missing/duplicate sequence, negative callback count, incomplete row만 거부하고 state 전이와 callback 증가가 일치하지 않는 유효 관측은 그대로 분류한다.
20. **probe 결과가 제품 지원이나 C13.3 완료로 승격되나?** — 제품 코드를 수정하지 않고 C13.3 및 `74/207` 미완료를 명시한다.

판정: 동일 dispatch 앞뒤의 recognizer state/callback counter 대조는 기존 앱 counter window보다 좁은 경계를 관찰하지만, action별 인과 연결 또는 UIKit 내부 원인을 증명하지 않는다. bounded ledger, registration gate, positive control과 source/binary hash 없이는 결과를 유효하게 세지 않는다.

## 검토 중 반영한 수정

첫 초안은 state transition이 있는데 callback delta가 0이면 harness 오류로 처리했다. 연속 recognizer는 `.changed` 상태가 그대로인 채 callback을 다시 보낼 수 있고, 공개 state 전이가 항상 target-action counter를 증가시킨다고 이 probe에서 가정할 수 없다. 최종 계획은 두 값을 독립 관측으로 기록하고 mismatch를 그대로 분류하도록 고쳤다. 또한 ledger 용량을 launch당 4,096 row로 고정하고 앱 event sequence를 XCUITest action window에 배분하지 않도록 경계를 명확히 했다. nested dispatch가 반환 순서를 바꿀 때에도 행을 정렬할 수 있도록 entry-time slot 예약을 지정하고, 결과 export는 app event dispatch가 끝난 뒤에 수행하도록 고정했다.
