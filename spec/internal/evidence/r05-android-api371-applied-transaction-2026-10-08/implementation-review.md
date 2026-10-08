# API 37.1 구현·실행 적대 검토

검토 대상은 병합된 Android lifecycle fixture/runner의 API 37.1 실행 결과, 실행 source/APK digest, environment manifest, QEMU 상태, 전체 Logcat과 screenshot이다. API 37.1 runtime 결과에 대한 새 검토이며 계획의 20개 표를 재사용하지 않는다.

| # | 독립 실패 관점 | 확인 근거 | 결과 |
|---:|---|---|---|
| 1 | API 이름으로 실제 platform version을 대신함 | `sdk_full=37.1`, `api=37`을 runner와 Android property 양쪽에서 확인 | 대상 일치 |
| 2 | 16KB AVD 표기만 믿고 guest page size를 추정 | `getconf PAGE_SIZE=16384` 및 environment manifest | 대상 일치 |
| 3 | 다른 serial 또는 비 ARM64 실행을 혼합 | `emulator-5580`, `spinon_api37_1_compat16k`, `arm64-v8a` | 대상 일치 |
| 4 | AVD 삭제 후 fresh boot로 결과를 과장 | data directory absent, QEMU PID 6922와 open deleted files 기록 | 기존 live process; cold boot 아님 |
| 5 | V8/source snapshot이 달라 결과가 비교 불가 | pinned V8 clean revision, source HEAD, 8개 source SHA-256 일치 | snapshot 연결됨 |
| 6 | 실제 callback 아닌 synthetic handler를 OS callback으로 셈 | baseline의 `stats_available=true`, callback outcome/count 로그 | `TransactionStats` 실제 전달 확인 |
| 7 | baseline callback 누락·중복을 정상 처리 | request 108에서 callback count 1, pending/timer 제거, executor idle | 정상 경로 통과 |
| 8 | 다른 입력·request에 callback을 연결 | request id 108–110, input sequence 1–3, generation tuple 기록 | tuple 연속성 확인 |
| 9 | API 37.1에서 R08 draw 없이 callback만 관찰 | revision 1/gen1, 2/gen1, 3/gen2 draw marker | 세 marker 통과 |
| 10 | callback을 queue에 넣기 전에 surface를 파괴 | 각 scenario에서 queue=1·active=1·count=0·request pending을 기록 | 대기 후 detach |
| 11 | 실제 view 경계 대신 취소 helper만 직접 호출 | `surface_destroyed=true`, 같은 view detach/reattach marker | 실제 lifecycle 통과 |
| 12 | destroy/recreate가 이전 callback보다 늦음 | request 109 queue 상태 뒤 destroy·cancel 로그, then callback release | 취소가 먼저 완료 |
| 13 | timeout이 cancellation과 경쟁해 먼저 승리 | `cancelled_before_timeout=true`, timeout count=0, timer 없음 | 취소가 terminal winner |
| 14 | 같은 view의 재부착 뒤 generation이 초기화·유지됨 | `same_view=true`, generation 1→2 | 단조 증가 확인 |
| 15 | stale callback이 새 surface에 귀속 | `late_after_cancel`, `current_surface=false` | 이전 세대로 격리 |
| 16 | signaled fence를 stale 화면 표시 성공으로 과장 | fence state signaled이나 `fence_signal_usable=false` | unusable 유지 |
| 17 | 새 세대 executor callback 경로가 복구되지 않음 | request 110, generation 2, stats 존재, callback 1, current surface true | 복구 경로 통과 |
| 18 | 100회 timeout/callback 경합의 terminal 누수 | 100 iterations, callback wins 58, timeout wins 42, pending 0 | 각 반복 terminal 상태 하나 |
| 19 | overflow 이후 queue 잔류 또는 idle 누수 | queue 64 뒤 drain/caller-runs, 60초·60 sample, pending/queue/active=0 | 정리 후 안정 |
| 20 | 결과 화면·환경·실행 artifact가 서로 다른 실행 | run-01 manifest·APK/source digest·Logcat·PASS screenshot 동일 폴더 | 증거 묶음 일치 |

검토 중 새 코드 결함은 발견하지 않았다. 이 실행의 증거는 API 37.1 debug emulator 한 대에 한정된다. API 35·37.0 actual lifecycle, API 29–34 fallback, iOS device runtime, 실기기와 제품 event-to-present는 계속 미검증이다.
