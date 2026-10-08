# R05.3 · Android 실기기 비동기 present fence 신호 확인 계획

**상태:** 계획 검토 완료 · 구현/실행 전
**상위 계획:** [입력→표시 신호 상관 계측](r05-input-to-presentation.md) · [R05 상태 대장](../spec/STATUS.md)

## 목적

2026-10-09 Samsung SM-S731N / Android 16 SDK 36.1 실기기에서 R08 WGPU SurfaceView를 대상으로 `adb shell input tap` 30회를 실행했다. 30회 모두 입력·revision 제출·`TransactionStats` callback이 연결됐고 fence descriptor도 유효했지만, callback 안에서 바로 읽은 `SyncFence.getSignalTime()`은 30회 모두 `SIGNAL_TIME_PENDING`이었다. 이번 경로는 callback 종료 직전에 fence를 닫으므로, 이 결과만으로 fence가 이후에도 영구히 pending이라고 결론 내릴 수 없다.

이 계획은 기존 callback 처리를 지연시키지 않고 fence handle을 복제해 별도 bounded worker에서 제한 시간 동안 기다린 뒤 신호 상태를 다시 읽는 진단 전용 실험을 정의한다. 목표는 다음 두 관찰을 구분하는 것이다.

1. `TransactionStats` callback 도착 시의 즉시 fence 상태.
2. callback 반환 뒤 복제 fence가 제한 시간 안에 실제로 signal되는지.

이는 Android R05 표시 신호 조사 전용이다. 공개 JS API, 제품 UI 동작, 일반 렌더링 경로, 실제 손가락 입력, 사용자 체감 지연, finger-to-photon 또는 플랫폼 성능 순위를 구현하거나 주장하지 않는다.

## 비교 모델과 실행 조건

- 기존 비교 기준은 같은 커밋의 현재 `R05PresentFenceProbe` 즉시 읽기 결과다. Android 공식 `SyncFence` 계약에 따라 `getSignalTime()`은 아직 신호되지 않은 fence에서 `SIGNAL_TIME_PENDING`을 반환하고, `await(Duration)`는 지정한 시간만 기다린다. API 35부터 `SyncFence(SyncFence)` 복사 생성자가 제공되며 복제본과 원본은 각각 닫아야 한다. [SyncFence 공식 API](https://developer.android.com/reference/android/hardware/SyncFence) · [TransactionStats 공식 API](https://developer.android.com/reference/android/view/SurfaceControl.TransactionStats)
- 현재 [`R05PresentFenceProbe`](../platforms/android/app/src/main/java/dev/spinon/bootstrap/R05PresentFenceProbe.java)는 callback executor를 worker 하나와 queue 64개로 만들고, 포화 시 `InlineOnRejectHandler`가 callback 작업을 실행자 thread에서 실행할 수 있다. callback에서 `await`하면 단일 worker head-of-line blocking 또는 rejection 시 inline blocking이 생기므로, signal waiter는 독립 executor여야 한다. 같은 파일은 즉시 읽은 `SyncFence`를 `finally`에서 닫고, framework가 listener 반환 뒤 `TransactionStats.close()`를 호출한다고 명시한다.
- 실험 대상은 Android API 36 이상 debug 빌드의 R08 WGPU/Vulkan SurfaceView다. 새 경로는 명시적인 debug intent extra로만 켠다. 기본 앱 경로와 release source set은 동작하지 않아야 한다.
- callback에서 원본 `TransactionStats`와 fence를 기존처럼 정리한다. 복제본은 callback이 반환되기 전에 만든 뒤 별도 bounded worker queue에 넘기고, worker의 모든 종료 경로에서 정확히 한 번 닫는다.
- wait는 Android UI thread와 transaction callback executor 밖에서 실행한다. 각 복제 fence는 최대 250ms만 기다린다. 별도 worker는 2개, queue는 64개로 고정한다. 포화 시 작업을 inline 실행하거나 표본을 성공 처리하지 않고 복제본을 닫아 `rejected`로 기록한다.
- 결과는 `request_id`, `input_seq`, `revision`, `surface_generation`을 그대로 보존한다. 결과 시각은 Android `CLOCK_MONOTONIC` 원시값과 `await` 경과 시간으로 기록한다. callback 시점의 fence 상태와 비동기 결과를 별도 필드로 남긴다.
- 실기기 실행은 서로 새 PID를 쓰는 3개 block으로 나누고, 각 block은 10개 scored 입력과 callback drain용 1개 unscored 입력을 사용한다. 탭 간격은 450ms다. 입력은 `adb shell input tap`으로 생성된 synthetic Android `MotionEvent`이며 실제 손가락 입력 표본이 아니다.
- 기존에 선택된 display mode·밝기·화면 timeout·USB 설정을 바꾸지 않는다. 시작 전후 값을 기록하고, global logcat buffer를 비우지 않으며, ADB serial은 공개 evidence에서 제외한다.

## 합격 및 중단 기준

- 각 scored 입력은 고유 input sequence/revision의 R08 submit, 한 건의 실제 transaction callback, 한 건의 async wait 결과로 연결되어야 한다.
- `await`의 `true`만으로 성공 처리하지 않는다. 복제 fence가 계속 유효하고 `getSignalTime()`이 양수이며 `SIGNAL_TIME_INVALID`/`SIGNAL_TIME_PENDING`이 아니고, 관측 시각보다 미래가 아니어야 `signaled`로 분류한다. latch 시각이 signal 시각보다 늦으면 유효 timestamp로 취급하지 않는다.
- `await`가 250ms 안에 끝나지 않으면 `timed_out`; queue 포화는 `rejected`; 복제·읽기·닫기 실패는 별도 실패로 유지한다. 누락·중복·timeout 표본을 다른 성공 표본으로 대체하지 않는다.
- 입력 source가 synthetic이므로 event-to-present 지연 수치나 성능 비교를 산출하지 않는다. Fence가 signal돼도 R05.3 완료로 표시하지 않는다.
- device/app이 foreground를 잃거나 API·renderer·APK/source digest가 예상과 다르면 해당 block은 무효다. 앱 ANR/crash, callback 또는 worker 잔류, 설정 변동, 캡처 누락이 있으면 중단하고 실패 자료를 보존한다.

## 계획 적대 검토 · 독립 실패 관점 20개

계획과 Android API의 복제·대기·신호 시각 계약, 기존 transaction callback의 자원 수명을 서로 다른 실패 경계로 대조했다. 아래 발견을 계획에 반영했다. 이 표는 계획 검토이며 코드 구현 또는 실기기 실행 결과를 뜻하지 않는다.

| # | 실패 관점 | 계획에 반영한 차단 기준 |
|---:|---|---|
| 1 | callback 반환 후 framework가 닫은 fence를 계속 사용 | framework 반환 전에 API35 복사 생성자로 별도 handle을 만든다. |
| 2 | 원본과 복제본 소유권을 혼동해 원본을 이중 close | 원본은 기존 callback 경로, 복제본은 worker가 각각 한 번 닫는다. |
| 3 | API 35 미만에서 없는 복사 생성자를 호출 | debug intent·SDK 조건을 모두 통과한 경로에서만 복사한다. |
| 4 | `await`의 API 지원 범위와 앱 minSdk를 혼동 | present-fence probe 가능 API에서만 debug 실험을 실행한다. |
| 5 | invalid fence의 `await()` 반환 true를 signal로 오인 | 대기 전후 `isValid()` 및 `getSignalTime()`을 함께 판정한다. |
| 6 | 무제한 `awaitForever()`로 worker를 영구 점유 | 모든 wait에 고정 250ms timeout을 사용한다. |
| 7 | UI thread에서 fence를 기다려 입력·화면을 정지 | 별도 worker에서만 wait하고 UI 경로에는 blocking join을 두지 않는다. |
| 8 | transaction callback executor를 대기로 포화 | callback에서는 즉시 상태 수집·복제·enqueue 후 반환한다. |
| 9 | wait worker 무제한 증가 또는 queue 메모리 무제한 성장 | worker 2개와 queue 64개로 제한한다. |
| 10 | queue rejection 때 callback thread에서 inline 대기 | rejection은 `rejected` 결과로 남기고 복제 handle을 즉시 닫는다. |
| 11 | executor queue에 제출 전후 pending counter가 어긋남 | accepted·rejected·completed를 별도 결과로 남겨 0 잔류를 확인한다. |
| 12 | 예외·timeout·rejection에서 복제 fence 누수 | 복제 뒤 enqueue 실패 및 worker의 모든 종료 경로를 `finally` close로 통일한다. |
| 13 | `await()==true`만으로 signal 시각을 생성 | invalid/pending/non-positive/future 및 latch 순서 위반을 성공에서 제외한다. |
| 14 | callback 중복 또는 늦은 callback이 같은 입력을 두 번 집계 | 첫 timely callback 한 건만 실험 queue에 넣고 duplicate/late를 성공 표본에서 제외한다. |
| 15 | 비동기 worker가 종료된 Activity/View를 오래 붙잡음 | 요청 식별자와 필요한 immutable clock/state snapshot만 넘기며 View를 캡처하지 않는다. |
| 16 | surface 재생성 전후 generation을 섞음 | request의 renderer·generation·revision을 결과에 복사하고 callback 당시 surface 상태를 기록한다. |
| 17 | pending 상태를 “GPU가 완료되지 않음”으로 과장 | pending은 그 관측 시점의 API 반환값으로만 서술하고 이후 결과와 구분한다. |
| 18 | `CLOCK_MONOTONIC` 신호 값을 uptime과 직접 차감 | 시간 domain 필드를 보존하며 이 synthetic run에서는 지연값을 계산하지 않는다. |
| 19 | ADB tap을 사람 touch 또는 물리 입력 timing으로 취급 | 입력 출처를 실행 manifest에서 synthetic으로 고정하고 실제 touch gate를 미완료로 유지한다. |
| 20 | 화면·빌드·기기 조건 불일치 또는 환경 변경을 숨김 | APK/source digest, USB/API/ABI/display·focus·설정 before/after와 캡처를 보존한다. |

## 산출물

- debug-only wait 실험 경로와 기본/release 경로 부정 대조.
- 3개 독립 process block의 환경·digest·원본 logcat·screenshot·입력 protocol·checksum.
- immediate result와 bounded async wait result를 request 단위로 조인한 요약과 실패/누락 수.
- [상태 대장](../spec/STATUS.md) 및 [R05.3 계획](r05-input-to-presentation.md)의 미완료 경계를 갱신한다. 실기기 synthetic run만으로 실제 touch 또는 제품 latency를 완료 처리하지 않는다.
