# PR 검토 메모

| # | PR 수준의 실패 관점 | 판정 |
|---:|---|---|
| 1 | 변경이 R05 완료 상태를 암시 | 로드맵은 사용자 직접 입력 대기로 남기고 R05 상태 체크와 phase-2 수치는 바꾸지 않는다. |
| 2 | 실험 도구 수정이 공개 API 계약으로 오해됨 | public API와 내부 interface 버전 변경은 없다. |
| 3 | 고정 APK 표본과 새 APK가 혼합됨 | SHA가 다른 exploratory 실행은 58/300 phase-2 집계에서 제외한다. |
| 4 | ADB synthetic tap을 physical 입력으로 표기 | 화면·로그·README에서 synthetic negative control로 분리한다. |
| 5 | 60초 no-contact을 유효 block으로 처리 | `prestart_no_contact`이며 `captureWindowValid=false`라고 기록한다. |
| 6 | 무접촉 뒤 도착한 입력을 sample에 붙임 | collector는 timeout 종료했고 late input은 추가하지 않았다. |
| 7 | 주황색 GPU 화면을 direct-touch 증거로 오인 | 화면 파일명과 캡션에 synthetic control임을 표시한다. |
| 8 | present callback 시각을 optical latency로 확대 | `target_vsync_id=-1`; VSync, scanout, photon, p95 주장은 없다. |
| 9 | callback 시점의 pending fence를 usable로 오인 | callback과 별도 async wait `signaled` 결과를 log에서 구분한다. |
| 10 | wait 뒤 queue 잔류를 무시 | control event는 `pending=0`, `active=0`, `queue_depth=0`으로 종료했다. |
| 11 | 설치 APK가 빌드 APK와 다름 | pulled installed APK와 local build SHA-256이 일치했다. |
| 12 | 다른 OS/API/기기의 결과를 합산 | evidence는 SM-S731N Android 16/API 36 한 기기의 exploratory run으로 제한한다. |
| 13 | R05 target이 아닌 앱 화면을 캡처 | 시작 화면의 R05 WGPU/Vulkan title과 중앙 GPU target을 확인했다. |
| 14 | target surface가 GLES 또는 emulator rendering | app marker는 `renderer=wgpu`, `SurfaceView`; 기기는 SM-S731N, `ro.kernel.qemu=0`이었다. |
| 15 | device list 변경으로 AVD가 기본 target이 됨 | explicit serial만 계속 사용하고 AVD target은 거부한다. |
| 16 | 추가 physical 기기가 opt-in 아래 숨겨짐 | opt-in은 online `emulator-<port>` 항목만 허용한다. |
| 17 | stale/offline AVD가 정상으로 간주됨 | extra entry state는 정확히 `device`여야 한다. |
| 18 | damaged `adb devices` output에서 첫 행 손실 | header를 확인하고 malformed listing은 거부한다. |
| 19 | screen confirmation이나 fresh PID gate 제거 | 두 preflight 조건은 그대로 유지된다. |
| 20 | 잘못된 touchscreen node에서 raw를 수집 | `sec_touchscreen`, direct property, MT tracking 검사를 유지한다. |
| 21 | app event가 다른 process에서 합류 | PID 한정 logcat과 fresh app PID 검사를 유지한다. |
| 22 | opt-in이 모든 연결을 무제한 허용 | 기본값과 validator 모두 최소 범위이며 non-AVD는 거부한다. |
| 23 | opt-in 사용 여부가 capture 후 사라짐 | manifest policy에 boolean을 기록한다. |
| 24 | screenshot 또는 log에 기기 serial이 노출 | 저장소 evidence는 serial을 제외하고 capture manifest 원본은 repo에 넣지 않는다. |
| 25 | 이미지가 사용자 알림 내용을 노출 | 화면에는 알림 아이콘만 보이며 메시지 본문은 없다. |
| 26 | 테스트 중 화면 회전·밝기·refresh 변경 | settings와 active mode를 읽기만 했고 시작·종료 값이 동일하다. |
| 27 | collector 종료 뒤 host ADB child 잔존 | 두 수집 manifest 모두 host stop 및 remote getevent 잔존 0을 기록한다. |
| 28 | 다른 AVD 또는 ADB server를 종료 | 기존 AVD와 ADB server는 유지하고 Spinon 앱만 종료했다. |
| 29 | 브라우저 확인이 terminal browser로 실행 | PR/preview 확인에는 Agent Browser를 사용하고 종료 상태를 확인한다. |
| 30 | 미리보기 동기화가 사용자 파일을 지우거나 서버를 재시작 | 기존 Tailnet preview server를 유지하고 필요한 산출물만 백업·검증한다. |

**판정:** 코드 회귀 테스트와 synthetic control은 통과했다. PR은 사용자 direct-touch positive sample 대기를 명시하며 R05 completion gate를 통과한 것으로 주장하지 않는다.
