# S03.2 · 제한 DOM façade 구현 전 비교 기준

**작성일:** 2026-10-04 · **기준:** WHATWG DOM Living Standard, 2026-09-24 · **대상:** Android·iOS V8 시제품 · **판정:** 구현 전 기준

## 목적과 경계

이 문서는 공개 DOM API 지원 판정이 아니라, 내부 façade 시제품의 동작을 구현 전에 고정한다. 표준 동작 oracle은 고정 Chromium 154.0.8037.95에서 같은 JavaScript 사례를 실행한 값과 WHATWG DOM 알고리즘이다. 화면 픽셀·CSS 계산·레이아웃·이벤트 전파·React 호환성은 이 비교 범위가 아니다.

모바일 `document`는 앱 `HostDocument`의 루트 façade다. 표준 브라우저 `Document` 전체를 구현하지 않는다. HostRoot 아래 다중 최상위 Element는 앱 루트 확장으로 허용하고, 최상위 Text는 거부한다. `body`, `documentElement`, `window`, HTML 파싱은 제공하지 않는다. 이 루트 차이는 호환 차이로 기록하며 브라우저 동등성을 주장하지 않는다.

## 고정 입력과 관찰값

시나리오 입력은 한 번의 JavaScript 평가 안에서 DOM API를 연속 호출하고 각 변경 직후 값을 읽는다. 독립 기준은 Chromium에서 분리된 `Document` 및 `DocumentFragment`를 사용해 루트 제약과 Element/Text 동작을 분리한다. 관찰값은 노드 종류·이름, 동일 객체 여부, 부모·자식·형제 관계, UTF-16 텍스트 코드 단위, 속성 값, 예외 이름, 이전 트리 보존이다.

| 사례 | 입력 | 기대 결과 |
| --- | --- | --- |
| HTML Element 생성 | `document.createElement("DiV")` | HTML namespace, `localName === "div"`, Element `nodeType === 1`, HTML `nodeName === "DIV"` |
| Text 생성 | `createTextNode("한글🌐")` | Text `nodeType === 3`, `nodeName === "#text"`, 데이터 코드 단위 보존 |
| Text의 자식 조회 | 새 Text에서 `firstChild`, `hasChildNodes()` 조회 | `null`, `false`; Element에만 자식 조회를 허용하는 Rust 표현 차이가 DOM 결과로 새어 나오지 않아야 함 |
| `nodeValue` | Document·Element·Text에서 읽고 비-Text에 설정 | Document·Element는 `null`, Text는 `data`; 비-Text 설정은 문서 상태를 바꾸지 않음 |
| 필수 인자 | `createElement()`, `createTextNode()`, `setAttribute(name)` 등 | DOM 메서드의 필수 인자 누락은 `TypeError`, 변경 전 문서 revision 유지 |
| nullable Node 인자 | `parent.insertBefore(child, undefined)` | Web IDL nullable Node 변환이 `undefined`를 `null`로 처리하므로 끝 삽입 |
| `DOMException` 생성자 | `new DOMException()`, `new DOMException("메시지")`, `new DOMException("메시지", "NotFoundError")` | 기본값 `message === ""`, `name === "Error"`; 이름·메시지는 읽기 전용. 제한 구현은 `code`와 상수를 제공하지 않음 |
| 잘못된 receiver·인자 | 비-Node receiver, `contains({})` | Web IDL interface receiver·인자 검증에 따라 `TypeError`, 트리 변경 없음 |
| 공개 속성 재정의 | Text wrapper의 own `nodeType`, `Node.prototype.parentNode` getter 재정의 | façade의 루트 종류·순환·포함 검증은 내부 Rust 노드 종류·부모를 사용해 재정의가 무결성을 바꾸지 않음 |
| 앱 루트 Text 차이 | Spinon `document.appendChild(text)` | 앱 HostRoot 제약: `HierarchyRequestError`, 분리 상태와 revision 보존. 브라우저 Document 비교 사례와 구분하는 의도적 확장 |
| 혼합 자식 순서 | Element 아래 Text·Element·Text를 삽입 | `firstChild`, `nextSibling`, `textContent`가 삽입 순서와 같음 |
| 동기 읽기 | 삽입·이동·삭제 직후 같은 평가에서 관계 조회 | 성공한 변경이 즉시 보이고 제거 노드는 분리 상태로 조회됨 |
| 재삽입·정체성 | 같은 노드를 다른 부모로 이동하고 다시 조회 | 같은 JS 객체와 HostNodeHandle, 새 부모와 순서가 일치 |
| no-op 삽입 | `insertBefore(node, node)` | 순서와 revision이 유지됨 |
| 속성 | `setAttribute("CLASS", "a")`, `id`, `className` 조회·수정·제거 | HTML 속성명 정규화, 메서드와 편의 속성의 같은 저장값 관찰 |
| 텍스트 읽기·쓰기 | Element `textContent`와 Text `data`를 읽고 Text 데이터를 변경 | 트리 자식 순서와 변경 직후 텍스트가 일치 |
| 실패 보존 | 잘못된 부모 자식 제거·순환 삽입·잘못된 이름 | 각각 표준 오류 이름, 트리와 두 revision 무변경 |
| 호출 범위 분리 | DOM 변경 성공 뒤 이후 별도 코드가 예외 발생 | 이미 성공한 메서드의 변경은 유지 |

## 허용 차이와 중단 조건

- 모바일 DOM root는 HostRoot를 감싸는 앱 문맥이어서 최상위 Element를 여러 개 허용한다. 표준 `Document`의 child 제한과 다르며, `documentElement`가 없다는 사실과 함께 공개 문서에 표시한다.
- `createElement`·속성 이름 검사는 WHATWG의 유효 이름 규칙 중 구현한 문자 범위만 판정한다. UTF-16 lone surrogate, NUL, 공백, `/`, `>` 등 거부 경계는 고정 테스트한다. Unicode 이름을 받지 못하는 경우 웹 차이로 보고한다.
- V8 시제품은 세션당 최대 16,384개 생성 노드와 보존 중인 문자열 총 16,777,216 UTF-16 코드 단위를 허용한다. 제거 노드는 런타임 종료까지 회수하지 않으므로 노드 수는 연결 여부와 무관하게 누적된다. 속성·Text 문자열 교체는 이전 값을 한도 산정에서 회수한다.
- Text·Element·Document처럼 DOM API상 자식이 없는 노드의 `firstChild`와 `hasChildNodes()`는 각각 `null`, `false`를 돌려준다. Web IDL에서 필수인 메서드 인자가 빠지면 Rust 변경 callback 전에 `TypeError`를 던진다.
- 문자열 인수는 Web IDL `DOMString`의 string-hint 변환 결과를 사용하고 `Symbol`은 `TypeError`로 거부한다. host commit 뒤 논리 조회는 동기 완료하지만 스타일·레이아웃·GPU 반영은 보장하지 않는다.
- C ABI 오류나 revision 불일치가 발생하면 해당 DOM 메서드 전체를 실패 처리한다. 저장 한도를 넘으면 `QuotaExceededError`, 정상 API 입력의 Rust 거부는 fixture 오류로 판정한다.
- `textContent` setter, `childNodes`·`children`, selector 조회, `createElementNS`, `innerHTML`, namespace 속성, Document 파싱·`body`·`documentElement`는 이번 façade에 없다. `createElement`와 속성 이름은 ASCII 문자 범위만 받는다. 이는 Chrome 비교 성공 사례가 아니라 미구현 경계다.

## 판정

필수 통과는 위 표의 노드·순서·정체성·값·예외·실패 보존이 Chromium 기대값과 일치하는 것이다. 앱 루트 다중 자식 및 문서 전역 범위는 명시된 의도적 차이로 별도 분류한다. 스타일 변경 후 픽셀 갱신, wrapper GC, 장기 실행 자원 회수, 프레임워크 Owner와의 동시 사용은 이 게이트에서 통과로 판정하지 않는다.

기준 참고: [WHATWG DOM Standard](https://dom.spec.whatwg.org/), [Web IDL DOMString](https://webidl.spec.whatwg.org/#idl-DOMString).

## Chromium 기준 확인

Google Chrome `154.0.8037.95` macOS headless에서 위 경계 사례의 DOM 표준 동작을 확인했다. Text `firstChild`는 `null`, `hasChildNodes()`는 `false`, `nodeValue`는 텍스트 데이터였다. Document·Element `nodeValue`는 `null`이고 값 설정은 무시됐다. `createElement()`, `createTextNode()`, `insertBefore(node)`, `setAttribute(name)`의 누락 인자는 모두 `TypeError`였다. 이 확인은 DOM API 비교이며 Spinon 모바일 실행이나 CSS·GPU 렌더링의 근거가 아니다.

신규 nullable 인자와 생성자 기본값의 기대 결과는 [Web IDL nullable 변환](https://webidl.spec.whatwg.org/#es-nullable) 및 [DOMException 인터페이스 정의](https://webidl.spec.whatwg.org/#idl-DOMException)에 맞춰 fixture로 확인한다. 앱 HostRoot의 최상위 Text 거부는 표준 브라우저 Document와 같다고 주장하지 않는다. Chrome headless를 재실행한 이번 검증에서는 프로세스가 제한 시간 안에 종료되지 않아 이 신규 행의 별도 Chromium 실행 결과로 기록하지 않는다.
