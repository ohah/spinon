# R05 · 계측 오버헤드 대조 계획

**상태:** 계획 검토 완료 · Android·iOS 시뮬레이터 진단 완료 · 오버헤드 결론 미확정

**상위 항목:** [R05 상태](../spec/STATUS.md) · [Android 계측 계약](../spec/internal/0022-r05-benchmark-attribution.md)

**구현 전 비교 기준:** [계측 오버헤드 사전 비교 모델](../spec/internal/evidence/r05-trace-overhead-precomparison-2026-10-08.md)

## 목적과 범위

Android·iOS의 기존 진단 화면에서 OS 계측을 켰을 때 앱 내부 측정값이 얼마나 달라지는지 플랫폼별로 조사한다. Android에서는 앱의 기존 `Trace` 표식을 유지하고 Perfetto 캡처를 켜거나 끈다. iOS에서는 기존 자동 R05 probe를 동일하게 실행하고 `Time Profiler` 수집을 켜거나 끈다. 서로 다른 OS 계측 도구의 값을 플랫폼 간 성능 순위로 비교하지 않는다.

이번 결과는 Android API 36 ARM64 에뮬레이터와 iPhone 17 Pro / iOS 26.2 시뮬레이터의 debug 진단에 한정한다. 제품 renderer, release, 실기기, 물리 입력, 실제 픽셀 발광 시간에 대한 결론을 내리지 않는다. 이 작업으로 R05를 완료 처리하지 않는다.

## 고정 실행 방법

### Android 에뮬레이터

- 동일한 APK와 `MainActivity`의 `spinon_frame_attribution=true` 조건을 양쪽 실행에 사용한다. 이 모드는 상태·로그 뷰 갱신을 양쪽에서 똑같이 생략한다.
- `capture-android-frame-attribution.sh spinon-event`에 `--perfetto on|off` 선택지를 추가하고, 별도 `run-android-trace-overhead-matrix.sh`로 짝 실행을 자동화한다. 앱 intent, `Trace` 호출, probe 자체는 양쪽에서 유지한다. 두 조건의 차이는 OS Perfetto trace session 유무다.
- 조건별 독립 실행 10회, 회당 20초, 10회 동일 좌표 탭, 5초 안정화로 고정한다. 순서는 재현 가능한 seed로 정하고 쌍 안에서 AB/BA를 번갈아 배치한다.
- trace-on 캡처 시작 시각을 기준으로 trace-off도 동일한 20초 구간 동안 기다린 뒤 같은 시각표에서 입력을 보낸다. 실행 직전·각 입력 전·직후 대상 Activity와 PID가 유지됐는지 검사하고, 쌍 내 탭 좌표가 일치하는지 확인한다. 각 실행에서 10개 입력, 10개 dispatch 결과, `status=0`, 유실 없는 유효 trace(수집 켬), `gfxinfo` 원본을 확인한다. 하나라도 다르면 그 쌍을 제외 사유와 함께 보존하고 재실행한다.
- 에뮬레이터와 시뮬레이터는 같은 Mac 자원을 쓰므로 플랫폼별 행렬을 동시에 실행하지 않는다. 각 플랫폼 내부의 20개 run은 순서대로 수행한다.
- 같은 앱 보고서의 queue residence, V8 wall time, actor-before-reply, response wait을 trace on/off로 나란히 보되 OS CPU 시간으로 해석하지 않는다. `gfxinfo`와 on 상태의 Perfetto frame은 진단 참고값이며 서로 대체하지 않는다.

### iOS 시뮬레이터

- `run-ios-trace-overhead-matrix.sh`가 동일 시뮬레이터 UDID·동일 설치 app을 지정한다. off는 `simctl launch`; on은 `xctrace record --template 'Time Profiler' --device <UDID> --launch -- <app executable> --spinon-r05-attribution`으로 실행한다. 앱의 32 sample 자동 probe 간격·로깅은 양쪽에서 유지한다.
- `xctrace --launch -- <Simulator app bundle> --spinon-r05-attribution` 경로를 12초 probe로 사전 실행해 실제 app process launch와 32 sample 기록을 확인했다. 원본 Time Profiler TOC와 unified log의 sample 시각이 겹치지 않으면 그 회차는 무효다.
- 조건별 독립 실행 10회, 실행마다 기존 32 sample을 요구한다. 순서는 seed로 재현하고 쌍 안에서 AB/BA를 번갈아 배치한다.
- 32개 모두 `status=0`, overlap/rejected 없음, 완료 표식 존재를 요구한다. 프로세스·앱 로그·빌드 digest를 보존하고, Time Profiler의 실제 기록 범위가 probe와 겹쳤는지 확인한다.
- queue, FFI, V8, main-queue callback, `CADisplayLink`까지의 기존 앱 계측값만 비교한다. Time Profiler sample은 CPU profile이며 thread state 또는 화면 표시 시간으로 해석하지 않는다.

## 판정과 보고

- 플랫폼별 paired run 원자료와 분포를 보존한다. 각 조건 10회와 표본 무결성이 확보되지 않으면 요약 수치나 overhead 결론을 내리지 않는다.
- 원시 관측값, 쌍별 차이, 중앙값 및 범위를 표기한다. 꼬리 표본을 이상치라며 제거하지 않는다. 이 표본 크기로 정식 p95/p99나 플랫폼 우열을 주장하지 않는다.
- Android trace-on 원본은 공식 Perfetto Trace Processor로 packet/data loss·오류 통계·필수 앱 marker/counter·FrameTimeline을 확인한다. 행렬 실행 전에 `SPINON_TRACE_PROCESSOR` 또는 `trace_processor` 실행 경로를 준비한다. 이 도구는 분석용 선택 의존성이며 앱·번들 빌드 의존성으로 추가하지 않는다. 설치·CLI는 [Perfetto Trace Processor 공식 문서](https://perfetto.dev/docs/reference/trace-processor-cli)를 따른다.
- trace on/off 값의 변화가 측정 잡음보다 작아 보이더라도 오버헤드 0이라고 단정하지 않는다. trace-on 효과와 시뮬레이터 변동을 구분하지 못하면 결과를 미확정으로 남긴다.
- Android Perfetto와 iOS Time Profiler는 수집 방식이 다르므로 수치를 합치거나 직접 비교하지 않는다. 이 자료는 진단 오버헤드 관찰이며 앱의 실제 성능 벤치마크가 아니다.
- 원본 `.pftrace`, `.trace`, 앱 로그는 로컬 `build/spinon/benchmark/`에 두고 커밋하지 않는다. 문서에는 기기·OS·빌드·커밋·APK/app digest·입력 순서·요약·실패/제외 이유만 남긴다.
- 계측 행렬은 한 플랫폼씩 실행해 두 에뮬레이터가 Mac 호스트 CPU·메모리를 동시에 압박하는 교란을 막는다.
- R05는 실기기 foreground·입력/화면 검증, 표시 시각과 오차 모델, 동등한 비교 앱 실측 등 남은 조건이 있으므로 계속 미완료로 둔다.

## 실행 명령

```sh
SPINON_TRACE_PROCESSOR=<trace_processor 경로> mise exec -- bun run benchmark:r05-trace-overhead:android --serial emulator-5554 --seed 20261008
mise exec -- bun run tools/benchmark/summarize-android-trace-overhead.mjs <행렬 디렉터리>

mise exec -- bun run benchmark:r05-trace-overhead:ios --udid <부팅된 iPhone 17 Pro Simulator UDID> --seed 20261008
mise exec -- bun run tools/benchmark/summarize-ios-trace-overhead.mjs <행렬 디렉터리>
```

## 계획 검토 기록

아래 관점은 구현 전 프로토콜에 대해 각각 따로 확인한다. 발견한 모순은 실행 전에 계획과 사전 비교 모델에 반영한다.

| # | 적대 관점 | 계획에서 확인할 불변 조건 | 결과 |
|---|---|---|---|
| 1 | 대상 빌드가 조건 사이에 바뀜 | 같은 digest의 debug APK/app만 한 쌍에 사용 | 수정 반영 · matrix 시작 시 고정하고 각 실행 전후 확인 |
| 2 | 앱 자체 계측이 조건 간 달라짐 | Android의 `frameAttributionMode`와 입력 동작을 양쪽에 유지 | 유지 |
| 3 | Perfetto가 켜져도 실제 trace가 없음 | trace 품질·packet loss·marker·FrameTimeline을 확인 | 유지 · trace-on 유효성 조건 명시 |
| 4 | trace off 실행이 trace on보다 짧거나 소스가 회차마다 달라짐 | 양쪽 duration·warmup·탭 수와 행렬 전체 source digest를 고정 | 수정 반영 · off도 trace start 기준 구간을 동일하게 대기하고 각 run digest가 행렬 시작값과 같은지 검사 |
| 5 | 자동화 입력이 실제 handler에 도달하지 않음 | hierarchy 좌표·전경 상태·입력/dispatch counter를 확인 | 수정 반영 · 매 입력 전 foreground/PID 검사 추가 |
| 6 | 프로세스 재시작이 결과를 섞음 | 시작/종료 PID를 기록하고 변경 시 쌍 실패 처리 | 유지 |
| 7 | 앱 상태 변경 비용이 한 조건에만 들어감 | Android 진단 모드가 양쪽 UI mutation을 똑같이 억제 | 유지 |
| 8 | 앱 표시 시간으로 과장 | callback 및 `CADisplayLink`를 실제 present/광학 지연과 구분 | 유지 |
| 9 | 서로 다른 계측기를 플랫폼 간 비교 | 플랫폼별로만 짝 비교하고 OS 간 값은 합치지 않음 | 유지 |
| 10 | Time Profiler 수집 범위가 probe와 어긋남 | `.trace`의 실측 시간 범위와 app sample timestamp를 겹쳐 검증 | 검증 가능 · 사전 12초 수집에서 32 sample/trace 기간 겹침 확인 |
| 11 | 표본 수가 꼬리 통계에 부족 | 10회 paired run 원자료만 요약하고 p95/p99 주장을 제한 | 유지 |
| 12 | 실행 순서/호스트 부하 편향 | 고정 seed, AB/BA 균형 순서와 회차별 장치 상태 보존 | 수정 반영 · Android/iOS 행렬 동시 실행 금지 |
| 13 | 실패 회차를 조용히 버림 | 제외 이유와 원본을 보존하고 재실행 내역을 연결 | 유지 |
| 14 | iOS 자동 selector를 물리 입력으로 오인 | iOS 결과를 자동 probe/selector 진단으로 명시 | 유지 |
| 15 | R05 전체 완료로 오인 | 이 증거만으로 상태 대장을 완료 처리하지 않음 | 유지 |
| 16 | 원본 trace에 민감 정보가 포함 | 원시 trace는 로컬 build 경로에 두고 커밋 금지 | 유지 |
| 17 | `gfxinfo`와 FrameTimeline을 같은 측정으로 오인 | 서로 다른 수집기의 지표·범위를 분리 표기 | 유지 |
| 18 | OS 스케줄링 원인으로 확대 해석 | Time Profiler는 CPU profile, Perfetto는 해당 trace 관측으로 한정 | 유지 |
| 19 | 캐시/첫 실행 편향 | 각 실행 전 동일 안정화와 실행순서 기록, 콜드/웜 의미 명시 | 수정 반영 · 프로세스 cold start를 포함하며 앱 재설치 효과는 제외 |
| 20 | 미연결 기기를 실기기 검증으로 보고 | 현재 시뮬레이터/에뮬레이터 한계를 결과에 고정 | 유지 |

## 실행 결과

2026-10-08 Android API 36 ARM64 에뮬레이터와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 10쌍을 끝냈다. 각 조건 10회 dispatch 또는 32회 앱 표본을 확보했고, 재시도·제외는 없었다. Android Perfetto trace 10개는 packet/data loss·error 통계·ftrace 설정 오류 0, runtime/V8/completion 표식 10개, 입력·dispatch counter 10, FrameTimeline display/surface token 확인을 통과했다. iOS Time Profiler trace 10개는 모두 app PID와 일치하고 32개 sample 전체가 실제 trace 구간 안에 있었다.

쌍별 차이의 범위는 모든 지표에서 0을 가로지른다. 따라서 이 시뮬레이터 표본으로 계측 오버헤드 크기를 확정하지 않는다. 전체 실행 환경·바이너리 digest·표본별 기술통계·원본 checksum은 [실행 증거](../spec/internal/evidence/r05-trace-overhead-simulators-2026-10-08.md)에 기록했다. R05 전체 완료 조건인 실기기 입력·표시 검증, 오차 모델, 제품 비교 대상 실측은 남아 있다.

## 구현 후 적대적 점검

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | 엉뚱한 Android 장치를 비교 | 행렬은 API 36 `sdk_gphone64_arm64`만 허용한다. 실제 실행 metadata도 `emulator-5554`, Android 16/API 36, ARM64로 일치했다. |
| 2 | 엉뚱한 iOS 장치·runtime을 비교 | iPhone 17 Pro / iOS 26.2 / 고정 UDID 조건을 수집기와 집계기가 각각 검사한다. 20개 run 모두 같은 UDID·runtime이었다. |
| 3 | 쌍 사이 앱 바이너리가 바뀜 | APK SHA-256 및 설치 APK hash, iOS app bundle·실행 파일 SHA-256을 고정했다. 전체 pair에서 일치했다. |
| 4 | 조건별 또는 회차별 소스 트리가 달라짐 | 행렬 시작 source digest를 모든 run에 대조하고 pair 동등성도 집계에서 다시 검사한다. Android·iOS 각각 20개 run의 digest가 단일 값이었다. 요약기 부정 대조에서 행렬 digest를 변조했을 때 양쪽 모두 거부했다. |
| 5 | on/off가 앱 실행 방식까지 다름 | Android 양쪽에서 같은 Activity·입력·앱 `Trace` 호출을 유지하고 Perfetto session만 바꾼다. iOS 양쪽에서 같은 자동 probe를 실행하고 Time Profiler 수집만 바꾼다. |
| 6 | trace session이 열리지 않았는데 on으로 셈 | Perfetto service 연결 로그·원본 기록 바이트와 내려받은 파일 크기를 대조한다. iOS는 xctrace 종료 상태·TOC·앱 PID를 확인한다. |
| 7 | Perfetto packet loss나 ftrace 오류를 숨김 | 10개 원본을 Trace Processor v58.2로 읽어 nonzero loss/error/ftrace 통계를 검사했다. 모두 0이며 손실 주입 부정 대조는 거부됐다. |
| 8 | 원본에 앱 실행·FrameTimeline이 없음 | Android runtime/V8/completion marker와 입력·dispatch counter 각 10개, 고유 FrameTimeline token을 요구한다. 10개 trace 모두 통과했다. |
| 9 | Perfetto off에서 실제로 수집함 | off marker와 trace 원본 부재를 집계에서 확인한다. on의 FrameTimeline을 off `gfxinfo`와 같은 지표로 취급하지 않는다. |
| 10 | 백그라운드 Activity나 재시작 프로세스 측정 | Android는 매 입력 직전 foreground Activity/PID와 종료 상태를 확인한다. 모든 dispatch가 성공했고 PID가 유지됐다. |
| 11 | 첫 sequence가 1이 아니면 정상 보고서를 오판 | 실제 노드 ID가 2부터 시작함을 로그에서 확인했다. 연속 번호 비교를 첫 관측값 기준으로 고쳤고, 3입력 Android 스모크와 10개 원본의 연속 sequence를 통과했다. |
| 12 | pair의 입력·APK·report 순번이 다름 | Android 탭 좌표·APK·source digest·dispatch sequence를 쌍 안에서 비교한다. 전체 10쌍에서 일치했다. |
| 13 | 실제 A/B 실행 순서가 기록되지 않음 | 원래 schedule header의 열과 기록 열이 달랐다. `started_utc`로 과거의 5/5 순서를 복원해 근거에 표시하고, 새 수집기는 order/status/path를 TSV에 명시적으로 쓴다. |
| 14 | 실패 실행이 성공 표본처럼 남거나 사라짐 | 행렬은 pair 전체를 재시도하고 실패 시도 경로·이유를 보존한다. 이번 본 행렬에는 실패나 재시도가 없었다. |
| 15 | 중복 pair·회차 누락이 평균에 들어감 | 집계기는 정확히 10개 순차 회차·균형 순서·schedule/pairs 동등성을 요구한다. Android·iOS 중복 회차 주입은 둘 다 실패했다. |
| 16 | Time Profiler가 probe보다 늦게 시작·일찍 종료 | 각 iOS `.trace` TOC의 실제 기록 구간과 앱 sample timestamp 32개 전부를 비교한다. 10개 trace 모두 겹쳤다. |
| 17 | 잘못된 PID·시간·status가 iOS 통계에 포함 | 32개 순번·PID·status·유한/단조 timestamp와 metric 필드를 재검사한다. timestamp 변조 부정 대조를 거부했다. |
| 18 | 중앙값 계산·표본 수·제외가 결과를 왜곡 | 각 run 중앙값 → 쌍별 차이 → 10쌍의 중앙값/범위 순서로 고정하고 이상치를 제외하지 않는다. 10쌍 미만은 정식 요약에서 거부한다. |
| 19 | 진단 수치를 제품·플랫폼 우열로 오인 | 결과 문서에서 debug·시뮬레이터·합성 입력, thread CPU/present/광학 한계, p95/p99 미주장과 플랫폼 간 비비교를 명시했다. |
| 20 | raw trace 공개 또는 상태 체크 과장 | 원시 `.pftrace`·`.trace`는 ignored `build/`에 두고 checksum만 기록했다. 공식 R05 체크는 미완료로 남기고 STATUS·계약·벤치마크 계획·Tailnet 원본을 동기화한다. |

구현 중 확인한 schedule 저장 형식, sequence 검증, trace 품질 확인 누락은 수정한 뒤 영향을 받는 비교·부정 대조를 다시 실행했다. 구현 후에는 표의 20개 실패 관점을 각각 대조했고, 중복 회차·잘못된 sample timestamp·trace data loss·전체 source digest 변조 부정 대조를 요약기에서 거부하는 것도 확인했다. 계획에서 정한 실기기·광학 검증은 이 PR 범위에 포함하지 않는다.
