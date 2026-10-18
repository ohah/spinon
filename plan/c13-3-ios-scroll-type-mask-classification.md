# C13.3 iOS continuous/discrete scroll type 분류 대조

## 질문과 범위

독립 UIKit host에서 `XCUICoordinate.scroll(byDeltaX:)` 입력 10회 중 continuous-only `UIPanGestureRecognizer` callback이 4회 관찰됐다. direct-touch control에서는 같은 종류의 recognizer가 30/30 callback을 냈지만, direct touch와 간접 scroll은 서로 다른 input source다. 이번 실험은 XCUITest의 간접 scroll 입력이 UIKit의 `.continuous`, `.discrete` 중 어느 mask를 통과하는지 분류한다.

Spinon runtime·adapter·공개 API는 수정하지 않는다. 결과가 `.discrete`로 분류되더라도 실제 trackpad가 mouse wheel로 들어온다고 일반화하거나 product recognizer mask를 바꾸지 않는다. 이 host의 `UIPanGestureRecognizer`까지 callback이 도착하는지와 입력 종류만 기록한다.

## 공식 API 근거

- Apple의 [`UIPanGestureRecognizer.allowedScrollTypesMask`](https://developer.apple.com/documentation/uikit/uipangesturerecognizer/allowedscrolltypesmask)는 이 mask가 mouse scroll movement와 trackpad의 two-finger scroll 같은 scroll event를 인식하게 한다고 설명한다. 이 설정은 touch scroll을 막지 않으므로 간접 scroll 관찰 recognizer에는 `allowedTouchTypes = []`도 설정한다.
- Apple의 [`UIScrollType`](https://developer.apple.com/documentation/uikit/uiscrolltype)는 `.discrete`를 mouse scroll wheel 계열, `.continuous`를 trackpad 계열로 구분한다.
- 계획을 세울 때 사용한 환경은 iPad Pro 13-inch (M5), iOS 26.2 Simulator build `23C54`, macOS 26.5.1이다. 실기기 입력은 범위 밖이다.

## 고정 host·fixture

- 기존 독립 host `dev.spinon.probe.oracle.host`와 일반 UIView target을 사용한다. Spinon framework, Rust queue, app adapter 및 UIScrollView는 연결하지 않는다.
- 새 `--plain-scroll-type-mask-oracle` launch mode는 한 UIView target에 세 recognizer를 동시에 설치한다: `.continuous` 전용, `.discrete` 전용, 두 종류를 허용하는 union mask.
- 세 recognizer는 각각 `allowedTouchTypes = []`, `cancelsTouchesInView = false`, `delaysTouchesBegan = false`를 사용한다. Delegate는 이 recognizer들이 동시 인식하도록 허용한다. 각 recognizer의 callback counter, state, 누적 translation과 per-callback delta를 분리해 표시·기록한다.
- 같은 simulator에서 앱을 새로 3회 실행한다. 각 실행에서 target hit-test가 가능하고 세 counter가 0인 상태를 확인한 뒤 `XCUICoordinate.scroll(byDeltaX:deltaY:)`로 positive-X와 negative-X를 다섯 쌍, 총 10 action 보낸다. direct-touch control은 별도 existing mode/test에서 계속 실행한다.
- action 전 세 counter 값을 저장한다. scroll action 뒤 union recognizer의 callback 증가를 최대 2초 poll하고, callback이 있거나 timeout이 난 뒤 150ms 정착 구간 후 세 counter를 모두 읽는다. 방향, 각 recognizer의 callback 증가량과 translation 관찰을 action별 row에 기록한다.

## 관찰 분류와 사전 판정

세 callback의 action 단위 증가 여부를 `C`(continuous), `D`(discrete), `A`(union mask)로 기록한다.

| 관찰 | 판정 |
| --- | --- |
| `C=1, D=0, A=1` | 해당 XCUITest action이 continuous recognizer까지 도달했다. 누락 action은 여전히 XCTest/Simulator/UIKit 세부 단계가 미분리다. |
| `C=0, D=1, A=1` | 해당 action은 discrete mask recognizer에 도달했다. 이 결과만으로 물리 trackpad의 실제 분류를 말하지 않는다. |
| `C=1, D=1, A=1` | 두 typed recognizer가 같은 action에서 callback을 냈다. duplicate 또는 source 분류 불변식 미확정으로 기록하고 adapter는 변경하지 않는다. |
| `C=0, D=0, A=1` | union control만 callback을 냈다. mask별 관찰 구조나 UIKit API 구성 결함 가능성이므로 source를 분류하지 않고 재현 입력을 조사한다. |
| `C=0, D=0, A=0` | 이 입력에서 세 recognizer callback을 관찰하지 못했다. OS scroll event 자체가 없었다고 단정하지 않는다. |
| `A=0`이고 `C=1` 또는 `D=1` | 동시 인식 positive control invariant 위반이다. 해당 action 분류와 실행은 무효로 표시하고 제품 결론을 내리지 않는다. |

유효한 run은 fresh app launch 3/3, zero callback baseline 3/3, action row 30/30, direct-touch control 30/30을 요구한다. callback이 없어도 진단 method 자체는 row를 남겨야 한다. UI test method pass만으로 scroll source 또는 전달 성공을 판정하지 않는다. Callback/type 분포는 action별 원본에서 계산하며 성공 비율이나 제품 지원률로 확장하지 않는다.

## 증거와 종료 조건

- XCTest result bundle의 `summary.json`, `tests.json`, action activity, 세 recognizer 별·action 별 attachment, aggregate, direct-touch screenshot을 보존하고 SHA-256 목록을 만든다.
- README에는 OS/build, simulator 모델, host가 Spinon과 독립임, 30 action 분류와 30 direct-touch control, 미분리 계층을 적는다. C13.3와 공식 완료 수 `74/207`은 유지한다.
- 계획 구현에 들어가기 전에 이 문서를 별도 20 관점으로 검토한다. 실행 후 변경 code/runtime은 새 20 관점으로 검토하고, PR 전체 diff는 30 관점으로 다시 검토한다.
- 결과가 source 분류 가설 하나를 지지하지 않으면 source는 미확정으로 남기고 더 강한 OS event instrumentation을 다음 slice로 계획한다. 실행 뒤에도 iOS physical trackpad, discrete mouse, device, product adapter, API 및 C13.3 전체 동작은 미검증이다.

## 실행 결과 · 2026-10-18

iPad Pro 13-inch (M5)/iOS 26.2 Simulator의 fresh launch 3회에서 30 action을 기록했다. typed mask 결과는 continuous-only 20, none 10, discrete 포함 0이었고 union-only 및 typed-without-union invariant violation은 모두 0이었다. positive-X와 negative-X가 각각 10/15 callback으로 같았다. Simulator callback log에는 continuous 41행과 union 41행, discrete 0행이 있으며 action callback delta와 일치했다. 같은 run의 direct-touch control은 callback/sign 30/30이었다.

따라서 observed XCTest scroll callback은 `.discrete`가 아니라 `.continuous` mask로 전달된다. 나머지 10 action의 전달 누락 단계는 아직 미확정이며 제품 adapter 수정 근거로 사용하지 않는다. C13.3 및 공식 완료 수 `74/207`은 유지한다. [실행 결과와 한계](../spec/internal/evidence/c13-3-ios-scroll-type-mask-classification-2026-10-18/README.md).
