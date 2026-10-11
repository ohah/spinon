# 0058 · C12.4 쌓임 맥락과 `z-index`

**문서 ID:** `0058` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 구현 후보 연결됨 · 전체 검증 및 병합 전 · **공개 API:** 아님

`0058`은 문서 ID다. 이 문서는 CSS stacking 전체를 지원한다고 선언하지 않는다. 내부 제한 runtime 구현 계약이며 [전용 계획](../../plan/c12-4-stacking-context.md), [깊이 안전성 계획](../../plan/c12-4-layout-depth-safety.md), [계획 실패 관점 검토](evidence/c12-4-stacking-plan-review-2026-10-11.md), [profile 계약 검토](evidence/c12-4-stacking-profile-review-2026-10-11.md), [고정 Chrome 사전 비교](evidence/c12-4-stacking-precomparison-2026-10-11.md), [구현 후 실패 관점 검토](evidence/c12-4-stacking-implementation-review-2026-10-11.md), [WPT 후보 런타임 의미 확인](evidence/c12-4-wpt-candidate-runtime-2026-10-11.md), [20개 WPT 회귀 검토](evidence/c12-4-wpt-runtime-implementation-review-2026-10-11.md), [플랫폼 실행 근거](evidence/c12-4-stacking-runtime-2026-10-11/README.md), [깊이 실행 근거](evidence/c12-4-layout-depth-safety-platforms-2026-10-11.md), [깊이 안전성 구현 검토](evidence/c12-4-layout-depth-safety-implementation-review-2026-10-11.md)와 함께 갱신한다.

## 입력과 범위

확정 profile `RuntimeBlockFlexStackingV1`은 같은 full cascade/layout revision에서 현재 지원 Block/Flex geometry, C12.1–C12.3 `position`, C10.3 Flex `order`, 단색 background paint와 Stylo computed z-index를 함께 전달한다. 새 profile은 초기 구현에서 전체 cascade를 계산하고 이전 profile의 cache 결과를 재사용하지 않는다. 기존 Block-only와 Flex-only snapshot을 나중에 합쳐 mixed document를 만들지 않는다. Root 및 box는 요소-only tree다. Text/inline fragment, float, table, Grid, replaced/anonymous/pseudo boxes, `display:contents`, sticky, top layer, computed `visibility:hidden|collapse` 및 현재 WGPU가 표현하지 않는 paint effects는 미지원 경계다.

Z-index는 문자열이 아닌 typed snapshot `Auto | Integer(i32)`로 보존한다. 구현은 잠금된 Stylo 0.22.0의 `ComputedValues::clone_z_index()`와 `GenericZIndex<Integer>`를 사용하며 raw declaration을 재파싱하지 않는다. `visibility`도 같은 computed style에서 typed enum으로 복사하고 `Visible`만 painter admission에 허용한다. CSS 변수·등록 사용자 지정 속성·cascade layer·specificity·inline 우선순위는 Stylo computed result가 소유한다. fallback 문자열 추측, 잘못된 typed value의 0 대체는 금지한다.

허용 declaration은 현재 runtime Block positioning 및 Flex layout/paint/registered-property allowlist의 합집합에 `z-index`, `visibility`를 더한 값으로 제한하며 `background-color`는 기존 단색 runtime paint 제한을 유지한다. inline style과 author stylesheet를 모두 검사한다. author CSS는 `@layer` statement/block, `@property`, 사용자 지정 속성을 허용한다. `@media`, `@supports`, nested style rule, 외부 stylesheet 및 알 수 없는 at-rule은 거부한다. 지원하지 않는 declaration은 cascade winner 여부와 관계없이 source preflight에서 보수적으로 거부한다. computed `visibility`가 `hidden|collapse`면 상속·`var()` 경로와 무관하게 scene 전체를 거부한다. `visibility:visible`은 성공 가능하지만 computed gate를 통과해야 한다.

Block positioning의 viewport-root margin 동작은 C12.2/3 profile과 같이 유지한다. nonzero root margin을 Flex 전용 profile처럼 새로 금지하지 않는다. 일반 Block auto margin과 direct in-flow Flex item auto margin은 각 기존 layout 규칙을 따른다.

## 쌓임 맥락 및 순서

- HostDocument root는 root stacking context다.
- positioned relative/absolute에서 `z-index:auto`는 paint상 pseudo-context이며 내부 일반 paint를 묶어 그리되 그 자손 stacking context를 격리하지 않는다.
- positioned relative/absolute integer 값은 자손을 원자적으로 paint하는 실제 stacking context다.
- fixed box는 `z-index:auto`에서도 실제 stacking context를 만들며 integer 값은 그 context 안 stack level이다.
- static Flex item의 `z-index`가 auto가 아니면 실제 stacking context를 만든다. static Block box의 z-index는 stack level 효과가 없다.
- 각 실제 context는 제한된 box 유형에 대한 CSS paint phase와 stack level 순서를 따른다. Block-level box는 기존 Block paint 순서, in-flow Flex item의 computed `order`는 해당 Flex container 안에서만 적용하고, positioned `auto` pseudo-context 및 0은 같은 context-local phase, 양수/음수는 level 오름차순으로 처리한다. `row-reverse`/`column-reverse`의 computed `order`와 front-to-back pixel 순서를 같은 값으로 추론하지 않는다. static Flex item의 non-auto `z-index`는 실제 context이며 non-context Flex item의 자손 context는 상위 context에 참여할 수 있다.
- absolute Flex child는 in-flow Flex item이 아니며 C10.3.5 `order:0`/source-tree 규칙을 따른다. fixed Flex child도 authored `order`로 Flex item처럼 재배치하지 않는다. 그 정확한 tie/phase는 구현 전 pinned Chrome overlap fixture에 저장한다.
- pinned Chrome 154의 여덟 reverse/non-reverse Flex 방향·`order` 조합을 screenshot pixel로 확인했다. `row-reverse`와 `column-reverse`는 해당 겹침 fixture에서 source-first item의 pixel이 topmost였다. 이 경험적 결과를 limited runtime profile에 적용했으며 [W3C Flexbox Level 1 §4.3, §5.1](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#flex-direction-property)의 reverse 값은 paint order를 바꾸지 않는다는 설명과 충돌한다. 따라서 구현은 pinned Chrome 호환 예외이고 표준 적합성이나 다른 엔진 일치를 뜻하지 않는다. Rust rank·WGPU pixel과 Android AVD `PixelCopy` 증거는 [reverse Flex 구현 근거](./evidence/c12-4-flex-reverse-paint-implementation-review-2026-10-11.md)에 기록했다.
- child context는 한 atomic item으로 부모 순서에 합친다. context 바깥에서 z-index를 전역 정렬하지 않는다. HostDocument/DOM source order, NodeId, 수명·접근성 순서는 paint rank 때문에 변경되지 않는다. hit-test/event dispatch 의미는 별도 단계다. 나중에 scene 기반 hit-test가 `paint_order`를 사용할 때는 그 scene의 실제 topmost 결과를 따로 검증한다.

## RuntimeRenderSnapshot 및 backend

Style-to-render는 revision 검증이 끝난 HostDocument, computed styles, layout frames에서 ephemeral context staging을 만든다. 현재 `RuntimeRenderSnapshot`의 flat box 배열과 contiguous `paint_order=0..N-1`을 back-to-front CSS paint list로 유지한다. context tree는 painter 입력 생성에만 사용하고 public JS API나 native view hierarchy로 내보내지 않는다. Renderer는 계산된 순서만 소비한다. z-index parsing/sorting을 WGPU에 복제하지 않는다.

z-index·position·order·display가 바뀌면 새 style revision의 scene을 만들어야 한다. 오래된 paint order를 같은 geometry라는 이유로 재사용하지 않는다. 중복/누락 node, unsupported property, stale revision, allocation/overflow, 잘못된 style/layout node set은 부분 scene이 아니라 전체 오류다.

## 실패 입력과 미지원 trigger

computed `opacity < 1`, non-initial transform/perspective 효과, filter/backdrop-filter, layout/paint containment, `will-change` stacking trigger, blend/isolation, clip-path/mask, container-type/content-visibility, sticky, top layer 및 다른 stacking-context 생성 효과는 이 계약에서 구현하지 않는다. inline·author stylesheet source 검사와 Stylo diagnostic 검증이 CSS가 computed stage에 오기 전에 fail-closed해야 한다. 속성명만 찾고 허용/거부 값을 정하지 않는 것은 불충분하다. source preflight가 selector winner를 모르는 데 따른 보수적 과잉 거부와 computed-style 검사 경계를 구현 전 고정한다. 알려진 속성 예시만 비교하는 allowlist로 전체 입력 검사를 대체하지 않는다. `@media`/`@supports`·nested rule·custom property substitution이 검사를 우회하면 실패다.

오류에는 가능한 경우 `NodeId`, source ID, CSS property/value와 실패 경계를 포함한다. `display:none` subtree는 일관성 검사에는 남지만 layout/paint box에서는 제외한다. paint builder의 명시적인 `Vec::try_reserve` 실패와 `u32` paint-order 변환 실패는 부분 scene 없이 반환한다. traversal은 반복형으로 구현한다. Rust의 모든 infallible allocator 호출까지 OOM 복구를 보장하지 않는다. 실패 뒤 이전 revision scene을 새 revision으로 간주하거나 일부 node만 새 순서로 제출하지 않는다.

## 비교와 완료 기준

고정 Chromium `154.0.8037.98` / revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33` fixture를 사용한다. Geometry는 node별 CSS px frame, stacking은 독립 기대값과 고정 overlap pixel로 비교한다. 여덟 reverse Flex microfixture 조합은 Rust paint-rank와 WGPU offscreen 표본 모두 기대값과 일치했다. Android API 37 AVD의 [실제 `SurfaceView` pixel run](./evidence/c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/comparison.json)은 지원 scene 21/21, 표본 22/22, 최대 채널 오차 0이다. iPhone 17 Pro / iOS 26.2 Simulator의 [실제 WGPU Metal surface pixel run](./evidence/c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/comparison.json)은 8개 reverse Flex 방향/order 조합 8/8, 채널 허용 오차 0이다. 두 실행은 Simulator/AVD의 제한 표본이며 hardware GPU 증거가 아니다. pixel channel 오차는 `1/255` 이하이며 context atomicity·rank·node set에는 pixel tolerance를 적용하지 않는다. 평균값으로 개별 실패를 덮지 않는다.

각 지원 frame의 축 오차는 `0.5 CSS px` 이하이고 기존 C10.3.2/C10.3.5 및 C12.1–C12.3 geometry는 변하지 않아야 한다. Android/iOS 실행은 해당 근거에 기록된 simulator·실기기 범위를 유지하며, 한 플랫폼 결과를 다른 플랫폼 pixel 결과로 대신하지 않는다. WPT 전체 통과·웹 CSS 전체·border/gradient/shadow/opacity/transform·성능 향상으로 확대하지 않는다.

계획 검토, profile 계약 검토, PR 문서 검토, 실제 구현 검토는 서로 대체하지 않는다. 내부 숫자 버전은 계속 `0.1.0`으로 고정한다. Android API 37 AVD 26-scene runtime matrix는 21개 지원·5개 기대 거부였고, 지원 fixture node 64개 frame은 pinned Chrome과 최대 오차 0 CSS px였다. Android `PixelCopy`는 지원 장면 21/21·표본 22/22·최대 채널 오차 0으로 끝났다. iOS Metal surface 표본은 8/8 exact pixel로 통과했다. iOS 26.2 Simulator 전체 runtime matrix도 26개 scene 중 21개 지원·5개 기대 거부가 일치했고, 지원 node 64개의 frame 최대 오차는 0 CSS px였다. 두 Simulator에서 session 0·1·2·4·8 누적 생성 및 해제 비용도 측정했다. iOS/Android process memory 결과는 CSS worker stack 단독 비용이 아니며, Android AVD `VmSize` 증가 원인은 권한 제한으로 매핑을 읽지 못해 미확정이다. AVD ANGLE/SwiftShader 결과는 hardware GPU 증거가 아니다. Chrome WPT 후보는 기준 엔진의 의미 확인이며 Spinon 적합성 결과가 아니다. 깊은 HostDocument 제한은 Android API 37 AVD와 iOS 26.2 Simulator 앱 경로에서 1,024 성공·1,025 거부·새 revision 복구를 확인했다. 실기기 RSS, thread별 stack commitment, Spinon WPT 실행, 전역 allocator OOM 복구는 미완료다. painter `Vec::try_reserve` 오류 주입은 실제 전역 allocator OOM 복구를 증명하지 않는다. C12.4와 공개 CSS/API 지원은 미완료다.

## 표준·관련 자료

- [CSS2.2 Appendix E](https://www.w3.org/TR/CSS22/zindex.html)
- [CSS Positioned Layout Level 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)
- [CSS Flexbox Level 1 · 2025-10-14 CRD](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/)
- [C10.3.2 Flex order contract](0051-c10-3-2-flex-order.md) · [C10.3.5 positioned Flex contract](0056-c10-3-5-positioned-flex.md) · [C12.3 fixed contract](0057-c12-3-fixed-viewport.md)
- [C12.4 implementation plan](../../plan/c12-4-stacking-context.md)
