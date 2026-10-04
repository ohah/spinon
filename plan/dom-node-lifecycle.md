# S03.3 DOM 노드와 JS wrapper 수명 관리 계획

**상태:** 계획 중 · 미구현

**상위 작업:** S03, J10, R03

**현재 계약:** [S03.2 제한 DOM façade `0.1.0`](../spec/internal/0020-s03-dom-facade.md)

## 목적

S03.2는 Rust `HostDocument`를 기준으로 DOM façade를 검증했지만, 노드와 JavaScript wrapper를 세션이 끝날 때까지 유지한다. 이 수명 모델은 단순하고 동작이 예측 가능하지만, 노드를 반복 생성·제거하는 긴 세션에서는 제거된 노드가 계속 한도를 차지한다.

S03.3은 노드 트리의 소유권, JS wrapper identity, 분리 서브트리의 생존 조건, V8 GC와 Rust 회수의 경계, 한도 동작을 고정한 뒤 안전한 회수 경로를 검증한다. 이 계획과 실험은 공개 DOM 지원을 뜻하지 않는다.

## 현재 구현 동작

- `removeChild()`는 부모 연결만 끊는다. 제거된 노드와 자손은 `HostDocument`에 남고 재삽입할 수 있다.
- JS wrapper는 강한 `Map`에 보관되어 같은 handle은 같은 JS 객체로 돌아온다.
- 분리 노드와 wrapper 모두 V8 세션 종료 전에는 회수하지 않는다.
- 제거된 노드도 세션당 16,384개 노드 한도에 포함된다. 연결 중인 노드 수만 세는 정책은 아니다.
- 양수 노드 ID는 세션 안에서 재사용하지 않는다. stale handle이나 재사용 ID에 대한 회수 후 계약은 아직 없다.
- `SpinonV8Runtime`에는 진단용 단일 `v8::Global<v8::Function> event_handler`가 있다. `spinon.onEvent(handler)`는 함수만 받아 이전 handler를 교체하고, `spinon_v8_runtime_free()`는 persistent handle을 reset한다. 등록 해제 API는 없고 DOM 이벤트 listener 구현도 아니다.
- DOM wrapper와 Rust 노드는 numeric handle로 연결된다. JS façade의 `wrappers`는 `Map<id, wrapper>`이고 `nodeIds`는 wrapper에서 ID를 찾는 `WeakMap`이다. FFI query/commit callback은 ID를 `HostDocumentBridge`의 Rust `HostNodeHandle`로 변환해 `HostDocument`를 조회·변경한다. Rust에는 노드별 `v8::Object`/`Global`이 없고, V8 GC에 Rust `HostNode`를 trace하는 embedder hook도 없다. 이는 호출 경계의 연결이지 cross-heap GC 연결은 아니다.
- 따라서 현재 JS `wrappers` Map은 JS wrapper를 세션 동안 강하게 유지하고, Rust `HostDocumentBridge`의 node/handle maps도 제거 노드를 세션 동안 보유한다. 둘 중 한쪽 GC만 켜도 다른 쪽 자원이 회수되는 구조가 아니다.

위 동작은 현재 `0.1.0` 계약이다. 다음 모델을 구현한 것처럼 읽어서는 안 된다.

## Chromium 참고 모델

Chromium의 Blink는 DOM C++ 객체를 Oilpan 관리 heap에 두고 `Member<T>` 등의 추적 참조와 클래스별 `Trace()`로 native 객체의 도달성을 기록한다. V8의 JavaScript heap과 Blink/Oilpan heap은 unified heap 수거에서 서로 건너가는 참조를 함께 추적한다. 그래서 DOM에서 노드를 떼는 동작과 메모리 회수는 별개이며, JS wrapper나 native 객체 그래프가 노드를 계속 도달 가능하게 만들면 살아 있고, 양쪽 heap의 strong root에서 끊긴 뒤 GC가 회수한다. Blink GC는 marking·sweeping을 사용하고 일반적으로 이벤트 루프에서 수거 작업을 예약한다. [Oilpan API와 동작](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/heap/BlinkGCAPIReference.md) · [unified heap과 wrapper tracing 상태](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/TraceWrapperReference.md)

Blink의 `ScriptWrappable`은 C++ DOM 구현과 V8 wrapper를 연결한다. wrapper는 객체 안에 저장하거나, 객체가 살아 있는 동안 wrapper를 보존하는 ephemeron map에 기록한다. 따라서 Chromium 구현을 단일 전역 강한 `Map`이나 wrapper를 무조건 약하게 둔 캐시 하나로 축약하면 안 된다. 정확한 wrapper 연계는 [DOMDataStore](https://chromium.googlesource.com/chromium/src/+/lkgr/third_party/blink/renderer/platform/bindings/dom_data_store.h)와 [ScriptWrappable](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/script_wrappable.h) 구현을 기준으로 한다.

이는 수명 관계를 설계할 비교 모델이지, 스피논의 현재 동작이나 선택된 구현안은 아니다. 스피논은 Rust `HostDocument`의 소유권과 V8 wrapper·callback 참조를 직접 연결할 별도 정책이 필요하다. Blink처럼 실제 JS↔native 참조를 GC가 추적하게 하려면 V8 unified heap/cppgc 연계 수준의 구현이 필요하며, `FinalizationRegistry`만 붙여 같은 동작을 얻을 수는 없다.

Rust 표준 라이브러리는 소유권·`Drop`으로 값을 해제하며, 기본 tracing GC를 제공하지 않는다. `Rc`/`Arc`는 참조 횟수 기반 공유 소유권이고 `Weak`는 그 소유권을 늘리지 않지만, 강한 참조 순환은 자동으로 수거하지 못한다. [Rust 소유권과 Drop](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) · [참조 순환과 Weak](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

따라서 S03.3에서 Rust GC 라이브러리를 도입하는 것은 전제 조건이 아니다. 검토 후보는 Rust `HostDocument`가 노드 저장소를 소유하고, HostRoot와 V8에서 보고한 살아 있는 wrapper root에서 관계 그래프를 추적해 분리·비도달 노드를 안전한 회수 지점에서 제거하는 방식이다. V8 weak persistent callback은 callback 실행 중 Rust 트리를 직접 수정하지 않고 불투명 handle/token만 isolate 소유 경로에 전달한다. Rust 쪽 회수는 callback·JS 실행과 경합하지 않도록 정한 safe point에서 수행한다. 이 방식도 최종 선택 전이며, 별도 Rust GC, refcount 기반 소유, 세션 수명 유지와 구현 비용·주기 참조·결정성을 비교해야 한다.

## JS 함수와 네이티브 callback 참조

앱이나 bootstrap에서 만든 일반 JS 함수는 V8 heap의 객체다. 전역·prototype·Context 등에서 도달 가능한 동안 V8이 보존하고, 도달 불가능해지면 V8 GC 대상이 된다. API 메서드를 Context에 설치했다는 이유만으로 별도 native callback registry에 복사해 보관하지 않는다.

네이티브가 나중에 호출할 JS 함수를 등록하면 상황이 다르다. `v8::Global<v8::Function>` 같은 strong persistent handle은 함수를 루트로 잡아 둔다. 함수의 closure가 캡처한 JS 객체도 함께 살아 있을 수 있다. 그러므로 등록 기능은 callback별 소유자와 유효 구간을 명확히 하고, 해제 지점에서 isolate 소유 실행 경로가 handle을 `Reset()`해야 한다. Rust 코어에는 V8 handle 대신 불투명 callback ID만 전달하고 V8 객체 자체는 C++ runtime 세션이 소유하는 방향을 권한다.

현재 `spinon.onEvent(handler)`는 `SpinonV8Runtime` 하나에 handler 하나를 보관하는 진단 hook이다. 런타임 소유 hook이므로 strong root이며, 앱의 DOM `addEventListener()` 지원·여러 listener의 독립 수명·remove semantics를 검증하지 않는다. 일반 pending host 작업은 완료·취소 전까지 런타임 root로 둘 수 있다. DOM listener는 해당 `EventTarget`이 소유하는 그래프 edge여야 하며, 살아 있지 않은 target까지 영구히 root로 만들지 않도록 별도 수명 계약이 필요하다. 전체 이벤트 동작은 J12 범위다.

`WeakRef`나 JS `FinalizationRegistry`는 active callback을 보유하는 수단으로 사용하지 않는다. V8 문서는 persistent handle을 여러 호출을 넘어 객체를 보유하는 용도로 설명하고, finalizer 실행 시점은 즉시성·순서·실행 자체를 보장하지 않는다고 안내한다. GC는 자원 회수 신호로만 고려하고 listener 해제, 요청 완료·취소, 세션 종료의 필수 정리는 명시적으로 수행한다.

## 검토할 수명 모델

| 모델 | 동작 | 장점 | 주요 비용·위험 |
| --- | --- | --- | --- |
| 세션 수명 유지 | 현재처럼 노드와 wrapper를 세션 종료까지 보존 | 구현이 단순하고 wrapper identity가 명확함 | 반복 생성·제거가 노드·문자열 한도를 소모함 |
| 명시적 dispose | 앱 코드가 분리 노드의 해제를 직접 요청 | 회수 시점과 오류를 명시적으로 다룰 수 있음 | 표준 DOM과 다른 API이며, JS 참조가 남은 노드의 해제는 안전하게 보장하기 어려움 |
| V8 GC 연동 회수 | wrapper를 약한 참조로 캐시하고 문서 트리·살아 있는 JS 참조를 기준으로 분리 노드를 회수 | JS 참조 수명에 가까운 동작과 세션 중 자원 회수를 목표로 할 수 있음 | GC callback 수명·스레드·재진입, 자손 참조가 있는 분리 트리, 테스트 결정성, shutdown 순서를 정해야 함 |

우선 검토 후보는 V8 GC 연동 회수다. 아직 확정된 제품 정책이 아니므로 구현 전에 아래 결정을 계약에 반영한다. 명시적 dispose를 추가하거나 현재 세션 수명 모델을 유지하는 결정도 비교 근거와 함께 기록한다.

## 권고안 · 결정 전

- HostRoot에 연결된 노드는 문서 트리가 살린다. JavaScript에서 wrapper가 계속 참조되는 노드는 그 wrapper가 살린다.
- 런타임이 strong persistent handle로 보유한 pending callback과 그 closure가 참조하는 wrapper는 완료·취소·세션 종료까지 생존 root로 취급한다. 반면 DOM listener는 EventTarget에 소유된 참조 edge로 추적해야 한다. 현재 진단 `spinon.onEvent`만 런타임 전역 root다.
- JS가 참조하는 노드가 분리 서브트리 안에 있으면, `parentNode`·`firstChild`·`nextSibling` 관찰 결과를 보존하도록 해당 분리 서브트리 전체를 유지한다. 어떤 wrapper도 참조하지 않는 분리 서브트리는 회수 후보가 된다.
- wrapper 조회·보관은 weak cache, ephemeron 연결 중 어떤 의미가 필요한지 결정한다. wrapper가 JS에서 살아 있는 동안 같은 handle 조회는 같은 객체를 반환해야 한다. 숫자 node ID는 JS `WeakMap` 키가 될 수 없으므로 `Map<id, WeakRef<wrapper>>`를 택하면 수거된 wrapper의 ID 항목을 정리할 별도 규칙도 필요하다. 필수 회수 시점은 `FinalizationRegistry`에 맡기지 않는다. Chromium의 `DOMDataStore`는 native `ScriptWrappable`이 살아 있는 동안 V8 wrapper를 유지하는 ephemeron map을 사용하므로, 단순한 weak cache 제안을 Chromium과 동등하다고 간주하지 않는다. 스피논의 선택은 detached node·native callback root와 함께 검증한 뒤 계약에 고정한다.
- 첫 회수 구현에서는 노드 ID를 세션 안에서 재사용하지 않는다. 나중에 재사용이 필요해지면 generation을 포함한 handle 계약을 먼저 추가한다.
- 한도는 누적 생성 횟수보다 살아 있는 노드·문자열의 resident 사용량을 기준으로 하는 안을 우선한다. 한도 직전 GC·회수 통지 drain 한 번을 요청하고도 초과하면 `QuotaExceededError`를 반환하는 방안을 비교한다. 강제 GC 지연과 실패의 결정성을 측정한 뒤 최종 선택한다.
- GC 중 callback은 HostDocument를 직접 변경하지 않고 회수 token만 남긴다. GC가 끝난 뒤 isolate 소유 실행 경로에서 token을 적용하는 방안을 우선한다. 세션 종료는 callback 등록을 먼저 닫고 pending token을 drain한 다음 Rust 문서를 파기한다.

위 항목은 다음 설계를 시작하기 위한 권고안이다. `0020`의 현재 시제품 계약을 바꾸지 않으며, 구현 계약을 고정하기 전에 각 항목을 검증·확정한다.

### Rust `HostDocument` 회수 흐름 후보 · 미구현

1. Rust가 노드를 만들고 불투명 handle을 반환한다. JS wrapper는 이 handle로 조회·변경 요청을 보낸다.
2. 연결된 노드는 HostRoot의 트리 관계가 살린다. 앱 변수나 pending callback closure가 wrapper를 참조하면 V8의 일반 도달성 그래프가 wrapper를 살린다.
3. `removeChild()`는 부모 연결만 끊는다. 노드 저장소는 여전히 값을 보유하지만, 저장소 보유 자체는 아래 논리적 도달성 검사에서 생존 root로 세지 않는 수거 구조를 검토한다.
4. wrapper를 약하게 추적하는 handle은 V8 GC가 wrapper를 도달 불가능하다고 판단하면 정리 통지를 낸다. 통지는 Rust 트리를 즉시 수정하지 않고 node handle/token을 isolate 소유 queue에 보낸다. JS 참조 또는 strong pending callback closure가 남아 있으면 통지는 아직 오지 않으므로 wrapper와 노드는 보수적으로 유지된다.
5. 정한 안전 지점에서 Rust가 HostRoot와 아직 살아 있는 wrapper handle부터 트리를 추적한다. 제거된 subtree가 이 root들에서 도달 불가능할 때만 노드·문자열·handle mapping을 회수하고 quota를 갱신한다. 회수 통지 전까지는 메모리와 quota 회수가 늦어질 수 있다.
6. DOM listener는 런타임 전역 pending callback과 다르다. listener와 target은 EventTarget이 소유하는 참조 관계여야 한다. 그 cross-heap edge의 추적·해제 계약은 J12에서 결정한다.

이 흐름은 Rust에 tracing GC를 추가하지 않는 후보지만, 지금의 `HostDocument` BTreeMap과 `wrappers` 강한 Map에 구현되어 있지 않다. wrapper 보관, V8 weak handle 알림, Rust 안전 지점 sweep, callback owner 간 계약이 모두 준비되기 전에는 회수했다고 간주하지 않는다.

## 구현 전에 확정할 결정

1. **살아 있는 노드의 정의:** 권고안의 detached subtree 전체 보존이 최소 Node 관계 API와 맞는지 확인하고, HostRoot·wrapper root에서 정확히 도달성 추적할 범위를 정한다.
2. **wrapper identity:** wrapper가 JS에서 살아 있는 동안 같은 HostDocument handle 조회가 같은 객체를 반환하는지, wrapper가 수거된 뒤 재생성할 수 있는지 확정한다.
3. **GC와 Rust 변경 경계:** GC callback이 V8·Rust 객체를 직접 변경하지 않도록 할지, 회수 통지를 어느 isolate 소유 실행 경로에서 적용할지, 세션 종료와 경쟁할 때 폐기 순서를 정한다.
4. **handle 유효성:** 권고한 세션 내 ID 비재사용과 stale native handle의 실패 결과를 확정한다. ID를 재사용한다면 generation 검증을 포함한다.
5. **quota 의미:** 노드 수와 보존 문자열 수를 live resident 자원량으로 세는 안, 한도 직전 회수 시도, 실패 오류를 확정한다. GC 시점에 따른 성공·실패 차이와 지연 비용을 측정한다.
6. **결정적 검증:** 테스트 전용 GC 요청·회수 queue drain 제어를 둘지 정한다. 제품 공개 API로 노출하지 않는다.
7. **함수 callback의 종료:** 런타임 pending 작업 callback과 EventTarget 소유 listener를 구분한다. callback ID별 handle의 owner, strong root 또는 node-owned edge 의미, 등록 해제·호출 완료·취소·target 회수·세션 종료 때 `Reset()`/trace 갱신 순서를 정한다. V8 handle의 접근·reset은 해당 Isolate의 실행 owner를 따른다.

## 검증 기준

계약과 구현은 아래 경로를 Android·iOS의 실제 V8 실행과 Rust/Bun 자동화에서 검증한다.

### 의도적 보존과 누수 판별

- **현재 시제품 기준선:** 노드를 반복 생성하고 `removeChild()`한 뒤 Rust `HostDocumentBridge`의 live handle/node 수와 보존 문자열 사용량을 기록한다. 지금 계약에서는 제거 노드가 계속 남는 것이 예상 결과다. `wrappers.size` 계측 또는 V8 heap snapshot에서 wrapper를 보유하는 경로를 확인해 강한 Map 보존을 따로 증명한다. 이는 현 정책의 관찰이지, GC 회수가 된다는 검증이 아니다.
- **회수 후보 검증:** 테스트 전용 설정에서 V8 full GC를 요청하고 weak-handle 통지 queue를 안전 지점에서 drain한 뒤 Rust node/handle/string 수와 V8 wrapper 객체 수를 기록한다. 테스트는 JS 지역 참조를 scope 밖으로 내보낸 다음 별도 GC turn에서 수거를 요청해 stack root가 결과를 왜곡하지 않게 한다. V8의 `RequestGarbageCollectionForTesting()`은 `--expose_gc`가 필요한 test-only API이며 제품의 수거 시점 제어로 쓰지 않는다. [V8 Isolate API](https://v8.github.io/api/head/classv8_1_1Isolate.html)
- **root 대조군:** 연결 노드는 wrapper 참조를 버려도 유지되어야 한다. 분리 노드는 JS 전역 변수나 대기 중 callback closure가 wrapper를 붙잡는 동안 유지되어야 하며, root를 놓고 GC·queue drain 뒤에는 회수 후보가 되어야 한다. pending callback 완료·취소의 양쪽을 확인하고, DOM listener owner 그래프는 J12 계약에 따라 별도 대조한다.
- **반복 기준:** warm-up 뒤 create·detach·drop·collect를 여러 차례 반복해 Rust의 live resource count와 V8 snapshot object count가 안정 기준선으로 돌아오는지 확인한다. snapshot의 retaining path에서 예상 밖 strong `Map`·`Global`을 추적한다. `used_heap_size`와 프로세스 RSS는 보조 지표이며 RSS 하나만으로 누수를 판정하지 않는다. GC가 객체를 회수해도 allocator page가 즉시 OS에 반환되지 않을 수 있다. [V8 HeapProfiler API](https://v8.github.io/api/head/classv8_1_1HeapProfiler.html)

- 연결된 노드의 wrapper만 사라져도 HostDocument 노드와 트리가 유지되고, 다음 조회에서 유효한 wrapper를 만들 수 있다.
- 살아 있는 JS 참조가 있는 분리 노드는 조회·수정·재삽입 가능하다.
- 분리된 자손 wrapper가 살아 있는 경우 필요한 부모·형제 관계가 회수 때문에 깨지지 않는다.
- JS 참조가 없는 분리 서브트리는 합의한 안전 지점에서 회수되고, 재사용되지 않는 ID의 stale 접근은 실패 폐쇄한다.
- 같은 wrapper가 살아 있는 동안 handle 조회가 identity를 유지한다. 수거 이후 identity 보장은 계약에 적힌 범위를 따른다.
- 런타임 pending callback은 비동기 호출 대기 중 계속 호출 가능하고, 등록 해제·교체·취소·세션 종료 뒤에는 더 호출되지 않으며 strong root가 해제된다.
- 런타임 pending callback의 closure가 분리 노드 wrapper를 캡처하면 callback이 살아 있는 동안 노드 관계가 유지되고, callback을 해제한 뒤 다른 JS root가 없다면 subtree 회수 대상이 된다. DOM listener의 node-owned edge와 target 수명은 J12에서 별도 검증한다.
- 다른 Isolate의 callback ID, 중복 해제, 이미 종료된 세션의 완료 응답은 잘못된 접근으로 실패 폐쇄한다.
- 반복 create/remove/collect에서 노드·문자열·wrapper 자원 사용량이 합의한 한도 안에 머문다. 수거 전에 한도에 도달하는 경우 오류와 회수 재시도 규칙이 결정적이다.
- 세션 종료 중 pending GC 통지, callback 중복, 다른 세션 handle, Rust panic·할당 실패가 다른 노드나 세션을 손상하지 않는다.

완료하려면 버전 있는 내부 인터페이스 계약, 실패 경로 fixture, Rust·Bun 검증, Android·iOS 시뮬레이터의 실제 V8 실행 근거를 같은 변경 흐름에 연결한다. 구현 전에는 이 문서의 미결정 항목을 해결하고, S03.3과 J10을 완료로 표시하지 않는다.

## 범위 밖

`textContent` setter, `NodeList`·`HTMLCollection`, selector API, 전체 DOM/Web IDL 적합성, 여러 Owner의 동시 수정, DOM 이벤트 등록·제거·캡처·전파 semantics, CSS invalidation, layout·GPU 연결은 이 작업에 포함하지 않는다. callback persistent handle의 수명과 node collector 간 root 관계만 여기서 선행 검증하고, 전체 이벤트 계약은 J12에서 따로 정한다.
