# C13.3 iOS UIApplication event 경계 계측 · 2026-10-18

## 결과

기존 독립 UIKit probe 앱에 opt-in `C133ProbeApplication: UIApplication`을 추가해 `sendEvent(_:)` 진입 시 `UIEvent.type`과 marker를 기록한 뒤 `super.sendEvent(event)`를 호출한다. XCTest 접근성 label 갱신 알림은 super dispatch 뒤 발행해 관찰 계측이 현재 이벤트의 UIKit 인식 전에 화면을 갱신하지 않도록 했다. iPad Pro 13-inch (M5)/iOS 26.2 Simulator에서 세 fresh launch × 10 scroll action을 실행했다. custom application class와 zero baseline은 각각 3/3이었다. counter parser는 30개 action window 모두 완전한 event·recognizer 값을 읽었고 단조 증가를 확인했다(30/30, invalid 0).

**30개 post-action observation window 모두 `.scroll` counter가 증가했다.** 최종 revision의 window별 event count delta는 2개 2회, 3개 26회, 4개 2회로 총 90개였다. 그중 21개 window는 `.continuous`와 union recognizer에서 callback을 관찰했고 9개는 앱 경계 `.scroll`만 관찰했다. positive-X 11/15, negative-X 10/15 window에서 callback이 관찰됐다. 이는 단일 실행 표본이며 이전 실행과 callback 분류가 달라 고정 누락률로 해석하지 않는다. `.discrete` callback과 union invariant 위반은 0이며, recognizer callback은 있으나 `.scroll` event가 없던 window도 0이다. event는 XCUITest action ID로 태그되지 않으므로 이 숫자는 action 직후 counter observation window의 변화량이다. event 한 개와 action 한 개가 일대일로 대응한다고 해석하지 않는다.

최종 run에서 callback이 없던 9개 window는 각각 `.scroll` counter 증가와 함께 관찰됐다. 다만 action ID와 event 사이의 직접 join이 없으므로 그 자체로 같은 XCTest 호출의 event라고 단정할 수 없다. 앱 event가 `UIApplication.sendEvent`에 진입한 뒤 `super.sendEvent`가 이어가는 window/responder/gesture-recognizer 구간 가운데 정확한 원인도 아직 미분리다. 이 결과는 앱 진입 경계를 좁힐 뿐 UIKit 내부의 특정 단계나 실제 trackpad 동작을 입증하지 않는다. 반복 실행에서 callback window 수가 달라졌으므로 누락률로 일반화하지 않는다.

같은 실행의 direct-touch control은 3 fresh launch × 10 gesture 모두 유효한 counter window였고 `.touches` event counter가 증가하며 `.scroll` count는 유지됐다(30/30 row). UIKit pan callback과 translation 부호도 30/30이었다. 정착 구간 뒤 callback·offset을 다시 읽었다. 이를 간접 scroll 결과와 합산하지 않는다.

![앱 경계 `.scroll` count와 typed recognizer 결과](./runs/attachments/application-event-boundary-screen.png)

![direct touch는 `.touches` event로 기록된 대조](./runs/attachments/direct-touch-screen.png)

## 환경과 실행

- 독립 probe app `dev.spinon.probe.oracle.host`; iPad Pro 13-inch (M5), iOS 26.2 build `23C54` Simulator UUID `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`; macOS 26.5.1, Xcode 26.2 build `17C52`.
- 최종 소스의 clean build 뒤 focused `test-without-building`은 XCUITest 2/2 pass, failure 0, skip 0, `xcodebuild` exit 0이었다(267.904초). 직전 revision의 전체 harness suite는 17/17 pass였지만 XCTest 접근성 label 알림을 `super.sendEvent` 뒤로 옮기기 전 코드였다. 최종 revision으로 전체 suite도 다시 실행했으나 Xcode가 summary 저장 중 `mkstemp: No space left on device`를 기록했다. 해당 bundle summary는 `result=unknown`, test count 0이므로 이 시도는 pass 근거로 세지 않는다. 두 run 모두 같은 iPad Pro 13-inch (M5)/iOS 26.2 Simulator를 썼다. probe Info.plist의 향후 orientation·launch-screen runtime warning도 별도로 남아 있다. `.scroll`은 iOS 13.4부터 공개된 `UIEvent.EventType.scroll`이다.
- 재현 명령:

현재 코드의 focused Simulator 실행 명령은 [final-focused-command.txt](./runs/final-focused-command.txt)다. 전체 harness 17/17 실행은 앞선 notification-order revision에 대해 [full-suite-command.txt](./runs/full-suite-command.txt)로 재현한다. 최종 revision의 clean full-suite 재시도 명령은 [final-full-suite-attempt-command.txt](./runs/final-full-suite-attempt-command.txt), 실패 기록은 [여기](./runs/final-full-suite-attempt.txt)에 남겼으며, 불완전 result bundle을 test pass로 세지 않았다.

`-collect-test-diagnostics never`는 별도 sysdiagnose 수집을 끄며 UI test 및 XCTAttachment 결과는 수집한다. 초기에 Xcode가 436MB CoreSimulator diagnostic file을 붙이다 host volume 공간을 소진한 실패는 별도 로그와 원본 hash로 보존했다. 이후 이전 notification-order revision의 전체 suite 17/17 결과 bundle, 최종 source의 clean build와 focused 두 method 2/2 결과 bundle을 각각 확보했다. 최종 source 전체 suite 재시도는 테스트 종료 후 summary를 쓰는 중 `mkstemp: No space left on device`가 발생했다. 이 시도의 xcodebuild exit 0은 XCTest 성공 판정이 아니며, xcresult summary가 `unknown/0 tests`이므로 pass에서 제외한다.

전체 suite 17/17 결과는 [이전 revision의 clean-build 후 test-without-building run](./runs/full-suite-run.json)이다. 이전 source hash와 app debug dylib는 [별도 source record](./runs/full-suite-source-revision-before-notification-refresh.json)에 묶었다. 현재 final revision의 targeted 결과는 [2/2 summary](./runs/summary.json), [test tree](./runs/tests.json), [실행 metadata](./runs/final-focused-run.json), [exit](./runs/xcodebuild.exit), [test command](./runs/final-focused-command.txt)로 연결한다. 최종 revision의 전체 suite 재시도는 disk-full로 summary를 쓰지 못했고 [요약 판정](./runs/final-full-suite-attempt-summary.json)은 unknown/0개라 검증 결과에서 제외한다.

## 원본과 첨부

- 최종 focused run: [summary](./runs/summary.json) · [test tree](./runs/tests.json) · [실행 metadata](./runs/final-focused-run.json) · [명령](./runs/final-focused-command.txt) · [xcodebuild log/exit](./runs/xcodebuild.log) · [현재 source·app debug dylib hash](./runs/source-revision.json). 이전 revision 전체 suite: [17-test summary](./runs/full-suite-summary.json) · [test tree](./runs/full-suite-tests.json) · [실행 metadata](./runs/full-suite-run.json) · [명령](./runs/full-suite-command.txt) · [source·binary hash](./runs/full-suite-source-revision-before-notification-refresh.json). 최종 revision 전체-suite 재시도는 [명령](./runs/final-full-suite-attempt-command.txt), [저장 실패 기록](./runs/final-full-suite-attempt.txt), [미완성 summary](./runs/final-full-suite-attempt-summary.json), [xcodebuild exit](./runs/final-full-suite-attempt-xcodebuild.exit)로 구분해 둔다. [final focused 앱 이벤트 activity](./runs/app-event-activities.json) · [direct-touch activity](./runs/direct-touch-activities.json) · [SHA-256 목록](./runs/SHA256SUMS).
- [30 action rows](./runs/attachments/application-event-boundary-observations.txt) · [aggregate](./runs/attachments/application-event-boundary-summary.txt) · [launch 1](./runs/attachments/application-event-boundary-launch-1.txt), [2](./runs/attachments/application-event-boundary-launch-2.txt), [3](./runs/attachments/application-event-boundary-launch-3.txt).
- [sanitized app event 및 recognizer marker](./runs/attachments/app-event-and-recognizer-markers.txt) · [direct-touch 30 rows](./runs/attachments/direct-touch-boundary-observations.txt) · [direct-touch aggregate](./runs/attachments/direct-touch-boundary-summary.txt) · [xcresult attachment manifest](./runs/attachments/xcresult-export-manifest.json).
- [계획](../../../../plan/c13-3-ios-app-event-boundary.md) · [계획 적대 검토 20개](../../../../plan/c13-3-ios-app-event-boundary-plan-review-20.md) · [구현/실행 실패 검토 20개](./implementation-review-20.md) · [PR diff 적대 검토 30개](./pr-delta-review-30.md).
- Apple [`UIEvent.EventType.scroll`](https://developer.apple.com/documentation/uikit/uievent/eventtype/scroll) · [`UIApplication.sendEvent(_:)`](https://developer.apple.com/documentation/uikit/uiapplication/sendevent%28_%3A%29).

현재 focused run의 공유 marker derivative에서는 Simulator process/thread identifiers를 제거했다. full raw diagnostic archive는 실행 실패 원인 확인용으로만 로컬 `/tmp`에 압축 보존했으며 저장소/PR에 넣지 않는다.

## 남은 범위

custom application override에서 `.scroll`을 관찰한 뒤 `super.sendEvent`부터 window routing 및 recognizer callback까지의 세부 경계는 미측정이다. 실제 trackpad/discrete mouse, iOS 실기기, 제품 adapter·runtime 및 C13.3 전체 입력 contract도 미검증이다. 제품 코드는 바꾸지 않았고 C13.3 및 공식 완료 수 `74/207`은 미완료 상태다.
