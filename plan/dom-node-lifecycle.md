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

## 선택한 회수 방향 · 미구현

S03.3의 기본 방향은 **Rust가 노드 저장소와 회수를 소유하고, V8은 JS wrapper의 도달 불가를 알리는 모델**로 정한다. 전체 Rust 문서를 V8 heap에 넣거나 V8과 Rust를 하나의 unified heap으로 합치지 않는다. 별도 Rust tracing-GC 라이브러리도 전제하지 않는다.

이 방향은 `HostDocument`를 단일 기준 트리로 유지하면서 JS 참조의 생존 정보만 V8 경계에서 받는다. Rust의 노드 관계를 C++ GC 객체 그래프로 복제하지 않는 대신, Rust 도달성 sweep과 wrapper 통지·종료 규약을 직접 구현해야 한다.

- Rust `HostDocument`는 노드·속성·텍스트·관계의 기준 소유자다. 노드 저장소가 값을 보유한다는 이유만으로 그 노드를 생존 root로 간주하지 않고, 정한 root에서 Rust 노드 관계를 추적해 회수 여부를 결정한다.
- V8은 JS wrapper와 그 일반 JS 참조를 관리한다. V8 어댑터는 wrapper별 약한 persistent handle 또는 동등한 엔진 내부 추적을 두고, wrapper가 수거되면 해당 wrapper 등록을 식별하는 고유 token을 isolate 소유 queue에 남긴다. 같은 노드 wrapper가 재생성될 수 있으므로 node ID만으로 생존 등록을 제거하지 않는다. GC callback에서 Rust 문서를 직접 수정하지 않는다.
- Rust는 HostRoot, 아직 살아 있는 wrapper가 가리키는 노드, 명시적으로 등록된 외부 host root에서 도달성을 표시한다. 런타임 진단 handler와 pending 작업 callback 등 strong `v8::Global<Function>`은 reset될 때까지 V8 root이며, closure가 참조하는 wrapper도 살아 있는 것으로 본다. DOM listener는 의미상 EventTarget 소유 edge이지 Rust의 독립 생존 root가 아니다. 다만 listener를 strong `v8::Global`로 보관하면 V8 root가 되고 closure가 target wrapper를 다시 참조하는 순환을 만들 수 있다. 이 순환을 끊을 수명 기법은 J12에서 확정하기 전까지 미해결이며, 단순히 listener를 node-owned라고 선언하는 것만으로 해결됐다고 보지 않는다. 살아 있는 wrapper가 속한 분리 DOM 연결 성분은 부모·자식·형제 관찰 의미가 보존되도록 회수 대상에서 제외한다.
- Rust는 callback이 끝난 안전 지점에서 queue를 반영하고, 어떤 root에서도 도달할 수 없는 분리 노드만 저장소·handle·문자열 계수에서 회수한다. GC 통지가 늦으면 일시적으로 더 오래 보유하는 것은 허용하되 살아 있는 JS 참조를 먼저 회수하는 것은 허용하지 않는다.
- 현 JS façade의 강한 `wrappers` Map은 이 목표 구조와 맞지 않으므로 구현 시 약한 wrapper 추적으로 대체해야 한다. 살아 있는 wrapper의 identity는 유지해야 하지만, weak handle 저장소와 stale ID 항목 정리 방식은 아직 결정하지 않았다.

이 선택은 **노드 소유권과 회수 책임의 큰 방향**을 정한 것이다. weak handle API 세부, 정확한 root·분리 연결 성분 정의, callback/listener 소유 관계, quota와 shutdown 규칙은 아직 미결정이다. 구현·회귀 검증 전까지 S03.2의 세션 수명 보존 동작과 `0.1.0` 계약은 그대로다.

## Chromium 참고 모델

Chromium의 Blink는 DOM C++ 객체를 Oilpan 관리 heap에 두고 `Member<T>` 등의 추적 참조와 클래스별 `Trace()`로 native 객체의 도달성을 기록한다. V8의 JavaScript heap과 Blink/Oilpan heap은 unified heap 수거에서 서로 건너가는 참조를 함께 추적한다. 그래서 DOM에서 노드를 떼는 동작과 메모리 회수는 별개이며, JS wrapper나 native 객체 그래프가 노드를 계속 도달 가능하게 만들면 살아 있고, 양쪽 heap의 strong root에서 끊긴 뒤 GC가 회수한다. Blink GC는 marking·sweeping을 사용하고 일반적으로 이벤트 루프에서 수거 작업을 예약한다. [Oilpan API와 동작](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/heap/BlinkGCAPIReference.md) · [unified heap과 wrapper tracing 상태](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/TraceWrapperReference.md)

Blink의 `ScriptWrappable`은 C++ DOM 구현과 V8 wrapper를 연결한다. wrapper는 객체 안에 저장하거나, 객체가 살아 있는 동안 wrapper를 보존하는 ephemeron map에 기록한다. 따라서 Chromium 구현을 단일 전역 강한 `Map`이나 wrapper를 무조건 약하게 둔 캐시 하나로 축약하면 안 된다. 정확한 wrapper 연계는 [DOMDataStore](https://chromium.googlesource.com/chromium/src/+/lkgr/third_party/blink/renderer/platform/bindings/dom_data_store.h)와 [ScriptWrappable](https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/platform/bindings/script_wrappable.h) 구현을 기준으로 한다.

Blink는 JS와 native 객체 그래프를 unified heap에서 함께 추적하는 비교 모델이다. 스피논은 이를 그대로 택하지 않고 Rust 소유 문서와 Rust 회수기를 유지한다. V8은 wrapper 수거 알림을 제공하고 Rust가 자체 root 그래프를 추적한다. `FinalizationRegistry`의 비결정적 실행을 필수 정리나 정확한 quota 회수 시점으로 사용하지 않는다.

Rust 표준 라이브러리는 소유권·`Drop`으로 값을 해제하며, 기본 tracing GC를 제공하지 않는다. `Rc`/`Arc`는 참조 횟수 기반 공유 소유권이고 `Weak`는 그 소유권을 늘리지 않지만, 강한 참조 순환은 자동으로 수거하지 못한다. [Rust 소유권과 Drop](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) · [참조 순환과 Weak](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Rust 표준 라이브러리는 기본 tracing GC를 제공하지 않으므로, 선택한 방향에서는 Rust 소유권과 안전 지점의 도달성 sweep으로 노드 저장소를 회수한다. V8 callback은 불투명 handle/token만 isolate 소유 queue에 전달하고, Rust 쪽 적용은 callback·JS 실행과 경합하지 않는 safe point에서 수행한다. 별도 Rust GC, refcount 기반 노드 소유, V8 unified heap은 이 단계의 선택 모델이 아니다. 약한 통지 누락·지연 시에는 안전을 우선해 노드를 보수적으로 더 오래 보유한다.

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
| Rust 소유 mark-and-sweep + V8 약한 wrapper 통지 | V8 wrapper 생존 통지를 받아 Rust가 HostRoot와 명시된 참조 root에서 분리 노드 도달성을 추적·회수 | **선택한 설계 방향.** 구현 API와 상세 root 규칙은 미결정이다. |

## 선택한 생존 규칙 · 세부 계약 미결정

- HostRoot에 연결된 노드는 문서 트리가 살린다. JavaScript에서 wrapper가 계속 참조되는 노드는 그 wrapper가 살린다.
- 런타임 진단 handler와 pending 작업 callback처럼 strong persistent handle로 보유한 함수는 reset될 때까지 JS root다. closure가 wrapper를 캡처하면 해당 wrapper도 살아 있다. 현재 진단 `spinon.onEvent`는 런타임 전역 callback root이며 DOM node와 직접 연결되지는 않지만, handler closure가 앱 변수의 wrapper를 캡처하면 그 wrapper를 살릴 수 있다. DOM listener는 의미상 EventTarget 소유 edge지만 strong `Global<Function>` 구현은 JS heap에 독립 root를 만든다. target·listener·closure 사이 순환을 회수할 방법은 J12에서 결정해야 한다.
- JS가 참조하는 노드가 분리 서브트리 안에 있으면, `parentNode`·`firstChild`·`nextSibling` 관찰 결과를 보존하도록 해당 분리 서브트리 전체를 유지한다. 어떤 wrapper도 참조하지 않는 분리 서브트리는 회수 후보가 된다.
- 목표 구조에서는 wrapper를 강한 전역 `Map`에 넣지 않는다. wrapper가 JS에서 살아 있는 동안 같은 ID의 조회가 같은 객체를 반환해야 한다. 숫자 node ID를 JS `WeakMap` 키로 사용할 수는 없으므로, V8 약한 handle 저장소 또는 `Map<id, WeakRef<wrapper>>`의 stale ID 정리 방식을 정해야 한다. 필수 회수 시점은 `FinalizationRegistry`에 맡기지 않는다.
- 첫 회수 구현에서는 노드 ID를 세션 안에서 재사용하지 않는다. 이는 선택한 안전 기본값이다. 나중에 재사용을 지원하려면 generation을 포함한 handle 계약을 먼저 추가한다.
- 한도는 누적 생성 횟수보다 살아 있는 노드·문자열의 resident 사용량을 기준으로 하는 안을 우선한다. 한도 직전 GC·회수 통지 drain 한 번을 요청하고도 초과하면 `QuotaExceededError`를 반환하는 방안을 비교한다. 강제 GC 지연과 실패의 결정성을 측정한 뒤 최종 선택한다.
- GC 중 callback은 HostDocument를 직접 변경하지 않고 wrapper 등록 token만 남긴다. GC가 끝난 뒤 isolate 소유 실행 경로에서 token을 적용한다. queue가 가득 차거나 token이 중복·stale 상태여도 살아 있는 노드를 잘못 회수하지 않아야 한다. 세션 종료는 callback 등록을 먼저 닫고 pending token을 drain하거나 안전하게 무효화한 다음 Rust 문서를 파기한다.

선택한 Rust 소유 회수 모델은 `0020`의 현재 시제품 계약을 바꾸지 않는다. 위 세부 계약은 구현 전에 명세와 fixture로 고정한다.

### Rust `HostDocument` 회수 흐름 · 선택 방향, 미구현

1. Rust가 노드를 만들고 불투명 handle을 반환한다. JS wrapper는 이 handle로 조회·변경 요청을 보낸다.
2. 연결된 노드는 HostRoot의 트리 관계가 살린다. 앱 변수나 pending callback closure가 wrapper를 참조하면 V8의 일반 도달성 그래프가 wrapper를 살린다.
3. `removeChild()`는 부모 연결만 끊는다. 노드 저장소는 물리적으로 값을 보유하지만 저장소 보유 자체는 생존 root가 아니다. 안전 지점에서 정한 root 그래프가 도달하지 않는 분리 노드만 sweep한다.
4. wrapper를 약하게 추적하는 handle은 V8 GC가 wrapper를 도달 불가능하다고 판단하면 정리 통지를 낸다. 통지는 Rust 트리를 즉시 수정하지 않고 고유 wrapper registration token을 isolate 소유 queue에 보낸다. JS 참조나 wrapper를 캡처한 strong `Global<Function>`이 남아 있으면 통지는 아직 오지 않으므로 노드는 보수적으로 유지된다. EventTarget listener의 strong callback cycle은 이 모델만으로 회수되지 않아 J12 해결 전까지 지원 완료로 간주하지 않는다.
5. 정한 안전 지점에서 Rust가 HostRoot, 아직 살아 있는 wrapper handle, 명시적으로 등록된 외부 host root부터 트리를 추적한다. 제거된 subtree가 이 root들에서 도달 불가능할 때만 노드·문자열·handle mapping을 회수하고 quota를 갱신한다. 회수 통지 전까지는 메모리와 quota 회수가 늦어질 수 있다.
6. DOM listener는 런타임 전역 pending callback과 다르다. 의미상 listener와 target은 EventTarget이 소유하는 참조 관계다. strong `v8::Global<Function>`이 callback closure를 통해 wrapper를 살리면 target 수거를 막는 순환이 생길 수 있으므로, 그 cross-heap edge의 추적·해제 및 순환 회수 계약은 J12에서 결정한다.

이 흐름은 현재의 `HostDocument` BTreeMap과 `wrappers` 강한 Map에는 구현되어 있지 않다. 약한 wrapper 추적, V8 수거 알림, Rust 안전 지점 sweep, callback owner 간 계약이 구현되고 검증되기 전에는 회수됐다고 간주하지 않는다.

## 구현 전에 확정할 결정

1. **살아 있는 노드의 정의:** 선택 방향의 detached 연결 성분 보존이 최소 Node 관계 API와 맞는지 확인하고, HostRoot·wrapper root·외부 host root에서 정확히 도달성 추적할 범위를 정한다.
2. **wrapper identity:** wrapper가 JS에서 살아 있는 동안 같은 HostDocument handle 조회가 같은 객체를 반환하는지, wrapper가 수거된 뒤 재생성할 수 있는지 확정한다.
3. **GC와 Rust 변경 경계:** GC callback은 token만 isolate 소유 queue에 남기고, 정한 안전 지점에서 적용한다. queue의 재진입·중복·overflow·세션 종료와 경쟁할 때 token 폐기 순서를 정한다. queue가 overflow하면 해당 wrapper를 살아 있는 것으로 보수 처리해 조기 회수를 막는다.
4. **handle 유효성:** 선택한 세션 내 ID 비재사용과 stale native handle의 실패 결과를 명세하고, ID 재사용은 첫 회수 구현에서 지원하지 않는다. 이후 재사용하려면 generation 검증을 포함한다.
5. **quota 의미:** 노드 수와 보존 문자열 수를 live resident 자원량으로 세는 안, 한도 직전 회수 시도, 실패 오류를 확정한다. GC 시점에 따른 성공·실패 차이와 지연 비용을 측정한다.
6. **결정적 검증:** 테스트 전용 GC 요청·회수 queue drain 제어를 둘지 정한다. 제품 공개 API로 노출하지 않는다.
7. **함수 callback의 종료:** 런타임 진단·pending 작업 callback과 EventTarget 소유 listener를 구분한다. callback ID별 handle의 owner, 외부 생존 root 또는 node-owned edge 의미, 등록 해제·호출 완료·취소·target 회수·세션 종료 때 `Reset()`/trace 갱신 순서를 정한다. strong listener `Global`이 closure를 통해 wrapper를 살리는 순환을 어떻게 회수할지도 명세한다. V8 handle의 접근·reset은 해당 Isolate의 실행 owner를 따른다.
8. **wrapper 등록 token과 알림 queue:** 같은 HostNodeHandle에 wrapper가 재생성되어도 앞 wrapper의 늦은·중복 알림이 새 wrapper root를 지우지 않도록 token을 고유하게 식별한다. isolate별 queue 용량, GC callback에서 허용되는 작업, 중복·overflow·shutdown 정리와 계측 방식을 정한다.
9. **회수 실패 원자성:** Rust mark 단계의 할당 실패나 panic, handle 불일치, Isolate owner 불일치가 일부 노드만 제거한 상태를 만들지 않도록 회수 계획·적용의 경계와 오류 진단을 정한다. 실패하면 안전한 보존을 우선한다.

## 검증 기준

계약과 구현은 아래 경로를 Android·iOS의 실제 V8 실행과 Rust/Bun 자동화에서 검증한다.

### 의도적 보존과 누수 판별

- **현재 시제품 기준선:** 노드를 반복 생성하고 `removeChild()`한 뒤 Rust `HostDocumentBridge`의 live handle/node 수와 보존 문자열 사용량을 기록한다. 지금 계약에서는 제거 노드가 계속 남는 것이 예상 결과다. `wrappers.size` 계측 또는 V8 heap snapshot의 JS retaining path로 강한 Map 보존을 따로 증명한다. 현재 진단 `event_handler`를 포함한 native `Global<Function>`의 non-empty handle 수와 소유 callback ID도 별도 계측한다. 이는 현 정책의 관찰이지, GC 회수가 된다는 검증이 아니다.
- **회수 후보 검증:** 테스트 전용 설정에서 V8 full GC를 요청하고 weak-handle 통지 queue를 안전 지점에서 drain한 뒤 Rust live node/handle/string 수, V8 wrapper 객체 수, native `Global<Function>` active handle 수와 callback owner별 등록 수를 함께 기록한다. 테스트는 JS 지역 참조를 scope 밖으로 내보낸 다음 별도 GC turn에서 수거를 요청해 stack root가 결과를 왜곡하지 않게 한다. native handle counter는 callback의 명시적 해제 여부를 추적하고, heap snapshot retaining path는 JS closure와 객체를 누가 붙잡는지 보완한다. V8의 `RequestGarbageCollectionForTesting()`은 `--expose_gc`가 필요한 test-only API이며 제품의 수거 시점 제어로 쓰지 않는다. [V8 Isolate API](https://v8.github.io/api/head/classv8_1_1Isolate.html)
- **root 대조군:** 연결 노드는 wrapper 참조를 버려도 유지되어야 한다. 분리 노드는 JS 전역 변수나 대기 중 callback closure가 wrapper를 붙잡는 동안 유지되어야 하며, root를 놓고 GC·queue drain 뒤에는 회수 후보가 되어야 한다. pending callback 완료·취소의 양쪽을 확인하고, DOM listener owner 그래프는 J12 계약에 따라 별도 대조한다.
- **반복 기준:** warm-up 뒤 create·detach·drop·collect를 여러 차례 반복해 Rust live resource count, callback `Global` active handle count와 V8 snapshot object count가 안정 기준선으로 돌아오는지 확인한다. snapshot의 retaining path는 예상 밖 JS `Map`·closure를 찾는 데 쓰고, native callback owner counter는 persistent handle의 해제 여부를 별도로 확인한다. heap snapshot은 Rust allocation이나 OS view 메모리의 전체 계정이 아니므로 플랫폼 native memory profiler와 구분한다. `used_heap_size`와 프로세스 RSS는 보조 지표이며 RSS 하나만으로 누수를 판정하지 않는다. GC가 객체를 회수해도 allocator page가 즉시 OS에 반환되지 않을 수 있다. [V8 HeapProfiler API](https://v8.github.io/api/head/classv8_1_1HeapProfiler.html)

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
