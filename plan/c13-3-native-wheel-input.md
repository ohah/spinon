# C13.3 · 네이티브 wheel·trackpad 입력 수집과 정규화

**상태:** Android API 37 AVD native mouse wheel adapter 방향 행렬 8/8, 기존 거부 경로 3/3, Control-wheel 좌우 Ctrl·수평/수직 거부 4/4, uinput 복합 source 대조 2/2 통과 · iPadOS Simulator callback은 host `loginwindow` 상태에서 관찰되지 않아 iOS sign/adapter는 보류
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
- iOS: `translation(in:)`은 gesture 시작부터 누적값이므로 callback 간 차분만 보낸다. 후보식은 `delta_css = -translationDelta_points / pointsPerCssPx`다. Apple은 translation 누적 의미를 보장하지만 CSS wheel sign을 규정하지 않으므로 iPadOS Simulator의 ±X/±Y 관찰로 구현 전에 부호를 확정한다. 반대 방향이면 계획 oracle부터 갱신한다.
- AVD에서는 API 37 ARM64 emulator의 Android input shell에서 mouse와 touchpad scroll source를 주입하고 앱 `onGenericMotionEvent`의 action/source/axis/factor를 확인한다. ADB-created input/device id는 물리 입력장치 증거가 아니다.
- ADB `input keyevent`와 별도 `input mouse scroll` 명령은 Android callback에 Control meta를 전달하지 못했다. API 37 ARM64 AVD의 `/dev/uinput`에서 가상 mouse device 하나가 `EV_KEY KEY_LEFTCTRL/KEY_RIGHTCTRL`을 누른 채 `EV_REL REL_HWHEEL/REL_WHEEL`을 보낸 뒤 key-up과 `UI_DEV_DESTROY`를 수행한다. 앱 callback에서 left Control `meta=12288`, right Control `meta=20480`, 양쪽 모두 normalized `modifiers=2`와 `reason=control-wheel`을 관찰하고, queue/process 부재, revision 1, 초기 offsets, SurfaceView 픽셀 무변경, device 제거와 meta state 0을 확인한다. 이 입력은 Android AVD InputReader 경로의 가상 장치 근거이며 실기기나 외장 장치 근거가 아니다.
- Control-wheel 거부 행렬은 left/right Ctrl × horizontal/vertical 4 case를 각각 fresh fixture launch로 검사하고, no-Control `ACTION_SCROLL`을 positive control로 실행해 modifier 0과 queue→draw revision 2를 확인한다.
- iPadOS Simulator에서 mouse wheel/discrete와 trackpad/continuous scroll을 주입한다. UIKit callback/source mask가 관찰되지 않으면 FFI direct enqueue로 대체하지 않고 미검증으로 남긴다.
- 2026-10-16 Android API 37 ARM64 AVD에서는 mouse `ACTION_SCROLL`을 native callback→queue→draw까지 연결했다. ±H/V와 네 대각 조합 8/8에서 factor `168 px`, scale `2.625 px/CSS px`, axis magnitude `64 CSS px`, target `7`, revision 2와 각 owner offset이 고정 oracle과 일치했다. SurfaceView 밖 점, 위치 없는 ADB touchpad sample, 영 축은 3/3 거부됐고 revision/offset은 시작값을 유지했다. [native Android wheel 실행·pixel·artifact 근거](../spec/internal/evidence/c13-3-native-wheel-input-2026-10-16/README.md) · [동일 AVD 행렬 fresh rerun](../spec/internal/evidence/c13-3-native-wheel-input-avd-followup-2026-10-16/README.md).
- 같은 AVD 재검증에서 `InputDevice.getSources()` 지원 source 합집합으로 분류하던 문제를 수정했다. 복합 장치의 mouse event가 touchpad로 오분류되는 경우를 JVM legacy negative control로 재현했고, pointer가 없는 명시적 touchpad source도 fail-closed하는 classifier case를 고정했다. 실제 AVD mouse/touchpad event source와 pointer tool 진단을 확인했다. AVD에는 두 source를 함께 지원하는 실제 복합 장치가 없으므로 하드웨어 재현으로 확대하지 않는다. [source 분류기 수정·AVD 재검증 근거](../spec/internal/evidence/c13-3-native-wheel-source-classification-2026-10-16/README.md).
- API 37 ARM64 AVD에 test-only `uinput` composite 장치를 만들어 `SOURCE_MOUSE`와 `SOURCE_TOUCHPAD` 동시 지원을 InputReader에서 확인했다. 입력 callback은 event source `0x2002`, pointer tool `mouse`, 장치 지원 source 합집합 `0x10200a`를 함께 기록하고 `MouseWheel`로 분류했다. 내부 입력은 target 7·sequence 1·64 CSS px·revision 2까지 처리됐고 SurfaceView 바깥 입력은 queue 전에 거부됐다. 정렬한 root scrollport pixel 비교는 내부 changed 50,774·외부 0, 거부 대조는 내부/외부 모두 0이었다. 두 case 뒤 가상 장치 제거를 확인했다. [복합 source 분류·화면·pixel 근거](../spec/internal/evidence/c13-3-native-wheel-composite-avd-2026-10-16/README.md). 이는 AVD InputReader의 가상 장치 근거이며 물리 외장 장치 검증은 아니다.
- 별도 ADB keyboard/mouse 명령은 modifier를 연결하지 못했지만, 같은 API 37 ARM64 AVD의 virtual uinput device는 left/right Ctrl과 H/V wheel을 Android `ACTION_SCROLL` callback까지 전달했다. 4/4 거부에서 `modifiers=2`, queue/process 0건, revision 1, 초기 offsets와 SurfaceView crop 0 changed pixels를 확인했다. 모든 case 뒤 가상 device가 제거됐고 입력 meta state는 0이었다. 일반 wheel positive control은 modifier 0, target 7, revision 2와 기대 offsets를 만들었다. [재현 도구와 Control-wheel AVD 근거](../spec/internal/evidence/c13-3-android-control-wheel-runtime-2026-10-16/README.md).
- 같은 날 iPad Pro 13-inch (M5)/iOS 26.2 Simulator 앱은 ready였지만 호스트 `loginwindow`가 Simulator GUI를 가려 CGEvent가 앱 callback까지 도달하지 않았다. UIKit sign은 미관찰로 남기고 iOS adapter 구현 gate를 유지한다.
- `normalization-oracle.json`은 Android adapter 결과와 iOS 미관찰 상태를 함께 기록한다. Android mouse slice는 통과했지만 iOS는 native sign gate를 통과할 때까지 adapter 구현을 보류한다.

## 구현 절차

1. 기존 fixture lifecycle과 opt-in boundary를 확인하고 opt-in build에서만 native listeners/recognizers를 추가한다.
2. **완료 · Android API 37 AVD:** generic-motion callback에서 `ACTION_SCROLL`과 허용 event source/tool 조합만 선택한다. `InputDevice.getSources()`는 분류 입력으로 쓰지 않는다. factor와 surface scale이 finite·positive인지 검증한다. Ctrl-wheel, zero axes, invalid point와 source/action/tool은 queue 전에 reject한다. Android `MotionEvent` route에 pinch recognizer를 추가하지 않는다.
3. iOS discrete/continuous scroll recognizer를 각각 구성해 `MouseWheel`/`TrackpadScroll` source를 구분한다. recognizer별 translation baseline을 유지하고 직전 관찰값과의 차분만 enqueue한다. `modifierFlags`를 읽어 Control-wheel을 reject하고 finger input이 중복되지 않는지 확인한다.
4. 양 플랫폼은 raw delta, native unit, units-per-CSS scale, modifiers, source, phase, CSS point/delta, surface generation, render key, target, admission/result를 기록한다. callback에서 worker 완료를 동기 대기하지 않는다.
5. API에 명시적인 momentum phase가 없으면 `None`을 보낸다. UIKit began/changed/ended를 momentum lifecycle로 오해하지 않는다.
6. queue pressure, snapshot barrier, surface replacement, background/close 뒤 동작은 기존 bounded queue와 generation/lineage contract를 따른다. 거부된 delta는 후속 sample에 누적 재전송하지 않는다.

## 사전 판정과 적대 경로

### 성공 조건

- Android API 37 AVD mouse callback의 source/raw 값과 queue/draw 결과가 주입 manifest와 일치한다. iPadOS 26.2 Simulator callback은 아직 관찰되지 않아 iOS gate는 미통과다.
- Ctrl-left/right × horizontal/vertical은 callback에서 Control modifier로 판정돼 queue 전에 거부되고, ordinary wheel positive control은 queue와 draw까지 진행한다.
- Android 결과는 ViewConfiguration factors와 surface px/CSS px 변환식, iOS 결과는 translation 차분·point scale 및 preflight에서 확정한 부호식과 0.01 CSS px 이내 일치한다.
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

## 실행·증거 경계

Android API 37 ARM64 AVD와 iPadOS 26.2 Simulator에서 opt-in fixture를 실행한다. Android 실행은 [방향·거부 행렬과 화면 근거](../spec/internal/evidence/c13-3-native-wheel-input-2026-10-16/README.md)에, iOS 미관찰은 [preflight 근거](../spec/internal/evidence/c13-3-native-wheel-input-preflight-2026-10-16/README.md)에 기록했다. run마다 simulator/device ID와 OS, build flag, app/library hash, source/action, raw input, factor/scale, expected CSS delta, queue sequence/admission, before/after render key·revision·owner offsets, screenshot과 boundary pixel 비교를 보존한다. PR 본문에는 검증된 Android Simulator 화면을 올리고 Agent Browser로 이미지 표시를 확인한다. iOS 화면은 실제 native callback이 관찰되기 전까지 성공 화면으로 표시하지 않는다. 터미널 브라우저와 물리 기기는 사용하지 않는다.

검증을 통과해도 fixture input adapter에 한정한다. public DOM event dispatch, default-prevention, Android API 28 이하, 물리 장치, 실제 외장 trackpad, OS momentum/fling, iOS callback/sign/adapter, WPT, S04.10, S05.2, 전체 CSS profile과 C13.3 완료는 별도 gate다.
