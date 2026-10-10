# C10.3 · Flex 순서와 정렬 구현 계획

**상위:** [C10 Flexbox](c10-flexbox.md) · [공식 상태 대장](../spec/STATUS.md)
**현재 상태:** C10.3.1은 PR #119, C10.3.2는 PR #120, C10.3.3은 PR #122, C10.3.4는 PR #125로 리베이스 병합했다. C12.2도 제한 Block/LTR 구현으로 PR #130 병합을 마쳤다. C10.3.3은 고정 Chrome 154 50개 case·134개 node를 여섯 runtime profile에서 검사했고, Android SM-S731N 실기기와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8→Stylo→Taffy→WGPU 8-node frame이 Chrome 기준과 일치했다. C10.3.4는 Chrome 154 18 case·73 node를 DPR 1·2와 여섯 runtime profile에서 비교했고, Android 실기기와 iOS Simulator의 별도 V8→WGPU smoke를 확인했다. 텍스트 baseline, WPT 전체 실행, 전체 모바일 fixture 행렬, RTL·다른 writing mode는 완료 범위에 포함하지 않는다. 다음 단계는 [C10.3.5 전용 계획](c10-3-5-positioned-flex.md) 검토 후 독립 구현 PR로 진행한다.
**내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정.

## 목표

현재 runtime Flex adapter는 `row|column`, `nowrap|wrap`, 제한된 `align-items`와 `justify-content`만 typed layout 값으로 전달한다. 다음 단계는 CSS의 주축·교차축 방향, Flex item의 `order`, 줄 정렬, item별 정렬과 baseline을 단계적으로 연결한다. Taffy 0.14.0은 후보 계산기이고, 고정 Chromium 154 reference가 수치 비교 기준이다. enum이나 CSS computed property를 매핑하는 것만으로 지원 완료 처리하지 않는다.

이 계획의 모든 단계는 기본 runtime 경로인 V8 → Stylo cascade → typed layout snapshot → Taffy → WGPU를 대상으로 한다. 상태 대장의 C10.3 하위 체크는 각 단계별로 갱신하지만, 다섯 하위 단계와 C15 텍스트 baseline 연결이 끝나기 전에는 C10.3 또는 C10 전체를 완료로 표시하지 않는다.

## 구현 단계

| 하위 ID | 범위 | 완료 전제 | 상태 |
| --- | --- | --- | --- |
| C10.3.1 | `row-reverse`, `column-reverse`, `wrap-reverse`와 `flex-flow` 조합 | C10.1·C10.2에서 고정한 line 수집·크기 배분과 분리해 축 시작점 및 line stacking을 비교한다. | PR #119 병합 · [계약 0050](../spec/internal/0050-c10-3-1-flex-reverse.md) |
| C10.3.2 | `order`의 안정적인 계산 순서와 paint 순서 | HostDocument 자식 순서를 유지하고 layout·paint·source traversal을 분리한다. | [PR #120 병합](https://github.com/ohah/spinon/pull/120) · 제한 구현과 Android/iOS Simulator runtime 검증 완료 · [계획 검토](../spec/internal/evidence/c10-3-2-order-plan-review-2026-10-10.md) · [구현 계약 0051](../spec/internal/0051-c10-3-2-flex-order.md) · [실행 근거](../spec/internal/evidence/c10-3-2-flex-order-implementation-review-2026-10-10.md) |
| C10.3.3 | Flex Box Alignment의 비-baseline longhand·shorthand | `align-items`·`align-self`·`align-content`와 `place-items`·`place-self`·`place-content`를 계산·layout 경계까지 비교한다. `place-content`가 설정하는 `justify-content`도 shorthand의 전체 값을 처리할 수 있게 runtime Flex 범위에서 함께 확장한다. | [PR #122 병합](https://github.com/ohah/spinon/pull/122) · [내부 계약 0052](../spec/internal/0052-c10-3-3-flex-box-alignment.md) · [실패 관점 검토·실행](../spec/internal/evidence/c10-3-3-flex-alignment-implementation-review-2026-10-11.md) |
| C10.3.4 | item baseline self-alignment, baseline content-alignment 경계, Flex container baseline | `align-items`·`align-self`의 first/last baseline과 order·line별 baseline 참여를 비교한다. `align-content:first baseline`은 line baseline 분배로 간주하지 않고 textless fixed-box의 Chrome computed/geometry 관찰에 한정한다. Flex container의 first/last baseline 전파를 별도 nested fixture로 비교한다. | [PR #125 병합](https://github.com/ohah/spinon/pull/125) · [구현 검토](../spec/internal/evidence/c10-3-4-baseline-implementation-review-2026-10-11.md) · [계획 검토](../spec/internal/evidence/c10-3-4-baseline-plan-review-2026-10-11.md) |
| C10.3.5 | positioned Flex child의 static-position·순서·paint 통합 | 직접 Flex child와 Block wrapper 후손을 구분한다. absolute child는 Flex line 계산에서 제외하고 직접 Flex child는 paint 비교에서 `order:0`으로 취급한다. static-position formatting owner와 actual containing-block owner를 분리한다. | [전용 계획](c10-3-5-positioned-flex.md) · [계획 실패 관점 검토](../spec/internal/evidence/c10-3-5-positioned-flex-plan-review-2026-10-11.md) · C12.2 PR #130 병합 |

각 구현 PR은 하나의 하위 단계를 소유한다. C10.3.1부터 C10.3.4까지는 병합했다. C10.3.5는 전용 계획·계획 검토를 먼저 마친 뒤 별도 PR로 진행한다. 한 PR에 여러 단계를 묶지 않는다. C10.3 parent는 다섯 단계와 C15 텍스트 baseline 연결이 모두 닫히기 전까지 미완료다. 각 기능 구현 PR은 계획 검토와 독립된 새 실패 관점 20개를 기록한다.

## 공통 비교 계약

- **Oracle:** Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`. fixture, capture 도구와 Chromium 실행 파일의 SHA-256을 reference에 저장하고 테스트 실행으로 reference를 자동 갱신하지 않는다.
- **표준 기준:** CSS Flexbox Level 1, 2025-10-14 Candidate Recommendation Draft는 Flex item layout·paint 순서, CSS Box Alignment Level 3, 2026-10-08 Working Draft는 C10.3.3 alignment grammar와 shorthand 기준이다. Box Alignment 초안에서 overflow-position은 at-risk로 표시되므로 이를 안정 표준 기능으로 단정하지 않고 pinned Chrome 결과와 명세 버전을 함께 보존한다. `order` longhand는 CSS Display Level 3, 2026-06-05 Candidate Recommendation Draft, CSS Values Level 4, 2024-03-12 Working Draft는 integer 계산·반올림 기준으로 삼는다. 초안·후보 초안은 고정된 기준 문서일 뿐 최종 표준이라고 주장하지 않으며, 실제 고정 Chromium과 잠긴 Stylo 결과를 함께 기록한다. [CSS Flexbox 2025-10-14](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/) · [CSS Box Alignment 2026-10-08](https://www.w3.org/TR/2026/WD-css-align-3-20261008/).
- **환경:** viewport `320×240 CSS px`, locale `en-US`, timezone `UTC`, DPR 1과 2 각각. 입력 fixture는 `writing-mode:horizontal-tb`, `direction:ltr`를 명시한다. RTL과 다른 writing mode는 C17 선행 계약을 확인하기 전까지 지원 범위에 넣지 않는다.
- **관찰값:** 각 지원 노드의 computed `flex-direction`, `flex-wrap`, `flex-flow`의 longhand 결과, `order`, `align-items`, `align-self`, `align-content`, `justify-content`, node ID별 `x`, `y`, `width`, `height`, 필요 시 paint-list 순서를 기록한다. computed CSS 문자열, Chromium Typed OM에서 읽은 integer, Rust typed 값은 별도 필드로 보존한다. Chromium `getComputedStyle().order`는 int32 경계에서 지수 표기로 정밀도를 잃으므로 경계값 비교는 `computedStyleMap().get("order").value`와 geometry를 사용한다. 현재 없는 runtime hit-test와 pointer target은 C10.3 관찰값에 넣지 않으며 S05에서 별도 연결한다.
- **기하 허용치:** 각 노드·각 frame field의 최대 절대 오차 `0.5 CSS px`. 평균 오차로 단일 실패를 가리지 않는다. DPR 1·2 결과가 각각 통과해야 하고 CSS px geometry는 DPR에 따라 달라지면 안 된다.
- **스타일 경로:** `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1` 여섯 profile의 typed projection, inline full/incremental cascade, author stylesheet full cascade, 등록 사용자 지정 속성 경로를 확인한다. 현재 author stylesheet incremental 경로가 재사용을 거부하면 전체 cascade fallback을 확인하며, 지원하지 않는 stylesheet 재사용을 성공으로 간주하지 않는다.
- **순서 경계:** 현재 `CalcLayoutTree`는 Taffy 노드를 postorder 위치로 만들고 `Layout::order`를 초기화한다. Flex algorithm은 일부 자식의 출력 order를 다시 지정한다. 현재 runtime paint snapshot은 별도로 HostDocument preorder를 순회해 paint rank를 만든다. 이 세 순서를 CSS `order` 값과 혼동하지 않는다. 필요한 계산 child 순서는 flex container의 형제 범위에서만 안정 정렬하고, paint list는 해당 형제 item의 order-modified 순서를 재귀적으로 반영하되 nested subtree를 부모 밖으로 끌어내지 않는다. HostDocument는 원본 순서를 보존한다.
- **앱 실행:** 각 구현 단계가 Rust·고정 Chromium oracle을 통과한 뒤, 연결된 Android 실기기가 있으면 우선 사용하고 없으면 Android emulator를 사용한다. iOS는 iPhone 17 Pro / iOS 26.2 Simulator에서 확인한다. runtime node frame, WGPU 제출과 오류 로그를 보존한다. Android software backend 및 simulator 실행을 실기기·하드웨어 GPU 성능 증거로 확대하지 않는다.
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

- `order`는 음수와 양수를 받는 정수이며 initial은 0, 비상속이다. CSS 표준의 abstract integer와 별도로, 잠긴 Stylo 0.22.0은 computed integer를 `i32`로 표현하고 고정 Chromium 154도 literal 초과값을 `i32::MIN..=i32::MAX`로 clamp한다. 따라서 typed layout 값은 `i32`로 두고 `i32::MIN`, `i32::MIN - 1`, `i32::MAX`, `i32::MAX + 1`의 적용 결과를 반대 source order로 비교해 경계 clamp와 동점 안정성을 구분한다. 참조 문자열만으로 경계값을 비교하지 않는다.
- 일반 `1.0`·`1.5` 같은 non-integer token은 invalid declaration로 무시되고, 앞선 유효 선언 또는 initial `0`이 남아야 한다. CSS integer 계산은 유효할 수 있으므로 `calc(1.5)`와 `calc(-1.5)`의 tie rounding도 포함한다. 고정 Chromium 154는 각각 `2`, `-1`을 계산한다. `var()`를 이용한 order와 등록 여부가 다른 runtime custom-property profile도 확인한다.
- `order`는 Flex item의 line collection과 layout 순서를 바꾼다. 이 단계는 지원 runtime Flex profile의 in-flow element child만 다루며, CSS Grid item 적용은 Grid layout 지원 전까지 제외하고 `display:grid`를 기본 Flex/Block으로 조용히 바꾸지 않는다. text/anonymous flex item은 C15에서 연결한다. `display:none` child는 Flex item이 아니며 line 계산과 paint에서 제외한다. out-of-flow absolute child는 flex line 계산에서 제외되지만 Flexbox 기준 paint 비교에서는 `order:0`이므로, 이 혼합 순서와 static-position alignment는 C10.3.5/C12에서 따로 닫고 그전에는 `order` 전체를 완료 처리하지 않는다.
- line collection, basis/grow/shrink 계산과 item frame이 order-modified document order를 따른다. 값이 같은 item은 원본 문서 순서를 유지한다. 전체 HostDocument 및 사용자 JS의 `childNodes`/`children` 결과는 원본 source order다.
- W3C가 정한 in-flow Flex paint 순서와 layout 순서를 별도 경로에서 확인한다. `order`가 달라진 겹침 case에서는 runtime paint-list 순서와 실제 Android/iOS WGPU 출력의 불투명 box 픽셀을 모두 확인한다. sibling flex item을 정렬할 때 각 item의 descendant subtree를 함께 유지한다. 서로 다른 nested container의 item을 한 global `order` 값으로 섞지 않는다. `row-reverse`와 `column-reverse`는 이 paint rank를 대신 바꾸지 않는다. absolute child의 `order:0` 상호 순서, static-position alignment, `z-index`와 stacking context는 C10.3.5와 C12·C22 계약에 맡긴다.
- 현재 `RuntimeRenderSnapshot`에는 일반 runtime hit-test API가 없고 S05 입력 이벤트 전달도 미구현이다. 이 단계에서는 frame·paint entry가 안정된 NodeId와 연결되는 것까지만 확인한다. CSS paint order를 실제 플랫폼 pointer target으로 사용한다고 주장하지 않는다. S05가 구현될 때 overlap target 선택을 별도 계약·검증으로 연결한다.
- 시각 `order`는 source/speech/순차 키보드 탐색 순서를 재정렬하지 않는다. HostDocument source order 불변식을 테스트한다. 네이티브 접근성 트리는 아직 별도 구현 범위이므로 접근성 통합이 완료되었다고 주장하지 않는다.

### 완료 조건

- 같은 order tie는 입력 tree order로 안정적이며, signed `i32` order가 작은 item부터 배치·그려진다. 변화 후 layout, paint와 style revision이 하나의 일관된 결과로 게시된다. Chromium 경계값은 CSS Typed OM 값 및 frame으로 판정하고 `getComputedStyle()`의 지수 표기 손실을 허용치에 숨기지 않는다.
- order는 CSS non-flex context의 일반 자식·다른 subtree ordering을 바꾸지 않고, 다른 flex container의 item과 비교하지 않는다. 계산 순서 정렬용 vector를 원본 DOM 자식 목록에 다시 써 넣지 않는다.
- Chromium fixture의 node별 geometry와 paint rank가 일치하고, Android API 37 emulator와 iOS 26.2 Simulator에서 겹친 색상 결과까지 확인한다. latest style 계산이 실패하면 과거 paint order를 새 revision frame으로 노출하지 않는다. 오류에는 node/property가 포함된다.

## C10.3.3 · Flex Box Alignment의 비-baseline 값

### 지원할 값과 cascade 범위

- 기준 문법은 위에 고정한 CSS Box Alignment 2026-10-08 §4–§7 및 CSS Flexbox 2025-10-14를 사용한다. Box Alignment는 Working Draft이며 overflow-position(`safe`/`unsafe`)을 at-risk로 표시한다. 이 범위는 “그 문서에 있는 모든 CSS Box Alignment”가 아니라 아래 Flex 관련 longhand·shorthand 및 명시한 교차 조건이다.
- `align-items`: `normal`, `stretch`, 그리고 선택적 `safe|unsafe`가 붙는 `center|start|end|self-start|self-end|flex-start|flex-end`. `baseline` 계열은 C10.3.4에 맡긴다. 이 단계는 기존 C04.3 profile을 넓히지 않고 여섯 runtime Flex profile의 값만 확장한다.
- `align-self`: `auto`, `normal`, `stretch`, 그리고 `align-items`와 같은 비-baseline positional set 및 `safe|unsafe`. `auto`는 CSSOM에 `auto`로 보존하며 used alignment는 같은 Flex container의 computed `align-items`를 따른다. 부모 값의 CSS 상속으로 구현하지 않는다. `normal`은 Flex에서 used `stretch` 동작을 한다.
- `align-content`: `normal`, `stretch`, `space-between|space-around|space-evenly`, 그리고 선택적 `safe|unsafe`가 붙는 `center|start|end|flex-start|flex-end`. `normal`은 computed keyword로 보존하면서 Flex의 used 동작을 `stretch`와 비교한다. `left|right`는 `align-*` 값이 아니므로 invalid declaration이다. baseline 계열은 C10.3.4 소유다.
- `place-items`와 `place-self`는 해당 align longhand의 입력 shorthand로 포함한다. 한 값일 때 두 번째 `justify-items`/`justify-self` 성분이 복사되는 cascade 결과를 보존하되, 그 성분은 Flex item 배치에 영향을 주지 않아야 한다. Flex가 아닌 Grid 효과를 지원한다고 주장하지 않는다.
- `place-content`는 `align-content`와 `justify-content`를 함께 설정하므로 두 성분을 모두 이 단계에서 지원한다. 두 번째 값 생략 시 CSS shorthand 규칙대로 첫 값을 복사한다. 이에 따라 여섯 runtime Flex profile의 `justify-content`도 `normal`, `stretch`, distribution 값, `center|start|end|flex-start|flex-end|left|right` 및 positional 값의 `safe|unsafe` 조합을 처리한다. Flex main-axis의 `stretch`는 Flexbox 규칙에 따른 used `flex-start` 결과를 낸다. `left|right`는 `justify-content`의 두 번째 성분에만 허용하고, `column`처럼 main axis가 좌우 축과 평행하지 않은 경우 pinned Chrome fallback과 비교한다. 기존 C04 profile은 변경하지 않는다.
- `safe`/`unsafe`는 positional 값에만 허용한다. `safe space-between`, `unsafe stretch`, `safe auto`, `safe` 단독, `align-items:auto`, `align-content:left` 등 잘못된 조합은 cascade에서 invalid declaration로 무시되는지 Chrome과 비교한다. 유효한 지원 값과 baseline 등 이 단계 밖의 computed 값을 Rust adapter가 받으면 기본값으로 대체하지 않고 property/node 문맥을 포함해 실패한다.

### 사용값과 레이아웃 상호작용

- `align-items`는 line 안의 모든 Flex item 기본값이고 `align-self`는 개별 item override다. 두 속성 모두 비상속으로 처리한다. `align-self:auto`가 부모 `align-items`를 참조하는 경로와 inline/stylesheet/custom-property cascade가 같은 computed 값을 읽는 경로를 확인한다.
- 정렬 대상은 Flex item의 margin box이며 정렬 컨테이너는 해당 item이 속한 flex line이다. row와 column의 교차축, 각 child별 line, 빈 컨테이너·item 하나를 분리한다. `row-reverse`, `column-reverse`, `wrap-reverse`의 flex-relative start와 writing-mode-relative `start/end`를 혼동하지 않는다. 기준 환경은 `writing-mode:horizontal-tb; direction:ltr`; 다른 writing mode, 상이한 item direction 및 RTL은 C17에서 다룬다. 이 제한 안에서 `self-start/end`의 의미와 `flex-start/end`가 `wrap-reverse` 등으로 달라지는 경우를 고정한다.
- cross-axis auto margin은 align-self보다 우선한다. positive free space가 auto margin에 먼저 분배되는 경우, 남는 free space가 0인 경우, item이 overflow해 auto margin이 효과를 잃는 경우를 각각 둔다. align-self가 남은 공간을 다시 차지한다고 가정하지 않는다.
- `stretch`는 해당 축의 computed size가 `auto`이고 그 축에 auto margin이 없을 때만 늘어난다. 고정 cross size, cross-axis auto margin, min/max clamp, `box-sizing`, padding, border를 조합해 content/border/margin box 차이를 대조한다. 한 line 안의 다른 item 크기가 line cross size를 결정하는 상호작용과 stretch 이후 min/max 제한도 확인한다.
- `align-content`는 flex line을 교차축에서 정렬하고 Flex의 multi-line container에만 효과가 있다. `nowrap`에서는 한 개 line만 존재하므로 `align-content`가 배치·크기를 바꾸지 않는 음성 대조를 둔다. `wrap`/`wrap-reverse`는 실제로 line이 한 개만 생성되어도 multi-line container로 동작하는지 별도로 확인한다. `space-between`처럼 subject 하나에서 분배할 공간이 없는 값은 해당 fallback과 비교한다.
- 실제 2개 이상 line은 cross-axis free space 양수·0·음수 상태를 둔다. `stretch`, 모든 distribution 값, positional 값, safe/unsafe 및 생략된 safety를 pairwise 조합한다. `row-gap`/`column-gap`은 line/item 간 기존 gap 위에 정렬 공간을 중복 분배하지 않는지 검사한다. `wrap-reverse`와 `align-content:flex-start|flex-end` 및 `start|end`를 교차해 축 의미를 고정한다.
- `align-content:stretch`가 line cross size를 바꾸고 그 뒤 `align-items`/`align-self:stretch`가 item used size에 미치는 연쇄를 확인한다. line 위치만 일치해도 item 크기 오차가 있으면 실패다. safe/unsafe는 overflow 시 Chrome의 사용 좌표로 비교한다. C13 scroll container의 scrollable overflow·자동 scroll-safety 동작은 이 단계 범위 밖이다. W3C 초안의 일반 안전 기본값을 구현 근거로 대신하지 않는다.
- Shorthand 단일/두 값, 뒤따르는 longhand override, inline 대 author stylesheet의 우선순위, `var()` fallback/미정의 값/등록 사용자 지정 속성, incremental 변경을 검사한다. 특히 부모 `align-items` 변경이 자식 `align-self:auto`에 재계산되는지, `place-content` 한 값이 두 computed longhand와 두 layout 축 모두에 반영되는지 본다.

### 비교 행렬 seed

구현 전 고정 Chromium reference는 최소한 다음 독립 조건을 담고, 명시된 상호작용만 pairwise case로 추가한다. 크기 계산이 범위 밖의 intrinsic/text 동작을 끌어들이지 않도록 빈 element와 definite container/item 크기를 기본으로 쓴다.

| 비교 축 | 최소 입력 | 독립 예상·실패 조건 |
| --- | --- | --- |
| 기본값·override | `align-items:normal`; 자식 `align-self:auto|center`; row·column | computed `normal`/`auto`를 보존하고 used frame은 pinned Chrome과 대조한다. |
| self positions | 모든 positional 값과 safe/unsafe, `wrap-reverse`, item direction은 LTR 동일 | `start/end`와 flex-relative start/end를 한 좌표로 치환하지 않는다. |
| auto margin | cross-axis 한쪽/양쪽 auto, positive/zero/negative free space | auto margin 우선순위와 overflow 시 좌표가 align-self fallback과 혼동되지 않는다. |
| stretch | cross size auto/fixed, auto margin, min/max, content-box/border-box, padding/border | stretch eligibility 및 clamp 뒤 content/border/margin box가 일치한다. |
| line alignment | nowrap 1 line, wrap 1 actual line, wrap 2+ lines | nowrap 음성 대조와 wrap 1-line 효과를 구분한다. |
| distribution | `normal`, `stretch`, positional/distribution 전체; 양수·0·음수 공간 | 단일 subject fallback, gap 공제, wrap-reverse 축, line·item 연쇄가 일치한다. |
| overflow | safe/unsafe positional values와 안전 값 생략, scroll container 아님 | safe fallback, unsafe overflow, Chromium의 기본 동작을 각각 관찰한다. |
| shorthand/cascade | `place-items`, `place-self`, `place-content` 한 값/두 값 및 longhand override | 확장된 computed longhand, 무효 paired property, incremental 재계산·revision을 확인한다. |
| invalid·profile 경계 | 잘못된 safety 조합, baseline, legacy C04 profile, non-Flex parent | CSS invalid declaration 처리와 adapter fail-closed, 범위 격리를 구분한다. |

### 완료 조건

- 구현 전에 HTML/CSS, inventory와 손계산 불변식을 확정해 pinned Chrome 154 reference를 캡처한다. capture는 기존 reference를 덮어쓰지 않고 fixture·inventory·capture script·helper·Chromium binary digest를 기록한다. 각 지원 node의 computed longhand와 `x/y/width/height`를 DPR 1·2로 저장하고 node/frame field마다 최대 `0.5 CSS px` 오차를 각각 적용한다.
- `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`에서 typed projection, inline full/incremental cascade, author stylesheet cascade, custom-property 경로를 확인한다. C04·Block·다른 profile로 값이나 동작이 새지 않아야 한다.
- Taffy 0.14.0 매핑은 참고 구현일 뿐이다. computed CSS keyword와 safety modifier를 typed projection에서 보존하고, used layout 동작을 Taffy에 전달하는 변환을 별도 검증한다. 기존 Taffy enum/associated constant 이름과 CSS semantics가 같다고 가정하지 않는다.
- Chromium/Rust fixture와 별도 불변식이 통과한 뒤 연결된 Android 실기기를 우선 사용하고, 없을 때 API 37 emulator로 대체한다. iPhone 17 Pro / iOS 26.2 Simulator에서는 V8 fixture의 computed 값·frame·WGPU 제출·오류 로그를 확인한다. simulator나 software backend 결과를 실기기 또는 hardware GPU 성능 증거라고 부르지 않는다.
- upstream WPT는 `d5a765f1089ce6d3f72300281481edf3dddff7f3`로 고정하고 `css/css-align/`·`css/css-flexbox/`의 적용 가능한 shorthand, computed style, auto-margin, stretch, distribution, overflow tests를 fixture inventory에서 개별 경로로 대응한다. 경로별 pass를 확인하지 않고 WPT 전체 지원을 주장하지 않는다. 유효 baseline/positioned/text/scroll case는 각각 C10.3.4, C10.3.5/C12, C15, C13 범위 밖으로 기록한다.
- 이 하위 단계는 [기존 fixture 경로](../tests/fixtures/css/c10/flex-order-alignment.html)를 재사용하지 않고 전용 `flex-alignment-v1` inventory/fixture/reference를 둔다. 권장 경로는 `tests/fixtures/css/c10/flex-alignment.html`, `tests/fixtures/css/c10/flex-alignment-inventory.json`, `tests/fixtures/css/c10/runtime-flex-alignment.js`, `tools/css-reference/capture-c10-3-3-flex-alignment.mjs`, `tools/css-reference/c10-3-3-flex-alignment.test.mjs`, `tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json`이다. inventory는 WPT revision/path, 제외 이유, 독립 예상, 지원 profile과 case/node 수를 함께 고정한다.

## C10.3.4 · baseline

- `align-items`·`align-self`의 `baseline`/`first baseline`과 `last baseline`은 이 하위 단계의 성공 범위다. `baseline`은 first-baseline self-alignment의 computed alias로, `last baseline`은 별도 값으로 보존한다. `place-items`·`place-self`가 만드는 해당 longhand도 같은 입력·출력 계약을 따른다.
- 성공 입력은 `horizontal-tb`·LTR에서 텍스트가 없는 빈 fixed-size element box다. first·last baseline fallback, single participant, first/last 그룹의 공존, cross-axis margin, order-modified 순서, wrap·wrap-reverse와 중첩 Flex container의 first/last baseline 전파를 각각 reference에 둔다. column은 computed keyword와 cross-start fallback을 비교하고 row baseline 알고리즘을 적용하지 않는다.
- Taffy 0.14.0의 Flex output은 first baseline만 제공한다. first와 last를 같은 Taffy 값으로 축약하지 않는다. 구현은 baseline 측정치·Flex line 식별·frame 보정의 소유자와 revision을 명시해야 하며, last baseline이 필요한 중첩 입력에서도 parent가 정확한 last baseline을 받도록 해야 한다.
- `align-content:first baseline`은 line 간 baseline 정렬로 설명하지 않는다. 이 단계에서는 computed value와 pinned Chrome에서 관찰한 textless empty-content geometry만 비교하며, 실제 콘텐츠 baseline distribution을 지원했다고 주장하지 않는다. `align-content:last baseline`은 pinned Chrome의 invalid declaration/cascade fallback을 비교하고 성공 문법으로 처리하지 않는다. `place-content`는 동일한 computed longhand 경계를 따른다.
- text node·anonymous flex item·replaced intrinsic size·font shaping·line box·vertical writing mode·fragmentation은 완료 범위 밖이며 각 선행 계약(C14/C15/C17)을 따른다. baseline 산출에 해당 입력이 필요하면 임의 높이로 대체하지 않고 명확히 실패 처리한다.
- C10.3.4의 계획 검토와 확정된 관찰은 [기준선 계획 검토](../spec/internal/evidence/c10-3-4-baseline-plan-review-2026-10-11.md)와 [범위 모순 수정 후 재검토](../spec/internal/evidence/c10-3-4-scope-revision-review-2026-10-11.md)에 기록한다. 계획 검토 기록은 구현 후의 새 실패 관점 검토로 재사용하지 않는다.

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
| `baseline-first-items` | row, 높이 20/40/30px인 빈 fixed-size item, `align-items:baseline`과 `first baseline` | computed `baseline` alias, 공유 baseline 위치, 단일 참여자 fallback과 비참여 형제의 line 크기를 node별 비교한다. |
| `baseline-last-items` | 같은 row에 `align-items` 또는 `align-self:last baseline`; first·last 그룹을 따로 실행 | computed `last baseline`, line cross-end fallback, bottom margin이 서로 다른 경우를 비교한다. last를 Taffy first-baseline 값으로 축약하지 않는다. |
| `baseline-content-empty-items` | nested row Flex item에 `align-content:first baseline`, 내부는 빈 fixed-size box; 별도 2-line Flex line 분배 대조 | 첫 baseline content-alignment를 Flex line끼리 baseline 정렬하는 기능으로 해석하지 않는다. computed `baseline`과 고정 Chrome의 empty-content geometry만 기록하고 텍스트 콘텐츠 정렬은 C15 전까지 지원 완료로 표시하지 않는다. |
| `baseline-content-last-invalid` | `align-content`에 `last baseline`을 유효 선언 뒤 설정 | pinned Chrome의 invalid/fallback 결과를 확인한다. 이 입력을 `align-content:last baseline` 성공 경로로 투영하지 않는다. |
| `baseline-container-first-last` | 중첩 row Flex container, wrap·wrap-reverse·order와 비균등 높이 item | 첫/마지막 baseline set의 시각적 startmost/endmost line·item 선택과 바깥 baseline 정렬을 분리해 비교한다. text node는 넣지 않는다. |

| 축 | 고정·변형 값 | 잡을 오류 |
| --- | --- | --- |
| 주축과 교차축 | row, column; reverse 조합 | 주축·교차축 또는 gap 축의 물리 투영 오류 |
| wrapping | nowrap, wrap 단일 line, wrap 여러 line, wrap-reverse | 실제 줄 수와 multi-line container 구분 실패 |
| 순서 | source order, 음수/0/양수 order, tie, style mutation | unstable sort, DOM mutation, stale order snapshot |
| 정렬 | auto/explicit item alignment, line alignment 분배 값 | 부모 기본 정렬, auto margin, stretch·clamp 누락 |
| cascade | 초기값, shorthand, longhand override, invalid declaration, stylesheet | computed style과 typed projection 불일치 |
| 자식 종류 | visible element, display none, nested flex | item 참여 조건 또는 NodeId 매핑 오류 |
| 환경 | DPR 1/2, fixed viewport, simulator profile | 반올림·환경 차이를 CSS geometry 오차로 가림 |

### C10.3.4 · 범위와 baseline 모델

- `align-items`·`align-self`의 `baseline`과 `first baseline`은 first-baseline self-alignment다. `last baseline`은 서로 다른 값이며 first baseline alias로 축약하지 않는다. `place-items`·`place-self`가 만드는 longhand도 같은 값을 사용한다.
- baseline sharing group은 같은 Flex line 안의 호환되는 first 또는 last 참여자만 포함한다. 다른 그룹의 참여자, `align-self` 비참여자, cross-axis auto margin은 각각 분리한다.
- 현재 단계의 성공 경로는 빈 fixed-size element box다. 실제 line box와 텍스트 baseline은 C15의 font shaping·line layout 계약 전까지 산출하거나 추정하지 않는다. baseline 계산에 텍스트가 필요한 입력은 일반 높이로 조용히 대체하지 않는다.
- Flex item baseline 합성은 border edge에서 시작한다. 각 item의 first/last baseline 값과 item baseline 그룹을 별도 계산한다. C07.2 border 폭·padding·cross margin이 baseline과 line cross size에 미치는 영향을 fixture로 비교한다.
- `align-content:first baseline`은 content-alignment다. Flex item 내부 콘텐츠의 baseline content-alignment와 Flex line distribution을 구분한다. pinned Chrome에서 관찰한 textless empty-content 기하는 `flex-start` 대조와 같았지만, 이를 유용한 text baseline 지원으로 확대하지 않는다. pinned Chrome에서 `align-content:last baseline`은 지원되지 않아 invalid declaration/cascade fallback으로 다룬다.
- Flex container baseline은 `order` 반영 뒤 시각적 startmost/endmost line에서 파생한다. row·wrap-reverse를 포함하고, nested container가 baseline participant가 되는 경로를 비교한다. Taffy `0.14.0`은 Flex output에 first baseline만 제공하므로 last baseline을 Taffy 출력의 alias로 취급할 수 없다. implementation은 first·last 메타데이터와 frame 보정의 소유자·revision을 명시하고, 없는 근거를 추정하지 않는다.
- `row`는 baseline self-alignment를 비교한다. `column`은 computed keyword와 pinned Chrome의 cross-start fallback을 별도 확인하며 row baseline 계산을 적용하지 않는다. RTL·다른 writing mode는 C17 선행 계약이다.
- 계획 검토에서 확정한 실패 경계와 실제 비교 관찰은 [C10.3.4 계획 검토 기록](../spec/internal/evidence/c10-3-4-baseline-plan-review-2026-10-11.md)에 있다. 계획 검토 결과를 구현 검토 20개 관점에 재사용하지 않는다.

#### 구현 상태 · 2026-10-11

- 제한 구현과 검증은 끝났으며 [PR #125로 병합](https://github.com/ohah/spinon/pull/125)했다. 내부 계약은 [0053](../spec/internal/0053-c10-3-4-flex-baseline.md)에서 관리한다. 내부 숫자 버전은 `0.1.0`을 유지한다.
- Chrome 154 고정 기준은 18 case·73 node, DPR 1·2다. 여섯 runtime Flex profile에서 computed baseline 값과 각 node frame을 비교하고, invalid `align-content:last baseline`의 cascade fallback을 확인했다.
- wrapped line의 `align-content:stretch`·`row-gap`, `wrap-reverse`·`order`, single-item line과 동일 main 좌표의 zero-size/gap 경계를 확인했다. Taffy final frame으로 line을 재구성할 수 없는 음수 main margin은 fail-closed다.
- Android SM-S731N 실기기와 iPhone 17 Pro / iOS 26.2 Simulator에서 12-node V8→Stylo→Taffy→WGPU smoke를 재실행했다. 이는 18 case 전체의 모바일 비교가 아니다.
- 미검증: WPT 원본 실행, text/replaced baseline, RTL·다른 writing mode, 18-case 전체 모바일 행렬, Chrome과의 pixel equality, 제품 성능.

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
