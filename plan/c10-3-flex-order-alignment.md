# C10.3 · Flex 순서와 정렬 구현 계획

**상위:** [C10 Flexbox](c10-flexbox.md) · [공식 상태 대장](../spec/STATUS.md)
**현재 상태:** 계획 단계. 기능 구현·테스트·플랫폼 실행을 주장하지 않는다.
**내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정.

## 목표

현재 runtime Flex adapter는 `row|column`, `nowrap|wrap`, 제한된 `align-items`와 `justify-content`만 typed layout 값으로 전달한다. 다음 단계는 CSS의 주축·교차축 방향, Flex item의 `order`, 줄 정렬, item별 정렬과 baseline을 단계적으로 연결한다. Taffy 0.14.0은 후보 계산기이고, 고정 Chromium 154 reference가 수치 비교 기준이다. enum이나 CSS computed property를 매핑하는 것만으로 지원 완료 처리하지 않는다.

이 계획의 모든 단계는 기본 runtime 경로인 V8 → Stylo cascade → typed layout snapshot → Taffy → WGPU를 대상으로 한다. 상태 대장의 C10.3 하위 체크는 각 단계별로 갱신하지만, 다섯 하위 단계와 C15 텍스트 baseline 연결이 끝나기 전에는 C10.3 또는 C10 전체를 완료로 표시하지 않는다.

## 구현 단계

| 하위 ID | 범위 | 완료 전제 | 상태 |
| --- | --- | --- | --- |
| C10.3.1 | `row-reverse`, `column-reverse`, `wrap-reverse`와 `flex-flow` 조합 | C10.1·C10.2에서 고정한 line 수집·크기 배분과 분리해 축 시작점 및 line stacking을 비교한다. | 미구현 |
| C10.3.2 | `order`의 안정적인 계산 순서와 paint 순서 | HostDocument 자식 순서를 유지하고 layout·paint·source traversal을 분리한다. | 미구현 |
| C10.3.3 | `align-self`와 `align-content`의 비-baseline 값 | auto margin, stretch, wrap 상태, line 수와 gap의 상호작용을 비교한다. | 미구현 |
| C10.3.4 | item·line baseline 정렬 및 Flex container baseline | 우선 빈 고정 크기 상자의 합성 first/last baseline을 비교한다. 텍스트 baseline은 C15의 실제 글꼴 측정 계약과 연결하기 전까지 미완료로 남긴다. | 미구현 |
| C10.3.5 | positioned flex child와 순서·정렬 교차 통합 | C12 이후 absolute child는 flex line 계산에서 제외하고 paint order에서는 `order:0`으로 취급하며, static-position `align-self`를 비교한다. | C12 선행 |

각 구현 PR은 하나의 하위 단계를 소유한다. C10.3.1부터 C10.3.4까지 순서대로 진행하고, C10.3.5는 C12 positioning 구현 뒤 별도 PR로 진행한다. 한 PR에 여러 단계를 묶지 않는다. C10.3 parent는 다섯 단계와 C15 텍스트 baseline 연결이 모두 닫히기 전까지 미완료다. 각 기능 구현 PR은 계획 검토와 독립된 새 실패 관점 20개를 기록한다.

## 공통 비교 계약

- **Oracle:** Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`. fixture, capture 도구와 Chromium 실행 파일의 SHA-256을 reference에 저장하고 테스트 실행으로 reference를 자동 갱신하지 않는다.
- **환경:** viewport `320×240 CSS px`, locale `en-US`, timezone `UTC`, DPR 1과 2 각각. 입력 fixture는 `writing-mode:horizontal-tb`, `direction:ltr`를 명시한다. RTL과 다른 writing mode는 C17 선행 계약을 확인하기 전까지 지원 범위에 넣지 않는다.
- **관찰값:** 각 지원 노드의 computed `flex-direction`, `flex-wrap`, `flex-flow`의 longhand 결과, `order`, `align-items`, `align-self`, `align-content`, node ID별 `x`, `y`, `width`, `height`, 필요 시 paint-list 순서를 기록한다. computed CSS 문자열과 Rust typed 값은 별도 필드로 보존한다. 현재 없는 runtime hit-test와 pointer target은 C10.3 관찰값에 넣지 않으며 S05에서 별도 연결한다.
- **기하 허용치:** 각 노드·각 frame field의 최대 절대 오차 `0.5 CSS px`. 평균 오차로 단일 실패를 가리지 않는다. DPR 1·2 결과가 각각 통과해야 하고 CSS px geometry는 DPR에 따라 달라지면 안 된다.
- **스타일 경로:** `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1` 여섯 profile의 typed projection, inline full/incremental cascade, author stylesheet full cascade, 등록 사용자 지정 속성 경로를 확인한다. 현재 author stylesheet incremental 경로가 재사용을 거부하면 전체 cascade fallback을 확인하며, 지원하지 않는 stylesheet 재사용을 성공으로 간주하지 않는다.
- **순서 경계:** 현재 `CalcLayoutTree`는 Taffy 노드를 postorder 위치로 만들고 `Layout::order`를 초기화한다. Flex algorithm은 일부 자식의 출력 order를 다시 지정한다. 현재 runtime paint snapshot은 별도로 HostDocument preorder를 순회해 paint rank를 만든다. 이 세 순서를 CSS `order` 값과 혼동하지 않는다. 필요한 계산 child 순서는 flex container의 형제 범위에서만 안정 정렬하고, paint list는 해당 형제 item의 order-modified 순서를 재귀적으로 반영하되 nested subtree를 부모 밖으로 끌어내지 않는다. HostDocument는 원본 순서를 보존한다.
- **앱 실행:** 각 구현 단계가 Rust·고정 Chromium oracle을 통과한 뒤 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8 fixture를 실행한다. runtime node frame, WGPU 제출과 오류 로그를 보존한다. Android software backend 및 simulator 실행을 실기기·하드웨어 GPU 성능 증거로 확대하지 않는다.
- **오류:** cascade에서 유효하지 않은 CSS declaration은 CSS cascade 규칙으로 무시되는지 Chromium과 비교한다. adapter 직접 입력이나 지원 밖 상호작용은 기본값으로 성공시키지 말고 node/property 문맥을 포함해 거부한다. 실패 뒤 이전 frame이 새 revision의 결과로 노출되면 안 된다.
- **WPT 교차표:** 각 지원 값과 의미 상호작용을 upstream Web Platform Tests의 Flexbox/Baseline test path에 대응한다. inventory에는 WPT git commit과 경로를 고정하고, 해당 profile로 실행할 수 없는 원본 case는 제외 이유와 로컬 fixture 대응 case를 적는다. 자체 fixture에 대응 기준이 없는 지원 문법은 완료 처리하지 않는다.

고정 reference 파일은 `tests/fixtures/css/c10/flex-order-alignment.html`, `tests/fixtures/css/c10/flex-order-alignment-inventory.json`, `tools/css-reference/capture-c10-3-flex-order-alignment.mjs`, `tools/css-reference/c10-3-flex-order-alignment.test.mjs`, `tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json`으로 둔다. 앱 실행 fixture는 `tests/fixtures/css/c10/runtime-flex-order-alignment.js`다. inventory에는 대응 WPT revision/path와 제외 이유를 기록한다. capture 전용 case 수는 Chromium 결과와 독립 불변식을 대조한 뒤 확정한다.

## C10.3.1 · 방향 역전

### 포함 범위

- `row-reverse`, `column-reverse`, `wrap-reverse`의 computed value와 geometry를 각기 분리해 관찰한다. `row-reverse wrap-reverse`, `column-reverse wrap-reverse` 조합도 포함한다.
- row와 column, 단일 line과 여러 line, 빈 컨테이너와 자식 하나, 정확히 맞는 줄바꿈과 다음 항목 하나가 넘치는 경계를 사용한다. 기존 C10.1·C10.2 fixture와 같은 고정 크기·빈 비텍스트 item을 우선 사용해 intrinsic measurement를 끌어들이지 않는다.
- `flex-flow` shorthand에서 `row-reverse wrap`과 `wrap row-reverse`처럼 허용된 토큰 순서 둘 다 reverse longhand를 올바르게 전개하는지, 뒤따르는 longhand가 cascade 승자가 되는지 확인한다.
- `row-reverse`/`column-reverse`는 주축 시작과 끝을 바꾸고, `wrap-reverse`는 줄이 쌓이는 교차축 방향을 바꾼다. 이 값만으로 DOM child vector, source traversal 또는 기본 paint 순서를 역전시키지 않는다. 이 동작을 `order`의 재정렬과 혼동하지 않는다.
- `justify-content:flex-start|flex-end` 및 `align-content:flex-start|flex-end`는 물리 좌표 기준만으로 비교하지 않고 computed axis와 해당 축의 start/end를 함께 기록한다. reverse 조합에서 start/end가 뒤집히는지 명시적으로 확인한다.

### 완료 조건

- row/column 각각의 역방향, wrap-reverse 및 조합별 Chromium frame이 node별 허용치 안에 든다. 여러 line에서 item 수집 순서는 유지되고 공간상 line 순서만 cross-start에 따라 달라진다.
- nested Flex container 하나를 포함해 자식별 방향이 부모의 방향을 잘못 상속하거나 적용하지 않는지 확인한다. display none child는 line 수집 및 paint 목록에서 제외한다.
- 여섯 runtime Flex profile에서 typed mapping을 확인하고 기본 V8 앱 path에서 실제 frame을 비교한다. 축 reverse를 order 처리로 흉내 내는 구현은 통과로 보지 않는다.

## C10.3.2 · `order`, layout 순서와 그리기 순서

### 포함 범위

- `order`의 초기값 0, 음수·양수, 동일값 tie, source order와 다른 값의 혼합, 큰 값 경계, 소수처럼 문법상 invalid한 선언, 스타일 변경 전후를 비교한다. 수치 범위는 Stylo computed representation과 Chromium 결과를 먼저 조사해 고정하고, 임의 Rust 정수 변환으로 범위를 줄이지 않는다.
- `order`는 상속되지 않으며 flex item의 line collection과 layout 순서를 바꾼다. 이 단계의 fixture는 in-flow element child로 한정하고, 텍스트/anonymous flex item은 C15에서 연결한다. `display:none`과 out-of-flow absolute child는 flex line 계산에서 제외한다. 고정 Flexbox 표준에서는 absolute child가 flex item의 paint order와 비교될 때 `order:0`으로 취급되므로, 이 혼합 paint 순서와 static-position alignment는 C10.3.5/C12에서 따로 닫고 그전에는 `order` 전체를 완료 처리하지 않는다.
- line collection, basis/grow/shrink 계산과 item frame이 order-modified document order를 따른다. 값이 같은 item은 원본 문서 순서를 유지한다. 전체 HostDocument 및 사용자 JS의 `childNodes`/`children` 결과는 원본 source order다.
- W3C가 정한 in-flow Flex paint 순서와 layout 순서를 분리한다. `order`가 달라진 겹침 case에서는 runtime paint-list 순서 또는 겹쳐진 불투명 box 결과로 order-modified paint 순서를 확인한다. sibling flex item을 정렬할 때 각 item의 descendant subtree를 함께 유지한다. 서로 다른 nested container의 item을 한 global `order` 값으로 섞지 않는다. `row-reverse`와 `column-reverse`는 이 paint rank를 대신 바꾸지 않는다. absolute child의 `order:0` 상호 순서, static-position alignment, `z-index`와 stacking context는 C10.3.5와 C12·C22 계약에 맡긴다.
- 현재 `RuntimeRenderSnapshot`에는 일반 runtime hit-test API가 없고 S05 입력 이벤트 전달도 미구현이다. 이 단계에서는 frame·paint entry가 안정된 NodeId와 연결되는 것까지만 확인한다. CSS paint order를 실제 플랫폼 pointer target으로 사용한다고 주장하지 않는다. S05가 구현될 때 overlap target 선택을 별도 계약·검증으로 연결한다.
- 시각 `order`는 source/speech/순차 키보드 탐색 순서를 재정렬하지 않는다. HostDocument source order 불변식을 테스트한다. 네이티브 접근성 트리는 아직 별도 구현 범위이므로 접근성 통합이 완료되었다고 주장하지 않는다.

### 완료 조건

- 같은 order tie는 입력 tree order로 안정적이며, signed order가 작은 item부터 배치·그려진다. 변화 후 layout, paint와 style revision이 하나의 일관된 결과로 게시된다.
- order는 CSS non-flex context의 일반 자식·다른 subtree ordering을 바꾸지 않고, 다른 flex container의 item과 비교하지 않는다. 계산 순서 정렬용 vector를 원본 DOM 자식 목록에 다시 써 넣지 않는다.
- 최신 style이 실패하면 과거 paint order를 새 revision frame으로 노출하지 않는다. 오류에는 node/property가 포함된다.

## C10.3.3 · `align-self`와 `align-content`

### `align-self`

- 첫 값 집합은 `auto`, `flex-start`, `flex-end`, `center`, `stretch`다. 현재 지원 `align-items` 값도 기존 C04.3 계약에 따라 함께 고정한다.
- `align-self`는 상속되지 않으며 flex item에서만 geometry에 영향을 준다. `auto`의 computed value가 `auto`로 보존되는지와 used alignment가 부모의 `align-items`에 따라 정해지는지를 따로 확인한다. 기본·명시 item cross size, stretch의 auto cross size, nonzero cross size, min/max clamp, padding/border, cross-axis auto margin을 각각 분리한다. CSS에서 cross-axis auto margin이 우선해 align-self를 무효화하는 경우를 포함한다.
- 이 substep에서 `normal`, `start`, `end`, `self-start`, `self-end`, overflow-position(`safe`/`unsafe`), anchor-specific alignment는 지원하지 않는다. `baseline`/`first baseline`/`last baseline`은 C10.3.4에서만 다룬다. stylesheet의 잘못된 선언 무시와 유효하지만 typed adapter가 지원하지 않는 computed value의 명시적 거부를 구분한다.
- row와 column에서 실제 물리 cross axis가 서로 달라지는 paired case를 둔다. align-items가 자식 안쪽 정렬이 아닌 컨테이너 자식에 대한 기본값이며, align-self가 개별 item override임을 확인한다.

### `align-content`

- 첫 값 집합은 `normal`, `stretch`, `flex-start`, `flex-end`, `center`, `space-between`, `space-around`, `space-evenly`다. `normal`은 computed CSSOM에서 키워드를 보존하고 used geometry가 `stretch`와 같은지 pinned Chromium으로 고정한다. `safe`/`unsafe`, `start`/`end`, 물리 `left`/`right` 값은 이 substep에 포함하지 않는다. `baseline`/`first baseline`/`last baseline`은 C10.3.4 소유다. Taffy enum 이름을 CSS computed value로 가정하지 않는다.
- `flex-wrap:nowrap`은 단일-line container이므로 `align-content`가 line 위치·크기를 바꾸지 않아야 한다. `flex-wrap:wrap`은 line이 실제 하나만 만들어져도 multi-line container이므로 `align-content`가 line에 미치는 효과를 별도 확인한다. 이를 “실제 line 개수”만으로 분기하지 않는다.
- 실제 여러 line일 때 cross-axis free space가 0, 양수, 음수인 경우를 다룬다. `stretch`, 각 분배 값, overflow fallback과 `row-gap`/`column-gap`이 함께 있을 때의 사용 간격을 기록한다. `wrap-reverse`와 조합해 cross-start/end를 바꾼다.
- `align-content:stretch`가 line cross size를 늘리고 `align-items`/`align-self:stretch` item size를 다시 바꾸는 연쇄를 별도 case로 둔다. line 위치만 맞는 결과를 완료로 보지 않는다.

### 완료 조건

- 각 typed style 값의 inline full/incremental 결과, author stylesheet full cascade, 여섯 runtime Flex profile과 실제 V8 path의 frame을 고정 Chromium과 비교한다.
- item 정렬과 line 정렬을 독립적으로 검증한다. `nowrap`과 `wrap`의 의미를 혼동하지 않고, single actual line `wrap` case에서 align-content 동작이 유지된다.
- cross-axis margin, explicit cross size, min/max, padding/border 중 fixture가 사용한 값은 결과의 설명 가능한 입력으로 남긴다. 기준을 넘는 차이는 평균이나 pixel screenshot으로 숨기지 않는다.

## C10.3.4 · baseline

- `align-items:baseline`, `align-self:baseline`과 기본 first-baseline 동작을 지원 후보로 둔다. `last baseline`, vertical writing mode, fragmented layout, inline/text descendants는 별도의 선행 계약이 없으면 지원 완료에 포함하지 않는다.
- 먼저 높이가 서로 다른 빈 fixed-size flex item의 CSS가 요구하는 synthesized baseline을 비교한다. Taffy의 baseline 입력·fallback이 Chromium의 border-box/child baseline synthesis와 같은지 확인하고, Taffy 동작에 대한 단위 비교만으로 완료 처리하지 않는다.
- parent flex container가 다른 Flex container 안에서 baseline-aligned item으로 사용되는 경우, `order`와 reverse/wrap-reverse 후 어느 line/item이 first baseline을 제공하는지 함께 검사한다.
- 실제 텍스트 baseline과 글꼴·script·line box는 C15의 text layout/measurement 결과를 받아야 한다. C15가 완료되기 전에는 text baseline을 구현했다고 주장하지 않고 C10.3.4를 미완료로 유지한다. baseline fallback을 임의로 bottom edge 또는 font ascent로 고정하지 않는다.
- `baseline`은 first-baseline alignment의 별칭으로 처리한다. `align-self`/`align-items`의 `first baseline`과 `last baseline`, 그리고 row wrap의 `align-content:first baseline`/`last baseline` line 정렬을 별도 값으로 다룬다. Column Flex에서 cross axis가 block axis와 평행하지 않아 baseline content-alignment가 적용되지 않는 경계도 고정한다. 모든 텍스트 기반 first/last baseline 결과는 C15까지 미완료다.

## fixture와 회귀 축

각 substep의 capture case에는 다음 교차 조건 중 그 단계에 필요한 항목을 포함한다. 모든 조합의 곱을 무작정 실행하지 않고 pairwise 표를 만들며, 표준 규칙이 상호작용을 요구하는 조합은 별도 case로 지정한다.

다음은 구현 전 비교를 시작할 최소 seed case와 손계산 geometry다. 정식 reference 값은 HTML/CSS를 확정한 뒤 Chromium으로 캡처하고 아래 불변식과 대조한다. 차이가 나면 기대값을 그대로 강제하지 말고 표준·fixture·브라우저 버전을 조사한다.

| Case | 입력 요약 | 독립 예상·관찰 |
| --- | --- | --- |
| `row-reverse-three` | row reverse, 컨테이너 `300×20`, 세 item `100×20`, grow/shrink 0 | 원본 첫째/둘째/셋째의 x는 `200/100/0`; HostDocument 자식 순서는 그대로다. |
| `column-reverse-three` | column reverse, 컨테이너 `100×90`, 세 item `100×30` | 원본 첫째/둘째/셋째의 y는 `60/30/0`. |
| `wrap-reverse-two-lines` | row wrap reverse, 컨테이너 `220×40`, 세 item `100×20`, gap 0 | 원본 첫째·둘째는 아래 line y=20, 셋째는 위 line y=0; line 안 item 순서는 유지된다. |
| `order-three-items` | row, 컨테이너 `300×20`, source 순서의 item order `2/-1/0`, 각 `100×20` | source 첫째/둘째/셋째의 x는 `200/0/100`; render entry는 같은 NodeId를 가진 order-modified item 순서다. |
| `order-tie-stability` | source order A/B/C, order `0/-1/0`, 각 폭 100 | 배치 순서는 B/A/C이며 A와 C의 tie는 source order를 유지한다. |
| `order-invalid-fraction` | source A/B/C, 각 폭 50; B는 `order:4` 다음 `order:1.5`, C는 `order:1.5`만 선언 | invalid fractional declaration은 무시된다. computed order는 A=0/B=4/C=0, source item x는 A=0/B=100/C=50이며 computed CSS 값도 기록한다. |
| `align-self-fixed-size` | row 컨테이너 높이 100, 높이 20인 item에 flex-start/center/flex-end | y는 `0/40/80`; 고정 cross size의 stretch는 height 20을 강제로 늘리지 않는다. |
| `align-self-auto-and-stretch` | parent `align-items:center`; child `align-self:auto`; 별도 auto cross-size child에 stretch | auto의 computed value는 `auto`로 남고 used alignment/geometry는 center와 같으며, stretch child는 제한 조건이 없을 때 line의 사용 cross size를 채운다. |
| `align-content-wrap-single-line` | definite 높이 100의 `flex-wrap:wrap`, 실제 line 하나, item height 20 | `flex-start`·`space-between` y=0, `center`·`space-around`·`space-evenly` y=40, `flex-end` y=80, `stretch`와 `normal`은 line cross size 100인 결과를 Chrome과 비교한다. `nowrap` 대조와 구별한다. |
| `align-content-wrap-two-lines` | `wrap`, 실제 두 line의 높이 20, 컨테이너 높이 100, gap 0 | line 시작 y는 `flex-start=0/20`, `flex-end=60/80`, `center=30/50`, `space-between=0/80`, `space-around=15/65`, `space-evenly=20/60`, `stretch=0/50`이며 stretch line 높이는 50이다. `normal`의 결과도 Chrome과 비교한다. |
| `baseline-empty-boxes` | 서로 다른 높이의 empty fixed-size item, baseline alignment | node별 frame과 baseline fallback을 Chromium/Taffy에서 직접 비교한다. text node는 넣지 않는다. |
| `baseline-content-lines` | row wrap 두 line, line마다 높이가 다른 empty fixed-size item, `align-content:first baseline`/`last baseline` | computed keyword, line frame과 baseline fallback을 비교한다. 같은 fixture의 column wrap 대조는 CSS Align baseline content-alignment 축 제한을 확인한다. |

| 축 | 고정·변형 값 | 잡을 오류 |
| --- | --- | --- |
| 주축과 교차축 | row, column; reverse 조합 | 주축·교차축 또는 gap 축의 물리 투영 오류 |
| wrapping | nowrap, wrap 단일 line, wrap 여러 line, wrap-reverse | 실제 줄 수와 multi-line container 구분 실패 |
| 순서 | source order, 음수/0/양수 order, tie, style mutation | unstable sort, DOM mutation, stale order snapshot |
| 정렬 | auto/explicit item alignment, line alignment 분배 값 | 부모 기본 정렬, auto margin, stretch·clamp 누락 |
| cascade | 초기값, shorthand, longhand override, invalid declaration, stylesheet | computed style과 typed projection 불일치 |
| 자식 종류 | visible element, display none, nested flex | item 참여 조건 또는 NodeId 매핑 오류 |
| 환경 | DPR 1/2, fixed viewport, simulator profile | 반올림·환경 차이를 CSS geometry 오차로 가림 |

HTML reference는 기본 empty box와 명시 치수를 사용한다. 각 단계에서 텍스트 또는 replaced 요소가 꼭 필요한 경우 C15/C14를 의존성으로 기록하고, 해당 단계의 완료를 그 입력까지 넓히지 않는다. 캡처 reference에는 전체 DOM node 식별자, 계산 속성, frame, 필요 시 paint 순서를 저장한다. Runtime report에는 CSS px 좌표를 node ID와 함께 남긴다.

## 단계별 작업 순서

1. 이 계획을 기준으로 각 하위 단계의 fixture inventory, 대응 WPT path/revision과 사전 판정 기준을 작성한다. 이후 고정 Chromium에서 reference를 한 번 만들고 fixture·capture·browser digest를 저장한다.
2. 해당 단계의 현행 경로를 Stylo computed values → profile allowlist → `ComputedStyleSnapshot` → `LayoutStyle` → Taffy style/tree → frame output → 현재 paint list까지 추적한다. S05 입력 target 경로가 이미 있는지도 확인하되, 미구현이면 계획 범위에서 구현 완료를 가정하지 않는다.
3. Chromium reference와 독립 손계산/순서 불변식을 먼저 맞춘다. fixture 입력이나 기대값이 어긋나면 코드를 바꾸기 전에 원인을 해결한다. Chromium 값이 의심되면 reference를 덮어쓰지 말고 버전·기준 환경·CSS 해석을 조사한다.
4. 해당 substep만 구현하고 실패 입력을 구체 오류로 거부한다. CSS source cascade가 잘못된 선언을 무시하는 동작과 adapter가 표현할 수 없는 computed value를 거부하는 동작을 구별한다.
5. Rust 단위/통합 비교, 고정 Chromium geometry·paint oracle, V8 runtime report, Android/iOS Simulator 실행을 같은 하위 단계에 대해 수행한다. 실제 테스트를 실행하지 않았다면 PR과 증거에서 완료라고 하지 않는다.
6. 구현 PR에 내부 인터페이스 명세(입출력, typed 값, profile, 오류·revision, DOM/paint 순서, 예제), `spec/STATUS.md`, fixture 설명, 구현 review evidence를 함께 넣는다. 공개 CSS 지원 표를 바꾸는 단계면 공개 spec도 같이 갱신한다. 내부 계약 숫자 버전은 `0.1.0`으로 유지한다.
7. 구현 뒤 계획 검토 표를 재사용하지 않고 실제 변경 코드·fixture·오류·runtime path에 대해 새로 20개의 서로 다른 실패 관점을 검토한다. 발견 결함을 수정하고 해당 관점을 다시 확인한다.

## 범위 밖과 순서 제약

- RTL, `writing-mode` 변경, vertical text, `direction`과 reverse의 전체 상호작용은 C17 방향성 계약이 준비되기 전에는 이 계획의 지원 범위 밖이다. LTR fixture만으로 RTL 호환을 암시하지 않는다.
- `inline-flex`, `visibility:collapse`, pseudo-element와 out-of-flow child는 inline formatting·collapsed item·positioning 선행 계약이 없으므로 이 계획의 성공 입력에 포함하지 않는다. 이를 `display:flex` 또는 일반 `display:none` 동작으로 조용히 치환하지 않는다.
- anonymous flex items, text/replaced intrinsic sizes, text baseline은 각각 C15/C14 의존성을 따른다. 현재 element-only box fixture 결과를 일반 HTML 자식 전체로 확대하지 않는다.
- absolute/fixed child의 Flex paint order, stacking context·`z-index`, opacity/clip/transform과 hit-testing 연계는 C12/C13/C22를 소유자로 둔다. 이 지원 없이는 겹침 paint case가 실제로 사용할 수 있는 최소 배경 페인트 조건을 별도로 고정한다.
- Flex 전체를 거부하는 fallback은 허용하지 않는다. 이미 완료된 C10.1·C10.2의 지원 조합을 보존하고, 새 속성 입력은 대상 runtime profile에서 단계적으로 연결한다. 다른 profile에 우연히 새지 않도록 회귀한다.
- 이 문서는 구현 계획이며 공개 지원 선언이나 첫 릴리스 범위 결정이 아니다. 구현 완료와 사용자 노출은 별도 상태·API 명세 승인 절차를 따른다.

## 기준 자료

- [W3C CSS Flexible Box Layout Module Level 1, 2025-10-14 Candidate Recommendation Draft](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/)의 [§4.3 z-order](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#painting), [§5.1 direction](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#flex-direction-property), [§5.2 wrapping](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#flex-wrap-property), [§5.4 ordering and accessibility](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#order-property), [§8.3 item alignment](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#align-items-property), [§8.4 line alignment](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#align-content-property), [§8.5 baselines](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#flex-baselines), [§9 algorithm](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#layout-algorithm). 여기에는 absolute child가 flex item paint order와 비교될 때 `order:0`으로 간주되는 조건이 포함된다.
- [W3C CSS Box Alignment Module Level 3, 2026-10-08 Working Draft](https://www.w3.org/TR/2026/WD-css-align-3-20261008/)의 flex container 규칙을 함께 사용한다. 이 문서는 Flexbox가 열거하는 값보다 `normal`, `space-evenly`, overflow-position 및 first/last baseline value를 확장한다. Working Draft이며 기준 결과를 대체하지 않는다. 구현 시작 때 두 표준의 상태를 다시 확인하고 차이를 기록한다.
- [저장소의 Taffy 0.14.0 source](../Cargo.lock)와 `crates/spinon-layout/src/style.rs`, `taffy_style.rs`, `calc_tree.rs`. Taffy는 후보 계산기이며 CSS compatibility oracle가 아니다.
- [C10.1 wrap 계약](../spec/internal/0048-c10-flex-wrap.md), [C10.2 distribution 계약](../spec/internal/0049-c10-flex-distribution.md), [CSS 적합성 기준](../spec/0001-conformance.md).
