# C11.4 Android 실기기 시도 — 화면 결과 미검증

- 대상: Samsung SM-S731N, Android 16 / API 36, arm64-v8a; renderer 확인값은 Samsung Xclipse 940 / Vulkan 1.3.279다.
- 실행 APK SHA-256과 설치된 base APK SHA-256은 모두 `cf918196c17e6763bf60ee6d316f6116fd76c8e500d3123647d1810b1085a27a`였다.
- 첫 장면에서 `layout=ready`와 Columns→Rows trace 4개 chunk를 받았다. 그러나 기기 전원이 `Dozing`, interactive=false였고 keyguard가 표시되어 SurfaceView가 숨겨졌다. runtime log에 `SPINON_C0410_DRAW ... presented`가 없었고 PixelCopy sample도 도달하지 않았다.
- capture 도구는 `화면 표시 결과가 없습니다`로 장면을 거부했다. 따라서 이 시도는 실기기 draw/present나 pixel 결과의 통과 증거가 아니다. Android runtime pass는 API 37 ARM64 AVD 9/9로 한정한다.

저장한 실행 자료:

- [APK 설치 log](../c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/install.log)
- [설치 APK hash 대조](../c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/installed-apk-verification.json)
- [Activity launch log](../c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/launch-fixed-auto-area-columns-fill-collision.log)
- [화면 상태](../c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/screen-state.txt)
- [capture 검증 오류](../c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/capture-error.txt)
