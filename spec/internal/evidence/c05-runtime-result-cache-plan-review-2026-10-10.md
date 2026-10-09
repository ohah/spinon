# C05.3 runtime 결과 cache 계획 검토

- **대상:** [`c05-runtime-result-cache.md`](../../../plan/c05-runtime-result-cache.md)
- **구분:** 코드 구현 전에 수행한 계획 검토
- **판정:** cache key에서 `DocumentRevision`을 뺄 수 있는 조건, 전체 revision 재발행, 단일-entry 수명과 connected mutation 무효화를 명시했다. dirty-subtree 구현으로 과장하지 않는다.

| # | 공격 관점 | 계획에 반영한 대응 |
|---:|---|---|
| 1 | `DocumentRevision`이 다르면 무조건 cache hit/miss를 잘못 판단할 수 있음 | key에서 생략하는 유일한 이유를 연결 `RenderTreeRevision` 불변으로 한정하고 detached mutation 사례만 hit로 둔다. |
| 2 | 재생성된 document에서 NodeId가 같아 과거 결과가 충돌함 | `DocumentGeneration`을 key에 포함하고 서로 다른 generation은 반드시 miss로 처리한다. |
| 3 | 연결 tree 변경이 render revision을 올리지 않아 stale CSS가 남음 | `same_render_projection`이 connected node 값 전체를 비교하는 기존 코드와 속성·구조 mutation 테스트를 계획에 포함한다. |
| 4 | id/class/namespace/tag 변경이 selector 결과에 반영되지 않음 | 모든 connected `HostNode` 변경은 render revision miss로 처리하며 property별 추정 invalidation을 만들지 않는다. |
| 5 | 형제 순서·`:first-child`·`:nth-child` 관계가 캐시됨 | 연결 child order 변화가 render projection에 포함되는지 fixture로 확인하고 전체 recompute한다. |
| 6 | 조상 custom property 수정이 자손으로 전파되지 않음 | 연결 조상 attribute/style 변경은 miss이며 C05.1 inheritance reference로 결과를 대조한다. |
| 7 | author `<style>` 추가·수정·이동·분리 뒤 registry가 낡음 | 연결 style text와 순서는 connected projection 변화로 miss 처리하고 C05.2 transition을 재검증한다. |
| 8 | detached style 변경이 connected sheet scope를 바꾼다고 잘못 가정 | Chromium oracle에서 detached source와 connected 결과 불변을 관찰하고 연결 stylesheet 변경은 별도 miss로 분리한다. |
| 9 | viewport 크기는 같지만 media scheme/pointer/hover가 달라짐 | `EnvironmentRevision`, 실제 viewport와 media environment 모두 key에 포함한다. |
| 10 | device scale 변화가 key에 빠짐 | `device_scale_factor.to_bits()`를 cache key에 포함한다. |
| 11 | `-0.0`, NaN, float revision alias로 다른 viewport가 동치 처리됨 | 유효성 검증을 먼저 유지하고 finite float의 bit representation으로 key를 구성한다. |
| 12 | profile별 allowlist·`@property` mode가 같은 worker cache를 공유함 | cache는 생성 시 profile이 고정된 단일 coordinator/worker 내부에만 둔다. |
| 13 | style snapshot만 고치고 layout/render snapshot key는 오래된 채 남음 | computed style, layout completed, render key 및 outer completed key의 전체 tuple 재발행을 요구한다. |
| 14 | key를 바꾼 뒤 연결 style/layout payload도 변형함 | payload는 immutable backing arrays로 공유하고 metadata만 새 wrapper에 연결한다. |
| 15 | 큰 결과를 cache와 published snapshot에 각각 복사해 메모리가 두 배가 됨 | Arc-backed style/frame/render arrays를 사용하고 cache slot은 한 건으로 제한한다. |
| 16 | invalid CSS/cascade 오류 뒤 직전 정상 cache가 새 revision에 재사용됨 | mismatch 시 기존 entry를 계산 전에 제거하고 실패 결과는 저장하지 않는다. |
| 17 | layout 실패 결과가 성공 cache로 재사용됨 | `RuntimeLayoutState::Failed` 결과는 cache에서 제외한다. |
| 18 | 늦게 완료된 stale worker request가 최신 cache entry를 덮음 | stale 완료는 publish하지 않고 cache에 저장하지 않으며 request full-key 비교를 유지한다. |
| 19 | FFI가 cache hit과 실제 계산을 구별하지 못하거나 duration을 오해함 | `cacheHit`을 내부 JSON에 추가하고 duration 0의 의미·비성능 진단 한계를 contract에 명시한다. |
| 20 | worker 종료 뒤 cache가 누수되거나 여러 revision이 누적됨 | cache ownership을 worker stack으로 한정하고 한 entry 교체 및 shutdown drop을 검증한다. |
| 21 | simulator fixture가 detached wrapper를 잃거나 성능 측정이 cache hit 호출만 세고 복사 비용을 숨김 | V8 fixture가 detached node를 `globalThis`에서 보유하며 attach/detach 전이를 확인한다. Release 반복의 p50/p95 worker 처리시간과 Arc payload 공유를 함께 기록하고, medium/large hit p50이 miss보다 낮지 않으면 성능 개선으로 통과시키지 않는다. |

## 계획 보정

cache key는 connected style/layout input이 바뀌지 않은 경우에만 재사용하도록 좁혔다. `DocumentRevision` 외의 모든 revision·viewport·media 입력을 key에 남기며, 연결 변경에 대한 dirty-subtree 추정은 이 단계에 포함하지 않는다. 결과 재발행 시 FFI가 검사하는 전체 revision tuple을 새 request에 맞춰 유지하도록 성공 조건을 구체화했다.
