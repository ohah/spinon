# R05 · iOS·Android callback·프레임 지연 원인 추적

**측정일:** 2026-10-07 · **상태:** 원인 분리 증거 · **R05 완료 판정:** 미완료

## 확인 요약

- Android에서 반복되던 15–17ms `runOnUiThread` handoff 지연은 동기 Handler 메시지가 View traversal의 sync barrier 뒤에 대기하는 경로다. 같은 앱·같은 callback을 `Handler.createAsync`로 전달하는 진단 대조에서는 이후 탭 20개의 p50이 16.125ms에서 75µs로 줄었다. 이는 barrier를 건너뛰는 전달 경로가 해당 반복 지연을 제거한다는 개입 결과다.
- 별도의 129.711ms Android outlier는 Rust/V8 실행이 아니다. 56.154ms의 `postAndWait` 동안 RenderThread가 깨어난 뒤 54.731ms 뒤에야 실행됐다. 그 사이 가상 디스플레이의 Composer HAL 프로세스가 CPU 3에서 50.657ms 실행됐고 SurfaceFlinger가 4.680ms 실행됐다. 즉 이 trace의 긴 프레임 대기는 Android Emulator의 `composer3-service.ranchu`/SurfaceFlinger 표시 경로에서 발생했다.
- 그 129.711ms 중 앞의 72.057ms는 callback 게시 뒤 다음 기록된 `Choreographer#doFrame`까지의 대기다. 앞선 VSync ID와 다음 ID 사이에 7개 차이가 있지만, 이 간격에서 VSync가 지연된 더 아래 원인은 trace로 입증되지 않았다. Composer HAL 작업은 그 뒤 56ms render wait를 설명하지만 앞선 72ms를 설명하지 않는다.
- Composer Binder transaction code 5는 이 Android 16 system image가 사용하는 Composer AIDL V4의 메서드 순서와 대조하면 `IComposerClient.executeCommands`에 해당한다. trace에 Binder interface descriptor는 없으므로 wire-level descriptor를 직접 읽은 결과와는 구분한다.
- iOS Simulator 32/32 dispatch는 성공했다. FFI/V8/main queue callback 구간은 sub-ms였고 main queue p95는 39.417µs다. callback부터 다음 `CADisplayLink` tick까지의 p50은 7.935ms였다. 이는 다음 화면 tick까지의 위상 대기이지 픽셀이 실제 표시된 시각 측정은 아니다.
- 이 자료는 Android 16/API 36 ARM64 에뮬레이터와 iPhone 17 Pro/iOS 26.2 Simulator 결과다. 실기기 성능, 제품 GPU renderer, optical presentation latency, emulator host가 guest delay에 준 영향은 입증하지 않는다.

## Android: 동기 Handler 대 asynchronous Handler

`build/spinon/benchmark/r05-android-handoff-intervention/20261006T161114Z/`에 조건별 5개 독립 실행, 회당 5탭, 총 50개 callback trace를 보관했다. 실행 순서는 두 조건을 교대로 배치했으며, 5회라 시작 순서는 3:2다. 같은 debug APK와 MainActivity를 썼고, frame-attribution 모드에서 상태 TextView·로그 뷰 변경은 두 조건 모두 억제했다. 조건 차이는 completion callback 전달 방식이다.

| 조건·표본 | p50 | p95 | 최대 | 16.667ms 초과 |
| --- | ---: | ---: | ---: | ---: |
| 기본 `runOnUiThread`, 전체 25탭 | 16.255ms | 28.401ms | 32.498ms | 8/25 |
| 기본 경로, 각 실행의 첫 탭 제외 20개 | 16.125ms | 16.933ms | 16.958ms | 3/20 |
| `Handler.createAsync`, 전체 25탭 | 75.125µs | 233.125µs | 27.877ms | 1/25 |
| asynchronous 경로, 각 실행의 첫 탭 제외 20개 | 75.125µs | 199.292µs | 233.125µs | 0/20 |

첫 탭은 앱을 매번 재시작한 뒤 실행한 첫 dispatch다. 정상 경로의 첫 탭은 다섯 실행 모두 27.394–32.498ms였다. asynchronous 조건에도 첫 실행 첫 탭 한 건이 27.877ms였고, Perfetto상 Java runtime worker의 `runtime-main-thread-post` 범위에서 CPU를 사용했다. 이 한 건은 V8 구간 이후에 생긴 별도 worker-side outlier이며, 정확한 Java/ART 함수는 이 trace에 stack sample이 없어 확정하지 않는다. 제외하지 않고 전체 분포에도 포함했다.

진단용 asynchronous Handler는 제품 권고가 아니다. Android Handler 문서상 비동기 메시지는 synchronization barrier를 건너뛰며, 다른 Handler의 동기 메시지와 전역 순서를 보장하지 않는다. callback ordering 계약을 정하기 전에 기본 경로를 교체하지 않는다. [Android 16 `Activity.runOnUiThread`](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/java/android/app/Activity.java), [Android 16 `Handler.createAsync`](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/java/android/os/Handler.java), [Android 16 `ViewRootImpl.scheduleTraversals`](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/java/android/view/ViewRootImpl.java).

## Android: 129.711ms outlier 분해

대상 원본은 `build/spinon/benchmark/r05-reply-fixed-ui-runtime-attribution/20261006T150821Z/runs/20261006T151615Z-spinon-event/frame-attribution.pftrace`다. 앱은 Android 16/API 36 ARM64 `sdk_gphone64_arm64`, 1080×2400, 420dpi, 60Hz debug emulator였다. trace 데이터 유실은 0이다.

| 구간 | 관측 |
| --- | --- |
| 이벤트 → Android main callback | 129.711ms |
| worker / FFI / V8 | 144.458µs / 128µs / 76µs |
| callback post → 실행 | 129.072ms |
| main thread | 대부분 잠든 상태; trace 시각 569798218.791ms의 post 뒤 569798290.848ms의 다음 기록된 `doFrame`까지 72.057ms |
| `draw-VRI[MainActivity]` / `postAndWait` | 56.316ms / 56.154ms |
| RenderThread wake → run | trace 시각 569798291.445ms → 569798346.175ms, 54.731ms |
| CPU 3 표시 서비스 실행 | TID 581 `binder:493_3`가 569798290.838–341.495ms 동안 50.657ms 실행, 이어서 PID 543 `surfaceflinger`가 4.680ms 실행 |
| main callback 본문 | 7µs |

Android 기기에서 trace의 PID 493은 `android.hardware.graphics.composer3-service.ranchu`로 확인했다. PID/TID와 프로세스 이름은 같은 emulator에서 수집했다. ADB 프로세스 원본은 로컬 `build/spinon/benchmark/r05-reply-fixed-ui-runtime-attribution/20261006T150821Z/runs/20261006T151615Z-spinon-event/composer-service-process.txt`에 있으며 저장소에 포함되지 않는다. Binder trace에서는 PID 543 `surfaceflinger`가 trace 시각 569798290.835ms에 PID 493으로 transaction code 5를 보냈고, Composer HAL 응답은 569798341.480ms에 기록됐다. CPU 3에서는 그 transaction이 처리되는 동안 RenderThread의 wake event가 있었지만 실행되지 않았고, SurfaceFlinger가 4.680ms 실행한 뒤에야 RenderThread가 CPU를 받았다. `DrawFrameTask`/`RenderProxy::postAndWait`의 AOSP 구현도 render task를 게시한 caller가 조건 변수에서 완료를 기다리는 구조다. [AOSP HWUI `RenderProxy.cpp`](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/libs/hwui/renderthread/RenderProxy.cpp).

따라서 이 trace의 56ms render wait는 Spinon이나 V8이 아니라 에뮬레이터 Composer HAL·SurfaceFlinger 쪽 CPU 작업에 귀속한다. Android 16 Composer AIDL V4 소스 대조로 transaction code 5를 `IComposerClient.executeCommands`에 매핑했지만, trace 자체에 Binder interface descriptor가 없어 wire-level method를 직접 판독한 것은 아니다. 앞선 72ms VSync 간격의 원인은 여전히 미확정이다. 같은 20개 trace 중 `postAndWait >20ms`는 1개뿐이었다. 이 예외를 일반 Android 실기기 특성으로 일반화하지 않는다.

## iOS Simulator: callback과 display tick

수정된 iOS 진단 화면을 다시 빌드해 실행했다. 원본은 `build/spinon/benchmark/r05-ios-attribution/20261007-final/ios.log`, 화면은 `ios.png`다. Simulator 자동 probe는 버튼 selector를 호출하므로 물리 터치를 재현하지 않으며, 개발용 UIKit 진단 화면이다.

| 구간 | 표본 | p50 | p95 | 최대 |
| --- | ---: | ---: | ---: | ---: |
| runtime worker queue | 32 | 49.875µs | 64.292µs | 83.208µs |
| Rust FFI 전체 | 32 | 265.542µs | 418.625µs | 453.208µs |
| V8 호출 | 32 | 111µs | 242µs | 246µs |
| main queue post → callback | 32 | 29.041µs | 39.417µs | 43.209µs |
| callback → 다음 `CADisplayLink` tick | 32 | 7.935ms | 12.439ms | 15.066ms |
| 입력 action → 다음 display-link tick | 32 | 8.307ms | 12.586ms | 15.575ms |

32회 모두 `status=0`이며 rejected/overlap 로그가 없었다. `CADisplayLink` callback은 frame callback 관측이지 GPU present 완료나 물리 화면 발광 시각이 아니다. 이 결과로 iOS 실기기나 GPU renderer 성능을 주장하지 않는다.

## 결론과 남은 검증

일상적인 Android 15–17ms 대기는 동기 main Handler와 View traversal sync barrier의 상호작용으로 실험에서 확인됐고, callback scheduling을 바꾼 대조군에서 제거됐다. 극단 129ms 사례는 Android Emulator 가상 Composer HAL이 CPU를 오래 사용해 RenderThread를 늦춘 56ms 구간과, 그 이전 72ms의 다음 `doFrame` 대기로 분리됐다. 후속 100회 입력에서는 별도의 113ms Composer VSync 공백과 Composer 서비스의 CPU 실행이 함께 관측됐다. 그러나 원래 72ms 구간에서는 Composer Binder thread와 앱 main thread가 대부분 잠들어 있었고 게스트 CPU도 유휴 상태였으므로, 두 공백을 같은 원인으로 간주할 수 없다. 원래 72ms의 직접 원인과 비동기 조건 첫 탭의 worker-side 27.877ms outlier는 미확정이다. iOS Simulator에서 같은 runtime callback 경계의 지연은 관찰되지 않았다.

R05는 미완료다. 남은 검증은 원래 Android 72ms 공백을 재현하는 유효한 host scheduler trace, 실제 Android/iOS 기기의 동등 계측, iOS display presentation 시각·GPU renderer·계측 오버헤드다. 이번 QEMU Time Profiler는 Android 재현과 겹쳐 CPU stack sample을 남겼지만 OS 스케줄 전환·대기 시간을 기록하지 않는다. macOS System Trace는 여러 기록 방식에서 실제 이벤트 범위가 요청보다 짧게 저장되는 문제가 확인됐다. 현재 결과는 runtime worker/V8 계산을 129ms outlier의 원인으로 지목하지 않으며, emulator 표시 경로의 관측을 Android 전 기기에 적용하지 않는다.

## 2026-10-07 추가 검증: 반복 실행과 UI-only 대조

추가 캡처는 Android 16/API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터, 60Hz, debug APK에서 진행했다. 원본은 아래 `build/spinon/benchmark/` 경로에 보관했다. 계측 trace가 켜진 원인 분석 값이며 release 성능 수치로 사용하지 않는다.

| 실행 | 표본과 계측 결과 | 해석 |
| --- | --- | --- |
| 기본 동기 main handoff, `20261006T213137Z-spinon-event` | 60회. `runtime-main-thread-handoff` p50 16.367ms, p95 17.154ms, 최대 21.052ms, 16.667ms 초과 17/60. 같은 trace에서 post p50 7.500µs, p95 10.083µs, 최대 14µs. | 짧은 게시 비용 뒤에 main thread callback 대기가 반복된다. 앞의 sync-barrier 개입 대조를 보강하지만, 에뮬레이터 한 번의 60회 실행이다. |
| asynchronous Handler, `20261006T212355Z-spinon-event` | 60회. handoff p50 80.250µs, p95 202.167µs, 최대 229.292µs. post 최대 62.834µs, callback 완료 최대 3.875µs. | 과거 첫 dispatch 27.877ms worker outlier는 재현되지 않았다. 이 대조는 제품 callback 순서 계약이나 실기기 지연을 입증하지 않는다. |
| asynchronous Handler + ART/Dalvik 추적, `round-01..05` | 새 프로세스 5개에서 첫 callback을 각 4회, 총 20회. post p50 14.250µs, p95 43.666µs, 최대 54µs. 전체 handoff p50 56.500µs, p95 481.625µs, 최대 1.027ms. post/handoff 모두 2ms 초과 0/20. | 과거 27.877ms는 새 프로세스 첫 callback에서도 재현되지 않았다. 추적된 JIT/GC 이벤트와 그 outlier의 인과관계를 증명할 수는 없다. |
| 동일 화면 `spinon-ui-only`, `20261006T213927Z-spinon-ui-only` | 탭 60회, 고유 MainActivity surface frame 119개, `SpinonR05RuntimeDispatchCount=0`. 두 surface frame token(1423557, 1423564)이 각각 App Deadline Missed와 Late Present로 기록됐다. 첫 프레임 `DrawFrames`는 29.019ms였고 RenderThread가 그 구간에서 CPU에 실제 실행된 시간은 28.705ms였다. 같은 프레임의 앱 main thread `draw-VRI`는 0.209ms, `postAndWait`는 0.107ms였다. | 적어도 이 첫 frame miss는 JS/Rust runtime dispatch 없이 재현됐다. 지연 실행은 앱 main thread보다 Android 에뮬레이터의 RenderThread 그리기 구간에 있었다. 이 결과는 원래 129.711ms 사례의 56ms wait와 같은 원인이라는 증거는 아니다. |

UI-only 캡처에서 첫 두 지연 프레임은 각각 `Prediction Error, App Deadline Missed`와 `Prediction Error, App Deadline Missed, Buffer Stuffing`으로 분류됐다. 같은 에뮬레이터의 `dumpsys SurfaceFlinger`는 `Android Emulator OpenGL ES Translator (Apple M4 Max)`, OpenGL ES 3.0 및 `vulkan_renderengine=false`를 보고했다. 따라서 이 View 대조의 그래픽 경로는 Spinon 제품 `wgpu` renderer가 아니라 에뮬레이터의 GLES translation 경로다. 이 trace는 RenderThread가 실행 중이었다는 사실을 알려주지만, 그 28.705ms를 소비한 구체 HWUI/Skia 함수나 GLES translation/driver 단계는 기록하지 않았다. CPU call-stack 측정 수단이 없어 함수 단위 귀속은 미확정이다.

## 2026-10-07 추가 검증: ATrace gfx와 EGL swap 경계

`tools/benchmark/android-frame-attribution-detailed.textproto`에 `gfx` ATrace와 `android.surfaceflinger.frame` data source를 추가해 Android 16/API 36 ARM64 에뮬레이터, 60Hz, debug APK에서 각각 60탭을 수집했다. 수집 스크립트는 `SPINON_ANDROID_TRACE_CONFIG`로 별도 설정을 선택하도록 했다. 설정 원본은 [저장소 파일](https://github.com/ohah/spinon/blob/main/tools/benchmark/android-frame-attribution-detailed.textproto)이고, 측정 trace는 아래 `build/spinon/benchmark/` 경로에 있다. 이 설정은 release·실기기 성능용이 아닌 원인 분석용이다.

| 실행 | 관측값 | 판정 범위 |
| --- | --- | --- |
| 실제 runtime event 60회, `20261006T225859Z-spinon-event` | `SpinonR05RuntimeDispatchCount=60`. MainActivity token 1435282의 App Deadline Missed 프레임은 32.242ms, `DrawFrames`는 32.039ms, `eglSwapBuffersWithDamageKHR`는 31.402ms였다. 같은 RenderThread의 해당 `DrawFrames` 구간 sched CPU 실행 합은 31.521ms였다. `v8-handler-call`은 0.522ms였고, 별도 `spinon-platform` worker의 runtime reply 대기는 29.018ms였다. | 이 프레임에서 RenderThread가 EGL swap 경계 안에서 CPU를 오래 사용한 사실을 확인했다. Runtime 응답 대기는 다른 thread의 구간이므로 그 자체를 프레임 miss의 원인으로 단정하지 않는다. EGL 내부의 개별 함수나 실제 GPU 실행 시간은 여기서 분해되지 않는다. |
| runtime dispatch 없는 UI-only 60회, `20261006T230512Z-spinon-ui-only` | `SpinonR05RuntimeDispatchCount=0`. App Deadline Missed token 1440193은 32.307ms였고 대응 RenderThread `DrawFrames`는 33.173ms, `eglSwapBuffersWithDamageKHR`는 31.556ms였다. 전체 122개 EGL swap 중 3개가 16.667ms를 넘었고 최대는 31.872ms였다. 추가 Buffer Stuffing frame도 1개 있었다. | 적어도 이 에뮬레이터에서는 runtime dispatch 없이도 같은 GLES View/RenderThread/EGL 경로에서 지연 프레임이 발생했다. 이는 Spinon runtime이 원인인 모든 frame을 뜻하지 않으며 제품 `wgpu` renderer의 측정도 아니다. |
| `gfx` VSync 표식 | runtime event trace에서 `onComposerHalVsync`가 2,387회 관측됐고 30ms 초과 간격은 34.071ms와 46.429ms였다. 34.071ms 간격은 token 1435282의 `Choreographer#doFrame` resync(32.6ms)와 겹쳤다. UI-only trace의 최장 간격은 47.479ms였다. 어느 trace에서도 과거 72.057ms 구간은 재현되지 않았다. | VSync/Choreographer 간격은 더 측정할 수 있으며, 이번에 관측한 공백은 원래 129.711ms 사례의 앞선 72.057ms 공백을 설명하지 않는다. |

원본 상세 trace는 `onComposerHalVsync`, `Choreographer#doFrame`, `DrawFrames`, `eglSwapBuffersWithDamageKHR`, `GPU completion fence` 표식을 실제로 기록했고 packet loss는 0이었다. 후속 최소차이 대조에서 `gfx` ATrace 범주가 `ftrace_setup_errors` 37건 전부를 유발했고, 오류 내용은 AVD에 없는 DPU/G2D/Mali/MDSS/panel/SDE vendor driver event의 `Ftrace event unknown`이었다. `gfx` 없는 대조에서는 설정 오류가 0건이고 scheduler slice가 기록됐다. 별도 원본 상세 trace에는 FrameTimeline과 앱 표식도 있었지만, 짧은 무부하 대조는 프레임 표식의 설정 간 동등성을 검증하지 않았다. 이 결과는 generic trace buffer 손실을 뜻하지 않지만 vendor GPU/display driver 세부 event는 수집되지 않았다는 한계를 남긴다. 상세 결과는 [Perfetto `gfx` 설정 오류 분석](r05-perfetto-gfx-setup-errors-2026-10-07.md)을 참고한다. `simpleperf record`는 `sched:sched_switch`와 `task-clock` event를 지원하지 않는다고 반환했다. Perfetto `linux.perf` stack-profile 시도도 stack sample 0개였고, 별도 ftrace scheduler tracepoint는 동작했다. 이 AVD에서 native call-stack profile을 얻지 못한 것이므로 Android 전반에서 측정 불가능하다는 결론은 아니다.

같은 Android 16 Composer AIDL V4의 frozen API에서 `IComposerClient`의 첫 다섯 메서드는 `createLayer`, `createVirtualDisplay`, `destroyLayer`, `destroyVirtualDisplay`, `executeCommands` 순서다. 서비스의 `IComposer`는 `createClient`, `getCapabilities` 두 메서드뿐이다. 따라서 PID 493의 Composer service를 향한 Binder transaction code 5는 V4 AIDL 순서상 client의 `executeCommands`로 매핑된다. 비교한 primary source는 [Android 16 V4 `IComposerClient.aidl`](https://android.googlesource.com/platform/hardware/interfaces/+/refs/tags/android-16.0.0_r1/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/4/android/hardware/graphics/composer3/IComposerClient.aidl), [Android 16 V4 `IComposer.aidl`](https://android.googlesource.com/platform/hardware/interfaces/+/refs/tags/android-16.0.0_r1/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/4/android/hardware/graphics/composer3/IComposer.aidl)다. Binder trace에 target node의 interface descriptor가 없어서 이 mapping은 빌드된 service·version·transaction 번호를 결합한 소스 대조이며 payload 직접 decode는 아니다.

## iOS Instruments 추적 시도 판정

`build/spinon/benchmark/r05-ios-instruments/20261007-system-trace/`의 System Trace는 CLI가 20초 기록 완료를 보고했지만, trace TOC의 시간 범위는 2026-10-07 06:37:23.281–23.454 KST(0.173초)였다. 앱 probe 로그는 06:37:26.685에 시작해 06:37:33.619에 끝났다. 따라서 이 trace는 32개 probe와 겹치지 않아 iOS thread/frame 원인 분석 자료로 사용할 수 없다. `xctrace export --toc`도 `getpwuid_r did not find a match for uid 501` 오류로 실패했다. 앞서 확인한 `Animation Hitches` template의 Simulator 미지원과 Time Profiler의 V8 DWARF symbolization 경고·계측 시간 변화도 그대로 한계로 둔다.

## 2026-10-07 추가 검증: 100회 입력과 macOS `xctrace`

Android 16/API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터, 60Hz debug APK에서 100회 입력을 수집하는 동안 Mac의 QEMU PID 23588에 Time Profiler를 붙였다. Android trace와 host 기록은 별도 원본이며, 에뮬레이터 결과는 실기기 성능 주장이 아니다.

| 측정 | 관측 | 판정 범위 |
| --- | --- | --- |
| Android runtime callback 100회 | 성공 100/100. `runtime-main-thread-handoff` p50 16.449ms, p95 17.460ms, 최대 30.480ms; 16.667ms 초과 31/100, 30ms 초과 1/100, 50ms·72ms 초과 0/100. `v8_call_us` 최대 920µs. | 원래 72ms handoff는 재현되지 않았고, 이 100회에서 V8 실행이 긴 대기 원인이라는 증거도 없다. 최대 30.480ms는 별도 단일 표본이다. |
| Android Composer VSync 공백 | `onComposerHalVsync` 3,574회에서 30ms 초과 공백 9개, 최대 113.368ms. 최대 공백은 어떤 runtime handoff와도 겹치지 않았다. 해당 구간에서 PID 493 `android.hardware.graphics.composer3-service.ranchu`의 TID 581 `binder:493_3`가 CPU 0에서 112.813ms 실행됐고, 나머지 세 게스트 CPU는 각각 110ms 이상 유휴였다. | 이 별도 공백은 에뮬레이터 Composer 서비스의 Binder thread가 CPU를 점유하는 구간과 맞물린다. Composer 내부 함수와 host OS 스케줄 지연의 기여는 이 trace만으로 나뉘지 않으며, 원래 72ms 공백의 원인으로 소급하지 않는다. |
| 원래 129.711ms trace의 72.057ms 구간 재검산 | 앱 main TID 18386은 `S` 상태로 72.164ms 대기했다. Composer TID 581도 공백 중 대부분 잠들어 있었고 끝부분에만 0.074ms 실행됐다. 네 게스트 CPU는 69.321–71.343ms씩 `swapper` 상태였다. Composer thread의 50.657ms 실행은 다음 `doFrame` 뒤에 시작됐다. | 이 원본 구간은 게스트 CPU 과부하나 Composer thread가 72ms 내내 CPU를 쓴 상황이 아니다. VSync 공급 지연인지 host QEMU scheduling인지 아직 확정하지 못했다. |

### `xctrace` 기록 시간 유효성

Xcode 선택 경로는 `/Applications/Xcode.app/Contents/Developer`, `xcrun xctrace`도 같은 Xcode 아래에 있다. `xcodebuild`는 Xcode 26.2, `xctrace`와 Instruments TOC는 26.0으로 표시되지만 build ID는 모두 `17C52`다. Android 실행 중 QEMU PID 23588은 기록 전후 계속 살아 있었다. 따라서 혼합 Xcode 설치나 사라진 attach 대상은 관측된 짧은 System Trace 범위를 설명하지 않는다.

| 기록 방식 | 요청·실행 방식 | `.trace`에 기록된 실제 데이터 길이 |
| --- | --- | ---: |
| System Trace, attempt-01 | 45초 요청, 기본 window | 0.223525초 |
| System Trace, host-probe | 10초 요청, 전체 프로세스, 10초 window | 0.462900초 |
| System Trace, QEMU attach | 10초 요청, 기본 window | 0.254464초 |
| System Trace, QEMU attach | 20초 요청, 10초 window | 0.970526초 |
| System Trace, attempt-04 | 12초 요청, 12초 window | 0.568739초 |
| System Trace, attempt-05 | 제한 시간 없이 20초 기록 후 SIGINT, 30초 window | 1.532561초 |
| System Trace, attempt-06 | 12초 요청, QEMU attach, `--window` 미지정 | 0.213520초 |
| Time Profiler 대조 | QEMU attach, 75초 요청 | 75.813310초 |

attempt-04의 System Trace metadata는 `rawStartTime`–`rawEndTime` 12.690445초, `_rawDuration` 0.568739초를 기록했다. 내보낸 context-switch 행도 0.003916–0.568682초에만 있었다. 제한 시간 옵션을 쓰지 않고 수동 중단한 attempt-05도 raw 실행 간격 18.234015초에 비해 `_rawDuration`은 1.532561초였고, context-switch 표 범위도 0.004887–1.532146초였다. attempt-04는 xctrace가 종료 코드 0으로 완료됐지만 실제 trace는 0.569초뿐이었다. attempt-05의 xctrace도 저장 완료를 출력했으나, 실행 wrapper가 종료 코드 변수에 zsh 예약 변수 `status`를 사용해 마지막 상태 기록에 실패했고 셸은 1로 끝났다. 따라서 수동 중단 비교는 xctrace 로그·저장된 trace metadata·context-switch 표로만 판정했다. 추가 attempt-06은 `--window`를 지정하지 않았지만 TOC의 실제 기록 모드는 `Windowed (5 seconds)`였고, 12초 요청 중 저장된 데이터는 0.213520초였다. 설치된 `xctrace record` 도움말에는 녹화 모드 옵션이 없었다. 반면 75초 Time Profiler는 raw metadata와 표본 시각 모두 약 75.8초였다. 따라서 명시적 `--window` 값만으로는 현상을 설명할 수 없고, 짧은 데이터 범위는 export 이후가 아니라 `.trace`에 이미 저장돼 있다.

설치된 `System Trace` 템플릿은 attempt-06의 TOC에서 기본 `Windowed (5 seconds)`로 표시됐다. Apple도 System Trace가 기본적으로 윈도 모드에서 최근 수 초만 보존한다고 설명한다([WWDC16 System Trace 심층 설명](https://developer.apple.com/videos/play/wwdc2016/411/)). 따라서 과거의 요청 길이와 실제 기록 길이 차이에는 이 템플릿의 윈도 모드 설정이 섞여 있다. 다만 12초 요청에 기본 5초 모드인데도 실제 데이터가 0.214초뿐인 현상은 윈도 크기만으로 설명되지 않는다. System Trace의 `RunIssues.storedata`에는 `Data stream: Time Mapping`이 남아 있다. 동일 메시지가 유효한 75초 Time Profiler에도 있어 이 문구 하나만으로 System Trace 실패의 내부 원인을 확정할 수는 없다. unified log에서 권한 거부나 명시적 수집 실패는 찾지 못했다. 현재 확인된 것은 **이 Mac의 Xcode 26.2/macOS 26.5.1에서 System Trace가 기본 윈도 모드였고, 요청한 범위보다 짧은 데이터가 `.trace`에 저장된 것**이다. 설치된 CLI 도움말에는 녹화 모드를 직접 바꾸는 옵션이 없으므로 GUI Instruments의 기록 설정 또는 다른 Xcode/macOS 조합에서 deferred 모드와 동일 QEMU 재현을 비교해야 한다. Xcode와 macOS 중 어느 쪽 내부 결함인지, 짧은 데이터가 time mapping 문제인지 다른 System Trace 수집 경로 문제인지는 입증되지 않았다.

## 아직 측정하지 못했거나 원인을 확정하지 못한 항목

| 항목 | 지금까지 확인 | 측정 가능성과 남은 절차 |
| --- | --- | --- |
| 원래 129.711ms 사례 앞부분의 72.057ms 다음 `doFrame` 대기 원인 | 원본 trace에서 앱 main thread는 72.164ms, Composer Binder thread는 해당 구간 대부분 잠들어 있었고, 네 게스트 CPU는 각각 69.321–71.343ms 유휴였다. Composer thread의 50.657ms 실행은 다음 `doFrame` 이후다. 100회 재현에서 발견한 별도 113.368ms Composer 공백은 그 원본과 다른 시각·스레드 상태였다. | 원래 공백은 게스트 CPU 과부하나 Composer CPU 점유로 설명되지 않는다. 유효한 host scheduler trace와 동일 outlier 재현으로 VSync 공급 경로와 host QEMU 기여를 나눠야 한다. System Trace 수집 길이 문제는 아래 xctrace 판정처럼 아직 해결되지 않았다. |
| 과거 asynchronous 첫 callback의 27.877ms worker-side 지연 | 60회 반복과 새 프로세스 첫 callback 20회에서 재현되지 않았다. `simpleperf`는 `cpu-cycles`, `cpu-clock`, `task-clock`, `sched:sched_switch` 측정 중 일부를 이 emulator에서 지원하지 않는다고 반환했고, Perfetto CPU stack trial은 표본 0개였다. | 측정 불가능하다고 볼 수 없다. 같은 첫 dispatch 반복 중 재현을 기다리며 ART method trace를 붙이거나 CPU profiling이 되는 기기에서 기록한다. 재현 전에는 Java/ART·JIT·GC 원인을 추정하지 않는다. |
| RenderThread EGL swap 내부의 native 함수·driver/GPU 기여 | `gfx` trace에서 지연 frame의 `eglSwapBuffersWithDamageKHR` 경계와 sched CPU 점유를 관측했다. runtime dispatch 없는 UI-only에서도 EGL swap 31.556ms와 App Deadline Missed 32.307ms가 함께 있었다. 이 emulator의 CPU stack profiler는 표본을 내지 않았다. | EGL 경계는 측정됐다. 함수 내부 귀속은 native sampling이 가능한 Android debug 기기나 별도의 그래픽 driver/GPU trace가 필요하다. 현재 증거는 Spinon 제품 `wgpu` 성능을 말하지 않는다. |
| Composer Binder transaction code 5의 method | Android 16 system image는 Composer AIDL V4이고, V4 메서드 순서상 code 5는 `IComposerClient.executeCommands`다. `IComposer` control interface는 두 메서드만 둔다. | source mapping은 됐다. Binder trace에 target object의 interface descriptor가 없으므로 raw transaction node/descriptor를 직접 확인한 것은 아니다. method 명칭 자체는 더 이상 미측정 항목으로 두지 않는다. |
| 상세 Perfetto trace의 누락 범위 | 최소차이 대조로 `gfx` 범주가 37개 `Ftrace event unknown`을 유발함을 확인했다. 모두 AVD에 없는 vendor driver event다. 짧은 `gfx` 제외 대조에서는 scheduler slice가 기록됐으나 프레임 표식 동등성은 확인하지 않았다. | Emulator 결과는 관측된 generic frame/scheduler 구간에 한정한다. 실기기에서는 해당 장치에서 사용 가능한 GPU/display event를 별도로 확인하고, vendor driver 내부가 계측됐다고 과장하지 않는다. |
| Android 지연에서 host QEMU scheduling이 차지한 정확한 시간 | 새 75초 Time Profiler는 100회 Android 재현과 겹치는 QEMU CPU stack sample을 남겼고 113ms Composer 공백 주변에서도 QEMU 표본이 있다. 그러나 CPU sampling은 OS context switch·off-CPU 대기 시각을 기록하지 않는다. 새 System Trace도 요청 12초 중 실제 데이터는 0.569초만 남았다. | host CPU stack 활동은 관찰했지만 host scheduler의 정확한 기여 시간은 여전히 미측정이다. GUI Instruments 또는 다른 Xcode/macOS 환경에서 full-range System Trace를 확보한 뒤, guest/host clock marker와 같은 재현을 겹쳐야 한다. |
| macOS System Trace의 짧은 실제 수집 범위 | 전체 프로세스·PID attach, 명시 window·기본 window, time-limit 자동 종료·수동 중단을 바꿔도 `.trace`의 실제 데이터 범위는 0.214–1.533초였다. attempt-06 TOC는 기본 `Windowed (5 seconds)` 모드였다. QEMU 대상 Time Profiler는 75.813초가 기록됐다. System Trace RunIssues에는 `Data stream: Time Mapping`이 있다. | 알려진 템플릿 기본 설정은 윈도 모드지만 5초보다 짧은 저장 원인은 설명하지 못한다. GUI Instruments에서 deferred 모드를 명시해 재시도하고, 필요하면 별도 Xcode/macOS 조합에서 같은 대조를 해야 한다. Xcode 내부와 macOS 중 어느 구성요소가 영향을 주는지는 미확정이다. |
| iOS Simulator의 OS thread와 frame 원인 | 기존 System Trace는 32개 callback log와 시간상 겹치지 않았다. Animation Hitches template은 이 Simulator에서 지원되지 않았고 Time Profiler는 계측 시점을 바꿨다. | 측정 자체는 가능하지만 해당 capture는 다시 해야 한다. 실행 중 앱 process에 attach하고 trace window가 app marker를 덮는지 확인한다. 같은 빌드로 profiler on/off 대조 전에는 지연 수치를 비교하지 않는다. |
| 실제 입력부터 화면 픽셀 발광까지의 지연 | Android 입력은 ADB 주입이고 iOS 입력은 selector 호출이다. `CADisplayLink`와 FrameTimeline은 패널 발광 시각이 아니다. | 현재 Simulator/Emulator만으로는 실제 입력·광학 지연을 측정할 수 없다. 실기기 물리 입력과 고속 카메라 또는 photodiode, 측정 오차 및 반복 분포가 필요하다. |
| Spinon 제품 GPU renderer의 frame 비용 | 이번 지연 frame은 Android View와 Emulator GLES translation 경로다. `wgpu` 제품 renderer의 scene build·GPU 제출·present를 측정하지 않았다. | 측정 가능하지만 같은 JS 이벤트 시나리오를 제품 GPU surface에 연결해야 한다. 이어서 Android/iOS release 실기기에서 GPU frame trace와 실제 화면을 수집한다. |
| 계측이 callback/frame 지연에 더한 오버헤드 | 같은 조건의 tracing on/off 대조를 하지 않았다. | 측정 가능하다. 같은 빌드·기기·입력 순서를 교대해 켜짐/꺼짐 각각 분포를 수집한다. |
| 실기기·release·다른 refresh rate 재현성 | 현재 연결된 장치는 Android API 36 emulator와 iPhone 17 Pro iOS 26.2 Simulator뿐이다. | 지금은 이 환경에서 측정할 수 없다. Android/iOS 실기기와 release 빌드, 고정 주사율 조건이 필요하며 기존 simulator 결과와 합산하지 않는다. |

### 추가 원본

- Android 100회 입력 + QEMU Time Profiler 75초 동시 기록: `build/spinon/benchmark/r05-host-guest-sync-20261007/attempt-03/` (Android `frame-attribution.pftrace`, `input-events.txt`, `clock-markers.txt`, QEMU `.trace`, `xctrace.log`)
- QEMU System Trace 시간 제한 12초·window 12초: `build/spinon/benchmark/r05-host-guest-sync-20261007/attempt-04/`
- QEMU System Trace 20초 수동 중단·window 30초: `build/spinon/benchmark/r05-host-guest-sync-20261007/attempt-05/`
- QEMU System Trace 시간 제한 12초·기본 window: `build/spinon/benchmark/r05-host-guest-sync-20261007/attempt-06/`
- System Trace 짧은 범위 대조 45초·10초·20초 및 5초 Time Profiler 양성 대조: `build/spinon/benchmark/r05-host-guest-sync-20261007/attempt-01/`, `host-probe/`, `host-attach-probe/`, `xctrace-controls/`

- Android 동기 handoff 60회: `build/spinon/benchmark/r05-android-host-scheduling-20261007/20261006T213137Z-spinon-event/`
- Android asynchronous Handler 60회: `build/spinon/benchmark/r05-async-handler-simpleperf-20261007/20261006T212355Z-spinon-event/`
- Android cold-process Dalvik/JIT/GC 20회: `build/spinon/benchmark/r05-android-dalvik-attribution-20261007/round-01/` ~ `round-05/`
- Android runtime 없는 UI-only 60회: `build/spinon/benchmark/r05-android-ui-only-control-20261007/20261006T213927Z-spinon-ui-only/`
- Android `gfx`·SurfaceFlinger 상세 runtime event 60회: `build/spinon/benchmark/r05-detailed-gfx-vsync-20261007/20261006T225859Z-spinon-event/`
- Android `gfx`·SurfaceFlinger 상세 UI-only 60회: `build/spinon/benchmark/r05-detailed-gfx-vsync-20261007/20261006T230512Z-spinon-ui-only/`
- Android CPU stack profile 기능 확인: `build/spinon/benchmark/diagnostic-capabilities/` (에뮬레이터에서 Perfetto `linux.perf` sample 0개와 `simpleperf` unsupported 오류 기록)
- Android 상세 trace 설정: [저장소 파일](https://github.com/ohah/spinon/blob/main/tools/benchmark/android-frame-attribution-detailed.textproto); 기본 capture script에서 `SPINON_ANDROID_TRACE_CONFIG`로 선택한다.
- Android emulator host sample: `build/spinon/benchmark/r05-android-host-scheduling-20261007/emulator-host.sample.txt` (host와 guest trace의 정밀 시간 동기 자료는 아님)
- iOS probe 로그: `build/spinon/benchmark/r05-ios-instruments/20261007-system-trace/ios.log`. Instruments 원시 bundle은 OS trace가 무효이고 프로세스 metadata가 포함될 수 있어 공유 근거에 넣지 않는다.

따라서 미측정 항목이 전부 측정 불가능한 것은 아니다. 상세 trace로 VSync 표식과 EGL swap 경계, runtime 없는 UI frame miss까지 더 측정했고 Composer method도 AIDL V4 순서로 매핑했다. 100회 입력에서는 별도의 긴 Composer 실행과 VSync 공백을 잡았지만 기존 72ms 공백의 원인과 동일하다고 증명하지 못했다. QEMU Time Profiler는 CPU 실행 표본만 제공했고, System Trace는 유효한 시간 범위를 저장하지 못해 host scheduling 기여를 확정하지 못했다. 실제 input-to-photon은 현재 장비로 잴 수 없어 실기기와 광학 센서가 필요하다. 원래 129ms 사례의 72ms 원인·RenderThread 내부 함수·host scheduler의 정확한 기여·iOS OS scheduling·제품 GPU renderer·계측 오버헤드는 아직 확정되지 않았으며 R05 완료 조건은 충족되지 않았다.
