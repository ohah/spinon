# 0025 · C04.4 Cascade Layers와 Flex alignment

**계약 버전:** `0.1.0` · **상태:** 내부 profile 구현 완료 · **Stylo:** `0.22.0` · **제품 CSS 지원:** 미완료

## 목적과 경계

기존 C04.3 `FlexAlignmentV1`을 변경하지 않고, author stylesheet 안의 CSS Cascade Layers를 처리하는 새 `FlexAlignmentCascadeLayersV1` profile을 제공한다. 계산 cascade는 Stylo가 소유한다. Spinon의 author CSS 검사기는 `@layer` 구조를 순회하며 지원 declaration 범위를 확인할 뿐 cascade 우선순위를 다시 계산하지 않는다.

이 계약은 `spinon-style`과 `spinon-style-to-layout`의 내부 Rust 인터페이스다. 제품 runtime, JS API, 공개 CSS 지원, stylesheet 다운로드, native renderer·GPU 제출, Android/iOS 화면을 제공하지 않는다.

## 입력과 내부 진입점

```rust
pub fn compute_flex_alignment_layers_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError>;
```

`view`와 `snapshot`은 같은 document generation, document revision, render-tree revision을 가져야 한다. viewport에는 유한한 양수 CSS px 크기·배율과 `EnvironmentRevision`이 포함된다. 계산 결과는 입력의 source·style·environment revision을 보존한다. author stylesheet는 `CssOrigin::Author`만 허용하며 전달 순서가 Stylo 등록 순서가 된다. 내장 UA stylesheet는 기존 C04 cascade와 동일하게 먼저 등록한다.

새 computed profile ID는 `ComputedStyleProfile::FlexAlignmentCascadeLayersV1`이다. 허용 속성과 값 변환은 [0024 C04.3](0024-c04-flex-alignment.md)의 Flex layout·align-items·justify-content 범위와 같다.

## 허용하는 layer 규칙

- `@layer name;` 및 `@layer first, second;` order statement
- named, anonymous, nested `@layer name { ... }` block
- 같은 이름의 layer 재개방과 author stylesheet 사이 같은 이름 재사용
- layer 안의 일반 style rule. 각 선언은 0024 property allowlist를 통과해야 한다.

Stylo cascade 순서를 기준으로 normal declaration은 뒤 layer가 앞 layer보다 우선하고 unlayered normal은 explicit layer보다 우선한다. `!important`에서는 layer 우선순위가 반전되며 explicit-layer important는 unlayered important보다 우선한다. 각 origin에서 layer 단계는 specificity 및 source order보다 먼저 적용된다. nested layer와 parent의 implicit sublayer·anonymous layer 고유성도 Stylo의 구현을 따른다. 규범 근거는 [CSS Cascading and Inheritance Level 5 §6.4](https://www.w3.org/TR/css-cascade-5/#cascade-layers)다.

## 제한과 오류

| 입력 | 결과 |
|---|---|
| 허용 property 밖의 declaration, 사용자 지정 속성 | `UnsupportedAuthorCss`; 전체 요청 실패 |
| `@layer` 외 at-rule, layer 밖 중첩 style rule, layer 안 CSS nesting | `UnsupportedAuthorCss`; 전체 요청 실패 |
| malformed CSS parse diagnostic | `CascadeDiagnostic`; computed style 또는 layout 부분 결과 없음 |
| CSS `@import layer()`, 조건부 layer 순서, inline `style` | 미지원; 전체 요청 실패 |
| `FlexAlignmentV1`에 `@layer` 전달 | 기존 C04.3 allowlist가 계속 거부 |
| view/snapshot generation 또는 revision 불일치 | `SnapshotMismatch` |
| profile이 처리할 수 없는 computed value·텍스트 노드·Taffy 오류 | 구체 오류로 전체 요청 실패 |

허용 선언은 `display`, `box-sizing`, `width`, `height`, `flex-direction`, `flex-grow`, `flex-shrink`, `flex-basis`, `direction`, `row-gap`, `column-gap`, `align-items`, `justify-content`다. layer 내부에서도 동일 목록을 재귀 적용한다.

## Chromium 비교 기준

- Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`; macOS `26.5.1` build `25F80`, arm64, Node `v24.20.0`.
- viewport `301×40` CSS px, scale `1`, locale `en-US`, timezone `UTC`, `screen/light`.
- 16개 정상 입력의 parent computed `align-items`·`justify-content` 문자열과 parent/세 child의 64 frame을 비교한다. 여러 stylesheet에서 같은 named layer를 재사용하는 경우도 포함한다. 모든 좌표·크기는 개별 최대 절대 오차 `0.5 CSS px` 이하다.
- 기준 입력과 Chromium 관찰값은 [cascade-layers.v1.json](../../tests/fixtures/css/c04/cascade-layers.v1.json), [reference](../../tests/fixtures/css/references/c04-cascade-layers-v1.json), [실행 도구](../../tools/css-reference/capture-c04-cascade-layers.mjs)에 고정한다. 출력 파일이 존재하면 수집기는 덮어쓰지 않는다.

## 완료 범위

이는 0025의 제한 profile 구현 완료다. C04 전체의 cascade layers 완성도, 제품 UA stylesheet·제품 runtime 연결, C04의 `@import`·`@scope`, C05 사용자 지정 속성, 일반 앱 CSS, GPU·Android/iOS 적합성 완료를 의미하지 않는다. 자동 실행과 실패 경계는 [실행 근거](evidence/css-c04-cascade-layers-2026-10-09.md)에 기록한다.
