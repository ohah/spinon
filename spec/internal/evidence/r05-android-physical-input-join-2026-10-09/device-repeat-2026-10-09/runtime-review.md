# 실행 자료 검토 · Android R05 실기기 재검증

| # | 실패 경로·오판 위험 | 이 실행에서 확인한 근거와 판정 |
|---:|---|---|
| 1 | 에뮬레이터 결과를 실기기로 표기 | `adb devices -l`에서 Samsung SM-S731N USB physical device 하나를 확인했다. |
| 2 | 테스트 APK를 다른 build와 혼동 | 설치본을 추출해 PR #79 debug artifact의 SHA-256과 대조했다. 일치했다. |
| 3 | 검사 도중 APK를 바꿔 실제 실행 artifact를 모름 | APK를 재설치·재빌드하지 않았다. |
| 4 | OS/API 버전이 맞지 않음 | Android 16, API 36, SDK_INT_FULL 36.1을 기기 property에서 확인했다. |
| 5 | GPU backend를 WGPU로 잘못 주장 | PID log에 Vulkan과 Samsung Xclipse 940이 기록됐다. |
| 6 | 화면 크기와 탭 좌표가 불일치 | 기기 해상도 1080×2340에서 이전 검증 좌표 (540,1170)를 사용했고 화면 활성화 수가 증가했다. |
| 7 | raw 장치 이름만으로 touchscreen을 선택 | `sec_touchscreen`의 `INPUT_PROP_DIRECT`와 touch event capability를 확인했다. |
| 8 | 재연결에 따라 바뀔 event node를 고정 | 실행 직전 `sec_touchscreen`이 `/dev/input/event6`임을 확인했다. 기록은 이번 실행에만 적용한다. |
| 9 | 수집기 시작 전 tap을 보내 raw가 비었다고 오인 | negative-control·physical-window·repeat 각각의 raw 수집 세션을 tap 전에 시작했다. |
| 10 | 권한 거부된 getevent를 0 input으로 해석 | 각 getevent 세션은 foreground 프로세스로 실행 중이었고 permission 오류가 관측되지 않았다. |
| 11 | ADB injected MotionEvent를 물리 contact로 오인 | synthetic tap 한 건과 10회 모두 앱 입력은 기록됐지만 raw touchscreen은 0 bytes였다. |
| 12 | 물리 입력 창에서 앱 로그만 누락되어 touch를 놓침 | 약 60초 physical window에서 raw와 앱 PID log를 함께 수집했다. 둘 다 입력 0건이다. |
| 13 | 앱 `input_source=unknown`을 출처 증거로 과장 | 출처 판정에 이 필드를 사용하지 않았다. |
| 14 | input sequence 중복·누락·순서 추정 | 반복 block의 `input_seq`는 정확히 1–10, 중복·누락 없이 확인했다. |
| 15 | 입력이 GPU 제출에 연결되지 않음 | sequence/revision 1–10과 generation 1의 WGPU submit 10건을 일치시켰다. |
| 16 | submit 뒤 실제 transaction callback 누락 | 동일 request/input/revision/generation의 `TransactionStats` callback 10건을 확인했다. |
| 17 | fence 신호가 invalid·pending·다른 요청 것 | 복제 fence 결과 10건 모두 같은 key의 `signaled`, `fence_signal_usable=true`였다. |
| 18 | pending 작업이 종료 후 남음 | 각 signal 결과의 pending/active/queue depth와 마지막 완료 상태가 모두 0이었다. |
| 19 | VSync 미지원 결과를 latency/scanout로 확대 | 모든 callback에서 `target_vsync_id=-1`; 입력 latency·VSync·scanout·p95를 주장하지 않았다. |
| 20 | 테스트 뒤 개발자 기기 상태를 바꿈 | 화면 Awake, 밝기 248, timeout 30,000 ms, mode 2/60 Hz를 대조하고 Chrome foreground로 복귀했다. |

이 검토는 실행 자료의 실패 경로와 주장 범위를 확인한다. 물리 입력 positive sample을 대신하지 않으며 R05.3 또는 R05 완료 판정에 사용하지 않는다.
