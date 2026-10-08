# R05.3 · Android 실기기 비동기 present fence 신호 확인 계획

**상태:** debug 진단 구현 및 Android 16.1 실기기 synthetic 실행 완료 · 기본 debug 경로 부정 대조와 release runtime 미실행 · release build는 V8 source checkout 부재로 중단 · R05.3 제품 계측 미완료
**상위 계획:** [입력→표시 신호 상관 계측](r05-input-to-presentation.md) · [R05 상태 대장](../spec/STATUS.md)

## 목적

2026-10-09 Samsung SM-S731N / Android 16 SDK 36.1 실기기에서 R08 WGPU SurfaceView를 대상으로 `adb shell input tap` 30회를 실행했다. 30회 모두 입력·revision 제출·`TransactionStats` callback이 연결됐고 fence descriptor도 유효했지만, callback 안에서 바로 읽은 `SyncFence.getSignalTime()`은 30회 모두 `SIGNAL_TIME_PENDING`이었다. 이번 경로는 callback 종료 직전에 fence를 닫으므로, 이 결과만으로 fence가 이후에도 영구히 pending이라고 결론 내릴 수 없다.

이 계획은 기존 callback 처리를 지연시키지 않고 fence handle을 복제해 별도 bounded worker에서 제한 시간 동안 기다린 뒤 신호 상태를 다시 읽는 진단 전용 실험을 정의한다. 목표는 다음 두 관찰을 구분하는 것이다.

1. `TransactionStats` callback 도착 시의 즉시 fence 상태.
2. callback 반환 뒤 복제 fence가 제한 시간 안에 실제로 signal되는지.

이는 Android R05 표시 신호 조사 전용이다. 공개 JS API, 제품 UI 동작, 일반 렌더링 경로, 실제 손가락 입력, 사용자 체감 지연, finger-to-photon 또는 플랫폼 성능 순위를 구현하거나 주장하지 않는다.

## 실기기 실행 결과

최종 debug APK를 Samsung SM-S731N / Android 16 SDK 36.1 실기기에 설치해 서로 다른 새 process 3개에서 각 10개 scored synthetic 탭과 1개 drain 탭을 실행했다. 33개 모두 input sequence·revision, R08 submit, 실제 `TransactionStats` callback 및 usable async fence signal이 1:1 연결됐다. callback 직후 fence 상태는 pending 32개·signaled 1개였고, 별도 worker의 bounded wait 결과는 33/33 signaled였다. 각 process 종료 시 pending/active/queue depth가 0이었다. 실제 finger touch·화면 scanout·event-to-present latency는 측정하지 않았다. 상세 로그·APK/source digest·화면 증거는 [실기기 실행 보고서](../spec/internal/evidence/r05-android-physical-present-fence-2026-10-09/async-wait/README.md)에 있다. 구현 및 실행 근거의 분리된 실패 관점 점검은 같은 보고서의 [구현 검토](../spec/internal/evidence/r05-android-physical-present-fence-2026-10-09/async-wait/implementation-review.md)를 따른다.

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

- debug-only wait 실험 경로, release source-set 실행 거부 stub, debug build 및 release Java compile. 기본 debug 경로 부정 대조와 release APK runtime은 아직 실행하지 않았다.
- 3개 독립 process block의 환경·digest·원본 logcat·screenshot·입력 protocol·checksum.
- immediate result와 bounded async wait result를 request 단위로 조인한 요약과 실패/누락 수.
- [상태 대장](../spec/STATUS.md) 및 [R05.3 계획](r05-input-to-presentation.md)의 미완료 경계를 갱신한다. 실기기 synthetic run만으로 실제 touch 또는 제품 latency를 완료 처리하지 않는다.

## 후속 검증 계획 · debug 기본 경로와 release 격리

2026-10-09 재검증에서 `mise exec -- env ANDROID_HOME=... ANDROID_SDK_ROOT=... ./gradlew --no-daemon :app:assembleRelease`로 Android SDK 경로 문제는 해소했으나 `:app:prepareSpinonBootstrap`가 고정 V8 source tree가 없어 실패했다. 작업 트리에 V8 source가 없음을 확인했으며 `tools/v8/checkout.sh`는 대규모 외부 source/dependency download와 생성 파일 변경을 시작할 수 있어 자동 실행하지 않았다. 따라서 release variant build·설치·runtime은 미실행으로 유지한다. [빌드 로그](../spec/internal/evidence/r05-android-release-isolation-2026-10-09/gradle-release-sdk-configured.log). 같은 날 debug APK를 실기기에서 다시 실행한 synthetic 입력 결과는 [별도 재검증 보고서](../spec/internal/evidence/r05-android-physical-input-join-2026-10-09/physical-retest/README.md)에 기록했다.

**상태:** 계획 수정 후 20관점 재검토 완료 · debug 기본 bootstrap·R08 probe-off 대조 실행 완료 · release runtime은 V8 source checkout 부재로 미실행

### 범위와 비교 조건

비교 대상은 같은 source/artifact의 네 조건이다. 이 앱의 기본 `MainActivity`는 R08 화면을 만들지 않고 bootstrap JS fixture만 실행하므로, 기본 실행과 R08 GPU 시각 대조를 별도 조건으로 둔다.

1. **debug 기본 bootstrap:** R05/R08 intent extra 없이 새 process를 시작한다. `SPINON_BOOTSTRAP_EXECUTION`과 `SPINON_BOOTSTRAP_RESULT`가 보여야 하고 `SPINON_R05_` 표식은 없어야 한다. 이 fixture는 화면 render report를 표시하지 않아 흰 화면이 정상이며, GPU 탭이 가능하다고 판정하지 않는다.
2. **debug R08 probe-off 시각 대조:** `spinon_r08=true`만 전달하고 모든 R05 extra를 생략한다. WGPU surface가 ready인 log와 화면을 확인한 뒤 synthetic ADB tap 한 번에 GPU 도형 색상이 바뀌어야 하며 `SPINON_R05_` 표식이 없어야 한다. 이는 R08 개발 fixture의 probe-off 대조이지 앱 기본 실행이나 물리 입력 검증이 아니다.
3. **release 기본 실행:** release variant를 시작해 bootstrap 결과가 나타나고 R05 fence-wait 표식이 없어야 한다.
4. **release에 debug intent 요청:** `spinon_r05_async_fence_wait=true`를 전달한다. release stub가 `reason=debug_only`로 거부하고 Activity를 닫아야 한다. 이 경로는 release production signing·배포 가능성 검증이 아니다.

네 조건을 각각 새 process/PID로 분리한다. 모든 ADB 입력은 synthetic 별도 그룹이며 physical input 또는 성능 표본으로 취급하지 않는다. release APK는 로컬 debug keystore로만 서명해 설치 smoke를 수행할 수 있으며, 그러면 **release build variant의 로컬 signed smoke**로만 보고하고 배포 서명 검증으로 부르지 않는다. 기존 debug APK는 현재 공식 APK hash와 일치하는 사본을 복원용으로 보존한다. `mise exec --`에서 고정된 JDK/Gradle 도구를 사용한다.

### 실행과 복구

- 시작 전에 공식 source tree SHA, 설치된 debug APK SHA, 연결된 Android 기기 1대, 화면 설정·foreground를 기록한다.
- `mise exec --`를 통해 release variant를 빌드한다. 서명·설치가 필요한 경우 test-only 복사본을 local debug key로 서명하고 unsigned 원본과 signed APK를 각각 hash한다.
- debug 기본 bootstrap, debug R08 probe-off, release 기본 실행, release의 명시적 debug-only 요청을 새 PID로 분리한다. PID-filtered logcat을 저장하고 각 모드에서 기대 표식이 있는지와 없는지를 모두 확인한다. global logcat buffer를 비우지 않는다.
- release 모드에서 debug flag가 거부된 뒤 Activity가 화면에서 끝났는지 확인한다. Android가 idle process를 남기는 것은 실패로 간주하지 않고 crash/ANR과 구분한다. 이후 원래 공식 debug APK를 설치하고 Spinon을 force-stop, Chrome foreground 복귀, 화면 설정 전후 동일성을 확인한다.
- 이 smoke는 R05 worker 기본 경로 격리만 확인한다. release 성능·스토어 signing·R05.3 완료는 주장하지 않는다.

### 계획 적대 검토 · 독립 실패 관점 20개

| # | 실패 관점 | 계획의 방어·판정 |
|---:|---|---|
| 1 | debug 기본 실행에 이전 intent extra가 남음 | 매 조건을 새 process와 명시적인 intent extra 집합으로 시작한다. |
| 2 | 단순 앱 시작 실패를 marker 부정 대조 통과로 처리 | Activity launch 성공·PID·정상 기본 화면을 먼저 확인한다. |
| 3 | release source set이 아닌 debug APK를 release로 오인 | variant, APK 산출 경로, manifest, SHA-256을 모두 기록한다. |
| 4 | release 로그 부재를 `debug_only` 거부로 간주 | 요청 경로에서는 stub의 명시적인 `reason=debug_only` 로그를 합격 조건으로 둔다. |
| 5 | intent key 오타로 release extra가 전달되지 않음 | 실제 명령의 key와 debug source `MainActivity` 수신 key를 대조한다. |
| 6 | release 거부 종료를 crash로 오인 | process exit, crash buffer, ANR 및 `debug_only` marker를 함께 분류한다. |
| 7 | debug-only 거부 뒤 Activity가 남아 intent가 재사용되거나 화면이 열린 채 유지 | Activity가 finish되어 전면 화면에서 사라졌는지 확인한다. Android idle process는 남을 수 있어 process 종료를 합격 조건으로 요구하지 않는다. |
| 8 | release stub가 async worker를 참조해 class-load 오류 발생 | release default launch와 debug-only request를 분리 실행하고 linker/class verification 오류를 검사한다. |
| 9 | Gradle daemon이 다른 JDK로 빌드 | `mise exec -- java -version` 및 Gradle runtime을 evidence에 기록한다. |
| 10 | release build 과정이 고정 V8 artifact/source를 바꿈 | 시작 전 worktree status와 V8 revision을 기록하고 generated input diff를 검사한다. |
| 11 | unsigned APK를 설치 실패 후 release runtime 성공으로 과장 | unsigned build 결과와 설치 가능한 signed test copy를 분리 기록한다. |
| 12 | local debug signing을 production signing으로 오인 | 서명 key를 test-only라 명시하고 production release signing은 검증 범위 밖으로 둔다. |
| 13 | release 서명 mismatch로 기존 debug 앱 덮어쓰기 실패 | install 전에 현재 package signer와 test APK signer compatibility를 확인하고 debug APK 사본을 복원 가능하게 보존한다. |
| 14 | test package 데이터/기존 debug 앱을 불필요하게 제거 | `adb install -r`를 우선하고 uninstall은 하지 않는다. 설치 충돌이면 중단·기록한다. |
| 15 | renderer surface가 준비되기 전에 tap을 전송 | release 화면·surface ready를 확인한 뒤 synthetic tap 한 건만 보낸다. |
| 16 | R05 marker 필터가 main/debug/release tag 차이를 놓침 | package PID로 전체 app log를 수집한 뒤 모든 `SPINON_R05_` 접두사와 전용 start/result를 검사한다. |
| 17 | debug worker marker가 다른 PID의 stale log에 섞임 | PID-filtered logcat과 process start time을 이용하고 global buffer를 지우지 않는다. |
| 18 | Activity finish 뒤의 정상 idle process를 crash/ANR로 오인 | `dumpsys activity`의 resumed/visible Activity, crash buffer와 ANR을 따로 확인한다. process 생존만으로 crash를 판정하지 않는다. |
| 19 | device settings·Chrome 복귀 누락 | 밝기·timeout·stay-on·refresh·focus를 before/after 비교하고 debug APK 복원 뒤 Chrome을 foreground로 연다. |
| 20 | release smoke 결과로 성능·배포 안전·R05.3 완료 주장 | variant isolation smoke로만 기록하며 production key·release performance·physical input·VSync/scanout은 미검증으로 남긴다. |

20개 계획 관점을 각각 검토했다. 이 표는 runtime 결과가 아니며 실행 후에는 실제 APK·설치·process·log를 대상으로 별도의 새로운 20개 검토를 작성한다.

### 계획 수정 후 재검토 · 독립 실패 관점 20개

기본 bootstrap과 R08 visual fixture를 한 화면으로 가정했던 전제를 실제 MainActivity 경로와 실기기 화면으로 대조한 뒤 계획을 고쳤다. 아래 재검토는 수정된 네 조건 각각의 오분류 경계를 다룬다.

| # | 실패 관점 | 수정된 방어·판정 |
|---:|---|---|
| 1 | 이전 Activity intent extra가 새 기본 실행에 잔류 | 매 시나리오 force-stop 후 새 PID에서 전달한 extra를 명시적으로 기록한다. |
| 2 | 기본 bootstrap 흰 화면을 앱 launch 실패로 오판 | `SPINON_BOOTSTRAP_EXECUTION`과 fixture result를 확인하며 이 화면에서 시각 UI를 기대하지 않는다. |
| 3 | Activity 생존만으로 bootstrap JS 실행을 성공 처리 | worker 실행 marker와 `SPINON_BOOTSTRAP_RESULT` 모두를 요구한다. |
| 4 | R08 probe-off 실행에 fence flag가 암묵적으로 포함 | `spinon_r08=true` 외 R05 extra가 없는 시작 명령과 `SPINON_R05_` marker 부재를 함께 확인한다. |
| 5 | R08 surface가 준비되기 전 입력 | `SPINON_R08_WGPU=ready`, surface 크기, 전경 Activity를 확인한 뒤 한 tap을 보낸다. |
| 6 | 보이는 도형과 실제 GPU backend가 다름 | 화면 문구와 PID log의 backend/device marker를 맞춘다. |
| 7 | ADB tap을 물리 touch로 오분류 | synthetic을 명시하고 direct touchscreen raw event가 없으면 physical sample을 0으로 둔다. |
| 8 | R05 marker 부재가 다른 PID를 필터해 생긴 거짓 음성 | launch 전에 PID를 확보해 process log 전체를 수집한다. |
| 9 | 기본 모드에서 이전 debug APK가 실행 | 기기 설치 APK hash를 공식 baseline과 대조한다. |
| 10 | 흰 화면 캡처를 render pass 통과로 과장 | 기본 bootstrap 결과는 DOM fixture 처리만 증명하고 화면 pixel rendering 결과는 증명하지 않는다고 고정한다. |
| 11 | release build 실패를 release runtime 통과로 오인 | variant build가 실제 성공하기 전에는 설치·런타임 결과를 쓰지 않는다. |
| 12 | V8 source checkout 부재를 SDK 문제로 혼동 | SDK 경로와 Gradle preBuild 실패 지점을 각각 기록한다. |
| 13 | 검증 중 임의 V8 checkout/download로 disk·shared build 상태 변경 | pinned checkout이 없으면 원본을 생성·덮어쓰지 않고 release 실행을 보류한다. |
| 14 | release debug-only 요청 key가 source와 불일치 | intent key를 debug MainActivity와 release stub에서 교차 확인한다. |
| 15 | release stub 부재를 로그 없음으로 추정 | 요청 경로에서 `reason=debug_only`의 명시 marker를 합격 조건으로 둔다. |
| 16 | release finish를 process 종료로만 판정 | visible/resumed Activity 부재를 확인하고 정상 idle process는 허용한다. |
| 17 | 서명 충돌 해결을 위해 기존 앱 데이터를 삭제 | signer 확인 및 `install -r`만 허용하고 uninstall은 하지 않는다. |
| 18 | 테스트 종료 뒤 밝기·refresh·Chrome 상태가 변함 | 화면 설정과 focus를 전후 기록하고 Spinon을 종료해 Chrome을 복귀시킨다. |
| 19 | 앱 crash/ANR 확인 범위를 전체 OS log로 과장 | PID-filtered app log 및 Activity 상태에서 확인한 범위만 보고한다. |
| 20 | 두 debug 대조만으로 release 격리 또는 R05.3 완료 선언 | debug 두 조건만 부분 통과로 기록하고 release 두 조건·physical input·VSync/scanout은 미완료로 유지한다. |

수정된 계획의 20개 관점을 재검토했다. 실행 결과는 [Android 실기기 debug isolation 보고서](../spec/internal/evidence/r05-android-release-isolation-2026-10-09/README.md)에 별도로 기록하고 새 runtime 관점 검토를 붙인다.
