# ADB 입력 진단

- 대상 serial `emulator-5554`는 `ro.boot.qemu.avd_name=spinon_api37_1_compat16k`, API 37, ABI `arm64-v8a`였다. `localhost:5555` 응답은 같은 boot ID를 반환해 동일 AVD의 중복 ADB 주소로 판정했다. 테스트는 `emulator-5554`만 사용했다.
- `adb shell input help`의 `motionevent` 문법은 `DOWN|UP|MOVE|CANCEL`만 제공한다. 이 명령으로는 `ACTION_POINTER_DOWN`을 만들 수 없다.
- 비-root `sendevent /dev/input/event11 ...`는 `Permission denied`였다. kernel audit의 명시적 SELinux 기록은 [android-avc-denial.log](android-avc-denial.log)에 보존했다. AVD에서만 `adb root`를 켜자 shell UID가 0이 되어 evdev 쓰기가 허용됐다.
- 권한이 풀린 뒤에도 event11은 Android `Touch Input Mapper (mode - DISABLED)`이고 virtual display에 연결되어 있었다. 따라서 event11 입력이 앱에 도착하지 않은 것은 ADB transport 문제가 아니라 잘못된 device node 선택이었다.
- `dumpsys input`에서 event1은 같은 멀티터치 장치군 중 `mode - DIRECT`, internal `displayId=0`, 1080×2400 viewport로 매핑됐다. 그 장치에 두 slot을 넣자 InputReader가 2 contacts, 두 번째 slot 해제 뒤 1 contact를 기록했고 Activity는 MotionEvent action 5를 받았다.
- 수정은 입력 경로 선택과 권한 사용 범위에만 적용했다. Android 앱의 C13.3 gesture adapter 코드는 변경하지 않았다.

자세한 캡처는 [장치 매핑](android-input-device-map.txt), [InputReader 2 contacts](android-input-two-contacts.txt), [InputReader 1 remaining contact](android-input-one-contact-after-second-up.txt), [raw evdev trace](android-evdev.log.gz), [Android MotionEvent trace](android-runtime-key-trace.log)다.
