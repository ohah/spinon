# R05.3 Android 표시 callback 실패 주입 계획

**상태:** 계획 검토·API 37.2 에뮬레이터 fixture 실행 완료 · Android API 범위 제한 · R05.3 미완료

## 목적

현재 `R05PresentFenceProbe`의 정상 transaction callback과 포화 경로는 실행 근거가 있지만, fence probe 자체의 timeout·late callback·취소·중복·stale generation·timeout 경합은 코드 검사 또는 offline 판정기 부정 대조에 머물러 있다. 별도 JankData callback queue의 포화 결과를 transaction-completion callback의 근거로 재사용하지 않는다.

이 계획은 실제 OS callback을 받은 것처럼 꾸미지 않고, Android 35 이상 에뮬레이터에서 다음을 분리 확인한다.

1. 실제 `SurfaceControl.Transaction`에 completion listener를 등록하고 transaction을 적용하지 않아 callback이 없는 timeout을 유도한다. 정해진 기한에 실제 main `Handler` timeout이 실행되는지 확인한다.
2. 같은 요청 상태 전이 함수를 직접 구동해 timeout 뒤 late callback, cancel 뒤 callback, duplicate callback, stale generation, `cancelForSurface` 동작을 확인한다. 합성 callback에는 실제 `TransactionStats`나 valid fence가 없으므로 표시 성공 표본으로 세지 않는다.
3. 실제 callback executor의 bounded queue에서 용량 초과가 caller-runs inline으로 실행되고 `fence_signal_usable=false`에 반영되는지 확인한다. 막은 worker는 모든 종료 경로에서 풀고, queue drain을 제한 시간 안에 확인한다.
4. timeout·callback 원자 상태 경합을 반복해 단 하나의 terminal state와 pending map 정리를 확인한다. 이어 60초 idle 관찰에서 pending request·callback queue가 계속 비어 있는지 확인한다.

이 항목은 R05.3 구현 실패 주입이다. 물리 입력, renderer 성능, 실제 화면 표시 시각, iOS device callback, 16KB 호환성 전체를 완료 처리하지 않는다. 실기기 조작은 사용자가 별도로 요청하기 전에는 하지 않는다.

## 비교 모델과 fixture

| 항목 | 기준 |
|---|---|
| 상태 전이 기준 | `R05PresentFenceProbe.Api35`: `PENDING → CALLBACK_RECEIVED / TIMED_OUT / CANCELLED`; terminal state는 callback으로 되살아나지 않음 |
| pending 소유권 | `(request_id, renderer, generation)`; surface 교체·취소 뒤 이전 generation callback은 성공 표본이 아님 |
| callback executor | 실제 production `ThreadPoolExecutor`: worker 1개, 대기 용량 64, 거부 시 caller-runs 기록 |
| 실제 timeout oracle | API 37.2 16KB ARM64 AVD에서 transaction에 listener를 붙이고 적용하지 않은 뒤 `CALLBACK_TIMEOUT_MS` 안에 `callback_timeout`이 한 번 기록되고 pending에서 제거됨. 공식 API는 callback을 transaction이 presented된 뒤 한 번 호출한다고 설명하므로, 미적용 transaction은 callback이 오지 않는다는 fixture 가정을 둔다. 실제 관찰이 다르면 실패로 남긴다. ([Android API](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer))) |
| 합성 callback oracle | null `TransactionStats`만 전달; 정상 callback 처리는 상태 전이만 확인하고 `fence_signal_usable=false`를 반드시 유지 |
| race oracle | timeout과 callback 동시 출발 후 terminal state가 정확히 하나, pending map 잔류 0; 승자는 어느 쪽이든 허용하고 성공 신호로 해석하지 않음 |
| queue oracle | 1개 worker를 막고 64개 대기 task 수락, 초과 1개는 호출자 thread에서 inline 실행·overflow 증가, worker release 후 queue와 active task가 0으로 복귀 |
| idle oracle | 종료된 fixture 이후 60초 동안 매초 pending request 0, queue depth 0, 새 callback/timeout 전이 0 |

Fixture는 Android debug APK에서만 명시적 intent로 실행한다. `BuildConfig.DEBUG == false`면 어떤 fault도 실행하지 않고 거부한다. API 35 미만에서는 API 35 class를 초기화하지 않고 `unavailable`로 끝낸다. 기본 앱 실행과 R08 화면 동작은 바꾸지 않는다.

## 실행 순서와 합격 기준

1. Android API 37.2 ARM64 16KB AVD, page size, 앱 revision, APK digest를 기록한다. API 36 ARM64 AVD도 fallback 대조에 사용한다. 연결된 실기기는 선택하지 않는다.
2. 기존 callback executor가 idle이고 pending map이 비어 있음을 fixture 시작 전 확인한다. 전제조건이 맞지 않으면 상태를 비우거나 기다려서 숨기지 않고 fixture를 실패시킨다.
3. debug intent에서 별도 fault fixture만 실행한다. scenario ID마다 request ID를 기록하고 기본 R08 화면·JS runtime benchmark는 실행하지 않는다.
4. 미적용 transaction의 실제 timeout을 기다리는 동안 UI thread를 차단하지 않는다. timeout 뒤 transaction을 한 번 닫고 실제 listener가 전달되지 않았는지 기록한다. 별도의 합성 request에는 timeout 전이를 직접 만든 뒤 late callback을 보내 `late_after_timeout`이며 terminal state가 유지되는지 확인한다. 두 경로의 callback을 섞지 않는다.
5. 취소·surface 취소·duplicate·stale-generation을 각각 독립 request로 실행하고 기대 state·pending 제거·timeout Runnable 제거를 검사한다.
6. timeout과 callback race를 최소 100회 실행한다. 각 회차는 barrier 이후 별도 worker에서 동시 출발하고 bounded join을 쓴다. timeout 또는 callback 중 한 terminal winner, pending 잔류 0, 유효 fence 없음이 합격이다.
7. production callback executor saturation fixture는 blocker와 queue fill을 `try/finally`로 감싼다. queue가 사전 비어 있지 않으면 중단한다. 초과 task가 inline에서 1회만 실행되고 `inline_callback_total`이 정확히 증가하며, worker release 및 bounded drain 뒤 queue/active 수가 0이어야 한다.
8. 모든 fixture가 정리된 뒤 60초 idle 상태를 관찰한다. pending·queue가 매초 0이고 새 callback/timeout이 없어야 한다. 이는 CPU·배터리 효율 판정이 아니다.
9. API 36 fallback 앱을 새 프로세스에서 실행해 일반 GLES 입력/표시 경로가 fault fixture 변경 없이 유지되는지 확인한다.
10. logcat과 시나리오 요약을 보존한다. 성공 표본·latency·pixel-present·제품 지원 문구는 생성하지 않는다.

정상 종료 기준은 모든 scenario가 기대 결과를 기록하고, fixture 종료 시 pending 0·queue 0·active 0이며, 앱 PID가 살아 있는 것이다. timeout, race join, worker release, API fallback 가운데 하나라도 실패하면 PR을 통과로 보지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 실패 관점 | 계획의 차단·판정 |
|---:|---|---|
| 1 | 합성 callback을 OS 표시 callback으로 오해 | 실제 timeout과 합성 상태 전이를 분리해 로그에 source를 붙이고, 합성 결과는 표시 성공으로 세지 않는다. |
| 2 | timeout을 직접 호출해 Handler 구현을 검증했다고 주장 | 미적용 실제 transaction listener에서 실제 main Handler timeout을 기다린다. |
| 3 | transaction을 적용해 예상치 못한 실제 callback이 발생 | fixture는 transaction을 적용하지 않으며 실제 callback이 timeout 전에 도착하면 timeout 주입 전제가 깨진 것으로 실패 처리한다. 실제 요청과 합성 late 요청은 분리한다. |
| 4 | UI thread에서 2초 대기해 timeout callback을 막음 | 기다림은 별도 worker에서 하고 Handler를 소유한 main loop는 계속 실행한다. |
| 5 | timeout 뒤 늦은 callback이 terminal state를 성공으로 복구 | 합성 timeout request에 후속 callback을 보내고 `late_after_timeout` 및 terminal 고정을 검사한다. 실제 미적용 transaction 요청과 합성 요청을 혼합하지 않는다. |
| 6 | 취소 뒤 callback이 pending map에 재등록 | cancel 후 callback을 보내고 state가 `CANCELLED`, map 부재인지 확인한다. |
| 7 | surface 취소가 renderer가 다른 요청까지 제거 | 같은 generation에서 다른 renderer 요청을 대조로 두고 `(renderer, generation)` 범위만 제거한다. |
| 8 | 이전 surface의 callback을 새 generation에 귀속 | current generation을 요청보다 크게 바꾸고 `current_surface=false`, usable false를 요구한다. |
| 9 | duplicate callback이 두 번째 성공 표본을 만듦 | 첫 callback 처리 후 동일 request callback을 재호출해 duplicate outcome 및 usable false를 검사한다. |
| 10 | null stats 또는 invalid fence를 usable로 오인 | callback observation에서 fence 없음과 `fence_signal_usable=false`를 각각 단언한다. |
| 11 | callback·timeout 경합에서 두 terminal 결과가 모두 집계 | atomic CAS winner 하나만 terminal이 되며 map 제거와 bounded join을 매 회차 확인한다. |
| 12 | race 테스트가 매번 사실상 순차 실행 | barrier에서 두 worker를 함께 풀고 실행별 시작/완료·winner를 기록한다. |
| 13 | 한 race winner가 pending map에 남음 | 회차별 terminal state와 pending map 부재를 함께 확인한다. |
| 14 | static callback pool이 이미 바쁜데 saturation을 시작 | 시작 전에 active/queue/pending이 모두 0이 아니면 fixture를 중단하고 기존 일을 버리지 않는다. |
| 15 | 포화 fixture assertion 실패로 worker가 영구 정지 | blocker 해제를 `finally`에 두고 release 및 종료 join을 제한 시간 안에 확인한다. |
| 16 | 초과 task가 caller-runs로 실행됐는데 overflow 표시 누락 | 실제 executor reject handler를 통해 호출자 thread ID, overflow flag, counter delta를 검사한다. |
| 17 | 64 queue 수락 수와 64 callback record를 혼동 | queue task 수와 callback record 수를 별도 필드로 기록하며 본 fixture는 executor task accounting만 주장한다. |
| 18 | terminal 처리 후 timeout Runnable이 남아 나중에 다시 깨어남 | request별 timeout 예약·취소·실행 횟수와 종료 후 `hasCallbacks` 부재를 확인한다. |
| 19 | 60초 idle 결과를 배터리·장기 안정성 보장으로 과장 | pending/queue/상태 전이 부재만 결론 내리고 CPU·배터리·장시간 제품 안정성은 미판정한다. |
| 20 | debug fault 진입이 release 또는 일반 화면으로 노출 | 명시적 extra와 `BuildConfig.DEBUG`를 함께 요구하고 release 전용 no-op stub의 compile을 확인했다. unsigned Release APK에서 intent 거부 runtime은 미실행이며, API36 GLES fallback의 기본 R08 화면·탭 동작은 별도 확인했다. |

검토에서 transaction timeout은 상태 함수 직접 호출만으로 검증할 수 없고, 실제 적용하지 않은 transaction의 공개 listener와 main Handler를 이용해야 한다고 고정했다. 실제 timeout request와 합성 late request를 분리해 listener의 예기치 않은 callback을 혼합하지 않도록 했다. 또한 stale surface test에는 다른 renderer의 보존 대조가 필요하며, queue 포화 결과를 callback record 손실량으로 확대하면 안 된다. 위 조건을 합격 기준에 반영했다. 이 계획 검토는 구현 후 별도 실패 관점 검토를 대신하지 않는다.

## 실행 증거와 상태 반영

- 실행 전 oracle·scenario 목록·기대 로그·미지원 범위는 이 문서와 고정된 AVD 정보로 유지한다.
- 실행 뒤 새 evidence 문서에 원본 로그, APK/소스 revision, AVD/API/page size, scenario별 기대·관찰, 제한을 기록하고 SHA-256 manifest를 만든다.
- `spec/STATUS.md` R05.3 설명과 이 계획을 함께 동기화한다. 전체 R05.3은 Android API 범위, iOS 표시 callback runtime, 사용자가 요청한 실기기 검증 등 남은 gate가 충족될 때까지 미완료다.
- allthatnba의 `spinon/` 미리보기에는 제안·진행·완료를 구분해 링크를 갱신한다. GitHub Pages를 배포하지 않는다.

계획에 대한 20개 관점 검토와 별개로 구현 완료 뒤 20개 구현 실패 경로를 다시 확인했다. Android 37.2 16KB AVD의 11 scenario는 모두 통과했고, API 36 GLES fallback과 Release compile도 확인했다. 상세 결과·20개 구현 검토·남은 API 경계는 [실행 보고서](../spec/internal/evidence/r05-android-callback-failure-injection-2026-10-08.md)에 있다. 이 실행은 R05.3 전체 완료 판정이 아니다.
