# 실행 후 독립 실패 경로 검토 · 20개 관점

| # | 실패 가능성 | 실행 자료로 확인한 차단·한계 |
|---:|---|---|
| 1 | API 29·34 AVD가 서로 바뀌거나 기존 AVD를 덮어씀 | 새 이름을 만들고 `.ini` target, image 경로, guest API, profile을 각각 대조했다. |
| 2 | API 29/34 system image version을 플랫폼 API로 혼동 | system-image revision 13/14와 guest API 29/34를 별도로 기록했다. |
| 3 | 32-bit 또는 page-size 차이를 fallback 차이로 오인 | 두 guest 모두 arm64-v8a, 4096-byte page size였다. |
| 4 | 설치된 APK가 새로 빌드한 다른 artifact | 설치 전후 PR #79 APK SHA-256을 고정해 API 29·34에서 같은 artifact를 썼다. |
| 5 | APK의 minSdk가 API 29를 제외 | API 29 설치가 성공했고 APK minSdk는 29다. 시작 실패는 설치 거부와 분리했다. |
| 6 | API 29 native load error를 R05 API gate 실패로 잘못 분류 | exception은 `MainActivity.<clinit>`의 `System.loadLibrary`에서 발생했고 `SPINON_R05_` marker는 없었다. |
| 7 | 앱이 종료했지만 launcher screenshot만 보고 정상 실행으로 셈 | crash buffer의 `FATAL EXCEPTION` 및 `UnsatisfiedLinkError`와 app PID/focus 종료를 대조했다. |
| 8 | unresolved symbol 한 개만 보고 누락 runtime을 추정 | ELF undefined `_Unwind_*`와 `DT_NEEDED` 전체를 추출하고 link flags를 함께 확인했다. 세부 symbol provider의 OS 구현은 미확정으로 남겼다. |
| 9 | `-nostdlib++`와 `--unwindlib=none`을 구분하지 못함 | 두 옵션이 함께 있는 source line과 결과 ELF를 보존했다. 실제 후속 수정/재링크는 미실행이다. |
| 10 | API 29에서 공통 loader가 실패하는데 R05 flags를 반복 실행 | 모두 Activity static init 뒤라 실행하지 않았다. 반복하면 같은 pre-probe crash만 만든다. |
| 11 | API 34 bootstrap에서 API 29와 다른 APK 사용 | 같은 64-bit APK hash를 설치해 JS bootstrap result를 확인했다. |
| 12 | 기본 실행에 R05 diagnostic이 섞여 동작 | API 34 기본 process log에는 bootstrap result가 있고 R05 marker는 없다. |
| 13 | async fence의 unavailable 결과를 crash/성공으로 오해 | `status=UNAVAILABLE reason=api_below_35 api=34`를 확인했고 Activity가 종료됐다. |
| 14 | present-fence와 FrameTimeline 분기가 한 프로세스에서 섞임 | 서로 다른 fresh process에서 각 intent flag 한 개와 GLES control만 전달했다. |
| 15 | probe unavailable 뒤 GLES 제출이 중단 | present-fence 경로의 `draw_requested=true` 뒤 `draw_seq=3`, FrameTimeline 경로 뒤 `draw_seq=2`가 기록됐다. |
| 16 | synthetic ADB tap을 physical input으로 표시 | 두 probe 모두 ADB 주입 1회이며 physical positive input으로 기록하지 않았다. |
| 17 | 로그만 성공하고 화면은 갱신되지 않음 | 각 GLES probe 전/후 screenshot에서 파랑→빨강 및 activation count 1을 확인했다. |
| 18 | timestamp 표현 단위를 시계 정확도로 과장 | API 34 log의 nanosecond representation만 기록하고 accuracy·latency를 주장하지 않는다. |
| 19 | SwiftShader emulator 결과를 기기 성능 결과로 일반화 | software graphics 조건을 기록했고 모든 결과를 호환성 smoke로 제한했다. |
| 20 | 테스트 후 emulator/phone이 실행 상태로 남거나 기존 profile이 손상 | 새 AVD 두 개는 정상 종료했고 기존 정의를 보존했다. Android phone은 연결 상태로 유지했다. |

이 실행 후 검토는 이번 두 AVD의 provenance·실패 분류·fallback 분기만 다룬다. API 29 fallback 통과, API 30–33, 실제 손가락 입력, 성능, VSync/scanout, R05.3 완료를 뜻하지 않는다.
