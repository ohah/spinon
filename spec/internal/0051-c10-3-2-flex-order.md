# C10.3.2 · Flex `order`와 paint 순서

**문서 ID:** `0051` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현과 Android/iOS Simulator runtime 검증 완료 · **공개 CSS/API 완료:** 아님

이 계약은 제한된 runtime Flex profile에서 in-flow element flex item의 `order`로 layout item 순서와 scene paint 순서를 결정한다. CSS source cascade의 정수 계산, Taffy 입력, render list에서 같은 정렬 규칙을 사용하되 `HostDocument`의 자식 순서와 `NodeId`는 바꾸지 않는다.

## 입력과 typed 값

- Stylo computed style의 `order`를 runtime Flex style adapter가 signed 32-bit 정수(`i32`)로 읽는다. 초기값은 `0`이며 상속되지 않는다. DOM attribute나 Rust layout의 기본값으로 computed cascade를 대신하지 않는다.
- `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1` 여섯 profile에만 연결한다.
- CSS cascade가 유효하지 않은 authored declaration을 무시하는 것과, adapter에 이미 직렬화된 값이 `i32`가 아닌 경우 실패하는 것을 구분한다. 알 수 없는 typed 값은 node와 `order` property 문맥을 보존해 전체 projection을 거부한다.
- `calc()`의 `<integer>` 결과와 `var()`·author stylesheet 계산은 Stylo가 계산한 값을 받는다. adapter가 임의의 decimal 반올림, 누락값 대체 또는 overflow wrapping을 하지 않는다.

## 순서 계약

### Layout

`CalcLayoutTree`는 부모의 typed `display`가 `flex`일 때에만 직계 child Taffy ID를 각 자식 `LayoutStyle.order`의 오름차순으로 안정 정렬한다. 값이 같은 아이템은 HostDocument source order를 유지한다. 낮은 음수 값부터 처리하며 flex line 수집과 C10.2의 basis·grow·shrink 계산은 이 child 순서를 입력으로 받는다.

정렬은 layout용 임시 Taffy child 목록에만 적용한다. `HostDocumentSnapshot.children`, document/source preorder, `NodeId` 대조표와 원본 DOM child 목록은 변경하지 않는다. 비-Flex 부모의 자식은 `order` 값과 관계없이 원래 순서와 기존 layout 규칙을 유지한다.

### Paint

Runtime scene 순회는 각 `display:flex` 부모의 직계 element child를 같은 `order` 오름차순과 안정 tie 규칙으로 정렬한다. 정렬된 flex item의 subtree는 그 item 아래에 둬 부모 밖으로 끌어내지 않는다. 생성된 `RuntimeRenderBox`에는 scene에서의 연속 `paint_order`를 부여한다. 중첩 Flex 컨테이너는 자기 형제 범위에서 다시 정렬한다.

이 계약은 순서 및 겹침이 있는 불투명 배경 box의 scene 순서까지만 다룬다. Hit-test·pointer target·접근성 순서 변환은 구현하지 않는다. `z-index`/stacking context, opacity, clipping, transform, absolute/fixed child, anonymous/text flex item은 지원한다고 주장하지 않는다. Positioned child와 `order:0` paint 경계는 C12 및 C10.3.5 소유다.

## 내부 실행 연결

- Rust 입력: `LayoutStyle.order: i32`; 기본값 `0`.
- Rust layout: Flex 부모의 직계 child ID만 stable sort.
- Runtime style-to-render: Flex 부모별 local stable sort 뒤 node ID와 scene order를 render box에 기록.
- C ABI probe: `spinon_runtime_gpu_host_eval_flex_order_fixture`; Android intent extra `spinon_c1032_flex_order=true`; iOS launch argument `--spinon-c1032-flex-order`.
- C ABI probe는 내부 검증 fixture다. 공개 DOM/CSS API 또는 일반 앱용 호출 계약이 아니다. 호출 오류·timeout은 성공 scene으로 대체하면 안 된다.

## 비교 기준과 판정

- 고정 oracle은 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS arm64다. 재현 입력·inventory·capture script·해당 실행 파일 digest는 [고정 reference](../../tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json)에 있다.
- viewport는 `320×240 CSS px`, DPR 1과 2, `writing-mode: horizontal-tb`, `direction: ltr`다. 각 노드의 `x`, `y`, `width`, `height` 최대 오차는 각 DPR에서 독립적으로 `0.5 CSS px` 이하여야 한다.
- 기준은 11개 case·48개 element node다. 혼합 음수/0/양수, tie, `display:none`, wrap line 수집, grow, ±`calc()` 반올림, signed int32 경계 clamp, `var()`, 비-Flex 부모, 중첩 scope, 겹침 paint를 포함한다. 같은 CSS px fixture의 DPR 1·2 Chrome observation은 동일하다.
- upstream 교차 기준은 WPT revision `d5a765f1089ce6d3f72300281481edf3dddff7f3`의 `flex-order.html`, `flexbox-order-from-lowest.html`, `flexbox-order-only-flexitems.html`, `flexbox-paint-ordering-001.xhtml`다. stacking, absolute child, baseline case는 각각 C22, C12/C10.3.5, C10.3.4/C15 선행 때문에 이 계약에 포함하지 않는다.

## 현재 구현과 검증 경계

기본 layout, cascade adapter, scene paint ordering 및 fixture 검사는 구현했다. Chrome 기준 비교는 `tools/css-reference/c10-3-flex-order-alignment.test.mjs`와 `crates/spinon-layout/src/tests/flex_order.rs`가 수행한다. inline `order` 변경 뒤 incremental cascade와 새 layout 비교, 누락·손상 computed `order`의 fail-closed 동작도 단위 검사에 포함된다.

Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 pinned V8 fixture를 다시 빌드하고 실행했다. 두 실행 모두 동일한 9개 node frame을 보고했고 WGPU가 9개 box를 제출했다. 겹침 장면은 두 앱 screenshot에서 order가 적용된 색상 순서로 보인다. Android emulator는 ANGLE/SwiftShader software backend이며 하드웨어 GPU 근거나 성능 결과가 아니다. iOS build는 성공했지만 `RawWindowMetalLayer` 중복 linker symbol 경고가 `libspinon_wgpu_r08_spike.a`와 `libspinon_ffi.a` 사이에 남아 있다. 이 단계에서 Chrome screenshot과의 pixel-by-pixel 비교, 실기기와 hardware GPU 동작은 확인하지 않았다.

## 범위 밖

RTL/writing-mode 변형, `visibility:collapse`, pseudo-element, text/replaced intrinsic sizing, text baseline, positioned item 및 `z-index`, hit-test/접근성 순서, 화면 reader와 WGPU GPU별 pixel equivalence, 실기기·성능 기준은 이 계약에서 지원 선언하지 않는다. C10.3.2 구현은 C10.3 또는 전체 CSS/Flex 지원을 뜻하지 않으며 내부 계약 숫자 버전 `0.1.0`을 유지한다.

## 근거

- [C10.3 구현 계획](../../plan/c10-3-flex-order-alignment.md) · [C10.3.1 선행 계약](0050-c10-3-1-flex-reverse.md)
- [Chromium reference](../../tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json) · [fixture inventory](../../tests/fixtures/css/c10/flex-order-alignment-inventory.json) · [runtime fixture](../../tests/fixtures/css/c10/runtime-flex-order-alignment.js)
- [구현 실패 관점 검토 및 실행 결과](./evidence/c10-3-2-flex-order-implementation-review-2026-10-10.md) · Android [build](./evidence/c10-3-2-flex-order/android-build.log), [runtime log](./evidence/c10-3-2-flex-order/android-api37-emulator.log), [screenshot](./evidence/c10-3-2-flex-order/android-api37-emulator.png) · iOS [build](./evidence/c10-3-2-flex-order/ios-build.log), [runtime log](./evidence/c10-3-2-flex-order/ios-26.2-iphone-17-pro.log), [screenshot](./evidence/c10-3-2-flex-order/ios-26.2-iphone-17-pro.png) · [Agent Browser reference 보조 화면](./evidence/c10-3-2-flex-order/agent-browser-overlap.png)
