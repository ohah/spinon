# R01 · 웹 호환 범위 인벤토리 계획

- **상태:** 규범 표 반영 완료 · R01 적합성·첫 목표 범위 미완료
- **기준일:** 2026-10-09
- **상위 항목:** [구현 상태 대장 R01](../spec/STATUS.md#1-위험-검증)

## 목적

HTML 요소, CSS, 이벤트, 제한된 DOM 유사 API의 호환 경계를 한눈에 판정할 수 있도록 기존 규범 문서의 연결과 분류를 정리한다. 목표는 현재 공개 지원과 앞으로의 호환 목표를 분리하고, 각 HTML 요소를 `논리 요소 1:1 후보`, `조건부/전용 구현`, `문서·빌드·DOM 전용`, `직접 대응 없음`으로 분류하는 것이다.

이 작업은 첫 공식 릴리스 범위를 결정하지 않는다. 기존 명세에 적힌 첫 수직 구현 후보도 제품 지원 약속으로 승격하지 않는다. 각 영역의 세부 규범은 현재 문서인 `spec/0002-ui-tree-events.md`, `spec/0003-web-surface.md`, `spec/0007-dom-compatibility.md`, `spec/0008-css-compatibility.md`에 둔다. 새 문서는 이 규범들의 복제본이 되지 않는다.

## 기준과 범위

- HTML 요소 기준은 [WHATWG HTML Living Standard의 요소 인덱스](https://html.spec.whatwg.org/multipage/indices.html#elements-3)와 [폐기된 기능 목록](https://html.spec.whatwg.org/multipage/obsolete.html)이다. 작업 기준일에 확인한 표준 갱신일은 2026-10-07이다. 이 날짜는 재현 가능한 인벤토리 기준이지 향후 표준 갱신을 자동 반영한다는 약속이 아니다.
- 요소 인벤토리는 표준 인덱스의 HTML 요소 전부를 포함한다. SVG `svg`, MathML `math`, 작성자 정의 autonomous custom element, `Text` 노드는 HTML namespace 표준 태그와 분리한다. 폐기된 요소는 별도 목록으로 두며, 자동으로 일반 `<div>`로 바꾸지 않는다.
- `논리 요소 1:1 후보`는 태그와 namespace를 가진 논리 DOM 유사 요소 하나를 트리에서 보존한다는 뜻이다. 네이티브 `View`/`UIView` 하나나 GPU 도형 하나로의 대응, 브라우저 UA 스타일·접근성·상호작용의 동등성을 뜻하지 않는다.
- `조건부/전용 구현`은 논리 요소는 보존할 수 있으나 레이아웃 알고리즘, 상태·폼 계약, 자원 로더, 플랫폼 입력/접근성 또는 별도 GPU 렌더러가 필요하다는 뜻이다.
- `문서·빌드·DOM 전용`은 모바일 GPU 장면에 직접 그릴 요소가 아니며, 문서 루트·정적 번들 입력·DOM fragment 같은 단계별 계약 후보라는 뜻이다. 동적 DOM·CSSOM·모듈 로딩을 포함하지 않는다.
- `직접 대응 없음`은 브라우징 컨텍스트·플러그인 등 모바일 GPU UI 트리에 자체 대응하는 런타임 요소가 없다는 뜻이다. 사용자가 명시적으로 선택한 별도 WebView 컴포넌트는 이 분류를 바꾸지 않는다.
- 현재 공개 지원 완료 API는 0개다. 내부 PoC나 Rust 노드 이름 보관은 지원 근거가 아니다. 표의 분류는 `목표 매핑 제안`이며, 별도의 `현재 적합성` 상태와 실제 검증 근거 없이 지원으로 표시하지 않는다.

## 변경 계획

1. `spec/0003-web-surface.md`를 HTML 요소 인벤토리의 단일 원본으로 삼아 표준 요소를 빠짐없이 분류한다. 요소별로 별도 HTML/CSS/Event API를 약속하지 않고 필요한 세부 계약 문서와 연결한다.
2. `spec/0003-web-surface.md`에 HTML·CSS·이벤트·DOM 호환의 표면 경계와 교차 문서 표를 둔다. CSS의 전체 기능 범위는 `0008`, 이벤트 순서와 입력은 `0002`, DOM 시그니처·오류·수명은 `0007`을 그대로 참조한다.
3. 기존 `<div>`, 텍스트, `<button>`, `<span>`, `<input>`, `<img>`, `<p>`, `<a>`, `<ul>`, `<li>` 후보를 유지하되 `첫 수직 구현 후보`와 `첫 공식 릴리스 결정`을 명시적으로 분리한다.
4. React JSX/TSX의 `className`·`onClick`, Vue/Svelte 템플릿 문법, 번들러 출력 경계는 요소 호환성 자체와 구별한다. 컴파일 후 모바일 호스트에 전달되는 실제 태그·속성·이벤트 표면을 적합성 단위로 삼는다.
5. R01 설명을 실제 산출물에 맞춰 `spec/STATUS.md`에서 갱신한다. 릴리스 범위·목표 mapping이 아직 사용자 결정 또는 검증을 기다리면 R01 체크를 유지하지 않는다.
6. 변경된 SSOT의 링크만 `spec/README.md`와 `spec/api/index.md`에서 필요한 만큼 연결한다. 지원 완료 API 목록에는 제안 요소를 넣지 않는다.

## 고정할 판정 표

규범 표는 다음 경계를 각각 보인다.

| 영역 | 분류 축 | 세부 원본 |
|---|---|---|
| HTML 요소 | 논리 요소 매핑 후보, GPU/호스트 특수 처리, 현재 공개 지원 상태 | `spec/0003-web-surface.md` |
| CSS | 스타일 계산, 레이아웃, 페인트, 자원 입력이 각각 조건임을 표시 | `spec/0008-css-compatibility.md` |
| 이벤트 | 작성 프레임워크 문법과 런타임 이벤트 의미를 구분하고, 웹/모바일 차이를 기록 | `spec/0002-ui-tree-events.md` |
| DOM 유사 API | 제한된 후보 API의 시그니처, 반환/오류, 동기 조회, 노드·wrapper 수명을 기록 | `spec/0007-dom-compatibility.md` |
| JS 호스트 함수 | DOM과 분리해 fetch·타이머·저장소·플랫폼 모듈의 제안 및 상태를 기록 | `spec/0003-web-surface.md`, `spec/STATUS.md` |

현재 목표 후보와 현행 상태는 다른 열이어야 한다. `미지원`은 미구현이라는 추측만으로 붙이지 않는다. 범위가 정해졌으나 시험이 없는 기능은 `미검증`, 계약이 미정인 기능은 `미정`, 표에 적힌 동작을 시험하지 않은 문서는 `제안`으로 둔다. 공개 지원 API가 없다는 전체 선언과 내부 시제품은 별도로 표기한다.

## 검증 순서와 완료 조건

1. 계획 자체를 서로 다른 20개 실패 관점으로 점검하고 발견 사항을 반영한다.
2. WHATWG 요소 목록을 표준 인덱스와 대조해 누락·중복을 확인한다. SVG/MathML namespace, custom elements, 텍스트 노드, obsolete 요소를 HTML 태그 목록에 섞지 않는다.
3. 규범 문서를 갱신한 뒤 새 20개 실패 관점으로 요소 분류·영역 경계·상태 표기를 재검토한다.
4. 표에 등장하는 모든 지원 주장은 실제 코드·API 버전·브라우저/실기기 근거로 연결한다. 이번 문서 작업만으로 새 기능을 `지원` 처리하지 않는다.
5. `spec/STATUS.md`의 R01 링크와 공개 API 지원 개수가 문서 안에서 모순되지 않아야 한다. `git diff --check`와 로컬 링크/표기 검사를 수행한다. 문서 사이트 배포는 하지 않는다.
6. 공식 첫 릴리스 포함 범위는 결정하지 않는다. R01을 닫을 수 없는 사용자 정책 선택은 `미정`으로 남기고, 기술적으로 정할 수 있는 mapping 기준과 관찰 가능한 지원 상태는 문서화한다.

## 계획 적대 검토 · 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획에 반영한 예방 기준 |
|---:|---|---|
| 1 | 현재 WHATWG 목록과 나중에 갱신된 목록이 섞여 재현 불가 | 표준 스냅샷 날짜와 출처 URL을 문서에 고정하고 갱신 시 새 기준을 기록한다. |
| 2 | 비규범 요소 인덱스를 규범적 지원 요구사항으로 오해 | 요소 표준 인벤토리는 이름의 존재만 기준으로 삼고, 동작 요구는 관련 규범 본문을 따로 확인한다. |
| 3 | `svg`·`math`가 HTML namespace tag처럼 취급 | namespace 경계를 별도 분류하고 CSS/DOM 지원을 HTML 분류에서 추론하지 않는다. |
| 4 | `Text` 노드를 태그 목록에 넣어 태그 수와 구현 대상을 왜곡 | 텍스트 노드는 별도 node-kind로 기록하고 HTML 태그 수 계산에서 제외한다. |
| 5 | custom element가 임의 사용자 태그라는 이유로 자동 지원 처리 | 이름 보존과 CustomElementRegistry·lifecycle·shadow DOM 구현을 분리한다. |
| 6 | JSX 대문자 컴포넌트와 소문자 HTML 태그를 같은 파서 규칙으로 취급 | React/Vue/Svelte 컴파일 결과의 host tag와 component call을 구분해 적합성 입력을 고정한다. |
| 7 | “1:1”을 네이티브 뷰 하나 또는 GPU primitive 하나로 오해 | 용어를 DOM 유사 논리 요소 identity에만 한정하고 시각·접근성·상태 대응을 별도 판정한다. |
| 8 | semantic tag 이름 보존을 접근성 의미 지원으로 과장 | 플랫폼 접근성 트리의 role·name·action·focus 검증이 별도로 필요하다고 표에 명시한다. |
| 9 | `<button>`·`<input>`을 한 종류의 컨트롤로 단순화 | disabled·form state·keyboard·IME·input type별 동작을 조건부/전용 구현으로 둔다. |
| 10 | 파일·날짜·색상 선택기 같은 input type에서 OS UI 의존 누락 | type별 GPU/host/native service 경계를 별도로 계약하기 전에는 일반 input 지원으로 올리지 않는다. |
| 11 | `<img>` 존재를 원격 이미지 로더 지원으로 오해 | 논리 노드, 로컬 bundle resource, 원격 loader, 오류/고유 크기를 각각 나눠 판정한다. |
| 12 | `audio`·`video`·`iframe`·`object`를 일반 GPU 박스로 표시해 기능이 있는 척함 | media, child browsing context, plugin/resource 의미를 조건부 또는 직접 대응 없음으로 분류한다. |
| 13 | 앱의 GPU renderer가 `<canvas>` API와 같다고 추론 | Canvas 2D/WebGL/WebGPU public API와 내부 UI GPU backend를 분리한다. |
| 14 | `<script>`·`style`·`link`의 정적 bundler 처리를 동적 DOM API와 혼동 | 빌드 입력 처리와 runtime node insertion·CSSOM·module loading을 따로 적는다. |
| 15 | `<meta>`·`base`·`title`·viewport가 일반 표시 노드로 잘못 분류 | 문서 metadata, URL base, host viewport/title 정책을 분리해 별도 경계를 둔다. |
| 16 | obsolete·비표준 tag를 `<div>` fallback으로 조용히 렌더 | 구식 목록과 진단/파서 계약을 따로 두고 암묵적인 fallback 지원 주장을 금지한다. |
| 17 | HTML tag 지원을 CSS UA default 지원으로 오해 | tag inventory와 selector/cascade/layout/paint 적합성은 별도 축으로 유지한다. |
| 18 | React `onClick`을 DOM Event 구현 전체와 동일시 | adapter syntax와 capture/bubble/cancel/default action/pointer/keyboard 계약을 분리한다. |
| 19 | 제한된 `Document`가 있다고 `window`·전체 browser DOM까지 있다고 오해 | DOM façade의 메서드별 계약과 별도 web host API 범위를 유지한다. |
| 20 | 표 초안이 첫 릴리스 범위를 대신 결정하거나 내부 PoC를 공개 지원으로 승격 | 후보·현재 적합성·첫 공식 릴리스 결정을 독립 항목으로 두고 R01 완료 조건을 보수적으로 판정한다. |

### 계획 검토 판정

20개 관점은 서로 다른 표준 출처, namespace, 컴파일 결과, 렌더 단위, 플랫폼 서비스, 문서/번들 단계, 이벤트와 API의 경계를 검사한다. 계획 단계의 검토이며 규범 표 변경이나 구현 실행 근거가 아니다.
