# C05.3 · 연결 장면이 그대로일 때 runtime 계산 결과 재사용

- **문서 유형:** 구현 계획 · 공식 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C05 사용자 지정 속성과 재계산](../spec/STATUS.md#css-구현-체크리스트)
- **선행 조건:** C04.10 runtime CSS→WGPU, C05.1 사용자 지정 속성, C05.2 `@property`
- **내부 계약 버전:** `0.1.0` 고정 · 출시·호환성 정책 확정 전 숫자 버전 변경 금지
- **구현 전 비교 모델:** [detached-node 입력과 현재 전체 재계산 경로](../spec/internal/evidence/c05-runtime-result-cache-precomparison-2026-10-10.md)
- **계획 검토:** 구현 검토와 별개인 20개 실패 관점 기록

## 목표와 완료 경계

현재 CSS worker는 `register_document_snapshot()`이 받을 때마다 cascade와 layout을 다시 계산한다. 연결 장면이 그대로인 채 detached node의 attribute·style·text만 바뀌어 `DocumentRevision`은 증가하고 `RenderTreeRevision`은 그대로인 요청도 전체 재계산한다.

C05.3은 같은 worker/profile에서 이미 성공한 계산 결과를 한 건만 보관하고, 문서 세대·연결 RenderTreeRevision·StyleRevision·환경 revision과 실제 viewport 입력이 같을 때 재사용한다. detached-only 변경으로 DocumentRevision만 달라지면 결과의 revision 표지만 새 값으로 다시 묶어 publish한다. 이때 computed-style 원소, layout frame, renderer box payload는 불변 `Arc` 배열을 공유해 대형 결과 전체 복사를 피한다.

이 작업은 연결된 요소의 dirty-subtree 계산이 아니다. 연결 트리에 영향이 있는 모든 변경은 기존처럼 전체 cascade/layout을 다시 수행한다. C05 부모와 dirty-subtree 최적화는 미완료로 남긴다. 목표는 정확성을 유지하면서 detached-only revision에서 중복 계산을 건너뛰는 것이다.

## 현재 기준 동작

- 기준 source snapshot은 `origin/main`의 C05.2 병합 직후 `693be43`이다.
- `RuntimeUaCascadeHandle::register_document_snapshot`은 입력 snapshot마다 `request_latest`를 호출한다. worker는 pending request마다 `compute` closure를 실행한다.
- `HostDocument::commit`은 새 전체 document revision을 발행하고, `same_render_projection`이 연결 root에서 도달하는 모든 `HostNode`가 바뀌지 않았으면 RenderTreeRevision을 보존한다. detached subtree의 수정은 document revision만 바꾼다.
- `compute_request`는 현재 snapshot에서 author stylesheet를 다시 모으고 모든 HostRoot cascade를 계산한다. 단일 root면 Taffy layout과 paint snapshot도 새로 만든다.
- pinned Chrome 비교 모델은 C05.2 HTML fixture에서 detached 요소의 inline style을 바꾼 전후 연결 노드의 computed width·rectangle이 같음을 확인했다. 이는 detached 변경이 연결 화면 출력을 바꾸지 않는다는 독립 oracle다.

## 캐시 계약

1. 캐시는 CSS worker 하나당 최근 성공 결과 한 건만 보관한다. 캐시 값에는 `HostDocumentSnapshot`, `StyloDocumentView`, V8 handle, Rust DOM node, 외부 native root를 넣지 않는다.
2. cache key에는 `DocumentGeneration`, `RenderTreeRevision`, `StyleRevision`, `EnvironmentRevision`, viewport width/height/device scale의 `to_bits()` 값, color-scheme/pointer/hover 입력을 포함한다. worker 인스턴스 자체가 계산 profile을 고정하므로 profile 간 캐시 공유는 없다.
3. `DocumentRevision`은 cache key에서만 제외한다. cache hit 결과를 현재 revision으로 다시 묶을 때 `RuntimeUaCascadeCompleted`, 각 `ComputedStyleSnapshot`, `RuntimeLayoutCompleted`, `RuntimeRenderKey`의 revision tuple을 모두 갱신한다. 세대·render/style/environment revision은 cache key가 같으므로 원래 값과 같아야 한다.
4. 새 key가 기존 key와 다르면 이전 entry를 계산 전에 제거한다. 새 cascade가 실패하거나 계산이 panic 나도 이전 entry를 다시 사용할 수 없어야 한다. 성공한 cascade와 layout 결과만 다음 한 건으로 저장한다.
5. 계산 중 새 request가 최신이 되면 이전 결과는 publish하지 않는다. 최신 요청이 아닌 계산은 cache에 넣지 않는다. cache hit 결과도 기존 latest-wins full-key 검사에서 현재 request와 다시 비교한다.
6. cache hit에는 `cacheHit: true`, 실제 cascade/projection 수행이 없으면 해당 두 duration은 `0`을 반환한다. 실제 계산 결과에는 `cacheHit: false`를 반환한다. 이 값은 캐시 동작 진단이며 wall-clock request latency나 성능 향상 비율로 해석하지 않는다.
7. FFI runtime snapshot JSON에 내부 필드 `cacheHit`을 추가하고 계약 0035에 성공·실패·호환 범위를 기록한다. 제품 버전 및 내부 계약의 숫자 버전은 `0.1.0`을 유지한다.
8. worker shutdown 시 entry가 worker와 함께 해제된다. 캐시는 여러 revision을 누적하지 않으며 cache miss마다 이전 payload를 보존하지 않는다.

## cache miss 조건

| 입력 변화 | 동작 |
| --- | --- |
| 같은 document snapshot 재등록 또는 detached node의 attribute/style/text 변경 | DocumentRevision이 달라도 나머지 key가 같으면 hit, 결과 metadata를 현재 document revision으로 재발행 |
| 연결 element의 tag/namespace/attribute/state 변경 | RenderTreeRevision 변경, miss 후 전체 계산 |
| 연결 node 삽입·삭제·이동·형제 순서 변경 또는 연결 text 변경 | RenderTreeRevision 변경, miss 후 전체 계산 |
| 연결 `<style>` 내용·위치·media 변경 | RenderTreeRevision 변경, 새 stylesheet snapshot으로 전체 계산 |
| viewport 크기·scale·media 환경 변경 | environment revision 또는 viewport key 변경, miss 후 전체 계산 |
| DocumentGeneration 변경 | miss 후 전체 계산, 기존 node ID의 결과 재사용 금지 |
| CSS parse/cascade 실패 또는 layout projection 실패 | 완료 결과로 publish 가능한 경우와 실패 상태를 구분하고, 실패 결과는 cache에 저장하지 않음 |
| stale 계산 완료·worker panic·shutdown | stale 결과 publish/cache 삽입 금지; worker 종료 때 cache 해제 |

layout projection 오류 결과는 계산 실패와 구분한다. 다만 C05.3은 ready/empty 성공 장면의 재사용만 지원하며 `RuntimeLayoutState::Failed` 결과는 cache에 넣지 않는다.

## 출력·메모리 경계

- cache hit은 CSS 값·diagnostic·layout frame·paint box가 miss 기준 장면과 같아야 한다. 달라질 수 있는 출력은 현재 `DocumentRevision`에 맞춘 revision 필드와 `cacheHit`/duration 진단뿐이다.
- Rust runtime과 FFI JSON 양쪽에서 completed key, computed-style revision, layout key, render key가 모두 요청 key와 정확히 같아야 한다. 일부 metadata만 고쳐 stale snapshot을 수용하지 않는다.
- style element, layout frame, runtime render box의 대형 불변 배열은 `Arc<[T]>` 같은 공유 저장으로 cache와 published result 사이 복사를 피한다. cache 수는 1개로 고정하고 별도 LRU나 node별 cache를 도입하지 않는다.
- cache가 저장하는 성공 장면의 payload와 currently published result는 같은 immutable backing storage를 공유한다. 외부 호출자가 보유한 이전 snapshot 수명은 호출자의 Arc 보유 기간이며 worker 내부 entry 수를 늘리지 않는다.

## 비교 모델과 검증 기준

기준 CSS 동작은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 C05.2 고정 fixture다. cache hit 전후로 연결 노드 computed CSS는 문자열 정확 일치, 각 rectangle 좌표는 `0.5 CSS px` 이내로 비교한다. detached 대상 자체는 연결 노드·frame·render box 수에 들어오지 않는다.

Rust test fixture는 동일한 HostDocument에서 (a) 같은 snapshot 재등록, (b) detached node style 수정, (c) 수정한 node 연결, (d) 연결 node 분리, (e) stylesheet/class/attribute 순서 변경, (f) viewport/media revision 변경, (g) 다른 document generation을 차례로 만든다. 각 입력의 document/render revision 관계, compute invocation 수, cacheHit, 모든 revision tuple, style/frame/scene payload를 검사한다. 특히 `DocumentRevision`만 변경한 두 번째 request에서 계산 함수 호출 횟수는 증가하지 않아야 하며, 연결 변화·환경 변화 뒤에는 증가해야 한다.

성능은 플랫폼별 hardware 가속 비교로 과장하지 않는다. Rust release 실행을 서로 독립된 프로세스에서 세 번 반복하고, 각 실행은 16/256/2048 connected node scene마다 cache miss와 detached-only hit를 30회씩 짝지어 측정한다. worker 완료 wall-clock p50·p95, style/frame payload의 `Arc` 공유 여부, 실행 프로세스 peak resident memory를 기록한다. 시간은 요청 등록부터 terminal snapshot 관찰까지이며 fixture 변경·snapshot 구성 시간은 제외한다. peak RSS는 test binary 기준선 메모리를 포함하므로 캐시 단독 할당량으로 해석하지 않는다. cache hit은 모든 scene에서 p50·p95가 miss보다 낮아야 한다. 차이가 재현되지 않으면 속도 개선 주장을 하지 않고 단지 중복 계산 호출을 제거한 결과만 보고한다. Android API 37 emulator와 iOS 26.2 Simulator에서 실제 V8 fixture를 실행해 초기 `false` → detached style 변경 `true` → attach `false` → detach `false` → detached style 변경 `true` 전이를 확인한다. 고정 JS fixture는 detached element를 `globalThis`에 보관해 wrapper GC가 측정 전에 변수를 바꾸지 않게 한다. 실기기는 백로그다.

## 작업 순서

1. 이 계획을 구현 변경과 별개인 20개 실패 관점으로 검토하고, `same_render_projection`, runtime latest-wins, snapshot revision admission 코드를 확인한다.
2. Chrome detached-node 보조 비교와 기존 C05.2 reference를 실행해 연결 장면 불변 조건을 고정한다.
3. style/frame/scene payload를 불변 공유 배열로 저장할 수 있게 분리한다. 기존 JSON 결과와 computed style 의미는 바꾸지 않는다.
4. worker-local 단일 entry cache와 전체 key·invalidation·current revision 재결합을 구현한다. `cacheHit` 진단과 FFI JSON/internal contract 0035를 연결한다.
5. detached-only reuse, 각 cache miss 경계, stale completion, error, teardown 및 공유 payload 수명을 Rust adversarial fixture로 검증한다. 플랫폼 fixture는 초기 scene, detached style mutation, attach, detach를 순서대로 실행한다.
6. Rust release cache miss/hit 반복 측정, 전체 workspace/Clippy/FFI check, Android API 37 emulator·iOS 26.2 Simulator 빌드와 실제 V8 실행을 수행한다.
7. 구현 코드를 계획 검토와 다른 20개 실패 관점으로 검토하고 실행근거·상태 대장을 갱신한다. C05.3만 완료 표시하고 C05 부모·dirty-subtree·CSS 전체 지원은 미완료로 둔다.

## 미지원

dirty-subtree/selector dependency 추적, 부분 Taffy tree 재계산, per-node style cache, multiple-worker/profile 간 cache 공유, persistent cache, memory-pressure eviction, 외부 CSS 자원, CSSOM, 실기기 성능 비교는 범위에 포함하지 않는다.
