# C13.3 iOS native wheel adapter · iPadOS Simulator 부분 실행 근거

## 범위와 판정

이 후속은 C13.3 opt-in iOS fixture에서 UIKit continuous scroll callback을 native wheel sample로 바꾸어 기존 Rust bounded queue와 snapshot router에 전달하는 제한 adapter를 구현한다. 내부 계약 [0063](../../0063-c13-3-scroll-gestures.md), 계획 [native wheel 입력](https://github.com/ohah/spinon/blob/main/plan/c13-3-native-wheel-input.md), 사전 비교 [정규화 oracle](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-native-wheel-input-precomparison-2026-10-16/normalization-oracle.json)에 연결된다. 앱 제품 API, DOM `WheelEvent`, 실기기 지원 또는 C13.3 완료를 주장하지 않는다.

iPad Pro 13-inch (M5), iOS 26.2 Simulator에서 `C133WheelProbe`의 ±X/±Y UI test method 4개는 모두 pass했다. 이는 XCTest event synthesis와 앱 lifecycle이 오류 없이 끝났다는 뜻이다. 같은 test run의 Simulator unified log에서는 continuous UIKit adapter callback이 2/4 case에서만 확인됐다.

| 방향 | native callback | queue/process | 결과 |
| --- | --- | --- | --- |
| negative Y | 있음 | 있음 | raw `+3.9068756` points → CSS `−3.9068756`; target 5의 경계에서 소비되지 않아 revision 0·offset 0 유지 |
| positive X | 있음 | 있음 | raw `−19.394547` points → CSS `+19.394547`; target 7, revision 1, inner X `+19.394547 CSS px` |
| negative X | 없음 | 없음 | XCTest method pass만 관찰 |
| positive Y | 없음 | 없음 | XCTest method pass만 관찰 |

두 callback 모두 `source=continuous-trackpad`, `unit=UIKitPoints`, `pointsPerCssPx=1.0`, `momentum_phase=None`, `modifiers=0`이었다. callback이 없는 case를 성공 입력으로 계산하거나 FFI 직접 enqueue로 대체하지 않았다. negative-Y의 처리 결과도 실제 offset 변화와 혼동하지 않는다.

## 구현

- `UIPanGestureRecognizer.allowedScrollTypesMask`를 discrete mouse와 continuous trackpad 두 recognizer로 나누고, 각 recognizer의 touch input을 비운다.
- gesture 누적 translation에서 직전 sample을 빼 callback 간 증분만 사용한다. `.began`의 nonzero sample은 보존하며 zero/duplicate terminal delta는 거부한다.
- canvas UIKit point를 CSS px로 변환하고, `delta_css = -translationDelta / pointsPerCssPx`를 적용한다. 이 부호는 현재 continuous Simulator 표본에서만 관찰했다.
- surface generation이 바뀐 첫 callback은 새 baseline으로 rebase하고 enqueue하지 않는다. invalid coordinate/scale, outside point, non-finite sample, Control-wheel, runtime not-ready/closing을 FFI 전에 거부한다.
- FFI report가 `status=0`과 `admitted=true`를 모두 만족할 때만 worker drain을 예약한다. momentum phase는 `None`으로 전달한다.
- adapter launch marker와 route allowlist는 C13.3 fixture compile flag 안에 둔다. 첫 hostile review가 발견한 feature-off allowlist 문자열 누출을 수정하고 기본 app/archive negative control을 다시 확인했다.

## 빌드·테스트와 artifact 경계

- Device: `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`, iPad Pro 13-inch (M5), iOS Simulator 26.2, arm64.
- Opt-in build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` 통과. 원본 Xcode 출력 [`xcodebuild-opt-in.log.gz`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/xcodebuild-opt-in.log.gz)을 보존했다.
- Default build: `SPINON_ENABLE_C04_RUNTIME_GPU=0 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0 mise exec -- bun run build:ios-sim` 통과. 원본 Xcode 출력 [`xcodebuild-default.log.gz`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/xcodebuild-default.log.gz)을 보존했다.
- XCUITest: 4 passed, 0 failed, 0 skipped. `xctest-four-directions.log`는 XCTest 결과이며 callback 표본은 `runtime-callbacks-sanitized.log`를 기준으로 2/4로 판정한다.
- Static source boundary: `node --test tools/css-reference/c13-3-ios-native-wheel-adapter.test.mjs` 5/5 통과.
- Opt-in app executable SHA-256: `46c19b1070ce41ff14084c5c33abf30603c2c6fc1143a213b6da72df91c49f6e`.
- Default app executable SHA-256: `3106d4e641d31f5b5f812a980d192274116c76167ef7a4e6a949cc35a6140358`; adapter launch marker가 없다.
- Default `libspinon_ffi.a` SHA-256: `26a72e2ad024fc7d1e86bb86275b77a255cc7c70562b0e6b964e84eb0eb3ea34`; `_spinon_runtime_gpu_host_enqueue_c13_3_wheel` symbol이 없다. Opt-in archive에서 해당 symbol을 확인했다.
- 상세 source/build 식별자는 [`source-revision.json`](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/source-revision.json)에 둔다.

## 화면과 해석

![iPad Pro 13-inch Simulator C13.3 fixture 장면](./ios-simulator-scene.png)

이 캡처는 opt-in 앱의 고정 GPU fixture 장면이 그려진 것을 보여준다. 이 캡처가 나온 별도 positive-X XCTest에서는 native adapter callback이 관찰되지 않았다. 따라서 스크린샷은 runtime scene 확인용이며 scroll 처리 성공 증거가 아니다. callback과 처리 여부는 위 표와 sanitized Simulator log가 증명하는 표본에만 한정한다.

## 2026-10-18 inner-target 왕복 재검증

기존 테스트는 generic canvas 중심점에서 scroll을 합성해 포인터가 내부 scrollport를 벗어날 수 있었다. 이번에는 XCUITest가 canvas의 inner-target 좌표 `(0.33, 0.467)`로 이동한 뒤 왕복 입력을 보냈다. 대상 앱 실행 파일 SHA-256은 기존 opt-in artifact hash와 같아 다른 앱 빌드가 결과에 섞이지 않았다.

| Run | XCTest | 앱 native callback / 처리 | 관찰 |
| --- | --- | --- | --- |
| 01, X/Y 왕복 | 2/2 pass | 4/4 | 네 sample 모두 target 7, 두 축 offset이 0으로 복귀 |
| 02, X/Y 반복 | 2/2 pass | 3/4 | positive-X와 ±Y는 처리, negative-X는 callback/process 로그 없음 |
| 03, X 왕복 | 1/1 pass | 2/2 | 두 sample 모두 target 7, inner X offset이 0으로 복귀 |

Run 01에서 horizontal은 CSS `+4.680664`, 이어서 `−17.822418`을 전달해 inner X를 0으로 복귀시켰다. Vertical은 `+16.963684`, 이어서 `−20.789566`을 전달해 inner Y를 0으로 복귀시켰다. Run 02의 vertical도 `+21.039627`, `−24.50859`을 처리해 0으로 돌아왔다. Run 03 horizontal은 `+9.574493`, `−11.279755`을 처리해 0으로 돌아왔다. 초과분은 해당 inner scroll range 밖 remainder로 남았으며 다른 owner에서 소비된 것으로 해석하지 않는다. 모든 callback은 `source=continuous-trackpad`, `unit=UIKitPoints`, `pointsPerCssPx=1.0`, `momentum_phase=None`, `modifiers=0`이었다.

수평 왕복은 3회 중 2회에서 두 native callback/process가 관찰됐다. Run 02는 XCUITest method와 negative-X attachment 단계가 pass됐지만 app log에 native callback이 없어 해당 입력은 실패로 계산한다. 빠진 단계가 XCTest event synthesis, UIKit recognizer, 또는 앱 adapter 중 어디인지는 이번 log만으로 분리되지 않았다. 이 결과는 inner-target 경로가 성공할 수 있음을 보이며, 매번 재현되는 ±X/±Y gate를 충족하지 않는다. [run summary](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/runtime-followup-summary.json) · [Run 01 X/Y logs](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-01-horizontal-events.log), [Run 02 log](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-02-events.log), [Run 03 log](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-03-horizontal-events.log) · [Run 02 XCUITest steps](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-02-xctest-steps.log), [Run 03 XCUITest steps](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-03-horizontal-xctest-steps.log), [Run 01/02/03 xcresult summaries](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-01-horizontal-xcresult-summary.json), [vertical](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-01-vertical-xcresult-summary.json), [repeat](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-02-xcresult-summary.json), [horizontal repeat](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/run-03-horizontal-xcresult-summary.json) · [XCUITest harness와 재현 명령](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/xctest-harness/README.md) · [PR 변경 실패 관점 검토 30개](./pr-delta-review-30-2026-10-18.md).

![iPad Pro 13-inch Simulator의 inner-target positive-X 실행 장면](./inner-target-reruns/ios-simulator-scene-horizontal-positive-x.png)

이 스크린샷은 세 번째 수평 왕복 run에서 첫 입력 뒤 캡처한 fixture scene이다. pixel 변화나 callback은 화면만으로 판정할 수 없으며 위 Simulator log가 native 처리 근거다.

## 2026-10-18 iOS negative-X 반복 및 입력 target 후속

같은 opt-in app executable SHA-256 `46c19b1070ce41ff14084c5c33abf30603c2c6fc1143a213b6da72df91c49f6e`를 iPad Pro 13-inch (M5), iOS 26.2 Simulator에서 실행했다.

| Run | XCTest | UIKit callback / queue 처리 | 판정 |
| --- | --- | --- | --- |
| 04, inner-target negative-only | 3/3 pass | 3/3 | 매번 target 7·negative delta였으나 시작 offset 0 경계에서 `published=false`; 소비·offset 변화 없음 |
| 05, inner-target 왕복 5회 | 5/5 pass | positive 5/5, negative 3/5 | negative callback 누락은 반복 1·5; 전달된 세 negative sample은 모두 target 7이고 offset 0 복귀는 없었음 |
| 06, 중심점 왕복 대조 | 결과 판정 불가 | partial log의 app process 4개 | `No space left on device`로 xcresult가 `result=unknown`, `totalTestCount=0`; gate에서 제외 |

Run 04의 observed CSS X delta는 `−5.1843414`, `−2.1793213`, `−20.7473`이었다. 이 테스트는 negative callback이 들어올 수 있음을 보였지만 inner offset이 이미 0이라 세 sample 모두 scroll state revision 0과 offset 0을 유지했다.

Run 05의 XCTest activity는 매 repetition에서 `+38`, `−38` 요청을 기록한다. 앱 callback의 실제 CSS delta는 positive `+7.907501`, `+19.50438`, `+20.234985`, `+4.2195435`, `+9.195129`, negative `−15.970673`, `−4.989746`, `−2.3658447`이었다. 따라서 실제 positive/negative 크기가 서로 같지 않았고 final inner X offset은 반복 순서대로 `7.907501`, `3.5337067`, `15.245239`, `1.8536987`, `9.195129 CSS px`였다. XCTest method 5/5 pass를 native callback pass와 혼동하지 않으며, 요청 delta `±38`을 app이 받은 UIKit delta로 간주하지 않는다.

Run 06의 부분 callback 좌표는 center variant의 target 한계를 추가로 드러냈다. 첫 app process는 positive-X `target=7`에서 CSS `+11.60434`, 후속 negative-X는 `target=5`에서 `−18.205032`를 기록했다. 다른 partial process도 negative sample에서 target 5를 선택했다. 다만 결과 번들이 정상 완성되지 않았으므로 callback 반복률이나 XCTest 성공률로 세지 않는다.

이 결과만으로 누락 callback 원인을 특정할 수 없다. Run 05의 누락 반복에는 app native callback, queue process, adapter reject가 모두 기록되지 않았고, Run 04의 단독 negative 입력은 3/3 전달됐다. 따라서 adapter 방향 정규화나 queue 거부가 원인이라고 결론 낼 근거가 없다. XCTest synthesis, Simulator event delivery와 UIKit recognizer를 분리하는 추가 계측이 필요하다. app 코드 수정이나 C13.3 완료 판정은 하지 않는다.

원본: [Run 04 summary](./inner-target-reruns/run-04-negative-x-repeat/run-summary.json) · [Run 04 XCTest summary](./inner-target-reruns/run-04-negative-x-repeat/xctest-summary.json) · [Run 04 app log](./inner-target-reruns/run-04-negative-x-repeat/app-events.log) · [Run 05 summary](./inner-target-reruns/run-05-horizontal-roundtrip-repeat/run-summary.json) · [Run 05 XCTest summary](./inner-target-reruns/run-05-horizontal-roundtrip-repeat/xctest-summary.json) · [Run 05 actions](./inner-target-reruns/run-05-horizontal-roundtrip-repeat/xctest-scroll-actions.json) · [Run 05 app log](./inner-target-reruns/run-05-horizontal-roundtrip-repeat/app-events.log) · [Run 06 partial summary](./inner-target-reruns/run-06-center-roundtrip-repeat/run-summary.json) · [Run 06 partial app log](./inner-target-reruns/run-06-center-roundtrip-repeat/app-events.log) · [입력 harness와 재현 조건](./inner-target-reruns/xctest-harness/README.md) · [이번 diff 30개 검토](./pr-delta-review-30-2026-10-18-followup.md) · [재실행 artifact SHA-256](./inner-target-reruns/SHA256SUMS).

## 미완료 범위

Discrete mouse callback, iOS Control-wheel runtime rejection, pinch/magnification, 실제 trackpad/device, surface resize/background runtime negative injection, 다른 points/CSS scale, 반복 가능한 전체 ±X/±Y matrix, screenshot pixel-diff와 compositor 이후 출력, public DOM event 및 S04.10 displayed-frame targeting을 확인하지 않았다. 앞선 inner-target 수평 반복 세 번 중 한 번은 negative-X app callback이 누락됐고, 이번 다섯 회 재현에서도 두 번 누락됐다. C13.3 체크 및 공식 완료 집계는 그대로 미완료다.
