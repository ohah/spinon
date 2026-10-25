# 0083 · C13.4 scrollbar gutter geometry 내부 계약 초안

- **계약:** `0.1.0-draft` · **상태:** 플랫폼 표시 mode 미확정, 구현 전
- **제품 CSS 지원 선언:** 아님
- **계획:** [C13.4 첫 조각 · stable gutter](../../plan/c13-4-scrollbar-gutter.md)
- **비교 기준:** [Chrome 관찰과 범위 제한](evidence/c13-4-scrollbar-precomparison-2026-10-26/README.md)

## 계약 목표

이 초안은 C13.2가 제공하는 scroll owner/scrollport/offset snapshot을 C13.4 scrollbar gutter와 연결하기 전의 경계를 고정한다. 현재 제품·Simulator 환경이 classic scrollbar를 그리는지 overlay scrollbar를 쓰는지 확인되지 않았으므로, 구현용 API나 지원 선언을 정의하지 않는다.

## 결정되어야 할 입력

실행 환경은 다음 표시 mode 중 하나를 명시해야 한다.

- `Classic { used_width_css_px }`: 유한하고 0보다 큰 UA/OS scrollbar width가 필요하다.
- `Overlay`: gutter 폭은 0이다.

누락/중복 mode, 음수·NaN·무한대 폭, edge 위치를 판정하지 못하는 writing mode는 fail-closed해야 한다. Desktop Chrome 관찰값 15px 또는 11px을 Android/iOS 제품 폭으로 재사용하면 안 된다.

## 안정 배치 의미

고정 기준 [CSS Overflow Level 3 §4.2, 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#scrollbar-gutter)에 따르면 overlay scrollbar에는 gutter가 없다. Classic scrollbar에서 `stable`은 `hidden`/`scroll`/`auto` scroll container에 overflow 여부와 무관하게 gutter를 유지한다. `auto`의 gutter 선택은 overflow/scrollbar 상태를 따른다. `both-edges`는 선택된 inline edge 반대편에 동일 gutter를 둔다.

초기 profile 후보는 `horizontal-tb`·LTR Block element의 두 축 `overflow:scroll` scroll owner 및 `scrollbar-gutter:auto|stable|stable both-edges`뿐이다. `scrollbar-width/color`, scrollbar drawing/dragging, root viewport/body propagation, RTL/vertical mode, `overflow:auto`/`hidden`, grid/flex sizing interaction은 별도 계약과 관찰 전까지 범위 밖이다.

## C13.2 snapshot 불변조건

고정된 mode와 gutter token으로 계산한 reservation은 descendant layout, `scrollport_css_px`, scrollable extent/range, clip, paint와 hit-test에서 같은 revision으로 관찰되어야 한다. outer border-box는 유지한다. 새 scroll range가 이전 CSS px offset을 포함하면 offset을 보존하고, 범위를 벗어나면 기존 C13.2 clamp/reset 규칙을 적용한다. style/geometry/mode 변경 중 일부만 새 snapshot으로 게시할 수 없다.

Overlay mode의 성공은 0 gutter와 C13.2 기존 state/geometry의 exact equality다. Classic mode positive control은 지정 platform에서 실제 사용자 표시/기하 효과를 확인한 다음에만 구현 관문을 열 수 있다. 플랫폼에서 변화가 보이지 않으면 임의 scrollbar나 gutter UI를 만들지 않는다.

## 구현 상태

현재는 draft다. Android API 37 ARM64 AVD와 지정 iOS Simulator에서 표시 mode, computed style, scroll geometry 및 실제 화면 차이가 확인되기 전에는 fixture-test feature, FFI, layout code 또는 공식 상태 대장 변경을 허용하지 않는다. C13.4나 parent C13의 공식 완료 여부를 이 문서만으로 바꾸지 않는다.
