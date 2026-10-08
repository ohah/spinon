# 실행 증거 적대 검토 · Android 실기기 재검증

2026-10-09 재실행의 APK·전경 process·입력 출처·프레임 연결·복구 자료를 서로 다른 실패 관점 20개로 대조했다. 이 검토는 테스트 결과의 증거 품질 검토이며 제품 기능 구현이나 R05.3 완료 선언이 아니다.

| # | 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | emulator를 실기기로 잘못 선택 | ADB에는 SM-S731N 물리 기기 하나만 연결되어 있었고 실행 중 emulator process는 없었다. |
| 2 | 다른 설치본 또는 오래된 APK 실행 | 기기에서 pull한 설치 APK hash가 공식 PR #79 artifact의 SHA-256과 일치했다. |
| 3 | 이미 실행 중인 process의 stale log 재사용 | 앱을 force-stop하고 새 PID 27283으로 시작했으며 PID-filtered log를 수집했다. |
| 4 | 앱이 전경이 아닌데 입력 수집 | 시작 시 resumed Activity가 Spinon이었고 화면 캡처에 R08 WGPU 도형이 보였다. |
| 5 | 중앙 target 밖에 입력 | 540×1170은 1080×2340 화면 중앙의 GPU 도형 내부다. |
| 6 | ADB 주입을 물리 touch로 집계 | command protocol은 명시적으로 ADB synthetic tap이며 raw touchscreen은 0줄이다. 물리 표본으로 세지 않았다. |
| 7 | touchscreen node를 다른 센서나 touchpad로 선택 | 매 run `getevent -lp`로 `sec_touchscreen`과 `INPUT_PROP_DIRECT`를 다시 확인했고 `sec_touchpad`는 제외했다. |
| 8 | raw capture 미시작을 0 touch로 오인 | getevent process를 실행 중인 capture로 열고 ADB 입력 전체 구간에 유지한 후 의도적으로 종료했다. 파일은 0 bytes다. |
| 9 | unknown input source를 finger source로 가정 | 앱 `input_source=unknown`을 기록하고 Android event metadata로 직접 입력을 주장하지 않았다. |
| 10 | app input sequence 누락·중복을 놓침 | PID log에서 accepted `input_seq` 1–11이 각각 한 번씩 관측됐다. |
| 11 | input과 submit의 revision을 순서만으로 연결 | 각 submit의 `input_seq`/revision이 1–11로 대응하고 generation은 1이었다. |
| 12 | submit log만 보고 callback을 추정 | `TransactionStats` callback marker가 11개 request 모두에서 관측됐다. |
| 13 | callback 이후 fence signal을 추정 | async worker 결과가 11/11 `signaled`, `fence_signal_usable=true`였다. |
| 14 | stale signal 또는 다른 surface generation을 현재 결과로 연결 | 모든 sequence가 generation 1/current surface에 있었고 block 중 surface 전환 표식은 없었다. 한 block이므로 lifecycle 전환은 별도 미검증이다. |
| 15 | queue가 남은 채 성공 처리 | 마지막 async 결과의 pending/active/queue_depth는 각각 0, completed는 11이었다. |
| 16 | worker wait 시간을 제품 입력 latency로 부름 | 0.105–12.660 ms는 await 호출 구간일 뿐 입력→표시 수치로 사용하지 않았다. |
| 17 | OS fence를 VSync·광자 시각으로 과장 | API 36.1 `target_vsync_id=-1`; transaction fence는 optical scanout이 아니다. |
| 18 | screenshot만으로 log join 성공을 주장 | visual tap count/color와 별도로 sequence 기반 input/submit/callback/wait log를 대조했다. |
| 19 | 기기 상태 변경 또는 앱을 전경에 방치 | brightness/timeout/stay-awake/display mode를 전후 비교했고 Chrome을 foreground로 복귀, Spinon process는 종료했다. |
| 20 | 작은 단일 block으로 성능 일반화 | 11개 synthetic 입력 feasibility 재실행으로만 기록했다. p95, renderer 우열, 실제 finger input, R05.3 완료는 미주장이다. |

### 검토 결과

증거는 동일 실기기·동일 APK에서 synthetic input으로 11/11 input→submit→callback→async fence signal 경로가 재현된다는 주장만 지지한다. 실제 손가락 input-to-present join은 수집되지 않았으므로 이 부분은 보류다. API 36.1 VSync attribution, release runtime, scanout, 성능 일반화도 검증되지 않았다.
