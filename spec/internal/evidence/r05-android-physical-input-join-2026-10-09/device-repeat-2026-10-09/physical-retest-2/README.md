# Android 실기기 재검증 2 · 2026-10-09

## 결과

- 기기: Samsung SM-S731N, Android 16 / API 36 / ARM64, 1080×2340, density 450, 60 Hz mode 2.
- 화면: R08 GPU 도형이 전면에 있었고 앱 PID는 `5340`이었다. renderer는 WGPU/Vulkan, Samsung Xclipse 940이다.
- APK: 설치본을 추출해 확인한 SHA-256은 [`installed-apk.sha256`](installed-apk.sha256)이다. 이전 PR #79 debug artifact와 동일하다. 재빌드·재설치하지 않았으며 이 저장소 commit 산출물로 오인하지 않는다.
- 직접 손가락 입력 수집: `sec_touchscreen` / `INPUT_PROP_DIRECT` 입력 노드에서 약 60초 동안 `getevent -lt`를 실행했지만 raw event는 0건이었다. 이 구간은 직접 입력 positive sample이 아니다.
- synthetic 대조: 별도 구간에 `adb shell input tap 540 1170` 10회를 보냈다. 앱 입력 10/10, input sequence 1–10, revision 1–10, generation 1에서 실제 `TransactionStats` callback과 usable async fence signal이 각각 10/10 정확히 연결됐다. 각 완료 로그는 `pending=0 active=0 queue_depth=0`이다.
- 시각 결과: [시작 화면](launch.png)과 [synthetic 10회 뒤 화면](after-10-synthetic.png)에서 R08 GPU 도형과 활성화 횟수 10을 확인했다.
- VSync: `target_vsync_id=-1`이다. 입력→표시 지연, VSync 시각, panel scanout, 성능 순위는 산출하지 않았다.
- 복구: Chrome을 전면으로 돌렸고 화면 켜짐·30초 timeout·밝기 248·mode 2/60 Hz가 유지됐다. Android 설정은 변경하지 않았다.

## 판정 경계

이 실행은 Android 실기기에서 앱·WGPU surface·SurfaceControl callback/fence 경로를 다시 확인한 것이다. 10회 입력은 ADB가 주입했으므로 실제 손가락 입력 10회로 세지 않는다. 직접 입력 수집 0건도 입력 기능 실패로 해석하지 않는다. R05.3은 미완료로 유지한다.

## 원본

- [전체 앱 로그](app-logcat.txt)
- [직접 입력 수집 원본](raw-touchscreen.txt) · 0 bytes
- [장치 capability](input-device.txt)
- [실행 환경](environment.txt)
- [정확 join 요약](capture-summary.txt)
- [APK digest](installed-apk.sha256)
- [실행 후 독립 검토 20관점](runtime-review.md)
- [파일 체크섬](SHA256SUMS)
