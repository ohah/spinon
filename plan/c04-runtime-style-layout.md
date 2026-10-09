# C04 · Runtime CSS 스타일을 Taffy 레이아웃으로 연결

- **문서 유형:** 구현 계획 · 실제 구현 상태는 [상태 대장](../spec/STATUS.md#css-구현-체크리스트)에서 관리
- **기준일:** 2026-10-09
- **상위 항목:** [C04 stylesheet·selector·cascade](../spec/STATUS.md#css-구현-체크리스트)
- **선행 구현:** C04.8 RuntimeSession UA cascade, C04.6 제한 margin adapter, S02 HostDocument Taffy 입력, R06 CSS worker
- **계약 버전:** 미출시 내부 계약 `0.1.0` 고정. 이 계획과 구현만으로 숫자 버전을 올리지 않는다.

## 목적

C04.8이 계산한 CSS와 같은 HostDocument snapshot에서 실제 Taffy 프레임을 만든다. DOM 변경 뒤 runtime이 CSS 계산 결과만 반환하는 단계에서 끝나지 않고, 지원하는 inline CSS가 레이아웃 좌표에 반영되는 내부 기능을 추가한다. 스타일 계산과 Taffy 계산은 현재 CSS 전용 worker에서 순차 처리하고, native host는 완료된 불변 레이아웃 snapshot을 복사해 읽는다.

이 기능은 앱 공개 CSS 지원이나 화면 렌더링 완료가 아니다. GPU 장면 연결은 별도 S04 작업으로 남긴다.

## 고정 입력·동작 경계

- 입력은 RuntimeSession의 현재 불변 `HostDocumentSnapshot`, 명시적으로 설정된 `CssViewport`, 그리고 DOM façade가 이미 저장한 HTML `style` 속성이다. 외부 stylesheet 목록이나 원격 자원은 추가하지 않는다.
- cascade view는 C04.8과 같은 HTML fragment-root 의미를 유지한다. HostRoot나 임의 앱 root를 CSS `:root`·document element로 가장하지 않으며 기존 `:root` selector 결과도 바꾸지 않는다.
- 새 `RuntimeFlexLayout` 계산 profile은 UA stylesheet와 HTML `style` 속성을 한 번의 Stylo cascade에서 계산한다. profile은 C04.8의 7개 UA 속성에 `box-sizing`, `width`, `height`, Flex 방향·크기·정렬·간격, 네 physical `margin`과 네 physical `padding`을 더한 superset이다. C04.8 JSON serializer는 이 snapshot에서 기존 7개 속성만 골라 기존 schema·필드·revision 그대로 출력한다. 새 계산을 위해 같은 노드의 Stylo cascade를 두 번 실행하지 않는다.
- 허용 author inline declaration은 다음으로 고정한다: `display`, `box-sizing`, `width`, `height`, `flex` shorthand와 `flex-direction`, `flex-grow`, `flex-shrink`, `flex-basis`, `direction`, `align-items`, `justify-content`, `gap`, `row-gap`, `column-gap`, `margin`과 physical/logical margin longhand·shorthand, `padding`과 physical/logical padding longhand·shorthand. Stylo가 shorthand를 longhand로 확장한 뒤 계산하므로 허용 여부는 확장된 declaration 이름으로 검사하고, 계산된 longhand 값도 아래 Taffy projection 범위에 들어야 한다. 예를 들어 `flex: 0 1 auto`는 허용되지만 `flex: 1`이 만든 percentage `flex-basis`는 layout을 실패시킨다. 이 밖의 declaration 또는 layout 의미를 바꾸는 미지원 문법은 기존 UA JSON을 유지하면서 layout 상태에 구체 진단을 싣고 `failed`로 만든다. Stylo가 보고하는 일반 CSS syntax diagnostics는 브라우저처럼 유효 선언의 recovery 계산을 유지하고 JSON에 함께 반환한다. 부분 계산 결과는 게시하지 않는다.
- Taffy projection은 `display: flex|block|none`, px 또는 `auto` 크기, 기존 Flex 방향·정렬·간격, 유한한 nonnegative px padding, 기존 margin profile이 허용하는 px margin 값만 받는다. viewport에 직접 대응하는 단일 root의 nonzero margin, `auto` margin·음수 padding·percentage·font-relative 값은 지원하지 않는다. 값은 문서 fixture에 기록한 parser·Stylo computed serialization으로 정확히 해석한다. 나머지 값은 default 처리하지 않고 해당 layout snapshot을 실패시킨다.
- CSS worker는 스타일 계산과 레이아웃 투영을 같은 요청 key에서 순차 실행한다. 각 key는 generation, document revision, render-tree revision, style revision, environment revision을 모두 포함한다. 최신 요청만 게시하며 오래된 계산 결과는 버린다. Taffy 오류나 미지원 값은 cascade 완료 상태를 덮어쓰지 않고 layout 상태에만 기록한다. 기존 C04.8 `computationDurationUs`는 Stylo cascade 시간으로 유지하고 layout 완료 객체의 시간은 cascade 뒤 Taffy 투영 구간만 따로 기록한다.
- 첫 slice는 정확히 하나의 HostRoot 직속 Element와 그 Element 하위의 Element만 받는다. HostRoot가 비어 있으면 성공한 `empty`, 여러 HostRoot 요소가 있으면 교차 root 위치를 임의 추정하지 않고 layout 전체를 `failed`로 둔다. text node, inline formatting, replaced element, list marker는 구체 오류로 거부한다. 이 조건은 향후 지원을 영구 제외하지 않는다.
- `spinon_runtime_session_copy_layout_json(session, output, output_capacity, required_capacity)`는 숫자 버전을 붙이지 않은 `spinon.runtime.layout` schema를 복사한다. `not_configured`, `pending`, `ready`, `empty`, `failed` 상태와 요청·완료 revision, CSS px 프레임, syntax diagnostics, 실패 이유를 제공한다. 완료값은 `key`와 단일 root의 DOM preorder `frames` 배열이며 각 frame은 `nodeId`, `x`, `y`, `width`, `height`를 가진다. 모든 frame은 유한 좌표와 nonnegative 크기를 만족해야 하며 하나라도 위반되면 부분 frame을 게시하지 않는다. `failed` 진단은 안정 오류 코드·노드 ID·속성 이름을 제공하고 임의 길이 CSS 원문은 복사하지 않는다. 계산을 기다리지 않는다. UTF-8 JSON과 NUL을 포함하는 required capacity, 짧은 버퍼의 빈 NUL guard, session free와의 비동시 사용 규칙은 C04.8 copy API와 동일하게 적용한다. API 호출 자체는 worker 내부 실패를 JSON 상태로 반환하며 인자 오류는 `-1`, 출력 버퍼 부족은 `-3`이다. 함수명·오류 코드·C header 주석은 내부 API 계약에 기록한다.
- JavaScript 실행기와 UI thread에서 Stylo/Taffy 계산을 직접 실행하지 않는다. 별도 OS worker는 추가하지 않는다. worker 종료는 진행 중 계산을 취소하지 않으며 session 종료는 기존 C04.8 join 규칙을 따른다.

## 비교 모델과 합격 기준

- 기준 구현은 Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`이다. 기준 실행 조건은 macOS `26.5.1` (`25F80`) arm64, viewport `800×600` CSS px, device scale 1, locale `en-US`, UTC, light/no-preference/forced-colors none이다. 기존 C01 computed-value reference는 수정하지 않는다. 새 fixture는 JS DOM façade의 `setAttribute("style", ...)`로 만든 단일-root Flex 트리와 Block·`display:none` 경계를 포함하며 입력 HTML, CSS 문자열과 수집기 hash를 고정한다.
- 구현 전 확보한 구체적 HTML·inventory·Chromium 출력과 모든 hash는 [C04 runtime style-layout 비교 기준](../spec/internal/evidence/css-c04-runtime-style-layout-precomparison-2026-10-09.md)에 고정했다. 이 reference는 구현 후 덮어쓰지 않는다.
- Chromium에서 `getComputedStyle()`의 각 profile 속성 문자열과 `getBoundingClientRect()`의 각 노드 `x`, `y`, `width`, `height`를 비교한다. geometry는 CSS px로 비교하고 fixture별 최대 절대 오차를 사전에 0.5 CSS px로 고정한다. 좌표별 실패를 합산 평균으로 숨기지 않는다.
- Rust 검증은 C04.8 JSON의 기존 schema·필드·UA 속성·revision 회귀, 단일·빈 root, 여러 root 거부, text 거부, 깊은 자식·순서 보존, `display:none`, `block`/`flex`, 크기·margin·padding·gap·정렬, 같은 입력 revision, stale 요청 폐기, 빠른 DOM 변경, 미지원 값과 cascade 진단 분리, Taffy 오류, JSON 짧은 버퍼 및 retry를 확인한다.
- Android API 37.1 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8 DOM 변경→비동기 worker→layout JSON 복사를 각각 확인한다. 이는 simulator 결과이며 실기기·GPU draw·표시 완료나 성능 보장을 뜻하지 않는다.
- 실행하지 않은 플랫폼 빌드, 캡처, 성능 측정은 근거에 완료로 적지 않는다. 이 기능에서는 성능 우위를 주장하지 않는다.

## 구현 순서

1. CSS 코드를 바꾸기 전에 단일 root의 Flex·Block·`display:none` inline-style fixture를 만들고 위 고정 Chromium에서 computed property와 `getBoundingClientRect()` reference를 수집한다. 수집 결과·입력·Chromium 실행 파일 hash를 비교 모델로 기록하고 허용 오차를 먼저 고정한다.
2. `spinon-style`에 cascade 한 번으로 계산하는 `RuntimeFlexLayout` profile을 추가한다. C04.8의 UA profile과 기존 fixture profile 동작을 보존하고, C04.8 JSON에는 union snapshot에서 기존 7개 UA 속성만 직렬화한다. 내장 UA origin, inline style 우선순위, parser 진단, 전체 revision을 보존한다.
3. `spinon-style-to-layout`에서 이미 계산된 새 profile을 검증해 Taffy `LayoutInput`으로 투영한다. HostDocument 전체 하위 트리의 node set·DOM order·revision이 정확히 일치하지 않으면 거부한다. 계산된 style snapshot을 다시 Stylo에 넣어 중복 계산하지 않는다.
4. C04.8 CSS worker 요청에 layout 결과와 layout 전용 상태·오류를 추가한다. 새 worker나 무제한 큐는 만들지 않고 기존 capacity-one latest-wins 요청·stale 결과 폐기·종료 경계를 유지한다.
5. `spinon-runtime` 내부 snapshot 조회와 `spinon-ffi` C ABI JSON 복사 함수를 추가한다. 기존 C04.8 UA cascade JSON의 schema·필드·오류·revision 의미를 보존한다.
6. Rust·Bun fixture 및 Android/iOS simulator의 실제 V8 실행을 확인한다. Chromium reference에 computed 문자열 정확 비교와 각 geometry 0.5 CSS px 기준을 적용하며, 모든 성공·실패 결과에서 cascade 상태와 layout 상태를 구분해 보존한다.
7. 버전 있는 내부 인터페이스 명세, 작업 evidence, `spec/STATUS.md`, Tailnet용 CSS 계획 미리보기를 같은 변경 흐름에서 동기화한다. 내부 계약 숫자는 `0.1.0`으로 유지하고 C04 parent는 GPU 표시와 일반 CSS 지원이 끝날 때까지 미완료로 둔다.

## 범위 밖

- GPU 장면 생성·표시 및 S04 product renderer 연결
- author stylesheet 등록, CSSOM, 외부 CSS·폰트·이미지 자원 로딩
- Grid, inline formatting·line box, text measurement, replaced element, list marker
- `%`, `em`, `rem`, `calc()`, `min()`, `max()`, `clamp()`, border, overflow, transform, stacking context
- OS viewport·scheme·pointer 자동 수집, 여러 앱 root 합성, 실기기 결과
- dirty-subtree 계산, 스타일/layout cache, 계산 취소, 성능 우위 주장

미지원 항목은 영구 제외가 아니라 후속 호환성 fixture와 별도 작업의 입력이다. 이 계획은 C04 전체, CSS 호환 목표, 공개 API 또는 GPU 앱 렌더링 완료를 주장하지 않는다.
