# 0003 · HTML·CSS·JavaScript API 표면

**상태:** 제안 · **명세 버전:** `0.1.0-draft`

## 공개 작성 문법

앱은 HTML 이름의 요소를 사용한다. React에서는 JSX/TSX의 `className`·`onClick`, Vue와 Svelte에서는 각자의 템플릿·이벤트 문법을 사용한다. 웹 빌드는 브라우저 DOM에 연결하고 모바일 빌드는 스피논 노드·GPU 장면에 연결한다. `<view>`를 기본 공개 요소로 도입하지 않는다.

| 요소 | 초기 검토 후보(결정 전) | 모바일 의미 | 아직 결정할 것 |
| --- | --- | --- | --- |
| `<div>` | 수직 구현 후보 | 일반 배치 상자 | 내장 기본 규칙은 초안이며 Chromium 기준값과 실제 적용은 C01·C04에서 검증 |
| 텍스트 노드 | 수직 구현 후보 | 내용·노드 ID·글리프 출력 | 인라인 조각·줄바꿈·선택 |
| `<button>` | 수직 구현 후보 | GPU 화면, 클릭, 버튼 접근성 역할 | 포커스·키보드·`disabled`·기본 동작·기본 외형 |
| `<span>`, `<input>`, `<img>`, `<p>`, `<a>`, `<ul>`, `<li>` | 실사용 UI 후보 | 각각 인라인, 입력, 이미지, 문단, 링크, 목록 의미 | 태그별 속성·이벤트·접근성 세부 계약과 기본 외형 |

위 후보는 R01의 기술 검토 항목이며, 첫 공식 릴리스 포함 결정이나 현재 지원 선언이 아니다. [적합성 상태 정의](0001-conformance.md#지원-표기)에 따라 목표 범위와 현재 적합성은 분리한다. 현재 [공개 지원 완료 API 목록](api/index.md)은 0개다. 아래 표의 매핑은 목표 구조 제안이고, 요소별 첫 구현 포함 여부와 적합성 상태는 아직 `미정`이다.

## HTML 요소 인벤토리와 매핑 후보

### 기준과 판정 의미

인벤토리 기준은 [WHATWG HTML Living Standard 요소 인덱스](https://html.spec.whatwg.org/multipage/indices.html#elements-3)다. 이 인덱스는 비규범 목록이므로 요소 이름의 기준으로만 쓴다. 표준 동작은 HTML 표준 본문과 아래 연결된 각 계약을 따른다. 2026-10-09 기준 스냅샷에서 HTML namespace 요소 113개를 분류했다. 인덱스의 SVG `svg`, MathML `math`, autonomous custom elements는 이 수에서 제외했다. `Text`는 요소가 아닌 노드 종류라 따로 다룬다.

| 매핑 후보 | 요소 이름 | 목표 매핑의 뜻 | 현재 적합성 |
| --- | --- | --- | --- |
| 논리 요소 1:1 후보 | `abbr`, `address`, `article`, `aside`, `b`, `bdi`, `bdo`, `blockquote`, `br`, `cite`, `code`, `data`, `dd`, `del`, `dfn`, `div`, `dl`, `dt`, `em`, `figcaption`, `figure`, `footer`, `h1`, `h2`, `h3`, `h4`, `h5`, `h6`, `header`, `hgroup`, `hr`, `i`, `ins`, `kbd`, `main`, `mark`, `nav`, `p`, `pre`, `q`, `rp`, `rt`, `ruby`, `s`, `samp`, `search`, `section`, `small`, `span`, `strong`, `sub`, `sup`, `time`, `u`, `var`, `wbr` | 태그 이름·namespace·논리 노드 identity를 보존할 후보. DOM 유사 논리 요소 하나를 뜻할 뿐, 네이티브 뷰 하나·GPU 도형 하나·CSS/접근성/브라우저 동작의 1:1 대응을 뜻하지 않는다. | `미정` |
| 조건부·전용 구현 | `a`, `area`, `audio`, `button`, `canvas`, `caption`, `col`, `colgroup`, `datalist`, `details`, `dialog`, `fieldset`, `form`, `img`, `input`, `label`, `legend`, `li`, `map`, `menu`, `meter`, `optgroup`, `option`, `output`, `picture`, `progress`, `select`, `selectedcontent`, `slot`, `source`, `summary`, `table`, `tbody`, `td`, `textarea`, `tfoot`, `th`, `thead`, `tr`, `track`, `ul`, `ol`, `video` | 논리 태그를 보존하더라도 목록·표 레이아웃, 폼 상태·입력, 미디어·이미지 자원, 내부 hit-test, 라우팅 또는 Web Components 계약 같은 전용 동작을 별도로 정하고 검증해야 한다. | `미정` |
| 문서·빌드·DOM 전용 | `base`, `body`, `head`, `html`, `link`, `meta`, `noscript`, `script`, `style`, `template`, `title` | 모바일 GPU 화면 노드로 직접 그리지 않는다. 문서 루트 설정, 정적 번들 입력, DOM fragment 등 해당 단계의 계약으로 처리할 후보이며, 동적 DOM·CSSOM·모듈 로딩이 제공된다는 뜻은 아니다. | `미정` |
| 모바일 GPU 장면에 직접 대응 없음 | `embed`, `iframe`, `object` | 일반 GPU UI 트리에 자체 대응이 없는 후보. 명시적으로 포함하는 별도 WebView 또는 미디어/플러그인 대체 기능이 있더라도 이 매핑을 자동으로 바꾸지 않는다. | `미정` |

목록은 113개 HTML 요소 이름을 중복·누락 없이 한 번씩 포함한다. 모든 행의 현재 적합성 `미정`은 [적합성 상태 정의](0001-conformance.md#지원-표기)를 따른다. 첫 목표 범위가 아직 결정되지 않았다는 뜻이며 `미지원` 판정이 아니다. WHATWG 인덱스의 foreign-content 항목인 SVG `svg`와 MathML `math`는 각 namespace 규격·렌더링 계약을 별도로 정하기 전까지 이 표의 HTML 요소 지원으로 추론하지 않는다. autonomous custom element 이름도 일반 태그 보존과 구분한다. `CustomElementRegistry`, 생성·수명주기 callback, Shadow DOM을 제공한다는 계약은 아직 없다. `<slot>`의 존재 역시 이 기능들을 뜻하지 않는다.

폐기된 HTML 요소는 [WHATWG 폐기 기능 목록](https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features)에 별도 분류한다. `applet`, `acronym`, `bgsound`, `dir`, `frame`, `frameset`, `noframes`, `isindex`, `keygen`, `listing`, `menuitem`, `nextid`, `noembed`, `param`, `plaintext`, `rb`, `rtc`, `strike`, `xmp`, `basefont`, `big`, `blink`, `center`, `font`, `marquee`, `multicol`, `nobr`, `spacer`, `tt`는 위 113개에 포함하지 않는다. 이 목록을 일반 `<div>`로 조용히 변환하지 않으며, 수용 여부와 진단 형식은 구현 전 별도 계약으로 정한다. WHATWG 표준은 `acronym`을 의미·렌더링상 `abbr`과 동등하게 취급하도록 요구하지만, 그 요구가 스피논 지원을 자동으로 만들지는 않는다.

### 호환 범위와 단일 원본

| 표면 | 적합성 비교 단위 | 규범 원본과 현재 경계 |
| --- | --- | --- |
| HTML 요소·속성 | 최종 어댑터 출력의 tag·namespace·속성·자식 순서, 요소별 기본 동작 | 이 문서의 인벤토리는 매핑 후보만 정한다. 속성별 동작·오류는 이 문서에서 계약을 추가한 뒤 판정하며, 무시되거나 일반 상자로 대체되는 동작을 지원으로 표시하지 않는다. |
| CSS | 선택자 매칭 → cascade/상속 → 계산값 → 레이아웃 → 페인트 | [CSS 호환 명세](0008-css-compatibility.md)가 범위·Chromium 비교·오차 기준의 원본이다. CSS 매칭·계산값만 성공해도 레이아웃·GPU 표시 지원을 뜻하지 않는다. |
| 이벤트 | 프레임워크 adapter 문법과 런타임 입력·target·순서·취소 결과 | [UI 트리·이벤트 명세](0002-ui-tree-events.md)가 원본이다. `onClick` 문법은 DOM Event 전체나 capture/bubble/default action 지원을 뜻하지 않는다. |
| DOM 유사 API | 메서드별 입력·반환·오류·변경 반영 시점·wrapper 수명 | [모바일 DOM 호환 명세](0007-dom-compatibility.md)가 원본이다. 제한된 façade는 브라우저 DOM·HTML parser·전체 `window`가 아니다. |
| JavaScript 호스트 API | API·실행 환경별 타입·비동기·취소·오류·종료 동작 | 이 문서의 호스트 경계와 [J01–J18 구현 체크리스트](STATUS.md#javascript-api-구현-체크리스트)가 원본이다. V8 언어 기능에 `fetch`·타이머·DOM이 내장되는 것은 아니며 현재 공개 지원 API는 없다. |
| 프레임워크·번들러 입력 | React/Vue/Svelte adapter와 웹·모바일 별 산출물의 실제 호출/태그 출력 | 프레임워크 구성요소 호출과 최종 host element 생성을 구분한다. 첫 적합성 대상은 React·Vite·단일 모바일 번들이며 다른 framework·bundler 조합은 별도 대상이다. 웹 빌드는 브라우저 DOM을 쓴다. 모바일 지원은 adapter와 호스트 동작의 검증으로 판정한다. |

이 표의 `논리 요소 1:1 후보`, `조건부·전용 구현`, `문서·빌드·DOM 전용`, `모바일 GPU 장면에 직접 대응 없음`은 목표 매핑 분류다. 현재 적합성 표기가 아니다. 대상 버전·플랫폼을 정하지 않은 기능은 `미정`, 범위를 정했지만 아직 시험하지 않은 기능은 `미검증`이다. 근거 없이 `미지원`이라고 추정하거나 내부 노드 저장을 공개 지원으로 승격하지 않는다.


초기 UA 규칙 후보는 [`spinon-style` 내장 stylesheet 초안](https://github.com/ohah/spinon/blob/main/crates/spinon-style/resources/ua/supported-elements-v0.css)으로 관리한다. 이 자원은 HTML namespace의 9개 태그를 대상으로 Rust 바이너리에 포함되지만 Stylo cascade나 화면에는 아직 연결되지 않았다. 이는 공개 지원 요소 목록이 아니다. 폼 컨트롤의 외형·링크 상태별 표현도 미구현이다. 세부 범위는 [CSS 호환 명세](0008-css-compatibility.md)와 [구현 상태 대장](STATUS.md)의 C01·C04를 따른다.

첫 수직 구현 외의 요소를 조용히 일반 `<div>`처럼 바꾸지 않는다. 지원되지 않는 요소와 화면에 영향을 주는 속성은 빌드 또는 개발 실행에서 진단한다. 제한된 `document`·노드 API의 공개 후보는 [DOM 호환 명세](0007-dom-compatibility.md)에 둔다. S03.2 내부 시제품이 작은 모바일 façade를 V8·Rust 문서 트리에 연결했지만, 이는 공개 지원이나 전체 브라우저 DOM·모든 태그 조회 기능이 있다는 뜻이 아니다.

## CSS 처리 단계

최종 목표는 버전을 고정한 Chromium과 CSS 관찰 동작을 100% 맞추는 것이며, 현재 지원이나 첫 릴리스 범위를 뜻하지 않는다. 기능군과 완료 우선순위는 [CSS 호환 명세](0008-css-compatibility.md), 지원 상태는 [구현 상태 대장](STATUS.md#css-구현-체크리스트)에 둔다.

웹 빌드는 브라우저 CSS 엔진을 사용한다. 모바일은 Spinon 문서 트리를 Stylo에 연결해 선택자·계단식·상속·계산 스타일을 구하고, 자체 변환 계층에서 Taffy와 추가 레이아웃 알고리즘으로 연결한다. 페인트 속성은 GPU 렌더 경로에서 별도로 처리한다. Stylo가 CSS를 파싱하거나 계산하는 것만으로 렌더링 지원을 표시하지 않는다. 모바일 런타임은 Blitz DOM에 의존하지 않는다.

Vite·Rspack은 CSS import·모듈·에셋·청크 관계를 보존한다. Lightning CSS는 의미를 바꾸는 변환을 Chromium 기준과 동등하다고 확인한 범위에서만 사용할 수 있다. 구현은 기본 상자 모델·단위·Flex·색·글꼴부터 시작하고, Grid·인라인 서식·반응형·애니메이션·고급 레이아웃으로 확장한다. 각 기능은 fixture로 검증하기 전까지 제안 또는 미검증이며 첫 릴리스 포함 여부는 별도 결정이다.

Tailwind는 별도 모바일 렌더러가 아니다. Tailwind가 **생성한 CSS**가 같은 변환 경로에 들어간다. 유틸리티 하나가 사용하는 선택자·선언·값·변수·계층이 모두 지원될 때만 그 유틸리티를 지원이라고 표시한다. Preflight, 테마 변수, 반응형·상태 변형은 별도 항목으로 판정한다.

## JavaScript와 호스트 API

V8의 ECMAScript 언어 기능과 스피논이 제공해야 하는 호스트 기능을 구분한다. `Promise`, `Map`, `ArrayBuffer`, `globalThis` 같은 언어 기능의 존재가 `fetch`, 타이머, `URL`, `console`, DOM을 자동으로 제공하지 않는다.

첫 수직 구현은 JS 이벤트 콜백, 마이크로태스크 체크포인트, 타이머, 예외·콘솔 출력을 목표로 한다. `fetch`, 저장소, 바이너리·네트워크 객체는 각각 입력·결과·오류·취소·백그라운드 동작을 정의한 뒤 지원 여부를 표시한다. 모바일 `document`는 전체 브라우저 문서가 아니라 [별도 명세](0007-dom-compatibility.md)의 제한된 호환 façade로 다루며 현재 구현은 내부 시제품에 한정한다. 전체 `window`, 서비스 워커, Canvas/WebGL/WebGPU는 모바일 기본 API로 약속하지 않는다. GPU 내부 구현이 앱 공개 WebGPU를 뜻하지 않는다.

### 엔진 연결과 사용자 확장 경계 제안

V8 임베딩에는 엔진별 C++ 어댑터가 필요하다. 이 경계는 V8 Isolate·Context와 JS 값·함수의 수명 및 Rust 호스트 호출을 연결하는 내부 구현이며 React Native의 JSI를 가져다 쓰거나 공개 API로 노출한다는 뜻이 아니다. JSI는 React Native가 JavaScript와 C++ 객체를 연결하는 인터페이스다. 스피논은 V8을 직접 임베딩하므로 V8 API에 맞는 내부 연결이 필요하다.

앱 개발자가 Kotlin·Swift·Rust·C++로 고유 기능을 추가하는 요구는 별도의 **버전 있는 플랫폼 모듈 계약**으로 받는다. 이 계약은 JS 타입 선언·빌드 시 바인딩 생성·플랫폼별 등록·비동기 결과 전달을 엔진 어댑터 뒤에 둔다. 입력·출력·오류·권한·취소·thread affinity·Isolate 수명·앱 종료·바이너리 호환성을 명시한다. 앱 코드는 버전 있는 TypeScript/JavaScript 표면만 사용하고 V8 핸들, Rust 포인터, 내부 C ABI를 직접 소유하지 않는다.

이미지·카메라 프레임 같은 대용량 데이터의 복사를 피할 필요가 생기면 일반 모듈 호출과 분리해 소유권·해제·역압력을 갖춘 불투명 핸들이나 공유 버퍼 경로를 검토한다. 기본 모듈 API를 모든 호출의 zero-copy 경로로 설계하지 않는다. 모듈 등록 형식과 코드 생성기는 [JS API 구현 체크리스트의 J15](STATUS.md#javascript-api-구현-체크리스트)에서 X08의 하위 작업으로 추적하며 아직 공개 지원 API가 아니다. 세부 JS API 작업 항목은 [JS API 구현 체크리스트](STATUS.md#javascript-api-구현-체크리스트)에 둔다.

### Fetch 호스트 API 제안

웹의 `fetch()`는 브라우저 호스트 API다. V8을 임베딩하는 것만으로 제공되지 않는다. 모바일에서도 웹과 같은 호출 표면을 제공하려면 JS의 `fetch`·`Request`·`Response`·`Headers`와 취소 신호를 스피논 호스트 API로 연결해야 한다. 정확한 첫 API 범위와 본문 스트림 지원은 미정이며 현재 지원 API가 아니다.

이 경로는 UI 트리와 분리한다. V8의 Promise·객체 표면 → 버전 있는 `NetworkHost` 요청/응답 계약 → Android·iOS 또는 공통 네트워크 전송 구현으로 나눈다. 기본 URL, 리다이렉트, 헤더, 본문, 취소, 앱 백그라운드 전환, TLS 오류, 쿠키·자격 증명, CORS/출처 정책을 각 계층의 책임과 적합성 사례로 정한다. HTTP 비성공 상태는 Fetch 의미에 맞게 Response로 돌려주고, 전송 실패·취소는 별도로 다룬다.

비동기 응답·오류·취소 결과는 V8 Isolate의 소유 실행 경로에 작업으로 전달해 Promise를 처리해야 한다. Promise reaction은 해당 실행 경로의 V8 마이크로태스크 규칙을 따라야 한다. 네트워크 작업 스레드가 V8 객체나 콜백을 직접 만지는 구조는 호스트 계약으로 가정하지 않는다. Isolate 종료·앱 재시작·OTA 교체와 경합할 때 요청 취소, Promise 결과 폐기 또는 거부, 콜백 해제 규칙을 정하고 검증한다. 이 규칙이 UI 스레드 차단을 요구하지 않는지도 실기기에서 확인한다.

WHATWG Fetch 전체는 브라우저 환경, 출처 정책, 스트림과 여러 웹 표준에 걸친 큰 계약이다. 첫 구현의 범위를 정하지 않고 `fetch()` 함수만 추가해서 호환을 주장하지 않는다. [WHATWG Fetch Standard](https://fetch.spec.whatwg.org/)를 기준으로 포함·제외 항목을 정하고, 웹·Android·iOS의 결과·오류·취소·본문 소비를 비교한 뒤 API 명세를 분리한다. 이 작업은 HTML 파서나 DOM 트리를 바꾸지 않으며 Blink·WebView 삽입을 요구하지 않는다.

브라우저와 같은 이름의 API라도 앱 권한·쿠키·출처·수명주기 때문에 동작이 다르면 `차이`로 기록한다. 모바일 전용 기능은 웹 API 이름을 흉내 내지 않고 명시적 플랫폼 모듈에서 제공한다.

`<a>`의 앱 내부 이동, 외부 URL, 시스템 뒤로 가기와 딥링크는 [라우팅 계약](0006-routing.md)에 따른다. 모바일에 `window.location`·`window.history` 전체가 있다고 가정하지 않는다.

## 적합성 자료

[HTML·CSS 대응표](https://macstudio.tailed42f2.ts.net/spinon/compatibility.html)와 [JS API 대응표](https://macstudio.tailed42f2.ts.net/spinon/js-api.html)는 기능 후보를 살펴보는 비규범 미리보기다. 이 사이트의 내용이 `spec/`의 목표 범위·지원 상태를 변경하지 않는다. 현재 표의 단계 표시는 구현 증거가 아니다. R01의 목표 범위와 기능별 버전·플랫폼 적합성 표는 저장소의 `spec/`을 원본으로 작성하고 이 명세에 연결한다.
