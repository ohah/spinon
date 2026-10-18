# C13.3 iOS inner-target XCUITest 입력 드라이버

이 프로젝트는 앱에 입력을 합성하는 진단용 XCTest host다. `C133WheelProbeUITests`는 설치된 `dev.spinon.bootstrap` 앱을 실행하고 C13.3 opt-in scene의 inner target 좌표에 scroll 입력을 보낸다. host 앱의 scroll view는 XCTest 실행에 필요한 test host이며 Spinon 비교 oracle이 아니다.

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
