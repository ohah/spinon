# 0027 · C04.6 Flex margin computed-style → Taffy adapter

**계약 버전:** `0.1.0` · **상태:** 내부 Rust API 구현 · **제품 API:** 없음 · **제품 CSS 지원:** 미완료

## 목적과 경계

C04.2의 `FlexLayoutV1` 입력을 보존하면서 cascade 결과의 위·오른쪽·아래·왼쪽 margin을 `LayoutStyle`을 거쳐 Taffy Flex layout에 전달한다. 신규 profile은 `ComputedStyleProfile::FlexMarginV1`이며 신규 entrypoint는 `compute_flex_margin_style_layout`이다.

이 API는 주어진 snapshot에서 한 번 계산하는 동기 내부 함수다. 제품 runtime worker, 자동 style invalidation, JavaScript/CSSOM, text layout, GPU scene 및 Android·iOS 앱 표시를 제공하지 않는다. fixture 성공을 C04 전체나 공개 CSS 지원으로 해석하지 않는다.

## 호출 계약

```rust,ignore
pub fn compute_flex_margin_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError>;
```

`snapshot`과 `view`의 generation, document revision, render-tree revision이 모두 일치해야 한다. viewport 폭·높이·scale은 유한한 양수 CSS 값이어야 한다. 선택한 HostRoot 직속 요소 하위 트리에 text node 또는 inline `style` 속성이 있으면 전체 요청을 거부한다. 성공 출력은 `FlexMarginV1` computed-style snapshot과 같은 source/style/environment revision을 보존한 Taffy layout을 함께 가진다.

## 지원 profile

C04.2의 기존 whitelist를 유지하고 다음 computed property를 더한다.

| 입력 CSS | computed 출력 | Taffy 입력 |
|---|---|---|
| `margin` shorthand | `margin-top`, `margin-right`, `margin-bottom`, `margin-left` | 각 물리 edge의 길이 |
| 네 physical longhand | 같은 이름의 computed value | 대응하는 물리 edge |
| `margin-inline`·`margin-block` shorthand와 네 logical margin longhand | Stylo 계산 physical margin 값 | 위 네 edge로 투영 |

지원되는 선언 집합 안의 `margin` shorthand는 1~4개 값을 Stylo cascade에 맡긴다. logical margin shorthand와 longhand는 `horizontal-tb` writing mode에서 Stylo가 계산한 physical computed value를 사용한다. `writing-mode` 변경 및 다른 입력 속성은 이 profile에서 허용하지 않는다.

computed margin 값은 유한한 CSS px만 Taffy에 전달한다. 음수는 그대로 허용한다. `%`, `auto`, `calc()` 등 최종 computed value가 px가 아니면 node·property·값을 포함한 `UnsupportedComputedValue`로 전체 요청을 실패한다. 미지원 author property나 Stylo diagnostic도 부분 결과 없이 요청 전체를 거부한다. CSS `gap`과 margin은 각각 별도 Taffy 입력이다.

선택한 layout root의 computed margin 네 방향이 모두 0이어야 한다. root는 현재 viewport와 같은 고정 크기로 직접 계산되며 부모 containing block이 없으므로 nonzero root margin은 `UnsupportedRootMargin`으로 거부한다.

기존 `FlexLayoutV1`, `FlexAlignmentV1`, `FlexAlignmentCascadeLayersV1`, `S04FlexPaintV1`의 CSS 허용 범위와 동작은 바꾸지 않는다. 각 profile에서 margin을 사용하면 거부한다.

## 사용 예

```rust,ignore
let output = compute_flex_margin_style_layout(
    &document_snapshot,
    &stylo_view,
    app_root,
    &author_stylesheets,
    css_viewport,
    style_revision,
)?;
assert_eq!(
    output.computed_styles.profile,
    ComputedStyleProfile::FlexMarginV1,
);
```

이 예는 Rust 내부 호출이며 V8 연결, 비동기 실행, 앱 화면 또는 CSSOM API를 제공하지 않는다.

## 비교 기준과 근거

Chrome `154.0.8037.98` / revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS `26.5.1` arm64, locale `en-US`, time zone `UTC`, viewport `301×80 CSS px`, scale `1`을 고정한다. [기본 10 case](../../tests/fixtures/css/c04/margin-layout.v1.json)와 [logical shorthand 3 case](../../tests/fixtures/css/c04/margin-logical-shorthand.v1.json), 각 [Chromium reference](../../tests/fixtures/css/references/c04-margin-layout-v1.json)·[logical shorthand reference](../../tests/fixtures/css/references/c04-margin-logical-shorthand-v1.json)는 physical longhand·1~4-value shorthand·negative margin·LTR/RTL logical shorthand/longhand·gap 상호작용·`display:none`·important cascade를 보존한다.

계산 margin 문자열은 정확히 비교하고 표시 box의 각 `x`, `y`, `width`, `height`는 최대 `0.5 CSS px` 오차를 허용한다. `display:none` 요소 자체는 CSS layout box가 없으므로 해당 요소의 `getBoundingClientRect()` 원점은 geometry oracle에서 제외하며 이후 visible sibling 위치는 비교한다. `%`·`auto`·비-px `calc()`·미지원 속성·inline style·parse diagnostic은 fail-closed한다. 실행 환경·SHA-256·실행 결과는 [C04.6 근거](evidence/css-c04-flex-margin-layout-2026-10-09.md)에 기록한다.
