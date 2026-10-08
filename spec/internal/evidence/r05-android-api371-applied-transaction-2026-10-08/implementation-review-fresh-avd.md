# API 37.1 fresh AVD 구현·실행 실패 경로 검토

검토 대상은 새로 만든 API 37.1 AVD에서의 `run-02-fresh-avd`, AVD 복구 증거, 현재 runner/source hash와 실제 Android callback log다. 이전 AVD의 open deleted file을 사용한 run-01 및 AVD 복구 계획 검토를 재사용하지 않았다.

| # | 독립 실패 관점 | 대조한 실행 증거 | 결과 |
|---:|---|---|---|
| 1 | 새 AVD에서 다른 serial·기기로 실행 | `environment.txt`: `emulator-5580`, `spinon_api37_1_compat16k`, `sdk_gphone16k_arm64` | target tuple 일치 |
| 2 | 이전 live QEMU를 fresh boot로 재라벨링 | 이전 PID 6922 종료 확인, 새 AVD data directory 생성, 새 PID 56444 기록 | 실행 분리됨 |
| 3 | foreground session 종료 뒤 emulator가 사라지는데 fixture를 계속 실행 | 첫 background launch 실패는 fixture 이전 차단으로 기록. foreground session PID 56444와 post-run ADB 연결을 확인 | 첫 시도는 PASS로 세지 않음; 수정 실행 유지됨 |
| 4 | 실행 중 AVD data directory가 다시 삭제되거나 unlinked | `avd_data_dir_present=true`, post-run `lsof`에서 해당 QEMU의 `.android/avd/` 삭제 file 없음 | 새 backing directory 사용 |
| 5 | 삭제된 원래 hardware profile과 같은 화면이라고 주장 | 기존 `config.ini` 부재, 새 `pixel_6` profile 1080×2400/420 기록 | geometry 비교 주장을 제외 |
| 6 | 37.2 image 또는 다른 revision을 API 37.1로 오인 | image `source.properties`: API 37.1, 16KB tag, ARM64, revision 9; guest full version 37.1 | image와 guest 일치 |
| 7 | 16KB 호환성을 host에서 읽은 page size로 추정 | guest `getconf PAGE_SIZE=16384` | guest에서 직접 확인 |
| 8 | AVD ini만 보고 정상 등록으로 오인 | `avdmanager list avd`, ini 경로, config image/device와 guest의 `emu avd name`을 대조 | 등록·실행 이름 일치 |
| 9 | CLI 12 orphan config가 새 실행에서 읽힘 | orphan은 `.avd.cli12-partial`로 분리되고, 새 config는 image path와 `hw.device.name=pixel_6`를 포함 | 새 AVD가 orphan과 분리됨 |
| 10 | 기존 결과 파일을 새 run으로 덮어씀 | run-01 유지, run-02 경로가 없음을 확인한 뒤 runner 실행 | 별도 산출물 생성 |
| 11 | 수정 source 또는 APK hash가 기록값과 불일치 | source hash 8/8, APK hash 확인 | 모두 일치 |
| 12 | V8 revision이 달라 callback 결과 비교가 흐려짐 | 요구·실제 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf` 일치, checkout clean | pinned V8 확인 |
| 13 | runner가 device 부재인데도 build/test를 통과 처리 | 첫 시도는 missing serial에서 build 전 실패; 최종 시도는 debug build·install log와 app PID 4084 보유 | 실패와 통과를 구분 |
| 14 | baseline transaction의 Android callback을 synthetic 결과로 대체 | request 108의 non-null stats, callback count 1, current surface true | actual callback 통과 |
| 15 | R08 draw 또는 callback queue 없이 surface lifecycle을 성공 처리 | revision 1/gen1·2/gen1 draw marker와 production executor queue=1/active=1 기록 | 사전 상태와 draw 연결 |
| 16 | detach 뒤 취소보다 callback/timeout이 먼저 완료 | request 109 queue 상태 후 실제 `surface=destroyed`, `cancelled_before_timeout=true` | 취소 순서 통과 |
| 17 | signaled stale fence를 새 surface 표시 성공으로 오인 | request 109 stats 존재·late callback·current surface false·fence unusable | stale 결과 분리 |
| 18 | 재생성된 surface에서 이전 generation을 재사용하거나 recovery 누락 | 같은 view, generation 1→2, request 110/input 3, current surface true·callback 1 | 새 세대 회복 |
| 19 | 경합·overflow·idle cleanup 중 잔류를 숨김 | 100 races: callback 52/timeout 48; queue 64 drain/caller-runs; idle 60/60; pending/queue/active 0 | 정리 조건 통과 |
| 20 | 결과 화면을 latency/제품 표시 완료로 과장하거나 다른 실행과 연결 | run-02 `result.png`의 12/100/60 PASS를 Logcat summary·PID·manifest와 대조 | debug synthetic fixture 결과로만 제한 |

첫 background-launch 시도에서는 guest boot 자체는 성공했으나 shell 세션이 끝난 뒤 QEMU가 사라져 runner가 serial을 거부했다. 해당 시도에는 build, 앱 launch, assertion이 없고 fixture PASS로 계산하지 않았다. 유지되는 foreground session에서 재실행한 run-02만 API 37.1 applied-transaction 결과로 사용한다.

최종 성공은 API 37.1 / Pixel 6 profile / debug emulator 한 실행에 한정된다. 입력은 synthetic `MotionEvent`이며 물리 입력, 제품 성능, 광학 scanout, 다른 API의 applied lifecycle, iOS device callback 또는 R05.3 전체 완료를 뜻하지 않는다.
