# R05.3 직접 touch block 02 · 2026-10-09

첫 block 02 시도는 13:48 KST 수집 창에 raw touch가 없어 `prestart_no_contact`로 무효 종료됐다. 창 종료 후 도착한 사용자의 완료 응답은 해당 시도와 결합하지 않았다. 별도의 새 앱 process와 새 GO 창으로 다시 수집한 결과만 아래에 기록한다.

## 결과

| 단계 | 사전 고정 첫 30개 결과 |
|---|---:|
| raw direct touchscreen contact | 30/30 |
| 정확한 raw release → 앱 `ACTION_UP` | 30/30 |
| WGPU submit | 30/30 |
| 현재 surface generation callback | 30/30 |
| 같은 request의 usable async fence | 30/30 |
| 유효 clock-bracket 후보 | 30/30 |
| 제외 입력·겹친 포인터·raw protocol 오류 | 0 |

## 실행과 판정

- 기기: Samsung SM-S731N · Android 16 / API 36 · WGPU/Vulkan · portrait · 60Hz. APK SHA-256은 계획의 고정값과 일치했다. thermal status는 0, 배터리 100% 및 USB 전원 연결을 확인했다.
- 새 앱 PID에서 화면의 R05 표제와 파란 GPU 사각형을 확인하고 수집을 시작했다. `GO`는 2026-10-09 13:56:27.995 KST, 첫 raw 접촉은 13:56:57.655 KST였다.
- 30번째 접촉 이후 2초 drain이 끝나 13:57:22.398 KST에 자동 종료했다. raw contact 30개, active/pending contact 0, overlap 0, protocol error 0, host/device getevent 잔존 0, `captureError=null`이다.
- 앱 화면 캡처는 GPU 도형 활성화 30회를 표시한다. 직접 입력 marker 30개가 raw release와 timestamp 단위로 exact join됐다. synthetic ADB input은 사용하지 않았다.
- 30개 bracket 기반 **transaction-fence 후보 interval**의 block envelope는 **28.070127–45.156253 ms**다. `target_vsync_id=-1`이며 frame ID·panel scanout이 연결되지 않았다. 이는 제품 input-to-photon이나 p95가 아니다.
- 13:48 KST no-contact 시도는 별도의 무효 기록으로 남겼다. 이번 성공 수집과 합쳐서 늘리거나 수정하지 않았다. phase-2 누적은 **58/300** 후보, 유효 block 2/12이며 유효 block 10개가 남았다.

공개 저장소 사본은 device serial, 전체 system dump, 다른 입력 장치 목록, 터치 좌표·접촉 면적을 제외했다. `raw-touch.log`는 tracking identity·slot·timestamp·event order를 유지하고 좌표/면적 축을 제거했으며, 저장소 사본에서 분석기를 다시 실행해 같은 30/30 결과를 확인한다. 화면은 상태 표시줄과 navigation bar를 잘라냈다.

## 파일

- `capture-manifest.json`: 실행·종료 상태, APK provenance, collector 종료 기록
- `environment.json`: 기기·화면·입력 변환·전원·열 조건의 최소 요약
- `raw-touch.log`, `app.log`, `input-devices.txt`: 좌표를 제거한 raw touch와 PID별 R05 marker, direct touchscreen 정보
- `join-summary.json`: exact join과 후보 interval 전체
- `screen-before-app.png`, `screen-after-app.png`: 수집 전후 화면
- `GO.txt`, APK hash, collector stderr와 `SHA256SUMS`: 시작 anchor·APK·종료 및 digest 근거
