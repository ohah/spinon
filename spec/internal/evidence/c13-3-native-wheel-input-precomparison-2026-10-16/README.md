# C13.3 native wheel·trackpad 구현 전 비교 · 2026-10-16

## 결론과 경계

기존 C13.3 Chrome 155 fixture는 normalized CSS px wheel 결과와 owner offsets를 고정한다. 이 문서는 기대값과 시험 입력을 구현 전에 고정했다. Android API 37 AVD native mouse adapter의 방향·queue·draw 행렬은 별도 [실행 근거](../c13-3-native-wheel-input-2026-10-16/README.md)에 기록했다. Android ADB touchpad source는 좌표가 없어 거부됐고 target 가능한 trackpad 입력은 미검증이다. iOS 부호는 실제 UIKit callback이 관찰될 때까지 미확정이다.

![기존 Chrome 155 C13.3 scroll fixture](../c13-3-scroll-gesture-precomparison-2026-10-15/chrome155-c13-3-scroll-gesture-400x240.png)

## 기준 자료

- Chrome for Testing `155.0.8059.40`의 고정 C13.3 `400×240` CSS fixture와 [기존 Chrome 관찰](../c13-3-scroll-gesture-precomparison-2026-10-15/README.md)은 scroll offset/clipping oracle다. 기존 wheel fixture `delta=(40,40)` 및 `delta=(0,40)`로 inner/outer/root offset과 border 바깥 pixel 0을 비교한다.
- [Android MotionEvent API](https://developer.android.com/reference/android/view/MotionEvent)는 mouse `ACTION_SCROLL`의 `AXIS_VSCROLL`/`AXIS_HSCROLL`과 generic-motion 경로를 정의한다. positive V는 viewport up/content down, positive H는 viewport right/content left다. [ViewConfiguration](https://developer.android.com/reference/android/view/ViewConfiguration) factors를 axis에 곱해 native scroll pixels를 얻는다.
- [Android View touchpad notes](https://developer.android.com/reference/android/view/View#POINTER_CAPTURE_MODE_RELATIVE)는 touchpad two-finger scroll이 mouse-like `ACTION_SCROLL`/H/V axes로 전달될 수 있다고 설명한다. API 34 classified-move gesture-distance axes는 이 계획에서 지원하지 않는다.
- [Apple UIPanGestureRecognizer scroll types](https://developer.apple.com/documentation/uikit/uipangesturerecognizer/allowedscrolltypesmask)는 scroll wheel과 two-finger trackpad scroll을 인식한다. [UIScrollTypeMask](https://developer.apple.com/documentation/uikit/uiscrolltypemask)는 discrete=mouse, continuous=trackpad로 구분한다.
- [Apple UIGestureRecognizer modifierFlags](https://developer.apple.com/documentation/uikit/uigesturerecognizer/modifierflags)는 gesture modifier 상태를 제공한다. Control-wheel은 zoom intent로 거부한다.
- [Apple translation(in:)](https://developer.apple.com/documentation/uikit/uipangesturerecognizer/translation%28in%3A%29)는 gesture 시작부터의 누적 translation이다. 문서는 scroll recognizer translation sign/unit을 CSS WheelEvent와 동일하다고 정하지 않으므로 iPadOS Simulator의 ±X/±Y가 구현 전 gate다.
- [W3C UI Events WheelEvent](https://www.w3.org/TR/uievents/#wheelevent)는 default scroll 방향에서 right-handed axes를 사용한다. positive X는 오른쪽, positive Y는 아래쪽 document 방향이다.

## 고정 정규화 oracle

| 플랫폼 sample | native 단위 | 예상 CSS px 변환 | 독립 확인 |
| --- | --- | --- | --- |
| Android mouse / mouse-equivalent touchpad | `ACTION_SCROLL`, `HSCROLL`/`VSCROLL`, ViewConfiguration factor 적용 physical px | `deltaX = HSCROLL × horizontalFactorPx / surfacePxPerCssPx`; `deltaY = -(VSCROLL × verticalFactorPx) / surfacePxPerCssPx` | Android mouse AVD에서 양/음 H/V와 네 대각, factor·scale 식, queue/draw offsets 통과; ADB touchpad는 point 부재로 거부 |
| Android classified touchpad move | `CLASSIFICATION_TWO_FINGER_SWIPE`, `AXIS_GESTURE_SCROLL_*_DISTANCE` | 이 slice 미지원, queue로 보내지 않음 | no-enqueue control |
| iOS discrete mouse | `allowedScrollTypesMask = .discrete`; UIKit point translation | `deltaCss = - (translationNow - translationPrevious) / pointsPerCssPx`; sign은 Simulator preflight 통과 전 구현 금지 | 양/음 H/V, 반복 translation no-op |
| iOS continuous trackpad | `allowedScrollTypesMask = .continuous`; UIKit point translation | 같은 cumulative-to-incremental 변환, 독립 baseline/source | source identity, 방향 control |

wheel sample의 `coordinateScale`은 native source units per CSS px다: Android surface physical px/CSS px, UIKit points/CSS px. `raw_delta`는 CSS conversion 전 native unit value다. Android factor는 원본 axis와 별도 기록한다. iOS 수식은 제안값이며, 양/음 recognizer translation을 캡처해 확정한 뒤 구현한다.

## 입력과 사전 판정

- Android AVD API 37 ARM64의 Android input shell에서 명시적인 `mouse` 또는 `touchpad` source와 `scroll` command를 쓴다. 앱 callback log는 framework-dispatched `ACTION_SCROLL`을 증명한다. ADB input (callback의 device id는 `-1`)은 physical hardware proof가 아니다. mouse 방향/queue/draw 행렬은 [Android 실행 근거](../c13-3-native-wheel-input-2026-10-16/README.md)에 있다.
- iPadOS 26.2 Simulator에서 앱 opt-in route로 Simulator UI scroll을 보낸다. recognizer callback과 `.discrete`/`.continuous` source 분류가 관찰돼야 한다. direct fixture FFI injection은 native input proof를 대체하지 않는다.
- 각 source에서 ±H/±V, diagonal, repeated-sample controls를 inner fixture point에 넣는다. native coordinate/source/raw delta/scale/CSS delta와 final owner offsets를 함께 기록한다. final state는 관찰 CSS delta를 existing normalized wheel oracle에 넣어 계산한다.
- Simulator 입력이 특정 source를 전달하지 않으면 capability gap을 unverified로 남긴다. synthetic FFI route로 바꿔 native-input pass라고 표시하지 않는다.
- UI Events는 scroll 방향을 규정하지만 개별 기기/configuration의 물리 delta 크기는 다를 수 있다. oracle는 OS 간 동일한 물리 delta가 아니라 관찰 native sample의 정규화 정확도를 요구한다.

## 구현 전에 확인할 미지수

- iPadOS 26.2 Simulator에서 discrete/continuous callback 모두 실제로 오는지 확인한다. 한 source만 전달되면 다른 source는 미검증이다.
- 양/음 iOS translation과 UIKit point scale을 callback log로 캡처한다. 수식이 반대면 코드를 쓰기 전에 이 oracle과 계획을 수정한다.
- ADB touchpad `ACTION_SCROLL` callback의 source와 axis는 도착했지만 pointer position이 `(0,0)`이라 reject됐다. 다른 target 가능한 touchpad source는 별도 입력장치 없이는 미검증으로 남긴다.
- 이 precomparison은 product API, physical-device input, public DOM events, momentum/fling, compositor/scanout 또는 C13.3 완료를 증명하지 않는다.
