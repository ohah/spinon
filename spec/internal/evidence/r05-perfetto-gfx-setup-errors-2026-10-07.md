# R05 · Perfetto `gfx` 설정 오류 원인 분석

## 판정

`ftrace_setup_errors=37`은 상세 trace 설정의 `atrace_categories: "gfx"`가 이 Android Emulator에 존재하지 않는 그래픽·디스플레이 드라이버 ftrace event 37개를 요청해 생겼다. 37개 모두 Perfetto import log의 `Ftrace event unknown`이며 `Ftrace event failed`나 ATrace 오류는 없었다. 따라서 이 값은 generic scheduler tracepoint나 trace-buffer packet loss가 37회 실패했다는 뜻이 아니다.

빠진 event는 DPU 2개, G2D 2개, Mali 1개, MDSS 21개, panel 1개, SDE 10개다. 이들은 특정 SoC의 GPU·display driver 계열 event다. 테스트 대상 Emulator는 호스트 GLES translator 경로이며 이 vendor tracepoint들을 제공하지 않는다. 그러므로 현재 측정에서 RenderThread·EGL·FrameTimeline의 공통 경계는 관측할 수 있지만, 해당 vendor driver 내부를 관측했다고 할 수 없다.

## 최소 차이 대조

Android 16/API 36 ARM64 `sdk_gphone64_arm64`에서 상세 설정을 5초씩 두 번 캡처했다. 두 설정의 유일한 차이는 `atrace_categories: "gfx"` 한 줄이다. Perfetto `stats`와 `_trace_import_logs`를 조회해 설정 오류와 scheduler 자료를 확인했다.

| 설정 | `ftrace_setup_errors` | import log | `sched_slice` 행 | 판정 |
| --- | ---: | --- | ---: | --- |
| `gfx` 제외 | 0 | 설정 오류 없음 | 3,590 | scheduler tracepoint는 기록됨 |
| `gfx` 포함 | 37 | `Ftrace event unknown` 37개, failed/atrace 오류 0개 | 3,495 | `gfx` 요청이 없는 vendor event 목록을 드러냄 |

이 짧은 무부하 쌍은 원인 격리용이다. 프레임 수나 성능을 비교하는 실행으로 해석하지 않는다. `gfx` 제외 대조에서는 `sched_slice`가 기록됐지만, 이 대조만으로 프레임 표식이 양쪽 설정에서 동등하게 수집됐다고 결론내리지 않는다. 별도 원본 40초 상세 실행에서는 `sched_slice` 104,539행, FrameTimeline 300행, `Choreographer#doFrame`·`DrawFrames`·`eglSwapBuffersWithDamageKHR` 표식이 기록됐다. Perfetto packet loss와 버퍼 손실은 0이었다. 따라서 원본 trace는 공통 scheduler·프레임·앱 표식 분석에는 쓸 수 있지만, emulator에 없는 vendor GPU/display event까지 포함한 완전한 계측은 아니다.

## 해석과 남은 범위

- `gfx`를 빼면 설정 오류는 사라지지만 `gfx` ATrace 범주가 제공하는 VSync·RenderThread·EGL 표식도 수집하지 못한다. 상세 진단에서는 `gfx`를 유지하고, 이 AVD에서 생기는 37개 vendor event 누락을 예상된 플랫폼 범위 차이로 분리한다.
- 이 원인은 과거 129.711ms callback의 앞부분에 있던 72.057ms `doFrame` 공백을 설명하지 않는다. 그 공백은 여전히 재현·확정되지 않았다.
- 실제 기기에서는 다른 vendor event 집합이 적용된다. Android 실기기에서 요청·사용 가능한 driver event를 다시 비교해야 하며, Emulator 결과로 하드웨어 GPU 내부 비용을 추정하지 않는다.

## 원본과 도구

- `build/spinon/benchmark/ftrace-setup-isolation-20261007/without-gfx.pftrace`
- `build/spinon/benchmark/ftrace-setup-isolation-20261007/with-gfx.pftrace`
- 두 실행의 설정 원본은 같은 디렉터리의 `without-gfx.textproto`, `with-gfx.textproto`다. 이 원본 trace는 저장소에 커밋하지 않는다.
- [상세 캡처 설정](../../../tools/benchmark/android-frame-attribution-detailed.textproto)
- [Perfetto ATrace 데이터 소스](https://perfetto.dev/docs/data-sources/atrace) · [Trace Processor 통계](https://perfetto.dev/docs/analysis/sql-stats)
