# 0021 · S03.3 DOM 노드와 JS wrapper 수명 계약

**인터페이스 버전:** 0.1.0-draft · **상태:** Rust 회수 경로와 V8 weak wrapper scan 연결 구현 · **Android·iOS 기본·반복 simulator fixture:** 통과 · **제품 완료:** 미완료

**상태 대장:** S03.3, J10 일부 · **기준 시제품:** [0020 S03.2 제한 DOM façade](0020-s03-dom-facade.md)
**계획:** [S03.3 노드 수명 관리](../../plan/dom-node-lifecycle.md) · **결정성 fixture:** [node-lifecycle-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-v1.rs), [node-lifecycle-limits-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-limits-v1.rs)
**구현 전 비교 기준:** [Rust HostDocument 회수 비교](evidence/s03-hostdocument-collector-precomparison-2026-10-04.md) · **실행 근거:** [Rust 회수 경로](evidence/s03-hostdocument-collector-2026-10-04.md)

## 목적과 범위

이 문서는 Rust HostDocument와 V8 JavaScript heap 사이의 노드 생존, wrapper 정체성, 약한 handle 검사, 자원 계측과 세션 종료 계약을 고정한다. S03.2의 고정 노드 수 quota는 동적 저장소와 ID 범위 검사로 대체한다. Rust 회수 callback과 V8 weak wrapper scan을 연결했고 Android·iOS 실제 V8 기본·반복 fixture에서 고정 입력을 검증했다. 별도 진단으로 활성 평가·대기 명령·닫힌 세션 제출의 제한된 종료 경합도 확인했다. 16,384개를 넘는 runtime 회수 경로, 최대 scan 비용, listener·external root lease와 raw C ABI 포인터를 동시 해제하는 경로는 여전히 미검증 또는 금지 경계이며, 이 내부 경로는 공개 DOM 지원 완료를 뜻하지 않는다.

S03.3 구현은 Rust가 노드 저장소와 mark-and-sweep을 소유하고, V8 C++ adapter가 자동 reset되는 약한 wrapper handle을 안전 지점에서 검사하는 경계를 따른다. V8 unified heap, 별도 Rust GC, 앱에 노출되는 JSI 유사 API는 이 범위에서 사용하지 않는다.

## 용어와 소유권

| 용어 | 계약 |
| --- | --- |
| HostRoot | 앱 문서의 논리 루트다. HostRoot 아래 모든 최상위 노드가 생존 root다. 브라우저 Document 객체를 뜻하지 않는다. |
| JS façade ID | façade와 FFI가 쓰는 양수 i32 키다. 0은 HostRoot이며 노드 ID로 쓰지 않는다. |
| Rust HostNodeHandle | DocumentGeneration과 NodeId의 쌍이다. JS façade ID와 별개의 타입이며 문서 간 전달할 수 없다. |
| wrapper weak handle | C++ runtime이 JS façade ID별로 보유하는 `v8::Global<v8::Object>`다. 강한 JS root가 아니며 Rust callback이 façade ID를 현재 `HostNodeHandle`로 검증·변환한다. 별도 등록 token은 두지 않는다. |
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
- JS façade ID는 1부터 발급하며 한 세션에서 다른 노드에 재사용하지 않는다. 노드가 살아 있는 동안 wrapper가 GC되어도 ID↔handle 매핑은 유지한다. 노드 sweep commit 때 해당 매핑을 제거하고 high-water mark는 유지한다. `QUERY_NEXT_ID`는 high-water mark를 올리며 번호를 예약한다. 예약된 뒤 노드 생성이 실패해도 번호는 gap으로 건너뛴다. 대기 ID 예약은 256개까지 허용하고 한도에서 `QuotaExceededError`로 거부한다. 내부 batch가 아직 발급하지 않은 ID를 직접 지정하면 현재 high-water mark 이상만 허용하고, 그 ID 다음으로 high-water mark를 올린다. i32::MAX 다음 신규 노드 생성은 명시적 ID 공간 소진 오류로 닫힌다.
- Rust NodeId도 한 DocumentGeneration 안에서 재사용하지 않는다. 회수된 HostNodeHandle 조회는 stale handle 오류로 실패 폐쇄하고, 다른 generation의 handle은 외부 ID로 다시 매핑하지 않는다.
- wrapper registry key는 JS façade ID이며, isolate에서 관찰한 각 ID당 weak handle을 하나 둔다. Rust callback은 모든 root ID를 현재 문서의 `HostNodeHandle`로 해석하고 stale·unknown·중복 ID를 거부한다. 만료된 wrapper를 다시 만들 때는 빈 weak handle slot을 새 객체로 교체한다. 노드 ID는 재사용하지 않으므로 별도 weak registration token과 오래된 callback 방지가 필요 없다.
- stale JS ID는 InvalidStateError, JS ID 공간 소진과 저장 quota 초과는 QuotaExceededError로 façade에 전달한다. 내부 오류는 진단에서 각각 구분한다.

새 노드와 그 JS wrapper의 생성은 외부에서 원자적으로 보여야 한다. façade는 비재사용 JS ID를 예약하고 아직 반환하지 않은 wrapper와 약한 `Global` registry entry를 먼저 준비한다. 이어 Rust 변경 callback이 `HostNodeHandle`을 stage하고 문서 batch를 commit한다. 두 단계가 성공한 뒤에야 façade가 wrapper를 호출자에게 반환한다. registry entry 준비나 Rust commit이 실패하면 façade가 staged wrapper entry를 owner에서 제거하고, 공개 문서·revision·handle mapping은 바꾸지 않는다. 실패에 앞서 예약된 JS ID와 NodeId는 gap으로 남긴다. C++ map entry의 allocation은 Rust commit보다 앞서 완료하므로 map 삽입 실패가 문서 commit 뒤에 발생하지 않는다.

### V8 약한 handle과 안전 지점

- V8 weak wrapper registry는 isolate별 C++ runtime이 소유한다. JS의 강한 Map으로 wrapper를 보관하지 않는다. wrapper에서 ID를 찾는 WeakMap은 사용할 수 있으나, 그것을 node-to-wrapper canonical cache로 간주하지 않는다.
- wrapper `Global<Object>`에는 callback 인자가 없는 `SetWeak()`를 사용한다. V8은 GC가 객체를 도달 불가로 판정하면 해당 phantom handle을 자동으로 reset한다. callback 인자형 `SetWeak()`은 최선 노력 방식이며 호출 시점이나 호출 자체가 보장되지 않는다. callback을 쓰면 첫 단계에서 해당 handle을 `Reset()`해야 하고, 그 뒤에는 V8 API를 호출할 수 없다. 따라서 callback을 Rust 노드 회수의 필수 신호로 사용하지 않는다. 아래는 저장소가 고정한 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`의 원본이다. [`PersistentBase::SetWeak()` 계약](https://chromium.googlesource.com/v8/v8.git/+/7b50b62cb18f28617959e8452e2cd18195b38bcf/include/v8-persistent-handle.h) · [약한 handle 자동 reset 구현](https://chromium.googlesource.com/v8/v8.git/+/7b50b62cb18f28617959e8452e2cd18195b38bcf/src/handles/global-handles.cc) · [`IsEmpty()` 계약](https://chromium.googlesource.com/v8/v8.git/+/7b50b62cb18f28617959e8452e2cd18195b38bcf/include/v8-handle-base.h)
- 가장 바깥 JS 작업과 microtask checkpoint가 끝난 owner 안전 지점마다 C++가 전체 weak registry를 선형 검사한다. `Global::IsEmpty()`인 wrapper ID만 root snapshot에서 제외하고 live wrapper의 façade ID를 Rust에 넘긴다. Rust는 각 ID를 현재 문서의 `HostNodeHandle`로 확인한 뒤 HostRoot와 함께 도달성을 계산한다. 이 root snapshot을 Rust 계획·commit이 끝날 때까지 고정하며 중간에 JS/V8을 재진입하거나 wrapper를 바꾸지 않는다. empty wrapper의 ID↔handle 매핑은 노드가 다른 root로 살아 있는 동안 보존하고, 노드가 실제 sweep될 때 함께 제거한다. Registry와 root snapshot에는 고정 노드 개수 상한을 두지 않고 필요량에 따라 확장한다. 외부 root ID 결과 buffer가 작으면 Rust가 필요 크기를 반환하고 C++가 buffer를 확장해 재호출한다. 매 safe point 검사는 현재 registry 전체에 선형 비례하며 최대 규모 비용은 검증 항목이다.
- 안전 지점 밖에서 Rust sweep을 실행하지 않는다. 중첩 실행 중 재진입한 회수 요청은 pending 표시만 남기며 재귀 실행하지 않는다. Rust가 전체 mark set·삭제 목록·revision 결과를 준비해 commit 성공을 반환한 뒤에만 C++가 빈 weak registry slot을 제거한다. 실패하면 Rust 상태와 C++ registry 항목을 보존하고 다음 안전 지점에 재시도한다.
- V8 GC 시점 자체는 통제하지 않는다. 객체가 JS에서 도달 불가해도 GC가 아직 실행되지 않았으면 weak handle은 비어 있지 않을 수 있고, 그 노드는 보수적으로 유지된다. quota 오류 뒤에 GC나 자동 재시도를 강제하지 않는다.

### 동적 registry와 검사 실패

- isolate마다 `facade ID → Global<Object>` registry와 동적으로 확장되는 root/reclaimed ID vector를 둔다. 새 weak handle은 메모리 할당에 성공하면 등록한다. root snapshot은 `std::vector`의 성장 정책을 따르고, 회수 결과 buffer는 Rust가 알린 필요 크기까지 여유 용량을 기하급수적으로 확장한다. 이 두 buffer의 payload byte 수와 registry entry 수를 runtime 진단에 기록한다. map allocator overhead와 Rust tree/string 전체 byte 수는 이 진단에 포함되지 않는다.
- C++ root buffer 또는 Rust 회수 임시 buffer를 확보하지 못하면 sweep을 연기하고 문서·문자열·revision·registry 상태를 유지한다. 결과 buffer가 부족하다는 응답은 Rust가 collection commit 전에 반환하므로, buffer를 확장한 뒤 재호출해도 첫 호출에서 상태가 변경되지 않는다. Rust BTreeMap 노드 할당은 표준 allocator의 fallible API를 제공하지 않으므로 프로세스 전체 메모리 고갈이 복구 가능한 오류로 항상 전달된다고 보장하지 않는다.
- 중복된 `HostNodeHandle`/다른 generation, 잘못된 출력 크기·정렬은 fail-closed하고 세션 진단에 기록한다. scanner 또는 collector의 일시 할당 실패는 다음 safe point에서 다시 시도할 수 있다. 식별 ID 공간 소진은 checked increment 오류로 닫고 ID를 재사용하지 않는다.
- 지연이나 GC 미실행은 실제 회수를 늦출 수 있으나 살아 있는 wrapper의 조기 회수를 허용하지 않는다. 런타임 진단은 전체 scan 횟수, 마지막 scan의 단조시계 시작 시각(ns)·소요 시간(μs), scanned/live/empty handle 수, root·reclaimed buffer payload bytes, 누적 deferred 횟수와 마지막 오류를 세션 종료까지 보존한다. Rust 문서 진단의 `document_nodes`와 `document_string_units`도 함께 기록한다. 성공한 scan은 마지막 오류를 지우지 않는다.

### 내부 Rust callback ABI

`SpinonDocumentCollectCallback`은 C++에서 Rust `HostDocumentBridge`로 동기 호출하는 private ABI다. C++ 호출자는 isolate owner safe point에서만 호출하며, 인자 포인터와 Rust `document_user_data`는 호출 내내 유효해야 한다.

| 인자 | 의미 |
| --- | --- |
| `document_user_data` | 현재 isolate/session에 귀속된 mutable `HostDocumentBridge` |
| `root_node_ids`, `root_count` | non-empty weak Global에서 만든 JS façade ID 목록. count가 0이면 pointer는 null이어도 된다. |
| `reclaimed_node_ids`, `reclaimed_capacity` | Rust가 성공 시 반환할 회수 façade ID 배열. 성공 결과는 오름차순·중복 없음이다. |
| `reclaimed_count` | 성공 시 반환된 ID 개수. 출력 buffer가 부족하면 변경 없이 필요한 현재 문서 capacity를 쓴다. |
| `error_output`, `error_capacity` | 오류 진단용 UTF-8, NUL 종료 buffer. 잘린 경우에도 마지막 문자는 완전한 UTF-8 경계에서 끝난다. |

| 반환값 | 의미 |
| --- | --- |
| `0` | 전체 검증·회수·bridge 계수 정리 성공 |
| `1` | 결과 buffer 부족. 문서와 계수는 바꾸지 않으며 `reclaimed_count`에 필요 capacity를 반환한다. |
| `-1` | null 인자, root pointer/count 또는 결과 pointer 오류 |
| `-2` | 중복·unknown/stale root, 불일치 인덱스, allocation·그래프·계약 검증 실패. 변경 전 상태를 유지한다. |
| `-3` | unwind 가능한 panic을 callback 경계에서 잡음. `panic=abort` 빌드에서는 프로세스 복구를 보장하지 않는다. |

성공 시 Rust는 회수 ID를 C++가 제공한 output buffer에 복사한 뒤 반환한다. C++는 반환 count·정렬·중복과 live Global 부재를 확인하고, 회수 ID의 weak Global만 owner isolate에서 제거한다. 결과 buffer가 부족하면 Rust callback은 문서·계수를 변경하지 않고 필요한 capacity를 반환한다. C++는 결과 vector를 동적으로 늘려 한 번 재호출하며, 재할당 실패나 재호출 실패는 회수를 미룬다. Rust callback 오류는 commit 전에 발생하므로 Rust 문서와 C++ registry를 보존하고, 성공한 JS eval/dispatch status를 덮어쓰지 않은 채 `document_collection_error`와 누적 deferred 횟수에 남긴다. 성공 scan 뒤에도 마지막 오류는 세션 종료까지 유지한다. Rust 성공 반환 뒤 C++가 결과 불변식을 위반한 것을 발견하면 cross-language 상태를 안전하게 복구할 수 없으므로 runtime을 poisoned 상태로 만들고 후속 eval/dispatch를 거부한다. 이 경우 세션을 닫고 새로 만들어야 한다. 이 ABI와 `getNodeWrapper`·`registerNodeWrapper`·`unregisterNodeWrapper`는 facade 초기화 때 closure 안에 캡처한 뒤 `spinon.__internal`에서 삭제하므로 앱 JavaScript에서 접근할 수 없다. C++ wrapper registry에는 임의의 항목 수 상한을 두지 않는다. `requestLifecycleCollectionForTesting()`은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 검증 빌드에서만 존재한다.

### callback과 external root

- V8 strong Global<Function>은 Reset될 때까지 함수와 그 closure를 살리는 JavaScript root다. 현재 spinon.onEvent 진단 handler는 런타임 소유 root이고 DOM listener API를 뜻하지 않는다.
- pending native 작업이 JS callback을 보유하면 callback owner, 완료·취소 시점, isolate owner에서의 Reset 시점을 기록한다. 그 작업이 Rust HostNodeHandle을 직접 보유한다면 별도의 external root lease도 등록한다. JS closure가 wrapper를 캡처한 경우는 해당 weak handle이 non-empty인 동안 wrapper root로 관찰한다.
- external root lease token은 세션 안에서 재사용하지 않는다. 16,384개 활성 한도에 도달하면 새 native 작업을 등록하지 않고 호출자에게 backpressure 오류를 돌려준다. 해제는 owner에서 정확히 한 번 적용하며 중복·stale·다른 세션의 해제 요청은 root를 더 제거하지 않는 멱등 no-op이다. token sequence 소진도 새 작업 등록을 거부한다.
- S03.2에는 DOM addEventListener가 없다. DOM EventTarget listener의 node-owned edge, closure capture cycle과 회수 정책은 J12가 정한다. J12 계약과 구현 전까지 listener를 지원한다고 표시하거나 strong listener Global을 이 collector의 정상 root로 간주하지 않는다.

### 자원 예산과 실패 결과

- resident node 개수에는 고정 quota를 두지 않는다. 양수 i32 façade ID와 NodeId는 재사용하지 않으며, 각 ID 공간이 소진되면 checked arithmetic이 신규 생성을 실패시킨다. 보존 문자열은 16,777,216 UTF-16 code units까지 허용한다. 문자열 계수는 namespace·local name, Text data, attribute name과 value를 포함한다. Rust가 회수한 node와 문자열은 동일 sweep commit에서 문자열 계수에서 뺀다.
- 입력 변환과 예상 resident 문자열 합계를 commit 전에 검사한다. 문자열 예산·한 묶음 작업 수·pending ID reservation 한도를 넘는 변경은 QuotaExceededError로 실패하며 HostDocument, handle map, string accounting, revision을 바꾸지 않는다. ID를 이미 예약했다면 해당 JS ID는 재사용하지 않고 gap으로 남긴다. 저장소 allocation 실패의 복구 범위는 아래에 적은 C++/Rust buffer별 계약을 따른다.
- 문자열 예산 실패 때 제품 경로에서 강제 full GC를 호출하거나 같은 메서드를 자동 재시도하지 않는다. resident node 수는 고정 quota로 막지 않는다. 보통의 V8 GC가 자동 reset한 weak handle은 다음 안전 지점에서 검사한다. 앱의 후속 호출은 실제 확보 가능한 메모리와 문자열 예산 범위에서 처리하며, 메모리 고갈을 성공으로 보장하지 않는다. V8 테스트 전용 강제 GC는 자동화 테스트 전용이다.
- internal bridge는 `QUERY_NEXT_ID`가 반환한 번호를 포함해 create 작업에 처음 제출된 번호를 성공·실패와 무관하게 소비한다. 실패한 번호는 다시 예약하지 않는다. 대기 reservation은 batch당 최대 작업 수인 256개로 제한한다.
- C++ weak-wrapper registry root snapshot과 회수 결과 vector는 필요한 크기까지 동적으로 확장한다. root snapshot 또는 결과 vector 할당 실패는 해당 collection을 deferred 처리하고, 아직 commit되지 않은 문서·wrapper registry 상태는 유지한다. Rust 쪽 `try_reserve`로 감지한 mark buffer·worklist·sweep plan·external root slot 할당 실패도 collection을 미루거나 새 비동기 작업을 거부하며 원인을 세션 진단에 남긴다. Rust standard BTreeMap allocation 실패가 unwind로 복구된다고 가정하지 않는다.
- sweep은 먼저 전체 mark set과 삭제 목록을 임시로 만들고 generation·owner·모든 root를 검증한 뒤 적용한다. 오류나 unwind 가능한 panic이면 삭제 목록 적용 전에 중단하여 node map, wrapper registry, external roots와 quota를 모두 보존한다. panic=abort 빌드는 프로세스 복구를 보장하지 않으며 이 계약의 panic 복구 범위 밖이다.
- isolate owner 불일치, session/document generation 불일치, 손상된 external root lease는 fail-closed다. collection을 실행하지 않고 원인 계수만 증가시킨다. 비동기 회수 오류는 사용자 JS 예외로 바꾸지 않고 다음 런타임 진단에서 확인할 수 있게 세션 종료까지 보존한다.

### 종료

세션 종료는 isolate owner에서 Active → Closing → Closed 순서로만 진행한다.

1. 새 JS 진입·wrapper 등록·external root 등록을 거부하고 pending host 작업을 취소한다.
2. callback별 strong Global을 isolate owner에서 Reset하고 external root lease를 해제한다.
3. live weak wrapper Global을 owner isolate에서 Reset하고 wrapper registry와 동적으로 커진 snapshot/result buffer를 폐기한다. 자동 reset된 빈 Global은 다시 사용하지 않는다.
4. Rust HostDocument bridge·handle map을 파기한다.
5. V8 Context와 Isolate를 dispose한다. Closed 이후 도착한 외부 완료 응답은 session generation을 검사해 no-op으로 닫는다.

종료 중 collection은 수행하지 않는다. 세션 종료는 남은 node를 순회 회수하는 단계가 아니라 격리된 전체 세션 소유물을 폐기하는 단계다.

## 구현 상태와 결정성 기준 fixture

`spinon-core::HostDocument::plan_collection`은 HostRoot와 전달된 root handle에서 양방향으로 도달 가능한 연결 성분을 표시하고 회수 handle을 NodeId 순으로 담는다. 그래프·root·generation을 먼저 검증하며 계획 단계에는 문서 변경이 없다. `commit_collection`은 문서 generation과 두 revision이 계획 시점과 같고 회수 handle이 아직 저장소에 있을 때만 제거한다. 성공한 제거는 두 revision을 올리지 않는다.

`HostDocumentBridge::collect_unreachable_with_ids`는 handle map의 양방향 대응과 노드별 UTF-16 계수를 먼저 점검한다. 회수할 handle·외부 ID와 문자열 양을 계획에서 계산한 뒤 core 회수와 façade ID/handle/string map 정리를 같은 배타 호출에서 수행한다. FFI callback은 입력 ID와 출력 용량을 검증하고, 출력 buffer가 작거나 root가 잘못되면 문서를 바꾸지 않는다. V8 adapter는 `std::map<int32_t, Global<Object>>`에서 live façade ID를 수집하고, outer eval/dispatch의 Local·Context scope가 끝난 뒤 Rust callback을 호출한다. Rust가 회수한 ID에 해당하는 약한 Global만 지운다. 검증용 `LowMemoryNotification()` 요청 hook은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 빌드에서만 JS 내부 객체에 등록한다. Android 16/API 36 ARM64 에뮬레이터와 iPhone 17 Pro/iOS 26.2 시뮬레이터의 실제 V8 실행 근거는 [evidence/s03-v8-weak-wrapper-2026-10-04.md](evidence/s03-v8-weak-wrapper-2026-10-04.md)에 둔다. 해당 고정 fixture에서 attached HostRoot와 detached wrapper root 보존, orphan sweep, 실제 wrapper reset 뒤 재생성 identity를 확인했다. 추가 시뮬레이터 fixture에서는 현재 `spinon.onEvent()` strong persistent callback의 closure root 보존·호출, callback 교체 뒤 회수, 6회 반복 자원 기준선 복귀를 확인했다([반복 실행 근거](evidence/s03-repeat-lifecycle-2026-10-05.md)). 이는 진단용 단일 callback root에 한정된다. 외부 root lease와 DOM listener edge는 구현하지 않았다.

공통 fixture는 [node-lifecycle-v1.rs](../../tests/fixtures/dom/s03/node-lifecycle-v1.rs)다. Rust integration test는 고정 노드·부모·root·문자열 입력에서 연결 성분 결과를 확인하고 weak handle scan, 새 wrapper 객체와 기존 façade ID의 분리, ID 공간 소진, foreign session/document generation·missing node root, malformed parent/child/root/registry 입력의 fail-closed 동작과 failure transaction의 기대 상태를 검사한다. reference model에서 성공 sweep은 document/render revision을 바꾸지 않으며 성공 노드 생성만 document revision을 올린다. fixture 기준은 이 문서의 불변 조건과 S03.2 HostDocument 관계 조회다. 브라우저가 제공하는 native GC 의미를 복제한다고 주장하지 않는다.

별도 Rust integration fixture는 test-only reference model이다. 새 `HostDocument` 제품 코드 검사는 같은 고정 tree fixture의 root 생존 집합·회수 ID·문자열 계수·revision을 실제 Rust core와 bridge에 대조한다. 이 검사만으로 V8 GC 또는 자동 weak-handle reset을 증명하지 않는다. pinned V8 원본은 callback 없는 `SetWeak()`가 도달 불가 객체를 GC가 판정한 뒤 handle을 자동 reset한다고 명시한다. `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` Android·iOS 검증 앱은 강제 full GC 다음 safe point scan, HostRoot·live detached wrapper 보존, orphan WeakRef 제거, wrapper 재생성 뒤 identity를 확인했다([기존 실행 근거](evidence/s03-v8-weak-wrapper-2026-10-04.md)). 이어진 실제 V8 fixture는 현재 native callback closure root, callback 교체 후 회수, 6회 반복 create/detach/drop 뒤 node·UTF-16 문자열·live weak wrapper 기준선 복귀를 확인했다([반복 실행 근거](evidence/s03-repeat-lifecycle-2026-10-05.md)). 프로세스 RSS·V8 retaining path·최대 scan 비용·shutdown race·실기기는 이 결과로 검증하지 않았다.

## 남은 구현·검증 범위

- C++ V8 adapter의 callback 없는 약한 `Global<Object>` registry와 Rust 회수 callback을 outer eval/dispatch owner safe point에 연결했다. 검증 전용 강제 GC entry는 fixture 빌드에만 포함한다.
- Android·iOS 시뮬레이터의 고정 실제 V8 실행에서 기본 root·wrapper fixture와 별도 closure·반복 fixture를 확인했다. 각 실행 evidence가 뒷받침하는 입력·환경 범위만 검증 결과로 취급한다.
- 실제 V8 Android·iOS 시뮬레이터에서 무한 평가 중 대기 eval 세 건을 접수하고 별도 제어 thread에서 종료했다. 활성 평가는 `-8`, 대기 명령과 종료 후 eval/dispatch는 `-6`으로 닫혔고, runtime pointer 해제·worker join·반복 종료를 확인했다([S03.3 종료 경합 기록](evidence/s03-shutdown-2026-10-05.md)). unwind 테스트에서는 작업자 panic 뒤 대기 호출을 `-7`로 거부하고 stale runtime pointer를 지우는 동작도 확인한다. 이 경우 panic 난 owner의 V8 자원을 다른 thread에서 해제하지 않아 누수가 남을 수 있다. raw C ABI 포인터 `free`와 진행 중 FFI call의 동시 실행은 기존 계약으로 금지하며 실제 시뮬레이터 probe에서 실행하지 않는다. probe timeout은 실패를 보고하고 취소를 다시 요청하지만 OS thread를 강제 종료하지 않는다. 비동기 native host 작업·Promise·강제 프로세스 종료도 범위 밖이다.
- 최대 registry 크기에서의 scan 비용은 별도 검증이 필요하다.
- 작은 고정 반복 입력에서 node·UTF-16 string unit·live weak wrapper 수가 기준선으로 복귀하는 것은 확인했다. 합의한 quota 경계의 압력 동작, 장기 반복, 실제 byte·RSS와 회수 재시도 비용은 미검증이다.
- 현재 native callback strong root의 단일 closure 수명을 확인했다. DOM listener·external root lease·임의 closure graph의 root 관계와 V8 heap snapshot retaining path는 별도로 검증한다.
- 이 문서와 상태 대장을 반영하기 전에는 S03.3, J10 또는 공개 DOM 지원을 완료로 표시하지 않는다.

## 참고 경계

S03.2 DOM façade의 공개 후보 범위·예외는 [0020](0020-s03-dom-facade.md), 전체 작업 순서는 [계획](../../plan/dom-node-lifecycle.md), 기존 GC 비교와 React Native·Lynx 수명 조사 근거는 계획 문서에 둔다. Rust callback ABI와 V8 adapter hook은 private 내부 경계이며 공개 JavaScript API가 아니다. 구현된 callback 인자·오류 결과는 위 표를 따른다.
