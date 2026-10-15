# C13.3 native wheel 입력 부호 사전 확인 · 2026-10-16

이 기록은 native adapter를 구현하기 전 OS callback의 action, source, axis sign과 scale을 확인한 범위다. queue 연결, scroll offset 결과, 화면 pixel 결과를 증명하지 않는다.

## Android API 37 ARM64 AVD

- 대상은 `emulator-5554`, `sdk_gphone16k_arm64`, API 37 / Android 17이다. 앱은 `SPINON_ENABLE_C04_RUNTIME_GPU=1`과 `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1`로 빌드했다.
- 고정 화면은 `400×240` CSS px, Surface texture는 `1050×630` physical px, surface scale은 `2.625 px/CSS px`였다. `ViewConfiguration`의 horizontal/vertical factor는 각각 `168 physical px`였다.
- ADB mouse `ACTION_SCROLL`은 `source_bits=0x2002`, 좌표 `(360,1300)`, H/V 각각 `+1` 및 `-1`을 전달했다. mouse와 mouse-equivalent touchpad 모두 Android 문서의 generic scroll 경로와 H/V 축으로 도착했다.
- Android 공식 축 정의와 결합하면 axis `+1`은 `64 CSS px`의 horizontal/vertical magnitude를 만든다. positive H는 viewport 오른쪽 방향이어서 CSS `deltaX=+64`; positive V는 viewport 위쪽 방향이어서 CSS `deltaY=-64`다. 두 음수 입력은 각각 반대 부호다. 이는 문서식과 callback 표본으로 계산한 기대값이며 queue/화면 실행 결과는 아니다.
- ADB touchpad source도 `ACTION_SCROLL`과 H/V 값을 전달했지만 좌표가 `(0,0)`이고 device id가 `-1`이었다. 이 입력은 고정 surface의 유효 hit point가 없으므로 target을 추정하거나 이전 pointer 위치를 재사용하면 안 된다. ADB touchpad 표본은 source 수집만 확인한다.

원본 callback은 [android-avd-preflight.log](android-avd-preflight.log), simulator와 APK 정보는 [android-avd-environment.txt](android-avd-environment.txt)에 있다.

## iPadOS 26.2 Simulator

- iPad Pro 13-inch (M5), UDID `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`에서 `.discrete`와 `.continuous` recognizer를 포함한 opt-in 앱이 준비 상태까지 실행됐다.
- 화면은 simulator screenshot과 `SPINON_C133_READY` 로그에서 준비를 확인했다. 그러나 호스트 Window Server 위쪽에 `loginwindow` 화면이 있고 Simulator 창은 그 뒤에 있었다. Simulator PID에 올린 scroll CGEvent 뒤에도 앱 recognizer callback은 없었다.
- 따라서 UIKit translation sign과 source 분류는 **미관찰**이다. 현재 iOS callback 로그에는 준비 marker만 있고 native wheel callback은 없다. [ios-simulator-preflight.log](ios-simulator-preflight.log)
- host native input을 simulator에 전달하지 못한 harness 상태다. direct FFI enqueue로 대체하지 않았고 iOS adapter는 아직 구현하지 않았다.

## 판정

Android 부호와 단위 정규화는 adapter 구현 gate를 통과했다. iOS는 실제 `.discrete`/`.continuous` callback이 관찰될 때까지 adapter 구현과 부호 판정을 보류한다. Android AVD와 iPadOS Simulator 모두 physical input device 증거가 아니다.
