# S03.3 비용 분석 계획 PR 변경분 검토 · 독립 실패 관점 30개

**범위:** `plan/dom-node-lifecycle.md`의 비용 분석 링크, 새 S03.3 scan-cost 계획, 구현 전 비교 기준, 계획 20관점 검토
**검토 기준 base:** `origin/main` `86b9cd9125f01f5d1277db2d2e525a5498688d70`
**결과:** P1/P2 발견 없음. 문서 링크·diff whitespace 확인 통과. 이 PR은 계획/증거 추가이며 제품 코드·공식 상태·roadmap 수정이나 실행 측정은 포함하지 않는다.

| # | 독립 PR 변경 실패 관점 | 판정 |
|---:|---|---|
| 1 | 기존 S03.3 완료조건을 새 계획이 이미 달성한 듯 바꾸는가 | 기존 체크리스트는 미완료로 남고 새 계획 링크만 추가한다. 통과. |
| 2 | 새 계획을 기존 수명 계획에서 찾을 수 있는가 | `plan/dom-node-lifecycle.md`의 S03.3 설명과 미완료 비용 항목이 sibling plan을 가리킨다. 통과. |
| 3 | 새 plan→precomparison 링크가 repo-root 기준 올바른가 | local-relative target 존재 확인. 통과. |
| 4 | 계획에서 20관점 검토 기록으로 가는 링크가 올바른가 | plan header의 local-relative target 존재 확인. 통과. |
| 5 | precomparison에서 기존 2026-10-05 raw evidence 경로를 찾을 수 있는가 | 같은 evidence namespace의 측정·대량 registry 기록을 연결한다. 통과. |
| 6 | 계획 base와 최신 branch base가 달라져 근거 SHA가 오도되는가 | 28696fb 작업 시작과 docs-only 86b9cd91 rebase를 각각 기록하고 source 경로 불변을 밝힌다. 통과. |
| 7 | 과거 측정을 현재 `origin/main`에서 새로 얻은 값으로 표시하는가 | 2026-10-05 자료와 미래 current-source rerun을 명확히 분리한다. 통과. |
| 8 | 재시작/반복 scan의 median·min·max를 잘못 옮겼는가 | 기존 관측표와 원본 문서의 플랫폼별 수치를 대조했다. 통과. |
| 9 | 16,385 new wrappers와 9 baseline wrappers의 합을 틀리는가 | 비교 기준은 총 16,394로 산술과 기존 기록이 일치한다. 통과. |
| 10 | fixture 생성 batch 경계가 실행마다 달라질 수 있는가 | 기존 `runtime-lifecycle-probe-v1.js`의 128-node batching을 고정 입력으로 적는다. 통과. |
| 11 | 모두-live 재scan이 fixture strong-root array를 놓치는가 | 16,394 live와 0 empty/reclaimed를 요구한다. 통과. |
| 12 | release path가 base document wrappers까지 회수하도록 허용하는가 | 9 live wrappers·10 nodes·226 UTF-16 units baseline을 명시한다. 통과. |
| 13 | 큰 fixture의 경계 WeakRef 검사만으로 모든 node identity를 입증하는가 | count/resource oracle 및 소형 expected-retained-set fixture를 함께 요구한다. 통과. |
| 14 | outer `CollectDocumentAtSafePoint` timer를 Rust callback 비용으로 잘못 이름 붙이는가 | 전체 scan과 callback/collector/release 하위 phase를 따로 부른다. 통과. |
| 15 | explicit GC 시간이나 node creation 비용이 scan timer에 끼어드는가 | GC hook이 timer 전, setup이 scan 밖인 현재 경계를 유지한다. 통과. |
| 16 | CPU time을 wall-time으로 대체해 CPU cost 주장을 만드는가 | 동일 owner interval의 thread CPU source와 `unknown` 정책을 요구한다. 통과. |
| 17 | off-CPU 전체를 scheduler wait로 과장하는가 | runnable-but-not-running만 scheduler delay이며 wall residual은 대체 불가라고 한다. 통과. |
| 18 | PID/TID 재사용·clock drift가 다른 scan과 trace를 결합하는가 | run/process/thread lifetime identity와 marker-clock correlation을 join gate로 둔다. 통과. |
| 19 | trace loss를 0 scheduler delay로 해석하는가 | trace coverage 불완전 시 값은 `unknown`이다. 통과. |
| 20 | 논리 node destruction·weak wrapper reset을 OS memory release로 오인하는가 | Rust logical release, C++ registry cleanup, allocator/RSS를 분리한다. 통과. |
| 21 | inclusive phase timings를 합산해 outer wall을 중복 계산하는가 | non-overlap marker와 미계측 gap을 따로 보고하며 같은 합계를 가정하지 않는다. 통과. |
| 22 | instrumentation 자체의 overhead/optimisation 변경을 숨기는가 | instrumentation-on/off 비교와 benchmark-vs-product artifact 구분을 둔다. 통과. |
| 23 | profile 단계 없이 특정 최적화가 개선한다고 선결하는가 | 후보는 측정 결과 뒤 선택하며 현재 algorithm을 선택하지 않는다. 통과. |
| 24 | OOM·stale index·poison 오류 때 문서 partial mutation을 허용하는가 | plan/commit atomicity, revision·resource·error invariants를 correctness gate로 둔다. 통과. |
| 25 | callback buffer-too-small 재시도 실패를 allocator injection으로 가리는가 | 부족 응답 no-mutation/retry control과 C++ allocator injection을 구분한다. 통과. |
| 26 | vector/scratch 재사용이 high-water memory를 무한히 붙드는가 | 고용량 후 소용량 capacity와 retained memory를 보고, 무제한 retention 후보를 배제한다. 통과. |
| 27 | Android Debug/CFI-off AVD를 release/product 수치로 제시하는가 | Debug harness와 과거 CFI-off 조건을 기록하고 release profile을 별도 gate로 둔다. 통과. |
| 28 | AVD·Simulator나 상이 SoC를 교차 플랫폼 성능 순위로 만드는가 | 각 target 내부 paired comparison 및 device-specific claim으로 제한한다. 통과. |
| 29 | sample 수·최소 개선폭·장기 반복·실기기를 근거 없이 숫자로 정하는가 | 과거 10/7회는 재현용이며 target/N/long-loop/device는 승인 전 unresolved로 표시한다. 통과. |
| 30 | 계획 PR이 API/STATUS/roadmap 완료나 실행 증거로 잘못 읽히는가 | 공개·내부 interface version과 STATUS/roadmap은 수정하지 않고, 빌드·run을 실행하지 않았다고 기록한다. 통과. |

이 검토는 PR 문서 변경분에 한정한다. CPU·scheduler 측정, Release build, allocation failure injection, 장기 반복, 실기기 및 S03.3 완료를 증명하지 않는다.
