# C13.3 iOS scroll event와 recognizer 상태 대응 계측

- **유형:** 독립 UIKit/XCUITest 계측 harness 후속 · 제품 입력 코드 변경 없음
- **상위:** [C13 overflow·scroll](../spec/STATUS.md#css-구현-체크리스트) · [C13.3 gesture 계획](./c13-3-scroll-gestures.md)
- **선행 근거:** [UIApplication event 경계 계측](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/README.md)
- **계획 실패 검토:** [20개 관점](../spec/internal/evidence/c13-3-ios-recognizer-event-pairing-plan-review-20-2026-10-21.md)

## 질문과 범위

2026-10-18 iPadOS Simulator probe에서는 30개 post-action observation window 모두 `UIApplication.sendEvent(_:)` 경계에서 `.scroll` count가 증가했다. 21개 window에서는 continuous/union recognizer callback도 증가했고 9개에서는 callback이 없었다. 이벤트와 XCTest action의 직접 ID 연결은 없었으므로, 그 9개 event window의 `.scroll`이 바로 앞선 특정 XCUITest 호출에서 왔다고 단정하지 않았다.

이 후속은 독립 UIKit probe의 `UIApplication.sendEvent(_:)`에서 각 event를 `super.sendEvent(event)`로 전달하기 직전·직후에 같은 세 recognizer의 상태와 callback counter를 읽어 **한 앱 event dispatch와 recognizer 상태 변화가 같은 dispatch 안에서 관찰되는지** 기록한다. 테스트 action ID를 `UIEvent`에 붙였다고 주장하지 않는다. 제품 adapter, Rust queue, 공개 DOM event/API는 변경하지 않는다.

측정 경계는 공개 `UIApplication.sendEvent(_:)` override와 공개 `UIGestureRecognizer.state`만 사용한다. 상태를 읽을 수 없는 thread, recognizer 등록 전·해제 후 event, ledger overflow는 무효 run으로 분류한다. recognizer 내부의 비공개 판단 단계, XCTest 합성·Simulator 전달의 원인, 실제 trackpad 동작은 판정하지 않는다.

## 고정 조건과 oracle

- 같은 독립 `dev.spinon.probe.oracle.host` UIKit harness, iPad Pro 13-inch (M5) / iOS 26.2 Simulator, Xcode 26.2 환경을 사용한다. 기존 app bundle과 recognizer 설정을 복사하고 이전 실행 artifact는 덮어쓰지 않는다.
- 기존 입력은 fresh app launch 3회 × positive/negative X scroll 5회씩, 총 30 action이다. direct-touch positive control은 3 launch × 10 gesture로 따로 실행한다. 각 action의 기존 2초 event/callback observation window와 150ms settle을 보존한다.
- 기준은 기존 [30 action counter rows](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/runs/attachments/application-event-boundary-observations.txt), [event boundary README](../spec/internal/evidence/c13-3-ios-app-event-boundary-2026-10-18/README.md), 그리고 같은 source의 [30 action typed recognizer 비교](../spec/internal/evidence/c13-3-ios-scroll-type-mask-classification-2026-10-18/README.md)다. 기존 합계를 이번 event 단위 관찰과 혼합하지 않는다.
- 각 `sendEvent` 호출에 전역 단조 증가 `event_sequence`, `UIEvent.type`, app monotonic receive time, super-dispatch 경과 시간, continuous/discrete/union의 pre/post state, pre/post target-action callback count를 기록한다. event 하나와 XCUITest action 하나가 일대일이라고 가정하지 않는다.
- monotonic time은 ordering·elapsed duration에만 쓴다. `UIEvent.timestamp`나 테스트 runner의 시각을 같은 clock domain이라고 미리 가정하지 않는다. action ID attribution은 기존 per-action counter window로 제한한다.

## 계측 구조

1. `C133ProbeApplication.sendEvent(_:)` 진입 때 main-thread 여부를 검사하고 단조 sequence에 대응하는 고정 ledger slot을 예약한다. 그 slot에 pre-state와 callback counter를 기록하고 `super.sendEvent(event)`를 정확히 한 번 호출한다. 반환 직후 같은 slot에 post 값을 채워 `post - pre`를 계산한다. 중첩 event가 있으면 각 event의 진입 sequence slot을 보존하고 export 때 sequence 순으로 정렬한다.
2. recognizer는 기존 continuous/discrete/union 설정과 delegate를 유지한다. scroll-mode는 그 세 recognizer를, direct-touch mode는 해당 mode의 direct-touch pan 하나를 audit 대상에 등록한다. 상태 이름은 `possible`, `began`, `changed`, `ended`, `cancelled`, `failed`로 정규화하고 알 수 없는 값은 숫자를 보존해 기록한다. target-action callback은 recognizer별 단조 counter만 증가시킨다.
3. mode별 recognizer 등록·view setup·hittable 확인이 끝난 시점에 capture-ready marker를 내고 ledger sequence를 0부터 시작한다. ready 이전 앱 startup event는 scored ledger에서 제외하며 launch를 무효화하지 않는다. ready 이후의 event는 입력 종류에 관계없이 기록한다. recognizer는 setup 뒤 weak reference로 유지하고 audit 중 등록·해제하지 않는다.
4. post-state/counter slot을 채운 직후 기존 counter UI refresh를 main queue 다음 turn으로 예약해 `sendEvent` stack 안에서 동기적으로 accessibility notification이나 추가 UI 갱신을 하지 않는다. dispatch 안에서는 등록·해제, 기다림, 추가 event 전송도 하지 않는다.
5. ledger는 app launch마다 초기화하고 launch당 최대 4,096 event row를 보관한다. 상한 초과, duplicate/missing sequence, main-thread 외 state read 또는 mode별 등록 누락은 해당 launch를 무효화한다. event row에는 enum 원시값과 pre/post counter를 저장하고 문자열 변환·파일 I/O는 action settle 구간 밖에서 한다. 각 row는 `super` 호출 전후 값을 같은 sequence에 보존한다.
6. XCUITest의 기존 action 전후 counter window를 유지하고 app ledger는 별도의 전역 sequence로 보존한다. 앱 sequence를 action에 배분하거나 두 clock의 수치로 action 경계를 재구성하지 않는다. 보고서에는 action counter window와 전체 event ledger를 나란히 남기되, 어떤 event row가 특정 XCTest action에서 왔다고 결합하지 않는다.

state 표본과 target-action callback counter는 서로 독립적인 관측이다. 같은 `.changed` 상태에서 callback이 다시 증가할 수 있고, recognizer 상태 전이만 관찰되면서 target-action이 증가하지 않을 수도 있다. 두 delta의 1:1 대응이나 모든 상태 전이의 callback 동반을 요구하지 않는다. 각 `sendEvent` row의 pre/post state와 callback delta를 그대로 분류하며, 기존 continuous/discrete/union recognizer 간 불변식만 기존 oracle과 동일하게 검사한다.

## 사전 판정표

| 앱 event window | 같은 `sendEvent`의 recognizer 상태/callback 변화 | 판정 |
| --- | --- | --- |
| `.scroll` 관찰 | recognizer `possible → began/changed` 및 target-action 증가 | 이 `.scroll` dispatch 안에서 recognizer 상태와 callback 변화를 함께 관찰 |
| `.scroll` 관찰 | state 값은 같고 target-action 증가 | 이 dispatch 전후 공개 state 값은 같지만 callback count는 증가. 반복 `.changed`를 포함해 관측 그대로 기록 |
| `.scroll` 관찰 | state와 callback 변화 없음 | 앱 event 경계는 통과했으나 해당 `super.sendEvent` 반환 시점까지 recognizer 변화는 관찰되지 않음 |
| `.scroll` 관찰 | state 변화 있음, target-action counter 불변 | 공개 state 전이는 관찰됐고 target-action 증가는 관찰되지 않음. 이를 계측 오류로 자동 분류하지 않음 |
| `.scroll` 없음 | 다른 event에서 state/callback 변화 있음 | 해당 관찰 window에서 event type·recognizer sequence를 유지하고 별도 분류; `.scroll` 원인으로 합치지 않음 |
| `.scroll` 없음 | recognizer 변화 없음 | 앱 event 경계 이전 구간은 기존과 같이 XCTest 합성/Simulator 전달 미분리 |

`super.sendEvent` 전후 state/callback 관찰은 UIKit 내부 recognizer 단계의 원인을 특정하지 않는다. callback 전달이 다른 dispatch에서 일어나면 해당 event row의 delta에 나타나지 않을 수 있으므로 기존 action-level window와 direct-touch control도 별도로 보존한다.

## 유효 실행 gate

- scroll-mode fresh launch 3/3에서 custom application class와 continuous/discrete/union 세 recognizer 등록을 확인한다. 별도 direct-touch mode fresh launch도 ready marker, direct pan 한 개 등록, zero baseline, 단조 `event_sequence`와 ledger 용량을 확인한다.
- scroll-mode 30 action row 모두 기존 app event·recognizer counter parser에서 유효해야 한다. 동일 sequence의 pre/post recognizer snapshot이 하나씩 있고 callback counter 감소가 없어야 한다.
- direct-touch 30 row에서 `.touches` 증가와 direct-touch pan callback/sign control을 별도 확인한다. 이 결과를 indirect `.scroll` 행렬에 합산하지 않는다.
- direct-touch positive control에서 target-action callback 증가가 하나 이상의 `sendEvent` row 안에 관찰되지 않으면 이 per-event 계측이 callback 시점과 연결됐다고 판정하지 않고 재현을 중단한다. state transition은 별도 관측으로 남기며 callback 증가의 전제조건으로 삼지 않는다.
- `.scroll` event가 없었던 action, event는 있었으나 state/callback이 없었던 action, state/callback이 증가한 action을 사전 판정표대로 각각 센다. 평균·성공한 test method 수로 무효 행을 숨기지 않는다.
- 앱 event type별 marker, event sequence ledger, counter rows, callback/state summary, screenshots, command, app/source hash, XCUITest result summary를 새 evidence 디렉터리에 보존한다. identifier는 공유 artifact에서 제거한다.
- 각 launch의 action matrix와 150ms settle이 끝난 뒤 XCTest가 전용 `export-event-ledger` control을 호출한다. control은 현재 event dispatch가 반환된 다음 main-queue turn에서 완성된 ledger를 임시 파일에 직렬화하고 rename으로 원자 교체한다. host는 app 종료 후 해당 Simulator app container에서 JSON과 completion marker를 복사한다. finalizer event는 ledger에 남기되 scroll/direct-touch scored action으로 세지 않는다. export 파일 부재·불완전 row·completion marker 누락은 해당 launch를 무효화한다.
- probe source·test parser와 결과 artifact hash를 묶는다. app source 수정 전후 결과를 합치지 않고, 마지막 검증 revision과 정확히 같은 app binary로 실행한다.

## 범위와 상태

이 slice는 독립 simulator harness의 계측 정확도를 높이는 조사다. Spinon 제품 iOS adapter를 변경하거나, 실제 trackpad·mouse·실기기 입력, C13.3 또는 공식 완료 수 `74/207`을 완료 처리하지 않는다. simulator/HCI의 dispatch·recognizer 내부 원인을 분리할 수 없으면 남은 한계로 기록하고 추정성 제품 변경을 하지 않는다.

계획 자체를 코드보다 먼저 독립 실패 관점 20개로 검토한다. 구현 뒤에는 새 구현·실행 실패 관점 20개와 전체 PR diff의 별도 30개 검토를 남긴다.
