# Android 실기기 collector 무접촉 종료 smoke

## 판정

2026-10-09 13:03 KST, Android 16 / API 36 실기기에서 새 R05 process PID 11489로 bounded collector를 실제 ADB 경로에서 실행했다. GO 시점 process age는 129.15초였고 지정 APK와 설치 `base.apk` SHA-256은 기존 고정값과 같았다. R05 제목과 파란 GPU 사각형이 화면에 표시되는 것을 확인한 뒤, 입력하지 않은 2초 무접촉 smoke를 수행했다. collector는 `prestart_no_contact`와 `captureStatus=no_contact_timeout`로 닫혔다.

두 host ADB client는 모두 SIGINT 뒤 종료했고 SIGTERM/SIGKILL로 상승하지 않았다. 종료 뒤 device process 목록에서 `getevent` 잔존은 0개였고 host 쪽에도 이 capture의 `getevent`/PID logcat client가 남지 않았다. 시작·종료 screenshot의 SHA-256이 같아 UI 상태는 바뀌지 않았다. 이것은 실제 기기의 무접촉 자동 종료와 정리 경로 확인이지 direct touch 양성 표본이나 10 × 30 block이 아니다. `raw-touch.log`는 0바이트이며 `captureWindowValid=false`이므로 phase-2 점수에 넣지 않는다.

기기 serial과 ADB devices 원문은 evidence에서 제외했다. manifest에서는 serial·PID를 가리고 collector process identity도 제거했다. 원본 local run은 `/tmp/spinon-r05-android-capture-smoke-20261009-1303`에 있다.

## 확인된 값

| 항목 | 결과 |
|---|---|
| 기기·화면 | Samsung SM-S731N · Android 16/API 36 · portrait · 60Hz · WGPU/Vulkan/Xclipse 940 |
| 앱 process | PID 11489, GO 시점 age 129.15초, `SPINON_R05_FENCE_WAIT_START` 기록 |
| APK | 지정/설치 SHA-256 모두 `b87a5cdd1d3589a5ffea66b784d9bf40f64843b69ad4aeb876086412f4db5669` |
| 수집 제한 | 첫 접촉 timeout 2초, 목표 1개, drain 0초 (smoke 전용 override) |
| raw 입력 | contact group 0, active 0, pending release 0, protocol error 0 |
| 종료 | GO 13:03:49.003 KST → stop 13:03:51.693 KST · `prestart_no_contact` |
| host collector | getevent/logcat 모두 SIGINT 종료 · 추가 escalation 없음 |
| device collector | 종료 뒤 getevent 0개 |
| 최종 상태 | `no_contact_timeout` · `captureError=null` · `captureWindowValid=false` · `analysisStatus=not_analyzed_join_required` |

## 보존 파일

- `capture-manifest.redacted.json`: serial/PID를 가린 supervisor 결과
- `raw-touch.log`: 0바이트 원시 입력 (무접촉 대조)
- `app.log`: R05 startup부터 종료까지의 process log; accepted touch 없음
- `screen-before.png`, `screen-after.png`: 동일 화면 screenshot
- `GO.txt`, `device-uptime-go.txt`: run 경계 시각
- `input-devices.txt`: `sec_touchscreen` direct input capability 확인
- `installed-apk-sha256.txt`: 설치 APK digest
- `getevent-stderr.txt`, `logcat-stderr.txt`: collector stderr
- `SHA256SUMS`: 보존 파일 checksum

이 smoke로 raw touch→`ACTION_UP`→submit→callback→fence join을 검증하지 않았다. R05.3 phase-2의 유효 직접 입력 표본은 계속 0/300이다.
