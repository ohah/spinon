# R06 Android·iOS 시뮬레이터의 유한 우선순위 유입 검증

**일자:** 2026-10-08 · **Android:** Android 16 / API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터 · **iOS:** iPhone 17 Pro / iOS 26.2 시뮬레이터 · **V8:** `v8_jitless=false`, 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf` · **결과:** 두 플랫폼에서 각각 5회 통과 · **실기기:** 사용하지 않음

## 실행한 시나리오

각 실행은 독립된 실제 V8 세션에서 기존 혼합 우선순위·FIFO 검사를 먼저 실행하고, 이어서 유한한 높은 등급 유입 진단을 실행한다.

1. `background` 작업 하나를 먼저 접수한 뒤 공유 큐 64개 중 남은 63칸을 `user-blocking` 작업으로 채운다.
2. 큐에 공간이 생길 때마다 생산자 스레드가 `user-blocking` 작업 96개를 더 접수한다. 각 높은 등급 작업은 짧은 동기 JavaScript 실행과 callback을 포함한다.
3. 높은 등급 159개가 먼저 실행되고 background가 실행 순서 160번이 되는지, 높은 등급 FIFO와 Isolate owner/callback thread 일치가 유지되는지 판정한다.

Android는 Android 16 ARM64 에뮬레이터에서, iOS는 iPhone 17 Pro / iOS 26.2 Simulator에서 동일 진단 경로를 각각 5회 실행했다. iOS는 Rust runtime probe를 C ABI·Objective-C++·Swift 검증 화면에 연결했다. 공개 API는 추가하지 않았다.

## 결과

| 실행 | Android background queue residence | iOS background queue residence | 양쪽 결과 |
| ---: | ---: | ---: | --- |
| 1 | 501,074 µs | 477,232 µs | order 160 · FIFO · owner thread 통과 |
| 2 | 584,811 µs | 476,884 µs | order 160 · FIFO · owner thread 통과 |
| 3 | 507,639 µs | 476,826 µs | order 160 · FIFO · owner thread 통과 |
| 4 | 525,773 µs | 476,795 µs | order 160 · FIFO · owner thread 통과 |
| 5 | 560,906 µs | 477,286 µs | order 160 · FIFO · owner thread 통과 |

다섯 실행 모두 `background_seq=2`, `accepted_high=159`, `background_order=160`을 기록했다. `background_wait_us`는 명령 접수부터 scheduler dequeue까지의 `queue_residence_us`다. 이는 이 진단 입력에서 관측한 대기이며, 사용자 표시 지연이나 Android와 iOS 성능 비교값이 아니다.

## 재현 및 원본

저장소 루트에서 고정 revision의 Android·iOS Simulator V8 checkout을 사용해 실행한다. 외부 checkout은 `SPINON_V8_DIR`로 지정할 수 있다.

```sh
SPINON_V8_DIR=/path/to/pinned-v8-checkout \
  mise exec -- bun run verify:r06-priority:fairness:simulators
```

실행기는 Android `emulator-*` serial만 허용하고, 부팅된 iOS Simulator 하나를 대상으로 한다. 고정 V8 revision, Android arm64 및 iOS arm64 Simulator GN 설정, 양쪽 `v8_jitless=false`를 빌드 전에 확인한다. 반복 중 한 플랫폼이라도 빌드·probe·판정이 실패하면 즉시 실패하고 해당 실행 로그를 보존한다.

- [Android·iOS 10회 원본 진단 로그](r06-priority-fairness-simulators-2026-10-08.log)
- [Android 마지막 실행 화면](r06-priority-fairness-simulators-android-2026-10-08.png)
- [iOS 마지막 실행 화면](r06-priority-fairness-simulators-ios-2026-10-08.png)
- 단위 검사: `cargo test --locked -p spinon-runtime -p spinon-ffi` — runtime 59개 unit·16개 lifecycle integration, FFI 6개 통과
- Android debug APK 빌드 및 iOS Simulator 앱 빌드 통과

## 판정 범위

이 결과는 현재 strict-priority/FIFO 구현이 두 Simulator에서 유한한 입력을 계약대로 처리함을 보여준다. 높은 등급이 계속 들어오는 무한 유입에서의 기아, aging·양보 등 공정성 정책 비교, UI 입력 응답, 실기기 동작, release 성능은 확인하지 않았다. R06/E05 완료 판정이나 제품 공정성 정책 결정의 근거로 사용하지 않는다.
