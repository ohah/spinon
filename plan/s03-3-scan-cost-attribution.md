# S03.3 · registry scan 비용 귀속과 최적화 후보 비교

**상태:** 계획 초안 · 제품 코드 변경 전
**상위 작업:** S03.3 · J10
**기존 수명 계획:** [DOM node lifecycle](./dom-node-lifecycle.md)
**구현 전 비교 모델:** [S03.3 scan-cost precomparison](../spec/internal/evidence/s03-scan-cost-attribution-precomparison-2026-10-26/README.md)
**계획 실패 관점 검토:** [서로 다른 20개 관점](../spec/internal/evidence/s03-scan-cost-attribution-plan-review-20-2026-10-26.md)

## 목적과 경계

2026-10-05 Android AVD와 iOS Simulator에서 16,394개 live wrapper를 훑은 총 wall-time은 반복 실행 사이 편차가 컸다. 당시 수치는 thread CPU와 OS scheduler 상태를 수집하지 않아 scan 계산, thread가 runnable 상태에서 기다린 시간, block 또는 다른 off-CPU 비용을 구별하지 못한다. 이 계획은 이 비용을 분리하고, 정확한 수명 계약을 유지하는 범위에서 근거 있는 최적화 후보를 비교한다.

이번 계획은 측정·비교를 위한 것이며 제품 동작을 바꾸지 않는다. `owner safe point`마다 동기 전체 registry scan을 한다는 규칙, wrapper identity, HostRoot/external-root 도달성, stale ID와 오류 정책, 문서 revision 불변성을 그대로 둔다. 비동기·증분 sweep, scan 생략/빈도 변경, weak callback으로 수거 시점을 바꾸는 설계는 포함하지 않는다.

DOM EventTarget listener와 cross-heap closure cycle은 J12 선행 계약·작업에 남겨 두며 이 계획에서 해결하거나 S03.3 완료로 승격하지 않는다. OS allocator OOM에서 프로세스가 복구된다고 약속하지 않는다.

## 현재 경로와 재사용 기준

현행 계측과 작업 경계는 다음과 같다.

1. [`native/v8/src/spinon_v8.cc::CollectDocumentAtSafePoint`](../native/v8/src/spinon_v8.cc)는 `steady_clock` 전체 wall-duration을 기록하고 C++ `node_wrappers` map을 순회해 weak wrapper의 live root ID를 구성한다.
2. 같은 owner thread에서 [`crates/spinon-runtime/src/host/document/collection.rs::collect_document`](../crates/spinon-runtime/src/host/document/collection.rs)는 입력 ID·capacity와 bridge index를 검사하고 ID를 `HostNodeHandle`로 매핑해 core collector를 호출한다.
3. [`crates/spinon-core/src/document/collection.rs`](../crates/spinon-core/src/document/collection.rs)는 graph 유효성, root reachability, `CollectionPlan`, commit을 처리한다. Rust bridge는 external ID·handle·string index를 갱신한다.
4. C++ caller는 reclaimed ID를 검증하고 해당 weak `Global`을 reset한 뒤 registry entry를 제거한다. 현행 바깥 timer는 위 단계를 포함한다.

기준 입력, 생존 집합 oracle, 역사적 wall-time과 한계는 [precomparison 문서](../spec/internal/evidence/s03-scan-cost-attribution-precomparison-2026-10-26/README.md)를 단일 비교 정의로 사용한다. 실행 전 `origin/main` 기준 checkout에서 현재 코드로 기준선을 다시 채집해 역사적 2026-10-05 측정과 분리한다. 고정된 고정 파일 hash, V8 revision/GN args, app binary hash, source SHA, OS/toolchain, feature flags, 기기, output directory를 run manifest에 보존한다.

## 측정 모델

### 분리할 시간과 상태

- **전체 scan wall-time:** `CollectDocumentAtSafePoint` 시작부터 성공 또는 실패 반환까지. 현재 `document_collection_last_scan_us`의 의미를 보존한다.
- **owner-thread CPU-time:** 위 interval과 동일 thread/구간의 CPU time. 타깃별 OS counter/trace의 source와 resolution을 사전 확인한다. 지원·정밀도가 확인되지 않으면 `unknown`으로 기록한다.
- **scheduler delay:** OS trace에서 scan interval 중 owner thread가 runnable이지만 실제 실행되지 않은 interval만 합산한다. join key에는 run ID, process identity, thread identity/lifetime, scan marker와 clock correlation을 포함한다. TID만으로 process 재시작·thread 재사용 경계를 합치지 않는다. wall-minus-CPU는 blocked·page fault 등도 포함하므로 scheduler delay로 해석하지 않는다. identity/correlation을 증명하지 못하거나 trace가 잘리면 `unknown`으로 처리한다.
- **논리 회수 비용:** Rust `commit_collection`과 bridge의 node/ID/string index 제거 시간을 구분한다. 메모리 해제/allocator 내부 작업이 어느 경계에 포함되는지 표시하되 RSS/OS page return과 동일시하지 않는다.
- **V8 wrapper 정리 비용:** reclaimed ID 확인 이후 `Global::Reset()` 및 `node_wrappers.erase()` 시간을 별도 기록한다. GC 시간은 scan marker 시작 전에 실행되므로 이 값과 합치지 않는다.
- **계측 overhead:** 동일 입력의 instrumentation-off/on pair를 둔다. 계측 비용이 없다고 가정하지 않고 raw data와 차이를 보고한다.

marker는 C++ scan·Rust collector phase를 겹치지 않게 정의하고 같은 owner thread에 붙인다. Android의 scheduler state는 Perfetto scheduler trace 및 userspace marker로 연결한다. iOS는 해당 OS/toolchain에서 동일 thread·clock 구간을 추적할 수 있는 System Trace 또는 검증된 동등 자료를 먼저 확인한다. 한 쪽에서 제공되지 않는 metric을 다른 쪽에서 wall residual로 대체하지 않는다.

### 정확성 판정

기준선과 후보 각각에서 다음을 통과해야 timing 값 비교에 포함한다.

- 모두-live fixture에서 scan/live/empty/reclaimed count가 precomparison과 같고, ID·wrapper identity 및 오류 상태가 일치한다.
- 대량 release 뒤 weak wrapper 수명, reclaimed ID set, Rust node/string/wrapper 기준선이 일치한다.
- `node-lifecycle-v1.rs`의 HostRoot·wrapper root·external root별 expected retained set이 일치한다.
- callback result buffer가 부족한 negative control은 문서·index·revision 불변 상태로 `BUFFER_TOO_SMALL`를 반환하고, 재시도에서 기존 expected reclaimed set을 반환한다. allocation failure injection과 이 기존 callback 용량 오류를 같은 실패로 뭉뚱그리지 않는다.
- `deferred=0`, `poisoned=0`, `scanned=live+empty` 등 현행 invariant가 보존된다. 오류 injection은 아래의 실패 주입 gate에서 별도로 확인한다.
- 회수 성공 전후 `DocumentRevision`·`RenderTreeRevision`이 변하지 않는다.

화면·GPU frame·DOM listener·전체 프로세스 RSS는 scan 성능 판정에 사용하지 않는다. 메모리 동작을 추가 관찰하면 통계 계정 범위를 따로 표기한다.

## 비교할 후보와 제외 기준

현재 자료에는 어느 내부 단계가 병목인지 나타내는 phase timing이 없으므로 **최적화 알고리즘을 미리 선택하지 않는다.** 후보는 귀속 자료가 비용을 지목한 경우에만 비교한다.

| 후보군 | 비교 조건 | 제외/중단 기준 |
|---|---|---|
| 동일 traversal에서 C++ live/empty 집계와 root ID 구성 | 현재 별도 순회의 `IsEmpty()` 호출·map 순회가 실제 CPU cost인지 먼저 잰다. 실패 시 통계의 부분값/전체값 의미를 현재와 같게 유지한다. | 산출 계수 차이, OOM 뒤 잘못된 통계/부분 root 제출, identity 변화 또는 측정 가능한 이득 부재. |
| Rust 반복 임시 buffer/sets 재사용 | callback 내 `sorted_root_ids`, `root_handles`, reclaimed ID, reachability/graph-validation 임시 자료의 allocation·release를 phase별로 본 뒤 가장 비싼 것만 비교한다. 대량→소량→대량 상태의 capacity/high-water 동작도 기록한다. | 재사용 scratch가 owner 격리·재진입·atomic failure를 약화하거나 무제한 retained capacity를 만들면 제외. 필요량/상한 정책이 없으면 implementation하지 않는다. |
| 검증 순회·자료구조 비용 변경 | graph/index consistency 검사·sort·lookup이 병목으로 확인된 경우 별도 variant로 평가하고 오류 negative control을 유지한다. | 중복/미정렬 ID, stale handle, parent/child edge, string index 오류를 놓치거나 외부 FFI 계약을 암묵적으로 약화하면 제외. |
| scan 생략·배치·증분·비동기 sweep 또는 V8 weak callback | 없음. 이 계획의 comparison 대상이 아니다. | 수거 시점·stale handle·callback root·safe-point의 observable semantics를 바꾸므로 별도 제품 설계·계약·적합성 oracle 없이는 명시적으로 제외한다. |

성능 수치가 기준 입력에서 안정적으로 구분되지 않으면 추가 최적화를 하지 않고 measurement limitation으로 보고한다. 여러 후보를 한꺼번에 적용한 결과만으로 어느 변경이 개선했는지 주장하지 않는다.

## 재현 및 비교 순서

1. 이 계획과 [precomparison](../spec/internal/evidence/s03-scan-cost-attribution-precomparison-2026-10-26/README.md)을 기준으로 source, build profile, V8, 기기·OS 및 toolchain을 고정한다. 과거 CFI 비활성 Android AVD 자료는 새 Release 기준선으로 재사용하지 않는다.
2. `tools/verify-s03-dom-lifecycle-simulators.sh`로 existing correctness path와 10회 same-Isolate 재scan을 재현한다. fresh-process repeat은 매 실행에 새 output directory를 부여한다. 과거 재현용 10/7회는 성능 통계 충분성을 뜻하지 않는다.
3. 처음에는 계측만 붙인 baseline을 실행한다. raw outer wall-time, thread CPU, scheduler trace, Rust phase/release, V8 wrapper release, correctness oracle와 trace coverage를 별도 파일로 남긴다. 어떤 값이 `unknown`인지도 기록한다.
4. pilot에서 노이즈·계측 overhead·플랫폼별 분산을 확인한 뒤 최소 의미 효과, tail regression 허용치, 통계 기준과 공식 repetition count를 사전 등록한다. instrumentation-enabled artifact는 benchmark용 진단 binary로 표시하며, fixture-gate off product Release와 같다고 간주하지 않는다. 계측을 끈 candidate artifact도 correctness·artifact boundary를 별도로 확인한다. 이 값들을 제품/작업 owner가 승인하기 전까지는 timing을 수집할 수 있지만 pass/fail 개선 또는 성능 주장으로 판정하지 않는다.
5. 단계 3에서 실제 비용이 확인된 후보 하나만 구현 candidate로 고른다. baseline/candidate를 각 플랫폼·build profile 안에서 paired order로 비교하고, 서로 다른 후보를 같은 patch에 섞지 않는다. 실행 순서/seed 고정이 불가능하면 그 한계를 기록한다.
6. 후보가 동일 생존 집합 oracle을 통과한 뒤에만 해당 target의 wall/CPU/scheduler/release 자료를 비교한다. trace loss, clock mismatch, scheduler state 미지원, 기기 thermal 상태 변화, source/artifact 불일치가 있으면 해당 sample을 사전에 정의한 규칙으로 제외하고 원본을 보존한다.
7. Android/iOS 실기기·long-loop·allocation failure 및 feature artifact 경계를 별도 실행해 아래 의존성을 닫는다. Simulator/AVD 자료와 분리해 보고한다. strong JS array로 16,385개 wrapper를 강제로 살려 둔 입력은 재현 가능한 registry stress fixture이며 실제 앱 workload의 분포나 frame impact를 대표하지 않는다.
8. 구현이 필요하면 별도 후속 작업으로 새 20관점 implementation review와 30관점 PR diff review를 진행한다. 이 계획 review를 구현 review에 재사용하지 않는다.

## 미결정 값과 선행 조건

| 항목 | 현재 근거 | 구현 전 완료 조건 |
|---|---|---|
| 최소 의미 효과/허용 tail regression | 고정 제품 latency budget이 없고 이전 same-Isolate Android 범위 `7.498–174.678 ms`처럼 noise가 크다. | 작업 owner가 허용 CPU·scheduler budget, 최소 효과와 tail bound를 승인한다. 없으면 성능 개선 판정 없이 observation으로 남긴다. |
| 비교 N·장기 반복 횟수와 실행 시간 예산 | 과거 `10` same-Isolate, `7` fresh process와 `6×32` lifecycle repeat만 존재하며 이들은 충분성 근거가 아니다. | pilot 분산·원하는 effect/precision·thermal drift를 보고 N과 long-loop time budget을 사전 선택한다. 임의로 `30`회 등 숫자를 채우지 않는다. |
| per-thread CPU counter와 scheduler trace 구현 | 현재 코드에는 `steady_clock`만 있다. owner TID는 report에 있으나 scan phase와 OS scheduler state 간 clock join 근거는 없다. | Android/iOS에서 API·clock·thread identity·trace loss gate를 사전 검증하고 각 target의 지원 가능한 해상도를 기록한다. 불가능한 지표는 `unknown`. |
| Release profile·instrumentation overhead | 기존 app harness는 Android `assembleDebug`, iOS Simulator Debug다. product-like release와 동일 조건의 opt-in benchmark configuration은 없다. | 최적화/fixture 계측을 포함하되 optimization level·V8 args가 고정된 benchmark artifact를 정의하고, 기본/제품 artifact fixture symbol/payload 부재를 검증한다. Debug와 Release 결과는 별개다. |
| Android/iOS physical target | 과거 Android API 36 AVD와 iOS 26.2 Simulator 외 S03.3 scan 비용 실기기 자료가 없다. | 각 기기 model, SoC/ABI, OS, signing/connection을 run manifest에 고정한다. 기기 미연결이면 physical-device gate는 미완료로 두고 emulator/simulator 결과만 보고한다. |
| C++ allocation failure injection | 현행 vector 성장에는 `std::bad_alloc` 처리 경로가 있으나 주입 실행은 없다. Rust 표준 allocator의 OS OOM 복구도 보장하지 않는다. | fixture-only allocator injection으로 각 C++ vector growth/failure 경계를 순서대로 재현하고, document/resource/ID/invariant 보존 및 재시도 정책을 확인한다. Rust allocator OOM에 대한 보장으로 확대하지 않는다. |
| Native memory/RSS 해석 | 현재 buffer byte는 vector payload capacity이며 map/allocator/V8/Rust/process 전체를 계정하지 않는다. | heap/RSS를 보고할 경우 별도 profiler와 account scope를 정하고, 논리 release나 high-water capacity만으로 OS memory 반환을 주장하지 않는다. |
| DOM listener/external-root liveness | existing plan은 listener cycle을 J12에 위임한다. 대량 성능 fixture는 listener cycle 완료를 뜻하지 않는다. | 이 계획의 scope 밖이다. J12 contract와 listener/root tests가 별도로 선행되기 전에는 S03.3/J10 전체 완료로 표시하지 않는다. |

위 미결정 항목은 이번 계획이 비교 기준과 다음 결정 절차를 고정한 뒤 진행할 입력이다. 성능 기준·표본 수·실기기 모델을 과거 관측만으로 추정해 정하지 않는다.

## 제품 코드와 artifact 경계

향후 구현이 승인되면 예상 변경은 다음으로 한정하고 작업 전 actual diff를 다시 확인한다.

- `native/v8/src/spinon_v8.cc`, `native/v8/include/spinon_v8.h`: fixture-gated timing/phase fields, nonblocking scan markers, safe-point stats.
- `crates/spinon-runtime/src/v8.rs`, `crates/spinon-runtime/src/session/report.rs`, `crates/spinon-runtime/src/host/document/collection.rs`: typed collection timing/snapshot report 및 phase boundaries. `crates/spinon-core/src/document/collection.rs`는 profile 결과가 core plan/commit를 실제 지목할 때만 후보로 둔다.
- `tests/fixtures/dom/s03/runtime-lifecycle-probe-v1.js`, `tools/verify-s03-dom-lifecycle-simulators.sh` 또는 별도 benchmark harness: 고정 root set, 반복·artifact/environment manifest와 raw output.
- Android/iOS build flags와 capture adapters는 실제 trace 지원 사전 검증 후에만 추가한다.

`spec/STATUS.md`, `spec/public/roadmap.html`, 기존 `plan/dom-node-lifecycle.md`, 기존 evidence 및 C11.4 files는 이 계획 branch에서 수정하지 않는다. Release/fixture builds는 기본·제품 artifact에서 S03.3-only symbol, marker, code path, JS payload가 빠지고 opt-in benchmark artifact에만 positive control이 포함되는지 확인한다. main 직접 push/merge는 하지 않는다.

## 완료 조건

- 고정 input·현재-source baseline·build/runtime manifest와 raw measurement format을 재현한다.
- CPU, wall, runnable scheduler delay, Rust logical release, C++ wrapper release가 별도 필드로 남거나, unsupported 값이 이유와 함께 `unknown`으로 남는다.
- baseline/candidate 모두 동일 root set·node identity·revision·resource baseline oracle를 통과한다.
- fault injection·long-loop·physical-device 의존성의 판정 결과와 미측정 경계를 보존한다.
- 최적화 후보는 원인 귀속 결과로 선택하며, 승인된 목표·N·target matrix 없이 성능 우열을 주장하지 않는다.
- 구현이 시작되는 경우 계획 review와 별개인 신규 implementation review 20개, PR diff review 30개, project fixture/artifact boundary checks를 통과한다.

이 계획만으로 구현, 성능 개선, 기기 호환, release readiness, RSS 절감, S03.3 또는 J10 완료를 주장하지 않는다.
