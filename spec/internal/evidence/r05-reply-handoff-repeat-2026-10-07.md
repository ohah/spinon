# R05 응답 handoff 반복 관측 · 2026-10-07

## 목적과 범위

앞선 한 번의 `spinon-event`에서 관측한 63.009ms `reply-handoff`가 반복되는지 확인했다. 이 행렬은 같은 Android 에뮬레이터의 런타임 경로와 UI-only 대조를 번갈아 실행한 진단이며, 실기기 성능이나 제품 성능 순위를 판정하지 않는다.

## 실행 조건

- Android 16 / API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터 하나만 사용했다. 실기기는 연결·사용하지 않았다.
- 같은 Android debug APK와 같은 캡처 시점 코드 상태로 조건별 독립 프로세스 10회씩 실행했다.
- 각 회차는 5초 안정화 후 20초 Perfetto trace를 수집하고, 회차마다 세 번 탭했다. 전체 입력은 event 30회, UI-only 30회다.
- 실행 순서는 UI-only→event 5쌍, event→UI-only 5쌍으로 균형을 맞췄다.
- APK SHA-256: `544052aa9dcb4a06cbb085dc973254242a916dbb3b551f010d16a153dad1d213`
- 캡처 시점 source tree SHA-256: `25e7e1bb3b30da2bcf22eac09e25c4c16345aee81454d0a8a39a42b4279d264a`
- 캡처 시작: `2026-10-07T10:21:25Z`; seed: `20261007`; Trace Processor: Perfetto `v58.2-add693d8b`.
- 무시되는 로컬 원본 경로: `build/spinon/benchmark/android-reply-handoff-repeat-20261007/20261007T102125Z/`.

명세의 개발 원인 분석 기본 표본은 조건별 독립 실행 10회, 회당 20초 trace와 10회 탭이다. 이번 행렬은 실행 횟수·trace 길이에는 맞지만 회차당 탭 수는 캡처기의 제한으로 3회다. 따라서 회차당 10회 입력을 요구하는 추후 수집의 대체물이 아니며, 여기서는 30개 응답 표본의 재현 여부만 보고한다.

## 구조 품질

`summarize-android-ui-runtime-attribution.mjs`는 20 trace에서 APK와 source tree digest가 각각 하나인지, 조건 순서가 균형을 이루는지, 회차별 입력 수가 맞는지, FrameTimeline 앱 token이 일치하는지, report와 `reply-handoff` cookie가 1:1로 연결되는지 검사했다.

| 조건 | 회차 | 탭 입력 | runtime dispatch | 보고서 | handoff span | cookie 불일치 | trace/token 품질 실패 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `spinon-event` | 10 | 30 | 30 | 30 | 30 | 0 | 0 |
| `spinon-ui-only` | 10 | 30 | 0 | 0 | 0 | 0 | 0 |

두 조건의 APK SHA-256과 source tree SHA-256은 각각 단일 값이었다. 앱 actual/expected surface token 집합 불일치는 0회였다. 요약기는 입력·counter·보고서 필수 필드·report/thread/cookie 연결 오류를 발견하지 않았다. 이벤트 표식이 없는 UI-only 대조에서 runtime dispatch와 handoff span은 모두 없었다.

## 관측

| 지표 | 표본 | p50 | p95 | 최대 |
| --- | ---: | ---: | ---: | ---: |
| actor 응답 전송→Rust caller 수신 `reply-handoff` wall 구간 | 30 | 48.292µs | 352.541µs | 411.541µs |
| Rust caller 응답 대기 `response_wait_us` | 30 | 347µs | 1,023µs | 1,214µs |
| actor 응답 직전 처리 `actor_before_reply_us` | 30 | 225µs | 617µs | 976µs |
| V8 호출 `v8_call_us` | 30 | 217µs | 613µs | 970µs |
| 탭 제출→Android main 완료 callback | 30 | 16.963ms | 36.026ms | 82.367ms |
| `runOnUiThread` 게시→main callback 시작 | 30 | 16.339ms | 34.920ms | 81.802ms |

기존 단일 63.009ms actor→caller handoff는 이번 30개 응답 표본에서 재현되지 않았다. 이번 최대 handoff는 0.412ms다. 반면 별도 탭 완료 callback 경로에는 82.367ms 표본이 있었고 해당 main-thread 관측 구간 중 79.895ms가 sleeping 상태였다. 이는 해당 에뮬레이터 실행에서 UI callback이 늦게 재개된 상태를 보여 주지만, host scheduling·에뮬레이터 내부·운영체제 중 어느 요소가 원인인지 증명하지 않는다.

`spinon-event`에는 고유 앱 surface token 59개 중 App Deadline Missed token 17개, `spinon-ui-only`에는 58개 중 12개가 있었다. trace는 정적 화면의 프레임 계측 표본이며 각 조건의 기능 동등성·픽셀 표시 지연·상대 성능을 판정할 만큼 설계되지 않았다. 이 수치는 runtime 경로의 원인 귀속에 사용하지 않는다.

## 결론과 남은 범위

- 63ms 응답 handoff 표본은 이번 10회 반복에서 재현되지 않았다. 현재 근거로는 반복 원인으로 확정하지 않는다.
- 30개 응답의 actor 처리·V8 호출·Rust caller handoff는 각각 1ms 미만의 최대값이었다. 이 에뮬레이터 실행에서 그 구간들이 긴 callback tail을 만들었다고 볼 근거는 없다.
- UI 완료 callback의 긴 sleeping 구간은 별도 관측으로 남는다. 원인은 미확정이며 제품 지연이나 Android OS 탓으로 일반화하지 않는다.
- 회차당 탭 수를 10회로 맞춘 반복, Android·iOS 실기기 foreground 및 release 수집, trace on/off 오버헤드, 실제 입력부터 픽셀 표시까지의 시각 검증은 남아 있다. 실기기 검증은 별도 요청 전까지 수행하지 않는다.
- R05는 계속 미완료다.
