# Android 실기기 R05 비동기 fence 재검증

- **실행:** 2026-10-09 01:27–01:29 KST
- **기기:** Samsung SM-S731N · Android 16 · API 36 · SDK_INT_FULL 36.1 · 1080×2340 · density 450
- **화면:** active mode 60 Hz (기기는 120 Hz도 지원), 밝기 248, 화면 꺼짐 30,000 ms
- **renderer:** R08 WGPU/Vulkan `SurfaceView`
- **앱:** 새 process PID 27283, debug-only `spinon_r05_async_fence_wait=true`
- **APK:** 공식 PR #79 debug 산출물과 설치 APK SHA-256 일치: `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`

## 실행

기기에서 `sec_touchscreen` direct-input 장치를 다시 찾아 raw `getevent` 수집을 열고, 앱을 전경에 둔 뒤 중앙 GPU 도형 (540, 1170)을 450 ms 간격으로 `adb shell input tap` 11회 눌렀다. 자동 입력 전후 앱 PID와 foreground를 확인했다. 이 입력은 **실기기에서 실행한 synthetic ADB 주입**이다. `input_source=unknown`인 앱 기록만으로 입력 출처를 분류하지 않았으며, raw `sec_touchscreen` 수집은 0줄이었다. 따라서 손가락 입력 positive sample은 0건이다.

## 결과

| 신호 | 수량 | 판정 |
|---|---:|---|
| app `ACTION_UP` / input sequence | 11/11 | sequence 1–11, `input_source=unknown` |
| R08 frame submit | 11/11 | 같은 sequence/revision 1–11, generation 1, draw accepted |
| 실제 `TransactionStats` callback | 11/11 | request와 sequence가 연결됨 |
| 복제 fence의 bounded async signal | 11/11 | 모두 `signaled`, usable positive timestamp |
| 종료 queue 상태 | 0 pending / 0 active / 0 queued | completed 11 |
| raw direct-touchscreen event | 0 | injection negative control 결과와 일치 |
| ANR / fatal exception 표식 | 0 | PID-filtered app log 기준 |

`target_vsync_id=-1`이었다. worker의 `await()` 실행 구간은 0.105–12.660 ms였으나 입력→표시 지연으로 계산하지 않았다. fence signal은 광학 scanout이나 화면 발광 측정이 아니다. 이번 한 block은 p95·성능 순위·R05.3 완료 근거가 아니다.

캡처에서 GPU 도형의 색상과 활성화 수가 11회로 바뀐 것을 확인했다: [화면 캡처](after-11-synthetic-taps.png). 시작 화면은 [launch.png](launch.png)다.

## 복구와 한계

실행 뒤 Spinon을 force-stop하고 Chrome을 foreground로 복귀시켰다. 밝기 248, 화면 꺼짐 30,000 ms, `stay_on_while_plugged_in=3` 및 active display mode 2(60 Hz)가 유지됐다. ADB serial은 이 자료에 저장하지 않았다.

실제 손가락 접촉과 MotionEvent의 물리 출처를 exact join한 표본은 아직 없다. 사용자 직접 탭 없이 ADB가 물리 동작을 대체할 수 없으므로 R05.3은 미완료다. API 36 (SDK_INT_FULL 36.1) VSync ID unavailable, release 실행, 광학 측정도 미검증이다.

별도 release 빌드 preflight는 Android SDK 경로 설정 뒤에도 `tools/build-android.sh`가 고정 V8 source tree 부재로 중단했다. 큰 V8 checkout을 자동 생성하거나 기존 작업을 덮어쓰지 않았다. [Gradle 로그](../../r05-android-release-isolation-2026-10-09/gradle-release-sdk-configured.log).

## 자료

- [PID-filtered app log](app-logcat.txt)
- [raw touchscreen capture](raw-touchscreen.txt) (0 bytes)
- [실행 환경](environment.txt) · [serial을 제거한 ADB 연결 확인](connection.txt)
- [시작 화면](launch.png) · [11회 뒤 화면](after-11-synthetic-taps.png)
- [기기 복귀 상태](postflight-focus.txt), [display](postflight-display.txt), [brightness](postflight-brightness.txt), [screen timeout](postflight-timeout.txt), [stay-awake](postflight-stay-awake.txt)
- [설치 APK SHA-256](installed-apk.sha256) · [checksums](SHA256SUMS)
- [실행 증거 20관점 검토](runtime-review.md)
