# R05 Android 입력 수집 supervisor 구현 검토

## 범위와 경계

검토 대상은 `tools/benchmark/capture-android-physical-touch.mjs`, `tools/benchmark/android-physical-touch-capture.mjs`와 해당 가짜 ADB 통합 테스트다. phase-2 계획의 입력 수집·종료 계약을 코드에 옮겼는지, 실패가 유효 표본으로 승격되지 않는지 확인했다. 실제 Android ADB의 제한 시간 종료, 실제 사용자 입력, 입력→앱 이벤트 exact join과 fence 분석은 아직 실행하지 않았다.

## 서로 다른 실패 관점 검토

| # | 실패 관점 | 확인한 방어와 수정 | 확인 근거 |
|---:|---|---|---|
| 1 | 누락·중복·알 수 없는 CLI 옵션으로 다른 capture 실행 | 필수 provenance, duplicate option, event 경로와 화면 확인 flag를 검사한다. | `parseOptions` 단위 테스트 |
| 2 | 큰 timeout 숫자가 `Infinity`로 변환되어 무제한 대기가 됨 | timeout과 수량을 안전한 정수 범위로 제한한다. | `parseOptions` 단위 테스트 |
| 3 | ADB 출력에 두 기기 또는 offline/unauthorized 기기가 추가됨 | 전역 `adb devices -l` 결과가 지정된 online 한 대와 정확히 일치해야 한다. | device-list 단위 테스트 |
| 4 | 사용자가 지정한 serial과 실제 수집 대상이 달라짐 | 모든 기기 명령에 동일한 `-s` 값을 쓰고 device list를 unscoped로 검사한다. | preflight 코드 검토·가짜 ADB 통합 테스트 |
| 5 | 오래 실행된 앱 process가 fresh block으로 들어감 | `pidof`, `/proc/<pid>/stat`, uptime, clock ticks로 PID와 process age를 확인하고 상한을 둔다. | process-age 단위 테스트·preflight 코드 검토 |
| 6 | 다른 APK 또는 stale APK의 로그가 동일 실험에 섞임 | 설치된 단일 `base.apk` SHA-256이 지정값과 같아야 한다. | preflight 코드 검토·가짜 ADB 통합 테스트 |
| 7 | `/dev/input/event60`이 `/dev/input/event6` 요청과 부분 문자열로 오인됨 | input device header에서 event 경로를 완전 일치시킨다. 부분 문자열 검사를 수정했다. | exact path 부정 테스트 |
| 8 | touchpad나 proximity node를 direct touchscreen으로 오인 | 이름 `sec_touchscreen`, `INPUT_PROP_DIRECT`, `ABS_MT_TRACKING_ID`를 같은 device block에서 확인한다. | capability 단위 테스트 |
| 9 | 잠금화면·다른 앱·R05 flag 누락 상태에서 표본을 받음 | resumed activity와 PID 한정 R05 startup marker, operator의 R05 화면 확인을 모두 요구한다. | preflight 코드 검토·가짜 ADB 통합 테스트 |
| 10 | PNG signature만 있는 잘린 화면을 화면 근거로 저장 | PNG signature뿐 아니라 IHDR 크기와 IEND 존재를 확인하고 시작·종료 화면 모두 검사한다. signature-only 판정을 강화했다. | fake PNG 통합 fixture·코드 검토 |
| 11 | getevent timestamp의 10자리 이상 소수부를 조용히 잘라 잘못된 시각을 만듦 | 소수부 최대 9자리만 파싱하며 원래 source 정밀도를 유지한다. | timestamp 단위 테스트 |
| 12 | 멀티슬롯 입력을 한 손가락 탭으로 합침 | tracking ID와 slot별 active 상태를 보존하고 overlap을 기록한다. | 2-slot 단위 테스트 |
| 13 | down 없는 release를 정상 contact로 세거나 tracking ID 교체를 숨김 | 잘못된 release와 이전 release 없는 tracking ID 교체를 protocol error로 둔다. | raw contact counter 단위 테스트 |
| 14 | release 뒤 `SYN_REPORT` 전에 끝난 frame을 완전한 contact로 인정 | pending release frame 개수를 별도로 추적하고 0이 아니면 capture gate를 닫는다. counter에 pending 상태가 빠졌던 부분을 보강했다. | pending-frame 단위 테스트와 gate matrix |
| 15 | 시스템 wall clock 변경으로 60초 제한이 짧아지거나 길어짐 | capture deadline은 `performance.now()` 단조 시계를 사용한다. wall-clock `Date.now()` 기준을 수정했다. | timer 코드 검토·timeout 통합 fixture |
| 16 | 첫 접촉이 없는 run을 성공/0ms sample로 기록하거나 run 오류를 preflight 오류로 오분류 | 자동 `prestart_no_contact` 종료, `captureStatus=no_contact_timeout`, `captureWindowValid=false`, `runError`를 기록하고 성공 반환을 거부한다. `preflightError`라는 잘못된 분류를 수정했다. | no-contact fake-ADB 통합 테스트 |
| 17 | 30번째 contact 전후 추가 입력·불완전 접촉을 성공 count에 섞음 | 완결 contact group count를 사용하고 active/pending/protocol error gate를 요구한다. 분석은 별도 join 상태로 남긴다. | contact-limit 및 gate matrix 테스트 |
| 18 | Node `spawn` 이벤트를 놓쳐 supervisor가 준비 대기에서 멈춤 | child의 PID 기반 readiness를 확인하고 spawn error listener를 즉시 연결한다. 기존 fake 통합 테스트에서 정지 상태를 재현해 수정했다. | 두 stream 동시 수집 fake-ADB 통합 테스트 |
| 19 | 시작 도중 SIGINT, GO anchor 실패, PID 재사용 때 다른 process에 신호를 보내거나 collector를 남김 | startup interrupt를 보류해 안전한 shutdown 시점에 처리하고, 각 child의 PID·PPID·시작 시각·명령을 재확인한 뒤 단계별 bounded signal을 보낸다. identity mismatch면 신호하지 않는다. | 시작·종료 코드 검토, PID identity·signal escalation 단위 테스트 |
| 20 | device `getevent` 잔존, partial raw line, flush 실패를 성공 처리하거나 capture 완료를 exact join으로 잘못 부름 | remote process 잔존·최종 line·stream flush·checksum을 검사한다. `captureWindowValid`는 `raw_capture_complete_pending_join_validation` 상태만 만들고 `analysisStatus=not_analyzed_join_required`를 유지한다. | no-contact/성공 통합 테스트, checksum 검사, capture gate 코드 검토 |

## 실행 확인과 남은 한계

`mise exec -- bun run test:benchmark:r05-physical-touch`에서 15개 테스트가 통과했다. 가짜 ADB는 정상 2 contact 자동 종료, no-contact 제한 종료, 두 log stream 동시 수집, PNG·manifest·checksum 기록, child signal 종료를 확인했다. 별도 `node --check` 세 파일과 `git diff --check`도 통과했다. Android 16 실기기에서도 2초 no-contact smoke와 새 process의 60초 no-contact 시도를 실행해 두 host client가 SIGINT로 종료되고 device `getevent`가 남지 않는 것을 확인했다. 두 run의 raw count는 0이고 capture는 미완료다.

실기기 입력 양성 경로, 긴 제한 시간, USB/ADB 분리 중 teardown, raw→앱→submit→callback→fence exact join은 아직 확인하지 않았다. fake ADB나 무접촉 smoke는 이 경로들을 대체하지 않는다. 새 raw 성공 capture도 별도 analysis join 전에는 phase-2 표본이 아니다. R05.3은 미완료다.
