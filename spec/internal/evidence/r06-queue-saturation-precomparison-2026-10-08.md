# R06 · 큐 포화·복구 구현 전 비교 모델

**상태:** 구현 전 기준 · 실행 결과는 [별도 검증 기록](./r06-queue-saturation-2026-10-08.md)에 보존

**계획:** [R06 큐 포화·복구 검증 계획](../../../plan/r06-queue-saturation.md)

## 기준 구현

기준 commit은 `6749d1a` (`origin/main`, 2026-10-08)다. `crates/spinon-runtime/src/session/scheduler.rs`의 `TaskScheduler::try_enqueue()`는 `SchedulerState` mutex 안에서 `stopped`와 `queue.len() >= QUEUE_CAPACITY`를 확인한 다음 삽입한다. `QUEUE_CAPACITY`는 64이고, `crates/spinon-runtime/src/session.rs`의 `enqueue()`는 `EnqueueError::Full`을 현재 내부 오류 `-5`로 변환한다. 수락된 `submit()`은 결과를 기다리지만, queue-full 오류는 응답을 기다리지 않고 반환한다.

용량은 실행 중인 명령이 아니라 대기 queue의 항목 수다. 현재 실제 V8 fairness probe는 공간이 생길 때까지 producer가 대기하므로 포화 거부 경로를 대조하지 않는다. 기존 동시 unit test는 정확한 수락/거부 개수, 거부 side effect 부재, drain 뒤 재접수를 고정하지 않는다.

## 입력·환경

- 한 RuntimeSession에서 무한 동기 JavaScript를 실행 중으로 유지한다.
- 모든 검증 항목을 같은 `UserVisible` 우선순위로 넣어 priority reorder와 capacity 확인을 분리한다.
- 고유 marker를 가진 64개 command를 먼저 비동기 접수하고, 그 뒤 side effect를 남기는 65번째 command를 별도 producer thread에서 시도한다.
- 모든 표식 응답을 수집한 뒤 성공·실패 경로에서 임시 RuntimeSession worker 종료·join을 수행한다. helper thread join은 시간 제한을 두며, 제한 초과 시 실행은 실패다.
- 동일한 실제 V8 scenario를 Android ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행한다. 현재 simulator validation script의 V8 revision pin과 `v8_jitless=false` 검사를 유지한다.
- 각 플랫폼 5회 반복한다. 실기기·release 성능은 이 실행 범위가 아니다.

## 관찰값

- `queue.len()`가 64인 상태에서 65번째 enqueue 결과와 elapsed time
- blocker status, 64개 accepted task 각각의 status·sequence·`last_node_id`·owner/callback thread
- accepted side effect 값 집합과 FIFO 순서
- rejected marker 유무를 읽는 후속 JS 검증, drain 후 새 command 결과
- 플랫폼별 console/log 원본, 실행 환경, 캡처 및 SHA-256

## 사전 판정

| 관찰 | 기대 결과 | 실패 조건 |
| --- | --- | --- |
| 실행 blocker | cancel 전 active, cancel 뒤 `-8` | 시작 확인 불가 또는 다른 status |
| 대기 큐 | 64개 접수 뒤 깊이 64 | 64가 아닌 값 |
| 65번째 제출 | 실행 중 JS가 계속 돌 때 2초 안에 `-5` 반환 | timeout, 성공, 기타 오류 |
| accepted command | 64개 모두 `OK`, 표식 `1..64` 정확히 한 번, FIFO | 누락·중복·역전·오류 |
| rejected command | 전역 overflow side effect 미설정 | 후속 JS에서 side effect 관측 |
| recovery | 큐 drain 이후 후속 eval 정상 완료 | 신규 명령이 거부·정체되거나 검증값 오류 |
| V8 경계 | accepted task callback의 owner/callback TID 동일 | TID 불일치 또는 응답 누락 |
| 반복 | Android 5/5, iOS 5/5 | 둘 중 한 시도라도 판정 불일치 |

2초는 동기 실행 완료 대기를 찾는 timeout guard일 뿐 성능 pass 기준이 아니다. 증거는 한 에뮬레이터와 한 시뮬레이터의 진단 결과로만 설명한다.

## 해석 범위

이 모델은 기존 내부 admission 동작을 검증한다. 제품 adapter에서 queue가 한 개뿐이라는 뜻이 아니며, `64` 또는 `-5`를 공개 동작 계약으로 승격하지 않는다. priority별 capacity, UI 입력 대기열, 작업 병합·폐기·backpressure, 장기 fairness, 메모리 한도, 실기기 및 release 동작은 판정하지 않는다.
