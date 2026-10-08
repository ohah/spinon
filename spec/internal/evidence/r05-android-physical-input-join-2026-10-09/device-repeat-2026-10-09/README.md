# Android 실기기 R05 재검증 · 2026-10-09

- **실행 시각:** 2026-10-09 02:40–02:44 KST
- **기기:** Samsung SM-S731N · Android 16 · API 36 · SDK_INT_FULL 36.1 · ARM64 · 1080×2340 · density 450
- **화면:** mode 2 · 60 Hz · 밝기 248 · 화면 꺼짐 30,000 ms · 실행 중 Awake
- **renderer:** R08 WGPU/Vulkan · Samsung Xclipse 940
- **APK:** 기존 설치본을 추출해 SHA-256 확인. PR #79 debug artifact와 같은 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`; 재설치·재빌드하지 않음.
- **기준 저장소 commit:** `2b0b0218d8a0e0ce639a96244774fce9d456b424`. 설치 APK는 이 문서 commit에서 새로 빌드한 산출물이 아니다.

## 입력 출처 대조

새 프로세스에서 R08 GPU 도형을 열고 `adb shell input tap 540 1170`을 한 번 보냈다. 앱은 input sequence 1, submit, 실제 `TransactionStats` callback, usable async fence signal을 기록했고, 동시에 수집한 `sec_touchscreen` raw event는 0건이었다. 따라서 이는 실기기에서 실행된 **synthetic negative control**이다. 앱 로그의 `input_source=unknown`은 출처 증거로 사용하지 않았다.

## 직접 입력 수집 창

새 R05 probe 프로세스(PID 3893)를 전면에 두고 `/dev/input/event6`의 `sec_touchscreen` 원본과 PID별 앱 로그를 약 60초 동시에 수집했다. 장치는 실행 직전 `INPUT_PROP_DIRECT`로 확인했다. 앱은 R08 Vulkan surface를 준비했으나 수집 구간에 raw touchscreen event와 앱 `SPINON_R05_INPUT`이 모두 없었다. 직접 손가락 입력 positive sample은 0건이다. 수집 종료 시 raw 파일은 0 bytes였고 앱은 정상 전면 상태였다.

## 실기기 synthetic 반복

별도의 새 프로세스(PID 4422)에서 중앙 target에 ADB synthetic tap 10회를 약 1초 간격으로 보냈다. 고정 검증 스크립트로 sequence, revision, generation, request ID를 정확 대조했다.

| 신호 | 결과 |
|---|---:|
| 앱 `ACTION_UP` / input sequence | 10/10 · sequence 1–10 |
| WGPU submit | 10/10 · 같은 sequence/revision · generation 1 |
| 실제 `TransactionStats` callback | 10/10 · 같은 request/input/revision/generation |
| usable async fence signal | 10/10 · `signaled` |
| 종료 pending / active / queue | 0 / 0 / 0 |
| 동시 raw `sec_touchscreen` | 0 events · 0 bytes |
| 화면 활성화 수 | 10회로 캡처 확인 |
| `target_vsync_id` | 전부 -1 |

이 실행은 입력 경로를 포함한 실기기 synthetic 회귀 확인이다. 손가락 입력 latency, 제품 성능, p95, VSync 또는 scanout 검증은 아니다. worker `await` 시간이나 fence timestamp를 입력→표시 시간으로 해석하지 않았다.

## 실기기 재연결 반복 2

같은 APK hash로 R08 WGPU/Vulkan을 다시 열었다. 약 60초 direct `sec_touchscreen` 수집에서는 contact가 0건이었다. 별도 ADB synthetic tap 10회는 app input, submit, 실제 `TransactionStats` callback, usable async fence signal까지 10/10 exact join됐고 각 완료 queue가 0/0/0이었다. 화면에서 activation count 10을 확인했다. 이 API 36 실행도 `target_vsync_id=-1`이라 latency·VSync·scanout을 측정하지 않았다. [재검증 2 원본, 화면과 독립 20관점 검토](physical-retest-2/README.md).

## 복구 및 한계

실행 전후 Android 설정을 변경하지 않았다. 마지막에는 Spinon을 종료하고 원래 전면 앱인 Chrome을 복귀시켰다. 화면은 Awake, 밝기 248, timeout 30,000 ms, active display mode 2 / 60 Hz였다. APK는 설치하거나 덮어쓰지 않았다. ADB serial은 evidence에 기록하지 않았다.

R05.3의 직접 touch positive sample은 여전히 미완료다. 다음 direct sample은 수집 중 사용자가 실제 화면을 손가락으로 눌러야 한다. Android API 36에서 `target_vsync_id=-1`이므로 OS transaction fence 신호는 관측했지만 VSync·화면 발광 시각은 알 수 없다.

## 자료

- [반복 synthetic 앱 로그](synthetic-10-app-logcat.txt)
- [반복 raw touchscreen 수집](synthetic-10-raw-touchscreen.txt) (0 bytes)
- [10회 뒤 화면](synthetic-10-screen.png)
- [단일 synthetic control 로그](control-app-logcat.txt) · [raw](control-raw-touchscreen.txt) (0 bytes) · [화면](control-after-one-synthetic-tap.png)
- [물리 입력 수집 창 앱 로그](physical-window-app-logcat.txt) · [raw](physical-window-raw-touchscreen.txt) (0 bytes) · [시작 화면](physical-window-launch.png)
- [설치 APK SHA-256](installed-apk.sha256) · [실행 환경](environment.txt) · [정확 join 요약](capture-summary.txt)
- [독립 실패 경로 검토](runtime-review.md) · [체크섬](SHA256SUMS)
