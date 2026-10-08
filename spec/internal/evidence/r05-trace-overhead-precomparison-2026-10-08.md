# R05 · 계측 오버헤드 사전 비교 모델 · 2026-10-08

**상태:** 사전 비교 기준 고정 · 실행 결과는 별도 증거에 기록 · 성능 결론 아님

**계획:** [R05 trace 오버헤드 대조](../../../plan/r05-trace-overhead.md)

**실행 결과:** [Android·iOS 시뮬레이터 계측 오버헤드 진단](r05-trace-overhead-simulators-2026-10-08.md). 사전 표본·제외 기준은 결과를 본 뒤 변경하지 않았다.

## 기준 구현과 환경

| 플랫폼 | 대상 | 계측 켬 | 계측 끔 | 공통 앱 측정값 |
|---|---|---|---|---|
| Android | Android API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터, 동일 debug APK, `MainActivity` `spinon-event` | Android Perfetto session + 동일한 `android.os.Trace` call | Perfetto session 없이 동일 앱·intent·`Trace` call | queue residence, V8 wall time, actor-before-reply, response wait, 성공 dispatch 수 |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator, 동일 debug app, `--spinon-r05-attribution` | 앱 시작부터 `xctrace` `Time Profiler` | 동일 앱 인자 직접 실행 | runtime queue, FFI, V8, main queue, callback→`CADisplayLink` 관측값, 성공 sample 수 |

## 입력 및 표본

- Android: 조건당 10개 독립 실행 × 실행당 10탭; 20초 조건, 5초 안정화. 쌍 안 AB/BA 교대, 고정 seed 기록.
- iOS: 조건당 10개 독립 실행 × 실행당 기존 자동 probe 32 sample; 쌍 안 AB/BA 교대, 고정 seed 기록.
- 실행은 플랫폼 내부에서만 paired 비교한다. Android Perfetto와 iOS Time Profiler는 서로 다른 계측 개입이므로 결과를 합치지 않는다.
- 동일 Mac의 host load를 줄이기 위해 Android와 iOS 행렬을 동시에 실행하지 않는다. 각 run은 앱 프로세스를 새로 시작하는 cold-process 반복이며, 디바이스 재설치나 코드 변경은 pair 사이에 하지 않는다.

## 성공·제외 판정

- 각 실행의 입력 성공·최종 status·프로세스 지속성·로그 완료를 확인한다. Android trace-on에서는 유실과 marker/counter/FrameTimeline 정합성을 확인하고 trace-off에서는 Perfetto 출력이 없음을 확인한다. iOS trace-on에서는 Time Profiler 기록시간이 앱 sample 구간과 교차하는지 `.trace`에서 확인한다.
- Android는 10 dispatch 결과와 `status=0`, iOS는 32/32 `status=0` 및 overlap/rejected 부재를 요구한다.
- 전경 상실, PID 교체, 계측 시작/종료 실패, 입력/표본 불일치, 불완전 trace, 로그 누락은 해당 쌍 전체를 실패로 분류하고 다시 실행한다. 느린 관측값은 제외하지 않는다.
- Android는 각 탭 전 foreground Activity와 PID를 검사하고 양 조건의 좌표를 비교한다. iOS `xctrace`는 launched app PID·probe 시작/완료 로그를 대조하며, trace TOC의 실제 시간창에 32개 sample log가 포함되지 않으면 무효다.
- callback·display tick은 실제 화면 픽셀 표시 시각이 아니다. 이 비교는 계측 부하가 앱 내부 diagnostic metric에 미친 변화만 관찰한다.

## 예상 산출물 및 해석 제한

- raw trace는 로컬 `build/spinon/benchmark/`에 보존하고 Git에는 요약·원본 checksum·실패 사유만 남긴다.
- 10쌍을 채우지 못하면 결론을 유보한다. 백분위 수치를 성능 판정으로 이용하지 않고, paired 원자료·범위·중앙값을 기술통계로만 제시한다.
- 결과가 0 차이로 보여도 계측 비용이 0이라고 증명하지 않는다. simulator/emulator/debug 결과로 실기기·release·renderer 성능을 추론하지 않는다.
- 현재 연결 환경에서 물리 입력, GPU present 및 광학적 input-to-photon은 측정 범위 밖이다.
