# S03.3 세션 종료 경합 사전 비교 모델

## 기준

비교 기준은 [내부 런타임 세션 계약](../0005-v8-runtime-session.md)의 `free` 동작과 [DOM 노드·wrapper 수명 계약](../0021-s03-dom-node-lifecycle.md)의 `Active → Closing → Closed` 순서다. shutdown 중에는 새 작업을 받지 않고, 활성 JavaScript는 취소하며, 이미 큐에 접수된 작업은 종료 오류를 받고, V8 자원은 isolate owner thread에서 폐기해야 한다. raw C ABI 포인터의 동시 `free`는 기존 계약상 허용하지 않는다.

이 기준은 프레임워크 제품 지원 판정이 아니라 내부 실행기 종료 상태의 검증 oracle이다. 네이티브 앱의 프로세스 강제 종료나 비동기 host API의 Promise 처리까지 확장하지 않는다.

## 고정 입력과 관찰값

| 조건 | 고정값 |
| --- | --- |
| 런타임 | 세션마다 실제 V8 Isolate 하나, 세션 전용 OS thread 하나 |
| 활성 작업 | owner에서 `while (true) {}` 평가를 시작하고 active 상태가 확인될 때까지 기다림 |
| 대기 작업 | 활성 작업이 유지되는 동안 실제 V8 작업 3개를 큐에 접수 |
| 종료 주체 | RuntimeSession을 계속 소유한 별도 Rust thread에서 내부 shutdown을 호출. C ABI handle은 동시 해제하지 않음 |
| 작업자 비정상 종료 | 테스트 전용 실행 thread가 panic하고, 대기 중인 eval 1개와 stale runtime pointer sentinel이 있는 세션을 닫음 |
| 유휴 종료 | 활성 작업과 대기 명령이 없는 새 세션을 종료한 뒤 runtime 해제, worker join, 후속 eval 거부를 확인 |
| 사후 입력 | shutdown 반환 뒤 eval과 dispatch를 각각 한 번 접수 |
| 플랫폼 | Android 16/API 36 ARM64 emulator, iPhone 17 Pro/iOS 26.2 simulator; 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`, `v8_jitless=false` |

## 사전 판정 기준

1. active infinite eval은 shutdown 취소로 `-8`을 반환한다.
2. shutdown 시작 전에 큐에 접수된 세 명령은 한 건도 JavaScript를 실행하지 않고 모두 `-6` 종료 오류를 반환한다.
3. shutdown 뒤 eval과 dispatch도 각각 `-6`을 반환한다. JS 부작용을 가진 대기 코드는 JS 실행 성공 응답을 받지 않는다.
4. shutdown은 bounded probe deadline 안에 반환하고 작업자 thread가 join된다. 내부 runtime pointer가 지워지고 active 상태가 false여야 한다.
5. 두 번 호출한 shutdown은 추가 V8 호출이나 오류 없이 반환한다.
6. Android와 iOS 모두 실제 V8에서 동일한 상태 코드를 기록해야 한다. timeout, 응답 누락, queue 실행, callback 증가, worker 잔류는 실패다.
7. Rust unwind 환경에서 worker panic을 주입하면 대기 명령이 `-7`로 끝나고 session이 stale runtime pointer를 더는 노출하지 않아야 한다. panic 난 owner를 대신해 V8 객체를 다른 thread에서 해제하지 않는다.
8. 활성 작업과 대기 명령이 없는 유휴 세션도 runtime을 해제하고 worker를 join하며, 종료 뒤 eval은 `-6`을 반환해야 한다.

이 검증은 Rust 단위 fixture에서 종료와 제출 경합의 잠금 순서를 확인하고, 각 시뮬레이터에서 실제 V8 종료 경로를 한 번씩 실행한다. 시뮬레이터 결과는 실기기 동작, 강제 프로세스 종료, timeout 이후 안전한 강제 회수, platform queue의 동시 raw-pointer 해제를 증명하지 않는다.
