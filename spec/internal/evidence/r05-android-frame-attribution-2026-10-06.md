# R05 · Android 프레임 지연 귀속 실행 기록

**기록일:** 2026-10-06 · **상태:** 진단 증거 · **제품 성능 판정:** 미완료

## 결론

같은 MainActivity의 UI-only 조건과 실제 V8 event 조건에서 내부 dispatch 단계와 FrameTimeline을 관찰했다. 적대적 검증에서 두 기존 행렬의 schedule이 모두 `spinon-ui-only → spinon-event`만 수행한 사실을 발견했다. 따라서 기존 조건별 프레임 수·deadline miss 차이는 실행 순서와 분리되지 않으며 조건 효과 비교로 해석할 수 없다. 기존 수치는 원시 관측값으로만 보존한다. 후속 균형 순서 재측정은 아래 별도 절에 기록한다.

추가 검증에서 기존 runtime event만 dispatch 완료 문자열을 Android 로그 뷰에 추가하고 자동 스크롤한 점도 확인했다. 이는 JS 실행과 무관한 UI 비용을 조건에 더하므로, 새 진단 모드에서는 양 조건 모두 해당 완료 로그 뷰 변경을 생략하고 native report는 Perfetto·Logcat에 보존하도록 바꿨다. 아래의 균형 재측정은 이 보정된 APK에서 수행한다.

첫 계측 행렬 30 dispatch에서는 V8 owner thread가 약 27ms CPU를 쓴 호출 두 건과 owner thread가 깨어난 뒤 약 27.5ms 실행되지 않은 queue 구간 한 건을 관찰했다. 기존 후속 내부 phase 행렬 50 dispatch에서는 27ms end-to-end 지연 세 건이 관찰됐지만 V8 호출은 최대 0.841ms였다. schedule 순서 결함은 이 개별 trace 안의 계측 사실을 바꾸지는 않지만, 발생 빈도·조건 차이·인과 결론으로 일반화하지 않는다.

따라서 지금 확정할 수 있는 내용은 다음과 같다.

- 두 행렬 모두 lock wait 값은 전부 정수 `0us`로 기록됐다. 이 계측은 1µs 미만을 구분하지 않는다.
- FrameTimeline 원본의 프레임 집계를 다시 검증했다. Perfetto `actual_frame_timeline_slice`는 앱 surface 행과 display 행을 함께 담을 수 있으므로 행 수를 프레임 수로 보지 않는다. 앱 화면은 고유 `surface_frame_token`, display·SurfaceFlinger 항목은 고유 `display_frame_token`으로 집계했다. 두 행렬의 모든 40 trace에서 MainActivity 실제 surface 행·토큰·예상 토큰이 일치했고, App Deadline Missed 행과 고유 앱 토큰도 일치했다. 중복 display 행을 제거해 후속 행렬의 Dropped Frame 집계는 UI-only 4→2, event 4→3, SurfaceFlinger deadline은 UI-only 30→15, event 27→13으로 정정했다. [Perfetto FrameTimeline SQL 표 설명](https://perfetto.dev/docs/data-sources/frametimeline)
- 첫 행렬의 V8 장시간 호출은 뒤이은 50회 dispatch에서는 재현되지 않았지만, 확장한 100회 dispatch에서 V8 handler wall 28.366ms가 1회 재현됐다. 따라서 낮은 빈도의 V8 handler 장시간 호출은 배제할 수 없다.
- 후속 행렬의 27ms 초과 caller 지연 세 건 중 한 건은 queue residence 27.661ms와 V8 owner wake-to-run 27.651ms가 일치했다. 대상 CPU0에서 `RenderThread`가 27.301ms 실행됐다.
- 나머지 두 건은 queue와 V8이 각각 0.2–0.4ms였지만 caller가 응답을 받기까지 27ms를 썼다. Perfetto에서 caller의 wake-to-run이 26.519ms와 26.736ms였다. 첫 건은 wake target과 실제 실행 CPU가 모두 CPU2였으며, 그 사이 CPU2 대부분이 `swapper`(idle)로 기록됐다. 다른 건은 wake target CPU0에서 실제 재개 CPU3으로 이동했다. CPU0의 `HwBinder:475_3`가 runnable 상태로 실행된 시간은 관측됐지만, caller가 CPU3에서 재개했으므로 이를 caller 지연의 원인으로 귀속하지 않는다. guest trace만으로 host virtualizer scheduling과 guest scheduler 원인을 완전히 분리할 수 없다.
- 두 번째 paired matrix App Deadline Missed는 UI-only 28/1145 (2.45%), runtime event 35/1137 (3.08%)였다. 차이가 생겼지만 10회·조건, 10초 debug emulator 표본이고 누락 프레임과 개별 입력의 인과 연결도 없다. 이 결과로 runtime이 악화시킨다거나 무관하다고 판정하지 않는다.
- 이 자료는 emulator trace 파이프라인과 지연 구간을 분류한다. Android 실기기·release·GPU frame path 성능을 입증하지 않는다.

이 결과는 Android 16/API 36 ARM64 에뮬레이터의 debug 진단 빌드다. GPU renderer 성능, 실기기 사용자 체감, release 성능을 말해 주지 않는다.

## 실험 조건과 원본

| 항목 | 값 |
| --- | --- |
| 기기 | Android 16 / API 36 ARM64 emulator (`sdk_gphone64_arm64`) |
| 화면 | 1080×2400, density 420, 60Hz, deadline 16.667ms |
| 조건 | 같은 MainActivity의 `spinon-ui-only` 대 `spinon-event` |
| 회차 | 조건별 10회, 각 20초, 회차별 3탭, 실제 순서는 모든 회차 `spinon-ui-only → spinon-event` |
| 입력 | 두 조건 모두 Spinon 화면 버튼 입력 counter 30회; dispatch counter는 UI-only 0, event 30 |
| 앱 | 한 debug APK, SHA-256 `03ae9e3c062ffee041db455e8938173323b1208133b437a7ad828e69fa29ad4f` |
| 소스 | commit `935fdfd04b42b9aa90847ef2de6d47ea638cc538`, source tree SHA-256 `934585f86ea655c18cd2ad1e74ee57ac3f17e4fc1ea2c7f09abf3437f78471cd` |
| Trace Processor | Perfetto v58.2, `add693d8b338ba9599dbcbc3e300b1ab8c000897` |
| 로컬 원본 폴더 | `build/spinon/benchmark/android-ui-runtime-attribution-post-timing-fix/20261006T045121Z/` |
| 설정 seed | `20502` |

20개 trace 모두 `ftrace_setup_errors=0`, CPU별 ftrace drop 0, Trace Processor data loss 0, discarded chunks 0이었다. 모든 run은 같은 APK·캡처 당시 source tree digest를 기록했고 앱 PID가 캡처 전후 동일했다. 입력·dispatch counter가 기대값과 다른 실행은 없었다. 단, 당시 source tree digest는 실행 시점의 작업 트리 지문이고 APK 빌드 입력의 provenance 증명은 아니다.

추적 설정은 FrameTimeline, `sched_switch`/`sched_wakeup`, CPU frequency/idle과 `view`/`input`/`wm` ATrace를 사용한다. 초기 실험에서 setup error를 낸 `gfx`·`freq` ATrace 범주는 제거했다. 화면·입력·로그·trace 파일은 위 로컬 `build/` 폴더에서 확인할 수 있다. 해당 대용량 원본은 Git에 넣지 않는다.

## 첫 timing-correction 행렬의 화면 프레임 비교

| 조건 | MainActivity 실제 surface frames (고유 token) | App Deadline Missed (고유 app token) | Dropped Frame (고유 display token) | SurfaceFlinger deadline (고유 display token) | main thread D 상태, 회차별 중앙값 |
| --- | ---: | ---: | ---: | ---: | ---: |
| `spinon-ui-only` | 761 | 13 (1.71%) | 2 | 1 | 1.890ms |
| `spinon-event` | 762 | 14 (1.84%) | 0 | 0 | 1.942ms |

회차별 App Deadline Missed 수는 UI-only `1,2,1,1,1,1,1,2,1,2`, runtime event `2,1,1,1,2,1,1,1,2,2`였다. 비율은 실제 제출된 고유 앱 surface frame을 분모로 한 기술 통계다. 10회/조건의 debug emulator 표본이므로 이 차이로 성능 우열을 매기지 않는다. `Prediction Error`와 `Buffer Stuffing`은 앱 deadline miss 수에 더하지 않았다.

## 첫 timing-correction 행렬의 runtime dispatch 단계

`v8_call_us`는 V8 C++ 호출의 wall time이다. CPU 사용량은 `SpinonR05:runtime-worker-call` 구간과 `owner_tid`의 Perfetto thread state를 겹쳐 봤다. 백분위는 30개 action의 nearest-rank 관측값이며, 작은 action 표본은 정식 성능 percentile을 약속하지 않는다.

| 구간 | p50 | p95 | 최대 | 16.667ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| `submission_lock_wait_us` | 0us | 0us | 0us | 0/30 |
| `control_lock_wait_us` | 0us | 0us | 0us | 0/30 |
| `scheduler_lock_wait_us` | 0us | 0us | 0us | 0/30 |
| `queue_residence_us` | 58us | 483us | 27.510ms | 1/30 |
| `v8_call_us` | 94us | 27.202ms | 27.364ms | 2/30 |
| `post_v8_us` | 0us* | 0us* | 0us* | 0/30 |
| `report_build_us` | 3us | 4us | 6us | 0/30 |
| `response_wait_us` | 265us | 27.427ms | 28.936ms | 3/30 |
| `submit_total_us` | 283us | 27.458ms | 28.943ms | 3/30 |

`*` 각 Rust 필드는 정수 microsecond로 잘라 기록한다. `0us`는 측정값이 없다는 뜻이 아니라 1us 미만일 수 있다.

각 앱 process의 첫 runtime input(`seq=2`)과 이후 두 입력은 다음과 같았다.

| 위치 | 호출 수 | queue residence p50/p95/최대 | V8 wall p50/p95/최대 | submit total p50/p95/최대 |
| --- | ---: | --- | --- | --- |
| 첫 입력 | 10 | 64us / 27.510ms / 27.510ms | 295us / 27.364ms / 27.364ms | 706us / 28.943ms / 28.943ms |
| 후속 입력 | 20 | 33us / 417us / 456us | 86us / 133us / 218us | 212us / 605us / 644us |

세 긴 호출은 모두 fresh process의 첫 dispatch였다.

| 실행 | queue residence | V8 wall | caller sleep | owner Running | owner wake→run | wake CPU의 가장 긴 실행 thread | 해석 |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `20261006T045254Z` | 27.510ms | 1.346ms | 28.866ms | 0.494ms | 27.471ms | `HwBinder:475_3` 27.421ms, CPU0, `R+`, priority 104 | scheduler가 owner를 깨운 뒤 guest CPU가 owner를 27ms 넘게 실행하지 못했다. 같은 구간에서 App Deadline Missed 1건 |
| `20261006T045859Z` | 10us | 27.202ms | 27.216ms | 27.147ms | 6us | 해당 없음(관측 6us) | V8 wall의 대부분이 owner thread CPU 실행이며 App Deadline Missed frame과 겹침 |
| `20261006T050101Z` | 64us | 27.364ms | 27.418ms | 27.323ms | 10us | 해당 없음(관측 10us) | V8 wall의 대부분이 owner thread CPU 실행이며 App Deadline Missed frame과 겹침 |

첫 행렬은 C++ trace의 `native-session-ffi`와 후처리 구간까지만 분리했다. 이 trace에는 V8 handler와 문서 callback 세부 표식이 없으므로 해당 두 긴 호출의 내부 원인은 확정하지 않았다. 다음 행렬에서 추가한 내부 trace 계측은 아래 결과에 따로 기록한다.

## 내부 단계 trace를 추가한 후속 행렬

| 항목 | 값 |
| --- | --- |
| 회차 | 조건별 10회, 각 10초, 회차별 5탭; seed `20506` |
| 기기·화면 | 동일 Android 16/API 36 ARM64 emulator, 1080×2400, density 420, 60Hz |
| 앱 | 한 debug APK, SHA-256 `8ac839df3761bd4e0d3e09b62f14f943b213ba96a4380fca90663d9802b44700` |
| 소스 | commit `935fdfd04b42b9aa90847ef2de6d47ea638cc538`, source tree SHA-256 `467b844c49ef503036df2f2b074af1107e9253c1c2efe8e5cf9ee6ac20da7fd8` |
| trace 수·품질 | 20 trace; setup error, CPU ftrace drop, packet loss, discarded chunk 모두 0 |
| 입력 counter | UI-only 50; runtime event 50; dispatch 각각 0·50; counter 불일치 0 |
| 로컬 원본 | `build/spinon/benchmark/android-ui-runtime-phase-split/20261006T053803Z/` |

| 조건 | MainActivity 실제 surface frames (고유 token) | App Deadline Missed (고유 app token) | Dropped Frame (고유 display token) | SurfaceFlinger deadline (고유 display token) | main thread D 상태, 회차별 중앙값 |
| --- | ---: | ---: | ---: | ---: | ---: |
| `spinon-ui-only` | 1145 | 28 (2.45%) | 2 | 15 | 0ms |
| `spinon-event` | 1137 | 35 (3.08%) | 3 | 13 | 1.925ms |

표의 앱 프레임·miss 비율은 고유 `surface_frame_token`, Dropped Frame·SurfaceFlinger 수는 고유 `display_frame_token` 기준이다. 이 앱 프레임 수는 전체 60Hz display tick 수가 아니라 관찰된 MainActivity surface frame 수다. 입력 사이 앱이 새 surface frame을 내지 않은 구간은 이 분모에 포함하지 않는다. 원래 raw 행 합계에서는 한 display token이 여러 layer 행에 반복되어 dropped 수가 4/4, SurfaceFlinger 수가 30/27로 과대 집계됐다. 수정한 토큰 기준으로 회차별 앱 actual·expected token 집합은 20회 중 20회 일치했다. 이전 UI-only→event 고정 순서 행렬의 App Deadline Missed 7건 차이는 조건 비교 근거로 사용하지 않는다. 원인이나 제품 차이로 단정하지 않는다.

50회의 실제 dispatch 보고 값은 다음과 같다. `0us` lock 값은 1µs 미만 대기를 구분하지 않는다.

| 구간 | p50 | p95 | 최대 | 16.667ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| `submission_lock_wait_us` | 0us | 0us | 0us | 0/50 |
| `control_lock_wait_us` | 0us | 0us | 0us | 0/50 |
| `scheduler_lock_wait_us` | 0us | 0us | 0us | 0/50 |
| `queue_residence_us` | 84us | 531us | 27.661ms | 1/50 |
| `v8_call_us` | 101us | 591us | 841us | 0/50 |
| `post_v8_us` | 0us | 0us | 1us | 0/50 |
| `report_build_us` | 3us | 5us | 6us | 0/50 |
| `response_wait_us` | 308us | 27.124ms | 28.129ms | 3/50 |
| `submit_total_us` | 341us | 27.133ms | 28.140ms | 3/50 |

이 행렬의 내부 ATrace 구간 분포다. 값은 실행 구간의 wall duration이며 thread CPU 사용 시간은 별도로 scheduler state와 대조한다.

| 구간 | p50 | p95 | 최대 | 16.667ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| JNI → Rust FFI 전체 | 345.958us | 27.150ms | 28.301ms | 3/50 |
| V8 event handler | 72.792us | 573.917us | 818.292us | 0/50 |
| JS `createNode` → Rust callback | 5.209us | 7.916us | 144.583us | 0/50 |
| 문서 묶음 변환·commit bridge | 26.417us | 143.834us | 416.458us | 0/50 |
| HostDocument Rust commit callback | 8.875us | 26.791us | 74.833us | 0/50 |
| JS `setText` → Rust callback | 2.708us | 4.708us | 8.209us | 0/50 |
| V8 microtask checkpoint | 0.417us | 0.709us | 1.375us | 0/50 |
| HostDocument safe-point 회수 | 8.667us | 68.833us | 268.125us | 0/50 |

`native-session-ffi`는 Java caller가 Rust 동기 응답을 기다리는 전체 wall 구간이므로 CPU 비용으로 읽지 않는다. 세 긴 호출의 분해 결과는 다음과 같다.

| 실행 / 입력 | 전체 | queue | V8 | caller sleep | owner Running | caller wake→run | caller CPU slice | owner wake→run | owner CPU slice |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | ---: | --- |
| `20261006T054029Z` #1 | 28.140ms | 27.661ms | 0.401ms | 28.119ms | 0.419ms | 0.058ms | cpu2 `binder:607_A` 0.046ms | 27.651ms | cpu0 `RenderThread` 27.301ms R+ |
| `20261006T054355Z` #1 | 27.133ms | 0.425ms | 0.170ms | 27.113ms | 0.183ms | 26.519ms | cpu2 `swapper` 26.511ms R (idle) | 0.416ms | cpu1 `binder:682_14` 0.084ms R+ |
| `20261006T054436Z` #1 | 27.484ms | 0.420ms | 0.196ms | 27.336ms | 0.212ms | 26.736ms | cpu0 `HwBinder:475_3` 26.419ms R+ | 0.396ms | cpu2 `ksoftirqd/2` 0.119ms S |

따라서 이 후속 행렬의 세 27ms caller 지연은 V8 handler, HostDocument commit, microtask 또는 safe-point 실행이 아니다. 한 건은 worker owner가 OS scheduler에서 지연됐고, 두 건은 Java caller가 응답 wake 이후 실행되지 못했다. CPU0 idle slice가 보이는 사례도 있어 emulator guest scheduling과 emulator host scheduling을 분리하려면 host-side trace 또는 실기기 재현이 더 필요하다.

## 긴 JavaScript 실행·취소 smoke

계측 변경 후 별도의 10초 emulator smoke를 다시 실행했다. 무한 JS 중 heartbeat 증가량 33, 수동 취소 `status=0`, 평가 종료 `status=-8`, 대기 중 dispatch 세 건 성공, 화면의 `취소 완료`를 확인했다. 세 dispatch의 `queue_residence_us`는 약 718ms, 434ms, 999ms였고 V8 호출은 8~468us였다. 이는 동일 isolate의 긴 JS가 끝날 때까지 JS event queue에서 대기한 시간으로, UI 버튼·heartbeat·수동 취소는 동작했다. 이 smoke는 장시간 JS의 큐 정체와 취소 동작을 확인하며 위의 보통 이벤트 성능 표본에 합치지 않는다.

로컬 smoke 폴더는 `build/spinon/benchmark/android-runtime-metric-smoke/20261006T051412Z-spinon-long-js/`다. 새 보고서 형식으로 출력된 `submission_lock_wait_us`, `queue_residence_us`, `response_wait_us`, `submit_total_us`를 확인했다.

## 해석 한계와 다음 검증

1. 현재 조건은 Android debug emulator뿐이다. 실기기·release·thermal throttling·실제 GPU frame path는 측정하지 않았다.
2. 첫 timing-correction 행렬의 V8 27ms CPU 두 건은 내부 trace marker를 넣은 후속 행렬에서 재현되지 않았다. 해당 두 호출의 JS·V8 내부 원인은 여전히 직접 분해할 수 없다.
3. 후속 행렬의 caller wake-to-run 지연 중 하나는 target CPU가 idle로 기록됐고 다른 하나는 `HwBinder`가 오래 실행됐다. emulator host scheduler 영향과 Android guest scheduler 동작을 완전히 분리할 host trace가 없다.
4. 두 행렬의 FrameTimeline 집계 차이는 짧은 debug emulator 표본이고 입력별 frame token 연결이 없다. 통계적 또는 제품 성능 결론을 내릴 수 없다.
5. 입력부터 픽셀 표시까지의 optical latency, 메모리·CPU 전력, iOS 및 실기기 조건 계측은 없다.

다음 단계는 측정기 양성·음성 대조를 추가해 긴 V8 CPU 호출, owner scheduling 지연, caller wake 지연을 각각 기대한 분류로 검출하는지 확인하는 것이다. 그 뒤 같은 전경·열 조건에서 Android 실기기 release 측정과 iOS 시뮬레이터 계측을 진행한다. 입력→표시 시각 및 공통 기능 화면이 마련되기 전에는 프레임워크 성능 순위를 쓰지 않는다.

## actor 응답 전 타이밍 추가 후 균형 행렬

이전 phase 행렬 중 `20261006T084933Z`의 첫 dispatch는 caller JNI→Rust 구간이 26.923ms였지만 보고된 `queue_residence_us=50`, `v8_call_us=161`, `post_v8_us=0`, `report_build_us=4`만으로 긴 구간을 설명하지 못했다. Perfetto에서는 owner thread가 runtime worker call 안에서 26.838ms Running으로 관측됐다. 그 실행에는 actor 전체 처리시간 필드가 없어 이 구간은 미귀속 이상 사례로 남긴다. 이후 행렬은 같은 조건에서 재현되지 않았으며 원인을 확정하지 않는다.

이 누락을 가리기 위해 Rust actor가 큐에서 명령을 꺼낸 뒤 응답을 보내기 직전까지의 wall 시간을 `actor_before_reply_us`로 기록하고, 보조 보고서 문자열을 마무리하는 시간을 `report_finalize_us`로 분리했다. 요약기는 Android worker thread ID와 Rust report의 `caller_tid`를 연결한 다음 호출별 phase·scheduler 상태를 집계한다. report 출력은 trace가 끝난 뒤 별도 thread에서 수행한다.

| 항목 | 값 |
| --- | --- |
| 행렬 | `build/spinon/benchmark/android-ui-runtime-actor-timing/20261006T091102Z/` |
| 회차·부하 | 조건별 10회, 각 10초, 회차별 5탭; 총 20 trace·50 dispatch |
| 조건 순서 | UI-only→event 5회, event→UI-only 5회; schedule과 실제 캡처 폴더 순서 일치 |
| 기기 | Android 16 / API 36 ARM64 `sdk_gphone64_arm64`, 1080×2400, 420dpi, 고정 60Hz·16.667ms deadline |
| APK SHA-256 | `24b29e7ff51a1c79d7d65b8df9514f3b7696bc570fec4acd353aa8db811c9b63` |
| 캡처 작업 트리 SHA-256 | `036edc2a2fee2ad74cf9b6a3a59d56550d14ac19f608a3aa6fa63ab8cdd48227` |
| 검증 | actual/expected 앱 token 불일치 0, trace 품질 실패 0, 입력·dispatch counter 불일치 0, report/thread 연결 실패 0 |

| 지표 | p50 | p95 | 최대 | 16.67ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| `queue_residence_us` | 114us | 1.463ms | 27.190ms | 1/50 |
| `v8_call_us` | 104us | 515us | 2.014ms | 0/50 |
| `actor_before_reply_us` | 110us | 522us | 2.156ms | 0/50 |
| `response_wait_us` | 290us | 3.876ms | 27.678ms | 2/50 |
| JNI→Rust FFI wall | 410us | 15.093ms | 27.727ms | 2/50 |

긴 worker 호출은 두 건이다. `20261006T091344Z`는 queue 0.180ms, V8 0.159ms, actor 응답 전 0.163ms였지만 Java caller가 응답 wake 뒤 실행될 때까지 27.366ms 기다렸다. wake target과 실제 재개 CPU는 모두 CPU2였고 그 구간에서 `HwBinder:475_3`가 runnable 상태로 27.302ms 겹쳐 관측됐다. `20261006T091530Z`는 queue residence가 27.190ms였고 V8은 0.394ms, actor 응답 전은 0.400ms였다. owner는 wake 뒤 27.181ms 후 실행됐으며 target/실행 CPU는 CPU1, 같은 CPU에서 `HwBinder:475_3` runnable 구간 26.483ms가 관측됐다. 동시 CPU 점유는 지연 원인의 직접 증명으로 보지 않는다. 두 경우 모두 이 행렬에서는 긴 JS/V8 실행이나 actor 응답 전 Rust 처리시간이 16.67ms를 넘지 않았다. 에뮬레이터 guest scheduler와 host virtualizer의 기여는 분리되지 않았다.

| 조건 | 고유 앱 surface frame | App Deadline Missed |
| --- | ---: | ---: |
| UI-only | 1136 | 40 (3.52%) |
| runtime event | 1145 | 26 (2.27%) |

균형 순서의 단일 debug-emulator 행렬에서 runtime event 조건의 miss가 더 적었지만, 이 짧은 표본은 성능 우열이나 무관함을 증명하지 않는다. 입력과 특정 frame token의 인과 연결이 없고 UI-only/runtime event 실행 간 에뮬레이터 상태 변동도 남아 있다. 조건별 수치는 원시 기술 통계로만 둔다.

50 dispatch 행렬에서는 이전의 미귀속 actor 이상이 다시 나오지 않았지만, 아래 100 dispatch 확장 행렬에서 V8 handler 장시간 실행, owner queue/wake 지연, caller 응답 대기 지연이 관측됐다. 이는 과거 `20261006T084933Z`의 동일 실행이 그대로 재현됐다는 뜻은 아니다. 각 원인의 기여를 분리할 추가 계측과 실기기 release 측정, emulator host-side scheduler trace, UI 입력부터 표시까지의 latency, iOS 계측은 여전히 미완료다.

## 확장 균형 행렬 100 dispatch

이전 50회 표본에서 장시간 이상이 재현되지 않아, 같은 APK와 같은 10초·5탭 조건을 조건별 20회로 늘렸다. 실제 schedule은 UI-only→event 10회, event→UI-only 10회로 교대했다.

| 항목 | 값 |
| --- | --- |
| 행렬 | `build/spinon/benchmark/android-ui-runtime-actor-timing-20/20261006T130350Z/` |
| 회차·부하 | 조건별 20회, 회당 10초, 회차별 5탭; 총 40 trace·100 dispatch |
| 기기 | Android 16/API 36 ARM64 `sdk_gphone64_arm64`, 1080×2400, 420dpi, 60Hz |
| APK SHA-256 | `24b29e7ff51a1c79d7d65b8df9514f3b7696bc570fec4acd353aa8db811c9b63` |
| 캡처 작업 트리 SHA-256 | `320e79316f2e35a38b327f1bb7dfec16b4a435da7da5ecba1dadcdc6def33574` |
| 검증 | actual/expected 앱 token 불일치 0, trace 품질 실패 0, 입력·dispatch counter 불일치 0, report/thread 연결 실패 0 |

| 지표 | p50 | p95 | 최대 | 16.67ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| `queue_residence_us` | 69us | 940us | 29.808ms | 4/100 |
| `v8_call_us` | 107us | 509us | 28.366ms | 1/100 |
| `actor_before_reply_us` | 114us | 591us | 28.373ms | 1/100 |
| `response_wait_us` | 281us | 27.221ms | 30.270ms | 6/100 |
| JNI→Rust FFI wall | 328.875us | 27.788ms | 30.324ms | 7/100 |
| Java runtime worker wall | — | — | 30.363ms | 7/100 |

일곱 장시간 호출을 세 구간으로 분류했다.

| 관측 구간 | 실행 | 확인된 내용 | 남은 불확실성 |
| --- | --- | --- | --- |
| 큐 체류 | `130444Z`, `130935Z`, `131147Z`, `131519Z` | 네 호출의 queue residence가 26.558–29.808ms였다. 세 trace에서 owner의 첫 wake→run도 26.551–29.783ms였다. 그 구간에 `HwBinder` 또는 `RenderThread` runnable 상태가 겹쳤지만 원인으로 확정하지 않는다. | `130444Z`는 owner wake→run이 0.388ms로 길지 않았다. enqueue/notify/lock 해제/실제 dequeue 사이 세부 표식이 없어 큐 체류 전체를 OS 스케줄 대기로 단정할 수 없다. guest trace만으로 host virtualizer 기여도 분리되지 않는다. |
| caller 응답 대기 | `130603Z`, `130816Z` | 각각 queue 0.308ms/0.038ms, actor 응답 전 0.591ms/0.188ms였는데 JNI→Rust FFI는 28.971ms/27.997ms였다. `130603Z` caller는 wake 후 28.000ms 실행되지 않았고 `ksoftirqd/1` runnable 구간이 겹쳤다. | `130816Z` caller는 27.970ms를 응답 대기에 썼지만 Perfetto상 `spinon-platform`이 27.486ms Running이었다. Rust의 `Receiver::recv()`는 대기할 때 thread park를 사용하므로 이 CPU 상태와의 불일치는 현재 자료로 설명되지 않는다. 실제 caller CPU stack/sample과 enqueue→recv→reply 구간 marker가 필요하다. |
| V8 handler 실행 | `131001Z` | `v8_call_us=28.366ms`, `actor_before_reply_us=28.373ms`; owner는 해당 worker 구간에서 27.638ms Running이었다. V8 handler marker 안의 Rust node callback·문서 commit bridge·text callback은 각각 0.1ms 미만/0.1ms 수준이었다. | handler 안의 JS/V8 내부 CPU sample, V8 GC/JIT trace가 없어 JS 명령 실행, V8 내부 작업, 코드 생성·회수 중 무엇이 시간을 썼는지는 확정하지 못했다. |

확장 행렬에서 UI-only는 앱 surface frame 2317개 중 App Deadline Missed 44개, runtime event는 2307개 중 38개였다. 이는 debug emulator의 기술 통계이며 입력별 frame 연결이 없으므로 조건 우열이나 제품 영향의 증거로 쓰지 않는다. Trace Processor 요약 결과는 해당 행렬의 `summary.md`에 둔다.

따라서 미확정 병목은 하나의 구간이 아니다. (1) 드문 V8 handler CPU 장시간 실행, (2) actor queue 체류와 owner wake 지연, (3) 빠른 actor 완료 후 caller 응답 경계의 긴 지연이 각각 재현됐다. 지금 가장 구체적으로 남은 미귀속 구간은 `130816Z`의 caller 측이다. 다음 진단은 JNI→Rust 진입부터 enqueue 완료·recv 진입·reply send·recv 복귀까지 각각 marker/timer를 두고, 동일 실행에서 caller와 owner CPU call stack을 표본 수집하는 것이다. V8 handler에는 V8 GC/JIT event와 symbolized CPU profile을 추가한다. 이 자료를 얻기 전에는 Android 에뮬레이터 내부 원인, 런타임 원인, 가상화 호스트 원인을 하나로 지목하지 않는다.

## 명시적 응답 표식 적용 후 균형 행렬 50 dispatch

기존 50 dispatch에서 actor `reply-send`의 ATrace wall 길이를 Rust send 호출 비용으로 읽을 수 없음을 확인했다. actor thread가 표식 안에서 선점되면 CPU를 쓰지 않는 대기 시간도 구간에 포함된다. Rust 계측 guard를 `reply.send` 직후 명시적으로 닫도록 정리했고, 과거 trace는 이 구간의 분포를 요약기에서 제외했다. 새 표식의 단일 smoke에서는 `reply-send`가 3.334µs였다. 다음 균형 행렬은 수정한 APK로 별도 수집했다.

| 항목 | 값 |
| --- | --- |
| 행렬 | `build/spinon/benchmark/r05-reply-fixed-ui-runtime-attribution/20261006T150821Z/` |
| 회차·부하 | 조건별 10회, 회당 10초, 회차별 5탭; 총 20 trace·50 runtime dispatch |
| 조건 순서 | UI-only→event 5회, event→UI-only 5회 |
| 기기 | Android 16 / API 36 ARM64 `sdk_gphone64_arm64`, 1080×2400, 420dpi, 60Hz |
| APK SHA-256 | `278bdd0e3cf2ea4f575cf4cc9594772abfb03e23684a3530f6e4ba4d3cc84619` |
| 캡처 작업 트리 SHA-256 | `3a3af9a0d6a766763f3091eef1a5cb7a09ed8d16b4733aa8d81b31acc9f72667` |
| UI 계측 영향 억제 | 두 조건 모두 탭 상태 TextView와 로그 뷰 변경 생략; 보고서는 Logcat·Perfetto에 보존 |
| 검증 | actual/expected 앱 token 불일치 0, trace 품질 실패 0, 입력·dispatch counter 불일치 0, report/thread 연결 실패 0 |

| 런타임 구간 | p50 | p95 | 최대 | 16.667ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| `queue_residence_us` | 18us | 735us | 977us | 0/50 |
| `v8_call_us` | 102us | 621us | 1.186ms | 0/50 |
| `actor_before_reply_us` | 108us | 626us | 1.192ms | 0/50 |
| `response_wait_us` | 295us | 1.130ms | 1.356ms | 0/50 |
| Java runtime worker 전체 | 356.667us | 1.172ms | 1.411ms | 0/50 |

50개 수정 `reply-send` ATrace 구간의 p50은 13.375µs, p95는 205.041µs, 최대는 27.809ms였다. 이 marker는 wall 범위이고 p50/p95도 send 함수 실행 비용이 아니다. 긴 한 건(`20261006T151428Z` 첫 호출)은 caller FFI가 0.947ms 안에 끝났지만 actor TID 17695가 marker를 닫기 전 27.760ms 동안 `R+` runnable 상태였다. 따라서 이 27.809ms는 caller 응답 지연도 JS 시간도 아니며, actor가 응답한 뒤 CPU에서 재개되지 않은 시간으로 분류한다.

이 새 행렬에서는 Rust/V8 runtime dispatch가 프레임 예산을 넘은 호출이 없었다. 반면 비동기 탭 시작부터 메인 완료 callback까지는 p50 17.000ms, p95 31.512ms, 최대 129.711ms였다. `runOnUiThread` 게시부터 callback 시작까지는 p50 16.416ms, p95 30.417ms, 최대 129.072ms였고 callback 본문은 최대 11.708µs였다. 로그 flush는 두 조건 모두 생략되어 입력 구간과 겹친 로그 UI 작업이 0회였다.

`20261006T151615Z`의 두 번째 탭은 worker 144.458µs, FFI 128µs, V8 76µs였지만 main callback까지 129.711ms가 걸렸다. 앱 main thread는 UI handoff 구간 대부분 동안 잠들어 있었고, 게시 72.057ms 후 시작한 `Choreographer#doFrame`의 `traversal`/`draw-VRI[MainActivity]`가 56.352ms/56.316ms였다. callback 본문은 7µs였다. 이 trace는 지연이 Rust/V8 실행이 아니라 Android View main-thread/frame path에 있었음을 분리한다. 다만 traversal의 내부 어느 View·프레임워크 동작이 56ms를 썼는지, 앞선 72ms 동안 메시지가 대기한 정확한 이유는 이 계측만으로 확인되지 않는다. `Activity.runOnUiThread`의 handler 게시와 `ViewRootImpl.scheduleTraversals`의 synchronization barrier 동작은 이 패턴과 맞지만, 해당 barrier 자체를 trace에 기록하지 않았으므로 구체 메커니즘은 가설로 둔다. [Android 16 Activity 소스](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/java/android/app/Activity.java), [Android 16 ViewRootImpl 소스](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/java/android/view/ViewRootImpl.java)

> **2026-10-07 후속 검증:** sync Handler와 async Handler의 25탭씩 개입 대조로 반복 15–17ms callback 지연에서 sync barrier의 영향을 확인했다. 같은 129ms trace의 56ms render wait는 RenderThread보다 Android Emulator `composer3-service.ranchu`의 Binder 작업이 CPU 3을 먼저 사용한 것과 맞물렸다. 그 앞선 72ms의 다음 `doFrame` 대기는 여전히 미확정이다. 전체 원본·iOS 비교는 [교차 플랫폼 원인 분석](r05-cross-platform-callback-root-cause-2026-10-07.md)을 참고한다.

### 이전 caller 지연 판정 정정

과거 `20261006T145134Z` 첫 호출은 caller가 깨어난 뒤 27.385ms 후 실행됐다는 이유로 caller OS 스케줄 지연으로 분류했었다. 원본 trace를 다시 조회하니 같은 시간 owner TID 13375가 CPU2에서 27.355ms 연속 실행했고, caller TID 13470은 같은 CPU2를 wake target으로 받은 뒤 재개했다. 따라서 이 한 건을 Android scheduler 또는 가상화 host 단독 원인이라고 분류할 수 없다. 실제 V8 handler는 0.149ms였으나 actor의 V8 반환 뒤 구간에서 owner CPU가 사용됐다. 이 trace의 이전 `reply-send` 범위는 wall 구간이라 그 CPU 실행을 특정 Rust 함수 호출에 귀속하지 못한다. 정확한 actor 내부 호출 경계는 미확정이다.

수정 계측 행렬의 `20261006T151428Z` 첫 호출에서도 `reply-send` ATrace wall 구간이 27.809ms로 보였지만, caller FFI는 0.201ms였고 actor thread는 그 구간 대부분을 `R+` runnable 상태로 보내다가 marker를 닫았다. 즉 이 wall 구간에는 응답을 보낸 뒤 actor가 다시 실행될 때까지의 스케줄 대기도 포함될 수 있다. `reply-send` ATrace 길이를 `send()` 실행 비용이나 caller 지연의 원인으로 읽지 않는다. 이 marker는 scheduler 상태와 함께 위치를 확인하는 용도로만 쓴다.

균형 행렬의 UI-only 조건은 앱 surface frame 95개 중 App Deadline Missed 12개, event 조건은 95개 중 19개였다. 짧은 debug emulator 표본이고 실제 입력과 frame token이 연결되지 않아 이 값으로 런타임 조건의 프레임 우열을 주장하지 않는다. 원본 trace와 요약은 위 행렬 폴더에 보관했다.

이 측정으로 원인을 세 범주로 나눌 수 있다. (1) 재현된 27.357ms V8 handler CPU outlier, (2) 별도로 관찰된 owner actor CPU·wake 지연, (3) 주로 다음 Android frame에 맞춰진 main callback 지연과 드문 장시간 Android traversal이다. 이번 고정 계측 50회에서는 (1)·(2) runtime 호출 outlier가 다시 나오지 않았다. 남은 핵심은 actor 내부의 드문 post-V8 CPU 사용을 caller response와 독립적으로 계측하고, Android 실기기/release에서 main-thread frame 지연을 확인하는 것이다. 현재 자료는 emulator 원인을 분류하지만 host virtualizer와 guest scheduler 기여, Android framework와 앱 View traversal 내부의 세부 원인을 확정하지 않는다.
