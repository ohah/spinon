# 0021 · S03.3 DOM 노드와 JS wrapper 수명 계약

**인터페이스 버전:** 0.1.0-draft · **상태:** 구현 전 계약 확정 · **제품 GC 구현:** 미구현

**상태 대장:** S03.3, J10 일부 · **기준 시제품:** [0020 S03.2 제한 DOM façade](0020-s03-dom-facade.md)
**계획:** [S03.3 노드 수명 관리](../../plan/dom-node-lifecycle.md) · **결정성 fixture:** [node-lifecycle-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-v1.rs), [node-lifecycle-limits-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-limits-v1.rs)

## 목적과 범위

이 문서는 Rust HostDocument와 V8 JavaScript heap 사이의 노드 생존, wrapper 정체성, 약한 handle 검사, 자원 한도와 세션 종료 계약을 고정한다. S03.2의 동작을 소급 변경하지 않으며, 이 문서의 계약과 테스트용 기준 모델은 실제 수거 구현이나 공개 DOM 지원을 뜻하지 않는다.

S03.3 구현은 Rust가 노드 저장소와 mark-and-sweep을 소유하고, V8 C++ adapter가 자동 reset되는 약한 wrapper handle을 안전 지점에서 검사하는 경계를 따른다. V8 unified heap, 별도 Rust GC, 앱에 노출되는 JSI 유사 API는 이 범위에서 사용하지 않는다.

## 용어와 소유권

| 용어 | 계약 |
| --- | --- |
| HostRoot | 앱 문서의 논리 루트다. HostRoot 아래 모든 최상위 노드가 생존 root다. 브라우저 Document 객체를 뜻하지 않는다. |
| JS façade ID | façade와 FFI가 쓰는 양수 i32 키다. 0은 HostRoot이며 노드 ID로 쓰지 않는다. |
| Rust HostNodeHandle | DocumentGeneration과 NodeId의 쌍이다. JS façade ID와 별개의 타입이며 문서 간 전달할 수 없다. |
| wrapper weak handle | C++ runtime이 `HostNodeHandle`별로 소유하는 `v8::Global<v8::Object>`다. 강한 JS root가 아니며 별도 등록 token은 두지 않는다. |
| external root token | 네이티브 비동기 작업이 Rust 노드 handle을 직접 보유할 때 발급하는 세션 한정 단조 증가 lease다. 활성 한도는 세션당 16,384개이며 완료·취소 때 owner 실행기에서 해제한다. |
| 안전 지점 | isolate owner의 JavaScript 작업과 microtask checkpoint 또는 native host 작업이 끝나고 V8 진입 및 Local handle scope가 없는 구간이다. 이 구간에서 약한 handle registry를 검사하고 필요하면 Rust 회수를 요청한다. |
| resident 사용량 | HostDocument 저장소와 문자열 계수에 실제로 남아 있는 노드 및 UTF-16 단위다. 회수 후보도 성공한 sweep 전까지 포함한다. |

HostDocument의 BTreeMap 보유 자체는 root가 아니다. 노드 관계, wrapper 등록, HostRoot, 명시적 external root만 생존 여부에 참여한다. JavaScript 문자열로 이미 복사된 값은 Rust 노드의 생존 root가 아니다.

## 확정한 동작 계약

### 노드 도달성

1. HostRoot의 모든 최상위 노드에서 아래 방향으로 도달 가능한 노드는 유지한다.
2. V8에서 아직 살아 있는 wrapper 등록과 유효한 external root token이 가리키는 노드도 root다.
3. root에서 표시할 때는 부모·자식 관계를 양방향으로 따라 해당 연결 성분 전체를 유지한다. detached 자손 wrapper가 살아 있으면 부모, 형제, 형제의 자손 등 현재 Node 관계 API로 관찰 가능한 연결 성분을 보존한다.
4. 위 root 어느 것에서도 도달하지 않는 detached 연결 성분만 회수 후보다. 연결 여부만으로 살아 있는 wrapper가 가리키는 노드를 회수하지 않는다.
5. parent와 child 양쪽 관계는 서로 일치해야 한다. 중복 child, 끊어진 역참조, 어떤 root나 handle이 현재 DocumentGeneration에 속하지 않거나 저장소에 없는 노드를 가리키는 경우 sweep 전체를 중단한다. 검증 오류가 노드 일부의 삭제로 이어져서는 안 된다.

회수 대상은 위 관계로 JS·native 작업에서 관찰할 수 없는 연결 성분이다. 성공한 sweep은 내부 node·handle·문자열 보유량만 바꾸고 `DocumentRevision`과 `RenderTreeRevision`은 올리지 않는다. sweep은 HostRoot 트리나 스타일·레이아웃 입력을 바꾸지 않으므로 해당 revision을 무효화하지 않는다.

### wrapper 정체성과 식별자

- 같은 Rust node를 가리키는 wrapper가 JavaScript에서 살아 있는 동안 조회는 같은 JS 객체를 반환한다. 동시에 살아 있는 canonical wrapper는 노드당 하나다.
- V8이 약한 handle을 자동 reset한 뒤에도 Rust 노드가 다른 root로 살아 있다면 다음 조회는 새 JS wrapper 객체를 만들 수 있다. wrapper 객체 identity는 달라지지만 façade ID는 노드와 `HostNodeHandle`의 매핑이므로 기존 ID를 유지한다. 이전에 도달 불가능했던 객체와 새 객체의 identity 비교는 보장 대상이 아니다.
- JS façade ID는 1부터 발급하며 한 세션에서 다른 노드에 재사용하지 않는다. 노드가 살아 있는 동안 wrapper가 GC되어도 ID↔handle 매핑은 유지한다. 노드 sweep commit 때 해당 매핑을 제거하고 high-water mark는 유지한다. 예약된 뒤 노드 생성이 실패해도 번호는 gap으로 건너뛴다. i32::MAX 다음 신규 노드 생성은 명시적 ID 공간 소진 오류로 닫힌다.
- Rust NodeId도 한 DocumentGeneration 안에서 재사용하지 않는다. 회수된 HostNodeHandle 조회는 stale handle 오류로 실패 폐쇄하고, 다른 generation의 handle은 외부 ID로 다시 매핑하지 않는다.
- wrapper registry key는 전체 `HostNodeHandle`이며, 한 문서 세대에서 노드마다 weak handle 한 개만 둔다. 만료된 wrapper를 다시 만들 때는 빈 weak handle slot을 새 객체로 교체한다. 노드 ID는 재사용하지 않으므로 별도 weak registration token과 오래된 callback 방지가 필요 없다.
- stale JS ID는 InvalidStateError, JS ID 공간 소진과 저장 quota 초과는 QuotaExceededError로 façade에 전달한다. 내부 오류는 진단에서 각각 구분한다.

새 노드와 그 JS wrapper의 생성은 외부에서 원자적으로 보여야 한다. 런타임은 비재사용 JS ID와 Rust `HostNodeHandle`을 예약한 뒤, unpublished wrapper·약한 `Global` slot·handle mapping을 준비한다. Rust 변경 commit과 사전 확보된 C++ registry/map slot의 publish가 모두 성공해야 JS에 wrapper를 반환한다. 사전 준비나 Rust commit이 실패하면 staged `Global`을 owner에서 reset하고 Rust reservation을 취소하며 공개 문서·revision·handle mapping은 바꾸지 않는다. 실패에 앞서 예약된 JS ID와 NodeId는 gap으로 남긴다. 등록 용량과 C++ 저장 공간은 commit 전에 확보해 publish 단계에서 recoverable allocation이 발생하지 않게 한다.

### V8 약한 handle과 안전 지점

- V8 weak wrapper registry는 isolate별 C++ runtime이 소유한다. JS의 강한 Map으로 wrapper를 보관하지 않는다. wrapper에서 ID를 찾는 WeakMap은 사용할 수 있으나, 그것을 node-to-wrapper canonical cache로 간주하지 않는다.
- wrapper `Global<Object>`에는 callback 인자가 없는 `SetWeak()`를 사용한다. V8은 GC가 객체를 도달 불가로 판정하면 해당 phantom handle을 자동으로 reset한다. callback 인자형 `SetWeak()`은 최선 노력 방식이며 호출 시점이나 호출 자체가 보장되지 않는다. callback을 쓰면 첫 단계에서 해당 handle을 `Reset()`해야 하고, 그 뒤에는 V8 API를 호출할 수 없다. 따라서 callback을 Rust 노드 회수의 필수 신호로 사용하지 않는다. [V8 `PersistentBase::SetWeak()` 계약](https://chromium.googlesource.com/v8/v8.git/+/main/include/v8-persistent-handle.h) · [약한 handle의 자동 reset 구현](https://chromium.googlesource.com/v8/v8/+/refs/heads/main/src/handles/global-handles.cc) · [`IsEmpty()` 계약](https://chromium.googlesource.com/v8/v8/+/refs/heads/main/include/v8-handle-base.h)
- 가장 바깥 JS 작업과 microtask checkpoint가 끝난 owner 안전 지점마다 C++가 전체 weak registry를 선형 검사한다. `Global::IsEmpty()`인 wrapper만 이번 staged sweep의 root에서 제외하고, live handle은 `HostNodeHandle` 목록으로 Rust에 넘긴다. empty wrapper의 ID↔handle 매핑은 노드가 다른 root로 살아 있는 동안 보존하고, 노드가 실제 sweep될 때 함께 제거한다. registry 크기는 resident node 한도인 16,384개 이하로 제한하고, 검사·root 목록 저장 공간은 사전 확보한다. 매 safe point 최대 16,384개를 확인하는 비용은 제품 성능 실험에서 측정해야 한다.
- 안전 지점 밖에서 Rust sweep을 실행하지 않는다. 중첩 실행 중 재진입한 회수 요청은 pending 표시만 남기며 재귀 실행하지 않는다. Rust가 전체 mark set·삭제 목록·revision 결과를 준비해 commit 성공을 반환한 뒤에만 C++가 빈 weak registry slot을 제거한다. 실패하면 Rust 상태와 C++ registry 항목을 보존하고 다음 안전 지점에 재시도한다.
- V8 GC 시점 자체는 통제하지 않는다. 객체가 JS에서 도달 불가해도 GC가 아직 실행되지 않았으면 weak handle은 비어 있지 않을 수 있고, 그 노드는 보수적으로 유지된다. quota 오류 뒤에 GC나 자동 재시도를 강제하지 않는다.

### registry 상한과 검사 실패

- isolate마다 사전 확보한 `HostNodeHandle → Global<Object>` registry와 같은 상한의 root snapshot buffer를 둔다. resident node quota를 넘는 weak handle은 등록하지 않는다. 별도 알림 queue, overflow fallback cache, 사용자 callback은 없다.
- registry scan/root snapshot 준비, owner·generation 검증, mark buffer 또는 삭제 목록 구성에서 실패하면 sweep을 연기한다. 노드·문자열·revision·registry 상태는 commit 전과 같아야 한다.
- scan 대상이 상한을 넘거나 중복된 `HostNodeHandle`/다른 generation을 포함하면 fail-closed하고 세션 진단에 기록한다. scanner 또는 collector가 복구된 뒤 다시 시도할 수 있다.
- 지연이나 GC 미실행은 quota 회수를 늦출 수 있으나 살아 있는 wrapper의 조기 회수를 허용하지 않는다. 진단에는 마지막 전체 scan 시각, scanned/live/empty handle 수, deferred collection 횟수를 세션 종료까지 보존한다.

### callback과 external root

- V8 strong Global<Function>은 Reset될 때까지 함수와 그 closure를 살리는 JavaScript root다. 현재 spinon.onEvent 진단 handler는 런타임 소유 root이고 DOM listener API를 뜻하지 않는다.
- pending native 작업이 JS callback을 보유하면 callback owner, 완료·취소 시점, isolate owner에서의 Reset 시점을 기록한다. 그 작업이 Rust HostNodeHandle을 직접 보유한다면 별도의 external root lease도 등록한다. JS closure가 wrapper를 캡처한 경우는 해당 weak handle이 non-empty인 동안 wrapper root로 관찰한다.
- external root lease token은 세션 안에서 재사용하지 않는다. 16,384개 활성 한도에 도달하면 새 native 작업을 등록하지 않고 호출자에게 backpressure 오류를 돌려준다. 해제는 owner에서 정확히 한 번 적용하며 중복·stale·다른 세션의 해제 요청은 root를 더 제거하지 않는 멱등 no-op이다. token sequence 소진도 새 작업 등록을 거부한다.
- S03.2에는 DOM addEventListener가 없다. DOM EventTarget listener의 node-owned edge, closure capture cycle과 회수 정책은 J12가 정한다. J12 계약과 구현 전까지 listener를 지원한다고 표시하거나 strong listener Global을 이 collector의 정상 root로 간주하지 않는다.

### quota와 실패 결과

- 초기 한도는 S03.2와 같이 resident node 16,384개, 보존 문자열 16,777,216 UTF-16 code units다. 문자열 계수는 namespace·local name, Text data, attribute name과 value를 포함한다. Rust가 회수한 node와 문자열은 동일 sweep commit에서 계수에서 뺀다.
- 입력 변환과 예상 resident 합계를 commit 전에 검사한다. quota를 넘는 변경은 QuotaExceededError로 실패하며 HostDocument, handle map, string accounting, revision을 바꾸지 않는다. ID를 이미 예약했다면 해당 JS ID는 재사용하지 않고 gap으로 남긴다.
- quota 실패 때 제품 경로에서 강제 full GC를 호출하거나 같은 메서드를 자동 재시도하지 않는다. 보통의 V8 GC가 자동 reset한 weak handle은 다음 안전 지점에서 검사한다. 그 뒤 앱이 다시 호출하면 회수된 resident quota 범위에서 성공할 수 있다. V8 테스트 전용 강제 GC는 자동화 테스트 전용이다.
- registry snapshot·mark buffer·worklist·sweep plan·external root slot 준비 중 recoverable allocation 실패는 해당 collection을 deferred 처리하거나 새 비동기 작업을 거부한다. node·quota·이미 등록한 root 상태는 그대로 두고 원인을 세션 진단에 남긴다.
- sweep은 먼저 전체 mark set과 삭제 목록을 임시로 만들고 generation·owner·모든 root를 검증한 뒤 적용한다. 오류나 unwind 가능한 panic이면 삭제 목록 적용 전에 중단하여 node map, wrapper registry, external roots와 quota를 모두 보존한다. panic=abort 빌드는 프로세스 복구를 보장하지 않으며 이 계약의 panic 복구 범위 밖이다.
- isolate owner 불일치, session/document generation 불일치, 손상된 external root lease는 fail-closed다. collection을 실행하지 않고 원인 계수만 증가시킨다. 비동기 회수 오류는 사용자 JS 예외로 바꾸지 않고 다음 런타임 진단에서 확인할 수 있게 세션 종료까지 보존한다.

### 종료

세션 종료는 isolate owner에서 Active → Closing → Closed 순서로만 진행한다.

1. 새 JS 진입·wrapper 등록·external root 등록을 거부하고 pending host 작업을 취소한다.
2. callback별 strong Global을 isolate owner에서 Reset하고 external root lease를 해제한다.
3. live weak wrapper Global을 owner isolate에서 Reset하고 wrapper registry와 preallocated snapshot buffer를 폐기한다. 자동 reset된 빈 Global은 다시 사용하지 않는다.
4. Rust HostDocument bridge·handle map을 파기한다.
5. V8 Context와 Isolate를 dispose한다. Closed 이후 도착한 외부 완료 응답은 session generation을 검사해 no-op으로 닫는다.

종료 중 collection은 수행하지 않는다. 세션 종료는 남은 node를 순회 회수하는 단계가 아니라 격리된 전체 세션 소유물을 폐기하는 단계다.

## 결정성 있는 Rust 기준 fixture

공통 fixture는 [node-lifecycle-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-v1.rs)다. Rust integration test는 고정 노드·부모·root·문자열 입력에서 연결 성분 결과를 확인하고 weak handle scan, 새 wrapper 객체와 기존 façade ID의 분리, ID 공간 소진, foreign session/document generation·missing node root, malformed parent/child/root/registry 입력의 fail-closed 동작과 failure transaction의 기대 상태를 검사한다. reference model에서 성공 sweep은 document/render revision을 바꾸지 않으며 성공 노드 생성만 document revision을 올린다. fixture 기준은 이 문서의 불변 조건과 S03.2 HostDocument 관계 조회다. 브라우저가 제공하는 native GC 의미를 복제한다고 주장하지 않는다.

이 PR의 fixture는 test-only reference model이다. 실제 HostDocument에서 node를 삭제하지 않고 V8 GC 또는 자동 weak-handle reset을 실행하지 않는다. 따라서 Rust collector, wrapper identity의 V8 동작, quota 해제, scan 성능, Android·iOS 실제 V8 동작은 아직 검증되지 않았다. 이 범위를 통과한 것으로 표시하지 않는다.

## 구현 완료에 필요한 후속 증거

- 실제 Rust collector가 동일 fixture에 대해 root·sweep·stale handle·quota·failure atomicity를 통과한다.
- C++ V8 adapter가 callback 없는 약한 `Global<Object>` registry, 자동 reset 관찰, bounded full scan, isolate safe point를 구현하고 scan 비용을 측정한다.
- Android 및 iOS 시뮬레이터의 실제 V8 실행에서 연결·분리 wrapper, 자동 reset 뒤 재생성 identity, callback closure root와 종료 중 GC를 검증한다.
- 반복 create/remove/collect 뒤 resident node/string 수와 callback handle 계수가 기대 기준으로 돌아오는지 확인한다. V8 heap snapshot의 retaining path와 Rust 수거 계수는 별도로 판정한다.
- 이 문서와 상태 대장을 반영하기 전에는 S03.3, J10 또는 공개 DOM 지원을 완료로 표시하지 않는다.

## 참고 경계

S03.2 DOM façade의 공개 후보 범위·예외는 [0020](0020-s03-dom-facade.md), 전체 작업 순서는 [계획](../../plan/dom-node-lifecycle.md), 기존 GC 비교와 React Native·Lynx 수명 조사 근거는 계획 문서에 둔다. 이 계약은 V8 ABI 함수 signature나 공개 JavaScript API를 추가하지 않는다.
