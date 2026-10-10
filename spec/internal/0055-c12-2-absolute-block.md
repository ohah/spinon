# 0055 · C12.2 Block absolute positioning

**문서 ID:** `0055` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현·플랫폼 검증 완료 · [PR #130 병합](https://github.com/ohah/spinon/pull/130) · **공개 API:** 아님

`0055`는 문서 ID다. 출시 전 숫자 버전은 구현·검증·문서 갱신으로 올리지 않는다. 이 계약은 CSS 전체 absolute positioning 지원을 뜻하지 않는다.

## 범위

`position:absolute`인 Block formatting box의 containing-block 선택, flow 제외, 물리 inset·크기·margin 기하와 frame 반환을 C12.2 runtime profile에 연결한다. source/DOM parent와 source order는 HostDocument에 그대로 보존한다. absolute layout용 parent graph는 별도 계산 자료이며 HostDocument를 재부모화하지 않는다.

초기 성공 profile은 `horizontal-tb`·LTR에서 텍스트와 replaced element가 없는 fixed-size element box, 또는 네 inset과 min/max로 측정 가능한 박스다. 요소의 절대 크기 auto 계산은 해당 child geometry가 측정 가능할 때만 허용한다. 텍스트/replaced intrinsic measure에 의존하는 shrink-to-fit은 C14/C15 경계 진단으로 실패한다.

## 계산 입력과 소유 관계

- Stylo cascade가 computed `position` 및 물리 `top`·`right`·`bottom`·`left`를 제공한다. `inset` shorthand·longhand·layer·specificity·`!important`·custom property를 layout adapter에서 다시 파싱하지 않는다.
- `LayoutPositioning`은 일반 크기·Flex/Block `LayoutStyle`과 분리한다. `LayoutPosition::Absolute`만 out-of-flow로 투영하며 `static`과 `relative` 기존 의미를 바꾸지 않는다.
- containing-block owner는 같은 box tree 안에서 가장 가까운 `relative` 또는 `absolute` ancestor다. 해당 owner가 없으면 initial containing block인 CSS viewport다. `display:none` 자신과 subtree의 owner는 `NoBox`이며 이 subtree를 viewport 자식으로 승격하지 않는다.
- absolute box의 layout parent는 계산된 containing-block owner가 된다. source parent·DOM 자식 순서·이벤트·수명 소유자는 원래대로 유지한다. owner가 계산 입력에 없거나 끊긴 graph면 viewport로 대체하지 않고 전체 계산을 실패시킨다.
- 표시용 synthetic viewport는 layout tree 내부에만 존재한다. HostDocument, NodeId, renderer source tree와 output frame map에는 노출하지 않는다.

## 기하·프레임 규칙

- 일반 positioned block containing block의 기하 원점은 owner border edge 안쪽인 padding edge이고, 영역 크기는 border를 제외한 padding box다. inset percentage의 left/right basis는 그 영역 너비, top/bottom basis는 그 영역 높이다. viewport fallback은 CSS px viewport origin/size를 쓰며 Surface pixel·dp/point·safe-area inset을 혼합하지 않는다.
- `absolute`는 normal-flow sizing에서 제외된다. in-flow sibling 위치와 auto 부모 높이는 absolute child 추가 전/후 동일해야 한다. 최종 `frames`는 containing-block owner 기준 배치 결과를 viewport 좌표로 합성해 한 번만 반환한다.
- `flow_frames`에는 absolute box를 normal-flow 요소처럼 계산한 hypothetical flow 좌표를 담는다. `top/bottom` 또는 `left/right`가 모두 `auto`인 축의 static position은 CSS 2.1/CSS Position 규칙과 고정 Chromium oracle에 맞춰 이 flow 좌표와 containing-block 좌표계를 연결한다. 두 owner 좌표를 안전하게 변환할 수 없으면 실패한다.
- 지정 inset·크기·min/max·box-sizing·padding·border·margin·aspect ratio는 기존 typed CSS math와 Taffy `0.14.0` 계산을 사용한다. definite inset 두 면과 auto size의 stretch, auto margin, over-constrained LTR 규칙을 기준 fixture와 대조한다.
- absolute box의 used display는 CSS blockification을 반영하고 block formatting box는 독립 formatting context로 계산한다. inline fragmentation은 box topology가 제공되지 않아 지원하지 않는다. Flex static-position 정렬과 `order:0` paint는 C10.3.5 범위다.

## 실패·원자성

- fixed/sticky, RTL·논리 inset·다른 writing mode, Grid/Flex static-position, display:contents/table/fragmented inline, text/replaced intrinsic size, 미지원 ancestor containing-block 효과는 현재 성공 profile에서 식별 가능한 오류로 거부한다.
- NaN·무한대·잘못된 CSS math handle·indefinite percentage basis·누락/중복 node·중복 부모·숨겨진 owner·stale document/style/environment revision 중 하나라도 있으면 새 `frames` 일부를 반환하지 않는다.
- containing-block owner, position, inset, size, padding/border 또는 ancestor revision이 바뀌면 영향 descendant의 이전 snapshot을 재사용하지 않는다.
- 별도 paint, border stroke, stacking context/z-index, hit-test·pointer dispatch, accessibility traversal, scroll/clip을 지원 완료로 표시하지 않는다.

## 비교 기준 및 근거

- [CSS Position 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/), CSS 2.1 absolute non-replaced sizing, 고정 Chrome 154.0.8037.98·revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`를 사용한다.
- [C12.2 사전 비교](./evidence/c12-2-absolute-block-precomparison-2026-10-11.md)는 23 case·82 node, DPR 1/2, CSS px geometry 허용치, 세 flow probes와 실패 판정을 고정한다. 비교 도구와 fixture source hash가 달라지면 reference 검사를 중단한다.
- 고정 WPT revision `9ec154ff43db468923997c08bb08f905ceab62a5`의 다섯 대응 경로는 확인했지만 WPT suite는 실행하지 않았다.
- 성공 완료는 reference의 지원 field가 `0.5 CSS px` 이내이고 owner·source tree·parent auto height·computed inset이 모두 일치하며, 지원 밖 입력·stale revision의 negative test가 명시 오류를 내고, 실제 V8→Stylo→Taffy→WGPU fixture가 Android 실기기 및 iOS Simulator에서 실행된 뒤에만 판정한다. simulator smoke를 전체 23-case 적합성이나 hardware GPU 성능으로 확대하지 않는다.

## 관련 자료

- [C12 위치 지정 계획](../../plan/c12-positioning.md)
- [사전 비교 기록](./evidence/c12-2-absolute-block-precomparison-2026-10-11.md)
- Chromium reference: `../../tests/fixtures/css/references/c12-2-position-absolute-block-v1.json`
- WPT revision에 고정한 subset: `position-absolute-padding-percentage.html`, `position-absolute-percentage-height.html`, `position-absolute-margin-auto-001.html`, `position-absolute-dynamic-static-position.html`, `position-absolute-dynamic-formatting-context.html`
- [Android 실기기·iOS Simulator 실행 근거와 화면](./evidence/c12-2-absolute-block-2026-10-11/README.md)
- [구현 변경 실패 경로 검토](./evidence/c12-2-absolute-block-implementation-review-2026-10-11.md)
