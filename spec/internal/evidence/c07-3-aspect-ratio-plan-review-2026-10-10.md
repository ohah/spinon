# C07.3 종횡비 계획 실패 경로 검토

검토 대상은 [C07.3 계획](../../../plan/c07-3-aspect-ratio.md), 내부 계약 0045, 35-node Chrome 154 fixture/reference다. 기능 코드는 아직 이 계약으로 바꾸지 않았다. 계획 범위와 실제 precomparison을 서로 다른 실패 관점 20개로 대조했다.

| # | 독립 실패 관점 | 확인 결과와 반영 |
| --- | --- | --- |
| 1 | 작업이 출시 전 crate·제품 버전을 올리는가 | 계약과 계획을 `0.1.0` 고정으로 적었다. `0045`는 문서 ID이며 제품 버전이 아니다. manifest 변경은 계획에 없다. |
| 2 | `<ratio>`의 방향을 height/width로 뒤집는가 | Taffy 0.14.0 소스의 `width / height` 계약과 fixture `160×90 @ 16/9`를 맞췄다. |
| 3 | 분모 생략을 0 또는 미지원으로 처리하는가 | `1`, `2`, `4/3` 사례를 분리하고 CSS computed 값 및 frame을 보존한다. |
| 4 | fractional ratio를 정수 비율로 반올림하는가 | `1.5`와 `calc(16 / 9)` 사례를 넣었다. frame 비교 오차만 허용하고 비율 값을 정수로 바꾸지 않는다. |
| 5 | 계산형 숫자를 CSS 문자열 재파싱으로 구현하는가 | Stylo typed computed ratio만 입력으로 사용하고 `calc()` computed 결과도 고정 reference에 포함했다. |
| 6 | initial `auto`가 Taffy ratio 기본값과 다른 결과를 내는가 | 초기값과 명시 `auto`를 별개 node로 두어 둘 다 비율 없는 결과를 고정했다. |
| 7 | `auto <ratio>`를 bare ratio와 동일하게 축약하는가 | content-box 의미가 별도임을 명세하고 이 slice에서는 명시적 오류로 거부한다. positive layout fixture에는 섞지 않는다. |
| 8 | CSS에서 허용하는 zero degenerate ratio를 invalid CSS로 오인하는가 | Chrome은 `0/1`, `1/0`, `0/0` computed ratio를 유지하면서 empty box를 0 높이로 배치했다. 내부 adapter는 이를 무비율로 취급한다. |
| 9 | finite CSS 숫자가 f32에서 0 또는 무한대로 바뀌는가 | 변환 전후의 finite positive 조건과 overflow/underflow 거부를 계약에 명시했다. 구현 review에서 경계 입력을 따로 주입한다. |
| 10 | width와 height가 모두 definite인데 ratio가 지정 크기를 덮는가 | `160×80`와 `16/9` 사례에서 Chrome은 `160×80`을 유지한다. |
| 11 | block의 width/height auto에서 preferred ratio 계산을 생략하는가 | 빈 block의 stretch width 300과 16/9 결과 높이 168.75를 기준에 기록했다. |
| 12 | `box-sizing:content-box`와 `border-box`에 같은 box 기준을 적용하는가 | 같은 2:1 ratio에서 content-box `120×70`, border-box `120×60` 결과를 별도 사례로 캡처했다. |
| 13 | padding·border를 ratio 안팎에 중복 가산하는가 | content-box/border-box에 padding과 2px border 조합을 각기 분리했다. 지정 크기와 outer rect 모두 기록한다. |
| 14 | min/max 제약을 ratio보다 먼저 무시하거나 결과를 다시 비율에 맞추는가 | 네 축의 min/max 사례와 Flex child min/max 사례를 고정한다. Chrome 결과를 축마다 대조하며 수식으로 일반화하지 않는다. |
| 15 | Flex 기본 stretch가 비율을 항상 보존한다고 가정하는가 | 실제 Chrome에서 row child `100×100`, column child `100×100`으로 ratio 2:1이 깨진다. 기본 stretch와 `align-items:flex-start` 결과 `100×50`·`200×100`을 모두 fixture에 넣었다. |
| 16 | Flex row와 column에서 교차축을 반대로 적용하는가 | 각 축별 container·child를 따로 두고 property 및 geometry를 기록한다. |
| 17 | 부모 aspect ratio가 자식으로 상속되는가 | `ratio-parent`와 `ratio-not-inherited`를 따로 관찰했다. 자식 computed 값은 `auto`, 빈 child 높이는 0이다. |
| 18 | `display:none` node에서 ratio가 보이는 것과 geometry를 혼동하는가 | computed ratio는 기록하되 rect는 모두 0인 별도 사례를 유지했다. |
| 19 | `var()` invalidation 또는 invalid declaration이 이전 winning value를 잃는가 | custom property ratio와 뒤따르는 invalid negative declaration을 각각 넣어 `4/3`·`2/1` winner를 캡처한다. runtime 변경·cache invalidation은 구현 단계에서 별도 테스트한다. |
| 20 | reference가 바뀌거나 node 누락·DPR 차이가 평균에 숨는가 | Chrome revision, inventory/HTML/capture/helper digest, 35개 고유 ID, DPR 1·2 동등성을 capture 계약에 둔다. 모든 rect field를 node별 최대 오차로 판정하며 기준 불일치는 비교 실패다. |

## 실제 사전 관찰

- Oracle: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- viewport `400×1200` CSS px, DPR 1·2, locale `en-US`, timezone `UTC`; 35개 고유 node를 캡처했다.
- 계산의 대표 관찰값: `160px`+`16/9` → `160×90`; empty block auto width `400px`+`16/9` → `400×225`; content-box 2:1/padding 10 → outer `120×70`; border-box 2:1/padding 10 → `120×60`.
- 기본 Flex stretch는 ratio를 깰 수 있다. `align-items:flex-start`에서는 row의 `100×50`, column의 `200×100`으로 ratio를 반영한다.
- zero numerator/denominator는 Chrome computed output에서 degenerate 형태로 남지만 빈 상자의 선호 비율로 크기를 만들지 않는다.
- 초기 `aspect-ratio:auto`와 명시 `auto`는 빈 width-100 block의 height 0을 만든다. 부모의 2:1은 자식 computed 값에 상속되지 않는다.

계획 검토에서 발견한 관찰은 fixture와 계획에 반영했다. 이는 Taffy/Stylo 구현의 적합성 검증이 아니며 제품 코드는 이 기준에 따라 아직 변경되지 않았다.
