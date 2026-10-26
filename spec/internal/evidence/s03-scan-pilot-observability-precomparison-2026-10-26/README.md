# S03.3 pilot scan observability · 구현 전 비교 모델

**기준 source:** `origin/main` `8247a12b`
**선행 계획:** [scan 비용 귀속 계획](../../../../plan/s03-3-scan-cost-attribution.md)
**slice 계획:** [pilot observability](../../../../plan/s03-3-pilot-scan-observability.md)
**현재 동작 자료:** [대량 wrapper 실행](../s03-dynamic-wrapper-registry-2026-10-05.md) · [반복 scan 관측](../s03-dynamic-registry-repeat-measurements-2026-10-05.md)

## 비교할 동작과 입력

기준 동작은 owner safe point마다 동기 전체 registry scan을 실행한다. 기존 `runtime-lifecycle-probe-v1.js` 대량 live fixture는 container와 text wrapper 총 16,385개를 강한 배열로 보존하고, 동일 Isolate에서 10번 빈 eval을 호출해 각 operation의 마지막 scan 통계를 관찰한다. 과거 자료에서 총 scan 대상은 기존 wrapper 9개를 포함한 16,394개였다. 새 slice는 입력, isolate 경계, safe point, scan 횟수, Rust collector 및 C++ wrapper 정리 동작을 바꾸지 않는다.

비교 시 각 표본은 operation report에서 아래 값을 함께 가져온다.

| 필드 | 의미 | 해석 한계 |
|---|---|---|
| `seq` | session operation 순서 | 같은 run/session 안의 상관 키 |
| `trace_cookie` | 호출자가 전달한 operation correlation 값 | OS trace marker가 아님 |
| `owner_tid` | session V8 owner native thread ID | scheduler state가 아니며 process/thread lifetime 정보만으로는 충분하지 않음 |
| `document_collection_last_scan_start_ns` | scan 시작 시 `steady_clock` epoch ns | 다른 clock domain/process/reboot의 절대 시각과 바로 비교하지 않음 |
| `document_collection_last_scan_us` | scan 진입부터 성공/실패 반환 직전까지의 총 wall duration | CPU time, scheduler delay, 논리 회수 부분시간이 아님 |

각 플랫폼 로그의 `scan_sample_index`는 1부터 10까지이고 기존 `scan-us` 배열도 남긴다. 표본 존재성 검증은 성능 개선 판정이 아니다.

## 정확성 oracle

비교 대상·코드 변경 전후에 기존 expected values를 유지한다.

- live 재scan 10회: `scanned=16394`, `live=16394`, `empty=0`, `deferred=0`, `poisoned=0` 및 오류 없음.
- release/reclaim 뒤: 16,385 empty wrapper, 나머지 기존 wrapper 유지, weak wrapper의 실제 소멸, Rust node/string 및 live-wrapper 기준선 복귀.
- 소형 fixture의 HostRoot·wrapper root·external root expected retained set, revision 불변, callback buffer 부족 retry 및 기존 invariant 확인.

새 관측 필드가 누락된 run은 pilot metadata가 불완전하다고 표시한다. correctness 결과를 metric completeness와 합치거나, 어떤 latency 값으로 correctness를 대체하지 않는다.

## source/build identity 모델

기존 simulator runner가 생성하는 매 run output directory를 source of evidence로 유지한다. 그 안의 manifest는 commit/branch, tracked diff 및 untracked source hash, pinned V8 revision, opt-in S03 fixture flag, platform별 Debug build 명령/구성, host tool version, AVD/Simulator 식별자, Android APK hash, iOS `.app` bundle tree digest와 build log hash를 기록한다. 동일 source commit에 dirty 변경이 있으면 commit SHA만으로 재현됐다고 주장하지 않는다.

app sample report의 TID·monotonic start/duration은 run manifest의 source/build/platform identity와 같은 output directory 아래 저장된다. Android/iOS 출력 각각의 manifest 및 application output을 섞지 않는다. 해당 tuple은 OS scheduler trace를 지원·검증하지 않고, owner TID와 `steady_clock` 시작값의 외부 clock correlation을 만들지 않는다.

## 이번 변경에서 말하지 않는 것

현재 source의 pilot latency, 평균/중앙값, 분산, CPU cost, scheduler delay, release cost 또는 성능 개선 효과는 아직 채집/판정하지 않았다. AVD와 Simulator 이외 기기, CFI 설정 차이, release build, long-loop stability 및 allocation failure도 이 slice의 comparison 대상이 아니다. 성능 effect threshold, tail bound, 새 통계 N은 계속 미정이다.
