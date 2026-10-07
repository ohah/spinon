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

Android 기기에서 trace의 PID 493은 `android.hardware.graphics.composer3-service.ranchu`로 확인했다. PID/TID와 프로세스 이름은 같은 emulator에서 수집한 [ADB 프로세스 기록](../../../build/spinon/benchmark/r05-reply-fixed-ui-runtime-attribution/20261006T150821Z/runs/20261006T151615Z-spinon-event/composer-service-process.txt)에 보존했다. Binder trace에서는 PID 543 `surfaceflinger`가 trace 시각 569798290.835ms에 PID 493으로 transaction code 5를 보냈고, Composer HAL 응답은 569798341.480ms에 기록됐다. CPU 3에서는 그 transaction이 처리되는 동안 RenderThread의 wake event가 있었지만 실행되지 않았고, SurfaceFlinger가 4.680ms 실행한 뒤에야 RenderThread가 CPU를 받았다. `DrawFrameTask`/`RenderProxy::postAndWait`의 AOSP 구현도 render task를 게시한 caller가 조건 변수에서 완료를 기다리는 구조다. [AOSP HWUI `RenderProxy.cpp`](https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/libs/hwui/renderthread/RenderProxy.cpp).

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

일상적인 Android 15–17ms 대기는 동기 main Handler와 View traversal sync barrier의 상호작용으로 실험에서 확인됐고, callback scheduling을 바꾼 대조군에서 제거됐다. 극단 129ms 사례는 Android Emulator 가상 Composer HAL이 CPU를 오래 사용해 RenderThread를 늦춘 56ms 구간과, 그 이전 72ms의 다음 `doFrame` 대기로 분리됐다. 앞의 72ms가 생긴 더 깊은 이유와 비동기 조건 첫 탭의 worker-side 27.877ms outlier는 미확정이다. iOS Simulator에서 같은 runtime callback 경계의 지연은 관찰되지 않았다.

R05는 미완료다. 남은 검증은 실제 Android/iOS 기기에서 동등 계측을 반복하고, iOS display presentation 시각·GPU renderer·계측 오버헤드를 확인하는 것이다. 현재 결과는 runtime worker/V8 계산을 129ms outlier의 원인으로 지목하지 않으며, emulator 표시 경로의 관측을 Android 전 기기에 적용하지 않는다.

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

`tools/benchmark/android-frame-attribution-detailed.textproto`에 `gfx` ATrace와 `android.surfaceflinger.frame` data source를 추가해 Android 16/API 36 ARM64 에뮬레이터, 60Hz, debug APK에서 각각 60탭을 수집했다. 수집 스크립트는 `SPINON_ANDROID_TRACE_CONFIG`로 별도 설정을 선택하도록 했다. 원본은 아래 경로에 있으며, 이 설정은 release·실기기 성능용이 아닌 원인 분석용이다.

| 실행 | 관측값 | 판정 범위 |
| --- | --- | --- |
| 실제 runtime event 60회, `20261006T225859Z-spinon-event` | `SpinonR05RuntimeDispatchCount=60`. MainActivity token 1435282의 App Deadline Missed 프레임은 32.242ms, `DrawFrames`는 32.039ms, `eglSwapBuffersWithDamageKHR`는 31.402ms였다. 같은 RenderThread의 해당 `DrawFrames` 구간 sched CPU 실행 합은 31.521ms였다. `v8-handler-call`은 0.522ms였고, 별도 `spinon-platform` worker의 runtime reply 대기는 29.018ms였다. | 이 프레임에서 RenderThread가 EGL swap 경계 안에서 CPU를 오래 사용한 사실을 확인했다. Runtime 응답 대기는 다른 thread의 구간이므로 그 자체를 프레임 miss의 원인으로 단정하지 않는다. EGL 내부의 개별 함수나 실제 GPU 실행 시간은 여기서 분해되지 않는다. |
| runtime dispatch 없는 UI-only 60회, `20261006T230512Z-spinon-ui-only` | `SpinonR05RuntimeDispatchCount=0`. App Deadline Missed token 1440193은 32.307ms였고 대응 RenderThread `DrawFrames`는 33.173ms, `eglSwapBuffersWithDamageKHR`는 31.556ms였다. 전체 122개 EGL swap 중 3개가 16.667ms를 넘었고 최대는 31.872ms였다. 추가 Buffer Stuffing frame도 1개 있었다. | 적어도 이 에뮬레이터에서는 runtime dispatch 없이도 같은 GLES View/RenderThread/EGL 경로에서 지연 프레임이 발생했다. 이는 Spinon runtime이 원인인 모든 frame을 뜻하지 않으며 제품 `wgpu` renderer의 측정도 아니다. |
| `gfx` VSync 표식 | runtime event trace에서 `onComposerHalVsync`가 2,387회 관측됐고 30ms 초과 간격은 34.071ms와 46.429ms였다. 34.071ms 간격은 token 1435282의 `Choreographer#doFrame` resync(32.6ms)와 겹쳤다. UI-only trace의 최장 간격은 47.479ms였다. 어느 trace에서도 과거 72.057ms 구간은 재현되지 않았다. | VSync/Choreographer 간격은 더 측정할 수 있으며, 이번에 관측한 공백은 원래 129.711ms 사례의 앞선 72.057ms 공백을 설명하지 않는다. |

상세 trace는 `onComposerHalVsync`, `Choreographer#doFrame`, `DrawFrames`, `eglSwapBuffersWithDamageKHR`, `GPU completion fence` 표식을 실제로 기록했다. Perfetto packet loss는 0이지만 `ftrace_setup_errors=37`이 남았고 그 원인과 빠진 event는 확인하지 못했다. 따라서 캡처가 완전하다고 판정하지 않는다. `simpleperf record`는 `sched:sched_switch`와 `task-clock` event를 지원하지 않는다고 반환했다. Perfetto `linux.perf` stack-profile 시도도 stack sample 0개였고, 별도 ftrace scheduler tracepoint는 동작했다. 이 AVD에서 native call-stack profile을 얻지 못한 것이므로 Android 전반에서 측정 불가능하다는 결론은 아니다.

같은 Android 16 Composer AIDL V4의 frozen API에서 `IComposerClient`의 첫 다섯 메서드는 `createLayer`, `createVirtualDisplay`, `destroyLayer`, `destroyVirtualDisplay`, `executeCommands` 순서다. 서비스의 `IComposer`는 `createClient`, `getCapabilities` 두 메서드뿐이다. 따라서 PID 493의 Composer service를 향한 Binder transaction code 5는 V4 AIDL 순서상 client의 `executeCommands`로 매핑된다. 비교한 primary source는 [Android 16 V4 `IComposerClient.aidl`](https://android.googlesource.com/platform/hardware/interfaces/+/refs/tags/android-16.0.0_r1/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/4/android/hardware/graphics/composer3/IComposerClient.aidl), [Android 16 V4 `IComposer.aidl`](https://android.googlesource.com/platform/hardware/interfaces/+/refs/tags/android-16.0.0_r1/graphics/composer/aidl/aidl_api/android.hardware.graphics.composer3/4/android/hardware/graphics/composer3/IComposer.aidl)다. Binder trace에 target node의 interface descriptor가 없어서 이 mapping은 빌드된 service·version·transaction 번호를 결합한 소스 대조이며 payload 직접 decode는 아니다.

## iOS Instruments 추적 시도 판정

`build/spinon/benchmark/r05-ios-instruments/20261007-system-trace/`의 System Trace는 CLI가 20초 기록 완료를 보고했지만, trace TOC의 시간 범위는 2026-10-07 06:37:23.281–23.454 KST(0.173초)였다. 앱 probe 로그는 06:37:26.685에 시작해 06:37:33.619에 끝났다. 따라서 이 trace는 32개 probe와 겹치지 않아 iOS thread/frame 원인 분석 자료로 사용할 수 없다. `xctrace export --toc`도 `getpwuid_r did not find a match for uid 501` 오류로 실패했다. 앞서 확인한 `Animation Hitches` template의 Simulator 미지원과 Time Profiler의 V8 DWARF symbolization 경고·계측 시간 변화도 그대로 한계로 둔다.

## 아직 측정하지 못했거나 원인을 확정하지 못한 항목

| 항목 | 지금까지 확인 | 측정 가능성과 남은 절차 |
| --- | --- | --- |
| 원래 129.711ms 사례 앞부분의 72.057ms 다음 `doFrame` 대기 원인 | 원본 trace에서 앱 main thread는 잠들어 있었고 VSync ID 공백이 있었다. 상세 `gfx` trace는 다른 실행에서 34.071ms·46.429ms 공백을 잡았지만 72.057ms 구간은 재현되지 않았다. 이전 Mac `sample`은 Android trace와 시간 동기화된 scheduler 기록이 아니다. | 측정 자체는 가능하다. 같은 outlier를 재현할 때 guest Perfetto와 host scheduler를 함께 시작하고 clock marker를 넣어 VSync 공급자·QEMU host scheduling을 분리한다. 지금까지는 해당 원인을 확정하지 못했다. |
| 과거 asynchronous 첫 callback의 27.877ms worker-side 지연 | 60회 반복과 새 프로세스 첫 callback 20회에서 재현되지 않았다. `simpleperf`는 `cpu-cycles`, `cpu-clock`, `task-clock`, `sched:sched_switch` 측정 중 일부를 이 emulator에서 지원하지 않는다고 반환했고, Perfetto CPU stack trial은 표본 0개였다. | 측정 불가능하다고 볼 수 없다. 같은 첫 dispatch 반복 중 재현을 기다리며 ART method trace를 붙이거나 CPU profiling이 되는 기기에서 기록한다. 재현 전에는 Java/ART·JIT·GC 원인을 추정하지 않는다. |
| RenderThread EGL swap 내부의 native 함수·driver/GPU 기여 | `gfx` trace에서 지연 frame의 `eglSwapBuffersWithDamageKHR` 경계와 sched CPU 점유를 관측했다. runtime dispatch 없는 UI-only에서도 EGL swap 31.556ms와 App Deadline Missed 32.307ms가 함께 있었다. 이 emulator의 CPU stack profiler는 표본을 내지 않았다. | EGL 경계는 측정됐다. 함수 내부 귀속은 native sampling이 가능한 Android debug 기기나 별도의 그래픽 driver/GPU trace가 필요하다. 현재 증거는 Spinon 제품 `wgpu` 성능을 말하지 않는다. |
| Composer Binder transaction code 5의 method | Android 16 system image는 Composer AIDL V4이고, V4 메서드 순서상 code 5는 `IComposerClient.executeCommands`다. `IComposer` control interface는 두 메서드만 둔다. | source mapping은 됐다. Binder trace에 target object의 interface descriptor가 없으므로 raw transaction node/descriptor를 직접 확인한 것은 아니다. method 명칭 자체는 더 이상 미측정 항목으로 두지 않는다. |
| 상세 Perfetto trace의 누락 범위 | 두 상세 trace 모두 frame/VSync/scheduler를 기록하고 packet loss는 0이었지만 `ftrace_setup_errors=37`이었다. 어떤 요청 event가 설정 실패했는지는 아직 분해하지 않았다. | 항목별 최소 설정 trace를 비교해 37개 오류 원인을 분리하고, 필요한 event가 실제 기록됐는지 검증한다. 현재 trace를 오류 없는 완전 계측으로 간주하지 않는다. |
| Android 지연에서 host QEMU scheduling이 차지한 정확한 시간 | 이전 host process sample은 시각별 thread scheduling trace가 아니다. Mac System Trace attach는 30초 요청에도 TOC상 06:45:27.848–06:45:28.061 KST만 기록했고 Android UI-only 실행은 06:46:07 KST에 시작해 겹치지 않았다. | 측정 가능한 host scheduler 문제다. 실행 전체와 겹치는 privacy-safe host scheduler capture, guest/host clock marker, 반복 입력이 필요하다. 기존 capture는 무효라 host 기여를 아직 말할 수 없다. |
| iOS Simulator의 OS thread와 frame 원인 | 기존 System Trace는 32개 callback log와 시간상 겹치지 않았다. Animation Hitches template은 이 Simulator에서 지원되지 않았고 Time Profiler는 계측 시점을 바꿨다. | 측정 자체는 가능하지만 해당 capture는 다시 해야 한다. 실행 중 앱 process에 attach하고 trace window가 app marker를 덮는지 확인한다. 같은 빌드로 profiler on/off 대조 전에는 지연 수치를 비교하지 않는다. |
| 실제 입력부터 화면 픽셀 발광까지의 지연 | Android 입력은 ADB 주입이고 iOS 입력은 selector 호출이다. `CADisplayLink`와 FrameTimeline은 패널 발광 시각이 아니다. | 현재 Simulator/Emulator만으로는 실제 입력·광학 지연을 측정할 수 없다. 실기기 물리 입력과 고속 카메라 또는 photodiode, 측정 오차 및 반복 분포가 필요하다. |
| Spinon 제품 GPU renderer의 frame 비용 | 이번 지연 frame은 Android View와 Emulator GLES translation 경로다. `wgpu` 제품 renderer의 scene build·GPU 제출·present를 측정하지 않았다. | 측정 가능하지만 같은 JS 이벤트 시나리오를 제품 GPU surface에 연결해야 한다. 이어서 Android/iOS release 실기기에서 GPU frame trace와 실제 화면을 수집한다. |
| 계측이 callback/frame 지연에 더한 오버헤드 | 같은 조건의 tracing on/off 대조를 하지 않았다. | 측정 가능하다. 같은 빌드·기기·입력 순서를 교대해 켜짐/꺼짐 각각 분포를 수집한다. |
| 실기기·release·다른 refresh rate 재현성 | 현재 연결된 장치는 Android API 36 emulator와 iPhone 17 Pro iOS 26.2 Simulator뿐이다. | 지금은 이 환경에서 측정할 수 없다. Android/iOS 실기기와 release 빌드, 고정 주사율 조건이 필요하며 기존 simulator 결과와 합산하지 않는다. |

### 추가 원본

- Android 동기 handoff 60회: `build/spinon/benchmark/r05-android-host-scheduling-20261007/20261006T213137Z-spinon-event/`
- Android asynchronous Handler 60회: `build/spinon/benchmark/r05-async-handler-simpleperf-20261007/20261006T212355Z-spinon-event/`
- Android cold-process Dalvik/JIT/GC 20회: `build/spinon/benchmark/r05-android-dalvik-attribution-20261007/round-01/` ~ `round-05/`
- Android runtime 없는 UI-only 60회: `build/spinon/benchmark/r05-android-ui-only-control-20261007/20261006T213927Z-spinon-ui-only/`
- Android `gfx`·SurfaceFlinger 상세 runtime event 60회: `build/spinon/benchmark/r05-detailed-gfx-vsync-20261007/20261006T225859Z-spinon-event/`
- Android `gfx`·SurfaceFlinger 상세 UI-only 60회: `build/spinon/benchmark/r05-detailed-gfx-vsync-20261007/20261006T230512Z-spinon-ui-only/`
- Android CPU stack profile 기능 확인: `build/spinon/benchmark/diagnostic-capabilities/` (에뮬레이터에서 Perfetto `linux.perf` sample 0개와 `simpleperf` unsupported 오류 기록)
- Android 상세 trace 설정: [android-frame-attribution-detailed.textproto](../../../tools/benchmark/android-frame-attribution-detailed.textproto); 기본 capture script에서 `SPINON_ANDROID_TRACE_CONFIG`로 선택한다.
- Android emulator host sample: `build/spinon/benchmark/r05-android-host-scheduling-20261007/emulator-host.sample.txt` (host와 guest trace의 정밀 시간 동기 자료는 아님)
- iOS probe 로그: `build/spinon/benchmark/r05-ios-instruments/20261007-system-trace/ios.log`. Instruments 원시 bundle은 OS trace가 무효이고 프로세스 metadata가 포함될 수 있어 공유 근거에 넣지 않는다.

따라서 미측정 항목이 전부 측정 불가능한 것은 아니다. 상세 trace로 VSync 표식과 EGL swap 경계, runtime 없는 UI frame miss까지 더 측정했고 Composer method도 AIDL V4 순서로 매핑했다. 현재 도구가 stack sample을 만들지 못하거나 기존 trace가 시간상 겹치지 않은 항목은 다른 방법으로 다시 시도할 수 있다. 실제 input-to-photon은 현재 장비로 잴 수 없어 실기기와 광학 센서가 필요하다. 원래 129ms 사례의 72ms 원인·RenderThread 내부 함수·host 기여·iOS OS scheduling·제품 GPU renderer·계측 오버헤드는 아직 확정되지 않았으며 R05 완료 조건은 충족되지 않았다.
