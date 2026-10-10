# C09 · Block formatting 계획

## 목표와 병합 순서

현재 모바일 우선 Runtime CSS 경로에 Chromium과 일치하는 Block formatting 동작을 단계적으로 연결한다. CSS 선언은 Stylo가 계산하고, 레이아웃 adapter와 Taffy는 typed style·트리·포함 블록을 받아 geometry를 계산한다. 지원 선언을 조용히 버리거나 Block 기본값으로 바꾸지 않는다.

C08.1 구현 PR [#105](https://github.com/ohah/spinon/pull/105)은 `main`에 리베이스 병합됐다. C09 제품 코드 작업은 이 계획·reference 변경이 `main`에 병합된 뒤 C09.1부터 시작한다. C08.1의 기본 Block 흐름은 margin collapse나 formatting context 구현을 대신하지 않는다.

| 하위 ID | 동작 범위 | 순서·선행 조건 |
| --- | --- | --- |
| C09.1 | 일반 in-flow Block 크기 방정식, 수직 흐름, containing block | C08.1과 C09 계획/reference 병합 뒤 첫 구현 |
| C09.2 | 수직 margin collapse와 signed margin strut | C09.1 뒤 구현 |
| C09.3 | `display: flow-root`와 Block formatting context 경계 | C09.2 뒤 구현 |
| C09.4 | shrink-to-fit이 필요한 float·inline-block·absolute 문맥 | C12 positioning, C14 intrinsic sizing, C15 inline/text 측정, C26 float 중 해당 문맥의 선행 구현이 된 뒤 연결. 이 항목이 끝날 때까지 C09 상위는 미완료 |

C09.4는 단순히 Taffy의 `fit-content` 값으로 바꾸어 완료 처리하지 않는다. CSS 2.1의 shrink-to-fit 사용처는 서로 다른 formatting context에 있으므로 각 문맥과 intrinsic width 측정이 준비된 뒤 Chromium 기준을 별도로 닫는다.

구현 전 Chromium reference는 [23개 fixture·76개 node JSON](../tests/fixtures/css/references/c09-block-formatting-v1.json)으로 고정했다. [사전 비교 기록](../spec/internal/evidence/c09-block-formatting-precomparison-2026-10-10.md)에 입력 digest·환경·관찰 범위를 남겼다. 이는 Chromium oracle 준비 결과이며 C09 Rust/runtime 구현이나 Android·iOS 검증 결과가 아니다.

## 구현 경계

### C09.1 · 일반 Block 흐름과 containing block

- 성공 profile은 한 HostDocument 요소 subtree의 수평 쓰기 모드·LTR·in-flow·비-replaced Block box다. HostRoot 플랫폼 surface와 CSS box를 같은 노드로 취급하지 않는다. layout adapter는 viewport를 containing block으로 제공하되, 그 보조 입력은 DOM node·paint box·공개 element로 노출하지 않는다. 구현 형태는 기존 tree와 호환되면 합성 Taffy root 또는 동등한 방식으로 선택할 수 있다.
- 자식의 containing block은 CSS 규칙에 따라 상위 Block content box를 기준으로 한다. viewport/초기 containing block, 중첩 definite·indefinite 크기, content-box·border-box, C06 percentage basis와 C07 box model을 교차 검증한다. C06 단위 환산이나 C07 상자 계산을 다시 구현하지 않는다.
- Block width 방정식에서 `auto` width 및 좌우 `auto` margin 분배, 모든 값이 지정된 over-constrained LTR 경우의 우측 margin 처리, 음수·분수 margin, min/max 제약, 일반 자식 순서와 auto height를 검증한다. vertical `auto` margin은 CSS 사용값을 확인한다. LTR만 이 하위 항목의 성공 범위다. RTL·bidi·vertical writing mode는 C17에 둔다.
- `display:none` subtree는 margin strut·배치·paint에 참여하지 않고 기존 C04.9 frame 계약대로 DOM preorder 기록은 유지하되 subtree 모든 frame을 0으로 낸다. 보이는 텍스트, replaced element, inline/table/flex/grid item은 이 하위 항목에서 측정하지 않으며 기존 오류·별도 profile 경계를 유지한다.
- C04.8 의미상 HostRoot 직속 Element는 CSS `:root`나 `documentElement`가 아니라 독립 fragment cascade root다. C09.1 성공 범위는 HostRoot 직속 Element 하나로 한정하고 여러 root와 `body`·`documentElement` 의미는 제외한다. Chrome reference에서는 `html, body { margin:0; padding:0 }`로 초기 여백을 없앤 뒤 viewport 크기의 `display:flow-root` wrapper 안에 일반 앱 Block 하나를 둔다. case별 wrapper는 fixture에서 같은 viewport 원점에 절대 배치해 서로 겹치며, wrapper와 그 위치는 Spinon DOM·paint node가 아니다. runtime 비교기는 inventory에서 case 하나씩 materialize하고 wrapper를 제품 트리에 넣지 않은 채 viewport를 containing-block 입력으로 제공한다. wrapper는 HTML root/body 특례와 wrapper-child margin collapse를 차단하고 숨김 box의 원점 rectangle도 안정시키는 비교 장치다. 앱 Block의 자기 margin 및 viewport 기준 frame만 대조하며 실제 CSS root margin propagation은 적용하지 않는다.

### C09.2 · 수직 margin collapse

- 같은 Block formatting context에 참여하는 in-flow Block 사이의 세로 margin만 다룬다. 좌우 margin은 collapse하지 않는다.
- 인접 형제, 부모와 첫 자식, auto-height 부모와 마지막 자식, 높이·min-height가 0인 빈/self-collapsing Block을 각각 분리해 검증한다. 두 개씩만 대조하지 않고 세 개 이상이 하나의 strut으로 합쳐지는 경우도 포함한다.
- 양수 margin은 최댓값, 음수 margin은 절댓값 최댓값을 양수 최댓값에서 빼는 규칙을 검증한다. 양수 없이 음수만 있는 경우, 0, 분수 및 여러 단계로 통과하는 margin도 포함한다.
- collapse 인접성은 computed author `border-width` 문자열이 아니라 C07.2의 used border width와 padding을 사용해 판정한다. 예를 들어 `border-style:none`으로 used width가 0인 면은 보이는 border/padding barrier가 아니며, 유효한 nonzero used border는 barrier다. C09는 border 선 페인트를 요구하지 않는다.
- 부모·자식 사이의 조건은 면별 border/padding, 실제 used height, min-height, in-flow child 유무와 fragment 경계를 조합해 검사한다. HostRoot 직속 fragment Element에는 CSS parent가 없으므로 부모-자식 collapse를 만들어내지 않는다. text/inline line box가 collapse를 막는 사례는 C15와 함께 교차 검증하기 전까지 성공 입력으로 허용하지 않는다.
- CSS clearance 및 float가 개입하는 margin은 C26 전까지 지원 완료로 표시하지 않는다. `float`·`clear`가 성공 path에서 무시되거나 일반 Block으로 위장하지 않게 실패 경계를 둔다.

### C09.3 · formatting context와 `flow-root`

- `display:block`과 `display:flow-root`의 computed value와 layout 결과를 구분한다. `flow-root`는 내부 자식을 위한 독립 BFC를 만든다. 내부 자식 margin은 flow-root box와 collapse하지 않는다.
- flow-root box의 내부 경계와 외부 margin 관계를 구분한다. 내부 child와의 margin collapse는 막되, 그 box 자체의 위·아래 margin이 비-BFC 부모의 첫째/마지막 자식 및 상위 BFC의 이전·다음 형제와 collapse할 수 있는 조건을 각각 검증한다. fragment root에는 CSS parent가 없으므로 부모 경계 fixture는 중첩된 flow-root에 둔다.
- C09에서 명시적으로 구현하는 formatting-context 생성 값은 `flow-root`다. `overflow`의 BFC 효과는 C13, positioned box는 C12, flex/grid는 C10/C11, float와 table은 C26, inline-block은 C15가 소유한다. 이 값들이 C09 profile에 전달되면 조용히 `block`으로 바꾸지 말고 지원 profile 밖으로 거부한다. 각 소유 항목이 구현될 때 C09 collapse·BFC fixture를 교차 실행한다.
- BFC 경계가 margin collapse, 외부 float 상호작용, 내부 float 포함 동작을 모두 자동 구현했다고 주장하지 않는다. 외부 float와 float containment는 C26 fixture에서 추가로 닫는다.

### C09.4 · shrink-to-fit

- 대상 문맥을 분리한다: float non-replaced auto width(C26), inline-block non-replaced auto width(C15), absolute non-replaced의 CSS 2.1 §10.3.7 cases 1·3처럼 inset/width 조합이 실제 shrink-to-fit을 선택하는 경우(C12). absolute의 모든 `width:auto`가 shrink-to-fit인 것은 아니다. 각 문맥은 실제 사용처가 구현되는 시점에 해당 profile의 입력·오류 계약을 먼저 고정한다.
- min-content/preferred minimum width와 max-content/preferred width는 C14/C15의 intrinsic measurement 결과를 사용한다. width 식은 CSS2.1의 `min(max(preferred minimum width, available width), preferred width)` 관계를 기본 설명으로 기록하되, CSS2.1이 정확한 알고리즘을 고정하지 않는 부분은 pinned Chromium 결과로 결정한다.
- shrink-to-fit을 `width:fit-content`와 같은 CSS value로 간주하지 않는다. 줄바꿈, available inline size, padding/border, percentage, min/max 및 각 사용처의 containing block을 함께 측정한다. 측정 함수를 제공하지 않거나 available width가 불명확하면 성공 결과를 만들지 않는다.
- Taffy 0.14.0의 float layout은 현재 Spinon dependency feature에서 꺼져 있다. feature flag를 켜거나 Taffy의 `fit-content`를 매핑하는 것만으로 C09.4가 통과했다고 보지 않는다. 수치 차이가 있으면 원인을 기록하고 adapter 또는 독립 계산의 수정으로 Chrome 허용치에 들어온 뒤 지원한다.

## 명시적 제외와 교차 소유

| 기능 | C09에서의 처리 | 소유 항목 |
| --- | --- | --- |
| float·clear·문자 주변 감싸기 | 지원 완료 주장 금지, C09 margin fixture에 clearance를 넣지 않음 | C26 + C15 |
| absolute·fixed·sticky containing block | positioned containing block 미지원 | C12 |
| inline·inline-block 및 줄 상자·글리프 측정 | visible text와 inline formatting은 성공 경로 밖 | C15·C16 |
| min-content·max-content·replaced intrinsic size | shrink-to-fit 입력으로 임의 상수 사용 금지 | C14·C18 |
| overflow·scroll·clip이 만드는 formatting context | overflow semantics 없이 BFC flag만 추정하지 않음 | C13 |
| Flex/Grid/Table와 고급 writing mode | 이 Block profile에서 조용히 변환하지 않음 | C10·C11·C17·C26 |
| border 선, 색, radius 등 paint | geometry·collapse barrier 입력과 시각 paint를 구분 | C22 |
| 전체 HTML/CSS 적합성·공개 API 지원 | 이 내부 profile만으로 전체 지원을 선언하지 않음 | C30 및 공개 계약 검토 |

## 비교 기준과 통과 판정

- Oracle은 기존 CSS 기준과 동일한 고정 Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`로 한다. 기준 HTML, inventory, capture runner, Chrome 실행 파일 hash, viewport `320×240` CSS px, `en-US`, `UTC`, light, DPR 1·2의 provenance를 [reference JSON](../tests/fixtures/css/references/c09-block-formatting-v1.json)에 고정한다. 현재 고정 입력은 23개 case·76개 app node다. capture 입력이나 digest가 바뀌면 기존 reference를 덮어쓰지 않고 새 기준을 검토한다.
- 기준 CSS는 margin 값·computed value, display, box-sizing, width/height/min/max, padding·used border, 부모/자식 관계를 inventory 순서대로 관찰한다. collapse 결과는 직접 관찰 가능한 별도 CSS property가 아니므로 node별 `getBoundingClientRect()`의 x/y/width/height 및 주변 sibling의 상대 위치를 판정한다. computed 값과 used geometry를 혼합하지 않는다.
- fixture 원본 HTML과 inventory는 Chromium reference 및 Rust/runtime 비교의 공통 입력이다. 단, runtime은 inventory의 case 하나씩 materialize해 그 case의 단일 app root를 HostRoot에 연결하고 viewport를 별도 containing-block 입력으로 전달한다. wrapper 및 다른 겹친 case를 제품 트리에 포함하지 않는다. node ID·parent ID·DOM preorder가 명시되며 capture가 누락·중복 node, 실행 파일/버전/revision/hash 불일치, viewport·DPR 변화를 감지하면 결과를 실패 처리한다. reference를 테스트 중 자동 갱신하지 않는다.
- 모든 지원 node의 frame field별 최대 절대 오차는 `0.5 CSS px` 이하다. 평균 오차로 개별 실패를 가리지 않는다. DPR 1과 2의 CSS px computed value·geometry는 같아야 한다. geometry 테스트는 pixel screenshot 색상이나 장치 pixel로 대신하지 않는다.
- 계산 실패·미지원 값·Stylo 진단·stale source/style/environment revision은 node/property를 식별하는 오류여야 한다. Taffy 호출 뒤 partial tree, 이전 frame과 새 frame의 혼합, 누락 margin을 0으로 대체하는 결과를 성공으로 반환하지 않는다.
- 각 하위 항목은 C09 profile의 실제 V8 JavaScript 경로를 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행한다. 두 플랫폼 모두 동일 fixture의 node별 CSS frame/revision을 로그로 수집해 고정 Chromium reference와 비교하고 화면 캡처를 보조 근거로 남긴다. box count나 screenshot만으로 layout 정확도를 승인하지 않는다. 실기기와 hardware GPU 성능은 주장하지 않는다.
- 하위 항목별 전체 관련 Rust workspace test, CSS reference suite, `cargo fmt`, Clippy 및 플랫폼 빌드를 실행한다. 실패·오류 후 새 revision으로 재계산했을 때 이전 layout/scene이 재사용되지 않는지 확인한다.

## 구현 단계

1. C08.1 PR #105는 병합됐다. 이 계획·reference 변경이 `main`에 병합된 뒤 최신 `main`에서 작업 branch를 만든다. 그 전에는 C09 제품 코드를 추가하지 않는다.
2. 완료된 [C09 HTML·inventory·Chromium capture](../tests/fixtures/css/c09/block-formatting.html)와 [`css:reference:c09-block-formatting`](../tools/css-reference/capture-c09-block-formatting.mjs)을 유지한다. [고정 JSON reference](../tests/fixtures/css/references/c09-block-formatting-v1.json) 및 [검증 테스트](../tools/css-reference/c09-block-formatting.test.mjs)가 전달하는 독립 oracle·환경·입력 digest를 runtime 구현 전에 확인한다.
3. 계획 자체를 기능 코드와 분리해 20개의 서로 다른 실패 관점으로 검토하고, 지적을 반영한 최종 계획 hash와 검토 결과를 남긴다.
4. 내부 계약 ID `0047`과 C09.1–C09.4 상태 대장을 추가한다. 공개 API가 아닌 내부 profile임을 적고 숫자 버전은 `0.1.0`으로 고정한다.
5. C09.1에서 Stylo typed layout input, normal-flow containing block, Block width equation과 Taffy projection을 연결한다. box-sizing·percentage·border/min/max 회귀를 유지하고 실제 V8 Android/iOS 경로의 node별 frame을 기준에 대조한다.
6. C09.2에서 same-BFC margin strut과 collapse eligibility를 연결한다. multi-margin, 음수 조합, parent/child, empty block, used border/padding barrier와 root boundary를 개별 fixture로 대조한다.
7. C09.3에서 `flow-root` profile과 BFC 경계를 연결한다. 내부 child margin과 flow-root 외부 margin을 별도로 검증하고 다른 BFC 생성 값을 fail closed한다.
8. C12/C14/C15/C26의 해당 입력·측정기가 준비되면 C09.4의 float·inline-block·absolute 문맥을 각자 별도 profile/fixture로 통합한다. 그전까지 C09 상위의 shrink-to-fit 부분은 미완료로 남긴다.
9. 각 하위 구현 뒤 계획 검토와 겹치지 않는 새 실패 관점 20개로 코드·Chromium 차이·V8/FFI·Android/iOS runtime을 검토한다. 결함을 수정하면 영향받은 oracle과 양 플랫폼 실행을 다시 수행한다.
10. 계약·계획·STATUS·internal index·evidence index·PR 본문과 Tailscale 미리보기를 같은 PR 상태로 동기화한다. 문서 사이트 자동 배포는 실행하지 않는다.

## 완료 조건

- C09.1–C09.3 지원 property/value와 실패 경계가 `0047`에 명시되고 Chromium reference와 하위 항목별 실제 V8 Simulator 수치 비교가 통과한다.
- `flow-root` 내부와 외부 margin 관계, root/viewport 경계, used border-width barrier, 음수 포함 다중 margin collapse가 property별·node별로 입증된다.
- C09.4는 float·inline-block·absolute 사용처마다 해당 C12/C14/C15/C26 dependency가 준비된 뒤 Chromium 기준을 별도로 통과해야 한다. 한 문맥의 통과를 다른 문맥의 지원으로 확대하지 않는다.
- 화면·로그는 실행 경로를 보여주는 보조 evidence이고, 수치 결론은 고정 Chrome reference 대조 결과로만 낸다. simulator 결과를 실기기 또는 hardware GPU 결과로 표현하지 않는다.
- 각 하위 구현 및 PR은 계획 검토표와 다른 관점의 20개 적대 검토를 별도 기록한다. 하나라도 unresolved면 완료 체크를 하지 않는다.
- C09를 마쳐도 Flexbox·Grid·positioning·overflow·텍스트·float/table·전체 CSS 적합성 완료를 뜻하지 않는다.

## 공식 참고 자료

- [CSS 2.1 §8.3.1 · margin collapse](https://www.w3.org/TR/CSS21/box.html#collapsing-margins) — positive/negative strut, sibling·parent-child·self collapse 조건.
- [CSS 2.1 §9.4.1 · Block formatting contexts](https://www.w3.org/TR/CSS21/visuren.html#block-formatting) — in-flow Block 배치와 BFC 경계.
- [CSS 2.1 §10.1 · containing blocks](https://www.w3.org/TR/CSS21/visudet.html#containing-block-details), [§10.3.3 · in-flow Block width](https://www.w3.org/TR/CSS21/visudet.html#blockwidth).
- [CSS 2.1 §10.3.5 · float](https://www.w3.org/TR/CSS21/visudet.html#float-width), [§10.3.7 · absolute non-replaced](https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width), [§10.3.9 · inline-block](https://www.w3.org/TR/CSS21/visudet.html#inlineblock-width) — shrink-to-fit 사용처. 정확한 알고리즘이 정해지지 않은 부분은 Chromium oracle로 판정한다.
- [CSS Display Level 3 · `flow-root`](https://www.w3.org/TR/css-display-3/#flow-root) — Editor’s Draft이며, 구현 판정에서 고정 Chromium 결과를 대신하지 않는다.
- [Taffy 0.14.0 공식 API 문서](https://docs.rs/taffy/0.14.0/taffy/) · [latest 문서](https://docs.rs/taffy/latest/taffy/) — 계획 작성 시 최신 published docs도 0.14.0을 가리키며, 현재 Cargo dependency는 `=0.14.0`이다. Spinon은 기본 feature를 끄고 `float_layout`을 활성화하지 않았다.
