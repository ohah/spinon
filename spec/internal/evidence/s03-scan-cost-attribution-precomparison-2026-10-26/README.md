# S03.3 registry scan 비용 분석 · 구현 전 비교 모델

**상태:** 기존 동작의 비교 계약과 재현 절차를 고정한다. 이 문서 작업에서 제품 코드·계측 코드·fixture를 실행하거나 변경하지 않았다.

**계획:** [`plan/s03-3-scan-cost-attribution.md`](../../../../plan/s03-3-scan-cost-attribution.md)
**비교 기준 checkout:** `origin/main` `86b9cd9125f01f5d1277db2d2e525a5498688d70`
**기존 구현 전 실행 자료:** [반복 scan 관측](../s03-dynamic-registry-repeat-measurements-2026-10-05.md), [대량 registry 실행](../s03-dynamic-wrapper-registry-2026-10-05.md)

## 비교할 동작

현행 기준은 V8 owner safe point마다 동기 전체 registry scan을 수행하고, live weak wrapper ID 집합으로 같은 owner의 `HostDocument` unreachable component를 수거한다. 비용 최적화 전후에도 다음 동작을 그대로 유지한다.

- owner safe point에서 `node_wrappers` 전체를 확인한다. JS façade ID, 살아 있는 wrapper identity, HostRoot·외부 root 연결 성분과 ID 비재사용 계약을 바꾸지 않는다.
- registry root를 Rust `HostDocumentBridge`에 넘겨 구조·ID 인덱스를 검증하고 도달성을 계획한 뒤 commit한다. 전체 C++/Rust 경계에서 실패한 회수는 문서 상태를 보존하며, 불변식 위반은 현재 poison/defer 정책을 따른다.
- 성공한 회수는 document/render-tree revision을 바꾸지 않는다. 회수 가능한 모든 노드 ID·문자열 계수·wrapper mapping이 같은 생존 집합 oracle로 판정되어야 한다.

## 고정 입력과 정확성 oracle

기존 `tests/fixtures/dom/s03/runtime-lifecycle-probe-v1.js`의 대량 입력을 기준으로 한다. HostRoot 아래 container 하나와 text node 16,384개를 만들고 그 wrapper 16,385개를 JS 강한 배열로 보존한다. 기존 문서 wrapper 9개를 더해 C++ registry의 측정 대상은 **16,394개**다. 생성 순서는 128개 노드 단위 batch로 고정되어 있다.

두 상태를 각각 비교한다.

1. **모두 live:** 대량 JS 배열을 유지한다. 재scan 10회 동안 `scanned=16394`, `live=16394`, `empty=0`, 회수 결과 없음, `deferred=0`, `poisoned=0`을 요구한다. 반복 scan은 같은 Isolate의 연속 빈 eval이며 V8 호출과 safe-point 경계는 기존 probe를 따른다.
2. **대량 release:** container 연결을 끊고 배열·container strong root를 놓은 뒤 검증용 GC 요청과 safe-point scan을 실행한다. 첫/마지막 wrapper `WeakRef` 소멸, `empty=16385`, `live=9`, `scanned=live+empty`, 오류·poison 없음, Rust 기준선 `nodes=10`, `UTF-16 units=226`, `live wrappers=9`를 요구한다. 기존 소형 `node-lifecycle-v1.rs`의 HostRoot·wrapper root·external root 연결 성분 예상 생존 집합도 그대로 통과해야 한다.

두 상태는 최적화 후보 전후의 동일성 oracle이다. 기존 대량 JS probe는 첫·마지막 wrapper를 직접 확인하고 총 empty count 및 Rust 자원 기준선을 검사한다. 후보가 그보다 넓은 부분 생존 집합을 건드리면 소형 deterministic fixture의 정확한 retained set 검증을 함께 사용하고, 필요한 대형 혼합-root 입력은 구현 전 별도 fixture 계획으로 정한다. 성능 통과만으로 생존 집합 동일성을 대체하지 않는다.

## 기존 숫자와 해석 한계

2026-10-05 고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`, Android 16/API 36 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 얻은 기존 wall-time은 다음과 같다.

| 관측 | Android AVD | iOS Simulator |
|---|---:|---:|
| 7회 fresh-process setup scan 중앙값 (최소–최대) | 63.153 ms (8.043–133.193 ms) | 13.473 ms (8.085–19.068 ms) |
| 같은 Isolate의 연속 재scan 10회 중앙값 (최소–최대) | 42.125 ms (7.498–174.678 ms) | 17.707 ms (8.732–25.980 ms) |
| 당시 마지막 fresh-process setup scan | 132.520 ms | 9.528 ms |

이 값은 비교할 기존 관측이지 목표치나 현재 `origin/main`의 재현 결과가 아니다. 기록된 구간은 `CollectDocumentAtSafePoint` 전체의 `steady_clock` wall-time이며 registry 순회, Rust callback/collector, C++ 결과 검증 및 약한 wrapper 정리를 포함한다. CPU time과 scheduler run/wait 상태는 수집하지 않았고, 큰 Android 편차가 실제 scan 계산인지 OS scheduling인지 분리하지 못했다. 플랫폼 간 순위를 뜻하지 않는다.

Android 과거 실행은 emulator에서 `autib1716` SIGILL을 피하려고 그 검증 V8 산출물만 CFI를 비활성화했다. iOS 결과는 Simulator다. 어느 쪽도 release app, 실기기, hardware 성능 근거가 아니다.

## 재현 가능한 기준 실행

기존 재현 harness는 다음 명령을 사용한다. 예시 ID와 output 경로는 해당 기기에서 확인해 고정하며, 매 실행마다 새 output directory를 쓴다.

```sh
SPINON_ANDROID_EMULATOR_SERIAL=emulator-5554 \
SPINON_IOS_SIMULATOR_UDID=ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D \
SPINON_S03_DOM_GC_OUTPUT_DIR=build/spinon/dom-lifecycle-cost/unique-run-id \
bash tools/verify-s03-dom-lifecycle-simulators.sh
```

이 계획에서는 위 명령을 실행하지 않았다. 계획 worktree는 시작 시 `28696fbf`를 기준으로 만들었고 `origin/main`이 `86b9cd91`로 진행된 뒤 rebase했다. 그 사이 main delta는 C11.4 계획·STATUS·roadmap 문서만 포함하며 S03 source·fixture·evidence 경로는 바뀌지 않았다. harness는 Android `assembleDebug` 및 iOS Simulator scheme의 기본 Debug build를 사용한다. scan 비용 구현의 비교용 **현재 기준선**을 만들 때는 checkout SHA, dirty diff, V8 revision·GN args·artifact hash, SDK/toolchain, 기기/OS, feature flags, 결과 디렉터리를 함께 고정한다. 기존 10개 동일-Isolate scan의 정확성 재현과 7개 fresh-process 관측을 다시 수집할 때는 실행마다 새 directory를 사용한다. 이 10회·7회는 과거 관측 재현 수이지 통계적 충분성이나 성능 목표가 아니다.

## 측정값의 분리 계약

비교는 각 대상 플랫폼과 build profile 안에서만 baseline/candidate를 짝지어 수행한다. 하나의 `document_collection_last_scan_us` 총합을 여러 종류 비용으로 해석하지 않는다.

| 값 | 시작·끝 경계 | 보고 방식과 제한 |
|---|---|---|
| 총 scan wall time | `CollectDocumentAtSafePoint` 진입부터 모든 성공/실패 반환 직전까지 | 현행 `steady_clock` 값을 보존하고 정밀 단위를 명시한다. callback·buffer growth·검증·wrapper erase를 포함한다. |
| scan/thread CPU time | 위와 같은 owner thread와 scan interval | 타깃에서 검증된 per-thread CPU counter 또는 OS scheduler trace로 구한다. 지원·정밀도·overflow가 입증되지 않으면 `unknown`이다. wall-minus-CPU를 CPU time으로 대체하지 않는다. |
| scheduler delay | OS trace상 owner thread가 runnable이었으나 실행되지 않은 구간 중 scan marker와 교차하는 구간 | Android는 owner TID와 맞는 Perfetto scheduler 상태 및 userspace marker를 요구한다. iOS도 같은 thread identity·clock interval을 연결할 수 있는 System Trace 또는 검증된 동등 자료가 필요하다. trace loss/clock mismatch/상태 미노출이면 `unknown`으로 남긴다. 단순 `wall-CPU` 잔여는 block/page fault/기타 off-CPU를 섞으므로 scheduler delay로 부르지 않는다. |
| 논리 회수 적용 시간 | Rust collector 안의 `HostDocument::commit_collection` 및 bridge index/resource 제거 구간 | 노드와 bridge mapping의 논리적 제거 비용이다. allocator가 OS에 페이지를 반환한 시간이나 RSS 감소를 뜻하지 않는다. |
| V8 wrapper release 시간 | C++의 reclaimed ID 검증 뒤 해당 `Global::Reset()` 및 `node_wrappers.erase()` 구간 | 약한 wrapper registry entry 정리 비용이다. JS GC 시간과 혼합하지 않는다. |
| 계측 overhead | 계측 on/off에서 동일 build·입력의 총 wall/CPU 값 | 비교용 계측 자체가 결과를 바꾸는지 따로 보고한다. overhead가 무시 가능하다고 임의 판정하지 않으며 원자료를 보존한다. |

Rust callback 내부에 phase marker를 추가한다면 root ID 검증·handle 매핑·graph validation/reachability·CollectionPlan 생성·commit 및 index removal 경계를 겹치지 않게 정의한다. 각 phase의 합을 전체 wall-time과 직접 같다고 가정하지 말고, 미계측 구간과 timer 오차를 별도로 기록한다. 계측과 scratch buffer는 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 전용으로 격리하고, 기본/제품 artifact에 새 benchmark marker·hook·payload가 없는지 negative/positive control을 확인한다.

## 아직 정하지 않은 실행 예산

- **성능 개선의 최소 의미 효과와 허용 tail regression:** 기존 표본은 두 자릿수에서 170ms대까지 넓게 퍼졌고 제품 scan budget은 이 계획 입력에 없다. 중앙값 차이 하나로 합격선을 만들지 않는다. 동일 target의 pilot으로 분산·측정 해상도를 확인하고, 사용자/작업 owner가 허용 지연·최소 의미 효과를 사전 승인한 뒤 반복 수를 정한다.
- **통계적 비교 반복 수:** 10개 same-Isolate 표본과 7개 fresh-process 표본은 기존 데이터의 반복 수다. 큰 편차에 대한 분포 추론이나 새 후보의 충분한 검정력을 보장하지 않는다. 재현 baseline은 이 수를 그대로 복제하되, 새 성능 판단 N은 목표 효과·신뢰구간/오류 허용 정책이 정해질 때까지 미결이다.
- **장기 반복 횟수·시간 예산:** 과거 6×32 create/detach/reclaim은 resource baseline 회귀지만 장기 안정성 gate로 고정되지 않았다. 추가 장기 반복은 실행 시간, thermal throttling, 상태 누적 위험을 pilot에서 확인하고 고정한다. 숫자 미정 상태의 관측은 pass/fail 성능 주장이 아니다.
- **Release profile:** 현재 재현 harness는 Debug 경로다. Android release task, iOS release configuration, opt-in instrumentation의 최적화 조건과 V8 args를 별도로 고정해야 release 비용이라고 부를 수 있다. 기존 Debug 결과를 release에 대입하지 않는다.
- **Scheduler trace 지원 범위:** Android/iOS별 OS 버전에서 thread-state trace, owner TID, monotonic clock 상관이 모두 재현되는지 확인해야 한다. 한 플랫폼에서 실패하면 해당 scheduler 값만 미측정으로 남기며 CPU와 wall 차이를 대신 쓰지 않는다.
- **실기기 선택:** 과거 baseline은 AVD/Simulator뿐이다. Android SoC·기기·OS, iOS 기기·OS, signing/실행 경로가 정해지지 않았다. 실행 시 정확한 기기 ID·모델·OS와 artifact hash를 기록하고, 연결된 실기기가 없으면 simulator/AVD 측정으로 한정한다. 기기 결과를 다른 SoC나 플랫폼에 일반화하지 않는다.
