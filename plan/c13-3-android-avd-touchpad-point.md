# C13.3 · Android AVD targetable touchpad 입력 후속 계획

**상위 계획:** [C13.3 native wheel·trackpad 입력](c13-3-native-wheel-input.md) · [공식 상태 대장](../spec/STATUS.md)
**상태:** Android API 37 AVD의 ADB ACTION_SCROLL을 test-only SurfaceView 이동으로 target 7·draw revision 2까지 연결했다. 실제 물리 touchpad와 iOS callback은 미검증이며 C13.3은 미완료다.
**대상:** Android API 37 ARM64 AVD와 opt-in `c13-3-gesture-fixture-test`만 사용한다. 실기기·hardware GPU·공개 DOM event는 범위 밖이다.
**계획 검토:** [실패 관점 20개](../spec/internal/evidence/c13-3-android-avd-touchpad-point-plan-review-v2-20-2026-10-16.md)
**구현 검토:** [실패 관점 20개](../spec/internal/evidence/c13-3-android-avd-touchpad-point-implementation-review-20-2026-10-16.md)
**PR 변경 검토:** [실패 관점 30개](../spec/internal/evidence/c13-3-android-avd-touchpad-point-pr-delta-review-30-2026-10-16.md)

## 실패 원인 재현과 기준

Android API 37 AVD에서 기존 `input touchpad scroll --axis HSCROLL,1 --axis VSCROLL,0` 명령은 앱 callback에 `event_source_bits=0x100008`, `pointer_count=1`, `pointer_tool=finger`, `point_screen_px=0,0`을 전달했다. 화면 표면 시작점은 `(15,991)`이므로 앱은 `point-outside-surface`로 거부했다. 재현 결과는 [C13.3 native wheel 실행 기록](../spec/internal/evidence/c13-3-native-wheel-input-2026-10-16/android-touchpad-missing-position.log)에 있다.

이 입력 셸은 ADB 연결 실패가 아니다. Android 공식 [InputDevice API](https://developer.android.com/reference/android/view/InputDevice#SOURCE_TOUCHPAD)는 `SOURCE_TOUCHPAD=0x00100008`과 `SOURCE_CLASS_POSITION`을 연결하고, 실제 touchpad 장치는 `getSources()`에서 mouse와 touchpad를 함께 보고하며 `MotionEvent.getSource()`에는 보통 mouse source를 돌려준다고 명시한다. Agent Browser로 확인한 [AOSP `InputShellCommand.runScroll`](https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/services/core/java/com/android/server/input/InputShellCommand.java)은 `SOURCE_CLASS_POINTER`인 경우에만 positional X/Y를 event axes에 넣는다. 따라서 `input touchpad scroll x y ...`는 이 클래스에 좌표를 전달하지 않는다. 앱에서 이전 cursor 위치를 재사용하거나 `getSources()` 합집합으로 event를 재분류하지 않는다.

## 구현 전 모델과 판정

- **독립 기준:** Android `InputShellCommand`와 `InputDevice`의 source/class 설명, API 37 AVD `input help`, 앱 native callback 로그, 고정 C13.3 Chrome 155 fixture와 기존 CSS wheel offset oracle를 함께 사용한다.
- **재현 입력:** ADB `input touchpad scroll`은 screen `(0,0)`에서 좌표 없는 `ACTION_SCROLL`을 만들고, 기본 app surface `(15,991)`에서는 `point-outside-surface`로 거부된다. 이 native event를 positive로 쓰기 위해 opt-in fixture mode에서만 SurfaceView를 이동해 screen `(0,0)`이 기존 Chrome target `(132,112) CSS px`에 대응하도록 한다. 이벤트 좌표나 이전 mouse cursor 위치는 조작·재사용하지 않는다. 이동 없는 기본 surface는 동일 명령의 negative control로 보존한다.
- **uinput 비교:** 가상 two-finger uinput contact는 Android `Touchpad Input Mapper`와 libgestures의 `scroll` 인식까지 만들었지만 app에는 `SOURCE_MOUSE` `ACTION_DOWN/MOVE/UP`, tool `FINGER`로 들어왔다. native `ACTION_SCROLL`이 아니므로 양성 근거에 세지 않고 다른 입력 경로의 negative control로 기록한다.
- **event identity:** 허용 판정은 `MotionEvent.getSource()`, action, pointer tool, pointer count, native point를 사용한다. `InputDevice.getSources()`는 보조 진단만 쓴다. event shape가 `SOURCE_MOUSE + TOOL_TYPE_FINGER` 또는 point가 확인된 명시 `SOURCE_TOUCHPAD`가 아니면 거부한다.
- **양성 pass:** 실제 InputReader→`ACTION_SCROLL` callback의 raw point `(0,0)`이 이동된 SurfaceView 기준 target CSS `(132,112)`에서 축마다 0.5 CSS px 이내이고, `SOURCE_TOUCHPAD` 또는 mouse+finger source 판별, 원본 H/V 축, factor·surface scale 정규화가 기대값(0.01 CSS px 허용 오차)과 맞는다. 지원 source는 queue sequence 1, target 7, draw revision 2와 고정 Chrome fixture owner offset을 만든다. SurfaceView origin, AVD 화면, raw event와 owner를 함께 기록한다.
- **음성 pass:** 이동 없는 기본 surface에서 동일한 좌표 없는 touchpad event는 queue/process 없이 거부되고 revision 1과 시작 offsets를 유지한다. uinput의 `SOURCE_MOUSE` touch gesture는 native wheel 양성 판정에 사용하지 않는다. 캡처 비교에서 거부 case의 화면 변경은 0 pixel이다.
- **보류 기준:** ADB `input touchpad scroll`이 `ACTION_SCROLL`을 만들지 않거나 opt-in origin target mode가 raw `(0,0)`을 target CSS point로 변환하지 못하면 harness/platform 경계로 기록하고 targetable touchpad를 미검증으로 둔다. uinput의 mouse/touch stream, direct FFI injection, 이전 cursor point 재사용, synthetic app callback은 positive evidence로 대체하지 않는다.
- **재현 자료:** device descriptor, `dumpsys input`/`getevent`, event callback raw fields, queue/process/draw logs, before/after screenshots, AVD metadata, APK/helper/source hash와 cleanup 결과를 한 evidence directory에 둔다.

## 구현 경계와 복구

- uinput helper는 실행 전용 AVD serial emulator-5554의 QEMU/API/ABI를 다시 확인하고 고유 device name을 쓴다. 일반 경로와 오류 경로는 UI_DEV_DESTROY 후 fd를 닫고, SIGINT/SIGTERM은 release 후 정리한다. 프로세스 종료 뒤 dumpsys input에서 장치가 사라졌는지 확인한다.
- origin target translation은 별도 opt-in intent에서만 적용하고 surface CSS viewport·scene·scale을 바꾸지 않는다. 기본 fixture에는 적용하지 않으며, event point가 없거나 실제 bounds 밖이면 scroll state, queue, revision, draw를 바꾸지 않는다.
- event callback 이후 기존 queue가 닫혔거나 surface generation이 달라진 경우 stale input을 reject한다. helper 종료 뒤 가상 입력 device가 남지 않았는지 다시 조사한다.
- Android Simulator 증거는 물리 touchpad나 제품 입력 지원 완료로 해석하지 않는다. `spec/STATUS.md`의 C13.3 체크 상태는 이 계획/AVD 결과만으로 변경하지 않는다.


## 실행 결과 · 2026-10-16

Android 17/API 37 ARM64 QEMU AVD emulator-5554에서 fixture-test APK를 실행했다. opt-in intent로 surface origin을 조정한 양성 case는 실제 ACTION_SCROLL callback의 source 0x100008, pointer tool finger, raw point (0,0), H axis 1.0, factor 168 px, scale 2.625를 기록했다. 계산 CSS point (132.19048,112.0)에서 sequence 1이 admit되어 target 7과 draw revision 2가 나왔고 outer owner X가 64 CSS px 이동했다.

origin 조정이 없는 기본 surface 음성 대조는 point (0,0)을 (15,991) origin 기준 표면 밖으로 판단해 queue 전에 거부했다. 전후 화면 SHA-256은 모두 f186a078d3d15f908be491730286a46452468ae6eed3419d7647d5723bb77c91였다. 두 손가락 uinput negative control은 앱에 SOURCE_MOUSE ACTION_DOWN/MOVE/UP으로 도달했으며 native ACTION_SCROLL 양성 근거로 세지 않았다. [실행·화면·로그·build 근거](../spec/internal/evidence/c13-3-android-avd-touchpad-point-2026-10-16/README.md).

이 결과는 AVD 입력 shell과 fixture 전용 표면 이동에 한정된다. 실기기·외장 trackpad, iOS callback, hardware GPU/compositor 출력, 공개 DOM event와 C13.3 완료는 입증하지 않는다.
