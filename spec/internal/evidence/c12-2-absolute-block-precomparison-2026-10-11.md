# C12.2 Block absolute 사전 비교 기록

이 기록은 기능 구현 전 고정 Chrome 관찰값, 입력과 판정 방법을 보존한다. Spinon absolute layout이 구현·검증됐다는 뜻이 아니다.

## 고정 실행 조건

| 항목 | 값 |
| --- | --- |
| Chromium | Chrome 154.0.8037.98, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS arm64 |
| Chromium binary SHA-256 | `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954` |
| CSS 기준 | [CSS Positioned Layout 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/) |
| viewport | 360×800 CSS px, DPR 1·2 |
| 환경 | `en-US`, UTC, light, forced colors 없음, coarse pointer, hover 없음, LTR, `horizontal-tb` |
| inventory | 23 case, 82개 식별 DOM node, source parent·case·expected containing-block owner 고정 |
| geometry | `x/y/width/height` 각 field 절대오차 최대 `0.5 CSS px`; owner 또는 node identity 불일치는 독립 실패 |
| WPT | commit `9ec154ff43db468923997c08bb08f905ceab62a5`; 대응 파일 경로만 확인, 실행하지 않음 |

실행은 `mise exec -- bun run css:reference:c12-2-absolute-block`, 검사는 `mise exec -- node --test tools/css-reference/c12-2-absolute-block.test.mjs`다. [inventory](../../tests/fixtures/css/c12/position-absolute-block-inventory.json), [HTML](../../tests/fixtures/css/c12/position-absolute-block.html), [JavaScript fixture](../../tests/fixtures/css/c12/runtime-position-absolute-block.js), [capture tool](../../tools/css-reference/capture-c12-2-absolute-block.mjs), [reference](../../tests/fixtures/css/references/c12-2-position-absolute-block-v1.json)의 SHA-256은 reference JSON에 고정한다. 기존 reference를 capture 명령으로 덮어쓰지 않는다.

## 고정한 동작 분리

- viewport fallback과 relative owner의 padding edge를 구분한다. owner의 border thickness를 빼고 left/right percentage는 padding-box width, top/bottom은 padding-box height로 비교한다.
- static 중간 ancestor는 absolute containing block으로 선택하지 않는다. 중첩 relative와 absolute ancestor는 각각 nearest owner가 된다. DOM `parentId`는 이 owner와 독립적으로 비교한다.
- absolute child가 빠져도 in-flow 앞/뒤 형제 위치와 auto-height 부모 크기는 유지된다. `flow-absolute`와 `static-absolute`를 각각 잠시 static으로 바꾼 관찰값을 flow probe로 보존한다.
- definite right/bottom, 네 inset의 percentage, `calc()` 분수값, auto width/height stretch, auto margin, LTR over-constraint, min/max, content-box/border-box, aspect ratio, signed margin/inset을 별도 case로 둔다.
- 모든 inset이 auto인 block의 hypothetical flow position, static source parent와 다른 positioned containing-block owner의 좌표 변환, computed `display:inline`인 absolute element의 blockified computed display, absolute block 내부 margin, `display:none` subtree와 absolute ancestor의 descendant owner를 독립 관찰한다.
- shorthand·layer·selector·`!important` cascade의 computed side를 layout parser와 별도로 기록한다.

## 대표 Chromium 관찰값

| 관찰 | 결과 |
| --- | --- |
| viewport absolute | `(17, 11, 30, 20)`, owner `viewport` |
| border 4px인 positioned owner, left 5px/top 7px | padding-edge 기준 `(9, 81, 30, 12)` |
| static wrapper 아래 absolute | source parent `skip-static-wrapper`, owner `skip-owner`, `(8, 146, 24, 10)` |
| nested relative owner | owner `nested-inner`, `(32, 232, 20, 12)` |
| flow 제거 | auto 부모 `(0, 280, 300, 24)`, 뒤 sibling y=`292`; static probe y=`292` |
| 축별 percentage | target `(52, 389.1875, 20, 10)`, computed left=`48px`, top=`11.1875px` |
| `calc()` inset | target `(27.1875, 461.1875, 20, 10)`, computed left=`23.1875px`, top=`13.1875px` |
| definite insets + auto size | `(12, 523, 172, 60)` |
| min/max로 stretch 제한 | width=`130px` |
| content-box vs border-box | 각각 `(60, 30)`·`(80, 50)` outer size |
| all-auto inset hypothetical position | static probe와 absolute target 모두 `(0, 876, 40, 10)` |
| block formatting context | absolute box `(3, 964, 80, 22)`, 자식 top margin 10px 포함 |
| hidden subtree | absolute child는 box 없음, owner `none` |

## 제외·판정 한계

- 기준의 모든 좌표는 browser oracle일 뿐 Spinon 출력이 아니다. WPT suite, Android/iOS runtime, paint pixel, hit-test, 성능은 이 단계에서 실행하지 않았다.
- fixture는 text/replaced intrinsic measurement, Flex static-position, Grid, RTL·논리 inset, ancestor transform/contain, fixed/sticky, inline fragmentation, dynamic style mutation을 대표하지 않는다.
- computed `display`는 Chrome이 absolute inline box에 blockification을 적용한 결과인 `block`이다. 이를 authored declaration 원문이나 paint proof로 해석하지 않는다.
- 이 reference만으로 C12.2, C12, CSS, 또는 모바일 적합성 완료를 체크하지 않는다.
