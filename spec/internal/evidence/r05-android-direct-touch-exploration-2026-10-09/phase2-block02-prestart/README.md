# R05 phase-2 block 02 사전 입력 없음

## 관측 결과

- 기기: Samsung SM-S731N / Android 16 / API 36. 같은 phase-2 APK SHA-256을 확인했고 새 앱 process에서 R05 화면과 파란 GPU 사각형이 보였다.
- `GO`: 2026-10-09 13:48:18.279 KST. collector의 첫 raw 접촉 대기 한도는 60초였다.
- 종료: 2026-10-09 13:49:18.971 KST. `stopReason=prestart_no_contact`, `captureStatus=no_contact_timeout`, `captureWindowValid=false`다.
- `sec_touchscreen` raw completed contact 0건, 해당 앱 PID의 R05 입력·submit·touch marker 0건이었다. 기기의 `getevent` 잔존은 0개였고 host `getevent`·`logcat` client는 SIGINT로 종료됐다. `captureError=null`이다.
- 시작·종료 cropped screenshot에는 같은 R05 화면이 보인다. raw 자료는 화면을 실제로 탭했는지 여부를 기록하지 못했으므로 operator 동작 원인을 추정하지 않는다.
- capture 종료 뒤 사용자가 30회 탭 완료라고 응답했지만 응답 시각만으로 각 탭이 60초 수집 window 안에 있었는지는 확인할 수 없다. 이 응답을 raw 입력이나 성공 표본으로 역결합하지 않았다.

이것은 점수화 block이 아닌 `prestart_no_contact` 시도다. 원시 접촉을 사후에 붙이거나 0ms 표본으로 세지 않는다. 이 시점 누적은 **28/300**, 유효 block은 1/12였다. 이후 새 GO와 fresh app process로 block 02를 다시 수집한 별도 결과는 [직접 입력 evidence](../phase2-block02-direct-touch/README.md)에 있으며 현재 누적은 58/300이다.

전체 `dumpsys activity`, `battery`, `display`, `input`, 입력 장치 및 열 상태 덤프는 앱 목록·최근 입력 상태 등 무관한 정보를 포함할 수 있어 공개 저장소 사본에서 제외했다. 캡처 manifest는 device serial과 host command를 제외한 최소 필드로 재작성했다. 화면 이미지는 상태 표시줄·navigation bar를 잘라냈다.

## 파일

- `capture-manifest.redacted.json`: 무효 종료 상태, 시각, raw count, collector 종료 결과
- `raw-touch.log`, `app.log`: 빈 raw contact와 앱의 R05 입력 marker 부재 확인 자료
- `screen-before-app.png`, `screen-after-app.png`: 수집 전후 R05 화면
- `GO.txt`, `getevent-stderr.txt`, `logcat-stderr.txt`, `installed-apk-sha256.txt`: 시작 시각·collector 오류·APK 확인
- `SHA256SUMS`: 저장소에 보존한 파일 checksum
