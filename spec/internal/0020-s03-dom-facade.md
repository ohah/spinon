# 0020 · S03.2 제한 DOM façade

**인터페이스 버전:** `0.1.0` · **상태:** 내부 시제품 구현 · **공개 API 안정성:** 보장 안 함 · **상태 대장:** S03.2, J10 일부

## 목적과 비교 기준

S03.1의 `spinon.__internal.commitDocumentBatch`는 Rust `HostDocument` 변경 경계만 검증했다. 이 슬라이스는 V8 전역 `document`의 작은 DOM 유사 표면을 해당 문서에 연결한다. 공개 DOM 완성, 웹 API 전체 호환, CSS 표시, 프레임워크 renderer 완료를 뜻하지 않는다.

비교 모델은 WHATWG DOM Living Standard 및 Chromium `154.0.8037.95`다. 구현 전에 고정한 사례와 의도적 차이는 [구현 전 비교 기록](evidence/s03-dom-facade-precomparison-2026-10-04.md)에 둔다. 앱 루트 확장·지원 목록·예외·한도는 아래 계약을 따른다.

## 문서와 소유권

- `globalThis.document`는 V8 세션 하나의 `HostDocument`를 가리킨다. `nodeType`은 `9`, `nodeName`은 `#document`, `parentNode`는 `null`, `ownerDocument`와 `textContent`는 `null`이다.
- HostRoot는 브라우저의 `Document`, `documentElement`, `body`가 아니다. 이 앱 문서는 여러 최상위 Element를 허용하는 Spinon 확장이다. 최상위 Text는 거부한다.
- 문서 소유자 ID는 세션마다 하나다. 다른 framework Owner의 동시 쓰기, 여러 문서 간 노드 이동, `adoptNode()`는 지원하지 않는다.
- 각 getter는 V8 소유 실행기에서 커밋된 Rust `HostDocument`를 직접 동기 조회한다. 전체 `HostDocumentSnapshot`을 getter·변경마다 복제하지 않고, 별도 JS 트리 복사본을 조회 원본으로 두지 않는다. 스타일·레이아웃 소비자가 요구하는 불변 snapshot 생성은 별도 경로다.
- 생성 후 분리된 노드는 세션 종료까지 보존한다. JavaScript wrapper는 강한 `Map`에 유지된다. wrapper GC, 노드 폐기, 세션 중 자원 회수는 없다.
- 세션당 노드 최대 16,384개, 이름·텍스트·속성을 합친 보존 문자열 최대 16,777,216 UTF-16 코드 단위다. 제거 노드도 이 수에 포함한다. 텍스트나 속성 값을 교체하면 이전 값의 코드 단위는 한도에서 빠진다.
- 조회 문자열은 Rust callback 버퍼, C++ 임시 버퍼, V8 문자열로 복사될 수 있다. 최대 한도에서의 순간 메모리 사용량은 기기에서 측정하지 않았으며, 상한을 실제 앱 메모리 적합성으로 해석하지 않는다. C++ 출력 버퍼 할당 실패는 JS 오류로 바꾼다.

## 지원하는 표면

| 객체 | 제공 항목 | 규칙 |
| --- | --- | --- |
| 전역 | `document`, `Node`, `Element`, `Text`, `DOMException` | `document`는 변경 불가 전역 속성이다. `Node`·`Element`·`Text`는 전역 이름을 제공하지만 wrapper 무결성을 위해 직접 생성할 수 없다. 네 생성자 전역 속성은 writable·configurable이고 enumerable하지 않다. 제한된 `DOMException(message = "", name = "Error")` 생성자는 노출하며 constructor 이름과 `Symbol.toStringTag`는 `DOMException`이다. `name`·`message`는 인스턴스 own 속성이 아닌 enumerable prototype getter이며 setter가 없다. 레거시 `code`·상수는 미지원이다. |
| `Document` | `nodeType`, `nodeName`, `parentNode`, `ownerDocument`, `textContent`, `firstChild`, `hasChildNodes()`, `appendChild()`, `insertBefore()`, `removeChild()`, `createElement()`, `createTextNode()` | Element/Text 생성은 각각 Rust에 즉시 커밋한다. HTML 요소 이름은 ASCII 문법 subset을 검사하고 ASCII 소문자로 저장한다. |
| `Node` | `nodeType`, `nodeName`, `parentNode`, `parentElement`, `firstChild`, `nextSibling`, `textContent`, `nodeValue`, `ownerDocument`, `hasChildNodes()`, `contains()`, `isSameNode()`, `appendChild()`, `insertBefore()`, `removeChild()` | getter는 Rust HostDocument 조회 결과다. Element `nodeName`은 ASCII 대문자, Text는 `#text`다. Text의 `nodeValue`는 `data`와 같고 다른 노드는 `null`이다. Text가 자식이 없을 때 `firstChild`는 `null`, `hasChildNodes()`는 `false`다. receiver는 세션 wrapper여야 한다. `contains()`·`isSameNode()`의 명시적 nullish 인자는 `false`, 비-Node 객체는 `TypeError`다. 트리 무결성 검사는 재정의 가능한 JS getter나 인스턴스 `nodeType` 속성 대신 Rust 노드 종류·부모 조회를 사용한다. 메서드는 성공 시 변경을 동기 커밋한다. |
| `Element` | `localName`, `tagName`, `id`, `className`, `getAttribute()`, `hasAttribute()`, `setAttribute()`, `removeAttribute()` | HTML 요소만 생성한다. 속성 이름은 ASCII subset을 검사하고 ASCII 소문자로 정규화한다. `id`·`className`은 같은 HostDocument 속성에 반영한다. |
| `Text` | `data` | `Node.nodeValue`를 포함해 두 setter/getter가 같은 텍스트 데이터에 연결된다. Element·Document의 `nodeValue` setter는 DOM 동작대로 아무 변경도 하지 않는다. V8 UTF-16 코드 단위를 보존한다. |

지원하지 않는 속성·메서드는 전역에서 제공하지 않는다. 특히 `textContent` setter, `childNodes`, `children`, `previousSibling`, `lastChild`, `isConnected`, selector 조회, `createElementNS`, Comment/DocumentFragment, namespace 속성, `innerHTML`, 파서, `body`, `documentElement`, 이벤트 리스너·전파, 스타일·레이아웃·GPU 반영은 미구현이다. 요소 이름은 ASCII 정규식 subset만 허용한다. `Symbol`을 DOMString으로 받는 호출은 `TypeError`를 던진다.

## 변경과 조회

- `createElement()`·`createTextNode()`는 새 HostDocument 노드를 만들고 detached wrapper를 반환한다. 연결 키는 양수 정수이며 세션 안에서 재사용하지 않는다. 다음 키는 현재 성공한 키의 최대값 다음으로 정한다.
- append·insert·remove·Text 데이터·속성 변경은 호출마다 S03.1 변경 callback 하나를 동기 호출한다. 실패한 callback은 해당 메서드 변경을 공개하지 않는다. 메서드 뒤 같은 평가 안의 getter는 갱신된 Rust snapshot을 읽는다.
- `insertBefore(node, node)`는 callback을 부르지 않는 no-op이다. 두 번째 인자 `undefined`는 nullable Web IDL 인자에 맞춰 `null`처럼 끝 삽입으로 처리한다. 부모가 아닌 기준 노드를 쓰면 `NotFoundError`; 자기 자신 또는 자손 아래로 삽입하면 `HierarchyRequestError`; 지원하지 않는 부모·문서 루트의 Text 자식은 `HierarchyRequestError`다.
- 같은 HostDocument handle은 JS wrapper `Map`에서 한 객체로 반환한다. 제거는 부모 연결만 끊고 노드 ID·wrapper·자식 subtree를 보존한다. 다른 부모에 재삽입할 수 있다.
- HostDocument commit 오류는 호출 종류에 대응하는 제한 DOMException으로 감싼다. Rust 예상 밖 오류는 `InvalidStateError`; 속성·요소 이름 오류는 `InvalidCharacterError`; 세션 노드·보존 문자열 상한은 `QuotaExceededError`; JS 인자 형식 오류는 `TypeError`다. 작업 인덱스가 붙은 quota 오류도 FFI에서 `QuotaExceededError`로 보존한다. 오류 메시지와 DOMException의 레거시 `code` 호환은 보장하지 않는다.
- commit callback은 성공 `0`, 잘못된 FFI 인자 `-1`, 문서 변경 거부 `-2`, panic 복구 `-3`, quota 초과 `-4`를 반환한다. query callback은 성공 `0`, 출력 버퍼 부족 `1`(필요 코드 단위를 먼저 읽고 정확한 버퍼로 재호출), 잘못된 FFI 인자 `-1`, 조회 거부 `-2`, panic 복구 `-3`을 반환한다.
- `document` 앱 루트와 브라우저 `Document`의 구조 차이, 제한된 유효 이름 문법, 강한 wrapper cache와 회수 한도는 호환성 차이다.

## 검증과 미완료 경계

Bun fixture는 같은 façade 스크립트와 모의 HostDocument를 사용한다. Rust 단위 테스트는 query callback의 HTML `nodeName`, 부모·자식·형제 ID 매핑, Text의 빈 자식 결과, UTF-16 왕복, 속성 읽기, next ID, 잘못된 출력 포인터와 총량 한도를 확인한다. fixture는 필수 인자 누락과 문서 루트 Text의 원자적 거부, DOMException 기본·메시지 단독 생성·constructor 이름·Symbol.toStringTag·prototype getter·전역 descriptor·Symbol 거부·읽기 전용 동작과 legacy code 제외, nullable `insertBefore` 인자, 잘못된 receiver·비-Node 인자의 `TypeError`, Rust 문서 변경 전 TypeError 동작을 확인한다. `bun run test`, Clippy, rustfmt, Bun 번들 및 Android API 36 ARM64 에뮬레이터와 iPhone 17 Pro / iOS 26.2 시뮬레이터의 실제 V8 앱 빌드·실행이 통과했다. OS·도구 버전 및 원본 로그는 [실행 근거](evidence/s03-dom-facade-runtime-2026-10-04.md)에 둔다.

이 구현으로 J10 전체, R03 전체 또는 S03 전체를 완료 처리하지 않는다. 미완료: 공개 오류·DOMException 세부 일치와 레거시 `code`, 전체 Web IDL 변환과 Unicode 이름, mutable `textContent`, NodeList/live collection, wrapper GC·node dispose, 다중 Owner 동시 변경, framework renderer 통합, CSS invalidation·layout·GPU 적용, 웹 대체 구현, 실기기 성능과 최대 quota의 기기 메모리.

구현 범위와 근거가 준비되기 전에는 [공식 상태 대장](../STATUS.md)의 J10·S03 전체 완료 체크를 바꾸지 않는다.
