# R06 · JS 실행 큐 포화·복구 검증 계획

**상태:** 내부 진단 구현 및 Android·iOS Simulator 각 5회 검증 완료 · 제품 역압력 계약은 미정

**상위 항목:** [R06·E05 상태](../spec/STATUS.md) · [스케줄러 내부 설계](../spec/internal/0006-js-task-scheduler.md)

**구현 전 비교 기준:** [큐 포화·복구 사전 비교 모델](../spec/internal/evidence/r06-queue-saturation-precomparison-2026-10-08.md)

## 작업 범위

현재 Rust 런타임은 실행 중인 JavaScript 한 건과 별도로 대기 명령을 최대 64개 보유한다. 같은 mutex 구간에서 용량을 확인하고 삽입한다. 용량을 넘긴 접수는 현재 내부 경계에서 `-5`로 즉시 거부한다. 기존 단위 테스트는 65개 동시 호출 중 일부 거부와 취소 뒤 완료만 확인해, 정확한 수락·거부 수, 거부 작업 미실행, 큐 복구를 충분히 증명하지 않는다. 기존 simulator 공정성 probe도 큐 공간을 기다린 뒤 다음 작업을 넣으므로 포화 시 실제 거부 경로를 지나지 않는다.

이 작업은 그 차이를 고정된 포화 입력과 실제 V8 시뮬레이터 실행으로 검증한다. 대기 큐의 용량·현재 오류 코드를 관찰하는 내부 진단이며, 제품용 호출 API, 플랫폼 adapter의 상위 큐 정책, 우선순위 예약·병합·폐기 정책은 결정하거나 공개하지 않는다.

## 고정 입력과 기대 결과

1. 실제 V8 owner에서 끝나지 않는 동기 JavaScript를 시작해 실행 중 명령이 선점되지 않는 상태를 만든다.
2. 동일 우선순위의 고유 표식 작업 64개를 비동기 enqueue 경계로 차례로 접수한다. 실행 중 작업은 용량 계산에 넣지 않으며 대기 큐 깊이는 정확히 64여야 한다.
3. 별도 호출 스레드에서 side effect 표식이 있는 65번째 작업을 접수한다. 실행 중 JavaScript가 끝나기 전에 호출이 반환되어야 하며 반환 상태는 현재 내부 동작인 `-5`여야 한다. 대기 큐 깊이는 64로 유지한다.
4. 차단 JavaScript를 취소한다. 차단 작업은 `-8`로 끝나야 한다. 이미 수락된 64개는 `OK`로 완료되고, 동일 우선순위 FIFO에 따라 JS 표식 값 `1..64`를 정확히 한 번씩 남겨야 한다.
5. 거부 작업의 side effect는 실행되지 않아야 한다. 큐가 빌 때까지 기다린 뒤 검증 명령을 다시 접수해 세션이 정상적으로 새 작업을 수락하는지 확인한다.
6. 성공 보고에는 실제 수락·거부·완료 개수, overflow 상태, 거부 표식 판정, 복구 결과 및 owner/callback thread 일치를 담는다. 보조 thread는 제한 시간 안에 join하며, 회수 시간 초과는 실패로 기록하고 OS thread를 강제 종료했다고 주장하지 않는다. 마지막 `RuntimeSession` 소유권을 놓으면 기존 `Drop` 종료·join 경로가 실행된다. Android와 iOS 결과는 분리 보존하고 각 플랫폼 5회 반복한다.

2초 timeout은 거부 접수가 차단 JS 완료를 기다리지 않는지 검출하고 진단 thread를 회수하기 위한 안전 한도다. 성능 임계값이나 사용자 체감 기준으로 해석하지 않는다. timeout이 나면 blocker를 취소해 cleanup한 뒤 해당 시도를 실패 처리한다.

## 독립 확인과 판정

- Rust 단위 테스트는 가짜 V8을 사용해 동시 65개 요청의 수락 64·거부 1, 거부 명령 미실행, FIFO 및 큐 drain 뒤 재접수를 확인한다. 이는 OS별 실제 V8 동작을 대신하지 않는다.
- Android ARM64 에뮬레이터와 iPhone 17 Pro / iOS 26.2 시뮬레이터의 실제 V8 probe는 동일한 표식·순서 불변식을 확인한다. 검증 빌드는 저장소에 고정된 V8 revision과 `v8_jitless=false`를 사용한다.
- 플랫폼별 5회 모두 사전 기대값과 일치해야 통과다. 수락·거부 개수, 상태, FIFO, side effect 부재, 정상 재접수 중 하나라도 다르면 실패다. 평균이나 다른 pass 항목으로 실패를 상쇄하지 않는다.
- 로그, 환경, 화면 캡처와 SHA-256을 실행별로 보존한다. 캡처는 진단 화면의 결과만 보여주며 그 자체를 큐 의미론의 oracle로 삼지 않는다.
- timeout, thread 생성 오류, 응답 누락에서는 세션을 닫고 blocker 취소·scheduler stop을 수행한다. pending 응답을 제한 시간 안에 회수하고 probe thread join을 시도한 뒤 마지막 session owner를 놓아 기존 `Drop`이 worker를 join하게 한다. 제한 시간이 지나 thread handle이 detach되면 실행은 실패로 보고하고 성공 로그를 만들지 않는다. Rust에서 임의 OS thread를 안전하게 강제 종료할 수 있다고 가정하지 않는다.

## 구현 산출물

- 결정성 있는 Rust queue/session 테스트를 보강한다.
- 기존 R06 실제 V8 진단에 포화·거부·drain·재접수 단계를 연결한다. 구현 경계는 테스트 진단 전용이며 JS 앱 API와 사용자 문법을 추가하지 않는다.
- Android/iOS 실행 스크립트의 기대 표식·로그·캡처 검증과 플랫폼 설명을 갱신한다.
- 사전 모델은 실행 근거와 분리해 유지하고, `spec/STATUS.md`에서 R06/E05를 미완료로 둔다.

## 계획 적대 검토에서 반영한 점

계획을 코드보다 먼저 다음 20개 서로 다른 실패·계약 관점으로 점검했다. 검토 중 `RuntimeSession::Drop`이 `shutdown()`을 호출해 worker를 join하는 기존 경로를 확인했으므로, 새 수명 정책을 만들지 않고 probe가 만든 보조 thread를 먼저 join한 뒤 마지막 owner를 놓도록 범위를 바로잡았다. 반영한 사항은 timeout cleanup, 별도 overflow 호출 thread, 실행 중 항목과 대기 용량의 구분, 고유 side effect 표식, 명시적인 재접수, 플랫폼별 실제 V8 반복, simulator/실기기 및 진단/성능 주장 분리다.

| # | 적대 관점 | 계획에서 확인한 안전 조건 |
| --- | --- | --- |
| 1 | 총 용량과 실행 중 명령의 경계 | 실행 중 1개와 대기 64개를 분리하고 queue 길이만 판정한다. |
| 2 | 용량 확인과 삽입 경합 | 현재 구현은 같은 mutex 아래 확인·삽입하며 probe는 실제 대기 깊이를 확인한다. |
| 3 | overflow 전 owner가 작업을 꺼내는 경합 | 동기 무한 JS가 active인 동안만 접수하므로 dequeue가 멈춰 있다. |
| 4 | blocker가 단순 대기 상태인 오인 | `RuntimeControl.active`를 확인하고 timeout이면 시도를 실패 처리한다. |
| 5 | 65번째 호출이 JS 완료를 기다리는지 | 별도 thread 결과를 blocker가 실행 중일 때 기다린다. |
| 6 | overflow 오류 코드 혼동 | 기대값은 구현의 정확한 내부 상태 `-5`; 다른 오류는 실패다. |
| 7 | 수락 수가 64개 미만인 경우 | queue 깊이와 수신한 성공 응답 수 모두 64를 요구한다. |
| 8 | 같은 priority FIFO 위반 | 고유 sequence와 JS 증가 표식 `1..64`를 순서대로 비교한다. |
| 9 | 중복·누락 표식 | 전체 결과를 정렬된 정확 일치로 판정한다. |
| 10 | 거부 command가 나중에 실행되는 경우 | 전역 overflow side effect 후속 조회가 미설정 상태여야 한다. |
| 11 | cancel이 blocker보다 늦거나 다른 명령을 취소 | 65번째 admission 결과 확인 뒤 blocker를 취소하고 status `-8`을 요구한다. |
| 12 | 수락 명령이 cancel 때문에 같이 폐기되는 경우 | 64개 응답이 모두 `OK`여야 한다. |
| 13 | 응답 채널 누락·trace handoff 누수 | 각 pending 응답을 timeout으로 받고 성공·실패 모두 handoff를 마친다. |
| 14 | 큐 drain 오인 | 64개 응답 뒤 새 command를 넣고 global accepted count를 확인한다. |
| 15 | drain 후 세션이 복구되지 않는 경우 | 후속 JavaScript가 정상 완료되어야 한다. |
| 16 | callback이 owner 이외 thread에서 실행되는 경우 | 모든 accepted 응답에서 owner/callback TID 동일성을 확인한다. |
| 17 | timeout·thread 생성 실패 cleanup | blocker를 중단하고 producer를 회수하며 session의 기존 `Drop` join을 유지한다. |
| 18 | 플랫폼 harness·결과 파서가 다른 경우 | 같은 FFI report schema와 양 플랫폼의 동일한 pass marker를 사용한다. |
| 19 | 환경 차이로 결과가 섞이는 경우 | V8 revision/JIT 설정을 확인하고 플랫폼별 simulator 로그·캡처를 분리한다. |
| 20 | 진단을 제품 정책·성능으로 확대 해석 | 문서에 내부 admission 관찰만 기록하고 R06/E05 및 제품 계약은 미완료로 둔다. |

## 구현 적대 검토 결과

아래는 구현·단위 테스트·플랫폼 실행·문서의 서로 다른 실패 경계를 최종 코드와 대조한 결과다. 검토 중 발견한 사항은 같은 표에서 수정 결과를 기록했다.

| # | 적대 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 시작 전 실행 중인 작업 | `active`가 꺼질 때까지 제한 대기하며, 남은 대기 명령이 있으면 포화를 시작하지 않도록 queue-empty 검사를 추가했다. |
| 2 | blocker가 실제 V8에서 실행 중인지 | V8 owner의 active 상태를 확인한 다음에만 제출을 시작한다. |
| 3 | 실행 중 명령과 대기 용량 혼동 | 대기 queue 길이를 별도 잠금으로 확인해 정확히 64인지 판정한다. |
| 4 | dequeue와 포화 측정 경합 | blocker는 동기 무한 JS이므로 취소 전에는 owner가 다음 명령을 dequeue할 수 없다. |
| 5 | 동시 제출 중 용량 검사 경쟁 | 기존 `try_enqueue()`의 mutex 안 용량 검사·삽입 경계를 사용하고 별도 우회 경로를 추가하지 않았다. |
| 6 | 제출 65건 전체가 실제 완료됐는지 | 64개 admission은 비동기 응답 receiver를 보관하고, overflow 호출은 별도 thread에서 수행한다. |
| 7 | blocker 종료 뒤에야 overflow가 반환하는지 | overflow 응답을 받은 뒤 blocker를 취소하며, timeout이면 시도를 실패 처리한다. |
| 8 | 오류 코드가 다른 실패를 숨기는지 | 65번째 상태가 현재 내부 상수 `ERR_QUEUE_FULL`(`-5`)와 정확히 같아야 통과한다. |
| 9 | 거부 후 queue 변형 | overflow 응답을 받은 직후 queue 깊이가 64인지 재확인한다. 테스트의 eager `then_some` unwrap도 lazy 경로로 고쳐 거부 결과에서 테스트가 panic하지 않게 했다. |
| 10 | 거부 항목의 지연 실행 | overflow 전역 표식을 후속 JS에서 검사하고 `true`이면 실패한다. |
| 11 | 취소 대상이 blocker인지 | overflow 거부 확인 전 취소하지 않으며 blocker 응답 `-8`과 cancel 반환 `0`을 모두 요구한다. |
| 12 | 취소가 수락된 명령까지 잃게 하는지 | 수락된 64개 각각의 응답을 파싱하고 성공 상태만 허용한다. |
| 13 | 중복·누락·역전 FIFO | marker `1..64`, 연속 실행 순서, 증가 sequence를 각각 대조한다. fake runtime oracle의 dispatch 항목도 실제 node ID를 기록하도록 수정했다. |
| 14 | owner/callback thread 이탈 | 모든 응답의 두 TID가 0이 아닌 동일 owner TID인지 확인한다. |
| 15 | 응답·trace handoff 누락 | 정상 응답은 소비 직후 handoff를 끝내고, 오류 경로는 제한 응답 수집 및 receiver drop으로 종료시킨다. |
| 16 | timeout에 무한 join이 숨어 있는지 | blocker와 overflow helper 모두 제한 join을 사용한다. 이전 무제한 overflow `join()`을 제한 join으로 수정했다. |
| 17 | 실패 경로에서 안전한 thread 종료를 과장하는지 | timeout 시 handle detach와 실패를 기록하며 OS thread 강제 종료를 주장하지 않도록 계획을 수정했다. |
| 18 | 응답만으로 queue가 비었다고 오인하는지 | 수락 응답 수집 뒤 실제 queue 길이 0을 확인하고 새 eval을 제출한다. |
| 19 | drain 뒤 세션이 실제로 회복되는지 | 새 eval의 성공, node id 64, recovery owner/callback TID 일치 및 마지막 queue 0을 확인한다. |
| 20 | 플랫폼·제품 계약 범위 혼동 | 실제 V8 Android/iOS Simulator 각 5회, 빌드 설정·화면·로그를 보존하고 R06/E05와 제품 역압력 정책은 미완료로 둔다. |

이 검토는 정적 계약 점검과 실행 결과를 교차 확인한 기록이다. iOS 실기기, Android 실기기에서의 이 진단, release 성능, OS thread 강제 종료는 이 실행의 통과 범위가 아니다.

## 미해결 경계

현재 `64`와 `-5`는 구현 상태를 확인할 기준값이지 제품 계약 결정이 아니다. 우선순위별 예약 용량, adapter의 선행 큐, 병합·폐기·재시도, 대기자 알림, 실제 메모리 예산, 무한 유입 공정성, iOS 실기기 및 release 성능은 이 작업이 결정하지 않는다.
