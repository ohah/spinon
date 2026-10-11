# C12.3 viewport fixed 위치 계획 검토

이 검토는 `plan/c12-3-fixed-positioning.md`가 구현에 들어가기 전에 CSS 의미, 현재 Spinon 구조, 고정 Chromium 동작, 모바일 환경 경계를 놓치지 않았는지 서로 다른 실패 경로로 대조한다. 계획을 수정한 뒤 현재 남은 선행 산출물과 완료 조건을 다시 확인했다.

## 기준 자료

- [CSS Positioned Layout 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/): fixed와 absolute containing block, inset, fixed/static-position 의미.
- [CSS Transforms 1](https://www.w3.org/TR/css-transforms-1/) 및 [CSS Containment 2](https://www.w3.org/TR/css-contain-2/): fixed descendant containing block을 만드는 transform·containment 동작.
- 고정 Chromium `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 [`LayoutObject::ComputeIsFixedContainer`](https://chromium.googlesource.com/chromium/src/+/b859317bf11f6be47f9b7799ec690a0a42a1fb33/third_party/blink/renderer/core/layout/layout_object.cc).
- WPT commit `9ec154ff43db468923997c08bb08f905ceab62a5`의 viewport fixed, fixed의 absolute 자손, transformed sibling test source. WPT suite는 실행하지 않는다.
- 현재 저장소의 [C12.2 계약 0055](../0055-c12-2-absolute-block.md), [C10.3.5 계약 0056](../0056-c10-3-5-positioned-flex.md), `CssViewport`, Taffy 0.14 owner·synthetic viewport 구현, revision 검증 경로.

## 계획 검토

| # | 별도 실패 관점 | 확인 및 계획의 결정 |
|---:|---|---|
| 1 | 앱 root frame이 viewport보다 작거나 원점이 달라도 fixed child가 root에 붙는가 | containing block은 `CssViewport`의 CSS px 원점·크기라고 고정했다. root와 viewport가 같은 크기라고 가정하지 않으며 root frame 크기·원점 차이를 비교 입력에 둔다. |
| 2 | Android dp·iOS point·GPU texture pixel·safe area가 viewport CSS px와 섞이는가 | 각 단위를 별도 기록하고 CSS viewport만 layout basis로 쓴다. safe-area와 물리 surface 크기는 지원 의미로 넣지 않는다. |
| 3 | scroll API가 없는데 fixed가 스크롤에도 화면에 고정된다고 오인되는가 | 이 단계 완료 주장은 초기 frame·resize까지만이다. scroll 불변은 C13에서 별도로 닫는다. |
| 4 | `fixed`를 `absolute`와 같은 입력 값으로 덮어써 향후 owner·paint 의미가 사라지는가 | Spinon DTO는 `Fixed`를 보존하고 Taffy 경계에서만 `Absolute`로 투영한다. |
| 5 | `relative` 또는 `absolute` ancestor가 fixed child를 잘못 소유하는가 | viewport-only profile에서 일반 positioned ancestor는 fixed containing block이 아니라고 명시했다. 중첩 ancestor fixture를 둔다. |
| 6 | fixed child가 fixed parent에 고정되는가 | fixed parent는 그 자체만으로 fixed containing block이 아니다. fixed owner와 absolute owner를 분리하고 fixed descendant도 viewport를 사용하게 했다. |
| 7 | fixed parent의 absolute child가 viewport에 잘못 붙는가 | `fixed` box는 absolute descendant의 containing block이 될 수 있으므로 `absolute_owner`는 fixed NodeId를 가질 수 있다고 고정했다. |
| 8 | `transform`, `translate`·`rotate`·`scale`, `perspective`, `transform-style`을 놓치는가 | Blink의 pinned computed predicate를 effect inventory의 기준으로 삼고 transform 관련 효과를 fixed-CB 표식에 넣었다. 현재 profile은 해당 author 선언을 지원하지 않는다. |
| 9 | `filter` 또는 `backdrop-filter`가 viewport fallback으로 조용히 잘못 계산되는가 | W3C/Blink 기준의 non-initial 효과를 명시적으로 차단 목록에 넣고 오류 경로를 요구한다. |
| 10 | `contain:layout/paint`, `content-visibility`, shorthand 및 `will-change`가 owner를 바꾸는가 | 적용된 layout/paint containment와 `content-visibility`가 활성화하는 containment, Blink가 고려하는 `will-change` effect를 검증 목록에 넣었다. 지원 whitelist에서 누락하면 안 되는 실패 fixture로 고정했다. |
| 11 | stylesheet만 preflight하고 inline `style`이 fixed-CB 효과를 우회하는가 | author stylesheet source preflight와 inline preflight를 독립 확인하고 양쪽 negative fixture를 계획에 추가했다. |
| 12 | preflight 호출을 우회한 snapshot에서 Taffy 계산이 viewport를 잘못 owner로 받는가 | computed snapshot에 effect 표식을 보존하고 projection/layout 경계에서 ancestor 표식을 다시 검사하도록 계획을 보강했다. |
| 13 | CSS fixed CB를 만드는 Blink의 특수 box가 일반 element처럼 처리되는가 | ViewTransition root, canvas-for-drawing, SVG foreignObject, UA text control 내부, anonymous fieldset topology를 이번 profile에서 제외하고 지원 밖 입력의 성공 판정을 금지했다. |
| 14 | percentage inset이 viewport가 아니라 nearest source parent나 높이/너비가 뒤바뀐 축을 쓰는가 | left/right와 top/bottom의 viewport 축 basis를 명시하고 non-square viewport 기준을 고정했다. |
| 15 | 모든 inset `auto`에서 fixed box의 static position을 임의로 0에 놓는가 | static-position 의존 축을 이번 성공 경로에서 제외하고 오류로 닫는다. Flex static-position과 현재 C12.2 범위를 섞지 않는다. |
| 16 | auto size·min/max·margin·border·padding이 C12.2 검증 없이 새 경로에서 달라지는가 | C12.2의 측정 가능한 box subset만 재사용하고 같은 회귀 suite 및 새 viewport Chrome 비교를 요구한다. intrinsic text/replaced 입력은 거부한다. |
| 17 | `display:none` subtree를 viewport synthetic root로 올려 화면에 되살리는가 | hidden ancestor·self를 viewport child 추출 전 검증하고 frame·render entry가 없음을 판정하도록 했다. |
| 18 | 계산 parent를 바꾸는 과정에서 HostDocument parent·source order·NodeId owner가 손상되는가 | viewport synthetic root는 계산 트리 전용이며 source/event/lifetime graph를 바꾸지 않는 조건을 완료 기준에 넣었다. |
| 19 | resize 결과보다 늦게 끝난 이전 environment 계산이 새 화면에 제출되는가 | environment revision과 CSS viewport를 함께 확인하고 stale completion·빠른 resize 왕복에서 새 revision만 제출해야 한다고 정했다. 오류/부분 결과는 전체 계산을 실패시킨다. |
| 20 | DPR만 달라졌는데 CSS frame이 움직이거나, 캡처/시뮬레이터를 실제 pixel·scroll 검증으로 과장하는가 | same viewport DPR-only 비교는 CSS geometry 불변으로 판정하고 surface scale은 별도 로그에 둔다. Android 실기기와 iOS Simulator 확인 범위도 분리했다. |

## 반영한 정리

- fixed의 CSS 의미를 Taffy의 `Absolute`에 흡수하지 않고 DTO에 보존한다.
- absolute owner와 fixed owner를 분리하고, fixed box의 absolute 자손 owner 규칙을 별도로 명시한다.
- stylesheet/inline preflight 뒤에도 computed snapshot과 projection에 effect 검증을 둔다.
- viewport CSS px와 OS·GPU 좌표를 분리하고 resize, DPR-only, stale 결과에 독립 판정을 둔다.
- static-position, scroll, paint order, stacking, hit testing을 이 단계의 성공 주장 밖에 둔다.

## 남은 구현 전 게이트

- 구현 전 Chrome 비교 산출물은 PR #135에서 병합했다. 22개 case·79개 node, 2개 viewport와 DPR 1·2·2.625·3, resize 왕복·no-op가 고정되어 있다. 이 사전 비교는 런타임 테스트를 대신하지 않고 WPT subset도 실행하지 않았다.
- computed effect 추출 seam은 `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs`의 Stylo `ComputedValues`에서 확인했다. Stylo 0.22.0의 `content-visibility`는 Gecko 전용이므로 generated longhand API에 없고 computed snapshot에서 추출할 수 없다. 이 속성은 author preflight에서 거부하며, 직접 계산 가능한 effects는 projection 경계에서도 다시 검사한다.
- Android 실기기와 iOS Simulator의 동작 검증은 기능 구현 뒤에만 한다. 이번 계획 검토는 런타임·플랫폼 구현 검증이 아니다.
