# Android 회수 검증 화면 노출 확인

**실행일:** 2026-10-05 · **기기:** Android 16 / API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터 · **결과:** 검증용 버튼 노출 및 자동 fixture 통과

일반 앱 빌드는 검증용 `LowMemoryNotification()` hook과 회수 버튼을 포함하지 않는다. 그래서 기본 실행 화면에서는 버튼이 보이지 않는다. Android에서 이를 확인하려면 fixture를 켜서 빌드하고 전용 Intent로 실행한다.

```sh
SPINON_ENABLE_S03_DOM_GC_FIXTURE=1 mise exec -- bun run build:android
adb install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_dom_gc true
```

최신 검증 빌드 APK의 SHA-256은 `732f89ee7787aad087a8e0df0449f2ffc23a8c6a5be46cd2da7f4b51cd13f712`다. 실행 화면에서 `V8 약한 wrapper 회수 검증` 버튼이 보이며 자동 fixture 상태는 `V8 반복 수명 회수 검증 통과`로 표시됐다.

화면 캡처 제목 주위의 녹색 테두리는 Android 접근성 포커스 오버레이이며 앱 레이아웃 장식이 아니다.

원본 결과는 `dom_gc=PASS`이며 attached tree·live wrapper 보존, wrapper 재생성, orphan weak reference 정리, callback closure root 보존·해제, 16,385개 wrapper의 보존·회수, 반복 기준선 복귀와 collector 계수 검사가 통과했다. [원본 Android 로그](s03-android-dom-gc-visible-2026-10-05.log)와 [검증 화면](s03-android-dom-gc-visible-2026-10-05.png)을 참고한다.

이 실행은 Android 에뮬레이터 한 대의 개발 전용 fixture다. OS 메모리 압박 통지나 자동 회수 정책은 시험하지 않았다. 사용된 기존 Android V8 artifact는 `v8_jitless=false`, `arm_control_flow_integrity="none"`이며 실기기·배포 설정의 검증으로 확대하지 않는다. scan 표본 시간은 성능 비교값이 아니다.
