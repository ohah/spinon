# 실기기 재검증 후 실패 경로 검토 · 20개 관점

| # | 실패 가능성 | 확인 결과와 경계 |
|---:|---|---|
| 1 | 다른 기기/에뮬레이터를 실기기로 오인 | USB physical device의 SM-S731N, API 36, ARM64 속성을 확인했다. 결과에는 장치 일련번호를 기록하지 않았다. |
| 2 | 설치 APK가 다른 바이너리 | 기기에서 `base.apk`를 추출해 기존 PR #79 SHA-256과 일치함을 확인했다. 이 commit에서 새로 빌드한 것은 아니다. |
| 3 | API 버전 오기 | Android 16 / API 36을 guest property에서 확인했다. API 37 capability를 주장하지 않는다. |
| 4 | R08 화면이 전면이 아님 | 입력 전 window focus와 resumed Activity가 `MainActivity`이고 PID 5340임을 확인했다. |
| 5 | 예상과 다른 renderer/GPU | app log가 WGPU/Vulkan 및 Samsung Xclipse 940을 기록했다. GLES 결과로 섞지 않았다. |
| 6 | 다른 input node를 관찰 | 실행 시 `sec_touchscreen` 이름과 `INPUT_PROP_DIRECT`를 대조했다. event 번호는 이 실행에서만 식별자로 썼다. |
| 7 | getevent 권한 거부를 0 입력으로 해석 | shell UID의 `getevent -lp`에서 해당 node와 capability를 읽었고 raw 수집 프로세스가 실행됐다. 0 bytes는 해당 시간 창에 관찰된 원본이다. |
| 8 | 물리 tap과 ADB 주입을 구분 못함 | 직접 입력 창과 ADB synthetic tap을 시간상 분리했다. synthetic 구간은 별도로 표시했다. |
| 9 | synthetic tap을 손가락 입력으로 보고 | synthetic은 10회로만 기록하고 physical positive count는 0으로 유지했다. |
| 10 | 잘못된 좌표로 화면 바깥을 탭 | 화면 중앙 GPU target에 해당하는 (540,1170)을 사용했고 화면 활성화 count가 10이 됐다. |
| 11 | 탭 횟수와 ACTION_UP sequence 불일치 | 제어 명령 10회, 앱 `ACTION_UP` sequence 1–10, 화면 count 10이 일치했다. |
| 12 | stale revision 또는 다른 surface generation 결합 | 완료 레코드의 revision은 1–10, generation은 모두 1이고 input sequence와 exact match됐다. |
| 13 | callback 누락·중복 | request ID/input sequence 1–10 각각 하나의 완료 fence record가 있고 합계 10이다. |
| 14 | unusable/invalid fence를 성공으로 셈 | 각 완료 기록에서 fence valid before/after와 `fence_signal_usable=true`, `outcome=signaled`를 확인했다. |
| 15 | 큐가 남았는데 drain 완료로 오인 | 각 완료 기록은 pending/active/queue depth 0/0/0이다. |
| 16 | 화면이 바뀌지 않았는데 로그만 성공 | 시작 및 종료 screenshot에서 도형을 확인했고 종료 화면에 활성화 10회가 보인다. |
| 17 | VSync/광학 표시 지연을 과장 | `target_vsync_id=-1`이라 event→present latency와 scanout은 계산하지 않았다. fence signal은 photon 시각이 아니다. |
| 18 | 앱 crash/ANR 뒤 일부 로그만 정상 | 수집 후에도 앱 PID가 foreground에 있었고 화면 캡처가 가능했다. 이 결과는 장시간 안정성 검증은 아니다. |
| 19 | 기기 설정 변경이 결과를 오염 | 화면 켜짐·mode 2/60 Hz를 기록했고 설정 변경 명령은 실행하지 않았다. |
| 20 | 로그만 저장하고 원본을 잃거나 대형 APK를 불필요하게 보존 | app log, raw input, screenshots, 환경, digest를 보존하고 추출 APK 바이너리는 해시 확인 후 제거한다. |

이 검토는 이번 실행 자료의 provenance·join·표현 범위를 검사한다. 직접 손가락 입력 sample, event-to-present latency, VSync/scanout 및 R05.3 완료를 대신하지 않는다.
