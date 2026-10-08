# 구현·실행 실패 관점 검토

범위는 Android debug-only async present-fence observer, release source-set stub, Android 16.1 실기기 3-block 실행이다. 서로 다른 실패 경계 20개를 코드·compile 결과·process별 원본 log로 대조했다.

| # | 공격 관점 | 확인 결과와 경계 |
|---:|---|---|
| 1 | API 35 미만 기기에서 `SyncFence` 복사 API가 로드되는가 | API 35 gate 뒤 nested helper에서만 사용한다. API 29–35 runtime 행렬은 이번 실기기 작업에 포함되지 않았다. |
| 2 | release APK에서 실험 경로가 활성화되는가 | release source-set stub이 `enable()`을 거부하고 호출부를 제공한다. release Java compile은 통과했지만 release APK runtime은 실행하지 않았다. |
| 3 | 일반 debug 실행에서 observer가 자동으로 켜지는가 | 별도 intent가 없으면 `enable()`을 부르지 않으며 기본 상태는 비활성이다. 일반 debug negative-control 실행은 하지 않았다. |
| 4 | GLES·S04·R13 경로에서 WGPU fence라고 잘못 처리하는가 | 기존 mode conflict와 WGPU fence mode를 그대로 통과해야 한다. 별도 conflict intent 실행은 이번 run에서 하지 않았다. |
| 5 | fence 대기가 UI thread에서 실행되는가 | thread ID를 비교해 UI·callback·waiter가 세 block 모두 서로 다름을 확인했다. |
| 6 | 기존 transaction callback executor를 await로 막는가 | callback은 fence 복제 후 bounded queue에 제출하고 반환한다. 실제 callback과 worker의 TID가 분리됐고 33개 입력이 진행됐다. |
| 7 | `TransactionStats` 반환 후 framework가 닫은 원본 fence를 복사하는가 | 복사는 callback 반환 전, 원본 close의 `finally` 전에 실행한다. 세 block에서 33개 복제 wait가 완료됐다. |
| 8 | 원본 fence와 복제 fence 소유권이 충돌하는가 | 기존 probe가 원본을 닫고 worker가 복제본을 닫는다. async 결과 33개 모두 `fence_close_error=none`이다. |
| 9 | 복사 생성자·검증에서 예외가 나면 복제본이 남는가 | enqueue 전 예외는 로컬 소유 handle을 닫는다. 실기기에서는 복사/검증 오류가 없었다. 오류 주입은 하지 않았다. |
| 10 | queue rejection이 callback thread에서 inline 실행되는가 | `AbortPolicy`이며 inline fallback은 없다. 이번 저부하 실행은 33개 모두 접수돼 rejection 경로 자체는 실행하지 않았다. |
| 11 | pending counter가 reject 또는 enqueue 예외 뒤 증가 상태로 남는가 | 제출 실패 시 등록 카운터를 되돌리고 worker 완료 때 감소한다. 각 block 마지막에 pending 0이었다. |
| 12 | queue·worker 수가 입력량에 비례해 무한 증가하는가 | worker 2개, queue 64개로 제한된다. 포화 한계점은 이번 물리 run에서 의도적으로 시험하지 않았다. |
| 13 | 대기 시간이 무한하거나 callback timeout과 혼동되는가 | worker의 각 `await`는 250ms 상한이다. 이번 33개 모두 timeout 없이 signal됐다. |
| 14 | `await()==true`만으로 표본을 성공 처리하는가 | valid 전후 상태, 양수·유한 signal timestamp, 관측시각, latch 순서를 함께 검사한다. 33개 모두 검사를 통과했다. |
| 15 | `SIGNAL_TIME_PENDING`·invalid·0·미래 시각을 성공으로 센다 | 분류기에서 각각 실패 상태로 분리한다. 이번 worker 결과에는 그런 상태가 없었다. |
| 16 | uptime timestamp와 `CLOCK_MONOTONIC` signal을 빼서 지연값을 만드는가 | 두 시계 값을 결과 필드로 보존할 뿐 event→present 차이는 계산하지 않는다. report도 이를 latency로 부르지 않는다. |
| 17 | 중복·late callback이 한 입력을 두 번 성공시킬 수 있는가 | first timely callback만 enqueue하고 request ID·input sequence·revision을 보존했다. 중복/late 주입은 별도로 수행하지 않았다. |
| 18 | surface가 stale 또는 generation이 바뀐 뒤 결과를 새 입력에 귀속하는가 | callback 시점 current-surface와 generation을 확인한다. 물리 surface destroy/recreate 경합은 이 run에서 시험하지 않았다. |
| 19 | Activity 종료가 새 작업을 계속 넣거나 in-flight fence를 누수시키는가 | `onDestroy`에서 enqueue를 비활성화하고 이미 접수한 task는 자체 close한다. 세 앱은 처리 종료 후 명시적 force-stop됐고 OS exit reason은 USER REQUESTED다. 강제 종료 중 pending 유실은 별도 검증 범위다. |
| 20 | foreground 이탈·logcat 혼합·overlay 캡처를 성공 결과에 섞는가 | foreground가 아니던 최초 block은 제외했다. 유효 3 block은 새 process별 logcat을 분리했고, 앱 외 overlay가 담긴 캡처 하나는 폐기했다. global logcat buffer는 지우지 않았다. |

최종 debug build와 release Java compile은 통과했다. build 초기에 편집 중인 Java 파일의 닫는 괄호 오류가 발견되어 고친 뒤 최종 APK를 다시 빌드했다. 마지막 세 block은 각각 input/submit/callback/usable async signal 11/11이며 종료 counter는 0이었다. 미검증 범위는 위 각 행에 적은 API 호환·negative control·포화·고장 주입·surface lifecycle·release runtime이다.
