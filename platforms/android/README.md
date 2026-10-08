# Android 빌드 골격

`app/`은 V8 정적 라이브러리와 Rust FFI를 묶어 앱 시작 때 예제 JavaScript를 한 번 실행하는 개발용 부트스트랩입니다. 이 앱의 빈 화면과 로그는 제품 렌더러가 아니며 Android 네이티브 UI 지원을 뜻하지 않습니다.

필요한 도구는 JDK 17, Android SDK 36, NDK `27.1.12297006`, Bun·Rust는 루트 `mise.toml`의 고정 버전입니다. Gradle Wrapper `8.13`과 Android Gradle Plugin `8.13.2`를 사용합니다. V8 소스 커밋·빌드 산출물 준비는 [V8 빌드 안내](../../native/v8/VERSION.md)를 따릅니다.

Android SDK에는 `platforms;android-36`, `build-tools;35.0.0`, `ndk;27.1.12297006`을 설치합니다. NDK 버전의 단일 원본은 `tools/android-ndk-version.txt`이고 Gradle과 네이티브 빌드가 같은 값을 읽습니다. SDK가 `~/Library/Android/sdk`가 아닌 경로에 있으면 `ANDROID_SDK_ROOT`를 지정합니다. `ANDROID_NDK_HOME`이 다른 버전을 가리키면 경고를 남기고 SDK에 설치한 고정 버전을 우선 사용합니다.

```sh
sdkmanager --licenses
sdkmanager "platforms;android-36" "build-tools;35.0.0" "ndk;$(cat tools/android-ndk-version.txt)"
```

루트에서 `mise exec -- bun run build:android`로 APK를 빌드합니다. Gradle Wrapper의 `preBuild`가 Bun 번들 생성, Rust ARM64 정적 라이브러리 빌드, V8·JNI 공유 라이브러리 링크를 먼저 수행합니다. APK는 `platforms/android/app/build/outputs/apk/debug/app-debug.apk`에 생성됩니다.

S04 고정 CSS·GPU fixture는 내부 검증용 선택 기능이며 기본 Android 빌드에는 포함되지 않습니다. Cargo의 `#[cfg(feature = "s04-android-fixture")]`는 이 통합 fixture 경로를 조건부 컴파일하며, `#[cfg(test)]` 단위 테스트 전용 표시는 아닙니다. 기본 빌드에서는 JNI 진입점이 남아 `SPINON_S04_FIXTURE=disabled`를 반환하고 Rust fixture 구현은 포함하지 않습니다. 기본 빌드를 명시하려면 `mise exec -- env SPINON_ENABLE_S04_ANDROID_FIXTURE=0 bun run build:android`, 에뮬레이터에서 fixture 화면을 확인하려면 `mise exec -- env SPINON_ENABLE_S04_ANDROID_FIXTURE=1 bun run build:android`로 빌드한 뒤 `adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_s04 true`를 실행합니다. Cargo feature `s04-android-fixture`와 JNI compile flag는 이 환경 변수로 함께 켜집니다.

S04 화면의 하단 상태 문구는 비동기 RGBA readback을 시작하면 진행 중을, 비대칭 y fixture의 36개 sRGB 표본 검증이 끝나면 통과를 표시합니다. 오류와 5초 시간 초과는 실패 상태로 표시하고 자세한 내용은 Logcat에 기록합니다. [Android 에뮬레이터 실행 근거](../../spec/internal/evidence/s04-asymmetric-y-platforms-2026-10-07.md).

색상 띠를 탭하면 `MotionEvent`의 SurfaceView local physical pixel 좌표를 렌더와 같은 scale·letterbox 변환으로 역산해 고정 snapshot의 `NodeId`를 상태 문구와 `SPINON_S04_HIT_TEST` 로그에 표시합니다. 화면 상태를 확인하려면 `adb logcat -d | rg SPINON_S04_HIT_TEST`를 실행합니다. 이 opt-in 동작은 fixture hit-test이며 DOM 이벤트나 JavaScript callback을 실행하지 않고, `Queue::present`가 실제 표시된 frame을 식별하지도 않습니다. Android API 36 ARM64 emulator 결과와 캡처는 [S04.9 실행 근거](../../spec/internal/evidence/s04-hit-test-platforms-2026-10-07.md)에 있습니다.

기본 시작 smoke 로그는 `adb logcat -s SpinonBootstrap`에서 `SPINON_BOOTSTRAP_RESULT=nodes=2 last_node=8 tag=text text=이벤트:7`을 확인합니다. 기본 smoke 범위는 ARM64 단일 ABI이며 GPU·제품 입력·UI 트리·접근성·JIT 없는 기기 빌드는 포함하지 않습니다. 별도 R06 화면에서만 개발용 터치 버튼을 실행합니다. 구현 완료 표시는 [공식 상태 대장](../../spec/STATUS.md)과 [내부 V8 실행 인터페이스](../../spec/internal/0001-v8-bootstrap.md) 기준을 따릅니다.

## V8 실행 스레드 실험

개발용 APK에서 Rust 세션 스레드·입력·취소 화면을 열려면 아래 Intent를 사용합니다. 기본 시작 화면과 R10 실험 화면은 유지됩니다.

```sh
adb install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_runtime_threads true
adb logcat -s SpinonBootstrap:I
```

화면에서 긴 JavaScript 실행 중 탭을 눌러 UI가 반응하는지 확인하고, 별도 취소 버튼을 누른 뒤 `owner_tid`가 유지되는지 로그를 확인합니다. 대기 상태에서는 긴 JS 시작만 활성화하고 취소는 비활성화합니다. 평가가 실행 중일 때 시작을 비활성화하고 취소를 활성화하며, 실행 중 접수한 JS dispatch가 끝난 뒤 다음 평가를 허용합니다.

취소 요청 `status=0`은 요청 접수이고, 평가 `status=-8`은 V8 실행 중단 확인입니다. 화면은 이 결과를 `취소 완료`로 표시합니다. 취소 입력이 없으면 12초 뒤 안전 취소를 요청합니다. 로그 항목은 메인 스레드 작업 하나로 모아 TextView에 반영하고, 화면 기록은 20,000자를 넘으면 오래된 줄을 덜어내 16,000자 안팎으로 유지하며 생략 문구를 표시합니다. 전체 진단 기록은 Logcat에 남깁니다. iOS도 메인 큐에서 로그를 모아 반영하고 같은 표시 한도·생략 문구를 사용하며 취소와 대기 이벤트 실행 뒤 같은 여섯 검증 결과를 표시합니다. 다섯 번 반복한 Android 프레임 표본과 두 플랫폼의 긴 JS·지연 응답·취소·안전 시간 초과 실행은 [iOS·Android 화면 일치 기록](../../spec/internal/evidence/s03-r06-cross-platform-ui-parity-2026-10-05.md)에 있습니다.

Android 어댑터는 동기 FFI 호출을 4개 작업자와 최대 64개 대기 작업으로 제한하고, 취소는 별도 제어 실행기에서 보냅니다. 실제 V8 에뮬레이터 실행의 버튼 상태·취소·대기 이벤트는 [Android 수동 실행 근거](../../spec/internal/evidence/s03-android-long-js-ui-2026-10-05.md)에 기록합니다. 가짜 V8 단위 테스트는 제어 경로만 검증하므로 실제 V8이 실행된 에뮬레이터의 원본 로그도 확인합니다. [대기열 압력 원본 로그](../../spec/internal/evidence/r06-android-queue-pressure-2026-09-30.log)는 Android 16 에뮬레이터에서 70회 탭 주입, 화면 카운터 69회, 플랫폼 작업 2회 거부, V8 dispatch 67회 성공을 기록합니다. 이 수치는 플랫폼 대기열 실험이며 Rust 런타임 큐 포화를 뜻하지 않습니다. 범위와 한계는 [V8 실행 스레드 실험 명세](../../spec/internal/0005-v8-runtime-session.md)와 [R06 검증 근거](../../spec/internal/evidence/r06-v8-runtime-thread-2026-09-30.md)에 기록합니다.

### R05 Android 프레임 지연 대조 수집

같은 Android View 화면에서 입력만, 상태 TextView 갱신, 로그 추가·자동 스크롤 비용을 나눠 기록하고 실제 V8 이벤트 화면과 대조하려면 아래 도구를 사용합니다.

```sh
mise exec -- bun run build:android
bash tools/benchmark/capture-android-frame-attribution.sh idle --serial emulator-5554
bash tools/benchmark/capture-android-frame-attribution.sh input-only --serial emulator-5554
bash tools/benchmark/capture-android-frame-attribution.sh status --serial emulator-5554
bash tools/benchmark/capture-android-frame-attribution.sh log-scroll --serial emulator-5554
bash tools/benchmark/capture-android-frame-attribution.sh spinon-event --serial emulator-5554
```

같은 Spinon 화면에서 runtime dispatch 효과를 분리하려면 UI-only와 event를 10회씩 짝지어 수집합니다. 분석기는 공식 Perfetto Trace Processor와 Bun이 필요합니다.

```sh
bash tools/benchmark/run-android-ui-runtime-attribution-matrix.sh \
  --serial emulator-5554 --repeats 10 --duration 20 --taps 3
SPINON_TRACE_PROCESSOR=/path/to/trace_processor \
  mise exec -- bun run tools/benchmark/summarize-android-ui-runtime-attribution.mjs \
  build/spinon/benchmark/android-ui-runtime-attribution/<matrix-id>
```

요약기는 행렬별 실제 탭 수와 조건 순서를 schedule 파일에서 읽고 JNI caller와 V8 owner thread를 따로 분석합니다. paired 조건은 seed로 시작 순서를 정한 뒤 회차마다 교대하며, 분석기는 실제 schedule의 균형과 순서를 검증합니다. 앱 프레임은 고유 `surface_frame_token`, display·SurfaceFlinger 지표는 고유 `display_frame_token`으로 중복 없이 집계하고 actual/expected token 일치도 표시합니다. Rust runtime report는 caller TID로 Java worker trace와 연결하고 queue·V8·actor 응답 전 시간을 분리합니다. Android V8 handler, HostDocument callback, microtask, safe-point 구간을 비교할 때 `native-session-ffi`와 `reply-send` ATrace wall 시간을 CPU 실행 시간으로 해석하지 마세요. `runOnUiThread`의 main callback 대기는 runtime 계산과 따로 기록하며, CPU wake target·재개 CPU·thread state를 함께 확인합니다. [행렬과 구간별 실행 근거](../../spec/internal/evidence/r05-android-frame-attribution-2026-10-06.md)를 확인하세요.

기본 `runOnUiThread`와 sync barrier를 우회하는 `Handler.createAsync`를 같은 MainActivity에서 원인 분리용으로 비교하려면 다음을 실행합니다. 비동기 Handler 경로는 진단 플래그 전용이며 제품 callback 순서의 기본값이 아닙니다.

```sh
bash tools/benchmark/run-android-main-handoff-intervention.sh \
  --serial emulator-5554 --repeats 5 --duration 10 --taps 5
```

Android 16/API 36 에뮬레이터 후속 탭에서 기본 경로 p50은 16.125ms, 비동기 Handler는 75µs였다. 한 129ms outlier는 V8 이후 Android Emulator Ranchu Composer HAL·SurfaceFlinger 경로에서 RenderThread가 기다린 것으로 분리했다. 실기기 성능·표시 시각 검증은 아니다. [교차 플랫폼 원인 분석](../../spec/internal/evidence/r05-cross-platform-callback-root-cause-2026-10-07.md)을 참고하세요.

네 입력 조건을 회차마다 섞어 조건별 기본 10회 반복하려면 matrix runner를 사용합니다. 빠른 도구 점검은 `--repeats 1 --duration 10 --taps 3`; 원인 귀속 표본으로는 기본 반복 설정을 사용합니다.

```sh
bash tools/benchmark/run-android-frame-attribution-matrix.sh --serial emulator-5554
```

기본 회차는 Perfetto 20초, 탭 10회이며 결과는 `build/spinon/benchmark/android-frame-attribution/`의 고유 실행 폴더에 보관합니다. 빠른 파이프라인 점검에는 `--duration 10 --taps 3`을 쓸 수 있지만 반복 측정이나 성능 수치로 취급하지 않습니다. 조건·원본·분류 규칙은 [R05 계측 계약](../../spec/internal/0022-r05-benchmark-attribution.md), 기존 Android 프레임 표본과 새 실행 결과는 [R05 실험 기록](../../spec/internal/evidence/r05-android-frame-attribution-2026-10-06.md)에 있습니다. 이 에뮬레이터 수집은 Perfetto 경로와 원인 분리용이며 실기기 성능이나 GPU 렌더러의 벤치마크가 아닙니다.

원본 trace를 요약하려면 [공식 Perfetto Trace Processor](https://perfetto.dev/docs/getting-started/command-line-analysis)를 설치한 뒤 다음 명령을 실행합니다.

```sh
trace_processor query -f tools/benchmark/summarize-android-frame-attribution.sql \
  build/spinon/benchmark/android-frame-attribution/<실행 폴더>/frame-attribution.pftrace
```

### 실제 V8 우선순위 선택 검증

에뮬레이터에서 세 우선순위 선택 순서와 각 등급의 FIFO 순서를 확인하는 개발 진단 화면을 실행합니다.

```sh
adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_priority_probe true
adb logcat -s SpinonBootstrap:I | rg 'SPINON_PRIORITY_PROBE'
```

진단 화면은 두 개의 실제 V8 세션을 순서대로 실행합니다. 첫 세션에서 취소 후 우선순위를 섞어 접수한 6개 작업을 검사하고, 두 번째 Android 전용 세션에서 낮은 우선순위 작업을 먼저 넣고 큐 용량 64개를 채운 뒤 높은 우선순위 작업 96개를 추가합니다. 낮은 작업이 높은 등급 작업 159개 뒤에서 실행되는지, 높은 등급 FIFO와 owner thread가 유지되는지 봅니다. 유한 실험이므로 무한 유입 기아나 제품 공정성을 증명하지 않으며 성능 벤치마크도 아닙니다. 내부 검증 화면을 실행하려면 저장소 루트에서 `mise exec -- bun run verify:r06-priority:fairness:android`를 사용하세요. 이 명령은 `emulator-*` 대상만 허용하고 Android 에뮬레이터만 빌드·설치·실행합니다. 결과는 `build/spinon/priority-fairness-validation/`의 고유 실행 폴더에 저장됩니다. 단일 배치의 과거 근거는 [우선순위 시뮬레이터 검증](../../spec/internal/evidence/r06-priority-simulators-2026-09-30.md)을 참고하세요.

Android와 iOS 시뮬레이터에서 실제 V8의 혼합 우선순위/FIFO와 유한한 높은 등급 유입을 한 번씩 확인하려면 저장소 루트에서 `mise exec -- bun run verify:r06-priority:simulators`를 실행합니다. 이 명령은 `emulator-*` Android 대상과 부팅된 iOS 시뮬레이터만 사용합니다. 같은 시나리오를 각 플랫폼에서 5회 반복하려면 `mise exec -- bun run verify:r06-priority:fairness:simulators`를 실행합니다. 로그와 화면은 `build/spinon/priority-fairness-simulators/` 아래에 저장됩니다. [5회 교차 플랫폼 근거](../../spec/internal/evidence/r06-priority-fairness-simulators-2026-10-08.md).

### S03.3 DOM wrapper 회수 검증

검증 전용 강제 GC 진입은 기본 빌드에서 빠집니다. Android 에뮬레이터 검증 때만 flag를 켭니다.

```sh
SPINON_ENABLE_S03_DOM_GC_FIXTURE=1 mise exec -- bun run build:android
adb install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_dom_gc true
adb logcat -s SpinonBootstrap:I | rg 'DOM-GC 검증'
```

통과 결과는 HostRoot 트리와 살아 있는 detached 자손 wrapper를 보존하고, 연결된 Text wrapper의 WeakRef가 실제 GC 뒤 비워진 다음 조회로 wrapper가 재생성되는지 확인합니다. orphan WeakRef와 `document_nodes` 감소, collector 오류·poison 상태, scanned/live/empty handle 계수도 검사합니다. 기본 빌드는 hook과 버튼을 제외하며 결과 폴더가 이미 있으면 verifier는 기존 로그를 덮어쓰지 않습니다. 공유 Android·iOS verifier는 추가로 16,385개 wrapper 생성·scan·sweep, 같은 Isolate의 10회 재scan과 자원 기준선 복귀를 확인합니다([대량 registry 근거](../../spec/internal/evidence/s03-dynamic-wrapper-registry-2026-10-05.md), [반복 scan 관측](../../spec/internal/evidence/s03-dynamic-registry-repeat-measurements-2026-10-05.md)). 이 Android 측정용 artifact는 에뮬레이터에서 `autib1716` SIGILL이 발생한 뒤 V8 CFI를 끈 조건입니다. 모든 Android 에뮬레이터의 일반 동작을 뜻하지 않으며 실기기·CFI 활성 배포 빌드는 검증하지 않았습니다. 반복 wall-time 편차가 커서 성능 결론으로 쓰지 않습니다. 할당 실패 주입, 장기 반복, closure root, shutdown 경합, 실제 기기는 별도 범위입니다.

Android·iOS 시뮬레이터를 함께 빌드·실행하고 원본 log와 캡처를 저장하려면 `mise exec -- bun run verify:s03-dom-lifecycle:simulators`를 실행합니다. 기본 출력 위치는 `build/spinon/dom-lifecycle-validation/` 아래의 고유 실행 폴더입니다.

세션 종료 경합 probe는 `mise exec -- bun run verify:s03-shutdown:simulators`로 실행합니다. 고정 V8 revision과 Android·iOS Simulator의 `v8_jitless=false` 설정을 빌드 전에 확인하고 Android 16 에뮬레이터와 iOS Simulator에서 활성 eval 취소, 큐 대기 명령 거부, 종료 후 호출 거부, 반복 종료·worker join을 기록합니다. 기본 V8 경로는 `build/v8-source/v8`이며 별도 checkout은 `SPINON_V8_DIR`로 지정합니다. 원본 로그와 캡처는 고유 결과 폴더에 저장합니다. probe timeout은 통과로 처리하지 않으며, OS thread 강제 종료나 raw C ABI 포인터 동시 `free`를 검증하지 않습니다.
