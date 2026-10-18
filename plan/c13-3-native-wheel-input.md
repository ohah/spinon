# C13.3 · 네이티브 wheel·trackpad 입력 수집과 정규화

**상태:** Android API 37 AVD native mouse wheel 행렬 8/8, 기존 거부 3/3, Control-wheel 4/4, 복합 source 2/2와 ADB touchpad ACTION_SCROLL fixture route 1/1 통과. API 29 ARM64 AVD 최소 지원선 follow-up에서도 ±H/V·대각 8/8 및 Control-wheel 거부 4/4를 확인했다. iPadOS 26.2 Simulator의 opt-in continuous recognizer callback은 inner-target X/Y 왕복에서 한 번 네 방향 모두 queue→process까지 성공했고, 이후 반복에서는 X negative callback 누락 1회와 수평 왕복 재현 1회를 관찰했다. 전체 반복성은 아직 성립하지 않는다. discrete mouse, Control-wheel, 실기기 trackpad는 미검증이며 C13.3은 미완료다. [iOS adapter 실행 근거와 재검증 로그](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/README.md)
**ADB touchpad follow-up:** [실행·화면·artifact](../spec/internal/evidence/c13-3-android-avd-touchpad-point-2026-10-16/README.md) · [구현 검토 20개](../spec/internal/evidence/c13-3-android-avd-touchpad-point-implementation-review-20-2026-10-16.md) · [PR 변경 검토 30개](../spec/internal/evidence/c13-3-android-avd-touchpad-point-pr-delta-review-30-2026-10-16.md)
**Android API 29 follow-up:** [사전 판정·실행·화면·pixel·hash](../internal/evidence/c13-3-native-wheel-api29-2026-10-17/README.html) · [구현·실행 적대 검토 20개](../internal/evidence/c13-3-native-wheel-api29-2026-10-17/implementation-review-20.html) · [PR 변경 적대 검토 30개](../internal/evidence/c13-3-native-wheel-api29-2026-10-17/pr-delta-review-30.html)
**공식 상위 항목:** [구현 상태 대장](../spec/STATUS.md#css-구현-체크리스트)
**선행:** [C13.3 scroll gesture 계획](./c13-3-scroll-gestures.md), [normalized wheel router](./c13-3-wheel-delta-routing.md)
**계약:** [내부 계약 0063](../spec/internal/0063-c13-3-scroll-gestures.md) · 공개 `WheelEvent`나 DOM `scroll` API 변경 없음
**계획 실패 검토 20개:** [기록](../spec/internal/evidence/c13-3-native-wheel-input-plan-review-20-2026-10-16.md)
**계획 PR 변경 검토 20개:** [기록](../spec/internal/evidence/c13-3-native-wheel-input-pr-review-20-2026-10-16.md)
**Control-wheel 재현 계획 변경 적대 검토 20개:** [기록](../spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16/plan-review-20.md)
**Control-wheel 구현·실행 적대 검토 20개:** [기록](../spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16/implementation-review-20.md)
**Control-wheel PR 변경 적대 검토 20개:** [기록](../spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16/pr-delta-review-20.md)
**구현 실패 관점 검토 20개:** [기록](../spec/internal/evidence/c13-3-native-wheel-input-implementation-review-20-2026-10-16.md)
**PR #198 변경 관점 검토 30개:** [기록](../spec/internal/evidence/c13-3-native-wheel-input-pr-delta-review-30-2026-10-16.md)
**source 분류 수정 계획 검토 20개:** [기록](../spec/internal/evidence/c13-3-native-wheel-source-classification-plan-review-20-2026-10-16.md)
**source 분류 수정 구현·실행 검토 20개:** [기록](../spec/internal/evidence/c13-3-native-wheel-source-classification-implementation-review-20-2026-10-16.md)
**구현 전 비교 기준:** [사전 비교와 정규화 oracle](../spec/internal/evidence/c13-3-native-wheel-input-precomparison-2026-10-16/README.md) · [기계 판독 기준](../spec/internal/evidence/c13-3-native-wheel-input-precomparison-2026-10-16/normalization-oracle.json)

## 목표와 범위

Android와 iOS host가 OS input pipeline에서 전달받은 mouse wheel 및 trackpad scroll sample을 수집하고, native 좌표·delta 단위를 CSS px로 변환해 기존 bounded `WheelDelta` queue→snapshot router→draw 경로에 연결한다. 앱 route와 FFI는 `c13-3-gesture-fixture-test` opt-in build에서만 존재한다. Simulator 증거는 실제 기기나 실제 외장 입력장치 증거가 아니다.

지원 입력 경로:

- Android API 29 이상: `ACTION_SCROLL`의 `AXIS_HSCROLL`/`AXIS_VSCROLL`. 이벤트 자체의 `MotionEvent.getSource()`와 `getToolType(0)`로 mouse와 trackpad를 구별한다. pointer가 없으면 source가 touchpad여도 fail-closed한다. 명시적 `SOURCE_TOUCHPAD`는 trackpad이고, `SOURCE_MOUSE`는 pointer tool이 finger면 trackpad, mouse면 mouse다. 미확인 조합은 거부한다. `InputDevice.getSources()`는 장치가 지원하는 source 합집합이므로 현재 이벤트 분류에 사용하지 않고 진단에만 남긴다. Android `View` 문서는 touchpad 두 손가락 scroll도 이 action과 축으로 보고하도록 설명한다. API 34 classified-move gesture-distance axes는 이번 slice에서 지원하지 않는다.
- iPadOS Simulator: UIKit `UIPanGestureRecognizer.allowedScrollTypesMask`를 discrete(mouse)와 continuous(trackpad) recognizer로 분리한다. scroll recognizer의 `allowedTouchTypes`는 비워 기존 finger pan과 중복 처리하지 않는다.
- host가 scroll로 판정한 입력만 받는다. Android Ctrl-wheel과 pinch classification, iOS `UIGestureRecognizer.modifierFlags`의 Control-wheel, pinch classification, unknown source/action은 queue에 넣지 않는다. iOS는 scroll recognizer만 연결하고 pinch recognizer는 추가하지 않는다.
- OS API가 momentum phase를 제공하지 않으면 `RuntimeScrollMomentumPhase::None`으로 기록한다. recognizer의 ended 상태를 momentum ended로 추정하지 않는다.
- 공개 DOM event, `WheelEvent`, CSSOM, keyboard scroll, fling/inertia, zoom/magnification, snap, RTL/logical axis, S04.10 displayed-frame target, 실기기·hardware GPU·compositor 이후 출력은 범위 밖이다.

## 비교 기준과 고정 입력

- 기준은 Chrome for Testing `155.0.8059.40`의 고정 400×240 CSS px fixture와 [기존 C13.3 precomparison](../spec/internal/evidence/c13-3-scroll-gesture-precomparison-2026-10-15/README.md)이다. Chrome DOM wheel event는 Chrome 관찰에만 속한다.
- WheelEvent의 양의 X/Y는 오른쪽/아래쪽 document scroll 방향이다. [UI Events](https://www.w3.org/TR/uievents/#wheelevent)의 default scroll 방향도 오른쪽·아래쪽을 양수로 둔다. 플랫폼 부호와 화면 offset은 양/음 수평·수직 입력으로 검증한다.
- native point를 CSS point로 변환하고 enqueue snapshot에서 clip-aware target을 한 번 고정한다. 기본 point는 inner target `(132,112)` CSS px이며, 수평·수직·대각과 inner/outer/root 경계 입력을 각각 둔다.
- Android: `ViewConfiguration` horizontal/vertical scroll factor가 axis value를 physical px로 바꾼다. `deltaX_css = HSCROLL × horizontalFactorPx / surfacePxPerCssPx`; `deltaY_css = -(VSCROLL × verticalFactorPx) / surfacePxPerCssPx`. 부호는 Android viewport 방향과 UI Events CSS 방향을 대조한다. 원본 axis, factor, physical raw delta, surface scale, CSS delta를 기록한다. normalized modifier bit는 `SHIFT=1`, `CONTROL=2`, `ALT=4`, `META=8`이며 Control-wheel은 queue 전 거부한다.
- iOS: `translation(in:)`은 gesture 시작부터 누적값이므로 callback 간 차분만 보낸다. iPadOS 26.2 Simulator continuous callback에서 관찰한 부호에 따라 `delta_css = -translationDelta_points / pointsPerCssPx`를 구현했다. 현재 point scale은 `1.0 points/CSS px`였고, 양의 X sample은 inner owner를 오른쪽으로 19.394547 CSS px 이동시켰다. 양의 Y normalized sample은 scrollport 경계에서 revision/offset 변화 없이 처리됐다. callback 전달은 간헐적이며 discrete callback·전체 방향 재현성은 미확정이다.
- AVD에서는 API 37 ARM64 emulator의 Android input shell에서 mouse와 touchpad scroll source를 주입하고 앱 `onGenericMotionEvent`의 action/source/axis/factor를 확인한다. ADB-created input/device id는 물리 입력장치 증거가 아니다.
- ADB `input keyevent`와 별도 `input mouse scroll` 명령은 Android callback에 Control meta를 전달하지 못했다. API 37 ARM64 AVD의 `/dev/uinput`에서 가상 mouse device 하나가 `EV_KEY KEY_LEFTCTRL/KEY_RIGHTCTRL`을 누른 채 `EV_REL REL_HWHEEL/REL_WHEEL`을 보낸 뒤 key-up과 `UI_DEV_DESTROY`를 수행한다. 앱 callback에서 left Control `meta=12288`, right Control `meta=20480`, 양쪽 모두 normalized `modifiers=2`와 `reason=control-wheel`을 관찰하고, queue/process 부재, revision 1, 초기 offsets, SurfaceView 픽셀 무변경, device 제거와 meta state 0을 확인한다. 이 입력은 Android AVD InputReader 경로의 가상 장치 근거이며 실기기나 외장 장치 근거가 아니다.
- Control-wheel 거부 행렬은 left/right Ctrl × horizontal/vertical 4 case를 각각 fresh fixture launch로 검사하고, no-Control `ACTION_SCROLL`을 positive control로 실행해 modifier 0과 queue→draw revision 2를 확인한다.
- iPadOS Simulator에서는 discrete mouse와 continuous trackpad source를 각각 주입한다. 최신 XCTest는 continuous 경로만 실행했고 4개 방향 test method는 통과했지만 최종 반복에서 native callback은 2/4 case에서만 확인됐다. FFI direct enqueue로 대체하지 않으며 `.discrete` 입력은 미검증으로 남긴다. [최신 실행 근거](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/README.md)
- 2026-10-16 Android API 37 ARM64 AVD에서는 mouse `ACTION_SCROLL`을 native callback→queue→draw까지 연결했다. ±H/V와 네 대각 조합 8/8에서 factor `168 px`, scale `2.625 px/CSS px`, axis magnitude `64 CSS px`, target `7`, revision 2와 각 owner offset이 고정 oracle과 일치했다. SurfaceView 밖 점, 위치 없는 ADB touchpad sample, 영 축은 3/3 거부됐고 revision/offset은 시작값을 유지했다. [native Android wheel 실행·pixel·artifact 근거](../spec/internal/evidence/c13-3-native-wheel-input-2026-10-16/README.md) · [동일 AVD 행렬 fresh rerun](../spec/internal/evidence/c13-3-native-wheel-input-avd-followup-2026-10-16/README.md).
- 같은 AVD 재검증에서 `InputDevice.getSources()` 지원 source 합집합으로 분류하던 문제를 수정했다. 복합 장치의 mouse event가 touchpad로 오분류되는 경우를 JVM legacy negative control로 재현했고, pointer가 없는 명시적 touchpad source도 fail-closed하는 classifier case를 고정했다. 실제 AVD mouse/touchpad event source와 pointer tool 진단을 확인했다. AVD에는 두 source를 함께 지원하는 실제 복합 장치가 없으므로 하드웨어 재현으로 확대하지 않는다. [source 분류기 수정·AVD 재검증 근거](../spec/internal/evidence/c13-3-native-wheel-source-classification-2026-10-16/README.md).
- API 37 ARM64 AVD에 test-only `uinput` composite 장치를 만들어 `SOURCE_MOUSE`와 `SOURCE_TOUCHPAD` 동시 지원을 InputReader에서 확인했다. 입력 callback은 event source `0x2002`, pointer tool `mouse`, 장치 지원 source 합집합 `0x10200a`를 함께 기록하고 `MouseWheel`로 분류했다. 내부 입력은 target 7·sequence 1·64 CSS px·revision 2까지 처리됐고 SurfaceView 바깥 입력은 queue 전에 거부됐다. 정렬한 root scrollport pixel 비교는 내부 changed 50,774·외부 0, 거부 대조는 내부/외부 모두 0이었다. 두 case 뒤 가상 장치 제거를 확인했다. [복합 source 분류·화면·pixel 근거](../spec/internal/evidence/c13-3-native-wheel-composite-avd-2026-10-16/README.md). 이는 AVD InputReader의 가상 장치 근거이며 물리 외장 장치 검증은 아니다.
- 별도 ADB keyboard/mouse 명령은 modifier를 연결하지 못했지만, 같은 API 37 ARM64 AVD의 virtual uinput device는 left/right Ctrl과 H/V wheel을 Android `ACTION_SCROLL` callback까지 전달했다. 4/4 거부에서 `modifiers=2`, queue/process 0건, revision 1, 초기 offsets와 SurfaceView crop 0 changed pixels를 확인했다. 모든 case 뒤 가상 device가 제거됐고 입력 meta state는 0이었다. 일반 wheel positive control은 modifier 0, target 7, revision 2와 기대 offsets를 만들었다. [재현 도구와 Control-wheel AVD 근거](../spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16/README.md).
- 2026-10-18 최초 preflight에서는 host `loginwindow`가 Simulator GUI를 가려 callback을 관찰하지 못했다. 이 결과는 [당시 preflight 기록](../spec/internal/evidence/c13-3-ios-native-wheel-preflight-2026-10-18/README.md)에 보존한다. 후속 XCTest 실행에서는 continuous callback 일부와 sign을 관찰해 adapter를 구현했으며, 현재 oracle은 이 후속 결과와 남은 미검증 범위를 분리해 기록한다.

## 구현 절차

1. 기존 fixture lifecycle과 opt-in boundary를 확인하고 opt-in build에서만 native listeners/recognizers를 추가한다.
2. **완료 · Android API 37 AVD:** generic-motion callback에서 `ACTION_SCROLL`과 허용 event source/tool 조합만 선택한다. `InputDevice.getSources()`는 분류 입력으로 쓰지 않는다. factor와 surface scale이 finite·positive인지 검증한다. Ctrl-wheel, zero axes, invalid point와 source/action/tool은 queue 전에 reject한다. Android `MotionEvent` route에 pinch recognizer를 추가하지 않는다.
3. **부분 구현 · iOS continuous Simulator callback:** iOS discrete/continuous scroll recognizer를 각각 구성해 `MouseWheel`/`TrackpadScroll` source를 구분한다. recognizer별 translation baseline을 유지하고 직전 관찰값 차분을 enqueue한다. `modifierFlags`의 Control-wheel reject와 touch 중복 방지 경로를 추가했다. continuous callback은 일부 case에서 queue→process까지 관찰했지만 discrete callback, Control-wheel runtime, 반복 가능한 전체 축 행렬은 남아 있다.
4. 양 플랫폼은 raw delta, native unit, units-per-CSS scale, modifiers, source, phase, CSS point/delta, surface generation, render key, target, admission/result를 기록한다. callback에서 worker 완료를 동기 대기하지 않는다.
5. API에 명시적인 momentum phase가 없으면 `None`을 보낸다. UIKit began/changed/ended를 momentum lifecycle로 오해하지 않는다.
6. queue pressure, snapshot barrier, surface replacement, background/close 뒤 동작은 기존 bounded queue와 generation/lineage contract를 따른다. 거부된 delta는 후속 sample에 누적 재전송하지 않는다.

## 사전 판정과 적대 경로

### 성공 조건

- Android API 37 AVD mouse callback의 source/raw 값과 queue/draw 결과가 주입 manifest와 일치한다. iPadOS 26.2 Simulator continuous callback은 부분 관찰됐다. 전체 iOS gate는 미통과다.
- Ctrl-left/right × horizontal/vertical은 callback에서 Control modifier로 판정돼 queue 전에 거부되고, ordinary wheel positive control은 queue와 draw까지 진행한다.
- Android 결과는 ViewConfiguration factors와 surface px/CSS px 변환식, iOS 결과는 translation 차분·point scale 및 Simulator 관찰로 선택한 부호식과 0.01 CSS px 이내 일치한다. 이 iOS 기준은 callback이 실제 전달된 표본에만 적용한다.
- 양/음 H/V와 대각 입력은 CSS right-handed sign 및 고정 Chrome geometry oracle에 맞는다. normalized delta는 scroll owner 간 보존되고 화면 변화는 기대 surface 안에 제한된다.
- source/unit/scale/phase/modifier/target이 queue report까지 유지된다. 같은 iOS cumulative translation을 두 번 처리해도 두 번째 delta는 0이며 scroll revision이 증가하지 않는다.
- Android 분류 oracle은 pointer 부재면 source와 무관하게 거부, `SOURCE_TOUCHPAD → TrackpadScroll`, `SOURCE_MOUSE + finger → TrackpadScroll`, `SOURCE_MOUSE + mouse → MouseWheel`, unknown 조합 거부다. 장치 지원 source union을 바꾸어도 현재 event 판정은 달라지지 않는다.
- feature-off Android/iOS app artifact에 fixture marker, FFI symbol/string이 없고 opt-in positive control에 존재한다. public API와 C13.3 completion 체크는 바뀌지 않는다.

### 거부·복구 조건

- NaN/infinite/0/negative scale 또는 factor, 잘못된 point, clip-aware hit miss, unknown source/action, Ctrl-wheel, Control-wheel, pinch/zoom은 queue/offset/revision을 바꾸지 않는다.
- stylus/touch/mouse hover 및 ordinary finger pan은 wheel path에 들어오지 않는다. iOS scroll recognizer는 touch를 받지 않고 Android generic scroll route는 finger `ACTION_MOVE`를 wheel로 취급하지 않는다.
- surface generation 변경·scene replacement·background 뒤 이전 point/scale/baseline을 재사용하지 않는다. stale target은 새 위치에서 재-hit-test하지 않고 reject한다.
- queue capacity rejection/coalescing은 기존 queue identity와 epoch 계약을 유지한다. 거부된 delta가 다음 iOS cumulative translation 차분에 다시 포함되지 않는다.
- pan/wheel interleaving, background/close callback, ended/cancelled 중복, iOS recognizer 경합, zero/repeated/subpixel delta, axis boundary, root exhaustion, malformed enum, feature-off invocation을 별도 negative control로 둔다.

## 2026-10-16 ADB touchpad raw-origin follow-up

input touchpad scroll은 API 37 AVD에서 ACTION_SCROLL, source 0x100008, raw point (0,0)으로 callback에 도달했다. 기본 SurfaceView origin (15,991)에서는 이 점이 표면 밖이라 queue 전에 거부된다. 위치 정보가 빠진 입력을 이전 cursor 위치로 보완하지 않는다.

fixture-test build와 spinon_c133_touchpad_origin_target intent가 함께 켜진 경우에만 SurfaceView를 이동해 raw (0,0)을 Chrome fixture target (132,112) CSS px에 대응시킨다. AVD 실행에서 계산점 (132.19048,112.0), queue sequence 1, target 7, draw revision 2와 outer owner X offset 64 CSS px를 확인했다. 기본 surface 거부 대조는 revision/offset을 유지했고 두 screenshot hash가 같았다. AVD 입력 shell과 test-only view 배치는 물리 trackpad 검증이 아니다. [실행·화면·로그·artifact 근거](../spec/internal/evidence/c13-3-android-avd-touchpad-point-2026-10-16/README.md).

## 2026-10-17 Android API 29 minimum-API follow-up

`spinon_r05_api29_fallback` Android 10 / API 29 Google APIs ARM64 AVD에서 opt-in APK를 실행했다. API 29 ADB `input`에는 `mouse scroll` 명령이 없어 AVD 내부 `/dev/uinput`에 생성·제거하는 test-only virtual mouse로 `REL_HWHEEL`/`REL_WHEEL`을 보냈다. 앱은 실제 `ACTION_SCROLL`/`SOURCE_MOUSE` callback을 받았으며 FFI direct enqueue는 사용하지 않았다.

±H/V와 네 대각 입력 8/8에서 `ViewConfiguration` factor 168 physical px, SurfaceView scale 2.625 px/CSS px, 각 축 64 CSS px, point `(132.19048,112)` CSS px, queue sequence 1·target 7·revision 2 및 owner offsets가 기존 oracle과 일치했다. 좌우 Control × H/V는 `modifiers=2`로 queue 전에 4/4 거부됐다. positive-positive diagonal의 aligned SurfaceView 661,500 pixel 비교는 scrollport 바깥 changed 0, 안쪽 50,684였고 Control-left horizontal 거부 대조는 661,500 pixel 모두 unchanged였다. [API 29 사전 판정·실행·화면·pixel·hash](../internal/evidence/c13-3-native-wheel-api29-2026-10-17/README.html).

이 AVD는 GL/ANGLE/SwiftShader 소프트웨어 renderer이며 4 KB page size다. AVD uinput 결과를 물리 mouse/trackpad나 실기기 동작으로 확대하지 않는다. 당시까지 iOS UIKit callback/sign 미관찰로 adapter gate가 미통과였고, 후속 부분 관찰은 아래 2026-10-18 실행 섹션에 별도로 기록한다. C13.3 전체는 미완료다.

## 2026-10-18 iPadOS Simulator native wheel preflight 재실행 (당시 결과)

iPad Pro 13-inch (M5) / iOS 26.2 Simulator의 C13.3 opt-in 앱을 현재 소스와 고정 V8 revision으로 다시 빌드했다. `SPINON_C133_READY`에서 고정 canvas와 초기 scroll state를 확인했다. host WindowServer 목록에는 Simulator 창보다 위에 전체 화면 `loginwindow` layer가 있었고 전경 앱은 System Settings였다. Simulator 활성화 요청 뒤에도 전경 앱이 바뀌지 않아 해당 preflight에서는 scroll event를 보내지 않았다. 후속 결과는 아래 별도 섹션에 기록한다. [당시 빌드·ready 화면·host window 관찰·hash 근거](../spec/internal/evidence/c13-3-ios-native-wheel-preflight-2026-10-18/README.md).

## 2026-10-18 iPadOS continuous native wheel adapter 후속

iPad Pro 13-inch (M5), iOS 26.2 Simulator에서 source의 최종 adapter를 opt-in으로 빌드하고, XCUITest 4개 방향 method를 실행했다. XCTest는 4/4 pass·0 fail이지만, 이것은 각 테스트의 UI 자동화 완료를 뜻하며 native callback 4방향 통과와 같지 않다. 최종 matrix run의 Simulator unified log에서 continuous callback은 negative-Y와 positive-X 두 case에만 보였다. negative-Y는 raw `+3.9068756` UIKit points를 normalized `−3.9068756` CSS px로 queue/process했으나 inner/outer 경계에서 소비되지 않아 revision/offset은 그대로였다. positive-X는 raw `−19.394547` points를 normalized `+19.394547` CSS px로 처리해 target 7, revision 1, inner X offset `+19.394547`을 만들었다. 나머지 두 method에서는 callback이 관찰되지 않았다. `pointsPerCssPx=1.0`, source `TrackpadScroll`, unit `UIKitPoints`, phase `None`이었다.

구현은 opt-in `UIPanGestureRecognizer` discrete/continuous adapter를 추가한다. callback 간 cumulative translation 차분만 사용하고, zero delta·invalid scale/coordinate·Control-wheel·stale runtime/surface 입력을 queue 전에 거부한다. generation 변경 때 이전 baseline을 재사용하지 않도록 한 sample을 새 baseline으로 재설정하고, queue report가 accepted일 때만 worker drain을 예약한다. 손가락 입력과 겹치지 않게 두 recognizer의 `allowedTouchTypes`는 비워 둔다. discrete mouse·Control-wheel·lifecycle/resize negative runtime injection은 이번 실행에서 확인하지 않았다.

opt-in iOS Simulator build와 default feature-off build가 모두 통과했다. opt-in executable hash는 `46c19b1070ce41ff14084c5c33abf30603c2c6fc1143a213b6da72df91c49f6e`, default executable hash는 `3106d4e641d31f5b5f812a980d192274116c76167ef7a4e6a949cc35a6140358`이다. Default executable에서 adapter marker와 default Rust archive에서 wheel enqueue symbol이 없고, opt-in artifact에서는 두 항목이 확인됐다. adapter 정적 회귀 검사는 5/5 통과했다. 실행 화면은 fixture scene을 보여주지만 화면 capture 단독으로 scroll callback을 증명하지 않는다. [실행·log·화면·hash·artifact 근거](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/README.md) · [구현 적대 검토 20개](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-implementation-review-20-2026-10-18.md).

## 2026-10-18 iPadOS Simulator inner-target 왕복 재검증

기존 중심점 XCUITest는 합성 scroll 중 포인터가 inner scrollport 밖으로 이동해, callback이 오더라도 다른 hit target을 선택할 수 있었다. 별도 inner-target 좌표 `(0.33, 0.467)`를 hover한 뒤 continuous scroll을 보낸 XCUITest로 재검증했다. 설치된 Spinon app executable SHA-256은 원래 adapter 실행 근거의 opt-in hash `46c19b1070ce41ff14084c5c33abf30603c2c6fc1143a213b6da72df91c49f6e`와 일치한다.

첫 run은 수평·수직 왕복 2개 XCTest가 각각 통과했고, Simulator app log에서 네 sample 모두 native callback→queue→process까지 도달했다. 네 sample 모두 target 7이었고 inner X/Y offset은 반대 방향 sample 처리 뒤 0으로 돌아왔다. 다음 전체 repeat는 XCTest 2/2 pass였지만 앱 log는 3/4 callback이었다. positive-X와 ±Y는 처리됐고 negative-X에는 앱 callback/process 행이 없었다. 세 번째 수평 반복은 XCTest 1/1 pass와 native callback/process 2/2를 기록해 target 7·offset 0으로 복귀했다. 따라서 수평 왕복은 두 번 성공했으나 3회 중 1회 음의 X callback이 재현되지 않았다. XCTest method pass는 callback pass로 계산하지 않는다.

이 재현만으로 누락 callback의 원인을 특정할 수 없다. Simulator event synthesis, UIKit recognizer와 앱 adapter 중 어느 단계에서 빠졌는지 추가 계측이 필요하다. 성공 sample은 `source=continuous-trackpad`, `unit=UIKitPoints`, `pointsPerCssPx=1.0`, `momentum_phase=None`, `modifiers=0`이었다. [실행 횟수·판정](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/runtime-followup-summary.json) · [XCUITest 입력 드라이버와 재현 명령](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/inner-target-reruns/xctest-harness/README.md) · [PR 변경 실패 관점 검토 30개](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/pr-delta-review-30-2026-10-18.md).

## 실행·증거 경계

Android API 37 ARM64 AVD와 iPadOS 26.2 Simulator에서 opt-in fixture를 실행한다. Android 실행은 [방향·거부 행렬과 화면 근거](../spec/internal/evidence/c13-3-native-wheel-input-2026-10-16/README.md)에, iOS 결과는 [preflight 기록](../spec/internal/evidence/c13-3-ios-native-wheel-preflight-2026-10-18/README.md)과 [adapter follow-up](../spec/internal/evidence/c13-3-ios-native-wheel-adapter-2026-10-18/README.md)에 기록했다. run마다 simulator/device ID와 OS, build flag, app/library hash, source/action, raw input, factor/scale, expected CSS delta, queue sequence/admission, before/after render key·revision·owner offsets, screenshot과 boundary pixel 비교를 보존한다. PR 본문에 iOS Simulator fixture scene을 첨부할 때는 해당 캡처 자체의 callback/process 여부를 함께 표시하고, Agent Browser에서 이미지 표시를 확인한다. 터미널 브라우저와 물리 기기는 사용하지 않는다.

검증을 통과해도 fixture input adapter에 한정한다. public DOM event dispatch, default-prevention, Android API 28 이하, 물리 장치, 실제 외장 trackpad, discrete mouse, 완전한 iOS ±X/±Y callback matrix, Control-wheel/lifecycle/resize injection, OS momentum/fling, WPT, S04.10, S05.2, 전체 CSS profile과 C13.3 완료는 별도 gate다.
