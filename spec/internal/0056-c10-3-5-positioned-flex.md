# 0056 · C10.3.5 위치 지정 Flex 자식

**문서 ID:** `0056` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현·검증 완료 · [PR #132 리베이스 병합](https://github.com/ohah/spinon/pull/132) · **공개 API:** 아님

`0056`은 문서 ID다. 출시 전 숫자 버전은 구현·검증·문서 수정으로 올리지 않는다. 이 계약은 CSS Flexbox 전체 또는 모든 `position:absolute` 동작을 지원한다는 뜻이 아니다.

## 범위

제한 runtime Flex profile에서 Flex container의 직접 element child인 absolute box의 static position을 계산하고, in-flow Flex item과 분리된 paint 순서를 반환한다. 구현은 Stylo의 computed style을 typed layout 입력으로 전달하고 Taffy 계산 및 기존 V8→WGPU runtime을 사용한다. HostDocument의 source parent와 자식 순서는 보존한다.

성공 profile은 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`다. 각 profile의 기존 CSS 입력·사용자 지정 속성·paint 차이는 그대로 유지한다. C04/C05의 비-runtime profile과 Block legacy profile은 이 기능으로 승격하지 않는다.

## 소유 관계와 입력

- **source parent**는 HostDocument에서 노드가 실제로 삽입된 부모다. layout 계산을 위해 노드를 옮겨도 원래 관계와 NodeId는 바뀌지 않는다.
- **static-position formatting owner**는 absolute box의 source box parent다. source parent가 Flex box이면 Flex 규칙을 적용한다. Block wrapper 내부의 absolute descendant에는 조상의 Flex 정렬을 물려주지 않는다.
- **containing-block owner**는 C12.2와 같은 nearest positioned ancestor 또는 viewport다. static-position formatting owner와 달라도 된다. 두 좌표계는 검증된 source/owner frame으로 변환하며, 누락 또는 숨겨진 owner는 전체 계산 오류다.
- Stylo가 계산한 `position`, 물리 `top`·`right`·`bottom`·`left`, `order`, Flex 정렬 값과 box geometry만 입력으로 사용한다. 어댑터에서 원본 CSS 문자열을 다시 파싱하지 않는다.
- 지원 초기 slice는 `horizontal-tb`·LTR, 유한 CSS px의 fixed-size Flex container 및 fixed-size element child다. 텍스트, anonymous/replaced box, `display:contents`, intrinsic/불명확한 used size는 실패한다.

## static-position 계산

- absolute child는 Flex line 수집, wrap, gap, grow/shrink, 부모 used size와 in-flow sibling frame에 들어가지 않는다.
- 직접 Flex child의 auto inset 축은 그 child 하나만 있는 Flex container처럼 계산한 static-position frame을 사용한다. 주축은 `justify-content`, 교차축은 `align-self` 또는 부모 `align-items`의 pinned reference 동작을 사용한다. fixed child의 auto margin은 static-position 산출에서 0으로 취급한다.
- 한 축에서 양쪽 inset이 모두 `auto`일 때만 해당 축의 static position을 적용한다. definite inset이 하나라도 있으면 C12.2 absolute inset 계산이 그 축을 소유한다.
- source Flex parent와 containing block이 다르면 static position을 source parent의 content box에서 계산한 후 viewport 좌표로 변환하고, 최종 containing-block 배치에 합성한다.
- 중간 frame·계산 style·source revision이 빠지거나 비유한 값이면 부분 frame을 게시하지 않는다. 오류는 해당 NodeId와 실패한 CSS 입력을 식별한다.

## paint 순서

- 각 Flex container의 in-flow item은 computed `order` 오름차순, 동일 값은 source order로 처리한다. 이 순서는 HostDocument·DOM 유사 조회·이벤트 순서를 바꾸지 않는다.
- absolute child의 authored `order`는 보존하지만 Flex item 정렬이나 positioned paint 순서에는 쓰지 않는다. 같은 제한 paint context의 z-index:auto absolute box는 in-flow paint 뒤 source tree 순서로 제출한다.
- `display:none` 및 숨겨진 subtree는 style/layout 일관성 검증에는 남기고 scene box에서는 제외한다.
- 이 계약은 별도 stacking context, `z-index`, transform, opacity, clip/scroll, hit-test 및 접근성 순서를 정의하지 않는다. 해당 속성을 가진 입력은 성공 범위로 확대하지 않고 기존 fail-closed 규칙을 따른다.

## 검증과 한계

- Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 고정 reference를 사용한다. 기준 파일은 자동으로 덮어쓰지 않는다.
- viewport `320×240 CSS px`, DPR 1·2, LTR/horizontal-tb 기준에서 frame 각 축의 절대 오차 허용치는 `0.5 CSS px`다. owner, node set, flow 불변성, paint 순서 차이는 좌표 허용치로 상쇄하지 않는다.
- WPT 후보는 inventory에 고정된 revision으로 추적한다. suite를 실제 실행하기 전에는 WPT 통과로 표기하지 않는다.
- Android 실기기와 iOS Simulator에서 V8→Stylo→Taffy→WGPU 경로를 별도 실행하고 기기, OS, GPU backend, NodeId/frame, 로그와 화면을 남긴다. simulator smoke를 전체 CSS 적합성이나 성능 결과로 확대하지 않는다.

고정 fixture 전체는 Chrome 154에서 20개 case·70개 node를 DPR 1·2로 비교했고, Rust adapter의 frame·owner 결과가 모두 일치했다. 모바일은 같은 입력의 3개 case·15개 node만 실행한다. runtime paint profile이 받지 않는 `background` shorthand는 색상 보존 `background-color`로 정규화하며 이는 shorthand parser 검증으로 계산하지 않는다. Android SM-S731N / Android 16 API 36 / Samsung Xclipse 940 Vulkan과 iPhone 17 Pro / iOS 26.2 Simulator / Metal에서 각 15개 node frame이 Chrome과 최대 오차 0 CSS px였고 WGPU가 root를 포함한 16개 상자를 제출했다. [기기 로그·비교 결과·화면](evidence/c10-3-5-positioned-flex-2026-10-11/README.md). WPT suite, 전체 fixture 모바일 행렬, iOS 실기기, 픽셀 동일성과 성능은 검증하지 않았다.

## 예시

```html
<div style="display:flex;position:relative;width:180px;height:100px;
            justify-content:center;align-items:flex-end">
  <div style="position:absolute;width:30px;height:20px"></div>
  <div style="width:20px;height:20px"></div>
</div>
```

첫 번째 자식은 Flex line 및 두 번째 자식의 frame에 영향을 주지 않는다. 양 inset이 모두 auto인 축의 위치는 Flex static-position 규칙을 따르며, 실제 구현은 고정 Chrome reference와 일치해야 한다.

## 관련 자료

- [C10.3.5 구현 계획](../../plan/c10-3-5-positioned-flex.md)
- [C10.3.5 Chromium inventory](../../tests/fixtures/css/c10/positioned-flex-inventory.json)
- [C10.3.5 고정 Chromium reference](../../tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json)
- [C10.3.5 플랫폼 실행 근거](evidence/c10-3-5-positioned-flex-2026-10-11/README.md)
- [C12.2 Block absolute positioning 계약](0055-c12-2-absolute-block.md)
