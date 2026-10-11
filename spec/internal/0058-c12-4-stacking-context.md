# 0058 · C12.4 쌓임 맥락과 `z-index`

**문서 ID:** `0058` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제안·미구현 · **공개 API:** 아님

`0058`은 문서 ID다. 이 문서는 CSS stacking 전체를 지원한다고 선언하지 않는다. 기능 구현 전의 제한 runtime 계약 제안이며 [전용 계획](../../plan/c12-4-stacking-context.md), [계획 실패 관점 검토](evidence/c12-4-stacking-plan-review-2026-10-11.md), [고정 Chrome 사전 비교](evidence/c12-4-stacking-precomparison-2026-10-11.md)와 함께 갱신한다.

## 입력과 범위

제안 profile `RuntimeBlockFlexStackingV1`은 같은 cascade/layout revision에서 현재 지원 Block/Flex geometry, C12.1–C12.3 `position`, C10.3 Flex `order`, 단색 background paint와 Stylo computed z-index를 함께 전달한다. 기존 Block-only와 Flex-only snapshot을 나중에 합쳐 mixed document를 만들지 않는다. Root 및 box는 요소-only tree다. Text/inline fragment, float, table, Grid, replaced/anonymous/pseudo boxes, `display:contents`, sticky, top layer, computed `visibility:hidden|collapse` 및 현재 WGPU가 표현하지 않는 paint effects는 미지원 경계다.

Z-index는 문자열이 아닌 typed snapshot `Auto | Integer(i32-equivalent)`로 보존한다. integer의 최종 Rust 표현과 Stylo 0.22.0 accessor는 구현 시작 전에 공식 dependency source와 Cargo.lock로 다시 확인한다. CSS 변수·cascade layer·specificity·inline 우선순위는 Stylo computed result가 소유한다. raw declaration 재파싱, fallback 문자열 추측, 잘못된 typed value의 0 대체는 금지한다.

## 쌓임 맥락 및 순서

- HostDocument root는 root stacking context다.
- positioned relative/absolute에서 `z-index:auto`는 paint상 pseudo-context이며 내부 일반 paint를 묶어 그리되 그 자손 stacking context를 격리하지 않는다.
- positioned relative/absolute integer 값은 자손을 원자적으로 paint하는 실제 stacking context다.
- fixed box는 `z-index:auto`에서도 실제 stacking context를 만들며 integer 값은 그 context 안 stack level이다.
- static Flex item의 `z-index`가 auto가 아니면 실제 stacking context를 만든다. static Block box의 z-index는 stack level 효과가 없다.
- 각 실제 context는 제한된 box 유형에 대한 CSS paint phase와 stack level 순서를 따른다. Block-level box는 기존 Block paint 순서, in-flow Flex item의 computed `order`는 해당 Flex container 안에서만 적용하고, positioned `auto` pseudo-context 및 0은 같은 context-local phase, 양수/음수는 level 오름차순으로 처리한다. `row-reverse`/`column-reverse`의 computed `order`와 front-to-back pixel 순서를 같은 값으로 추론하지 않는다. static Flex item의 non-auto `z-index`는 실제 context이며 non-context Flex item의 자손 context는 상위 context에 참여할 수 있다.
- absolute Flex child는 in-flow Flex item이 아니며 C10.3.5 `order:0`/source-tree 규칙을 따른다. fixed Flex child도 authored `order`로 Flex item처럼 재배치하지 않는다. 그 정확한 tie/phase는 구현 전 pinned Chrome overlap fixture에 저장한다.
- pinned Chrome 154의 `row-reverse` 겹침 fixture에서는 source-first item의 screenshot pixel이 topmost다. 이는 해당 고정 Chrome의 empirical 기준이며 WPT 실행이나 다른 엔진 일치의 증거가 아니다. paint builder는 이 측정값을 다른 Flex case의 `order` 규칙으로 일반화하지 않는다.
- child context는 한 atomic item으로 부모 순서에 합친다. context 바깥에서 z-index를 전역 정렬하지 않는다. HostDocument/DOM source order, NodeId, 수명·접근성 순서는 paint rank 때문에 변경되지 않는다. hit-test/event dispatch 의미는 별도 단계다. 나중에 scene 기반 hit-test가 `paint_order`를 사용할 때는 그 scene의 실제 topmost 결과를 따로 검증한다.

## RuntimeRenderSnapshot 및 backend

Style-to-render는 revision 검증이 끝난 HostDocument, computed styles, layout frames에서 ephemeral context staging을 만든다. 현재 `RuntimeRenderSnapshot`의 flat box 배열과 contiguous `paint_order=0..N-1`을 back-to-front CSS paint list로 유지한다. context tree는 painter 입력 생성에만 사용하고 public JS API나 native view hierarchy로 내보내지 않는다. Renderer는 계산된 순서만 소비한다. z-index parsing/sorting을 WGPU에 복제하지 않는다.

z-index·position·order·display가 바뀌면 새 style revision의 scene을 만들어야 한다. 오래된 paint order를 같은 geometry라는 이유로 재사용하지 않는다. 중복/누락 node, unsupported property, stale revision, allocation/overflow, 잘못된 style/layout node set은 부분 scene이 아니라 전체 오류다.

## 실패 입력과 미지원 trigger

computed `opacity < 1`, non-initial transform/perspective 효과, filter/backdrop-filter, layout/paint containment, `will-change` stacking trigger, blend/isolation, clip-path/mask, container-type/content-visibility, sticky, top layer 및 다른 stacking-context 생성 효과는 이 계약에서 구현하지 않는다. inline·author stylesheet source 검사와 Stylo diagnostic 검증이 CSS가 computed stage에 오기 전에 fail-closed해야 한다. 속성명만 찾고 허용/거부 값을 정하지 않는 것은 불충분하다. source preflight가 selector winner를 모르는 데 따른 보수적 과잉 거부와 computed-style 검사 경계를 구현 전 고정한다. 알려진 속성 예시만 비교하는 allowlist로 전체 입력 검사를 대체하지 않는다. `@media`/`@supports`·nested rule·custom property substitution이 검사를 우회하면 실패다.

오류에는 가능한 경우 `NodeId`, source ID, CSS property/value와 실패 경계를 포함한다. `display:none` subtree는 일관성 검사에는 남지만 layout/paint box에서는 제외한다. 실패 뒤 이전 revision scene을 새 revision으로 간주하거나 일부 node만 새 순서로 제출하지 않는다.

## 비교와 완료 기준

고정 Chromium `154.0.8037.98` / revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 새 fixture를 작성한다. Geometry는 node별 CSS px frame, stacking은 독립 기대값과 고정 overlap pixel을 사용한다. Chrome screenshot의 interior pixel과 명시된 sRGB output을 쓰는 WGPU offscreen readback을 색 공간·capture 설정을 기록해 비교한다. Android 화면 캡처는 실행 증거이며 동일 픽셀 oracle은 아니다. pixel channel 오차는 `1/255` 이하이며 context atomicity·rank·node set에는 pixel tolerance를 적용하지 않는다. 평균값으로 개별 실패를 덮지 않는다.

각 지원 frame의 축 오차는 `0.5 CSS px` 이하이고 기존 C10.3.2/C10.3.5 및 C12.1–C12.3 geometry는 변하지 않아야 한다. Android 실기기 V8→Stylo→Taffy→WGPU와 iOS Simulator 같은 fixture에서 frame, paint rank, scene revision, screenshot/readback을 확인해야 한다. WPT 전체 통과·웹 CSS 전체·border/gradient/shadow/opacity/transform·성능 향상으로 확대하지 않는다.

계획 검토, PR 문서 검토, 실제 구현 검토는 서로 대체하지 않는다. 구현 완료 전에는 이 계약 상태를 제안·미구현으로 둔다.

## 표준·관련 자료

- [CSS2.2 Appendix E](https://www.w3.org/TR/CSS22/zindex.html)
- [CSS Positioned Layout Level 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)
- [CSS Flexbox Level 1 · 2025-10-14 CRD](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/)
- [C10.3.2 Flex order contract](0051-c10-3-2-flex-order.md) · [C10.3.5 positioned Flex contract](0056-c10-3-5-positioned-flex.md) · [C12.3 fixed contract](0057-c12-3-fixed-viewport.md)
- [C12.4 implementation plan](../../plan/c12-4-stacking-context.md)
