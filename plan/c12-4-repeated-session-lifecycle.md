# C12.4 반복 runtime session 생성·해제 검증

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md) · [단발 동시 session 측정](c12-4-concurrent-session-stack-cost.md)
**상태:** Android 실기기 현재 소스 APK로 16 cycle × 8 session(128개) 재실행 완료 · 각 cycle의 V8 owner/CSS worker 생성·종료 후 재생성 확인 · [최신 실행 근거](../spec/internal/evidence/c12-4-repeated-session-lifecycle-2026-10-12/c124-lifecycle-20261012T003625959Z-77816319/README.md) · [구현 검토](../spec/internal/evidence/c12-4-repeated-session-lifecycle-implementation-review-2026-10-12.md)
**대상:** Android debug fixture에서 같은 앱 process 안에 C12.4 `RuntimeSession`을 반복 생성하고 종료한다.

## 목적과 범위

기존 0→1→2→4→8→0 한 회 측정은 동시 session 생성·종료 경로를 확인했지만, 같은 process에서 여러 차례 반복할 때 다음 cycle 생성이 계속 가능한지와 종료한 V8 owner/CSS worker thread가 남는지는 확인하지 않았다. 이 계획은 Android 실기기 한 대에서 이 수명 경계를 검증한다.

- 검증 전용 Activity intent로만 실행하고, 일반 앱 진입·JS API·release 동작에 노출하지 않는다.
- 새 process 하나에서 16 cycle을 순서대로 실행한다. cycle마다 C12.4 RuntimeSession 8개를 만들고, 각 CSS worker stack residency 진단으로 CSS worker TID를 기록한 뒤 역순 해제한다.
- cycle의 새 session은 이전 cycle의 모든 owner/CSS worker TID가 사라진 다음에 만든다. Android `/proc/<pid>/task/<tid>` 부재를 앱 UID `run-as`로 확인한다.
- 각 cycle은 고유 번호와 create/ready/free/complete marker를 남긴다. 생성 실패, 표본 오류, 해제 뒤 남은 thread, process 종료, 시간 초과는 실패로 판정한다. 부분 생성은 만들어진 handle을 역순으로 모두 해제한다.
- 앱 시작 뒤 baseline을 읽을 3초를 둔다. cycle 1·8·16은 peak memory 수집용으로 5초, 나머지 cycle은 1초 유지한다. 해제 뒤 1.5초 동안 다음 cycle을 시작하지 않아 수집기가 이전 TID 종료를 확인할 수 있게 한다.
- 실행은 renderer·surface·layout evaluation을 만들지 않는다. 검증 대상은 V8 owner와 CSS cascade worker의 생성·종료·재생성이다. CSS 결과·GPU·화면·공식 WPT 적합성을 검증하지 않는다.
- PSS/RSS는 cycle 시작·peak 8 session·최종 해제 시점의 process 전체 관측으로만 보존한다. allocator cache나 런타임 예약 때문에 값이 baseline으로 돌아오지 않아도 누수로 단정하지 않고, 수치만으로 누수가 없다고 판정하지도 않는다.
- 대상은 Android SM-S731N 한 대와 debug APK 한 revision이다. 다른 기기·OS·release build로 일반화하지 않는다.

## 제한과 안전장치

- cycle 수는 코드에서 16으로 고정하고 session 동시 수는 8로 제한한다. 총 생성 상한은 128 session이며 동적 intent 입력으로 늘리지 않는다.
- 앱 process가 살아 있는 동안에만 결과를 인정한다. force-stop, crash, 생성 실패 또는 marker 누락은 해당 run 실패다.
- 종료 확인은 native free 반환만으로 통과시키지 않는다. 생성 시 기록한 V8 owner TID와 CSS worker TID 모두 `/proc` task 경로에서 사라지는 것을 확인한다.
- thread ID는 Android가 재사용할 수 있으므로 서로 다른 cycle 사이에 전역 고유성을 요구하지 않는다. 대신 각 cycle의 create 이전/ready 때 존재/해제 뒤 부재를 시간 순서로 확인한다.
- `stack_and_tls` RSS/PSS와 process PSS/RSS를 thread의 stack-only committed bytes나 high-water mark로 부르지 않는다.
- 전역 allocator OOM 복구를 일부러 유발하지 않는다. 이번 반복 검증은 결정적 session 생성 실패 API도 제공하지 않으므로 부분 생성 cleanup은 별도의 host-creation failure injection 항목으로 남긴다.

## 결과 판정

- [x] 단일 앱 process에서 16 cycle 모두 8 session 생성·해제가 성공한다.
- [x] 각 cycle의 V8 owner/CSS worker TID 수가 각각 8이고, 중복·누락이 없다.
- [x] 각 cycle 해제 뒤 해당 TID가 모두 사라지고, 다음 cycle이 새 session을 성공적으로 만든다.
- [x] 정상 종료 marker·process 생존·오류 없는 수집을 확인한다.
- [x] baseline·cycle 1/8/16 peak·final process memory 원본과 APK/device hash를 보존하되 leak 결론과 분리한다.
- [x] 로그 parser의 누락 cycle, 중복 TID, 남은 TID, 잘못된 run ID와 실패 marker 거부를 자동 검사한다.
- [x] 결과를 `spec/STATUS.md`, C12.4 계획, 내부 근거 index 및 Tailnet preview에 동기화했다.

통과해도 Android 한 기기에서 session thread 수명만 검증한 것이다. stack-only high-water, 장시간 제품 부하, Android 기기군, iOS 실기기, 전역 allocator OOM 복구, WPT 적합성 및 C12.4 전체 완료를 뜻하지 않는다.

## 최신 현재 소스 재실행 · 2026-10-12

Android API 37 AVD에서 16 cycle × 8 session 반복을 실행하고 매 cycle owner/CSS worker TID 종료를 확인했다. process PSS/RSS는 앱 전체 관측값이므로 leak-free 증거가 아니다. 이 AVD 결과를 이전 Android 실기기 run과 합산하지 않는다. [전체 요약](../spec/internal/evidence/c12-4-follow-up-verification-2026-10-12/README.md) · [최신 AVD lifecycle run](../spec/internal/evidence/c12-4-repeated-session-lifecycle-2026-10-12/c124-lifecycle-20261012T065858648Z-9152b856/README.md).

## 2026-10-19 Android API 37 AVD 재검증

기존 Android 실기기 결과와 분리해 `emulator-5554` (Android 17/API 37 AVD)에서 16 cycle × 8 session, 총 128회 생성·해제를 다시 실행했다. 각 cycle의 V8 owner/CSS worker thread 8개가 ready 시 존재하고 release 이후 사라진 것을 확인했다. 실행기는 선택한 serial만 조회하도록 ADB preflight를 좁혔다. 실패 두 건은 ADB가 아니라, 첫째 V8 pin mismatch, 둘째 PR #328 전의 오래된 JNI C symbol 참조에서 난 Android build failure였다. [실패 원인·AVD 근거](../spec/internal/evidence/c12-4-repeated-session-lifecycle-2026-10-19/README.md).

이 검증은 session thread 생성·종료 수명만 다룬다. Android AVD 외 기기, renderer/GPU/layout/WPT, process memory 기반 leak 결론, worker stack-only commit/high-water 및 전역 allocator OOM 복구는 다루지 않는다. 공식 C12.4 parent와 공개 CSS/API 지원 상태는 계속 미완료다.
