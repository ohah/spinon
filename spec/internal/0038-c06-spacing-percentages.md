# 0038 · C06.2 백분율 margin·padding·gap

- **상태:** 내부 구현 계약 · 공개 웹 API 지원 선언 아님
- **내부 계약 숫자 버전:** `0.1.0` 고정 · 출시 또는 호환성 정책을 사용자가 정하기 전 숫자를 올리지 않는다.
- **문서 식별자:** `0038`은 내부 명세 일련번호이며 제품·계약 버전이 아니다.
- **기준:** Chromium `154.0.8037.98` · Taffy `0.14.0` · Stylo `0.22.0`
- **작업 계획:** [C06 값·단위 변환](../../plan/c06-value-unit-conversion.md#c062--spacing-percentages-계획)
- **계획 공격 검토:** [실패 경로 20개](evidence/c06-2-spacing-plan-review-2026-10-10.md)
- **구현 공격 검토와 실행 근거:** [C06.2 검증 기록](evidence/c06-spacing-percentages-implementation-2026-10-10.md)

## 범위

현재 runtime Flex author-style profile에서 계산하는 `margin`, `padding`, `gap`, `row-gap`, `column-gap`의 percentage를 Stylo typed computed value에서 Rust layout 입력과 Taffy까지 보존한다. 대상은 `display:flex`, `flex-wrap:nowrap`, `writing-mode:horizontal-tb`, `direction:ltr|rtl`이며 새 CSS property allowlist를 추가하지 않는다.

이 계약은 공개 JavaScript `CSSStyleDeclaration`, `getComputedStyle()`, 일반 DOM/CSSOM, 전체 HTML/CSS 지원을 제공하지 않는다. Grid, flex-wrap, inline formatting, vertical writing mode, margin collapse, nonzero border 모델, 외부 stylesheet 자원 로더, CSS math와 viewport/container 단위는 범위 밖이다.

## 내부 값과 계산 기준

Stylo `ComputedValues`에서 `ComputedCssSpacingValue`로 변환할 때 serialization 문자열을 다시 파싱하지 않는다. `LengthPx(f32)`와 `Percentage(f32)`는 계산 의미를 보존한다. percentage는 비율로 저장하므로 `10% = 0.1`; 값은 0–1로 제한하거나 clamp하지 않는다. unsupported computed variant는 속성·NodeId를 보존한 오류로 닫는다.

| 속성 | percentage 기준 | 허용 값·동작 |
|---|---|---|
| 네 physical `margin-*` edge | containing block의 logical width | 음수·0·분수·100% 초과 허용. `auto`는 미지원 오류. |
| 네 physical `padding-*` edge | containing block의 logical width | 최종 CSS 값은 nonnegative. invalid authored/`var()` 값은 Stylo cascade 결과를 사용한다. |
| `column-gap` | Flex container content-box width | finite nonnegative. `normal`은 Flex used value `0`. |
| `row-gap` | Flex container content-box height | finite nonnegative. nowrap row Flex에서 줄 간격은 생기지 않는다. |

`horizontal-tb`에서 모든 physical margin/padding edge의 percentage 기준은 containing block width다. `direction:ltr|rtl`은 logical inline side가 physical edge에 연결되는 위치만 바꾼다. `margin`/`padding` shorthand의 1–4개 값, logical block/inline shorthand 및 longhand, `gap` 1–2개 값과 각 longhand는 Stylo cascade 결과를 따른다. stylesheet shorthand와 inline longhand의 우선순위도 Stylo가 결정한다.

Flex gap은 각 축의 content-box 크기를 기준으로 한다. row Flex의 주축은 width와 `column-gap`, column Flex의 주축은 height와 `row-gap`이다. nowrap에서 사용되지 않는 교차축 gap은 시각적인 줄 간격을 만들지 않는다. `var()`가 Stylo에서 percentage typed value로 계산되면 해당 percentage를 보존한다. `calc()` 등 현재 typed DTO가 표현하지 못하는 값은 추정하지 않는다.

## definite basis와 오류

백분율 spacing은 layout 전에 containing block basis가 definite임을 tree에서 증명해야 한다. root viewport, 고정 크기 및 definite ancestor를 통과한 percentage 크기, definite block containing block 아래의 block-flow auto width, definite Flex parent의 stretch 교차축 크기를 지원한다. 이 규칙은 현재 nowrap Flex/display profile에만 적용한다.

| 조건 | 결과 |
|---|---|
| edge percentage에 definite containing block width가 없음 | `IndefinitePercentageBasis { node, property, axis: "containing block width" }` |
| row/column main-axis percentage gap의 해당 축이 indefinite 또는 순환 | `IndefinitePercentageBasis { node, property, axis }` |
| root에 percentage gap | `UnsupportedRootPercentageSpacing { node, property }` |
| percentage root margin/padding가 viewport 계약으로 증명되지 않음 | `IndefinitePercentageBasis` |
| `margin:auto`, `calc()`·anchor 함수 등 지원하지 않는 computed value | `UnsupportedComputedValue { node, property, value }` |
| 내부 layout API의 margin `NaN`/무한대, padding/gap의 음수·비유한 값 | `InvalidStyle { node, field }` |
| 결과 frame의 좌표·크기가 비유한 값 | layout 결과 거부; 일부 frame을 제출하지 않음 |

`display:none` subtree는 basis 검증과 layout 사용에서 제외한다. signed margin만 음수 입력을 허용하고, padding 및 gap 입력은 nonnegative다. `0%`는 내부 DTO에서 `0px`와 합치지 않는다. 단, Chromium이 CSSOM used value로 두 값을 같은 문자열로 직렬화하면 typed 값과 frame을 별도로 검증한다.

## 런타임 연결

`spinon-style`은 margin·padding·gap의 Stylo typed 값을 계산된 element snapshot에 저장한다. `spinon-style-to-layout`은 CSS px·percentage·`auto`·`normal`·unsupported variant를 속성별 규칙으로 projection한다. `spinon-layout`은 `LayoutEdges`·`LayoutGap`과 `LayoutLengthPercentage`를 검증하고, 퍼센트는 Taffy `LengthPercentage`/`LengthPercentageAuto`에 직접 전달한다. JS/V8/Stylo 객체 포인터는 layout 계층에 저장하지 않는다.

기존 C04의 CSS px spacing은 length variant로 계속 전달한다. `RootSizingPolicy::MatchViewport` 및 C06.1 viewport/revision 계약은 변경하지 않는다. authored CSS의 parse/computed invalidation은 Stylo가 먼저 처리하고, 내부 DTO에 직접 전달된 잘못된 값은 layout 경계에서 거부한다.

## 검증 범위와 한계

- 고정 Chromium 기준은 78개 노드를 보유한다. Rust 비교 대상 71개 노드의 `x`, `y`, `width`, `height`를 각각 노드별 최대 절대 오차 `0.5 CSS px` 안에서 비교한다. `margin:auto` 1개, cyclic gap 3개, `inline-flex` oracle-only 3개는 지원 geometry 집계에서 제외하고 별도 오류·미지원 기대를 확인한다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator는 동일 7-node V8 fixture를 실행한다. 확인 항목은 `status=0`, `layout=ready`, `boxes=7`, WGPU `presented` 및 장면 캡처다. 이 화면은 78-node 데스크톱 oracle 비교를 대신하지 않고 실기기·hardware GPU 성능을 증명하지 않는다.
- 실행 기록은 [C06.2 구현·실행 근거](evidence/c06-spacing-percentages-implementation-2026-10-10.md)에 둔다. 구현 공격 검토는 계획 공격 20개를 재사용하지 않고 코드·runtime의 새 실패 경로 20개로 작성한다.
- 현재 브랜치에서 C06.2 구현 검토가 끝나도 C06 상위와 C06.4–C06.6은 미완료다. C06.3은 별도 [계약 0039](0039-c06-absolute-lengths.md)로 관리한다. 이 문서만으로 CSS 전체 지원 또는 출시 적합성을 주장하지 않는다.
