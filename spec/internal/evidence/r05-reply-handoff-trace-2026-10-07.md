# R05 응답 handoff trace 확인 · 2026-10-07

## 판정 기준

`reply-handoff` async slice는 Rust actor의 응답 전송 직전에 시작하고 호출자가 응답을 받은 직후 끝난다. 완료된 각 `spinon-event` 보고서의 `trace_cookie`와 async slice cookie가 정확히 하나씩 대응해야 한다. 런타임 dispatch를 생략한 `spinon-ui-only` 대조에는 이 slice가 없어야 한다. 이 확인은 표식 연결의 정확도만 판정하며 성능 합격·실기기 결과를 판정하지 않는다.

## 실행 환경

- Android API 36 / Android 16 ARM64 `sdk_gphone64_arm64` 에뮬레이터
- 1080×2400, 420 dpi, 실제 화면 주사율 60 Hz, Thermal Status 0
- 같은 APK를 사용하는 Android debug 앱; Rust `spinon-runtime`은 build script의 release profile로 빌드
- 두 조건 모두 전경 앱을 5초 안정화하고 10초 trace 중 ADB 탭 10회 실행
- 비교 조건: `spinon-event` 10회, `spinon-ui-only` 10회
- Trace Processor: Perfetto `v58.2-add693d8b`
- 기준 Git commit: `62d5939a0598559f873c3202148727873657b2a0`
- APK SHA-256: `544052aa9dcb4a06cbb085dc973254242a916dbb3b551f010d16a153dad1d213`
- 캡처 시점 source tree SHA-256: `ec37ddd55a056d717e34b5d4a97f9b8062f6c2e005f12af8b886e745bf896890`

캡처 원본은 작업 디렉터리의 `build/spinon/benchmark/android-reply-handoff-final-20261007/runs/` 아래에 보존했다. 경로는 각각 `20261007T101052Z-spinon-event/`와 `20261007T101127Z-spinon-ui-only/`다. source tree hash와 APK hash를 함께 두어 추적 계측 코드와 실제 설치된 바이너리를 구분한다.

## 관측 결과

| 조건 | 입력 | Android runtime dispatch | 응답 보고서 | reply-handoff slice | cookie 연결 실패 | trace 품질 |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `spinon-event` | 10 | 10 | 10 | 10 | 0 | ftrace setup error·ftrace drop·packet loss·discarded chunk 모두 0 |
| `spinon-ui-only` | 10 | 0 | 0 | 0 | 0 | ftrace setup error·ftrace drop·packet loss·discarded chunk 모두 0 |

`spinon-event` 보고서 cookie `2`~`11` 각각에 하나의 완결된 async slice가 있었다. trace에서 추출한 cookie와 span 길이는 다음과 같다.

| Cookie | span (µs) | Cookie | span (µs) |
| ---: | ---: | ---: | ---: |
| 2 | 126.625 | 7 | 96.167 |
| 3 | 6.125 | 8 | 9.917 |
| 4 | 7.959 | 9 | 64.709 |
| 5 | 63,009.000 | 10 | 5.500 |
| 6 | 112.125 | 11 | 142.875 |

단일 에뮬레이터 실행에서 span은 5.500 µs~63.009 ms였다. cookie `5`의 긴 표본은 V8 호출 254 µs·actor 처리 259 µs와 비교해 actor→caller 응답 handoff가 63.009 ms였고, caller thread의 wake→run은 62.941 ms로 관측됐다. 이는 이 표본에서 caller 재개 지연이 응답 구간을 차지한 정황이다. 에뮬레이터 host scheduling 기여와 실기기 원인은 확정하지 않았고, 이 단일 표본을 플랫폼 속도나 일반 응답 비용으로 해석하지 않는다. 요약기는 `trace_cookie`, caller TID와 span 시간 범위로 각 `runtime-worker-call` 및 보고서를 연결하고, 두 trace에서 연결 불일치가 없음을 확인했다. UI-only 부정 대조에서 span은 생성되지 않았다.

앱 FrameTimeline의 actual/expected surface token 집합은 두 trace 모두 일치했고, trace packet loss도 없었다. `spinon-event`에는 App Deadline Missed surface token 2개, `spinon-ui-only`에는 2개가 있었으나, 이 소량 행렬의 frame 결과는 handoff 계측 검증과 무관하므로 조건 간 성능 비교에 사용하지 않는다.

요약기의 실패 경로도 원본을 복제한 임시 행렬에서 확인했다. dispatch 보고서 cookie 하나를 trace에 없는 값으로 바꾸거나 다른 보고서 cookie와 중복시키면, 또는 필수 타이밍 필드를 지우면 귀속/완전성 불일치 회차가 1로 표시되고 종료 코드가 1이 됐다. 원본 캡처는 수정하지 않았다. 별도 worker-panic 단위 검증은 오류 응답에도 작업 종류·상태·caller TID·cookie가 남는지 확인한다. panic 시 Android trace 캡처와 복구 정책은 아직 검증하지 않았다.

## 해석과 남은 범위

- `reply-handoff`는 actor send 시작부터 caller가 응답을 받은 시점까지의 wall 경과다. 내부 CPU 실행·대기시간은 각 스레드의 scheduler trace와 따로 분류한다.
- 이번 실행은 cookie의 cross-thread 전달, 성공 응답의 완료, UI-only 대조에서의 부재를 확인했다. panic·프로세스 종료·trace가 span 중간에서 끝나는 경우의 제품 복구 정책은 검증하지 않는다.
- Android 에뮬레이터의 진단 확인이며 Android 실기기·iOS 실기기·release 최종 앱·실제 입력부터 픽셀 발광까지의 지연은 측정하지 않았다.
- R05 완료 조건과 제품 GPU renderer 귀속은 여전히 충족하지 않는다.
