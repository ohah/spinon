# C13.3 iOS UIApplication scroll event 경계 계측

## 질문과 범위

직전 독립 UIKit host의 XCUITest scroll type 분류에서 30 action 중 20개는 `.continuous`와 union `UIPanGestureRecognizer`에 callback이 함께 도달했고 10개는 세 recognizer 모두 callback이 없었다. observed callback은 `.discrete` misclassification이 아니었지만 no-callback action이 XCTest 합성, Simulator 전달, UIKit recognizer 중 어디서 사라지는지는 미분리다.

이번 slice는 같은 XCUITest 입력을 UIKit 앱 이벤트 디스패처 경계에서 관찰한다. 별도 `UIApplication` subclass의 `sendEvent(_:)` 진입 때 `UIEvent.type`을 기록하고 반드시 `super.sendEvent(event)`를 한 번 호출한다. 각 XCTest 호출 전후 observation window에서 `.scroll` event count와 typed/union recognizer callback count 변화를 비교한다. event에 action ID를 직접 연결하지 않으므로 이 delta를 개별 action의 원인·결과로 해석하지 않는다. 제품 runtime·adapter·공개 API는 수정하지 않는다.

판별할 수 있는 경계는 이 probe 앱의 `UIApplication.sendEvent(_:)`에서 `.scroll` counter 증가를 관찰했는지 여부다. 관찰되지 않은 action은 XCTest event synthesis와 Simulator delivery 사이를 더 나누지 않는다. 한 observation window에서 `.scroll` delta가 있고 recognizer delta가 없으면 앱 진입과 callback 관찰 사이의 구간을 조사 대상으로 좁히지만, event와 action의 직접 연결이 없으므로 특정 action의 loss 원인으로 단정하지 않는다. UIKit 내부의 정확한 처리 단계도 계측하지 않는다.

## 공식 API oracle

- Apple [`UIEvent.EventType.scroll`](https://developer.apple.com/documentation/uikit/uievent/eventtype/scroll)는 간접 입력 장치의 scroll에 해당한다고 명시한다. iOS 26.2 Simulator SDK header도 `UIEventTypeScroll`을 iOS 13.4부터 공개한다.
- Apple [`UIApplication.sendEvent(_:)`](https://developer.apple.com/documentation/uikit/uiapplication/sendevent%28_%3A%29)는 앱의 이벤트를 적절한 responder로 보내며, 이를 재정의해 입력을 관찰할 수 있다고 설명한다. 재정의는 관찰 뒤 `super.sendEvent(event)`로 전달을 계속해야 한다.
- 환경은 이전 run과 같은 iPad Pro 13-inch (M5), iOS 26.2 Simulator build 23C54, Xcode 26.2, macOS 26.5.1이다. 실제 trackpad·mouse와 실기기는 범위 밖이다.

## 고정 host와 계측 구조

- 기존 독립 `dev.spinon.probe.oracle.host` XCUITest 앱을 사용한다. Spinon framework, Rust queue, product adapter 및 `UIScrollView`를 입력 oracle로 사용하지 않는다.
- SwiftUI 자동 진입점을 작은 UIKit `UIApplicationMain` 진입점으로 교체해 `C133ProbeApplication: UIApplication`을 principal class로 명시한다. App delegate는 기존 `C133WheelOracleView`를 `UIHostingController`로 표시한다. 이 변경은 evidence harness 한정이다.
- `--app-event-boundary-oracle` 모드에서 `C133ProbeApplication.sendEvent(_:)`는 이벤트 전달 전에 event type별 단조 증가 count와 `SPINON_C133_APP_EVENT` marker를 기록하고 `super.sendEvent(event)`를 정확히 한 번 호출한다. 화면 및 접근성 counter 갱신은 `super` 호출 뒤에 발행해 계측 observer가 현재 이벤트의 UIKit 인식 전에 동기적으로 개입하지 않게 한다. 다른 launch mode에서는 count와 진단 log를 비활성화한다.
- accessibility counter에는 실제 `UIApplication` subclass 이름과 `scroll`, `touches`, `hover`, `transform`, `presses`, `motion`, `remoteControl`, `other` count를 표시한다. 각 앱 process가 시작할 때 count를 0으로 초기화한다.
- 기존 scroll-type mode는 continuous/discrete/union recognizer 세 개를 그대로 유지한다. XCTest는 app event type counters와 recognizer counters를 action 전후에 함께 읽고, 변화가 하나라도 생기거나 timeout이 날 때까지 최대 2초 대기한 뒤 150ms 정착 구간 후 row를 저장한다.

## 실행 행렬과 사전 판정

세 fresh app launch마다 positive-X와 negative-X를 각각 다섯 번, 총 30 scroll action 보낸다. 각 row는 launch·반복·방향·요청량·app event counter 전후/delta·recognizer counter 전후/delta·분류값을 담는다. 기존 direct-touch control도 세 fresh launch × 10 gesture로 실행해 `.touches` event counter와 pan callback/translation 부호를 기록한다.

| app의 `.scroll` 증가 | typed/union callback 증가 | 관찰 판정 |
| --- | --- | --- |
| 있음 | continuous/union 동시 증가 | 앱 경계와 continuous recognizer까지 도달 |
| 있음 | typed·union 모두 없음 | 앱 경계 뒤 UIKit dispatch/recognition 구간에서 callback이 관찰되지 않음 |
| 없음 | 다른 event type 증가 | action이 `.scroll`이 아닌 앱 event로 관찰됨. 입력 의미는 미분류로 남김 |
| 없음 | 모든 event type 불변 | 이 앱의 `UIApplication.sendEvent(_:)` 경계에서 event를 관찰하지 못함. XCTest 합성/Simulator 전달은 서로 미분리 |
| 없음 | recognizer callback 있음 | 계측 불변식 위반 또는 다른 dispatch 경로이므로 해당 row/run은 무효 |

유효 run은 custom UIApplication subclass 확인 3/3, scroll-mode fresh launch 3/3 및 zero baseline 3/3, scroll action row 30/30, direct-touch callback/sign 30/30과 `touches` 증가/`.scroll` 불변 row 30/30을 요구한다. direct-touch control은 XCUITest의 `.scroll` 분류와 합산하지 않는다. action당 정확히 한 `UIEvent`나 callback이 온다고 가정하지 않으며, UI test method pass는 delivery pass가 아니다.

## 증거와 종료 조건

- 계획 자체를 구현 전에 별도 실패 관점 20개로 검토한다. 구현 뒤에는 새 code/runtime 관점 20개와 PR 변경 관점 30개를 작성한다.
- XCTest result bundle의 `summary.json`, `tests.json`, action별 counter row, 앱 event marker log, callback log, aggregate, launch screenshots와 SHA-256 목록을 보존한다. process/thread 식별자는 공유 derivative에서 제거한다.
- README는 event boundary가 공개 UIApplication API 관찰 지점임을 설명하고, 화면/로그에서 app subclass와 zero baseline을 증명한다. 누락 action의 분류는 사전 판정표에 맞춰 산출하며 method 성공 수로 대체하지 않는다.
- C13.3와 공식 완료 수 `74/207`을 유지한다. 제품 adapter 변경, 실제 input 지원, 실기기 검증 완료를 주장하지 않는다. 이후에도 앱 경계 이전의 누락은 XCTest 합성과 Simulator delivery로 나뉘지 않으며 UIKit 내부 세부 원인도 공개 API 관찰만으로 완전히 특정하지 않는다.

## 참고 자료

- [Apple UIEvent.EventType.scroll](https://developer.apple.com/documentation/uikit/uievent/eventtype/scroll)
- [Apple UIApplication.sendEvent(_:)](https://developer.apple.com/documentation/uikit/uiapplication/sendevent%28_%3A%29)

## 실행 결과 · 2026-10-18

iPad Pro 13-inch (M5)/iOS 26.2 Simulator에서 custom `UIApplication` class 확인·zero baseline 각각 3/3, XCUITest scroll row 30/30, 완전하고 단조 증가한 event·recognizer counter window 30/30, direct-touch callback/sign 및 `.touches`-only row 30/30이었다. 30개 post-action observation window 모두 `UIApplication.sendEvent(_:)` interception에서 `.scroll` counter가 증가했다. 최종 revision의 window별 event count는 2개 2회, 3개 26회, 4개 2회로 총 90개다. 21 window는 continuous+union callback이 있었고 9 window는 앱 경계 `.scroll`만 관찰했다. `.discrete` callback과 recognizer invariant 위반은 0이다. positive/negative callback은 각각 11/15, 10/15였고 direct touch는 `.touches` event 30/30, `.scroll` event 0, callback/sign 30/30이었다. event에 XCTest action ID를 연결하지 않았으므로 이 값은 action 직후 observation window delta이며 일대일 causal join으로 해석하지 않는다.

같은 harness 전체 suite 17/17은 화면 갱신 알림을 `super.sendEvent` 뒤로 이동하기 전 source revision에서 통과했다. 최종 source는 clean build 뒤 focused `test-without-building` 2/2 pass를 확인했다. 최종 source 전체 suite 재시도는 summary 저장 공간 부족으로 `unknown/0 tests`를 남겨 pass로 세지 않는다. [최종 focused summary](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/runs/summary.json) · [이전 revision 전체 suite summary](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/runs/full-suite-summary.json) · [실행 근거와 한계](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/README.md).

따라서 callback이 없던 9개 observation window에서 앱 경계에 `.scroll` event가 함께 기록됐다. event/action 직접 join은 없으므로 해당 counter 증가가 바로 그 window 직전의 특정 XCTest 호출에 속한다고 단정하지 않는다. `UIApplication.sendEvent`가 기록한 뒤 `super.sendEvent`가 진행하는 window/responder/recognizer 경계 중 정확히 어디서 callback이 생략되는지는 이번 계측으로 분리하지 않는다. 반복 실행에서 callback 관찰 수가 달라져 고정 누락률을 주장하지 않는다. 제품 코드는 변경하지 않았고 C13.3 및 공식 완료 수 `74/207`은 미완료 상태로 유지한다. [실행 근거](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/README.md).
