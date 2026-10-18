# C13.3 UIApplication event 경계 계획 적대 검토 20개

구현 전 계획의 서로 다른 실패 경로와 판정 경계를 검토했다.

1. **기존 결과를 앱 전달 증거로 재포장하나?** — 기존 recognizer 결과는 동기만 제공하며 이번 계측은 `UIApplication.sendEvent`에서 새로 관찰한다.
2. **scroll type 공개 API 대신 private event 필드를 읽나?** — 공개 `UIEvent.type`의 `.scroll` 값만 분류에 쓴다.
3. **custom UIApplication이 실제로 설치됐는지 모른 채 count 0을 해석하나?** — `UIApplication.shared`의 동적 class를 세 launch 모두 검증한다.
4. **관찰 후 event forwarding을 누락해 동작을 바꾸나?** — override에서 `super.sendEvent(event)`를 정확히 한 번 호출하고 recognizer positive control을 보존한다.
5. **스크롤 이벤트만 세어 다른 타입 변환을 숨기나?** — touches·hover·transform 등 event type별 전체 counter를 action별 기록한다.
6. **앱 launch 이전 event가 남아 baseline을 오염시키나?** — 매 fresh process 시작에서 ledger를 초기화하고 3/3 zero baseline을 요구한다.
7. **기존 mask 실험과 host가 달라 비교가 무효가 되나?** — 같은 app bundle, target, 세 recognizer, simulator, XCUITest action을 유지한다.
8. **SwiftUI app lifecycle 교체가 target view를 바꾸나?** — 기존 view controller를 동일하게 `UIHostingController`로 표시하고 hittable 조건을 재검증한다.
9. **앱 subclass 미선택이 no-event로 잘못 기록되나?** — class identity 실패는 launch setup failure로 기록하고 결과를 무효 처리한다.
10. **counter 갱신 지연이 no-event 행을 만들나?** — app event와 callback 중 하나가 바뀔 때까지 최대 2초 poll한 뒤 150ms settle한다.
11. **callback이 여러 개라 action 수와 섞이나?** — per-action count delta와 per-event marker 행을 별도 단위로 저장한다.
12. **app event가 있고 callback이 없는 결과를 UIKit 내부 한 단계로 과장하나?** — 앱 진입 이후 dispatch/recognition 구간으로만 기술하고 내부 exact stage는 미확정으로 둔다.
13. **app event가 없으면 XCTest와 Simulator를 구분했다고 주장하나?** — 둘 사이를 나누지 않는다고 성공 기준과 README에 명시한다.
14. **recognizer callback이 있는데 `.scroll` event가 없을 때 정상 분류하나?** — 계측 invariant violation으로 run을 무효 처리한다.
15. **direct-touch control을 indirect scroll control로 합산하나?** — 30 action 각각 `.touches` 증가와 `.scroll` 불변을 요구하며 pan callback/sign을 별도 control로 보존한다.
16. **API 가용성 경계가 대상 OS보다 낮은가?** — `.scroll`은 iOS 13.4 공개 API이고 대상 Simulator는 iOS 26.2다.
17. **event subtype이나 delta에 공개되지 않은 접근자를 가정하나?** — event type count 외에는 공개 pan recognizer callback만 보고하며 scroll payload를 가정하지 않는다.
18. **logging이 입력 경로를 지연하거나 무한 출력을 만드나?** — opt-in launch flag에서 marker/counter만 기록하며 production app에는 포함되지 않는다.
19. **UI test method pass를 전달 성공으로 세나?** — action row 및 app/recognizer counters가 별도 gate이고 method status는 실행 상태만 나타낸다.
20. **진단 결과가 제품 지원 또는 로드맵 완료로 승격되나?** — adapter·공개 계약은 변경하지 않고 C13.3와 `74/207`을 그대로 미완료로 둔다.

판정: event source 가설보다 강한 앱 dispatch 경계를 공개 API로 관찰할 수 있는 계획이다. custom application bootstrap이 성공하고 event forwarding이 유지되는지 구현 검토와 Simulator run에서 별도로 확인해야 한다. 앱 경계보다 앞선 누락 원인과 실제 입력 장치 동작은 이 계획의 증명 범위가 아니다.
