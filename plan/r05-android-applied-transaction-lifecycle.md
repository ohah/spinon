# R05.3 · Android 적용 transaction의 표면 수명주기 callback 검증 계획

**상태:** 계획 검토 완료 · debug fixture 구현 및 API 37.2 emulator 실행 완료 · R05.3 미완료
**상위 항목:** [R05 입력→표시 신호 계획](r05-input-to-presentation.md) · [구현 상태 대장](../spec/STATUS.md) · [Android callback 실패 주입](r05-android-callback-failure-injection.md)

## 목적과 범위

기존 Android failure fixture는 실제 `SurfaceControl.Transaction`에 listener를 등록하고 적용하지 않은 채 timeout·취소를 검증했다. 나머지 surface-generation·late callback 경로는 synthetic request에 callback 함수를 직접 호출했다. 이 계획은 실제 R08 `SurfaceView`가 렌더한 frame에 transaction을 연결하고 Android가 전달한 실제 `TransactionStats` callback이 표면 파괴·재생성 중 callback executor에 대기하는 상황을 검증한다.

확인할 질문은 두 가지다.

1. 정상 SurfaceView frame에 transaction을 적용하면 실제 Android transaction-completed listener가 현재 surface generation에서 한 번 전달되고 executor가 복구되는가?
2. 실제 callback task가 전용 executor에 예약된 뒤 `surfaceDestroyed`가 발생하면 기존 generation 요청이 취소되고, 새 generation의 표면이 만들어진 다음 callback worker를 풀어도 늦은 callback이 현재 화면 성공으로 오인되지 않는가?

이 검증은 callback·surface lifecycle 내부 fixture다. GPU presentation fence의 유효성·신호 timestamp, 제품 입력 지연, 물리 입력, GPU 성능, iOS 동작을 주장하지 않는다. 전체 R05.3/R05를 완료 처리하지 않는다.

## 비교 모델과 불변 조건

- Android 공식 `SurfaceControl.Transaction.addTransactionCompletedListener` 계약에서 listener는 transaction이 표시된 뒤 한 번 호출되고 callback은 지정한 `Executor`로 전달된다.
- `SurfaceView.applyTransactionToFrame`은 다음 SurfaceView frame과 transaction을 함께 적용하며 시스템이 transaction 소유권을 넘겨받는다. 정확한 다음 frame을 지정하려면 호출 전에 연속 렌더링을 멈추고 호출 뒤 한 frame만 제출한다. 새 frame이 없으면 transaction은 적용되지 않을 수 있다.
- `SurfaceHolder.Callback.surfaceDestroyed`/`surfaceCreated`가 실제 view surface 경계를 정의한다. surface callback 동안 기존 surface를 향한 render를 멈추고, surface 파괴 handler는 production `R08GpuSurface`가 쓰는 `R05PresentFenceProbe.cancelForSurface(renderer, generation, ...)` 경로를 호출한다.
- callback의 `TransactionStats`는 실제 OS callback에서 얻은 객체여야 한다. request id·input sequence·revision·surface generation으로 요청을 식별한다. `callback_received`와 `fence_signal_usable`은 별도 결과이며, 이 fixture는 fence가 유효하거나 특정 시각을 가져야 한다고 요구하지 않는다.
- 취소 뒤 도착한 실제 callback은 `late_after_cancel`, `current_surface=false`, `fence_signal_usable=false`로 끝나야 한다. pending request·timeout·executor 작업은 최종 0이어야 한다. 새 generation의 양성 대조 callback은 정상 처리되어야 한다.
- fixture 입력은 `MotionEvent` 테스트 주입이며 `input_source=synthetic_fixture`로 표기한다. 물리 입력 및 event→present latency 표본으로 세지 않는다.

공식 비교 자료: [SurfaceView.applyTransactionToFrame](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction)) · [SurfaceControl.Transaction.addTransactionCompletedListener](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer)) · [SurfaceControl.TransactionStats](https://developer.android.com/reference/android/view/SurfaceControl.TransactionStats) · [SurfaceHolder.Callback](https://developer.android.com/reference/android/view/SurfaceHolder.Callback)

## 실행 절차와 판정

대상은 debug Android API 37.2 ARM64 emulator 한 대이며 기존 R08 OpenGL ES SurfaceView 경로를 재사용한다. emulator API, guest page size, process PID, APK·source digest, generation, request id, 입력 sequence, callback executor 상태와 screenshot을 보존한다. 이번 환경에서 연결된 API 37.2 process는 backing AVD directory가 제거된 채 실행 중이므로 새 emulator boot를 근거로 주장하지 않고 그 조건을 결과에 명시한다. 새 AVD를 만들거나 실행 중 emulator를 종료하지 않는다.

1. 현재 failure fixture precondition처럼 pending map과 callback executor가 모두 idle인지 확인한다.
2. R08 SurfaceView가 준비되고 첫 buffer를 제출했는지 확인한다. `RENDERMODE_WHEN_DIRTY` 상태에서 실제 R08 touch→revision submit 경로로 단일 frame을 만든다.
3. **정상 양성 대조:** callback worker를 막지 않고 실제 transaction callback을 기다린다. 요청 generation이 현재이고 callback count가 1이며 `TransactionStats`가 null이 아니고 pending/timer가 제거되어야 한다. fence signal 사용 여부는 기록만 하고 합격 조건에서 제외한다.
4. SurfaceView의 transaction callback executor에 bounded blocker를 넣고 실제 R08 touch 하나를 전달한다. listener 등록·transaction apply·frame submit을 기록하고 Android callback task가 blocker 뒤 queue에 들어온 것을 확인한다. main thread는 latch에서 기다리지 않는다.
5. callback task를 실행하기 전에 동일 `R08GpuSurface` view를 detach해 실제 `surfaceDestroyed` callback을 발생시킨다. production 취소 경로 완료, 이전 generation pending 제거, timeout 제거를 확인하고 같은 view를 다시 attach하여 generation 증가를 확인한다. callback은 이 lifecycle 구간 동안 test blocker가 계속 잡아둔다.
6. blocker를 반드시 `finally`에서 해제한다. Android가 준 `TransactionStats` callback이 실제로 실행되어 이전 request callback count 1, state cancelled, outcome `late_after_cancel`, current surface false, usable fence false임을 확인한다. 새 generation에 추가 입력을 한 번 전달해 정상 callback과 executor 복구를 확인한다.
7. 제한 시간 안에 listener task가 예약되지 않으면 timeout 또는 setup failure로 기록하고 PASS로 바꾸지 않는다. queue·pending·active·timeout이 남으면 fixture 실패다. 화면 캡처는 최종 결과 상태를 보여야 한다.

## 중단·실패 처리

- surface 미준비, API mismatch, runner serial 오류, debug route 이외의 앱 진입이면 fixture 시작 전 실패한다.
- callback executor를 점유한 뒤 어떤 assertion·surface callback이 실패해도 `finally`에서 blocker를 해제하고 제한 시간 동안 callback worker를 drain한다.
- transaction을 `applyTransactionToFrame`에 넘긴 뒤 fixture가 transaction을 닫거나 재사용하지 않는다. callback이 받은 fence는 기존 production handler에서만 읽고 닫는다.
- 정상 callback이 surface destruction보다 먼저 실행되면 lifecycle stale-callback scenario는 실패 처리한다. `callback_received` 한 건을 취소 경합 성공으로 인정하지 않는다.
- 실제 callback에서 `TransactionStats == null`, request 식별 불일치, 중복 callback, generation 역행, timeout이 cancellation보다 먼저 승리, 앱 재시작·ANR, callback 누락, worker 잔류는 실패로 남긴다.
- 한 API·한 emulator의 debug fixture 결과로 API 35·37.0·37.1, 실기기, release, GLES/WGPU 비교 우열을 추론하지 않는다.

## 산출물과 완료 기준

- API 37.2 실행 log·환경·APK/source hash·스크린샷을 저장한다.
- 정상 실제 callback, 실제 callback이 queue에 든 뒤의 surface destroy/recreate, 새 generation 복구 결과를 별도 scenario로 기록한다.
- 이 계획의 검증을 추가해도 `spec/STATUS.md`의 R05.3은 미완료로 유지한다.
- 다음 잔여 gate는 API 29–34 signal fallback, API 35/37.0/37.1에서 실제 적용 transaction/lifecycle, iOS device callback runtime·clock residual, device 측정 등이다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 공격 관점 | 계획의 차단·판정 |
|---:|---|---|
| 1 | synthetic callback을 OS의 실제 TransactionStats로 오인 | R08 실제 submit 경로와 callback count·stats availability를 확인하고 fixture function 직접 호출은 이 시나리오에 쓰지 않는다. |
| 2 | callback 완료를 present-fence 유효성·신호로 과장 | callback state와 fence state를 별도 기록하며 fence signal은 합격 조건에서 제외한다. |
| 3 | transaction을 붙인 frame이 지정되지 않음 | 연속 렌더를 끄고 `applyTransactionToFrame` 다음에 R08의 buffer 한 개를 제출한다. |
| 4 | frame이 제출되지 않았는데 callback 부재를 lifecycle 취소 성공으로 셈 | 실제 R08 draw/submit log와 executor queue 예약을 취소 전에 요구한다. |
| 5 | callback task queue 확인을 다른 task로 오인 | 시작 시 queue=0·active=0을 검증하고 단일 blocker 이후 callback 예약 증가와 request sequence를 함께 확인한다. |
| 6 | timeout이 취소보다 먼저 이겨 stale callback 검증이 무효 | surface 전환 시간을 제한하고 취소 시점 state를 먼저 저장하며 timeout 승리는 별도 실패다. |
| 7 | 테스트가 main thread를 막아 surface callback 또는 timeout을 유발 | 모든 대기·poll은 fixture worker에서 수행하고 UI handler는 빠르게 완료한다. |
| 8 | surface destroy 전에 callback worker가 이미 실행 | blocker 실행 상태·callback count 0·queue에 listener 작업 존재를 확인한 뒤 detach한다. |
| 9 | 다른 view 인스턴스를 만들어 generation이 초기화되어 증가를 가장 | 같은 `R08GpuSurface` 인스턴스를 detach/reattach하고 generation 단조 증가를 검사한다. |
| 10 | surface destroy 취소 경로를 production과 다르게 수동 호출 | `R08GpuSurface` 자체 `SurfaceHolder.Callback.surfaceDestroyed`가 production `cancelForSurface` 호출을 실행하게 한다. |
| 11 | renderer/세대가 다른 request를 잘못 취소 | request를 renderer·generation·input sequence 조합으로 추적하고 sibling request 격리는 별도 기존 scenario로 유지한다. |
| 12 | 새 surface callback이 이전 ticket을 current로 되살림 | callback에서 old request generation과 새 generation을 비교해 `current_surface=false` 및 unusable을 요구한다. |
| 13 | stale callback이 pending map이나 timer를 다시 채움 | callback 뒤 request identity 부재, timeout callback 없음, 최종 pending 0을 확인한다. |
| 14 | Android가 전달한 stats가 null이거나 중복이어도 실제 경로 통과 | non-null stats와 callback count 1을 요구하며 duplicate·null은 실패다. |
| 15 | SurfaceView가 transaction을 소유한 뒤 앱에서 중복 close | apply 이후 fixture는 transaction을 close/reuse하지 않고 system ownership을 기록한다. |
| 16 | fence를 callback 처리 뒤 닫지 않거나 두 번 닫음 | production handler의 close error를 기록하고 callback handler error 및 반복 close 흔적을 검사한다. |
| 17 | 테스트 중 assertion 예외로 executor worker가 영구 block | 모든 blocker를 `finally`에서 해제하고 bounded drain·active/queue 0을 성공 조건에 둔다. |
| 18 | 재생성 뒤 callback 경로가 망가져도 stale scenario만 통과 | 새 generation에서 정상 actual callback을 추가 실행해 same process executor 복구를 증명한다. |
| 19 | 스크린샷이 이전 화면·중간 상태인데 결과로 제출 | summary PASS를 기다린 뒤 화면을 결과 상태로 전환하고 PID와 screenshot을 저장한다. |
| 20 | API37.2 emulator 결과를 platform support·제품 latency로 확대 | API/boot/process 조건을 고정하고 R05.3, device/release/API matrix/fence/latency는 미완료로 유지한다. |

각 항목을 Android 공개 API 계약, 기존 R08 SurfaceView submit/destroy 경로, callback failure fixture의 pending·timeout·executor invariants와 대조했다. 이 표는 계획 전용 검토이며 실제 실행 성공을 뜻하지 않는다. 실제 코드 검토는 별도 [구현 실패 경로 검토](../spec/internal/evidence/r05-android-applied-transaction-lifecycle-2026-10-08/implementation-review.md)에 둔다.

## 실행 결과

API 37.2 ARM64 16KB emulator에서 actual R08 GLES `SurfaceView` transaction callback의 정상 경로, callback executor에 들어간 뒤 surface destroy/recreate를 거친 stale callback, 새 generation 회복 경로를 모두 확인했다. Debug fixture 12 checks와 기존 100회 timeout/callback 경합, 60초 idle 관찰은 통과했다. 실제 callback 세 건은 Android `TransactionStats`를 제공했고, 이전 generation 요청은 `late_after_cancel` 및 `current_surface=false`로 끝났다. run별 APK·source digest와 `SPINON_R05_DRAW` revision marker를 보존했다.

검증 시점의 AVD data directory는 이미 삭제된 상태였고 QEMU PID 10605가 열린 backing file로 실행 중이었다. 기존 emulator process를 사용했으며 fresh boot로 주장하지 않는다. 입력은 synthetic `MotionEvent`이고 fence signal 관찰은 event-to-present 지연이나 광학 표시 증거로 취급하지 않는다. 상세 원본과 제한은 [R05.3 실행 보고서](../spec/internal/evidence/r05-android-applied-transaction-lifecycle-2026-10-08/README.md)에 있다. API matrix 확장, 실기기, iOS callback runtime, 전체 R05.3은 미완료다.

추가 교차 플랫폼 회귀 확인으로 iOS R05 callback-ledger 자체 시험의 6개 그룹을 20회 재실행했고 20/20 통과했다. iPhone 17 Pro / iOS 26.2 Simulator의 WGPU drawable-acquire tap XCTest도 1/1 통과했다. iOS는 Android `TransactionStats` lifecycle과 다른 계약이며 이 결과는 Android 경로를 대체하거나 R05.3 전체를 완료 처리하지 않는다. [iOS 재실행 근거](../spec/internal/evidence/r05-android-applied-transaction-lifecycle-2026-10-08/README.md#ios-별도-회귀-확인).
