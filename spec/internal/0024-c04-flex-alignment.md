# 0024 · C04.3 Flex 정렬 computed style→Taffy 입력

**계약 버전:** `0.1.0` · **상태:** C04.3 내부 fixture slice 구현 · **제품 API:** 없음 · **제품 CSS 지원:** 미완료

## 목적과 경계

C04.2의 `FlexLayoutV1`은 레이아웃 결과를 기존처럼 유지한다. 새 `FlexAlignmentV1`은 `align-items`·`justify-content` 계산값을 별도 입력 profile로 받아 `LayoutStyle`을 거쳐 Taffy에 전달한다. 같은 함수가 C04.2 profile에 새 값을 몰래 추가하지 않도록 entrypoint도 분리한다.

이 Rust workspace 내부 계약은 앱의 CSS 지원, 제품 runtime 연결, JS/CSSOM API, GPU 화면, Android·iOS 앱 동작을 제공하지 않는다. 고정 fixture의 pass는 C04·S02 전체 완료나 CSS 100% 적합성을 뜻하지 않는다.

## 호출 경계

```rust
pub fn compute_flex_alignment_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError>;
```

함수는 같은 불변 문서 상태에서 만든 `HostDocumentSnapshot`과 `StyloDocumentView`를 받는다. generation·document revision·render-tree revision이 하나라도 다르면 cascade를 시작하지 않는다. CSS viewport의 width·height는 양수 CSS px이고 기기 배율은 좌표에 곱하지 않는다. 선택한 subtree에 텍스트가 있거나 inline `style` 속성이 있으면 전체 요청을 실패한다. 결과에는 computed style profile과 source·style·environment revision을 보존한 layout output이 함께 들어간다.

Stylo cascade 진입점은 `spinon_style::compute_flex_alignment_cascade`이며 결과 profile은 `ComputedStyleProfile::FlexAlignmentV1`이다. `spinon-layout`은 CSS 문법을 받지 않고 `LayoutAlignItems` 및 `LayoutJustifyContent` 값만 받는다.

## 입력 속성

새 profile은 C04.2의 제한 입력 속성에 아래 두 computed property와 author declaration만 더한다.

| 속성 | 허용 computed 값 | Taffy 입력 |
|---|---|---|
| `align-items` | `normal`, `stretch` | `LayoutAlignItems::Stretch` |
|  | `flex-start` | `LayoutAlignItems::FlexStart` |
|  | `flex-end` | `LayoutAlignItems::FlexEnd` |
|  | `center` | `LayoutAlignItems::Center` |
| `justify-content` | `normal`, `flex-start` | `LayoutJustifyContent::FlexStart` |
|  | `flex-end` | `LayoutJustifyContent::FlexEnd` |
|  | `center` | `LayoutJustifyContent::Center` |
|  | `space-between` | `LayoutJustifyContent::SpaceBetween` |
|  | `space-around` | `LayoutJustifyContent::SpaceAround` |
|  | `space-evenly` | `LayoutJustifyContent::SpaceEvenly` |

`normal`의 computed value 문자열은 snapshot에 그대로 보존한다. Flexbox layout mapping에서만 `align-items: normal`은 `stretch`, `justify-content: normal`은 `flex-start`로 처리한다. 기본 `LayoutStyle`은 C04.2와 같은 `stretch` 및 `flex-start` 배치를 보존한다.

새 profile에서도 `display`, `box-sizing`, width·height·flex-basis, flex-direction, direction, flex-grow·flex-shrink, row-gap·column-gap은 C04.2의 제한 parser를 사용한다. px 이외 단위·percentage·grid·wrap·position·margin·border·padding 등은 계속 거부한다.

## 실패 처리

| 입력 | 결과 |
|---|---|
| 속성 선언이 새 profile whitelist에 없음 | cascade 단계에서 `UnsupportedAuthorCss` |
| Stylo parse/cascade diagnostic | `CascadeDiagnostic`; 부분 계산값을 사용하지 않음 |
| 허용 속성의 computed 값이 위 표에 없음 | node·property·원본 값을 포함한 `UnsupportedComputedValue` |
| `baseline`, `start`, `end`, `left`, `right`, `safe`/`unsafe` 조합 | layout 투영에서 실패 |
| `align-self`, `align-content`, `justify-items`, `place-items`, wrapping | whitelist 검사에서 실패. shorthand가 `justify-items`로 확장된 경우에도 실패 |
| 선택 subtree의 HTML `style` 속성 | `UnsupportedInlineStyle` |
| view와 HostDocument snapshot revision 불일치 | 해당 `SnapshotMismatch` |
| 텍스트 노드·잘못된 트리·유효하지 않은 값·Taffy 계산 실패 | 기존 `spinon-layout` 구체 오류를 그대로 반환 |

요청 전체가 실패하면 새 `LayoutOutput`을 반환하지 않는다. 경고·미지원 declaration을 무시한 부분 성공은 허용하지 않는다.

## 비교 기준과 한계

비교 입력과 실행 결과는 [C04.3 Chromium fixture](../../tests/fixtures/css/c04/flex-alignment.v1.json), [Chromium reference](../../tests/fixtures/css/references/c04-flex-alignment-v1.json), [실행 근거](evidence/css-c04-flex-alignment-2026-10-09.md)에 고정한다. 기준은 Chrome `154.0.8037.98` / revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS `26.5.1` arm64, viewport `301×40` CSS px, scale `1`이다.

16개 case에서 computed alignment·direction 값은 문자열 정확 비교한다. 부모와 세 자식의 각 `x`, `y`, `width`, `height` 최대 절대 오차는 `0.5 CSS px`다. fixture는 LTR·RTL, 행·열, 자동 교차 크기, 정렬 keyword와 분배 keyword를 관찰한다. 이 고정 fixture는 negative free space, wrapping, reverse 방향, baseline, safe overflow, writing mode, intrinsic sizing, 텍스트, 실제 GPU/플랫폼 화면의 적합성 기준을 제공하지 않는다.

기준 수집은 `mise exec -- bun run css:reference:c04-flex-alignment`으로 수행한다. 출력 파일이 이미 있으면 덮어쓰지 않는다. input·HTML·기본 CSS·캡처 도구·Chromium 실행 파일의 hash와 브라우저 revision을 reference에 기록한다. `mise exec -- bun run test:css-reference`는 저장된 reference와 저장소 입력 hash 및 case/frame 구조를 확인한다.

## 사용 예

```rust
let result = compute_flex_alignment_style_layout(
    &snapshot,
    &stylo_view,
    root,
    &author_stylesheets,
    css_viewport,
    style_revision,
)?;
assert_eq!(
    result.computed_styles.profile,
    ComputedStyleProfile::FlexAlignmentV1,
);
```

위 예는 내부 adapter 진입점을 설명한다. 제품용 API, 앱 문서 lifecycle 연결, 동적 stylesheet 무효화 또는 최종 GPU 제출 경계는 정의하지 않는다.
