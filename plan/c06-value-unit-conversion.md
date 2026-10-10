# C06 · CSS 값·단위 변환

- **문서 유형:** 구현 계획 · 공식 완료 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C06 값·단위 변환](../spec/STATUS.md#css-구현-체크리스트)
- **내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정
- **비교 기준:** 저장소가 고정한 Chromium CSS reference와 실제 `Taffy 0.14.0` 입력·출력
- **첫 구현 하위 항목:** C06.1 `width`·`height`·`flex-basis`의 백분율 값을 CSS px 계산 전에 Taffy percentage로 전달
- **계획 검토 근거:** [서로 다른 실패 경로 20개](../spec/internal/evidence/c06-value-unit-conversion-plan-review-2026-10-10.md)
- **C06.2 상태:** 구현 검토·Chromium 비교·Android/iOS Simulator 실행 완료 · 현재 작업 브랜치 미병합
- **C06.2 계약·근거:** [내부 계약 0038](../spec/internal/0038-c06-spacing-percentages.md) · [구현 검토와 실행 결과](../spec/internal/evidence/c06-spacing-percentages-implementation-2026-10-10.md)
- **C06.3 상태:** 구현 검토·Chrome/Rust 비교·Android/iOS Simulator 실행 완료 · 현재 작업 브랜치 미병합
- **C06.3 계약·근거:** [내부 계약 문서 ID 0039](../spec/internal/0039-c06-absolute-lengths.md) · [전용 계획](c06-3-absolute-lengths.md) · [계획 검토](../spec/internal/evidence/c06-3-absolute-lengths-plan-review-2026-10-10.md) · [구현 검토와 실행 결과](../spec/internal/evidence/c06-absolute-lengths-implementation-2026-10-10.md)
- **C06.4 상태:** 구현·Chromium/Rust 비교·전체 검사·Android/iOS Simulator 실행 완료 · 현재 작업 브랜치 미병합
- **C06.4 계획·비교·검토:** [글꼴 상대 길이 단위 계획](c06-4-font-relative-units.md) · [내부 계약 0040](../spec/internal/0040-c06-font-relative-units.md) · [구현 전 Chromium 비교](../spec/internal/evidence/c06-4-font-relative-units-precomparison-2026-10-10.md) · [계획 실패 경로 검토](../spec/internal/evidence/c06-4-font-relative-units-plan-review-2026-10-10.md) · [구현 실패 경로 검토·실행 근거](../spec/internal/evidence/c06-font-relative-units-implementation-review-2026-10-10.md)

## 목표와 경계

C06은 CSS 값에 적힌 단위와 계산식을 해당 속성의 CSS 계산 기준에 맞춰 layout 입력으로 전달한다. Stylo가 cascade와 computed value를 소유하고, 이 어댑터는 확인된 computed value를 의미를 잃지 않는 Taffy 입력으로 바꾼다. CSS 길이를 임의로 화면 픽셀·Android dp·iOS point로 해석하지 않는다. Taffy layout은 CSS px 좌표를 반환하며 플랫폼 배율 변환은 이후 렌더링 경계의 별도 책임이다.

C06을 한 번에 완료로 표시하지 않는다. 다음 하위 항목을 각각 Chromium 비교·Rust full/typed oracle·런타임 검증과 함께 구현하고, 전체 범위가 끝날 때까지 상위 C06은 미완료로 둔다.

| ID | 구현 범위 | 선행·경계 |
|---|---|---|
| C06.1 | `width`·`height`·`flex-basis`의 `%` 값을 Taffy `Dimension::percent`로 전달 | 이 계획의 첫 구현이다. `%`를 길이로 조기 환산하지 않는다. |
| C06.2 | `margin`·`padding`·`gap`의 `%`와 속성별 기준 축·자동 크기 containing block 동작 | Taffy의 계산 기준이 CSS와 일치하는지 각 속성별로 확인한다. |
| C06.3 | 절대 길이 단위(`in`, `cm`, `mm`, `Q`, `pt`, `pc`)와 `px`의 CSS px 변환 | [전용 계획](c06-3-absolute-lengths.md): 96 CSS px/in, 단위별 Chrome computed value와 runtime fixture를 고정한다. |
| C06.4 | `em`·`rem`과 글꼴 상대 단위·글꼴 크기 계산 | `ex`·`ch`·`cap`·`ic` 등 실제 font metric 의존 값은 폰트 제공·측정 기준이 준비되지 않으면 지원을 가장하지 않는다. |
| C06.5 | `calc()`·`min()`·`max()`·`clamp()`의 typed 계산과 부호·차원·non-finite·오류 처리 | [전용 계획](c06-5-typed-css-math.md) · [Chromium 사전 비교·Stylo DTO 근거](../spec/internal/evidence/c06-5-typed-css-math-precomparison-2026-10-10.md) · 현재 브랜치에서 Stylo DTO, Taffy resolver, Rust/runtime fixture까지 구현·검증했다. Stylo 내부 포인터는 Taffy compact calc tree에 저장하지 않는다. |
| C06.6a | native viewport `vw`/`vh`·`vi`/`vb`·`vmin`/`vmax` 및 small/large/dynamic 변형 | [전용 계획](c06-6-viewport-units.md) · [내부 계약 0042](../spec/internal/0042-c06-viewport-units.md) · 현재 브랜치에서 Chromium/Rust 비교와 Android/iOS Simulator 실행 완료, 미병합. `0042`는 문서 ID이며 내부 숫자 버전은 `0.1.0`이다. |
| C06.6b | container-relative length 단위 | C21에서 eligible query container, 축별 size ownership, cycle 및 stale 결과 규칙을 확정한 뒤 별도 계획·fixture로 구현한다. 현재는 Stylo fallback을 성공처럼 노출하지 않고 fail-closed 한다. |

각 하위 항목은 구현 전 Chrome 기준을 고정하고 별도 계획 공격 검토 20개, 구현 뒤 독립된 실패 경로 검토 20개를 남긴다. 한 하위 항목 검토 결과를 다른 하위 항목의 검토로 세지 않는다.

## C06.1 계약 초안

### 입력 값

- cascade 결과는 화면용 computed-style 문자열을 다시 파싱하지 않는다. `spinon-style`이 Stylo `ComputedValues`의 typed `width`, `height`, `flex-basis` 값에서 CSS length/percentage/auto를 별도 bridge DTO로 추출한다. 다른 crate에는 Stylo 객체나 내부 포인터를 넘기지 않는다.
- typed `auto`는 기존 `LayoutDimension::Auto`로 유지한다.
- finite percentage `p%`는 bridge DTO와 `LayoutDimension::Percent(p / 100)`으로 보존한다. `150%`를 `100%`로 자르지 않고, `0%`를 `0px`와 합치지 않는다.
- typed absolute length는 CSS px로 변환해 `LayoutDimension::Fixed(css_px)`에 전달한다. width·height·flex-basis는 문자열 parser를 거치지 않으며, typed bridge 값이 세 속성의 source of truth다.
- 음수·NaN·무한대 또는 parser가 허용하지 않은 computed 형식은 기본값으로 바꾸지 않는다. Stylo가 CSS invalid declaration에 적용한 계산값은 받아들이되, adapter가 직접 받는 잘못된 비율은 구체적인 `UnsupportedComputedValue` 또는 `InvalidStyle`로 실패한다.
- typed `calc()`·수식·fit-content 계열 keyword는 이 하위 항목에서 percentage 숫자처럼 해석하지 않고 `UnsupportedComputedValue`로 거부한다. 해당 문법은 C06.5 또는 별도 sizing 계약에서 다룬다.

### 기준 축과 범위

- 자식 `width`·`height` 비율은 definite containing block의 content size를 기준으로 한다. C06.1 fixture는 기존 layout adapter가 표현하는 padding과 content-box/border-box를 비교하되, `LayoutStyle`에 border edge 모델이 아직 없어 nonzero border는 넣지 않는다. nonzero border가 있는 문맥은 이 하위 항목의 지원 판정에서 제외하고 후속 CSS/layout 계약으로 남긴다.
- Flex `row`의 percentage `flex-basis`는 definite main size인 content width, `column`은 definite main size인 content height를 기준으로 한다. width와 basis가 충돌할 때 Flex 규칙으로 어느 입력이 크기를 정하는지 fixture에서 비교한다.
- HostDocument root의 `%`는 `from_host_document_with_viewport_containing_block` 경로에서 viewport containing block에 대해 계산한다. 기존 코어 `Tree` 경로의 `MatchViewport` 정책과 고정 viewport root 검증은 바꾸지 않는다.
- containing block의 크기가 indefinite하거나 percentage가 순환하는 경우 Chrome의 계산·used value를 고정 fixture로 확인한다. Taffy 결과가 다르면 해당 문맥을 지원 완료로 표시하지 않고, 별도 기준 제공 또는 명시적인 fail-closed 경계를 계획에 추가한다.
- `margin`·`padding`·`gap` percentage, 논리 축 속성, viewport/container 단위는 C06.1 구현 범위 밖이다. 이 값들이 기존 adapter에서 오류를 반환하는 동작을 유지한다.

### 내부 레이아웃 표현

- `LayoutDimension`에 percentage variant를 추가하고 Taffy 0.14.0의 `Dimension::percent`로 직접 투영한다. percentage는 `[0.0, 1.0]`으로 제한하지 않는다. CSS에서 허용되는 `>100%` 값을 보존한다.
- `spinon-style` 안에서 Stylo의 width/height/flex-basis typed value를 CSS px·percentage·auto DTO로 바꾼다. 이후 renderer와 layout crate는 Stylo typed object에 의존하지 않는다. computed-style 문자열은 진단·비교 용도로 계속 보존한다.
- 이 typed DTO는 내부 layout adapter 입력이며 공개 `getComputedStyle`·CSSOM API가 아니다. Chrome CSSOM의 used-value serialization과 Stylo typed computed value가 다를 때 typed 값은 레이아웃 입력 의미를 보존하고, 브라우저 호환 판정은 최종 geometry에서 한다.
- 기존 `Fixed`·`Auto` 동작과 serialized layout output은 유지한다. 새 enum variant는 Rust 내부 타입이며 공개 JavaScript API나 CSSOM을 만들지 않는다.
- Taffy root layout에 viewport `AvailableSpace::Definite`가 전달되는 경로를 그대로 둔다. Core Tree의 root 고정 크기 invariant를 percentage 도입을 핑계로 제거하지 않는다.

## 구현 전 기준 fixture

새 고정 fixture는 다음 조건을 포함한다.

1. definite content-box 부모의 `50%` child width/height와 padding을 포함한 border-box 부모의 대응 case. 현재 projection이 border를 모델링하지 않으므로 fixture에 nonzero border는 사용하지 않는다.
2. `125%`, `0%`, fractional percentage, `auto`, 음수 percentage declaration의 computed value와 rectangle.
3. definite row·column Flex main size에서 `flex-basis: 50%`; width 기반과 basis 기반이 다를 때 shrink/grow 결과.
4. definite 및 auto-height containing block의 child percentage height 동작. fixture는 제품 profile에서 지원하지 않는 `position` 속성을 쓰지 않는다.
5. HostRoot `width: 100%`와 자식 `width`·`height: 50%`의 viewport/containing-block 결과 및 기존 core root sizing 정책.
6. unsupported `calc(50% + 10px)`, percentage margin/padding/gap의 명시 실패 기대.

Chromium `154.0.8037.98`, 실행 파일 revision/hash, `320×800 CSS px` viewport, scale, locale, time zone, fixture/hash와 캡처 도구 hash를 근거에 기록한다. fixture는 runtime layout profile이 지원하지 않는 `position`과 nonzero border를 쓰지 않는다. CSSOM computed-style 문자열은 reference observation으로 정확 보존하되, 이를 Stylo `computed_value_to_string`과 모든 속성에서 동일해야 한다고 요구하지 않는다. 실제 대조에서 Chromium의 `getComputedStyle(root).height`는 `800px`인데 Stylo typed computed value와 그 직렬화는 `100%`였다. 이는 used-value CSSOM과 typed computed-value의 차이다. `width`·`height`·`flex-basis`는 Stylo typed DTO와 최종 frame을 비교하고, 나머지 공통 serialization만 정확 비교한다. 각 layout frame의 `x`, `y`, `width`, `height` 최대 절대 오차는 각각 `0.5 CSS px` 이하로 제한한다. 평균 오차로 한 노드의 불일치를 가리지 않는다. 기존 C01 layout reference의 `%` case는 교차 확인에 사용하되 새 runtime fixture를 대체하지 않는다.

## 구현·검증 순서

1. 이 문서의 범위·오류·축별 기준을 Chromium fixture와 Taffy 0.14.0 소스에 대조해 계획 검토 20개를 완료한다.
2. 구현 전 fixture·캡처 도구를 만들고 pinned Chromium의 computed value와 rectangle 기준을 저장한다.
3. `LayoutDimension::Percent`와 CSS percentage parser/projector를 추가한다. percentage variant가 실제 Taffy `Dimension::percent`까지 도달하는 것을 검증한다.
4. Rust style projection oracle과 Taffy 결과를 fixed Chrome snapshot에 대조한다. zero-percent/zero-length, indefinite basis, negative/over-100%, content-/border-box와 row/column을 따로 검사한다.
5. Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 3-node V8 fixture를 실행해 실제 V8 evaluation, `boxes=3`, CSS viewport root frame, WGPU present와 백분율 장면을 확인한다. 이 모바일 fixture는 22-node Chromium/Rust 수치 fixture와 다르며 child frame의 수치 비교로 주장하지 않는다. 플랫폼별 화면은 별도 저장하고, 실기기 성능·하드웨어 GPU 결과로 해석하지 않는다.
6. `cargo fmt`, workspace test, Clippy, FFI all-features check, CSS reference verifier, Android·iOS Simulator build/run을 실행한다.
7. 코드·실패 주입·실행 기록을 대상으로 계획 표와 겹치지 않는 구현 공격 관점 20개를 수행한다. 발견한 결함을 수정한 뒤 영향받는 검토 관점을 다시 실행한다.
8. C06.1–C06.6은 각각 구현·검증한 하위 항목만 완료로 체크한다. C06 상위는 전체가 끝날 때까지 미완료로 둔다. API 또는 내부 계약 문서·예제·오류·플랫폼 경계를 연결한다. 숫자 계약 버전은 출시 전 `0.1.0`을 유지한다.

## C06.1 완료 판정

- 22-node fixed Chromium fixture와 Rust HostDocument oracle의 지원 대상 typed size 의미 및 모든 좌표·크기가 정해진 허용 오차 안에서 일치한다. Chrome CSSOM과 Stylo serialization 사이에 확인된 used/computed 표기 차이는 결과 계약에 따로 남긴다.
- adapter가 percentage 정보를 fixed px로 미리 뭉개지 않고 Taffy의 percentage variant로 전달한다.
- invalid value, negative CSS declaration, zero %, `>100%`, `auto`, definite/indefinite basis, content-/border-box, row/column Flex가 각기 독립적으로 검사된다.
- Android·iOS는 같은 3-node JavaScript fixture로 실제 V8→HostDocument→Stylo→Taffy→WGPU 성공 경로, root CSS viewport frame, 세 box의 화면 표시를 확인한다. 이 확인은 22-node Chromium/Rust fixture의 수치 비교와 구분한다. 모바일 child frame별 수치 계측은 아직 제공하지 않는다. Chrome CSSOM 문자열과 Stylo computed-value 직렬화는 해당 경계가 일치하는 공통 속성에만 정확 비교한다. 캡처 fixture에 있지만 현재 profile에서 지원하지 않는 `calc()`·percentage margin/padding/gap 값은 Chromium observation only이며 제품 성공 경로로 포함하지 않는다.
- 지원하지 않는 margin/padding/gap percentage, CSS math, viewport/container values가 성공처럼 보이지 않는다.
- C06.1 내부 계약은 `0.1.0`이며 상위 C06은 계속 미완료다.

## 명시적 미포함

- percentage margin·padding·gap 및 그 속성별 기준 축
- `em`·`rem`, 글꼴 상대 단위와 폰트 metric
- C06.5의 범위를 벗어난 CSS math 및 미지원 함수
- viewport·dynamic viewport·container units
- text shaping, font fallback/loading, line breaking
- 외부 CSS loader, CSSOM, 일반 CSS 전체 지원, 실기기 성능

## C06.2 · spacing percentages 계획

- **상태:** 구현 검토·Chromium 비교·Android/iOS Simulator 실행 완료 · 현재 작업 브랜치 미병합
- **범위:** 기존 runtime Flex author-property allowlist에서 이미 받아들이는 `margin`·`padding`·`gap` 선언의 percentage 값을 typed layout value로 보존한다. 새 CSS property allowlist는 만들지 않는다. 대상은 `display:flex`, `flex-wrap:nowrap`, `writing-mode:horizontal-tb`이며 `direction:ltr/rtl`을 포함한다.
- **계획 공격:** [C06.2 계획 검토](../spec/internal/evidence/c06-2-spacing-plan-review-2026-10-10.md)
- **내부 계약:** [0038 · C06.2 백분율 spacing](../spec/internal/0038-c06-spacing-percentages.md)
- **구현 공격 검토와 실행 근거:** [20개 코드·runtime 실패 경로 및 검증 결과](../spec/internal/evidence/c06-spacing-percentages-implementation-2026-10-10.md)
- **기준 근거:** [CSS Box Model Level 3 Recommendation](https://www.w3.org/TR/css-box-3/) §3–4; [CSS Logical Properties and Values Level 1](https://www.w3.org/TR/css-logical-1/); [CSS Gaps Module Level 1 Working Draft](https://www.w3.org/TR/css-gaps-1/) §2.3; 고정 Chromium `154.0.8037.98` fixture; Taffy `0.14.0` 소스.

### CSS 의미와 판정 기준

| 속성 | 백분율 기준 | 값 경계 |
|---|---|---|
| `margin-*` physical/logical | containing block의 logical width. `horizontal-tb`에서는 네 physical edge 모두 폭 기준이다. | 음수 허용; `auto`는 미지원 오류; percentage 비율을 clamp하지 않는다. |
| `padding-*` physical/logical | containing block의 logical width. `horizontal-tb`에서는 네 physical edge 모두 폭 기준이다. | 음수 authored value는 cascade가 결정한 invalid-at-parse 또는 invalid-at-computed-value 결과를 사용한다. |
| `column-gap` | Flex container content box width | 음수 불가; Flex의 `normal` 사용값은 0; definite 기준만 지원한다. |
| `row-gap` | Flex container content box height | 음수 불가; `nowrap` row Flex에서는 줄 간격이 없으므로 시각 효과가 없고, column Flex의 main-axis 간격은 별도 판정한다. |

`margin`·`padding` 1–4개 값 shorthand, 각 physical side, 기존 허용 범위의 `*-block`·`*-inline` shorthand/longhand, `gap` 1–2개 값 shorthand, `row-gap`·`column-gap`을 CSSOM/Stylo cascade 결과에서 판정한다. stylesheet shorthand를 inline longhand가 덮는 순서도 포함한다. `direction:ltr/rtl`은 logical inline side의 physical 매핑만 바꾸고 containing-block percentage 기준을 바꾸지 않는다. `var()`가 Stylo에서 최종 `LengthPercentage`로 계산된 값은 보존한다. `calc()` 및 계산 결과 타입이 지원 DTO에 없는 값은 C06.5까지 명시적으로 거부한다.

CSS Box Model Recommendation은 margin/padding percentage를 containing block의 logical width에 연결한다. pinned fixture에서 네 edge와 logical side를 모두 측정한다. `box-sizing:border-box` 부모의 padding을 둬 child spacing percentage가 부모 border-box가 아닌 content width를 사용하는지도 확인한다. CSSWG가 설명한 Flex margin/padding의 과거 엔진 차이는 별도의 경고 근거이며, 제품의 호환성 판단은 여기서 고정한 Chrome geometry와 지원 profile에 한정한다.

CSS Gaps Working Draft는 gap percentage의 기준을 해당 content-box 차원으로 정의하고, Flex의 cyclic percentage는 0으로 해석한다고 기술한다. pinned Chrome 154 fixture에서 definite row/column content box, block-flow의 auto-width row Flex, definite parent에 의해 교차축 stretch된 auto-height column Flex를 각각 측정한다. 두 auto-size 사례는 각각 32px·12px gap을 사용한다. 반면 block-flow auto-height column Flex는 `row-gap:10%` computed value를 유지하면서 0px gap을 쓴다. Taffy `0.14.0`은 auto main size를 정한 뒤 main-axis percentage gap을 재해석하므로 이 마지막 경로는 그대로 넘기면 Chrome과 다를 수 있다.

C06.2는 tree에서 percentage basis의 확정 경로를 검증한다. root viewport는 definite, 고정 크기와 definite ancestor를 거친 percentage 크기는 definite, definite containing block의 block-flow auto width와 definite Flex parent의 stretch cross-size도 definite로 인정한다. **Intrinsic main size에 순환으로 매달린 gap 또는 위 경로로 증명하지 못한 basis는 layout 전 `node + property + axis` 오류로 닫는다.** definite basis의 주축 gap과 nowrap에서 사용되지 않는 cross-axis gap percentage는 지원한다. `inline-flex` 자동 크기 관찰은 Chrome 전용 참고값이며 현재 display profile 밖이므로 성공 기대나 지원 주장에 넣지 않는다.

CSS Gaps 문서는 Working Draft이므로 이를 단독 상호운용 보증으로 간주하지 않는다. Chromium oracle fixture는 definite/auto-size 경계를 구분한다. Chrome 154의 oracle-only `inline-flex` 케이스는 intrinsic width 40px 뒤 최종 child gap 4px를 관찰하지만, 그 경로는 Spinon의 현재 display profile이 아니므로 C06.2의 gap policy를 일반화하는 데 쓰지 않는다.

### Typed layout 계약

- `LayoutEdges`와 `LayoutGap`의 물리 길이 `f32`를 `LengthPx` / `PercentFraction` typed value로 바꾼다. C04 px 입력은 길이 variant를 써 같은 좌표를 유지한다. 퍼센트는 `100% = 1.0`이며 `[0,1]`에 clamp하지 않는다.
- margin은 signed finite 값, padding과 gap은 finite nonnegative 값만 layout 입력으로 받는다. authored percentage 자체가 유한해도 최종 계산이 비유한 geometry를 만들면 layout commit을 거부한다.
- Stylo `ComputedValues`의 margin/padding/gap typed value를 DTO로 추출한다. computed string 재파싱으로 percentage 축·부호를 추론하지 않는다.
- `auto` margin, cyclic/indefinite main-axis gap, containing-block width를 확정할 수 없는 margin/padding, root spacing, `calc()` 등 지원 타입 밖 값은 속성·NodeId·축을 담은 오류로 닫는다. Taffy의 `None => 0` 경로로 조용히 바꾸지 않는다.
- Stylo의 invalid-at-parse 값과 `var()` 치환 뒤 invalid-at-computed-value 값은 서로 다른 cascade 결과를 따른다. 이전 값 유지와 initial value 복귀를 혼동하지 않는다. `normal` gap은 Flex used value 0으로 normalize한다.
- root의 `MatchViewport` invariant와 C06.1 `CssViewport`/revision 계약은 바꾸지 않는다. percentage padding은 일반 element에서만 검증하며 document root 동작은 별도 계약이 정해질 때까지 제외한다.
- margin collapse, nonzero border, vertical writing mode, Grid/multicol, `flex-wrap`, inline formatting, external CSS loader, CSSOM, CSS math, text measurement는 이 slice의 성공 범위가 아니다. unsupported fixture는 명시 거부를 기대한다.

### 계획 검증 fixture

계획 구현 전 비교모델은 [78-node Chromium reference](../tests/fixtures/css/references/c06-spacing-percentages-v1.json)이며 입력 목록은 [inventory](../tests/fixtures/css/c06/spacing-percentages-inventory.json)로 고정한다. pinned Chrome capture는 실행 파일·revision·fixture·capture helper의 SHA-256과 환경을 저장한다.

1. 네 physical margin/padding edge, 1–4 값 shorthand, stylesheet→inline longhand override, logical block/inline shorthand의 LTR/RTL physical 매핑.
2. containing block width 200px·height 120px와 border-box width 260px·padding 30px의 대조로 세로 edge도 content width 200px를 기준으로 하는지 확인.
3. signed negative·fractional·0%·125% spacing, shorthand `gap` 1/2 값, longhand override와 분수 33.333% 관찰.
4. definite row Flex `column-gap` 및 definite column Flex `row-gap`가 부모 padding을 제외한 content-box width/height를 쓰는지 확인. no-wrap row의 row-gap은 실제 줄 간격이 아니므로 지원 값과 시각 효과를 혼동하지 않는다.
5. `gap:normal` 및 negative literal declaration의 parse-time 무효 처리, `var()` 퍼센트 치환, 치환 후 negative padding/gap의 computed-time 무효 결과.
6. auto-height column Flex main-axis cyclic gap, definite containing block에서 width가 늘어나는 block-flow auto-width row Flex, definite row parent의 stretch로 height가 정해지는 auto-height column Flex, inline-flex intrinsic sizing을 서로 구분한다. 첫 경로는 unsupported diagnostic, 다음 두 경로는 definite gap, 마지막은 oracle-only다.
7. `margin:auto`, unresolved containing block basis, root spacing, `calc()` 값은 현재 지원 프로파일 밖으로 남긴다. 이 fixture의 Chrome used-value serialization이 typed DTO 종류를 보증한다고 보지 않는다.

Viewport는 `320×800 CSS px`, DPR 1, locale `en-US`, timezone `UTC`, coarse pointer, light color scheme다. 각 지원 노드의 `x/y/width/height` 최대 absolute error는 축별 `0.5 CSS px`다. CSSOM 문자열은 관찰값으로 보존하되, `margin`/`padding`의 used px와 `gap`의 computed percentage 표현을 typed DTO와 혼합 비교하지 않는다.

### 구현 후 완료 관문

1. C06.1 percentage dimensions 및 기존 C04 px margin/gap fixtures가 그대로 통과한다.
2. Rust typed DTO가 Stylo 값 종류·finite/부호 조건·NodeId를 보존하고, definite percentage가 Taffy `LengthPercentage`까지 전달되는 것을 단위 fixture로 검사한다.
3. Chrome 78-node reference 중 지원 대상 71개 node와 Rust fixture의 typed spacing·각 frame을 노드별 비교한다. 이번 실행의 최대 절대 오차는 `0.01953125 CSS px`다. oracle-only, unsupported expected-error 행은 geometry pass 집계에서 제외하고 별도 판정한다.
4. Android API 37 emulator 및 iPhone 17 Pro/iOS 26.2 Simulator의 동일 V8 fixture에서 음수 margin, logical edge, percent padding, percent gap을 실제 V8→Stylo→Taffy→WGPU 경로로 적용한다. DOM frame, `boxes`, WGPU `presented`, 화면을 보존한다.
5. 시뮬레이터 화면은 데스크톱 수치 oracle을 대신하지 않으며 실기기·GPU 성능 근거로 부르지 않는다.
6. 계획 검토와 겹치지 않는 새 코드·런타임 실패 경로 20개를 검사하고, 수정한 실패 경로를 재검증한다. 검토 결과는 구현 근거 문서에 각 경로의 코드·테스트와 함께 기록한다.
7. [`0038` 내부 계약 문서](../spec/internal/0038-c06-spacing-percentages.md)에 지원 축·typed value·오류·미지원 경계·예제를 기록한다. 문서 ID를 제외한 내부 숫자 계약 버전은 `0.1.0`으로 유지한다.
8. 각 검증된 하위 항목만 체크한다. 현재 C06.1–C06.6a는 각각 별도 계약과 실행 근거로 관리하며, C06 상위와 C06.6b는 미완료로 둔다. 출시 전 숫자 계약과 crate 버전은 `0.1.0`으로 고정한다.
