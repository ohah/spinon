# C10.3.5 paint phase 계획 보정 검토

**검토일:** 2026-10-11 · **범위:** Chrome 154 기준 캡처, pinned WPT `flexbox-paint-ordering-003.html`, 보정된 C10.3.5 paint 계약 · **성격:** 계획 오류 정정 후 독립 실패 경로 검토. 기능 구현이나 WPT 실행 증거가 아니다.

최초 계획은 absolute Flex 자식을 `order:0` paint rank처럼 처리했다. 고정 Chrome 겹침 관측과 WPT source가 그 가정이 틀렸음을 보였다. absolute 자식은 Flex item이 아니며 authored `order`는 paint 순서에 영향을 주지 않는다. 구현에 앞서 계획과 fixture 기준을 보정했다.

| # | 실패 경로 질문 | 확인 결과 | 계획·기준 반영 |
| ---: | --- | --- | --- |
| 1 | absolute child가 Flex item으로 분류되어 `order` 정렬에 섞이는가 | WPT 설명은 두 positioned child가 absolute이므로 Flex item이 아니며 `order`가 적용되지 않는다고 명시한다. | `order` 보존과 paint 영향 여부를 분리했다. |
| 2 | in-flow item `order`가 paint 순서에 적용되지 않는가 | Chromium 겹침 fixture에서 order `-1`, `0`, `1`의 paint stacking이 그 순서를 따른다. | in-flow item에만 order-modified order를 적용한다. |
| 3 | absolute child의 큰 양수 `order`가 다른 absolute child보다 위에 그려지는가 | Chromium은 이 값을 무시했다. pinned WPT도 서로 다른 absolute `order` 값이 source order를 바꾸지 않는 사례다. | authored 값은 computed style로 기록하고 paint comparator에서는 제외한다. |
| 4 | absolute child의 작은 음수 `order`가 먼저 그려지는가 | Chromium probe는 source order와 positioned phase를 유지했다. | 음수와 양수 `order`를 가진 absolute sibling을 함께 둔다. |
| 5 | positioned `z-index:auto` box가 in-flow item보다 먼저 그려지는가 | 겹침 probe에서 positioned child가 in-flow Flex item 위에 나타났다. CSS 2 §E.2의 positioned auto/zero 단계와 일치한다. | `z-index:auto` positioned phase를 in-flow paint 뒤에 둔다. |
| 6 | positioned 형제 사이 source tree 순서가 뒤집히는가 | 두 absolute box의 top-to-bottom 결과는 source order의 역순으로 관측됐다. | 내부 paint order는 source preorder를 유지한다. |
| 7 | Flex의 reverse 방향이 positioned phase의 tree order까지 역전하는가 | static-position 좌표와 positioned paint는 서로 다른 기준이다. | row/column reverse 배치 결과를 paint 순서로 재사용하지 않는다. |
| 8 | DOM 유사 child 순서를 paint 순서로 바꾸는가 | HostDocument source order는 사용자가 조회하는 의미를 유지해야 한다. | scene 순서만 별도 계산한다. |
| 9 | 동일 `order` in-flow tie에서 source order를 잃는가 | Flex item의 equal order는 안정 순서가 필요하다. | in-flow subset 안에서 source order tie를 보존한다. |
| 10 | nested Flex 자식의 `order`가 상위 Flex에 전역 적용되는가 | `order`는 direct Flex item 관계 안에서만 의미가 있다. | 각 Flex formatting owner 별로 따로 정렬한다. |
| 11 | in-flow Flex item subtree의 일반 descendant가 해당 item의 paint 구간을 벗어나는가 | 현재 제한 paint model은 item의 non-positioned descendant subtree를 함께 유지한다. | 비-positioned subtree만 item order 구간에 둔다. |
| 12 | in-flow item 안의 absolute descendant를 일반 subtree에 그대로 넣는가 | positioned auto descendant는 parent item의 order와 별도 paint phase를 갖는다. | absolute descendant를 positioned phase로 모으고 source preorder를 유지한다. |
| 13 | absolute box 아래 positioned descendant가 현재 제한 모델 밖에서 잘못 정렬되는가 | nested positioned stacking 관계는 이 단계 범위 밖이다. | `z-index`, stacking context, 복합 positioned subtree는 명시적으로 거부·미지원으로 둔다. |
| 14 | `z-index`를 무시하면서 CSS stacking 동등성을 주장하는가 | CSS 2 paint 단계와 `z-index` stacking은 더 넓은 계약이다. | 이 단계는 기본 `z-index:auto` 단색 box만 주장한다. |
| 15 | paint order 관찰을 pointer hit-test 순서와 동일시하는가 | `elementsFromPoint`는 Chrome oracle probe일 뿐 Spinon hit testing을 구현하지 않는다. | hit-test·pointer target은 미지원 경계에 유지한다. |
| 16 | 투명 box 또는 background 없는 box를 paint node 누락으로 오해하는가 | 현재 scene은 transparent paint도 node로 보존할 수 있다. | node-set 검증과 paint 색 표현을 별도 검사한다. |
| 17 | display:none absolute subtree를 positioned phase에 남기는가 | hidden subtree는 CSS box가 생성되지 않는다. | source node 보존과 frame/scene 제외를 별도로 둔다. |
| 18 | paint 순서만 수정하면서 Flex used geometry를 건드리는가 | absolute visibility toggle 전후 Chrome의 in-flow frame이 동일하다. | fixture는 in-flow frame 불변성과 paint order를 독립 결과로 기록한다. |
| 19 | DPR 변경으로 computed `order` 또는 scene source ordering이 달라지는가 | pinned Chrome의 DPR 1·2 관측에서 case·node 순서와 CSS px geometry가 동일하다. | 두 DPR의 serialized observation equality를 기준으로 둔다. |
| 20 | Chrome 관측이나 WPT source 확인을 WPT suite 통과로 과장하는가 | 고정 WPT source를 읽었지만 이 검토는 suite를 실행하지 않았다. | inventory는 `execution: not-run`을 유지한다. |

## 보정한 fixture 결과

- Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport `320×240`, DPR 1·2로 캡처했다.
- overlapping paint probe의 top-to-bottom 순서는 `abs-later`, `abs-first`, in-flow `order:1`, `order:0`, `order:-1`이다. positioned 형제는 source tree 순서를 따르고 in-flow Flex item은 computed order를 따른다.
- source tree/fixture/reference hash는 `tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json`에 고정했다. WPT suite는 실행하지 않았다.

## 기준 자료

- [pinned WPT source](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/flexbox-paint-ordering-003.html)
- [CSS Flexbox Level 1 §4.1](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#abspos-items) · [§4.3](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#painting)
- [CSS 2 §E.2 painting order](https://www.w3.org/TR/CSS2/zindex.html#painting-order)
