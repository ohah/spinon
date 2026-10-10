# C10 · Flexbox 구현 계획

## 목표와 진행 순서

스피논의 CSS 목표는 고정 Chromium 동작 100% 호환이며 모바일 구현을 우선한다. C10은 Taffy 0.14.0의 Flex 알고리즘을 Stylo의 computed style과 스피논 노드 트리에 연결하되, 구현하지 않은 CSS 값이나 상호작용을 기본값으로 치환하지 않는 작업이다. 이 문서는 구현 순서와 각 하위 항목의 비교 기준을 정한다. 실제 상태는 [공식 상태 대장](../spec/STATUS.md)만 갱신한다.

C09.4 shrink-to-fit은 C12 positioning, C14 intrinsic sizing, C15 inline/text measurement, C26 float의 선행 구현 이후 진행하도록 기존 계획에 명시되어 있다. 따라서 다음 독립 P1 작업인 C10을 시작한다. C09.4와 C09 상위는 계속 미완료다. C07.2의 `border-width`는 레이아웃 기하에 사용되고, 눈에 보이는 테두리 페인트는 C22가 소유한다.

| 하위 ID | 범위 | 선행·완료 조건 |
| --- | --- | --- |
| C10.1 | `flex-wrap: nowrap|wrap`의 row·column 줄 수집과 gap 배치 | PR #114 리베이스 병합 완료. |
| C10.2 | flex basis·grow·shrink 배분과 freeze 반복 | [전용 계획](c10-2-flex-distribution.md)을 먼저 검토·병합한 뒤 진행한다. 각 배분 단계와 min/max 제약을 별도 Chromium oracle로 고정한다. |
| C10.3 | 축 역방향, `wrap-reverse`, `order`, `align-self`, `align-content`, baseline | 방향·그리기 순서·접근성 순서를 함께 정한 뒤 속성별로 진행한다. |
| C10.4 | 자동 최소 크기, 내재 크기, percentage와 재배치 상호작용 | C14 intrinsic sizing 및 C17 writing mode 의존성을 확인한 뒤 진행한다. |

위 분해는 C10 하위 작업 계획이며 C10 전체 완료 표시가 아니다. C10.1은 여섯 runtime Flex profile(`RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`)의 `nowrap|wrap` 줄 수집과 gap 배치를 구현해 PR #114로 리베이스 병합했다. C10.2는 같은 runtime Flex 경계에서 flex basis·grow·shrink·min/max의 줄별 크기 배분을 다룬다. 실제 V8 앱은 사용자 지정 속성 paint profile을 사용하므로 반드시 포함한다. Block 전용 및 기존 static compatibility profile에는 지원 동작이 새지 않도록 fail-closed 경계를 유지한다. 출시 전 내부 계약 숫자 버전은 `0.1.0`으로 유지한다.

## C10.1 · flex line wrapping

### 구현 범위

- `flex-wrap`의 computed value `nowrap`과 `wrap`을 구분한다. 초기값 `nowrap`과 비상속 동작을 확인하고, 여섯 runtime Flex profile 모두에서 계산한다. 특히 실제 V8 앱이 쓰는 사용자 지정 속성 paint profile까지 같은 typed layout 값에 연결한다.
- runtime Flex profile에서는 일반 Block 요소의 `flex-wrap:wrap`도 computed style 단계에서 조용히 버리지는 않는다. Taffy는 `display:flex`인 노드에서만 wrap을 사용해야 하므로, 같은 값을 가진 Block 요소의 geometry는 바뀌지 않는지 확인한다. `RuntimeBlockPaintV1`, `RuntimeBlockFormattingV1` 등 C10.1 밖 Block 전용 profile에 author declaration이 들어오면 기존 fail-closed 경계를 유지한다.
- 기존 `flex-direction: row|column`, `direction:ltr`, `writing-mode:horizontal-tb`만 성공 범위로 둔다. row는 고정 main-axis width, column은 고정 main-axis height에서 line collection을 검증한다.
- `row-gap`과 `column-gap`을 사용해 항목 사이 main-axis gap과 줄 사이 cross-axis gap을 각각 확인한다. 추가 cross-axis 여유 공간이 생기지 않도록 reference의 고정 컨테이너 크기를 줄 크기 합과 gap 합으로 정한다. 따라서 아직 지원하지 않는 `align-content` 분배가 결과에 개입하지 않는다.
- line break 경계와 여러 줄의 순서를 검증한다. 컨테이너와 항목은 `box-sizing:border-box`, 명시적 main/cross size, `justify-content:flex-start`, `align-items:flex-start`, `flex-grow:0`, `flex-shrink:0`, `min-width:0`, `min-height:0`, 0 margin/padding/border를 쓴다. 각 fixture는 빈 비텍스트 요소로 만들어 flex item intrinsic sizing을 사용하지 않는다.
- DOM preorder와 자식 순서는 유지한다. 계산된 style과 node별 `x`, `y`, `width`, `height`를 DPR 1·2에서 각각 비교한다. CSS px geometry의 node·field별 최대 절대 오차는 `0.5 CSS px` 이하이고, DPR이 달라도 CSS 결과가 같아야 한다.
- `flex-flow: row|column wrap|nowrap` shorthand는 같은 두 longhand로 전개되는 경우만 입력에 포함한다. 한 case는 shorthand 다음에 longhand override를 두어 cascade 승자도 확인한다. shorthand가 지원되지 않거나 그 값이 다른 기능을 요구하면 전체 입력을 진단 오류로 거부한다.
- `wrap-reverse`, `row-reverse`, `column-reverse`, 비기본 `order`, 비기본 `align-content`, `align-self`, baseline, text node·replaced item, auto minimum sizing, auto main size, percentage flex basis, float·position 상호작용은 지원 범위 밖으로 명시하고 기존 실패 진단을 유지한다. 이 값을 무시해 성공 geometry를 만들지 않는다.

### 구현 순서

1. 코드 변경 전에 `tests/fixtures/css/c10/flex-wrap.html`, 입력 inventory, 고정 Chromium `154.0.8037.98` 기준을 생성한다. `tools/css-reference/capture-c10-flex-wrap.mjs`가 `tests/fixtures/css/references/c10-flex-wrap-v1.json`을 한 번 생성한다. 입력과 capture 도구·Chromium 실행 파일의 digest를 기록하고 reference를 테스트 중 자동 갱신하지 않는다.
2. fixture는 row exact-fit, 1 CSS px 초과 wrap, 3개 row line과 row-gap, column 방향 wrap과 cross-axis column-gap, 명시·기본 `nowrap`, empty/single-item container, 숨긴 항목 제외, shorthand와 longhand override를 포함한다. 별도 root fixture로 wrap 비상속을 확인하고, Block 요소의 wrap 선언이 layout에 영향 주지 않는 대조도 둔다. 실제 앱 runtime 사례의 viewport root는 세로 Flex 컨테이너로 둬 첫 자식 margin collapse가 C10.1 geometry 비교에 개입하지 않게 한다. Block margin collapse 자체는 C09. computed `flex-wrap`·`flex-direction`·gap 및 모든 지원 node frame을 기록한다.
3. Rust 레이아웃 단위 시험과 `tools/css-reference/c10-flex-wrap.test.mjs`에서 `LayoutStyle`의 typed wrap 값이 pinned Taffy 0.14.0의 `FlexWrap`에 전달되는지 검증한다. 기존 C04 Flex reference 및 C06 percentage/gap 시험을 회귀 실행한다.
4. Stylo computed style projection과 여섯 runtime Flex profile의 inline·stylesheet author CSS 입력에 `nowrap|wrap`을 연결한다. Block 전용 및 기존 static compatibility profile은 대상에서 제외한다. `wrap-reverse`, 지원되지 않는 `flex-flow` 값, 비기본 `order`/`align-content`는 구체적인 node/property 오류로 실패해야 한다.
5. `tests/fixtures/css/c10/runtime-flex-wrap.js`를 통해 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8→Stylo→Taffy→WGPU 경로를 실행한다. 화면만으로 geometry를 판정하지 않고 runtime이 출력한 같은 node별 CSS frame을 Chromium reference와 비교한다. 화면·로그 원본은 `spec/internal/evidence/c10-1-flex-wrap/`에 둔다.
6. `spec/internal/0048-c10-flex-wrap.md`에 profile, typed inputs, unsupported values, 오류·revision 계약과 예제를 기록한다. 상태 대장·fixture 설명·Tailscale의 `roadmap.html` 및 `css-plan.html`을 같은 PR 작업에서 동기화한다. PR 본문에 결과 캡처와 실행 한계를 기록한다.

### 위험 경계와 완료 기준

- wrap 시 line break는 flex item의 hypothetical outer main size, 현재 line의 남은 크기, gap과 순서에 의존한다. 이 첫 slice는 explicit size와 0 margin을 고정해 shrink/grow 분배, intrinsic size, auto margin 변수를 분리한다.
- row/column의 gap 축은 서로 바뀐다. fixture에는 각각 main-axis gap과 cross-axis gap을 실제 줄바꿈 결과에 반영하는 관찰점이 있어야 한다.
- `align-content: stretch`는 여러 줄의 cross-axis 크기를 남은 공간에 따라 바꿀 수 있다. reference는 줄 크기 합과 gap 합을 컨테이너의 cross size와 같게 고정해 free space를 0으로 만든다. 이 가정이 관찰값과 다르면 해당 case를 수정하고 reference를 새 기준으로 검토한다.
- `flex-wrap`은 flex container에만 레이아웃 의미가 있다. 여섯 runtime Flex profile 안의 Block 요소는 computed value를 보존하되 geometry에 영향 주지 않아야 한다. C10.1 밖 profile은 author CSS를 성공한 layout인 것처럼 내보내지 않는다.
- `align-items:stretch`는 item cross-size를 늘려 line size를 간접 변경할 수 있다. C10.1은 이미 있는 `flex-start`를 명시해 line collection과 item stretch를 섞지 않는다.
- empty container는 line이나 child frame을 만들지 않으며, single-item container는 불필요한 두 번째 line을 만들지 않는다. `display:none` child는 line collection에서 빠지고 그 subtree frame은 0이어야 한다.
- `flex-wrap`은 상속되지 않는다. 부모가 `wrap`이어도 별도 nested flex container의 초기값은 `nowrap`이어야 한다. 이 검증은 두 container의 크기를 고정해 intrinsic sizing과 분리한다.
- 유효한 CSS 값인 `wrap-reverse`, `order`, `align-content`를 지원 구현 없이 통과시키는 것은 실패다. 프로파일별 거부 경로와 runtime 오류 뒤 새 scene/frame을 성공으로 공개하지 않는 조건을 고정한다.
- Taffy의 enum 매핑만으로 완료하지 않는다. 여섯 runtime Flex profile의 Rust projection 비교, Chrome reference, Android/iOS에서 기본 앱의 사용자 지정 속성 paint profile로 실행한 V8 runtime report와 PR 증거가 모두 일치해야 C10.1을 완료 표시한다. simulator는 실제 기기·하드웨어 GPU 성능 증거가 아니다.
- 계획 검토 20개 관점은 이 계획의 병합 전에 실행한다. C10.1 기능 구현 뒤에는 이 표를 재사용하지 않고 구현 코드·오류·platform path에 대한 새 20개 관점을 별도로 기록한다.

## 변경·명세 경계

- C10.1은 공개 CSS API를 추가하거나 전체 Flexbox 지원 완료를 선언하지 않는다. 앱 작성자 지원 표는 이 runtime slice가 제품 API로 승인될 때 별도 검토한다.
- 기존 profile 값, fallback, 오류 우선순위와 render revision tuple을 변경하지 않는다. 지원 profile의 새로운 wrap input만 해당 layout snapshot을 바꾼다.
- C10.1만으로 C10 parent, S02, 전체 CSS, border paint 또는 C09.4를 완료 처리하지 않는다.

## C10.2 · flex basis와 유연 크기 배분

C10.2의 독립 계획과 범위·fixture·완료 조건은 [C10.2 전용 계획](c10-2-flex-distribution.md)에 둔다. 이 단계는 C10.1이 정한 줄에 대해 CSS Flexbox의 flex base/hypothetical main size, grow·scaled shrink 배분, 명시 min/max clamp와 freeze 재분배를 검증한다. 자동 최소 크기·내재 크기·indefinite percentage와 콘텐츠 기반 basis는 C10.4 및 C14 선행 작업으로 남긴다. 계획 PR #115는 별도 실패 관점 검토 뒤 병합했다. 구현 브랜치에서 고정 Chromium 27개 case·92개 node와 Rust·cascade 검사를 통과했고, Android·iOS Simulator 실제 V8 실행의 다섯 DOM frame과 WGPU 제출을 확인했다. 구현 실패 경로 검토도 마쳤다. 기능 구현 PR은 병합 전이며 PR 생성·리뷰·병합 뒤 공식 완료 상태를 반영한다.

## 기준 자료

- [W3C CSS Flexible Box Layout Module Level 1 §5.2 `flex-wrap`](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property), [§5.3 `flex-flow`](https://www.w3.org/TR/css-flexbox-1/#flex-flow-property), [§6 flex lines](https://www.w3.org/TR/css-flexbox-1/#flex-lines), [§9.2 line length determination](https://www.w3.org/TR/css-flexbox-1/#line-sizing). 이 표준은 동작 계약의 기준이며 현재 문서 상태는 Candidate Recommendation Draft다.
- [W3C CSS Gaps Module Level 1](https://www.w3.org/TR/css-gaps-1/)의 Flexbox main-axis/cross-axis gutter 정의를 row·column gap 관찰에 적용한다.
- [Taffy 0.14.0 문서](https://docs.rs/taffy/0.14.0/taffy/)와 저장소에 잠긴 crate source의 `style/flex.rs::FlexWrap`·`compute/flexbox.rs`를 확인한다. Taffy 구현은 후보 계산기이며 최종 수치 oracle은 고정 Chromium reference다.
