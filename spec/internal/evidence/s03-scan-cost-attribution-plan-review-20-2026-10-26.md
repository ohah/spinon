# S03.3 registry scan 비용 분석 계획 검토 · 독립 실패 관점 20개

**계획:** [`plan/s03-3-scan-cost-attribution.md`](../../../plan/s03-3-scan-cost-attribution.md)
**비교 기준:** [`s03-scan-cost-attribution-precomparison-2026-10-26/README.md`](s03-scan-cost-attribution-precomparison-2026-10-26/README.md)
**검토 기준:** 구현·계측 코드를 작성하기 전 측정 타당성, 수명 계약, 실패 경계, 재현성
**검토 대상 base:** `origin/main` `28696fbfef552892f851113ed8e0d84c638edad5`; PR 준비 전 변경 경로가 분리된 `86b9cd9125f01f5d1277db2d2e525a5498688d70`로 rebase했다.
**검토 계획 SHA-256:** `445f2e83bc98142594808911944df624ea0fd071a815ab186afa976fa2794d81`
**결과:** 계획 단계 P1/P2 미해결 없음. 구현 전 결정이 필요한 target·예산·반복 수는 계획의 명시적 gate로 남겼다.

| # | 독립 실패 관점 | 판정 및 계획 조치 |
|---:|---|---|
| 1 | outer scan time과 그 안의 Rust/V8 phase time을 더해 중복 계산하는가 | total wall과 exclusive phase boundaries를 분리하고, 미계측 구간과 timer 오차를 남긴다. phase 합이 outer wall과 같다고 가정하지 않는다. 통과. |
| 2 | `steady_clock` wall duration을 CPU 소비량으로 오인하는가 | existing wall metric과 per-thread CPU metric을 독립 필드로 고정한다. 타깃 CPU source/해상도를 확인 못 하면 `unknown`으로 둔다. 통과. |
| 3 | wall-minus-CPU 잔여를 scheduler delay로 부르는가 | runnable-but-not-running 상태를 OS trace에서 직접 합산한다. blocked/page-fault 잔여는 scheduler delay로 승격하지 않는다. 통과. |
| 4 | 재시작 뒤 TID 재사용이나 다른 process의 같은 TID를 scan과 잘못 join하는가 | run ID, process identity, thread identity/lifetime, marker와 clock correlation을 join 조건으로 둔다. TID 단독 결합은 금지한다. 통과. |
| 5 | trace drop·buffer overrun이 0 wait처럼 보이는가 | trace coverage/loss를 판정 입력으로 요구하고 누락이면 해당 scheduler 값을 `unknown`으로 처리한다. 통과. |
| 6 | 계측 interval이 `LowMemoryNotification()`의 GC 시간이나 JS tree 생성 시간을 포함하는가 | 기존 timer 시작은 fixture hook의 강제 GC 뒤다. 같은 hook 순서를 유지하고 setup/GC 구간을 scan metric 밖으로 둔다. 통과. |
| 7 | 강한 JS array 16,385 wrapper가 실제 앱 사용 패턴이라고 확대되는가 | 이 입력을 registry stress fixture로 한정하고 real workload/frame impact로 해석하지 않도록 기록한다. 통과. |
| 8 | boundary WeakRef 두 개만으로 전체 생존 집합이 증명되는가 | 대량 fixture의 empty/reclaimed count와 Rust 기준선을 결합하고, HostRoot/wrapper/external-root별 정확 집합은 별도 deterministic fixture oracle로 요구한다. 통과. |
| 9 | root 정렬·중복 제거 최적화가 C++ map 특성에 암묵적으로 의존하는가 | ID/order/index contract negative controls를 유지하고, 외부 경계 계약을 약화하는 candidate는 별도 근거 전 제외한다. 통과. |
| 10 | callback output buffer 부족을 allocation failure와 동일하게 처리해 mutation을 놓치는가 | `BUFFER_TOO_SMALL` no-mutation/retry oracle를 별도 지정하고 allocator injection과 분리한다. 통과. |
| 11 | C++ `std::bad_alloc` 처리 결과를 Rust/OS OOM 복구 보장으로 확대하는가 | C++ fixture injection 경계를 한정하고 Rust 표준 allocator의 process OOM 복구는 보장하지 않는다고 적는다. 통과. |
| 12 | `CollectionPlan` commit 전 오류가 문서·index를 부분 변경하는가 | revision·resource baseline과 atomic failure oracle를 필수로 둔다. 통과. |
| 13 | reusable scratch buffer가 이전 16k high-water capacity를 계속 붙들어 memory cost를 숨기는가 | 대량→소량→대량 상태의 capacity/high-water를 별도 기록하고 무제한 retained capacity를 후보 제외 기준으로 둔다. 통과. |
| 14 | C++ traversal과 Rust allocation 최적화를 한 번에 적용해 원인 귀속이 불가능한가 | phase 계측 뒤 실제 비용이 확인된 후보 하나만 선택하고 candidate patch를 분리한다. 통과. |
| 15 | Debug·CFI-off AVD 관측을 optimized Release나 product binary로 해석하는가 | existing harness의 Debug 범위를 기록하고, CFI/GN/V8/build profile을 고정한다. fixture instrumentation artifact와 fixture-off product Release를 별도로 검증한다. 통과. |
| 16 | Android AVD/iOS Simulator 수치를 실기기 또는 플랫폼 간 순위로 제시하는가 | 각 대상별 paired comparison과 명시 device/OS 기록을 요구한다. 미연결 실기기 gate는 미완료다. 통과. |
| 17 | 기존 10/7 sample 수나 중앙값을 새로운 통계적 충분성·성능 목표로 쓰는가 | 10/7은 역사 재현 수로만 보존하고, 최소 효과·tail bound 승인 뒤 pilot 기반으로 새 N을 고정한다. 통과. |
| 18 | 반복 실행 순서·온도·전원 조건이 baseline/candidate에 편향을 주는가 | pair order, OS/device 상태, thermal drift를 기록하고, 사전 규칙 없는 sample 제거를 금지한다. 실행 N/시간 budget은 임의 숫자를 채우지 않고 gate로 남긴다. 통과. |
| 19 | counters/revision/count overflow 또는 장기 반복 실패가 percentile에 숨는가 | count/poison/deferred/revision invariants를 pass gate에 두고 long-loop 횟수·시간 예산은 pilot 전에 결정한다. 통과. |
| 20 | `commit_collection` 시간이나 vector capacity를 OS 메모리 해제/RSS 절감으로 표현하는가 | release는 논리 node/index 삭제와 V8 wrapper entry 제거로 정의한다. RSS/heap은 별도 범위·profiler 자료 없이 release 성능의 대리값으로 쓰지 않는다. 통과. |

## 검토 뒤 확인된 제한

- 현재 instrumentation은 total `steady_clock` duration뿐이며 per-thread CPU, scheduler state, Rust/C++ subphase counter는 없다. 이 계획은 측정 결과를 만들지 않았고 performance improvement를 주장하지 않는다.
- 기존 Android 기록은 AVD V8 build에서 CFI를 끈 조건이며, 기존 harness는 Debug build다. Release 기준선·실기기 target은 새로 특정해야 한다.
- 제품 CPU budget, 최소 의미 효과, statistical N, long-loop 시간, physical-device model, iOS scheduler trace 지원은 현재 자료로 정할 수 없어 구현 전 승인/사전 검증 gate로 표시했다.

이 문서는 계획 검토이며 implementation review·런타임 실행·PR diff review를 대체하지 않는다.
