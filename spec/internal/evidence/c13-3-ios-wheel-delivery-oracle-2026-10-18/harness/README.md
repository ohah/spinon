# C13.3 iOS XCUITest 진단 harness

이 프로젝트에는 두 종류의 분리된 진단 실행이 있다. 기존 C13.3 inner-target 테스트는 설치된 `dev.spinon.bootstrap` 앱을 실행해 opt-in scene에 입력을 보낸다. 새 recognizer/event-pairing 테스트는 이 프로젝트의 독립 `dev.spinon.probe.oracle.host` UIKit host를 실행하며 Spinon 제품 앱이나 adapter를 검증하지 않는다. host 앱의 view와 recognizer는 UIKit/XCUITest 계측용이다.

## 재현 조건

- 대상: iPad Pro 13-inch (M5), iOS 26.2 Simulator, UDID `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`.
- 대상 앱: C13.3 fixture opt-in `SpinonBootstrap.app`. bundle executable SHA-256은 `46c19b1070ce41ff14084c5c33abf30603c2c6fc1143a213b6da72df91c49f6e`이어야 한다.
- 앱 시작 인수: `--spinon-c133-scroll-gesture --spinon-c133-native-wheel-adapter`.
- 매 test는 앱을 새로 시작한다. 반복 시나리오는 `testHorizontalRoundTripAtInnerTarget`, `testVerticalRoundTripAtInnerTarget`이며 canvas의 normalized point `(0.33, 0.467)`에 포인터를 둔 뒤 반대 방향을 입력한다.

고정 V8 checkout이 준비된 저장소에서 C13.3 opt-in 앱을 빌드한 뒤 simulator에 설치한다.

```sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 \
  mise exec -- bun run build:ios-sim
xcrun simctl install 9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF /path/to/SpinonBootstrap.app
```

그 앱을 유지한 채 이 XCUITest host에서 진단 입력을 실행한다.

```sh
xcodebuild test \
  -project spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/xctest-harness/C133WheelProbe.xcodeproj \
  -scheme C133WheelProbe \
  -destination 'platform=iOS Simulator,id=9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF' \
  -derivedDataPath /tmp/spinon-c133-inner-target-derived \
  -only-testing:C133WheelProbeUITests/C133WheelProbeUITests/testHorizontalRoundTripAtInnerTarget \
  -only-testing:C133WheelProbeUITests/C133WheelProbeUITests/testVerticalRoundTripAtInnerTarget
```

XCUITest method가 통과했다는 사실만으로 UIKit callback 전달을 판정하지 않는다. 각 실행과 동시에 Simulator unified log를 수집하고 `SPINON_C133_NATIVE_WHEEL` 및 `SPINON_C133_PROCESS` 행으로 callback, target, queue 처리와 offset을 확인한다.

```sh
xcrun simctl spawn 9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF log stream \
  --style compact --level debug \
  --predicate 'process == "SpinonBootstrap" AND (composedMessage CONTAINS "SPINON_C133_NATIVE_WHEEL" OR composedMessage CONTAINS "SPINON_C133_PROCESS")'
```

## 판정 경계

이 harness는 iOS Simulator의 XCTest event synthesis를 재현한다. Simulator에 실제 외장 trackpad를 연결한 시험이 아니며, XCTest 성공만으로 native callback 성공을 표시하지 않는다. `scroll(byDeltaX:)`에 기록된 요청량과 `UIPanGestureRecognizer.translation(in:)`에서 관찰한 실제 delta는 같다고 가정하지 않는다. Run 05에서 ±38 요청 뒤 실제 sample 크기는 서로 달랐고, 중심점 대조에서는 negative sample의 reported location이 target 5를 선택했다. 앱 로그의 callback이 빠지거나 target이 7이 아니면 해당 방향은 통과가 아니다. screenshot은 장면 확인용으로만 쓰고 queue 처리 증거로 쓰지 않는다. 누락 callback은 앱에 reject 로그도 남기지 않아 XCTest synthesis, Simulator delivery, UIKit recognition 중 단계가 확인되지 않았다.

## 독립 UIKit per-event recognizer 계측

`testPerEventRecognizerRowsForScrollTypeMasks`와 `testPerEventRecognizerRowsForDirectTouchControls`는 독립 UIKit host의 `C133ProbeApplication.sendEvent(_:)` 안에서 각 `super.sendEvent(_:)` 전후 recognizer state와 target-action callback count를 기록한다. `--recognizer-event-pairing-oracle`은 opt-in이며, mode와 UUID run/launch ID를 받는다. scroll mode는 `.continuous`, `.discrete`, union recognizer 세 개를, direct-touch mode는 direct pan 하나를 사용한다.

각 mode는 fresh process 3회, launch별 10 input action을 실행한다. scroll mode에서는 양·음 X scroll을 각 5회, direct-touch mode에서는 양·음 X drag를 각 5회 보낸다. 각 action은 앱 event와 callback count의 전후 snapshot을 남긴다. per-event ledger는 XCTest action ID와 결합하지 않는다. action별 observation window는 최대 2초에 150ms settle을 더한다.

각 launch가 끝날 때 XCTest가 화면의 export control을 누른다. 앱은 `super.sendEvent` 반환 뒤 다음 main-queue turn에서 JSON을 원자 기록하고 `Documents/c133-recognizer-ledger-<mode>-<run-id>-<launch-id>.json`을 표시한다. 테스트가 종료된 뒤 해당 Simulator app container에서 파일을 복사해 schema, sequence, thread, snapshot completeness, monotonic time, callback, union invariant과 overflow를 분석한다.

```sh
xcodebuild test \
  -project spec/internal/evidence/c13-3-ios-wheel-delivery-oracle-2026-10-18/harness/C133WheelProbe.xcodeproj \
  -scheme C133WheelProbe \
  -destination 'platform=iOS Simulator,id=9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF' \
  -parallel-testing-enabled NO \
  -only-testing:C133WheelProbeUITests/C133WheelProbeUITests/testPerEventRecognizerRowsForScrollTypeMasks \
  -only-testing:C133WheelProbeUITests/C133WheelProbeUITests/testPerEventRecognizerRowsForDirectTouchControls \
  -resultBundlePath /tmp/c133-recognizer-event-pairing.xcresult \
  CODE_SIGNING_ALLOWED=NO
```

최종 source revision 실행 결과와 JSON 원장은 [2026-10-21 evidence](../c13-3-ios-recognizer-event-pairing-2026-10-21/README.md)에 보존한다. iOS Simulator/XCTest synthesis 결과이며 외장 trackpad, 실기기 입력, UIKit private internals, event-to-action 인과관계 또는 Spinon 제품 adapter 동작으로 확대 해석하지 않는다.
