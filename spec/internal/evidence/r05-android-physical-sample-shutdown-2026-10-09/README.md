# R05 phase-2 무접촉 캡처와 종료 절차 재검토

## 실행 사실

- 기기: Samsung SM-S731N / Android 16 / API 36, 앱 process `28243`이 foreground였다.
- 실행: 2026-10-09 11:26:38 KST에 raw/app capture를 준비하고 60초 관찰했다. `stop-reason.txt`는 `timeout_no_contact`다.
- raw 입력: `raw-touch.txt` 0행, `getevent-stderr.txt` 0바이트다. 앱 캡처는 R05 surface/WGPU 시작 로그만 있고 이 process의 `SPINON_R05_INPUT`, `SPINON_R05_SUBMIT`, `SPINON_R08_TOUCH`는 없다.
- 종료 문제: host의 background `adb shell getevent`와 `adb logcat` client가 SIGINT 뒤에도 남았다. bounded wait가 구현되지 않은 supervisor가 기다려 종료가 멈췄다. 정확히 확인한 두 host PID에 SIGTERM을 보낸 뒤 `getevent_exit=143`, `logcat_exit=143`으로 반환됐다. 종료 후 host ADB client와 기기의 `getevent`가 남지 않은 것을 확인했다. 앱 process는 유지됐다.
- 사용자가 뒤이어 완료했다고 보고한 5회 탭은 capture 종료 뒤라 raw/app 동시 기록이 없다. [후속 화면](screen-after-user-confirmed-taps.png)은 파란 사각형 그대로이고, 현재 앱 process의 R05 입력 로그도 없다. 입력 시도 여부를 부정하는 자료가 아니라 이 capture에 들어오지 않았음을 보여 주는 자료다.
- 판정: 이것은 phase-2 block이 아니라 `prestart_no_contact` 실패 시도다. 표본에 0건을 넣거나 입력을 사후 결합하지 않는다. 유효 표본은 계속 0/300이다.

종료 판정에 필요한 최소 캡처 증거만 [`capture/`](capture/)에 보존했다. 전체 `dumpsys activity`, `battery`, `display`, `input`, 입력 장치 목록, 열 상태 덤프는 앱 목록·최근 입력 상태 등 무관한 기기 정보를 포함할 수 있어 저장소에서 제외했다. [`source-SHA256SUMS.txt`](capture/source-SHA256SUMS.txt)는 저장소에 남긴 캡처 파일만 대상으로 다시 만들었다. 후속 화면 SHA-256: `b16f312de2065ab4b5ef5fc6d484ca149a21222f2d165eebee5c4479fbab93f8`.

## 종료 절차에 대한 별도 20관점 검토

| # | 공격 관점 | 검증 결과와 필요한 방어 | 판정 |
|---:|---|---|---|
| 1 | 무접촉 timeout을 성공한 0ms 측정으로 처리 | raw 0은 점수가 아니다. `prestart_no_contact`로 분류하고 block/표본에서 제외한다. | 통과 |
| 2 | timeout 뒤 사용자 입력을 과거 capture에 붙임 | late tap에는 raw event와 동시 app log가 없으므로 이 캡처에 join하지 않는다. 이후 계획에도 사후 결합 금지를 유지한다. | 통과 |
| 3 | 30회 중 사용자가 일부만 한 뒤 완료를 알려도 계속 기다림 | 새 계약은 사용자가 완료 또는 중단을 알리면 현재 관측 수와 관계없이 즉시 닫고, 30 미만은 불완전으로 둔다. | 수정 반영 |
| 4 | 사용자의 5회 완료를 phase-2의 30회 완료로 오해 | 사용자 보고와 raw contact 수를 분리한다. 이번 다섯 회는 capture 기록이 없어 scored count에 넣지 않는다. | 통과 |
| 5 | background ADB child가 SIGINT를 무시 | 실제 두 client가 SIGINT 뒤 남았다. 종료 신호만 보낸 것을 완료로 보지 않도록 바꿨다. | 발견 · 수정 반영 |
| 6 | SIGTERM도 처리하지 않는 client가 무한 대기 | 계획에 동일한 정확한 process identity를 확인한 뒤 SIGKILL로 단계 상승하는 조건을 추가했다. | 수정 반영 |
| 7 | 최종 강제 종료도 wait 없이 성공 처리 | 각 SIGINT/SIGTERM/SIGKILL 뒤 개별 bounded wait를 요구하고, 만료는 collector 실패로 둔다. | 수정 반영 |
| 8 | wait 결과 143을 정상 input capture로 오해 | 이번 143은 TERM 종료 증거다. 강제 종료 exit code는 capture 성공 증거가 아니도록 명시했다. | 수정 반영 |
| 9 | 종료 reason을 자식 종료 뒤 덮어써 분류를 잃음 | `timeout_no_contact`를 보존하고, 종료 signal·exit status와 별도 필드로 구분한다. | 통과 |
| 10 | 빠르게 재사용된 PID에 신호를 보내 무관한 프로세스를 종료 | PID만으로 종료하지 않고 PID·명령·parent·시작 시각을 묶어 identity를 확인한다. 불일치/확인 불가는 실패 처리한다. | 수정 반영 |
| 11 | `adb kill-server`로 사용자 다른 기기의 세션도 끊음 | 계획은 전체 ADB server 종료를 금지하고 이 run이 시작한 client identity만 대상으로 제한한다. | 수정 반영 |
| 12 | 같은 host의 다른 ADB client까지 이름 검색으로 종료 | `pgrep adb` 같은 전역 이름 종료 대신 run에서 기록한 exact identity만 대상으로 한다. | 수정 반영 |
| 13 | host ADB client 종료가 원격 `getevent` 종료를 보장한다고 가정 | host client와 기기 내 `getevent` 상태를 각각 검사한다. | 수정 반영 |
| 14 | 원격 `getevent` 확인 명령이 실패했는데 잔존 없음으로 간주 | 기기 확인 결과가 없거나 모호하면 teardown 성공으로 처리하지 않고 block을 무효화한다. | 수정 반영 |
| 15 | getevent client만 닫고 별도 logcat client를 남김 | 두 stream을 독립 client로 추적하고 각각 종료·wait·잔존 검사를 수행한다. | 수정 반영 |
| 16 | 자식 종료 전에 stdout/stderr를 읽지 않아 pipe deadlock 발생 | 계획은 stdout/stderr를 각 파일로 직접 리디렉션하고, client 종료 및 `wait` 뒤 flush/close·file size·마지막 event·checksum 확정을 요구한다. 이 동작은 다음 실제 collector 실행에서 검증해야 한다. | 수정 반영 · 런타임 확인 대기 |
| 17 | 마지막 접촉의 BTN/SYN 레코드가 기록되기 전에 manifest/checksum 생성 | 마지막 완전 contact group·`SYN_REPORT`·raw count를 확인한 뒤에만 성공 manifest를 만든다. | 수정 반영 |
| 18 | raw와 app count가 같다는 이유만으로 입력을 exact join | count 비교는 진단일 뿐이다. tracking/contact와 `input_seq`에서 submit/callback/fence까지 유일하게 연결해야 한다. | 통과 |
| 19 | 앱 화면 색상이나 screenshot만으로 raw direct touch를 주장 | screenshot은 보조 자료다. 현재 캡처는 화면이 파란색이고 raw/app event가 없어 positive touch로 판정하지 않는다. | 통과 |
| 20 | teardown 실패 artifact를 phase-2 block에 섞거나 삭제 | 실패 상태 판정에 필요한 최소 캡처와 screenshot을 내부 evidence에 보존하고 0/300을 유지한다. 무관한 전체 시스템 덤프는 저장소에서 제외하며, 원시 기록은 Tailscale 요약 페이지에 올리지 않는다. | 통과 |

## 계획에 반영한 수정

[R05.3 수집 계획](../../../../plan/r05-android-physical-sample-collection.md)의 종료 계약을 실제 실패에 맞췄다. host child의 SIGINT 무시를 반영해 PID·명령·parent·시작 시각으로 process identity를 확인하고, bounded SIGINT→SIGTERM→SIGKILL, 단계별 2초 wait, host/device 잔존 검사, 파일 flush 이후 최종화, 조기 완료/중단 즉시 종료를 추가했다. 남은 제한 시간 초과나 잔존 process는 수집 실패다. 이 teardown 계약은 다음 실제 collector 실행에서 재검증해야 한다.

후속 검증에서는 새 collector로 실제 Android 16 기기에서 2초 no-contact smoke를 수행했다. host getevent/logcat client는 SIGINT로 닫혔고 device `getevent` 잔존은 0개였다. 새 직접 입력 sample은 포함되지 않았다. 원본과 redacted manifest는 [실기기 collector smoke](../r05-android-direct-touch-exploration-2026-10-09/collector-device-smoke/README.md)에 있다. 이 다섯 탭은 여전히 과거 capture 종료 뒤 보고된 입력이므로 재사용하지 않는다.
