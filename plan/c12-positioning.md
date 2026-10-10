# C12 · 위치 지정과 쌓임 구현 계획

**상위:** [공식 상태 대장](../spec/STATUS.md) · [코어 아키텍처](../docs/architecture.md)

**현재 상태:** C12.1 PR #128 병합 완료 · C12.2 제한 Block/LTR 구현과 Android 실기기·iOS Simulator 검증 완료, [PR #130 검토 중](https://github.com/ohah/spinon/pull/130) · C12 상위 구현은 미완료
**목표:** `position`, 물리 inset, containing block, out-of-flow geometry, 고정·sticky 위치, stacking과 `z-index`를 Chromium 기준으로 단계별 연결한다. 계획 초안이나 Taffy 기능 목록을 제품 지원 판정으로 쓰지 않는다.

## 로드맵 순서와 완료 경계

C10.3.4는 PR #125로 병합했다. 다음 칸반 작업 C10.3.5는 C12의 positioned child 계약에 의존한다. 그래서 이 계획은 C12를 여러 독립 구현 단계로 나누고, C12.2에서 Block 절대 위치 기준을 닫은 뒤 C10.3.5에서 Flex 절대 자식의 static-position 정렬·`order: 0` paint 교차를 별도 PR로 구현한다. C12 전체는 고정·sticky·쌓임까지 완료되기 전까지 미완료다. C09.4의 absolute shrink-to-fit은 C12 외에도 C14·C15·C26 선행 구현 뒤 별도로 진행한다.

| 단계 | 소유 범위 | 선행 조건 | 완료 증거 |
| --- | --- | --- | --- |
| C12.1 | CSS `position` 및 물리 `top/right/bottom/left`·`inset` cascade typed snapshot, `static` normal flow, `relative` 시각 offset과 containing-block 자격 | 현재 C04 runtime profile과 C07/C09 상자·Block 입력 | Chrome computed style·좌표를 비교하고 지원 fixture를 Android 실기기와 iOS Simulator에서 같은 입력으로 실행 |
| C12.2 | Block formatting context의 `absolute`, nearest positioned ancestor, padding-edge containing block, percentage·auto inset과 positioned size 계산 | C12.1 · C09.1/C09.3 · C07.1/C07.2 | Chrome 기하·computed style 비교, 정적 경로와 실패 경계, 양 플랫폼 runtime fixture |
| C10.3.5 | Flex absolute child의 line 제외, static-position rectangle/`align-self`, paint에서 `order: 0` 상호 순서 | C12.1–C12.2 · C10.3.1–C10.3.4 | [C10.3 계획](c10-3-flex-order-alignment.md)의 별도 구현 PR과 Flex 전용 Chrome·GPU 증거 |
| C12.3 | 기본 fixed viewport containing block과 viewport resize 반영 | C12.1–C12.2 · 환경 revision 경로 | resize·DPR reference 비교, 새 revision frame만 표시 |
| C12.4 | paint order, positioned `z-index`, stacking context의 생성·중첩·격리 | C10.3.2/C10.3.5 · renderer paint-list 계약 | 중첩·겹침 픽셀과 paint-list 순서, Android 실기기·iOS Simulator 비교 |
| C12.5 | `sticky`와 nearest scrollport 제약 | C13 실제 scroll container·scroll offset·clip 계약 | 정지/경계/양방향 scroll 시 Chrome과 frame·clip 비교 |

각 구현 단계는 독립 PR과 내부 계약·고정 비교 기준을 가진다. C12 상태를 완료로 바꾸려면 위 다섯 C12 단계와 C10.3.5 교차 계약이 모두 닫혀야 한다. C12.1–C12.2의 유효한 비지원 조합은 기본 배치로 대체하지 않고 위치·속성·원인을 진단한다.

## 현재 경계와 구조 결정

- 현재 `LayoutStyle`에는 CSS position과 inset 값이 없고, style-to-layout projection도 이를 전달하지 않는다. `CalcLayoutTree`는 HostDocument 자식 관계를 직접 Taffy 자식 관계로 쓴다. 현재 runtime CSS 성공 profile은 위치 지정 속성을 계산하지 않는다.
- Cargo.lock은 Taffy `0.14.0`을 고정한다. 그 버전의 `Position`은 `Relative`와 `Absolute` 두 값이며 CSS의 `static`, `fixed`, `sticky`를 모델링하지 않는다. 기본값도 CSS `static`이 아닌 Taffy `Relative`다. 그러므로 CSS `static`을 Taffy 기본 `Relative`에 그대로 맡기면 안 된다.
- source/DOM parent, normal-flow formatting parent, absolute containing-block owner, Flex static-position owner, paint traversal은 별도 관계다. HostDocument 자식 벡터를 재부모화하거나 하나의 Taffy parent를 네 관계의 SSOT로 취급하지 않는다. Style-to-layout 입력에 필요한 owner ID를 보존하고, adapter가 geometry를 계산한 뒤 NodeId별 최종 frame과 paint 입력을 만든다.
- 레이아웃 요청의 root가 HostRoot보다 아래인 경우에도 실제 containing block을 찾는 데 필요한 ancestor computed style·border/padding·revision을 입력에 보존한다. owner가 계산 subtree 밖에 있어도 root viewport로 바꾸지 않으며, owner snapshot이 없으면 명시 오류를 낸다.
- C12는 CSS 값 파서나 CSS 전체 엔진을 새로 만들지 않는다. Stylo computed typed value를 입력으로 받고, 제한된 값은 Rust layout/paint 계약으로 전달한다. Stylo가 구문상 받아들였더라도 현재 runtime profile에서 의미를 보존할 수 없는 조합은 commit 전에 실패한다.
- C12.1–C12.2는 물리 방향 inset과 `horizontal-tb`·LTR geometry를 우선 구현한다. CSS 논리 inset, RTL·다른 writing mode는 C17 계약을 따른다. 이 경계를 넘어선 입력을 LTR 물리 값으로 조용히 치환하지 않는다.
- `position: fixed`는 첫 단계에서 viewport 기준만 다룬다. transform·contain·filter 등 고정 containing block을 바꿀 수 있는 미지원 ancestor는 fail-closed한다. sticky는 scrollport와 clipping 정보가 연결되기 전에는 구현 완료로 표시하지 않는다.
- `z-index`는 전역 숫자 정렬로 처리하지 않는다. stacking context 트리·auto/음수/양수 stack level·Flex item 예외를 C12.4에서 painter 입력에 반영한다. transform·opacity 등 미지원 stacking-context 생성자는 해당 단계에서 목록화하고 지원 없이 통과시키지 않는다.
- viewport containing block의 CSS px 크기와 원점은 `EnvironmentRevision`에 묶인 CSS viewport에서 얻는다. Surface texture 물리 pixel, dp/point, safe area inset을 CSS viewport로 혼합하지 않는다. resize, root inset/size, device scale 변경이 owner·percentage 결과의 최신성에 반영되는지 확인한다.

## 표준 기준과 구현 고정본

- CSS Positioned Layout Level 3의 `position`, containing block, inset, relative/absolute/fixed/sticky 모델을 계약 근거로 삼는다. 이 계획은 2025-10-07 W3C Working Draft를 고정하며, Working Draft의 모호성은 고정 Chromium 관찰과 함께 기록한다. [CSS Position 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)
- Flex 자식의 out-of-flow 배치와 paint 순서는 고정 문서의 절대 위치 자식 절을 사용한다. [CSS Flexbox · 2025-10-14 CRD](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/) · [CSS Display 3 · 2026-06-05 CRD](https://www.w3.org/TR/2026/CRD-css-display-3-20260605/)의 Flex/Grid `order: 0` paint 규칙.
- 비교 브라우저는 CSS 기준 저장소의 Chromium `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`을 쓴다. 기준 빌드·fixture·reference digest가 맞지 않으면 geometry pass/fail 비교를 중단한다.
- WPT는 각 단계의 사전 비교 모델을 만들 때 upstream commit SHA와 case 경로를 고정하고 Chromium reference와 연결한다. 구현 시작 전에 이를 artifact에 기록한다. 일부 WPT 통과나 이 계획의 fixture 결과를 WPT 전체·웹 전체 지원 주장으로 넓히지 않는다.

## 단계별 동작 계약

### C12.1 · static과 relative

- CSS 기본 `position`은 `static`이다. `static`에서는 inset 값이 used positioning offset이 아니며 absolute containing block을 만들지 않는다.
- `relative`는 normal-flow에서 계산한 위치를 보존한 채 최종 시각 frame만 offset한다. 이 offset은 sibling의 layout position과 부모의 normal-flow 크기를 바꾸지 않는다. 상대 inset의 반대편·양쪽 값, `auto`, 음수, percentage, `calc()`는 고정 Chromium 기준으로 정한다.
- layout root가 `relative`인 경우 자신의 containing block은 viewport다. Taffy가 root inset을 계산하도록 viewport 크기의 synthetic parent를 해당 입력에만 추가하며, 이 parent는 HostDocument·NodeId·renderer tree에 노출하지 않는다.
- `relative` offset이 scrollable overflow에 미치는 효과는 C13 scroll/overflow 계약과 함께 닫는다. scrollable overflow 미구현 상태의 작은 fixture는 clipping·스크롤 지원 완료 증거가 아니다.
- `relative`는 자손의 absolute containing block이 될 수 있다. 자식의 containing-block owner를 computed `position` 값에서 찾고, static 중간 ancestor가 이를 가로채지 않는지 확인한다.
- `inset` shorthand의 1–4 값, longhand 재정의, cascade layer·specificity·`!important`·`var()`를 computed typed side에 보존한다. 속성 원문 파서로 CSS 문법을 재구현하지 않는다.
- `display:none`은 위치·inset 값이 있어도 box, containing block, layout frame 또는 paint entry를 만들지 않는다. `display:contents`, table/inline formatting과 fragmented inline containing block은 현재 box topology가 이를 표현하지 않으므로 성공 경로에 섞지 않고 명시적으로 거부한다.
- C12.1 구현은 in-flow frame와 visual-offset frame을 혼동하지 않도록 내부 결과에서 두 좌표를 구분하고, renderer에는 한 번만 offset된 final frame을 게시한다. hit-test 연결은 S05 입력 계약이 제공되기 전까지 완료됐다고 주장하지 않는다.

### C12.2 · Block absolute

- absolute box는 normal flow 및 Flex/Block sibling sizing에서 빠지지만 HostDocument parent와 source order는 그대로 보존한다. abs 노드는 DOM traversal·수명·이벤트 소유권에서 사라지지 않는다.
- absolute/fixed box는 CSS `display`를 blockify하고 독립 formatting context를 만든다. cascade의 computed `display`와 layout에 쓰는 used display를 분리하고, inline/flex/grid/table 계열 child display가 조용히 원래 formatting context를 유지하지 않는지 확인한다.
- block ancestor의 padding edge가 containing block을 만든다. border edge, content edge, viewport 원점을 서로 바꾸지 않도록 C07.2 border/padding geometry와 대조한다. 적절한 positioned ancestor가 없을 때 앱 문서의 initial containing block은 `EnvironmentRevision`의 CSS viewport로 정하고, safe-area inset이나 GPU surface 원점을 섞지 않는 case를 고정한다.
- inset percentage는 containing block의 대응 축 크기를 기준으로 계산한다. top/bottom은 block 축, left/right는 inline 축의 percentage basis를 혼합하지 않는다. CSS 계산식은 기존 C06 typed-math path를 통과한다.
- width/height·min/max·box-sizing·padding·border·margin과 0–4개의 definite/auto inset 조합을 CSS absolute sizing 규칙에 맞춰 처리한다. auto margin·양쪽 inset에 따른 stretch와 over-constrained 방향 선택을 케이스로 나눈다. `top/right/bottom/left`의 `calc()`는 기존 C06 expression evaluator에 새 property/basis 항목으로 추가하고 percentage axis별 resolution test를 둔다. C14/C15가 필요한 intrinsic text/replaced 크기는 미지원 진단으로 남기며 빈 값이나 0으로 바꾸지 않는다.
- absolute 자식의 containing-block owner와 원래 formatting context가 다를 수 있다. 두 관계를 유지하지 못하는 입력은 임의의 parent frame으로 계산하지 않고 fail-closed한다. Block formatting context에서 한 축의 양쪽 inset이 `auto`이면 static position은 in-flow hypothetical box 위치에서 유도한다. static-position rectangle은 static-position containing block의 inline 축 양끝에 걸친 두께 0인 영역이며 block-start 위치에 놓인다. 첫 물리 LTR subset은 block-start/inline-start 정렬을 쓴다. 이 owner는 실제 containing block과 다를 수 있다. source order·margin을 반영한 hypothetical 위치나 두 owner 사이 좌표 변환을 산출할 수 없는 노드는 명시적으로 거부하고 별도 fixture로 증명한다.
- absolute/fixed node의 used size는 definite inset·size가 없는 경우 shrink-to-fit/intrinsic sizing에 들어간다. 그 size를 측정할 수 없는 텍스트·replaced child는 ancestor까지 연쇄 실패시킨다. 중간 임시 zero-size/viewport-width frame은 게시하지 않는다.

### C10.3.5 · Flex absolute child 교차 계약

- absolute Flex child는 Flex line count·gap·flex sizing에 참여하지 않는다. Flexbox의 static-position rectangle은 실제 containing block과 독립적으로 Flex formatting context에서 정하고, auto inset에 `align-self`/`justify-content`가 미치는 효과를 따로 계산한다.
- Flex/Grid container의 절대 자식은 paint order를 정할 때 `order: 0`으로 취급한다. 같은 rank의 source order를 보존하되, HostDocument 자식 순서 자체를 `order`나 paint sort로 변경하지 않는다.
- 이 동작은 C12.2 Block absolute의 성공만으로 자동 완료 처리하지 않는다. C10.3.5 전용 Chromium fixture와 runtime paint evidence가 필요하다.

### C12.3–C12.5 · fixed, stacking, sticky

- 기본 fixed는 viewport를 fixed containing block으로 사용한다. 환경/viewport revision이 바뀌면 이전 계산을 새 frame처럼 게시하지 않는다. transform 등 ancestor가 fixed containing block을 만드는 조합은 해당 ancestor semantics가 들어오기 전까지 명시적으로 실패한다.
- fixed는 absolute와 같은 blockification/independent formatting context 규칙을 따른다. 첫 fixed 단계에서도 ancestor transform·contain·filter 경로를 전체 subtree에서 검사하고, 지원하지 않는 하나의 ancestor가 고정 viewport 결과에 섞이지 않게 한다.
- paint 순서는 source order, Flex/Grid order-modified order, stacking context tree를 별도로 계산한다. positioned `z-index:auto`, 음수·0·양수, nested context와 context 밖 sibling을 함께 비교한다. Flex item의 position이 static이어도 non-auto `z-index`가 paint에 영향을 주는 경우를 CSS Display/Flex 기준으로 확인한다. Sticky box는 stacking context를 만들고, 해당 규칙을 C12.4에서 별도 case로 둔다.
- sticky는 nearest scrollport, inset, scroll offset, containing block 한계, clip 및 oversize margin box를 모두 입력으로 받는다. C13의 scroll container·scroll revision·clip을 임의의 viewport 값으로 대신하지 않는다.

## 비교 모델과 fixture 규칙

구현 단계 시작 전 해당 단계 전용 사전 비교 artifact를 만들며, 기준 결과를 먼저 캡처한다.

| 항목 | 기준·판정 |
| --- | --- |
| 입력/환경 | CSS fixture source hash, Chromium revision, runtime profile, viewport·DPR, computed `position`/inset, parent 관계를 기록한다. 기본 환경은 `horizontal-tb`·LTR이며 C17 범위는 별도로 표기한다. |
| Geometry | NodeId별 `x`, `y`, `width`, `height`의 CSS px 값을 Chrome `getBoundingClientRect()`와 비교한다. 각 지원 field 허용 오차는 `0.5 CSS px`이며, 누락 노드·중복 노드·잘못된 containing-block owner는 오차 평균으로 숨기지 않고 실패다. |
| CSS cascade | computed `position`, 물리 inset, shorthand reset/override, custom property fallback의 값을 별도 비교한다. computed keyword가 같아도 used geometry가 틀리면 실패다. |
| Paint | C12.4/C10.3.5에서 paint-list rank와 겹치는 불투명 색 box pixel을 확인한다. geometry 일치를 paint 순서 증거로 대체하지 않는다. |
| 오류/원자성 | 지원 밖 값, non-finite 값, 끊긴 owner graph, revision mismatch는 NodeId·property가 식별되는 오류로 실패한다. 일부 node만 새 frame이고 나머지가 이전 frame인 부분 성공을 허용하지 않는다. |
| 모바일 | 각 단계에서 지원한다고 판정할 동일한 JS/CSS fixture case를 실제 V8→Stylo→Taffy→WGPU 경로로 Android 연결 실기기와 iOS Simulator에서 실행한다. 양쪽 실행이 불가능하면 실행한 플랫폼만 검증 완료로 기록한다. 빌드·환경·로그·화면을 증거에 보관하며, 별도 smoke를 전체 Chromium 행렬·성능·pixel equality로 확대하지 않는다. |

초기 C12.1/C12.2 inventory는 syntax/cascade 8개, static/relative flow 8개, CB ancestry와 edge geometry 12개, insets·size·margin 16개, negative/fail-closed 8개 이상을 포함한다. 각 case는 고정 CSS Position 3 버전·고정 Chromium·고정 WPT commit/case 경로와 예상 좌표를 기록하고, 실제 Chromium oracle을 캡처하기 전에는 구현 코드를 바꾸지 않는다. fixture case 수를 구현 완료 조건 대신으로 사용하지 않는다.

#### C12.2 Block absolute 사전 기준

C12.2 전용 기준은 [사전 비교 기록](../spec/internal/evidence/c12-2-absolute-block-precomparison-2026-10-11.md), [23-case inventory](../tests/fixtures/css/c12/position-absolute-block-inventory.json), [공용 JavaScript fixture](../tests/fixtures/css/c12/runtime-position-absolute-block.js), [HTML 진입점](../tests/fixtures/css/c12/position-absolute-block.html), [Chromium reference](../tests/fixtures/css/references/c12-2-position-absolute-block-v1.json)다. 고정 Chrome 154.0.8037.98·revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS arm64에서 360×800 CSS px·DPR 1/2, 23 case·82 node의 computed property·source parent·containing-block owner·frame을 수집했다. 지원 field 오차 한도는 각 축 `0.5 CSS px`이며 누락·중복 NodeId 및 owner 차이는 별도 실패다. `flow-absolute`, `static-absolute`, `static-diff-absolute`는 각 target을 잠시 `position:static; inset:auto`로 바꾼 독립 관찰값을 함께 고정한다. 마지막 target은 source parent와 nearest positioned containing-block owner가 다른 경우의 좌표 변환을 검증한다.

기준은 CSS Position 3 2025-10-07 Working Draft다. WPT revision `9ec154ff43db468923997c08bb08f905ceab62a5`에서 `position-absolute-padding-percentage.html`, `position-absolute-percentage-height.html`, `position-absolute-margin-auto-001.html`, `position-absolute-dynamic-static-position.html`, `position-absolute-dynamic-formatting-context.html` 경로가 존재함을 확인했다. WPT suite 자체는 실행하지 않았고 이 fixture 결과를 WPT 통과로 표현하지 않는다. 현재 inventory는 Block·LTR의 고정 크기 요소만 다룬다. 텍스트/replaced intrinsic sizing, Flex static-position, Grid, transform/contain, RTL·논리 inset, paint·hit-test는 계속 선행/후속 계약으로 남긴다.

## 구현·검증 게이트

1. C12 계획 실패 관점 검토를 구현 전 별도 문서에 기록하고 이 계획에 반영한다. 계획 검토 표를 구현 검토에 재사용하지 않는다.
2. C12.1/C12.2별 비교 모델, source/owner graph, Typed style DTO, 오류 계약을 정하고 내부 계약 ID와 예제를 먼저 작성한다. 계약 숫자 버전은 출시 전 `0.1.0` 고정이다.
3. 각 기능 PR에서 독립 실패 경로 20개 이상을 코드·fixture·실행 결과에 대조하고, 발견한 결함을 고친 뒤 영향 범위를 다시 확인한다.
4. `mise exec -- cargo fmt --all -- --check`, `mise exec -- cargo test --locked --workspace --all-features`, `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, `mise exec -- bun run test:css-reference`를 실행한다. 바뀐 범위를 덮지 않는 기존 전체 test pass만으로 C12를 완료 판정하지 않는다.
5. Android 실기기 설치/실행과 iOS Simulator 설치/실행을 각각 확인한다. 둘 중 실행하지 않은 플랫폼은 상태 대장에 완료로 표시하지 않는다. 필요하면 WGPU screen capture, V8/runtime log, C12 summary log를 같은 evidence 단위에 둔다.
6. 문서 구현 체크·PR·preview를 동기화한다. GitHub Pages workflow는 실행하지 않는다.

## 제외와 종속성

- C17: RTL, bidi, writing mode, 논리 inset.
- C13: scroll container, scroll offset, sticky scrollport와 clip. sticky를 viewport-fixed로 대체하지 않는다.
- C14/C15: min/max/intrinsic text size, line box, replaced element 측정, inline containing-block fragmentation.
- box-tree가 다른 display: `display:contents`, table formatting, inline fragment의 containing block, multi-column fragmentation.
- C16 및 기타 ancestor effect 계약이 확정되기 전: transform·contain·filter·perspective에 따른 containing block/stacking effect.
- C22: 일반 paint effect·clip·compositing·stacking 구현의 renderer 계약. C12.4가 C22 painter 입력을 요구해도 C22 전체를 이 PR로 끌어오지 않는다.
- Grid absolute placement는 X09 Grid geometry 및 Grid-specific containing block/static-position 규칙 이후 별도 구현한다.
- outline, shadow, border paint, hit-test·pointer dispatch, 접근성 탐색 순서는 이 계약의 positioning geometry만으로 완료 처리하지 않는다.
- position/inset/ancestor containing-block 변경은 해당 descendant의 layout/style cache를 무효화한다. cache key에서 관련 owner/style/environment revision이 빠지면 오래된 containing block 결과를 재사용하지 못하도록 이전 snapshot을 거부한다.

각 제외 입력은 성공을 가장하지 않도록 실행 profile에 진단 경계를 둔다. 하위 계획의 `제외`는 최종 호환 목표를 줄이는 결정이 아니며, 해당 기능의 선행 계약이 닫힌 뒤 C12 또는 그 소유 명세에서 계속 진행한다.
