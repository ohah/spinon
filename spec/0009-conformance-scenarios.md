# 0009 · 적합성 시나리오 카탈로그

**상태:** 제안 · **명세 버전:** `0.1.0-draft` · **시나리오 상태:** 7개 제안, 0개 실행

이 문서는 첫 적합성 작업의 입력·oracle·관찰값·판정 경계를 제안한다. `제안`은 구현·지원·통과를 뜻하지 않는다. 각 시나리오의 실행 결과는 웹·Android·iOS별로 따로 기록하며, 현재 공개 지원 완료 API는 [API 목록](api/index.md)과 같이 0개다.

## 계약 소유와 버전

아래 문서는 모두 `0.1.0-draft` 제안 계약이다. 표는 탐색용 교차표이고 규범 내용을 복제하지 않는다. 계약이 충돌하면 해당 소유 문서를 고친 뒤 이 카탈로그를 갱신한다.

| ID | 버전 | 규범 소유 범위 | 현재 상태 |
|---:|---|---|---|
| [0001](0001-conformance.md) | `0.1.0-draft` | 용어, 지원 상태, 버전, 적합성 판정 | 제안 |
| [0002](0002-ui-tree-events.md) | `0.1.0-draft` | UI 트리 변경, 입력 대상, 이벤트와 revision 경계 | 제안 |
| [0003](0003-web-surface.md) | `0.1.0-draft` | HTML·CSS·JavaScript 작성 표면과 미지원 진단 | 제안 |
| [0004](0004-runtime-build.md) | `0.1.0-draft` | 런타임, 웹·모바일 빌드, 번들·OTA 경계 | 제안 |
| [0005](0005-developer-tools.md) | `0.1.0-draft` | CLI, 리로드·HMR, Inspector, UI 조사, MCP | 제안 |
| [0006](0006-routing.md) | `0.1.0-draft` | 위치, 방문 기록, 링크, 플랫폼 뒤로 가기 | 제안 |
| [0007](0007-dom-compatibility.md) | `0.1.0-draft` | 제한 DOM façade와 웹 DOM 차이 | 제안 |
| [0008](0008-css-compatibility.md) | `0.1.0-draft` | CSS oracle, 범위, 오차, 지원·거부 정책 | 제안 |
| 0009 (이 문서) | `0.1.0-draft` | 시나리오 서식과 시나리오 결과 기록 | 제안 |

앱 작성자가 사용하는 공개 API 계약은 `spec/0001`–`0008`이 소유한다. [내부 명세 인덱스](internal/README.md)의 HostDocument·FFI·렌더 경계는 내부 구현 계약이며 공개 DOM/API 지원 근거가 아니다. 문서 ID는 고정 식별자이고 계약의 출시 버전이 아니다. 사용자 결정 전 숫자 버전은 `0.1.0`으로 유지한다.

## 결과 상태와 판정 규칙

문서 계약의 `제안`, 목표 범위의 `미정`, 실행하지 않은 시나리오 `미실행`, 실행 조건 때문에 결과를 얻지 못한 `차단`, oracle과 다른 결과 `실패`, 판정 기준을 모두 충족한 `통과`, 해당 플랫폼이 목표 대상이 아닌 `해당 없음`을 구분한다. `차단`은 통과도 실패도 아니다.

플랫폼의 `통과`는 지정 버전·빌드·입력·oracle에 대해 필수 관찰값과 negative control을 모두 만족하고 원본 로그·화면·hash가 연결된 경우에만 기록한다. 한 플랫폼의 통과, unit test, Chrome 비교, simulator 실행, GPU submit만으로 다른 플랫폼이나 공개 지원을 추론하지 않는다. `지원` 표기는 [0001](0001-conformance.md#지원-표기)의 정의와 근거를 따라 별도로 판정한다.

각 시나리오 기록에는 다음 항목을 보존한다.

- 고유 ID `SC-`와 시나리오 버전, 소유 명세 ID·버전, 관련 로드맵 ID
- 앱·adapter·bundle·런타임·OS·기기 또는 simulator·viewport·DPR/scale·locale·색상 설정
- 고정 입력, 초기 문서/환경 상태, fixture·oracle의 버전과 SHA-256
- 기대 geometry·paint·event order·접근성 노출·diagnostic·오류와 각 필드의 허용 오차
- positive control·negative control, 취소·stale·재생성 조건과 pass/fail 규칙
- 플랫폼별 결과 상태, 원본 로그·이미지·hash, 실행하지 않은 경계와 차이 해석

## 첫 시나리오 제안

아래 일곱 시나리오는 모두 초안이고 웹·Android·iOS에서 **미실행**이다. 연결한 기존 브라우저·시뮬레이터 기록은 입력과 oracle의 출처일 뿐 이 통합 시나리오의 통과 근거가 아니다.

### SC-UI-001 · 혼합 자식 순서와 동기 조회

- **계약:** [0002](0002-ui-tree-events.md)와 [0007](0007-dom-compatibility.md), `0.1.0-draft`; 표준 oracle는 [WHATWG DOM](https://dom.spec.whatwg.org/)의 노드 순서·`textContent`다.
- **입력:** 빈 앱 문서에서 `<div id="root">left<span>middle</span>right<em>end</em></div>`와 동등한 노드를 만든다. append 뒤 같은 JS 흐름에서 `childNodes`, `parentNode`, `textContent`를 읽는다.
- **기대:** 자식 종류·순서는 `#text, span, #text, em`이고 text 값은 `leftmiddlerightend`여야 한다. 삽입 직후 부모·형제 조회는 새 revision을 반영한다. self/descendant 삽입 negative control은 트리와 revision을 원자적으로 보존하고 정의된 오류를 낸다.
- **관찰·판정:** Node 종류·순서·관계·문자열·revision·오류 이름과 이전 트리 hash를 기록한다. fixture 동작을 고정하기 전에 HTML 파서 사용 여부와 오류 mapping을 명세해야 하므로 현재 실행은 보류한다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. 모바일 `textContent` setter, wrapper identity, 오류 종류, 노드 수명은 미정이다.

### SC-CSS-001 · Block layout과 불투명 paint 경계

- **계약:** [0008](0008-css-compatibility.md), `0.1.0-draft`; oracle 입력은 [`C08-block-flow-v1`](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c08/block-flow.html), [고정 reference](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/references/c08-block-flow-v1.json), Chrome `154.0.8037.98`이다.
- **입력:** viewport `320×240` CSS px, DPR 1과 2. reference에 고정된 root `280px`, 중첩 block, `display:none` subtree와 정적 배경색을 유지한다. HTML·inventory·reference SHA-256은 JSON manifest 값을 사용한다.
- **기대:** computed value는 exact match, 각 node의 `x/y/width/height` 오차는 필드별 최대 `0.5 CSS px`, 평균으로 가리지 않는다. root/hero/hero-child/first/second/last frame은 reference에 고정된 geometry와 일치하고 `hidden` 및 descendant frame은 모두 0이며 scene에서 제외된다. paint positive samples는 고정 fixture의 각 불투명 box 내부 `(10.5,5.5)`, `(10.5,20.5)`, `(10.5,40.5)`, `(10.5,60.5)`, uncovered root `(150.5,60.5)`, `(10.5,75.5)` CSS 좌표를 사용하고, 각각 `rgb(96,165,250)`, `rgb(51,102,255)`, `rgb(225,29,72)`, `rgb(249,115,22)`, `rgb(16,24,39)`, `rgb(34,197,94)`를 기대한다.
- **negative control:** hidden subtree의 red/green background가 frame·scene·sample에 나타나면 실패다. stale layout input, 누락/중복 node, DPR 혼용도 실패다. capture는 compositor/scanout 이전 표면만을 증명한다.
- **플랫폼 상태:** 웹 Chrome reference는 고정 입력의 oracle이며 새 시나리오 실행은 미실행 · Android 미실행 · iOS 미실행. 기존 C08 evidence는 이 catalog의 통과로 합산하지 않는다. cross-platform color management와 app capture scale은 실행 전 기록한다.

### SC-INPUT-001 · 단일 tap과 취소된 pan

- **계약:** [0002 이벤트 경계](0002-ui-tree-events.md#이벤트), `0.1.0-draft`; 기준은 고정 Chromium의 이벤트 trace와 각 플랫폼 native recognizer 결과를 분리해 사용한다.
- **입력:** 화면 가운데 한 `button`을 native tap으로 한 번 누른 뒤 놓는다. 별도 fresh session에서 pan threshold를 넘기는 move와 cancel을 넣고, 이어 두 번째 pointer가 활성 gesture 중 들어오는 경우를 실행한다.
- **기대:** accepted tap은 같은 target에서 activation 1회, canceled pan/second-pointer cancel은 click 0회다. 각 event에는 input이 선택한 frame/revision과 target identity가 보존되고 새 문서의 다른 node로 재-targeting하지 않는다.
- **negative control:** handler 제거 후 재호출, duplicate terminal, stale frame target, cancel 뒤 추가 move는 모두 0 activation이어야 한다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. 공개 event 이름·인자·전파·기본 취소·키보드 활성화 순서는 [0002](0002-ui-tree-events.md)의 미정 항목이며, 정하기 전 event-order 판정은 차단된다.

### SC-ASYNC-001 · handler·commit·microtask·표시 순서

- **계약:** [0002](0002-ui-tree-events.md), [0003](0003-web-surface.md), `0.1.0-draft`; 실행-loop oracle는 고정된 JS engine과 public host contract가 정해진 뒤 선택한다.
- **입력:** 하나의 handler에서 시작 marker, 노드 text 변경 commit, 같은 handler의 즉시 조회, 종료 marker, `queueMicrotask` marker를 기록하고 frame marker를 별도 수집한다.
- **기대:** 동기 commit/조회가 계약으로 채택될 경우 `handler-start → committed-value-read → handler-end → microtask`가 보존되고, GPU/frame 완료는 handler 완료를 동기 차단하지 않는다. 각 trace에는 document/style/environment/frame revision을 함께 저장한다.
- **negative control:** stale snapshot, callback 취소, session close 후 callback, commit 실패는 이전 문서 상태를 보존하며 잘못된 완료·표시 marker를 만들지 않아야 한다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. public commit·microtask checkpoint·frame completion 사이의 지원 계약이 미정이어서 현재 예상 순서는 제안이며 실행 gate는 차단이다.

### SC-DIAG-001 · 미지원 CSS의 위치와 이유

- **계약:** [0003](0003-web-surface.md)와 [0008](0008-css-compatibility.md), `0.1.0-draft`.
- **입력:** 고정 소스 파일 `unsupported-sticky.css`, 1행 1열의 `position: sticky; top: 0;`을 넣고 해당 profile이 아직 이 값을 지원하지 않는 앱을 빌드한다. 같은 input에는 지원되는 block declaration positive control을 둔다.
- **기대:** sticky 선언을 `static`으로 조용히 대체하지 않고 원본 file·line·column, property/value와 원인을 식별하는 build/development diagnostic을 낸다. 양성 대조는 정상 처리된다.
- **negative control:** 잘못된 selector, 선언 순서 변경, duplicate diagnostic은 위치를 잃거나 성공으로 바뀌면 실패다. baseline fixture hash가 바뀌면 결과는 무효다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. 정확한 diagnostic ID·severity·JSON/CLI 형식과 target profile은 미정이므로 machine-readable 비교는 차단된다.

### SC-LIFE-001 · surface 재생성과 stale frame 차단

- **계약:** [0002](0002-ui-tree-events.md)와 [0004](0004-runtime-build.md), `0.1.0-draft`; [R13 evidence](internal/evidence/r13-platform-gpu-recovery-2026-09-29.md)는 기존 lifecycle 입력 참고다.
- **입력:** `320×240` CSS viewport에 고정 UI tree를 표시한 뒤 Android Activity resize/백그라운드 복귀 또는 iOS scene resize/background 전환으로 surface를 다시 만든다. 이전 generation의 pending frame을 재생성 뒤 제출하려는 negative control을 주입한다.
- **기대:** 새 `SurfaceGeneration`과 현재 viewport로 새 frame을 만들고, 이전 generation frame은 GPU 제출 전에 거부한다. document tree/generation/revision은 화면 재생성만으로 임의 증가·초기화되지 않는다. 복구 실패는 정의된 오류 상태로 관찰 가능해야 한다.
- **관찰:** lifecycle callback trace, document/environment/surface revision, frame ID, admission/rejection, 표면 screenshot/hash를 기록한다. 제출과 OS 표시 완료를 구분한다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. R13의 개별 surface 복구 결과는 이 통합 tree/snapshot 시나리오 PASS가 아니다. 성공·복구 불가 오류 정책이 확정되기 전까지 판정은 차단이다.

### SC-A11Y-001 · 접근성 이름과 단일 활성화

- **계약:** [0001 최소 적합성](0001-conformance.md#적합성의-최소-단위), [0002 이벤트](0002-ui-tree-events.md), [0003 HTML 표면](0003-web-surface.md), `0.1.0-draft`.
- **입력:** `<button aria-label="Continue">Continue</button>` 한 개를 렌더하고 browser accessibility tree, Android accessibility node, iOS accessibility element를 각각 조사한다. 보조 기술/플랫폼 accessibility action으로 한 번 활성화한다.
- **기대:** 각 target이 role `button`, accessible name `Continue`, 활성 상태를 노출하고 action은 동일 UI node의 handler를 정확히 한 번 활성화한다. 시각 tap과 accessibility action 중복 전달은 없어야 한다.
- **negative control:** hidden/removed button은 접근성 tree와 action 대상에서 사라지고, 빈 accessible name은 대체 값을 임의로 만들지 말고 계약된 진단/계산 규칙을 따른다.
- **플랫폼 상태:** 웹 미실행 · Android 미실행 · iOS 미실행. `aria-label` 매핑, role/name 계산, focus·activation과 TalkBack/VoiceOver 동작은 미정이어서 판정은 차단이다.

## 미정 계약 등록부

아래 질문은 해결 전까지 지원 표기나 시나리오 통과 판정을 막는다. 결정은 각 소유 명세와 구현/근거 변경에서 해야 한다.

| 질문 | 소유 문서 | 시나리오 영향 | 결정 전 경계 |
|---|---|---|---|
| 첫 릴리스의 포함 API·최소 Android/iOS OS·framework/bundler 조합은 무엇인가? | [0001](0001-conformance.md), [0003](0003-web-surface.md), [0004](0004-runtime-build.md) | 전체 platform matrix | 미정 범위를 지원으로 표시하지 않음 |
| public spec/bundle/host API의 첫 지원 버전과 호환성 변경 규칙은 무엇인가? | [0001](0001-conformance.md), [API index](api/README.md) | 모든 SC ID의 지원 version | 내부 `0.1.0`을 공개 지원으로 승격하지 않음 |
| 혼합 text/element child, parser, `textContent`, wrapper identity, 오류·재삽입 의미는 무엇인가? | [0002](0002-ui-tree-events.md), [0007](0007-dom-compatibility.md) | SC-UI-001 | DOM façade 지원 주장을 보류 |
| target frame, event argument/order, propagation, cancellation, keyboard·accessibility activation은 무엇인가? | [0002](0002-ui-tree-events.md) | SC-INPUT-001, SC-A11Y-001 | click 한 건 외의 웹 이벤트 동등성을 주장하지 않음 |
| JS host APIs·microtask checkpoint·취소·예외·frame completion의 정확한 순서는 무엇인가? | [0002](0002-ui-tree-events.md), [0003](0003-web-surface.md), [0004](0004-runtime-build.md) | SC-ASYNC-001 | 제출/커밋/표시 신호를 합치지 않음 |
| CSS computed value·font·color·pixel tolerance와 unsupported diagnostic schema는 무엇인가? | [0003](0003-web-surface.md), [0008](0008-css-compatibility.md) | SC-CSS-001, SC-DIAG-001 | fixture·필드별 허용치만 사용하고 보편적 pixel parity를 주장하지 않음 |
| surface recreation, backgrounding, unhandled loss와 stale frame failure의 사용자 결과는 무엇인가? | [0002](0002-ui-tree-events.md), [0004](0004-runtime-build.md) | SC-LIFE-001 | 개별 GPU smoke를 lifecycle conformance로 확장하지 않음 |
| CLI·HMR·Inspector·MCP 출력/종료 오류와 공개 개발 서비스 범위는 무엇인가? | [0005](0005-developer-tools.md) | 후속 tool 시나리오 | 문서 제안만으로 도구 지원을 표시하지 않음 |
| push/replace/back·루트 종료·딥링크·경로 복원 시점은 무엇인가? | [0006](0006-routing.md) | 후속 routing 시나리오 | OS 뒤로 가기 결과를 추정하지 않음 |
| OS accessibility tree, role/name, focus/action과 화면 크기 설정의 공통 기대값은 무엇인가? | [0001](0001-conformance.md), [0002](0002-ui-tree-events.md), [0003](0003-web-surface.md) | SC-A11Y-001 | accessibility action을 click과 동일시하지 않음 |

이 목록은 현재 확인한 질문을 정리한 것이며 제품 결정 요청이나 승인 기록이 아니다. 미정 질문은 소유 계약에서 결정된 뒤 이 표와 영향을 받는 scenario criteria를 함께 갱신한다.

## 버전·변경 이력

이 카탈로그와 시나리오 ID는 공개 적합성 계약의 초안 버전 `0.1.0-draft`에 속한다. 문서 ID `0009` 및 시나리오 ID `SC-*`는 변경 가능한 제품 버전이 아니다. 시나리오 입력·oracle·pass criteria가 달라지면 이력과 fixture hash를 남긴다. 지원 버전 숫자 변경은 사용자가 출시·호환성 정책을 정한 뒤에만 한다.
