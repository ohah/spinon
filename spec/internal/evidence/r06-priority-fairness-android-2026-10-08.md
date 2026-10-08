# R06 Android 에뮬레이터의 우선순위 유입 검증

**일자:** 2026-10-08 · **검증 대상:** Android 16 / API 36 ARM64 `sdk_gphone64_arm64` 에뮬레이터 · **V8:** `v8_jitless=false`, 저장소 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf` · **결과:** 최종 코드로 5회 모두 strict-priority/FIFO 통과 · **실기기·iOS:** 사용하지 않음

## 실행한 실험

Android 진단 화면은 두 실제 V8 세션을 차례로 연다. 첫 세션은 기존 여섯 작업의 우선순위·FIFO 기준을 확인한다. 두 번째 독립 세션은 실행 중 JavaScript를 취소한 뒤 아래 입력을 만든다.

1. `background` 작업 한 개를 먼저 접수한다.
2. 공유 큐 용량 64개 중 나머지 63개를 `user-blocking` 작업으로 채운다.
3. 별도 생산자 스레드가 큐에 자리가 생길 때마다 `user-blocking` 작업 96개를 더 접수한다. 각 작업은 짧은 동기 JavaScript 루프 뒤 노드 callback을 실행한다.
4. 159개 높은 등급 작업 모두가 낮은 등급 작업보다 먼저 실행되고, 같은 등급 FIFO와 V8 owner/callback thread가 유지되는지 검사한다.

이 입력은 유한한 연속 유입 사례다. 낮은 작업의 큐 대기가 실제로 길어지는지 확인하는 기능 검증이지 처리량·프레임 성능 벤치마크가 아니며, 무한 유입에서 기아가 계속되는지를 증명하지 않는다.

## 실행 결과

| 실행 | `user-blocking` 수 | 낮은 작업 큐 대기 | 전체 stream probe | owner TID |
| ---: | ---: | ---: | --- | ---: |
| 1 | 159 | 537,011 µs | 통과 | 25,240 |
| 2 | 159 | 495,021 µs | 통과 | 25,356 |
| 3 | 159 | 572,862 µs | 통과 | 25,453 |
| 4 | 159 | 625,795 µs | 통과 | 25,524 |
| 5 | 159 | 657,022 µs | 통과 | 25,599 |

다섯 실행 모두 `background`는 `seq=2`로 가장 먼저 접수됐지만 `execution_order=160`으로 마지막에 실행됐다. 그 사이 나중에 접수한 높은 등급 작업 159개가 실행됐다. 각 결과의 V8 callback thread와 owner thread가 일치했다. `background_wait_us`는 명령 enqueue부터 scheduler가 해당 명령을 꺼낼 때까지의 `queue_residence_us`이며 JS 완료·화면 표시 시각은 아니다. 최종 코드에서 관찰한 대기는 495–657ms 범위였다. 에뮬레이터 진단 표본이라 이 값은 성능 수치로 일반화하지 않는다.

첫 에뮬레이터 시도에서 반복 최상위 `const deadline` 선언이 같은 V8 전역 컨텍스트에서 충돌하는 것을 발견했다. 작업 로컬 `for` 초기식으로 바꾼 뒤 네 번 연속 통과했다.

## 재현 명령과 원본

실행기는 `SPINON_ANDROID_EMULATOR_SERIAL=emulator-5554`를 지정해 Android 에뮬레이터만 선택했다. 같은 Mac에서 연결돼 있던 실기기 serial에는 설치·명령을 보내지 않았다. `SPINON_V8_DIR`에는 고정 revision의 기존 checkout 경로를 지정해 읽기 전용으로 참조했고, 앱 빌드 산출물은 이 작업 worktree의 무시 경로에 저장했다.

실기기 serial을 `SPINON_ANDROID_EMULATOR_SERIAL`로 지정한 부정 대조는 `emulator-숫자` 형식 검사에서 빌드 전에 거부됐다.

```sh
SPINON_ANDROID_EMULATOR_SERIAL=emulator-5554 \
  mise exec -- bun run verify:r06-priority:fairness:android
```

- [5회 원본 진단 로그](r06-priority-fairness-android-2026-10-08.log)
- [Android 에뮬레이터 화면 캡처](r06-priority-fairness-android-2026-10-08.png)
- 화면 SHA-256: `adb880336ba4a596a819a739acc4928a6f1b69a9a978c209d6b30d43c8f7334e`
- 런타임 단위 테스트: `cargo test --locked -p spinon-runtime` — unit 56개, lifecycle integration 16개 통과

## 결론과 한계

현재 선택 규칙은 계약대로 동작한다. 큐에 대기 중인 높은 등급이 있으면 새로 접수된 높은 등급이 오래 기다린 `background`보다 먼저 선택된다. 그 결과 이 유한 사례에서 낮은 작업의 dequeue는 495–657ms 걸렸다. 이는 현재 정책의 기아 위험을 보여 주며, 해당 지연이 제품 사용에서 적절하다는 승인은 아니다.

이번 변경은 aging·양보·선점 같은 정책을 도입하지 않는다. 높은 등급 유입 전체에는 10초 제한을 두어 진단 생산자의 누적 대기를 제한한다. 무한 입력, 입력 UI 응답, Android 실기기, iOS, release 빌드, 공유 실행기 간 공정성은 확인하지 않았다. 제품 기본 우선순위와 기아 방지 여부는 [스케줄러 미결정 설계 목록](../0006-js-task-scheduler.md#미결정-설계-목록)에 남는다.
