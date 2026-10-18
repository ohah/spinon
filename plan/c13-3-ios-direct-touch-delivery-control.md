# C13.3 iOS direct-touch recognizer 대조 계획

**상태:** 실행 완료 · 고정 iPadOS 26.2 Simulator에서 XCTest direct-touch control과 기존 continuous-scroll oracle을 비교했다. 제품 코드는 변경하지 않았다.

## 질문과 범위

2026-10-18의 standalone UIKit 대조는 같은 XCTest `coordinate.scroll(byDeltaX:)` 입력에서 `UIScrollView` 8/10, 일반 `UIView` 7/10 callback을 관찰했다. 누락 계층은 XCTest event synthesis, Simulator 전달, UIKit scroll recognizer 중 미확정이다. 이 후속은 입력 종류를 분리한다. `XCUICoordinate.press(forDuration:thenDragTo:)`로 직접 터치를 보내고 일반 `UIView`의 `UIPanGestureRecognizer` callback을 관찰한다. Apple 문서상 `allowedScrollTypesMask`는 마우스 스크롤·trackpad scroll을 허용하며, 이를 비워도 touch recognizer는 별도로 구성할 수 있다. 직접 터치 대조가 성공해도 간접 scroll 이벤트 경로를 검증한 것은 아니다.

계획에서 바꾸는 경로는 독립 probe harness와 이 계획·근거 문서뿐이다. Spinon runtime, 앱 fixture adapter, 공개 API 및 완료 상태는 수정하지 않는다.

## oracle와 fixture

- oracle host: 기존 `dev.spinon.probe.oracle.host` 독립 UIKit 앱. Spinon framework, Rust queue, renderer를 링크하지 않는다.
- Simulator: iPad Pro 13-inch (M5), iOS 26.2 build `23C54`, UUID `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`.
- 앱 장면: scroll view 없는 화면 전체 `plain-oracle-target` UIView. recognizer는 기존 `.continuous` oracle와 같은 `UIPanGestureRecognizer`지만 `allowedScrollTypesMask = []`, `allowedTouchTypes = [UITouch.TouchType.direct.rawValue]`로 직접 터치만 허용한다.
- 입력: XCUITest coordinate의 중앙선에서 x=0.8→0.2, 이어서 x=0.2→0.8로 누르고 끄는 입력을 반복한다. 화면 가장자리에서 시작하지 않으며 기기나 외장 포인터를 사용하지 않는다.
- 관찰: 매 gesture 직전·직후 callback counter와 callback translation 기반 누적 x를 읽는다. 앱을 fresh launch하는 세 batch에 각 10 gesture씩 총 30회를 실행하고 batch마다 방향·counter·offset 전후값을 별도 XCTest attachment에 기록한다. 앱 로그와 xcresult test/attachment manifest도 보존한다.
- 비교: 같은 harness의 기존 `testContinuousScrollRecognizerWithoutScrollView`를 별도 fresh launch로 실행한다. 그 recognizer는 `allowedScrollTypesMask = .continuous`, `allowedTouchTypes = []`이며 입력은 `coordinate.scroll(byDeltaX:)`다. 두 입력을 동일한 event source로 취급하지 않는다.

## 사전 판정 기준

1. 직접 터치 positive control은 별도 fresh app launch 3회, 각 launch의 시작 callback counter/offset이 0인지 확인한다. 각 launch 10 gesture 중 callback counter 증가 10/10 및 누적 offset 부호 일치 10/10이면 유효하다. 총 30 gesture 중 하나라도 실패하면 control 통과로 부르지 않고 해당 행과 로그를 보존한다. 세 batch는 한 UI test/result bundle에서 기록해 disk 사용량을 제한한다.
2. continuous-scroll 비교는 기존 방식으로 양·음 방향을 각 5회 기록한다. 예상 비율을 pass 기준으로 고정하지 않고 callback 누락의 재현 여부와 방향/offset을 그대로 보고한다.
3. 직접 touch control이 유효하면 “이 Simulator/XCTest 환경에서 직접 터치가 해당 UIKit recognizer까지 도달한다”만 결론 낸다. continuous scroll과 결과가 달라도 XCTest 합성, Simulator delivery, UIKit scroll recognition 사이의 원인 위치는 특정하지 않는다.
4. 직접 touch control이 실패하면 비교는 진단력이 없다고 표시한다. product fix, C13.3 완료, callback delivery 안정성 주장은 하지 않는다.
5. 두 입력 모두 10/10이어도 앞서 관찰된 누락이 이 run에서 재현되지 않은 것이므로 기존 결과를 지우거나 이전 실패를 반증한 것으로 처리하지 않는다.

## 실행과 산출물

고정 checkout에서 Xcode Simulator UI test만 실행한다. 물리 iOS 기기, terminal-browser, tmux는 사용하지 않았다. result bundle 용량과 남은 디스크 공간을 확인하고 direct-touch 3 batch와 기존 continuous-scroll 비교를 같은 xcodebuild 결과에 모았다. xcresult에서 run summary, test tree, test activity, batch별 attachment를 추출하고 xcodebuild log 및 증거 파일의 SHA-256 manifest를 만들었다. 보조 `simctl diagnose`는 테스트 종료 후 3분 이상 대기해 중단했으며, test summary와 attachment는 이후 정상 생성됐다. 결과는 별도 20개 실행 적대 검토로 판정했다.

**근거 범위:** 이 실험은 직접 touch와 indirect continuous scroll의 입력 경로를 비교한다. 두 입력 경로 어느 것도 실제 trackpad, 물리 기기, Spinon 앱 또는 제품 수준 scroll 지원을 대표하지 않는다.

## 실행 결과 · 2026-10-18

고정 iPad Pro 13-inch (M5), iOS 26.2 build `23C54`에서 direct-touch UI test 세션 3개를 각각 앱 fresh launch로 만들었다. 30/30 gesture에서 UIKit callback counter가 증가했고, 누적 recognizer translation 부호도 30/30에서 입력 방향과 일치했다. fresh launch의 zero callback/offset baseline은 3/3이었다. 기존 continuous-scroll test는 같은 날 같은 Simulator에서 4/10 입력에만 callback을 기록했다(positive-X 2/5, negative-X 2/5). 두 UI test method는 2/2 pass, xcodebuild 종료 코드 0이다. 실행 원본과 모든 action observation은 [evidence README](../spec/internal/evidence/c13-3-ios-direct-touch-delivery-2026-10-18/README.md)에 연결했다.

direct touch가 통과했다는 사실은 XCTest/Simulator의 간접 scroll 입력 경로를 특정하지 않는다. 2026-10-18의 별도 반복은 plain UIView 7/10이었으므로 이번 4/10은 누락 비율의 안정된 추정치도 아니다. C13.3, 공개 wheel/trackpad 동작 및 제품 adapter는 계속 미완료다.
