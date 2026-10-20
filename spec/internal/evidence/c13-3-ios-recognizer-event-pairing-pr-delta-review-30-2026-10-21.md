# C13.3 iOS recognizer event-pairing 계획 PR 변경 검토 30개

계획, 공식 상태 대장 연결, 계획 검토 기록의 전체 변경을 30개 실패 관점으로 대조했다.

1. **action attribution을 몰래 추가하나?** — app `event_sequence`와 XCTest action ID를 join하지 않고 action counter window와 전체 event ledger를 분리한다.
2. **`.scroll` type을 특정 테스트 action의 증거로 확대하나?** — 같은 dispatch에서 관찰된 event type만 기록하고 action-level 원인은 미확정으로 둔다.
3. **`super.sendEvent` 전달을 누락·중복할 수 있나?** — 계측 경로는 `super`를 정확히 한 번 호출하고 전달 순서를 바꾸지 않는다고 고정했다.
4. **`UIEvent`의 미공개 필드를 요구하나?** — 공개 `type`과 공개 recognizer state만 표본화한다.
5. **main thread 밖에서 UIKit state를 읽나?** — thread invariant를 검사하고 위반 launch를 무효로 분류한다.
6. **recognizer 등록 전 startup event나 mode별 다른 recognizer 수를 오분류하나?** — ready marker 이후 capture하고 scroll mode의 세 reference와 direct-touch mode의 한 reference를 각각 확인한다.
7. **weak reference를 등록 후 강하게 바꾸어 view lifetime을 늘리나?** — recognizer 소유자는 기존 view이며 probe reference는 weak로 유지한다.
8. **recognizer type·delegate·input 설정을 바꾸어 비교 대상을 바꾸나?** — 기존 continuous/discrete/union 구성과 delegate를 그대로 유지한다.
9. **반복 `.changed` callback을 state transition 부재로 버리나?** — pre/post state와 callback delta를 독립 필드로 기록하고 안정 state에서 증가한 callback도 보존한다.
10. **target-action이 없는 public state transition을 오류로 오분류하나?** — state 변화만 있는 row를 유효 관측으로 분류하며 일률적인 callback 동반을 요구하지 않는다.
11. **union recognizer 기대치가 바뀌나?** — 기존 typed/union invariant만 기존 oracle 규칙으로 확인한다.
12. **callback이 나중 event에서 실행되는데 앞 event에 소급 귀속하나?** — target-action counter delta는 해당 `sendEvent` stack에서만 기록하며 late callback은 기존 action window에만 나타날 수 있게 한다.
13. **다른 event type이 recognizer 변화를 일으키거나 UI refresh가 dispatch를 재진입하나?** — `.scroll` 외 type도 ledger에 남기며 refresh는 `sendEvent` 반환 뒤 main-queue turn에서 예약한다.
14. **clock domain을 혼합하나?** — monotonic clock은 앱 내부 순서와 super dispatch elapsed에만 쓴다.
15. **`UIEvent.timestamp`를 monotonic과 같은 clock이라고 가정하나?** — 두 시각을 결합하지 않고 receive monotonic time만 비교에 사용한다.
16. **action counter read 시각으로 event/action ID를 복원하나?** — action window는 기존 counter 비교만 유지하고 ledger sequence를 action 범위로 자르지 않는다.
17. **event burst로 메모리가 제한 없이 늘어나나?** — launch당 4,096 row 상한과 overflow invalidation을 명시했다.
18. **overflow 뒤 일부 row를 완전한 run처럼 보이나?** — overflow가 발생한 launch 전체를 무효 처리한다.
19. **event dispatch hot path에서 파일 IO나 문자열 생성을 하나?** — raw enum·숫자 필드를 bounded memory row에 저장하고 문자열화·IO는 사후 수행한다.
20. **nested dispatch나 duplicate/missing sequence가 aggregate에서 숨겨지나?** — entry-time reserved slot과 export sequence 정렬을 사용하고 duplicate, missing, partial pre/post를 parser gate에 둔다.
21. **새 launch의 sequence가 이전 process와 이어지나?** — 앱 process마다 sequence와 counter를 초기화하고 launch provenance를 분리한다.
22. **unknown event/state enum을 정상값으로 뭉개나?** — 알려진 state는 이름을 기록하고 unknown raw integer는 그대로 보존한다.
23. **하나의 XCTest action에 event 하나만 온다고 가정하나?** — 0개·복수 event 모두 보존하며 기존 action parser와 같은 window 기준을 유지한다. 마지막 export control은 별도 finalizer이고 scored action에 포함하지 않는다.
24. **callback/state outcome 표가 zero delta를 실패로만 해석하나?** — 둘 다 불변, 한쪽만 증가, 둘 다 관찰된 상태를 각각 분류한다.
25. **direct-touch 결과를 indirect scroll 분모에 더하나?** — 별도 positive-control matrix와 summary로 유지한다.
26. **positive control이 계측 작동을 실제로 입증하나?** — direct-touch target-action 증가가 하나 이상의 per-event row에 나타나야 계측 gate를 통과한다.
27. **positive control의 callback 증가를 모든 state transition의 보편 규칙으로 확대하나?** — callback row 한 건의 관측 가능성만 확인하며 각 state 변화에 callback을 요구하지 않는다.
28. **식별자나 unrelated screen content를 공유 evidence에 포함하나?** — process/thread 식별자 제거와 기존 harness 결과물 최소화 규칙을 유지한다.
29. **binary와 source 결과가 섞이나?** — 새 harness source, test parser, app bundle hash와 app-container ledger export를 같은 실행 단위로 묶고 source 변경 전후를 합산하지 않는다.
30. **상태 대장 계획 링크가 docs route dead-link를 새로 만들거나 C13.3 완료로 읽히나?** — plan 디렉터리는 docs route 밖이므로 GitHub main source URL로 연결하고 C13.3 및 `74/207` 상태를 유지한다.

## 판정

계획과 상태 대장 연결은 관찰 경계를 좁히되 제품 동작 지원이나 event/action 인과를 주장하지 않는다. state와 callback counter의 비대칭을 실패로 강제하지 않으며, 구현 PR 전에는 이 계획의 실행 oracle과 20개 구현/실행 검토를 별도로 충족해야 한다.
