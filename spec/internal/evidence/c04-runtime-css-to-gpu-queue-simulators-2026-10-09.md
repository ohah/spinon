# C04.10 · 대기열·종료·draw 실패 시뮬레이터 검증

2026-10-09에 미출시 내부 계약 `0.1.0`을 유지한 채 C04.10의 제한된 런타임 GPU fixture를 Android·iOS 시뮬레이터에서 확인했다. 실패 주입은 내부 시험 빌드에만 켰다.

## 실행 환경과 명령

- Android: Android API 37 ARM64 AVD, `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=1`, Debug APK.
- iOS: iPhone 17 Pro Simulator, iOS 26.2, 같은 두 시험 옵션을 켠 Debug 앱.
- 기본 C04.10 renderer 생성과 JS/CSS 계산을 확인한 뒤 Android는 GLES 강제 backend, iOS는 Metal로 앱을 실행했다.
- Android Vulkan 강제 실행은 `No suitable graphics adapter found`로 종료됐다. GLES 경로는 `ANGLE ... SwiftShader`를 사용했다. 해당 AVD의 software renderer 결과를 하드웨어 GPU 증거로 취급하지 않는다.
- Rust hook 시험: `mise exec -- cargo test --locked -p spinon-render-wgpu --features test-hooks` — 13개 통과.
- FFI hook 시험: `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-test-hooks` — 26개 통과.
- 플랫폼 lane 시험: `mise exec -- bun run test:c04-queue` — Android JVM 5개, Swift 4개 통과.
- 앱 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=1 mise exec -- bash tools/build-android-app.sh --rerun-tasks`, 같은 환경으로 `mise exec -- bash tools/build-ios-sim.sh`. 두 빌드 모두 성공했다.

## 대기열 한도와 최신 상태

Java·Swift의 실제 플랫폼 lane 소스를 독립 JVM/Swift 시험에서 실행했다. lane의 소비 작업을 latch로 멈춘 동안 생산자 1개가 100,000건을 보내고, 이어서 생산자 8개가 각각 10,000건씩 동시에 보냈다. 각 경우 대기 drain은 하나를 넘지 않았고, 입력이 끝난 뒤 마지막 canonical 상태를 읽었다. 100,000건 단일 생산자 경우 작업 실행은 진행 중 1회와 최신 상태를 읽는 후속 1회로 합쳐졌다.

두 시뮬레이터 앱에서도 render lane 작업을 의도적으로 멈춘 뒤 10,000개의 draw 요청을 넣었다. Android 로그에서 `scheduled=true dirty=true`, iOS에서 `scheduled=true dirty=true pending=0`을 확인했다. iOS의 `pending`은 실행 중 작업을 세지 않는 queue 대기 개수다. 실행 중 하나와 후속 drain 하나만 유지하며 callback별 closure를 쌓지 않는다.

## 종료 중 pending draw

- Android: 대기 중인 draw 상태에서 시험 종료를 시작했다. 종료가 lane을 닫고 `pendingDraw`를 비운 뒤 gate가 풀렸고, draw는 `SPINON_C0410_SHUTDOWN_PROBE_DRAW_DROPPED`로 폐기됐다. renderer 파괴가 성공한 뒤 `SPINON_C0410_DISPOSE_DRAINED`가 기록됐다.
- iOS: draw lane이 막힌 동안 10,000개 요청을 합친 뒤 `SPINON_C0410_SURFACE_DESTROYED`로 generation을 무효화했다. gate가 풀린 후 scene 제출 없이 `SPINON_C0410_DRAINED`까지 도달했다. main thread에서 render queue를 동기 대기하지 않았다.

## draw 실패 전달과 재사용

두 앱 모두 같은 renderer에서 정상 draw, 내부 주입 draw 오류, 다시 정상 draw를 순서대로 실행했다. Android와 iOS에서 주입 오류는 `status=-11`로 노출됐고, 바로 다음 draw가 `status=0`으로 성공했다. 첫 정상 draw와 복구 draw 모두 같은 `environment_revision`을 보고했다. 주입 hook은 JS·DOM·CSS scene을 수정하지 않고 다음 draw 한 번에서 소비된다.

## 이미지와 원본 로그

- Android 화면: [`c04-runtime-css-to-gpu-queue-android-api37-2026-10-09.png`](c04-runtime-css-to-gpu-queue-android-api37-2026-10-09.png)
- Android 로그: [`c04-runtime-css-to-gpu-queue-android-api37-2026-10-09.log`](c04-runtime-css-to-gpu-queue-android-api37-2026-10-09.log)
- iOS 화면: [`c04-runtime-css-to-gpu-queue-ios-26.2-2026-10-09.png`](c04-runtime-css-to-gpu-queue-ios-26.2-2026-10-09.png)
- iOS 로그: [`c04-runtime-css-to-gpu-queue-ios-26.2-2026-10-09.log`](c04-runtime-css-to-gpu-queue-ios-26.2-2026-10-09.log)

## 판정 범위

이 결과는 해당 고정 시뮬레이터·시험 빌드에서 lane의 bounded admission, close 뒤 pending 작업 폐기, 합성 draw 오류 전달, 다음 draw의 동일 scene 재사용을 확인한다. 실제 GPU·driver 오류 복구, 실제 기기 성능, 장시간 부하, 모든 surface-loss 상황, production 코드 서명은 증명하지 않는다. Android 강제 Vulkan adapter 초기화 실패도 이 AVD에서만 확인한 결과다.
