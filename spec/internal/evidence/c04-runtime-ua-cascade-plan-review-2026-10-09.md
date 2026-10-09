# C04.8 계획 검토 기록 · Runtime UA cascade 재계산

**대상:** [구현 계획](../../../plan/c04-runtime-ua-cascade.md) · **검토 범위:** runtime 소유권, worker 경계, revision, FFI readback, 실패 처리

| # | 독립 검토 관점 | 판정 및 계획 반영 |
|---:|---|---|
| 1 | `CssViewport::C04_FIXTURE` desktop 기본값이 실제 모바일에 묵시 적용되는가 | 미설정을 허용하고 host의 명시 환경 설정 뒤에만 요청하도록 했다. |
| 2 | viewport와 scheme/pointer가 서로 다른 OS 시점에서 합쳐지는가 | 한 `RuntimeCssEnvironment` 구조와 단일 environment revision으로 입력을 원자화했다. 첫 환경은 revision 0, 이후 변경만 증가한다. |
| 3 | 값이 바뀌지 않았는데 revision을 올려 오래된 layout을 무효화하는가 | 의미상 같은 입력이면 revision과 worker 요청을 유지하도록 했다. |
| 4 | 잘못된 수치가 상태를 먼저 바꾼 뒤 계산 단계에서 발견되는가 | 환경 입력 검증을 저장·revision 증가보다 먼저 수행하도록 했다. |
| 5 | C04.7의 invalid pointer 조합이 runtime API에서는 우회되는가 | 같은 `CssViewport`/media 검증을 사용하고 오류 시 기존 값을 보존한다. |
| 6 | environment revision overflow가 0으로 되돌아가 stale tuple을 승인하는가 | `checked_next()` 실패는 전체 환경 변경 거부로 명시했다. |
| 7 | 다중 framework root 중 일부만 스타일되는가 | HostRoot direct Element 전체를 root 순서대로 요청에 넣는다. |
| 8 | HostRoot에 HTML document/body 의미를 묵시 부여하는가 | 각 direct-root는 독립 cascade scope이며 HTML document 의미를 추가하지 않는다. |
| 9 | 여러 root에서 하나가 실패해도 앞 root의 부분 결과가 남는가 | root 집합 전체가 성공하기 전 computed 결과를 publish하지 않는다. |
| 10 | 스타일되지 않는 top-level Text가 버려져 결과 node 집합이 달라지는가 | top-level Text는 오류다. |
| 11 | detached-only mutation이 full document revision key에 반영되지 않거나 attach를 놓치는가 | 모든 `DocumentRevision` 변경에서 계산한다. detached 변경 최적화는 미루고 detached mutation 후 재삽입을 검증한다. |
| 12 | JS가 마지막에 throw하면 이미 커밋한 DOM 변경의 재계산이 빠지는가 | JS status와 독립적으로 task 전후 `DocumentRevision`을 비교한다. |
| 13 | viewport 변경 중 이전 계산이 완료되어 새 요청 결과로 게시되는가 | publish 시 전체 tuple key를 재확인한다. |
| 14 | 빠른 연속 변경이 worker queue를 무제한 늘리는가 | pending 슬롯 하나를 최신 요청으로 교체하고 실행 중 하나만 둔다. |
| 15 | worker가 V8 object/callback을 건드리거나 환경 setter가 UI thread에서 thread 생성으로 지연되는가 | 세션 초기화 중 worker를 준비하고 setter는 깨우기만 한다. 유휴 worker 비용을 측정하며 immutable Rust snapshot과 plain environment 값만 보낸다. |
| 16 | 이전 completed가 남거나 worker panic 뒤 요청이 무기한 pending인가 | 새 요청 때 completed를 비우고 key를 재검사한다. worker panic은 `failed` 상태와 이후 setter의 worker 오류로 고정한다. |
| 17 | ABI caller가 짧은 버퍼나 retry 사이 상태 변경으로 부분 JSON을 소비하는가 | required byte count, output NUL guard와 `-3` 전용 short-buffer status를 계약하고 각 호출에서 snapshot을 고정한다. |
| 18 | JSON 직렬화 lock 점유 또는 snapshot clone으로 JS owner thread가 막히는가 | JSON은 상태 clone 뒤 lock 밖에서 만들고 task 종료 전체 snapshot 복제는 O(노드 수)임을 명시해 소형·대량 DOM에서 측정한다. |
| 19 | Session 종료 중 actor/CSS worker join에서 lock 교착 또는 지연을 숨기는가 | actor 종료 뒤 lock 없이 CSS worker를 close/join한다. Stylo 계산 취소·종료 상한은 보장하지 않는다고 기록한다. |
| 20 | UA-only 문구가 지원 중인 inline `style` 속성을 누락하거나 CSSOM·전체 CSS·Taffy·GPU 지원으로 과장하는가 | 기존 C04.5 inline attribute cascade/diagnostics를 보존하고 author stylesheet·CSSOM API·Taffy/GPU/OS 수집은 제외했다. |

## 검토 뒤 고정한 구현 경계

CSS worker OS thread는 `RuntimeSession::new()` 중 세션별로 준비하고 환경 미설정 중에는 대기한다. Session startup/idle 비용과 종료 시 진행 중 Stylo 계산을 기다리는 시간은 실행 근거에 남긴다. HostDocument snapshot clone은 task 종료의 V8 owner thread에서 O(노드 수) 비용이므로 Stylo 계산과 분리됐다는 이유로 전체 경로가 non-blocking이라고 주장하지 않고 소형·대량 DOM 비용을 측정한다. 기존 V8 scheduler에는 style job을 넣지 않아 JavaScript task priority/FIFO 계약을 바꾸지 않는다. 환경 setter는 thread 생성·계산을 기다리지 않지만 짧은 상태 lock 경합은 있다. 부족한 버퍼로 인해 상태가 retry 사이에 바뀔 수 있고 JSON 복사 비용은 크기에 비례하므로 required capacity는 매 호출 재확인하며 JSON 조회는 UI thread에서 하지 않는다.
