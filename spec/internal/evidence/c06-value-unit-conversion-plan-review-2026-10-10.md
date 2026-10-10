# C06 값·단위 변환 계획 공격 검토

검토 대상은 [`c06-value-unit-conversion.md`](../../../plan/c06-value-unit-conversion.md)다. C06.1 구현에 들어가기 전에 값 의미·Taffy 경계·CSS percentage 기준·범위 표현을 서로 다른 실패 경로 20개로 대조했다. Taffy 세부 동작은 저장소가 고정한 `0.14.0` 소스를 기준으로 확인했다.

| # | 공격 관점 | 확인한 실패 가능성 | 계획에 반영한 방어 |
|---:|---|---|---|
| 1 | CSSOM/Stylo 문자열이 같은 단위 의미라고 가정하는가 | 실제 고정 비교에서 Chrome `getComputedStyle(root).height`는 `800px`인데 Stylo typed computed value 직렬화는 `100%`다. used-value CSSOM 문자열을 Taffy 입력으로 쓰면 percentage 의미가 손실된다. | `ComputedValues` typed width/height/flex-basis를 `spinon-style` 안에서 CSS px/percentage/auto DTO로 바꾼다. CSSOM 문자열은 별도 observation·진단으로 남기고 final frame을 비교한다. |
| 2 | typed percentage를 CSS px로 낮추거나 Taffy 비율로 잘못 넘기는가 | Taffy `Dimension::percent`는 0–1 fraction이고 기존 `LayoutDimension`은 fixed float만 표현한다. | bridge의 `p%`를 `p / 100`으로 바꿔 별도 `Percent` variant에 넣고 50%·125% 기준으로 검증한다. |
| 3 | containing block의 content box와 padding을 잘못 계산하는가 | percentage basis는 부모 content size이며 box-sizing과 padding이 그 크기에 영향을 준다. 현재 layout DTO에는 border edge가 없다. | padding을 둔 content-box와 border-box fixture를 쓰고, nonzero border는 구현 지원 범위에서 제외한다고 명시한다. |
| 4 | height%가 definite/indefinite parent에서 같은 값이라고 가정하는가 | auto height의 percentage resolution은 definite height와 다르다. | auto-height containing block도 비교하고 Taffy 차이는 명시적 미지원 경계로 승격한다. |
| 5 | Flex row·column의 basis percentage가 항상 width를 기준으로 하는가 | flex-basis는 주축 기준을 따른다. | row/column별 definite main size와 frame을 별도로 검사한다. |
| 6 | `width`와 `flex-basis`가 함께 있을 때 width가 항상 크기를 결정한다고 가정하는가 | Flex base size·shrink/grow가 최종 폭을 바꿀 수 있다. | 충돌 값을 갖는 fixture로 basis, width, 최종 geometry를 함께 비교한다. |
| 7 | root `%`를 모든 레이아웃 생성자에 무조건 허용하는가 | core Tree 생성자는 root가 viewport와 일치해야 한다. | HostDocument viewport-containing-block 경로만 허용하고 `MatchViewport` invariant는 보존한다. |
| 8 | root를 항상 viewport 크기로 강제해 `50%`를 잘못 그리는가 | runtime root는 viewport 내부에서 CSS 계산된다. | root `100%`·`50%`를 runtime input 계약에서 검증한다. |
| 9 | percentage를 `[0,1]`로 clamp하는가 | CSS width는 100% 초과가 유효하다. | `125%`를 1.25로 보존하며 범위 clamp를 금지한다. |
| 10 | negative percentage를 layout까지 전달하는가 | width/height/flex-basis의 음수는 유효한 computed size가 아니다. | invalid declaration의 Stylo computed result와 adapter 직접 오류를 구분한다. |
| 11 | CSS computed `auto`를 0%나 고정 0으로 읽는가 | 기존 `auto`는 LayoutDimension의 독립 상태다. | Auto variant와 serialized output을 회귀 검사한다. |
| 12 | `calc(50% + 10px)`의 선행 숫자만 파싱하는가 | 문자열 접미사 파서는 calc 문장을 오해할 수 있다. | C06.1에선 전체 토큰 문법만 받고 math를 명시 거부한다. |
| 13 | margin/padding/gap의 percentage를 width 규칙으로 처리하는가 | 해당 속성은 각기 기준 축·Taffy 입력 타입이 다르다. | C06.2로 분리하고 C06.1에서는 조용한 coercion 없이 실패시킨다. |
| 14 | box-sizing fixture가 없는 border/position 모델을 몰래 가정하는가 | 초안 fixture의 nonzero border와 absolute positioning은 현재 LayoutStyle/adapter가 표현하지 않아 runtime 대조가 불가능했다. | flow-only fixture로 고치고 padding이 있는 content-box·border-box를 비교한다. border/position의 미지원 경계를 남긴다. |
| 15 | viewport와 device scale을 혼동하는가 | CSS px는 Android dp/iOS point 및 backing pixels와 별개다. | 고정 CSS viewport로 비교하고 platform scaling은 별도 경계로 기록한다. |
| 16 | 분수 percentage 계산을 정수 반올림하는가 | 33.333% 등에서 누적 geometry drift가 생긴다. | fractional percentage와 각 좌표 0.5 CSS px 상한을 사용한다. |
| 17 | `NaN`, 무한대, 오버플로 percentage를 Taffy에 넘기는가 | 고정 float만 검증해도 새 enum 경로 검증이 누락될 수 있다. | Fixed·Percent 양쪽의 finite/nonnegative 검증과 직접 입력 실패 테스트를 둔다. |
| 18 | direction/layout revision/viewport 변경 뒤 old result를 재사용하는가 | percent의 basis는 viewport와 containing block geometry 영향을 받는다. | 같은 V8 fixture에서 새 revision의 computed style·frame을 확인하고 runtime 최신 revision 경계를 유지한다. |
| 19 | 중복 snapshot이나 cache가 typed value와 CSS string을 다른 revision으로 섞는가 | incremental cascade가 직렬화 output을 재사용하므로 typed 값이 profile/cache key와 함께 이동하지 않으면 불일치한다. | typed DTO를 `ComputedElementStyle`과 같은 revision·NodeId lifecycle에 포함하고 full-vs-incremental oracle equality에 추가한다. |
| 20 | C06.1 부분 지원을 C06 전체 완료로 보고하는가 | 남은 font/math/viewport/container 단위가 여전히 크다. | 공식 부모 C06을 미완료로 유지하고 C06.1 및 후속 경계를 각각 체크한다. |

## 반영 뒤 판단

C06.1은 percentage를 의미 보존 타입으로 전달하는 범위에 한정한다. CSS properties마다 다른 percentage basis를 한 번에 일반화하지 않고, 현재 Taffy profile에서 검증할 width·height·flex-basis만 다룬다. 나머지 C06 범위는 같은 계약이나 검토 수에 합산하지 않는다.

## 보완 검토 기록

첫 fixture 초안에 runtime profile에서 지원하지 않는 absolute positioning과 `LayoutStyle`이 표현하지 않는 nonzero border가 포함된 것을 확인했다. 그 상태로는 Chrome과 제품 HostDocument의 frame을 공정하게 비교할 수 없어 fixture를 normal flow로 바꾸고, padding이 있는 content-box/border-box만 남겼다. nonzero border는 C06.1의 지원 주장 밖에 명시했다. 추가로 Chrome used-value CSSOM과 Stylo typed computed-value serializer의 width/height 표현이 다름을 실제 캡처에서 확인해 문자열 일치 판정을 제거하고 typed value와 frame 비교로 바꿨다. 영향받는 fixture 적합성·box sizing·CSSOM serializer·지원 범위 관점을 다시 확인했다.
