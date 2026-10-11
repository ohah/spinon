# C12.4 동시 세션 worker stack 비용 검증 계획

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md)  
**검증 대상:** [실제 layout worker 깊이 검증](../spec/internal/evidence/c12-4-layout-depth-safety-platforms-2026-10-11.md)에서 남긴 동시 session 비용  
**기준 구현:** `RuntimeSession::new_runtime_gpu_block_flex_stacking_fixture()`와 `RuntimeUaCascadeCoordinator`  
**계획 실패 관점 검토:** [20개 계획 검토](../spec/internal/evidence/c12-4-concurrent-session-stack-cost-plan-review-2026-10-11.md)  
**상태:** iOS 26.2 Simulator·Android API 37 AVD의 0·1·2·4·8 process-level 측정과 CSS worker stack bounds/mincore 표본 완료 · 실기기 RSS와 worker별 commit/high-water는 미측정 · [원본 및 결과 요약](../spec/internal/evidence/c12-4-concurrent-session-stack-cost-2026-10-11.md) · [worker 표본](../spec/internal/evidence/c12-4-worker-stack-residency-2026-10-11/README.md)

## 질문과 판정 경계

C12.4 profile로 실제 `RuntimeSession`을 0·1·2·4·8개까지 같은 앱 프로세스에서 누적 생성하고, 각 단계에서 살아 있는 session 수와 프로세스 메모리를 관측한다. 모든 session에는 V8 전용 실행 thread와 CSS cascade worker가 생긴다. CSS worker는 코드에서 8 MiB stack을 요청하므로 고정 virtual stack reservation은 session당 8 MiB, 8개 단계에서 64 MiB다. 이는 OS가 그만큼 물리 메모리를 commit했다는 뜻이 아니다.

- iOS 26.2 iPhone 17 Pro Simulator와 Android API 37 AVD만 사용한다. Android 실기기는 사용하지 않는다.
- renderer/surface는 만들지 않는다. 측정하려는 대상은 session 및 그 worker들의 누적 비용이다.
- 단계별 프로세스 전체 메모리, 프로세스 시작 후 baseline 대비 증가, CSS worker의 설정 stack reservation을 각각 분리해 기록한다. 프로세스 증가분은 V8 heap·thread·allocator·UIKit/ART 등도 포함하므로 CSS worker에 귀속하지 않는다.
- iOS `footprint`/`vmmap`과 Android `dumpsys meminfo`/`/proc` 결과는 각 도구의 원래 metric 이름대로 저장한다. PSS, physical footprint, resident/virtual size를 서로 바꾸어 부르지 않는다.
- 한 플랫폼 결과를 다른 플랫폼, 실기기 RSS 또는 제품 성능으로 일반화하지 않는다. 이 계획의 표는 process-level memory다. worker별 pthread range와 순간 mincore 표본은 [별도 계획](c12-4-worker-stack-residency.md)의 완료된 결과로 보되 process-level 값을 소급해 stack에 귀속하지 않는다.

## 실행 구성

1. 검증 전용 앱 실행 인수를 추가해 일반 C12.4 화면 초기화 대신 session probe를 구동한다. 기존 C12.4 host 생성/free FFI를 재사용하고 제품 API나 기본 실행 경로는 바꾸지 않는다.
2. 프로세스가 안정된 뒤 session 0개 baseline을 기록한다. 이어 1·2·4·8개까지 누적 생성하며 단계마다 고유 marker, 실제 생성 개수, 요청한 CSS worker 예약 합계, 시간표시를 남긴다.
3. 각 단계에서 최소 2초간 session을 모두 살려 두고 외부 수집기가 프로세스 메모리를 두 번 이상 읽는다. 샘플 편차를 포함해 원본 출력 전체를 보존한다.
4. 생성 실패 시 그 단계에서 중단하고 이미 만든 handle을 모두 해제한다. 8개를 넘지 않는다. 앱 강제 종료, OOM, 누락 stage marker는 해당 플랫폼 실행 실패로 남긴다.
5. 8개 단계 후 모든 handle을 해제하고 안정화 단계 메모리를 기록해 회수 여부를 확인한다. 프로세스 allocator나 OS가 메모리를 즉시 반환하지 않을 수 있으므로 감소하지 않아도 누수라고 단정하지 않는다.
6. 두 시뮬레이터에서 앱을 새 프로세스로 실행하고 각각 독립 로그·메타데이터를 기록한다. 결과 비교는 session 수별 관측값과 예약 합계만 맞춘다.

## 완료 기준

- [x] iOS Simulator에서 0·1·2·4·8 동시 session을 누적 생성하고 모든 handle을 해제했다.
- [x] Android API 37 AVD에서 같은 단계를 누적 생성하고 모든 handle을 해제했다.
- [x] 각 단계의 프로세스 전체 memory metric과 원본 도구 출력을 보존했다. 단계마다 세 번 측정했다.
- [x] CSS worker의 8 MiB stack 요청 합계와 process-level 관측값을 구분했다.
- [x] 실제 생성 및 정상 해제 경로를 확인하고 구현 실패 검토·상태 문서를 추가했다. 강제 생성 실패/부분 생성 negative injection은 실행하지 않았다.
- [x] 결과를 Simulator/AVD 관측으로 한정하고 실기기 RSS, thread별 stack page commitment, 장기 생존 session 회수, 제품 최대 동시 session 수를 미확정으로 구분했다.

이번 완료 표시는 Simulator/AVD에서 실행한 0–8 session process-level 측정 범위에 한정한다. thread별 pthread 범위와 순간 resident-page 표본은 별도 검증을 완료했지만, worker stack-only RSS/commit/high-water, 실기기 결과와 global allocator OOM 복구는 남는다.

