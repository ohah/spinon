# 구현 실패 경로 검토

검토 대상은 [Android debug lifecycle fixture](../../../../platforms/android/app/src/debug/java/dev/spinon/bootstrap/R05AppliedTransactionLifecycleFixture.java), [R05 callback probe](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/R05PresentFenceProbe.java), [R08 GLES surface](../../../../platforms/android/app/src/main/java/dev/spinon/bootstrap/R08GpuDemo.java), runner, 최종 API 37.2 실행 로그다. 구현 전 계획의 검토 표와는 별도다.

| # | 독립 실패 관점 | 확인한 보호·증거 | 결과 |
|---:|---|---|---|
| 1 | API 35 미만에서 API 35 transaction 경로를 실행 | fixture가 `MIN_API`를 확인하고 runner는 API 37.2 full version을 요구한다. | 실행 전 거부 조건 유지 |
| 2 | 다른 AVD·페이지 크기를 이번 대상으로 혼동 | serial, API full, 16KB page size를 실행 전에 검사하고 기록한다. | API 37.2 / 16,384 확인 |
| 3 | 삭제된 AVD를 fresh boot 또는 새 상태로 주장 | 결과 시점 `.ini`, 사라진 data directory, QEMU PID, 삭제 파일 open handles를 기록한다. | 기존 live process 사용, cold boot 아님 |
| 4 | 다른 V8 커밋 산출물로 앱을 빌드 | 고정 revision과 clean checkout 상태를 environment에 기록하고 APK/source digest를 보존한다. | pinned revision 일치 |
| 5 | 합성 callback을 실제 framework callback으로 오인 | fixture는 callback 함수를 직접 부르지 않고 R08 `submitNextFrame` 경로의 OS listener를 기다린다. | `TransactionStats` 실제 callback 확인 |
| 6 | 입력 뒤 R08 draw가 없는데 frame 적용을 주장 | runner가 revision 1/gen 1, revision 2/gen 1, revision 3/gen 2의 R08 draw marker를 모두 요구한다. | 세 marker 통과 |
| 7 | transaction listener 없이 완료 callback만 관찰 | `SPINON_R05_PRESENT_FENCE_LISTENER` 및 실제 `applyTransactionToFrame` log, callback stats를 함께 확인한다. | listener 등록·적용·callback 연결 |
| 8 | blocker가 다른 executor를 막음 | blocker는 `executeCallbackForFailureFixture`로 production callback pool에 게시되고 active count 1을 요구한다. | 동일 pool blocker 확인 |
| 9 | 다른 request의 queue를 lifecycle request로 오인 | renderer·surface generation·input sequence tuple로 pending request를 찾아 callback count와 timer를 확인한다. | request 108–110 흐름 일치 |
| 10 | surface 파괴 전에 callback이 실행되어 취소 경합이 무효 | 파괴 전에 queue 1, active 1, request pending, callback count 0, callback 누계 불변을 요구한다. | 대기열 진입 뒤 파괴 확인 |
| 11 | 기다림이 main thread를 막아 timeout·surface callback을 조작 | latch 대기와 polling은 fixture worker에서 한다. main thread 작업은 짧은 `Handler` action으로 보낸다. | UI thread 대기 없음 |
| 12 | SurfaceView view detach 없이 취소 함수를 직접 호출 | 실제 parent에서 같은 `R08GpuSurface`를 제거하고 holder callback의 destroy 통지를 기다린다. | 실제 destroy 관찰 |
| 13 | fixture가 production 취소 경로를 건너뜀 | `R08GpuSurface.onR05SurfaceDestroyed`의 `cancelForSurface` 뒤 상태·timer를 확인한다. | production 취소 경로 통과 |
| 14 | 새 view가 같은 generation을 재사용하거나 원래 view가 복구되지 않음 | 같은 instance identity, 기존 parent 재부착, child 포함 여부, generation 증가를 모두 요구한다. | 동일 view, generation 1→2 |
| 15 | timeout이 cancellation보다 먼저 승리 | destroy 뒤 취소 상태·map 부재·timer 제거와 timeout count 0을 검사한다. | 취소가 먼저 완료 |
| 16 | callback이 null stats·중복 호출·inline overflow인데도 통과 | callback count 1, stats 존재, `callbackInlineOverflow=false`를 정상·stale 양성 경로에서 요구한다. | 실제 callback 각 1회 |
| 17 | 취소된 callback이 새 surface 성공으로 되살아남 | 같은 request callback 결과가 `late_after_cancel`, `current_surface=false`, unusable로 남는지 확인한다. | stale callback 오분류 없음 |
| 18 | fence signal 상태를 callback 완료 또는 latency로 확대 | fence state와 request outcome을 별도 저장한다. fence signal은 합격 요건이 아니며 stale callback의 signaled fence도 unusable이다. | 표시 시각·광학 scanout 주장 없음 |
| 19 | surface recreate 뒤 executor가 복구되지 않음 | 새 generation에서 별도 실제 양성 callback을 확인하고 pending·timer·queue·active 정리와 60초 idle을 검사한다. | 복구·drain·idle 통과 |
| 20 | 실행 결과가 다른 APK·소스·AVD 실행과 뒤섞임 | run별 환경, process PID, APK/source SHA-256, 전체 Logcat과 screenshot을 저장하고 runner가 필수 scenario/draw marker를 거부 조건으로 둔다. | 최종 run 자료 연결 |

## 검토 중 수정한 결함

- request lookup이 실패한 cleanup 경로에서 null request 취소를 호출할 수 있어 null guard를 추가했다.
- 동일 view 검사는 객체 identity 비교만으로는 항상 참이 될 수 있어, 재부착 parent와 child 관계도 확인하도록 바꿨다.
- 두 번째 합성 `MotionEvent` 생성 실패 시 첫 이벤트를 회수하지 못할 수 있어 nullable local과 `finally` 정리를 추가했다.
- actual callback의 executor inline-overflow 상태를 검사해 일반 callback과 queue overflow fallback을 구분한다.
- runner의 AVD console 응답에서 이름과 `OK`를 분리하고, 프로세스 PID·삭제 data directory·열린 삭제 파일을 기록하도록 고쳤다.
- R08 draw marker를 합격 조건에 넣고, ripgrep 옵션 파싱 오류를 수정해 최종 runner에서 확인했다.
- `source_worktree_dirty`는 tracked 파일만 세고 있어 이름을 `source_tracked_changes`로 정확히 했다.

이 수정 뒤 `git diff --check`, runner `bash -n`, Android debug build, API 37.2 lifecycle fixture를 다시 실행했다. 최종 fixture summary는 12 checks, 0 failures, 100회 timeout/callback 경합, 60초 idle, pending/queue/active 0이다. 이 검토는 Android API 37.2 debug fixture 범위이며 다른 API·실기기·제품 event-to-present 동작은 포함하지 않는다.

## 요청에 따른 재검토 · 2026-10-08

앞의 20개 실패 관점을 현재 소스에 다시 대조했다. 최종 실행의 SHA-256 manifest 8개 항목이 현재 소스와 모두 일치하고, 동일 API 37.2 AVD에서 runner를 한 번 더 실행했다. 새 실행에서도 baseline 실제 callback, surface 재생성 뒤 stale callback, 새 generation recovery가 각각 통과했고 timeout/callback 경합 100회, 60초 idle, pending/queue/active 0이었다. [재실행 환경](api37_2_adversarial-rerun/environment.txt), [전체 로그](api37_2_adversarial-rerun/logcat.txt), [화면](api37_2_adversarial-rerun/result.png).

이 20개는 같은 동작을 20번 반복한 횟수가 아니라 서로 다른 결함 경로를 점검한 수다. 실제 동시성 경합은 fixture가 각 runner 실행에서 100회 반복한다. iOS는 `SurfaceControl.TransactionStats`와 동일한 API를 제공하지 않으므로 이 Android lifecycle fixture의 iOS 통과로 간주하지 않는다. 별도 iOS R05 callback-ledger 자체 시험은 6개 그룹을 20/20회 재실행했고, Simulator UI tap smoke는 1/1 통과했다. 상세 자료는 [iOS companion 검증](README.md#ios-별도-회귀-확인)에 기록되어 있다.
