# S03.3 DOM 노드와 JS wrapper 수명 관리 계획

**상태:** Rust/V8 동적 회수 경로 구현 · Android·iOS 시뮬레이터에서 16,385개 wrapper 생성·회수와 6회 반복 기준선 복귀 통과 · 한 번씩 관측한 scan 시간은 벤치마크가 아니며 비용 곡선·할당 실패·listener/external root 수명 검증 진행 중

**상위 작업:** S03, J10, R03

**내부 계약:** [0021 DOM 노드·wrapper 수명 계약 `0.1.0-draft`](../spec/internal/0021-s03-dom-node-lifecycle.md)
**결정성 fixture:** [node-lifecycle-v1.rs](../tests/fixtures/dom/s03/node-lifecycle-v1.rs)

**현재 계약:** [S03.2 제한 DOM façade `0.1.0`](../spec/internal/0020-s03-dom-facade.md)

## 목적

S03.2는 Rust `HostDocument`를 기준으로 DOM façade를 검증했지만, 노드와 JavaScript wrapper를 세션이 끝날 때까지 유지한다. 이 수명 모델은 단순하고 동작이 예측 가능하지만, 노드를 반복 생성·제거하는 긴 세션에서는 제거된 노드가 계속 한도를 차지한다.

S03.3은 노드 트리의 소유권, JS wrapper identity, 분리 서브트리의 생존 조건, V8 GC와 Rust 회수의 경계, 한도 동작을 고정한 뒤 안전한 회수 경로를 검증한다. 이 계획과 실험은 공개 DOM 지원을 뜻하지 않는다.

## S03.2 기준선과 현재 구현 상태

- `removeChild()`는 부모 연결만 끊는다. 제거된 노드와 자손은 `HostDocument`에 남고 재삽입할 수 있다.
- S03.2 기준선은 JS 강한 `Map`과 세션 수명 보존이었다. 이번 변경은 JS 강한 wrapper `Map`을 제거하고 C++의 weak `Global<Object>` registry를 사용한다.
- resident node 개수에는 고정 quota를 두지 않는다. 연결에서 분리된 노드도 sweep까지 resident 사용량으로 계측하며, 양수 i32 façade ID와 Rust NodeId 공간 소진은 검사한다.
- Rust core/bridge 회수 callback과 C++ owner safe-point scan을 연결했다. Android·iOS 실제 V8 고정 fixture 실행은 [검증 근거](../spec/internal/evidence/s03-v8-weak-wrapper-2026-10-04.md)와 원본 로그에 기록했다. 회수 뒤 stale handle과 ID 동작은 [0021](../spec/internal/0021-s03-dom-node-lifecycle.md)에 정의했다.
- `SpinonV8Runtime`에는 진단용 단일 `v8::Global<v8::Function> event_handler`가 있다. `spinon.onEvent(handler)`는 함수만 받아 이전 handler를 교체하고, `spinon_v8_runtime_free()`는 persistent handle을 reset한다. 등록 해제 API는 없고 DOM 이벤트 listener 구현도 아니다.
- C++ weak registry는 `std::map<facade ID, v8::Global<Object>>`이며 JS `nodeIds` WeakMap은 wrapper에서 façade ID를 찾는다. FFI query/commit/collect callback은 façade ID를 Rust `HostNodeHandle`로 검증·변환한다. Rust는 V8 객체를 직접 trace하지 않고, 안전 지점에서 전달받은 live ID 집합을 별도 HostDocument 도달성 계산에 사용한다.

### 비교 프레임워크의 노드 저장 코드

React Native Fabric의 `ShadowNode`는 자식을 `std::vector`로 보유하고 `ShadowTreeRegistry`는 `unordered_map<SurfaceId, ShadowTree>`를 사용한다. `Tag`와 `SurfaceId`는 `int32_t`지만 확인한 저장·등록 경로에는 고정 live-node quota 검사가 없다. 레거시 Android `ShadowNodeRegistry`도 `SparseArray`에 추가·삭제하며 노드 수 상한을 검사하지 않는다. Lynx `NodeManager`는 `boost::unordered_flat_map<int, Element*>`로 요소를 등록하고 `NodeCount()`를 제공하며, 요소 ID는 `int32_t` 증가 함수로 발급한다. 확인한 경로에는 고정 live-element quota가 없다. 이 비교는 메모리·ID·프레임 시간 한계가 없다는 뜻이 아니며, Spinon의 임의 16,384 live-node 거부를 정당화하지 않는다. [RN Fabric ShadowNode](https://github.com/facebook/react-native/blob/024b474ce92ba66744af602ee67691cd11498485/packages/react-native/ReactCommon/react/renderer/core/ShadowNode.h) · [RN ShadowTreeRegistry](https://github.com/facebook/react-native/blob/024b474ce92ba66744af602ee67691cd11498485/packages/react-native/ReactCommon/react/renderer/mounting/ShadowTreeRegistry.h) · [RN Tag](https://github.com/facebook/react-native/blob/024b474ce92ba66744af602ee67691cd11498485/packages/react-native/ReactCommon/react/renderer/core/ReactPrimitives.h) · [Lynx NodeManager와 element ID](https://github.com/lynx-family/lynx/blob/ebb5086212257de51e4632d058fb766b9a435575/core/renderer/dom/element_manager.h) · [Lynx ID 발급](https://github.com/lynx-family/lynx/blob/ebb5086212257de51e4632d058fb766b9a435575/core/renderer/dom/element_manager.cc)

### handle 용어

- **JS façade ID:** JavaScript `wrappers`와 FFI가 쓰는 양수 `i32` 키다. `HostDocumentBridge`에서 Rust handle로 변환한다. ID는 JS 객체가 아니라 노드에 귀속되므로 wrapper만 GC되고 노드가 다른 root로 살아 있으면 ID↔handle mapping을 유지한다.
- **Rust `HostNodeHandle`:** 문서 generation과 코어 node ID를 가진 Rust 측 handle이다. JS façade ID와 같은 값이나 같은 타입이라고 가정하지 않는다.
- **wrapper weak handle:** C++ runtime이 JS façade ID별로 보유하는 약한 `v8::Global<Object>`다. Rust callback이 live ID를 현재 `HostNodeHandle`로 검증한다. wrapper가 살아 있는 동안 canonical identity를 재사용하고, V8이 handle을 자동 reset한 뒤에는 같은 registry slot에 새 wrapper를 둘 수 있다.

S03.3에서는 JS façade ID를 다른 노드에 재사용하지 않는다. `HostDocumentBridge`는 살아 있는 매핑과 독립된 단조 증가 high-water mark를 보유한다. `QUERY_NEXT_ID`는 ID를 예약하고, 생성 변경 묶음이 실패해도 발급된 ID는 건너뛴다. 대기 예약은 256개로 제한하고, 양수 `i32` 공간이 소진되면 `QuotaExceededError`로 새 노드 생성을 닫는다. 수거는 매핑을 제거해도 high-water mark를 되돌리지 않는다. wrapper registry key는 V8 callback token 없이 JS façade ID를 사용한다.

위 동작은 현재 `0.1.0` 계약과 구현 상태 요약이다. 한 개의 고정 시뮬레이터 fixture 통과를 반복 수명·제품 완료로 확대 해석하지 않는다.

## 선택한 회수 방향 · 동적 노드 저장과 Rust/V8 회수 경로

Rust HostDocument가 노드 저장소와 mark-and-sweep을 소유하고, V8 C++ adapter가 callback 없는 약한 handle의 자동 reset을 owner 안전 지점에서 검사한다. V8 unified heap이나 별도 Rust tracing-GC 의존성은 두지 않는다. node 개수 quota는 두지 않고 JS ID·Rust NodeId 소진을 검사한다. wrapper·root·회수 결과 buffer는 필요량에 따라 동적으로 늘린다. 호출 경계·root·문자열 예산·전체 scan의 비용·할당 실패와 shutdown 순서는 [내부 계약 0021](../spec/internal/0021-s03-dom-node-lifecycle.md)에 둔다.

이번 결정은 첫 공식 릴리스 전 내부 `0.1.0` 시제품의 이전 16,384 resident-node quota를 갱신한다. 과거 고정 상한 근거는 당시 동작의 역사적 기록으로 남긴다. Rust `HostDocument` 도달성 계획·commit, bridge의 node/handle/string map 회수 callback, C++ weak `Global` scan과 owner safe-point caller는 구현돼 있다. 이 변경은 C++ registry entry 제한을 제거하고 scan/output buffer를 동적으로 키우며, node count와 문자열 수·scan buffer byte 진단을 연결한다. Rust/C++ 회수 결과 불변식이 깨지면 세션을 poisoned 처리하고 후속 JS 실행을 막는다. 강제 GC 요청 entry는 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 검증 빌드에서만 노출한다. Android·iOS 실제 V8 시뮬레이터에서 16,385개 wrapper 생성·보존·회수와 Rust 자원 기준선 복귀를 확인했다([대량 registry 실행 근거](../spec/internal/evidence/s03-dynamic-wrapper-registry-2026-10-05.md)). 16,394개 live wrapper를 관찰한 단일 scan은 Android 15,273μs, iOS 7,067μs였으나 분포·프레임 비용이 없는 시뮬레이터 표본이므로 성능 보장에 쓰지 않는다. Android 검증은 emulator 호환을 위해 해당 V8 artifact의 CFI를 끈 조건이었다. 동적 buffer 할당 실패 주입, 더 큰 입력의 비용 곡선과 실기기 검증은 남는다. Rust 표준 트리 컨테이너의 OS 메모리 고갈에서 프로세스 복구는 보장하지 않는다.

## Chromium 참고 모델

Chromium의 Blink는 DOM C++ 객체를 Oilpan 관리 heap에 두고 `Member<T>` 등의 추적 참조와 클래스별 `Trace()`로 native 객체의 도달성을 기록한다. V8의 JavaScript heap과 Blink/Oilpan heap은 unified heap 수거에서 서로 건너가는 참조를 함께 추적한다. 그래서 DOM에서 노드를 떼는 동작과 메모리 회수는 별개이며, JS wrapper나 native 객체 그래프가 노드를 계속 도달 가능하게 만들면 살아 있고, 양쪽 heap의 strong root에서 끊긴 뒤 GC가 회수한다. Blink GC는 marking·sweeping을 사용하고 일반적으로 이벤트 루프에서 수거 작업을 예약한다. [Oilpan API와 동작](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/heap/BlinkGCAPIReference.md) · [unified heap과 wrapper tracing 상태](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/TraceWrapperReference.md)

Blink의 `ScriptWrappable`은 C++ DOM 구현과 V8 wrapper를 연결한다. wrapper는 객체 안에 저장하거나, 객체가 살아 있는 동안 wrapper를 보존하는 ephemeron map에 기록한다. 따라서 Chromium 구현을 단일 전역 강한 `Map`이나 wrapper를 무조건 약하게 둔 캐시 하나로 축약하면 안 된다. 정확한 wrapper 연계는 [DOMDataStore](https://chromium.googlesource.com/chromium/src/+/lkgr/third_party/blink/renderer/platform/bindings/dom_data_store.h)와 [ScriptWrappable](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/script_wrappable.h) 구현을 기준으로 한다.

Blink는 JS와 native 객체 그래프를 unified heap에서 함께 추적하는 비교 모델이다. 스피논은 이를 그대로 택하지 않고 Rust 소유 문서와 Rust 회수기를 유지한다. V8은 약한 `Global`의 자동 reset을 제공하고 Rust가 자체 root 그래프를 추적한다. `FinalizationRegistry`나 callback형 weak handle의 비결정적 실행을 필수 정리나 정확한 quota 회수 시점으로 사용하지 않는다.

Rust 표준 라이브러리는 소유권·`Drop`으로 값을 해제하며, 기본 tracing GC를 제공하지 않는다. `Rc`/`Arc`는 참조 횟수 기반 공유 소유권이고 `Weak`는 그 소유권을 늘리지 않지만, 강한 참조 순환은 자동으로 수거하지 못한다. [Rust 소유권과 Drop](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) · [참조 순환과 Weak](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Rust 표준 라이브러리는 기본 tracing GC를 제공하지 않으므로, 선택한 방향에서는 Rust 소유권과 안전 지점의 도달성 sweep으로 노드 저장소를 회수한다. V8 weak handle registry는 isolate별 C++ runtime이 소유하고, owner safe point에서 전체 검사한 생존 handle 목록을 Rust에 전달한다. 별도 Rust GC, refcount 기반 노드 소유, V8 unified heap은 이 단계의 선택 모델이 아니다. V8 GC 전에는 handle이 빈 상태가 아닐 수 있어 노드를 보수적으로 더 오래 보유한다.

## JS 함수와 네이티브 callback 참조

앱이나 bootstrap에서 만든 일반 JS 함수는 V8 heap의 객체다. 전역·prototype·Context 등에서 도달 가능한 동안 V8이 보존하고, 도달 불가능해지면 V8 GC 대상이 된다. API 메서드를 Context에 설치했다는 이유만으로 별도 native callback registry에 복사해 보관하지 않는다.

네이티브가 나중에 호출할 JS 함수를 등록하면 상황이 다르다. `v8::Global<v8::Function>` 같은 strong persistent handle은 함수를 루트로 잡아 둔다. 함수의 closure가 캡처한 JS 객체도 함께 살아 있을 수 있다. 그러므로 등록 기능은 callback별 소유자와 유효 구간을 명확히 하고, 해제 지점에서 isolate 소유 실행 경로가 handle을 `Reset()`해야 한다. Rust 코어에는 V8 handle 대신 불투명 callback ID만 전달하고 V8 객체 자체는 C++ runtime 세션이 소유하는 방향을 권한다.

현재 `spinon.onEvent(handler)`는 `SpinonV8Runtime` 하나에 handler 하나를 보관하는 진단 hook이다. 런타임 소유 hook이므로 strong root이며, 앱의 DOM `addEventListener()` 지원·여러 listener의 독립 수명·remove semantics를 검증하지 않는다. 일반 pending host 작업은 완료·취소 전까지 런타임 root로 둘 수 있다. DOM listener는 해당 `EventTarget`이 소유하는 그래프 edge여야 하며, 살아 있지 않은 target까지 영구히 root로 만들지 않도록 별도 수명 계약이 필요하다. 전체 이벤트 동작은 J12 범위다.

`WeakRef`나 JS `FinalizationRegistry`는 active callback을 보유하는 수단으로 사용하지 않는다. V8 문서는 persistent handle을 여러 호출을 넘어 객체를 보유하는 용도로 설명하고, finalizer 실행 시점은 즉시성·순서·실행 자체를 보장하지 않는다고 안내한다. GC는 자원 회수 신호로만 고려하고 listener 해제, 요청 완료·취소, 세션 종료의 필수 정리는 명시적으로 수행한다.

## 비교 프레임워크: React Native와 Lynx

React Native 새 아키텍처에서 JSI는 JavaScript와 C++ 객체가 서로를 참조하고 직접 호출할 수 있게 하는 runtime 경계다. JSI 헤더에는 `HostObject`, JS 객체에 붙는 `NativeState`, `WeakObject` 생성·잠금 API가 있다. JSI는 엔진 값을 다루는 C++ 인터페이스다. JSI 자체가 임의의 Rust heap까지 자동 추적하는 unified GC를 제공하지 않는다는 판단은 API 경계에 대한 설계상 추론이다. Fabric은 JS React Element Tree에서 C++의 immutable React Shadow Tree를 만들고, 새 tree를 commit한 뒤 플랫폼 UI thread에서 host view를 mount한다. 소스의 `ShadowNode`는 자식·props·state 등에 `shared_ptr` 소유권을 쓰고 runtime wrapper 연결은 `weak_ptr`로 둔다. 따라서 참고할 점은 GC 통합 자체보다 JS runtime handle, C++ tree ownership, native view 수명을 각 경계에서 관리한다는 것이다. [React Native 새 아키텍처와 JSI](https://reactnative.dev/architecture/landing-page) · [Fabric render pipeline](https://reactnative.dev/architecture/render-pipeline) · [`ShadowNode.h`](https://github.com/react/react-native/blob/main/packages/react-native/ReactCommon/react/renderer/core/ShadowNode.h) · [JSI API](https://github.com/react/react-native/blob/main/packages/react-native/ReactCommon/jsi/jsi/jsi.h)

Lynx는 main thread와 background thread에 별도 JavaScript runtime을 둔다. 문서는 main runtime으로 Lynx의 PrimJS를, background runtime으로 Android에서는 기본 PrimJS, iOS에서는 기본 JavaScriptCore를 설명한다. PrimJS는 QuickJS 기반으로 시작했지만 별도 GC를 사용한다고 프로젝트 저장소가 밝힌다. main-thread Element PAPI는 Lynx Element를 감싼 고성능 FFI 경로이며, 문서상 해당 JS object는 일반적인 getter/setter를 붙이는 Object Model과 다르다. Lynx의 memory query는 Element tree의 bytes·node count, UI/View, main/background runtime heap을 따로 집계하고, heap snapshot으로 JS retainer chain을 조사하도록 안내한다. 공개 설명만으로 Lynx의 모든 native↔JS 수거 규칙을 추정하지 않는다. [Lynx runtime 구성](https://lynxjs.org/next/guide/scripting-runtime/index.html) · [main-thread Element PAPI](https://lynxjs.org/guide/scripting-runtime/main-thread-runtime) · [PrimJS 저장소](https://github.com/lynx-family/primjs) · [Lynx memory query](https://lynxjs.org/next/guide/performance/monitor-performance/global-memory-usage-query)

스피논은 RN의 JSI를 사용자 API로 그대로 노출하거나 Lynx의 다중 runtime 구조를 복제하지 않는다. 앱 작성자에게 C++ `jsi::Runtime`·V8 handle을 공개하는 계획은 현재 없다. 사용자 정의 native 기능은 타입·권한·오류·취소·thread affinity를 명시하는 버전 있는 X08/J15 플랫폼 모듈 계약으로 연결한다. 수명 진단은 선택한 Rust 소유 문서의 live node·문자열 계수와 V8 heap snapshot의 retaining path를 함께 본다. 이는 위 프레임워크들과 동일한 구현이라는 주장이 아니라, 서로 다른 heap과 UI tree를 따로 계측해야 한다는 비교 근거다.

## 대안 비교와 선택

| 모델 | 동작 | 판단 |
| --- | --- | --- |
| 세션 수명 유지 | 현재처럼 노드와 wrapper를 세션 종료까지 보존 | S03.2의 기준 동작으로 유지한다. S03.3의 목표 회수 모델은 아니다. |
| 명시적 dispose | 앱 코드가 분리 노드의 해제를 직접 요청 | 일반 DOM 수명 관리의 대체 방식으로 채택하지 않는다. 필요하면 별도 자원 API에서만 검토한다. |
| V8 unified heap/cppgc | JS wrapper와 native 노드 그래프를 한 수거 경계에서 추적 | Rust `HostDocument`의 소유권을 유지하는 현재 방향에서는 채택하지 않는다. |
| Rust 소유 mark-and-sweep + V8 약한 wrapper handle scan | V8의 자동 reset 약한 handle을 owner safe point에서 검사하고 Rust가 HostRoot와 명시된 참조 root에서 분리 노드 도달성을 추적·회수 | **선택한 설계 방향.** Rust core/bridge, C++ scan, Android·iOS 고정 fixture를 확인했다. 반복 수명·shutdown·성능 증거가 남아 상위 기능은 미완료다. |

## 확정한 생존 규칙 · 제품 구현 미완료

- HostRoot, non-empty 약한 wrapper handle, 유효한 external root를 seed로 삼고 부모·자식 양방향 연결 성분 전체를 유지한다.
- 살아 있는 wrapper 조회는 canonical JS identity를 보존한다. V8이 약한 handle을 자동 reset한 뒤 노드가 다른 root에 살아 있으면 새 wrapper를 허용한다.
- JS façade ID와 Rust NodeId는 세션 안에서 각각 단조 증가하며 재사용하지 않는다. resident node 개수는 고정 제한이 없고 문자열 예산은 UTF-16 16,777,216 코드 단위다. checked ID 소진·문자열 예산·단일 묶음 크기 실패는 호출의 부분 상태를 공개하지 않는다.
- V8 callback 없는 weak `Global<Object>` registry를 매 outer task safe point에서 선형 검사한다. registry와 root snapshot은 필요량에 따라 확장하고 entry 수·scan 시간·root/output buffer payload bytes를 진단한다. scan 비용은 현재 registry 크기에 선형 비례한다.
- strong V8 callback handle은 owner isolate에서 교체·완료·취소·종료 때 Reset한다. Rust handle을 보유하는 비동기 host 작업은 세션당 최대 16,384개 external root lease 중 하나를 등록하며, 한도 초과는 작업 등록 단계에서 거부한다. DOM listener는 미구현이며 cross-heap listener cycle은 J12에서 다룬다.
- 기존 결정성 fixture는 독립 Rust reference model이다. Rust runtime test는 같은 tree fixture를 실제 core/bridge에 적용한다. Android·iOS fixture probe는 attached HostRoot, detached child wrapper root, orphan weak reference, wrapper 재생성을 확인한다. 이 probe는 force-GC macro 빌드에서만 실행 가능하며, simulator 근거와 repeat/shutdown/closure-root 범위는 별도로 기록한다.

### Rust `HostDocument` 회수 흐름 · runtime 연결 및 고정 fixture 통과

1. Rust가 노드를 만들고 불투명 handle을 반환한다. JS wrapper는 이 handle로 조회·변경 요청을 보낸다.
2. 연결된 노드는 HostRoot의 트리 관계가 살린다. 앱 변수나 pending callback closure가 wrapper를 참조하면 V8의 일반 도달성 그래프가 wrapper를 살린다.
3. `removeChild()`는 부모 연결만 끊는다. 노드 저장소는 물리적으로 값을 보유하지만 저장소 보유 자체는 생존 root가 아니다. 안전 지점에서 정한 root 그래프가 도달하지 않는 분리 노드만 sweep한다.
4. wrapper는 callback 없는 약한 `Global<Object>`로 registry에 둔다. V8 GC가 wrapper를 도달 불가능하다고 판단하면 handle을 자동 reset한다. V8 callback형 weak handle은 호출 시점과 실행 자체가 보장되지 않아 회수 경로에 쓰지 않는다. JS 참조나 wrapper를 캡처한 strong `Global<Function>`이 남아 있으면 약한 handle이 non-empty이므로 노드는 보수적으로 유지된다. EventTarget listener의 strong callback cycle은 이 모델만으로 회수되지 않아 J12 해결 전까지 지원 완료로 간주하지 않는다.
5. 가장 바깥 JS 작업과 microtask checkpoint가 끝난 안전 지점마다 C++가 전체 façade ID keyed wrapper weak handle을 검사하고 non-empty ID를 root snapshot에 넣는다. Rust callback은 ID를 HostNodeHandle로 검증·변환하고 HostRoot와 함께 도달성을 계산한다. wrapper가 비었어도 노드가 다른 root로 살아 있으면 JS façade ID↔handle 매핑을 유지한다. 모든 root에서 도달 불가해 노드를 회수할 때 해당 mapping과 문자열 계수를 같은 sweep commit에서 갱신한다. HostRoot 트리와 스타일·레이아웃 입력은 바뀌지 않으므로 sweep은 `DocumentRevision`과 `RenderTreeRevision`을 올리지 않는다. V8 GC 실행 전까지는 실제 메모리 회수가 늦어질 수 있다.
6. DOM listener는 런타임 전역 pending callback과 다르다. 의미상 listener와 target은 EventTarget이 소유하는 참조 관계다. strong `v8::Global<Function>`이 callback closure를 통해 wrapper를 살리면 target 수거를 막는 순환이 생길 수 있으므로, 그 cross-heap edge의 추적·해제 및 순환 회수 계약은 J12에서 결정한다.

도달성 검증·Rust sweep·bridge 매핑 및 UTF-16 사용량 정리는 Rust callback에 연결되어 있다. JS 강한 wrapper `Map`은 제거했다. Rust callback의 오류는 성공한 JavaScript 상태를 덮어쓰지 않고 세션 report의 `document_collection_error`에 별도 기록한다. Android·iOS 고정 fixture에서 진단용 callback closure, 작은 반복 수명 기준선, 16,385개 wrapper 동적 확장·회수와 제한된 shutdown 경합은 통과했다. external root lease와 DOM listener edge, 문자열 예산 경계·장기 반복, 반복 가능한 scan 비용 곡선과 동적 buffer 할당 실패 주입은 남아 있다.

## 확정한 구현 결정

1. **살아 있는 노드:** HostRoot·non-empty weak wrapper handle·external root에서 부모와 자식 방향 모두로 도달하는 연결 성분을 보존한다.
2. **wrapper identity:** 살아 있는 동안 canonical identity를 유지하고, V8이 약한 handle을 자동 reset한 뒤 살아남은 노드는 새 wrapper를 발급할 수 있다.
3. **GC와 변경 경계:** 최외곽 JavaScript 작업과 microtask checkpoint가 끝난 isolate owner 안전 지점에서 전체 weak registry scan과 sweep을 처리한다. callback 중 Rust 접근·재진입 sweep은 없다.
4. **ID와 handle:** JS façade i32 ID와 HostNodeHandle의 DocumentGeneration/NodeId는 별도 identity다. façade ID는 노드에 매핑되므로 살아 있는 노드의 wrapper가 GC된 뒤 재생성해도 유지하고, 노드 회수 뒤 다른 노드에 재사용하지 않는다. stale 접근은 실패 폐쇄한다. C++ wrapper registry는 façade ID로 keying하고 Rust가 HostNodeHandle을 검증한다.
5. **resource budget:** resident node 개수 quota는 없다. 양수 i32 façade ID와 Rust NodeId는 재사용하지 않으며 공간 소진 시 명시적으로 실패한다. resident string은 16,777,216 UTF-16 code units, pending ID reservation은 256개, external root lease는 16,384개까지다. 문자열·reservation·external-root 한도 초과는 QuotaExceededError/backpressure로 원자 실패하며 강제 full GC나 같은 호출 재시도는 하지 않는다. OS 메모리 고갈은 모든 Rust 컨테이너에서 복구 가능한 오류로 보장하지 않는다.
6. **결정성 검증:** 독립 Rust reference fixture가 root graph·weak handle reset 모델·wrapper 재생성 기대값을 고정한다. 별도 runtime test는 같은 고정 tree의 생존 집합·회수 ID·문자열 계수·revision을 실제 core/bridge 코드에서 대조한다. simulator에서 확인한 항목은 각 실행 evidence에 한정한다. 진단용 callback closure와 6×32 반복 회수, 16,385개 wrapper의 동적 확장·회수, 제한된 종료 경합은 확인했다. 문자열 예산 경계·장기 반복·반복 가능한 scan 비용 곡선·동적 buffer 할당 실패 주입은 남은 항목으로 추적한다.
7. **callback 수명:** strong callback Global은 owner isolate에서 교체·완료·취소·종료 시 Reset한다. Rust handle을 직접 보유하는 host 작업은 세션당 16,384개 이하 external root lease를 둔다. 한도 초과는 backpressure로 거부하고 token은 재사용하지 않는다. DOM listener는 미구현으로 남기고 J12가 cycle 계약을 정한다.
8. **dynamic scan:** isolate별 wrapper registry와 root snapshot은 동적으로 확장한다. 전체 선형 검사는 각 outer task safe point에서 수행하며 entry 수·마지막 소요 시간·root/result vector의 payload capacity bytes를 진단한다. byte 값은 map entry·allocator overhead·Rust 문서 저장소나 전체 프로세스 메모리를 포함하지 않는다. 큰 문서에서의 frame 영향은 구현 검증으로 측정한다.
9. **회수 실패 원자성:** mark·worklist·sweep 삭제 목록을 적용 전 구성한다. parent/child 관계 불일치, allocation·root·generation·owner 오류와 unwind panic은 registry·root·문서·quota를 보존하고 회수를 미룬다. panic=abort는 복구 대상이 아니다.
10. **revision 경계:** 성공한 sweep은 unreachable node와 내부 자원 계수만 제거한다. HostRoot 트리와 렌더 입력을 바꾸지 않으므로 `DocumentRevision`과 `RenderTreeRevision`은 유지한다.

자세한 순서와 오류 결과는 [내부 계약 0021](../spec/internal/0021-s03-dom-node-lifecycle.md)에 둔다. 이 결정표는 구현 완료 선언이 아니다.

## 검증 기준

계약과 구현은 아래 경로를 Android·iOS의 실제 V8 실행과 Rust/Bun 자동화에서 검증한다.

### 의도적 보존과 누수 판별

- **현재 수명 probe:** Rust resident node 수와 `document_nodes` delta, JS WeakRef 상태, live wrapper에서 다시 읽은 parent/child 관계를 함께 기록한다. 이 값은 GC 확인용 시뮬레이터 fixture이며 전체 heap 또는 RSS 누수 계정이 아니다. 강한 Map 제거는 소스와 default/fixture compile guard로 확인한다. Android·iOS에서 16,385개 wrapper 생성·회수, 작은 6×32 반복 자원 fixture와 제한된 종료 경합은 통과했지만 문자열 예산 경계, 장기 반복과 heap snapshot은 남아 있다. 대량 scan 시간은 한 번씩 기록했으며 비용 곡선을 측정한 벤치마크는 아니다.
- **GC 수동 요청:** `LowMemoryNotification()`은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1`인 Android·iOS 검증 빌드에서만 연결한다. 기본 빌드의 JS 내부 API에는 강제 GC 요청 entry를 등록하지 않으며 제품 수거 시점을 제어하지 않는다. [고정 V8 weak handle 계약](https://chromium.googlesource.com/v8/v8.git/+/7b50b62cb18f28617959e8452e2cd18195b38bcf/include/v8-persistent-handle.h)
- **root 대조군:** 연결 노드는 wrapper 참조를 버려도 유지되어야 한다. 분리 노드는 JS 전역 변수나 native 강한 callback closure가 wrapper를 붙잡는 동안 유지되어야 하며, GC가 약한 handle을 reset한 뒤 safe point scan에서 회수 후보가 되어야 한다. 현재 진단용 `spinon.onEvent()` callback의 보존·호출·교체 후 회수는 Android/iOS fixture에서 확인했다. 일반 pending callback 완료·취소 양쪽과 DOM listener owner 그래프는 별도 검증하며 J12 계약에서 다룬다.
- **반복 기준:** warm-up 뒤 create·detach·drop·collect를 여러 차례 반복한다. 의도한 root를 모두 해제한 test fixture에서는 V8 weak `Global` reset과 safe point scan 뒤 Rust live node·`HostNodeHandle`/JS façade ID 매핑·string 계수, 살아 있는 wrapper 등록 수, native callback `Global` active handle 수가 시작 기준선과 정확히 같아야 한다. V8이 GC를 수행하지 않은 iteration은 누수 판정에 넣지 않고 보존 지연으로 기록한다. root 대조군은 각 자원이 계속 유지되는지 별도로 확인한다. V8 heap snapshot의 전체 object 수는 pass/fail 수치로 쓰지 않고 retaining path에서 예상 밖 JS `Map`·closure를 찾는 데 쓴다. heap snapshot은 Rust allocation이나 OS view 메모리의 전체 계정이 아니므로 플랫폼 native memory profiler와 구분한다. `used_heap_size`와 프로세스 RSS는 보조 지표이며 RSS 하나만으로 누수를 판정하지 않는다. GC가 객체를 회수해도 allocator page가 즉시 OS에 반환되지 않을 수 있다. [V8 HeapProfiler API](https://v8.github.io/api/head/classv8_1_1HeapProfiler.html)

- 연결된 노드의 wrapper만 사라져도 HostDocument 노드와 트리가 유지되고, 다음 조회에서 유효한 wrapper를 만들 수 있다.
- 연결된 Text wrapper를 WeakRef로 관찰해 실제 GC reset을 확인한 뒤 조회로 재생성하고, 재생성 wrapper의 반복 조회 identity를 확인한다.
- 앱 JavaScript가 `spinon.__internal`의 wrapper get/register/unregister 함수를 찾거나 호출할 수 없다. ID 예약 없이 실패하는 Symbol 변환을 300회 반복한 뒤에도 정상 노드 생성이 가능하다.
- 살아 있는 JS 참조가 있는 분리 노드는 조회·수정·재삽입 가능하다.
- 분리된 자손 wrapper가 살아 있는 경우 필요한 부모·형제 관계가 회수 때문에 깨지지 않는다.
- JS 참조가 없는 분리 서브트리는 합의한 안전 지점에서 회수되고, 재사용되지 않는 ID의 stale 접근은 실패 폐쇄한다.
- 같은 wrapper가 살아 있는 동안 handle 조회가 identity를 유지한다. 수거 이후 identity 보장은 계약에 적힌 범위를 따른다.
- 런타임 pending callback은 비동기 호출 대기 중 계속 호출 가능하고, 등록 해제·교체·취소·세션 종료 뒤에는 더 호출되지 않으며 strong root가 해제된다.
- 런타임 pending callback의 closure가 분리 노드 wrapper를 캡처하면 callback이 살아 있는 동안 노드 관계가 유지되고, callback을 해제한 뒤 다른 JS root가 없다면 subtree 회수 대상이 된다. DOM listener의 node-owned edge와 target 수명은 J12에서 별도 검증한다.
- 다른 Isolate의 callback ID, 중복 해제, 이미 종료된 세션의 완료 응답은 잘못된 접근으로 실패 폐쇄한다.
- 반복 create/remove/collect에서 노드·문자열·wrapper 자원 사용량과 회수 지연을 기록한다. 문자열·external root lease 등 별도 예산에 도달하는 경우 오류가 결정적이고, 동적 buffer 부족은 문서 상태를 바꾸지 않은 채 회수를 미뤄야 한다.
- Android·iOS probe는 각 실행의 `document_collection_error`, poisoned 상태와 scanned/live/empty handle 계수를 검사한다. sticky callback 오류, poisoned runtime, 일관되지 않은 계수는 JS eval이 성공해도 전체 probe 실패다.
- 세션 종료 중 자동 reset된 weak handle, pending host callback, 다른 세션 handle, Rust panic·할당 실패가 다른 노드나 세션을 손상하지 않는다.

S03.3 완료에는 Rust core/bridge 실패 경로 fixture, Rust·Bun 검증, C++ weak handle owner safe-point 연결, Android·iOS의 실제 V8 실행, 반복 resource/closure/shutdown 검증, 비용 특성화와 실패 주입이 필요하다. Rust HostDocument·FFI collector와 실제 V8 registry에서 각각 16,385개 입력을 확인했다. Android·iOS의 대량 실행은 16,394개 live wrapper scan, 결과 vector 재확장, 16,385개 sweep과 기준선 복귀를 통과했으며 한 번씩의 시간·payload 관측값을 [실행 근거](../spec/internal/evidence/s03-dynamic-wrapper-registry-2026-10-05.md)에 남겼다. allocation failure 주입, 더 큰 규모의 반복 가능한 scan 비용, 실기기, listener/external root는 남아 상위 S03.3과 J10은 미완료다.

## 구현 단계 체크리스트

- [x] 내부 계약 0021과 결정성 Rust reference fixture를 추가하고 고정 입력·예상 생존 집합·실패 결과를 검증한다.
- [x] Rust HostDocument 도달성 plan/commit과 bridge node/handle/string 회수 메서드를 구현하고 고정 fixture 및 실패·quota runtime test에서 확인한다. Bun fixture는 256 pending ID 상한을 모델링해 반복 Symbol 변환 실패와 후속 생성을 확인한다.
- [x] V8 C++ adapter에 callback 없는 weak wrapper registry, isolate owner 안전 지점의 전체 선형 scan과 Rust 회수 callback을 연결한다. registry API는 facade closure에 숨기며, sticky 오류·scan 통계·불변식 파손 시 runtime 차단을 둔다. 강제 full GC 진입은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 검증 빌드에만 포함한다.
- [x] Android·iOS 시뮬레이터 실제 V8에서 attached/detached live root, orphan auto-reset·sweep, 실제 wrapper reset 뒤 재생성, collector 오류·poison·scan 통계 gate를 확인하고 원본 log·screen을 evidence에 기록한다. [실행 근거](../spec/internal/evidence/s03-v8-weak-wrapper-2026-10-04.md)
- [x] Android·iOS 실제 V8에서 현재 native strong callback closure가 분리된 child wrapper를 보존하고, callback 교체 뒤 회수되는지 확인한다. [실행 근거](../spec/internal/evidence/s03-repeat-lifecycle-2026-10-05.md)
- [x] Android·iOS에서 element·text 32쌍을 만들고 분리·회수하는 시나리오를 6회 실행해 node·UTF-16 string unit·live weak wrapper baseline 복귀와 `scanned = live + empty`를 확인한다. [실행 근거](../spec/internal/evidence/s03-repeat-lifecycle-2026-10-05.md)
- [x] Rust HostDocument와 C ABI collector가 16,385개 노드·root를 수용하고, 작은 출력 buffer에 대한 무변경 응답 뒤 재시도 성공·전체 회수를 확인한다.
- [x] Android·iOS 실제 V8에서 16,384개 초과 wrapper 생성·scan·sweep과 C++ root/result vector 동적 확장, `WeakRef` 해제, 자원 기준선 복귀를 검증하고 단일 관측 시간·payload를 기록한다. [실행 근거](../spec/internal/evidence/s03-dynamic-wrapper-registry-2026-10-05.md)
- [ ] C++ 동적 buffer 할당 실패 주입, 더 큰 규모의 반복 가능한 scan 비용 곡선, V8 heap/RSS, 장기 반복과 실기기를 검증한다.
- [x] Android·iOS 시뮬레이터에서 활성 eval 취소, 대기·종료 뒤 호출 거부, 반복 close와 worker join을 검증한다. [실행 근거](../spec/internal/evidence/s03-shutdown-2026-10-05.md)
- [ ] DOM EventTarget listener와 cross-heap closure cycle은 J12 명세와 구현에서 별도로 해결한다.

## 범위 밖

`textContent` setter, `NodeList`·`HTMLCollection`, selector API, 전체 DOM/Web IDL 적합성, 여러 Owner의 동시 수정, DOM 이벤트 등록·제거·캡처·전파 semantics, CSS invalidation, layout·GPU 연결은 이 작업에 포함하지 않는다. callback persistent handle의 수명과 node collector 간 root 관계만 여기서 선행 검증하고, 전체 이벤트 계약은 J12에서 따로 정한다.
