# C12 위치 지정 계획 실패 관점 검토

**검토일:** 2026-10-11 · **범위:** 계획 문서 · **기준:** CSS Positioned Layout Level 3, CSS Flexbox 2025-10-14, locked Taffy 0.14.0, 현재 Spinon layout/style adapter

계획 작성 후 표준 규칙과 구현 코드의 소유 경계를 대조했다. 서로 다른 실패 경로 20개를 확인해 빠져 있던 used display, subtree ancestor, viewport 단위, cache 무효화 조건을 계획에 추가했다. 아래 항목은 기능 구현 검토가 아니며 코드가 동작한다는 증거로 사용하지 않는다.

| # | 실패 경로 질문 | 대조한 사실·발견 | 계획에 반영한 조치 |
| ---: | --- | --- | --- |
| 1 | CSS 기본 `static`을 Taffy 기본값으로 바로 보낼 때 의미가 보존되는가 | locked Taffy 0.14.0 `Position`의 default는 `Relative`이고 enum에 `Static`이 없다. | CSS `static`을 Taffy 기본값에 맡기지 않는 결정과 typed position 계약을 명시했다. |
| 2 | static 중간 ancestor가 absolute containing block을 가로채는가 | CSS는 가장 가까운 positioned ancestor를 찾으며 static 상자는 containing block을 만들지 않는다. Taffy의 단일 parent tree만으로 CSS owner를 대체하면 오판할 수 있다. | source parent·formatting parent·containing-block owner·paint traversal을 분리했다. |
| 3 | 계산 subtree root 밖의 실제 containing block을 놓치는가 | HostDocument layout 요청은 제한된 root에서 시작할 수 있어 ancestor owner가 입력 노드에 없을 수 있다. | 필요한 ancestor style·edge·revision을 snapshot에 포함하고, 없으면 viewport 대체 대신 오류를 요구했다. |
| 4 | containing block을 border/content edge로 잘못 잡는가 | Positioned Layout 3의 일반 block ancestor containing block은 padding edge에서 형성된다. | C07.2 edge geometry와 padding edge 기준을 명시했다. |
| 5 | absolute/fixed element의 computed `display`를 used display로 그대로 쓰는가 | CSS Position 3은 absolute/fixed box를 blockify하고 independent formatting context를 만든다. | computed/used display를 분리하고 blockification을 C12.2·C12.3 계약에 넣었다. |
| 6 | `display:none` 또는 미지원 `display:contents`가 phantom box/owner가 되는가 | none은 box를 만들지 않지만 contents/table/inline fragments는 현재 box topology가 표현하지 않는다. | none의 무효과를 적고 contents/table/inline fragmentation을 명시 오류 경계로 추가했다. |
| 7 | relative offset이 sibling의 normal-flow 배치를 밀어내는가 | relative positioning은 배치 후 시각 offset이며 normal-flow sibling 위치를 재배치하지 않는다. | flow frame과 final visual frame을 구분하고 sibling/부모 크기 불변 비교를 요구했다. |
| 8 | relative offset으로 생긴 overflow를 레이아웃 정확성으로 과장하는가 | relative offset은 ancestor scrollable overflow에 영향을 줄 수 있다. C13 scroll/overflow가 아직 선행되지 않았다. | C13 의존성을 명시하고 scroll/clipping 완료 주장을 금지했다. |
| 9 | top/bottom percentage를 width basis로 계산하는가 | Position 3은 inset percentage를 대응 축 containing-block size로 정의한다. | 각 물리 side의 축 basis를 별도 비교하고 축 혼합을 실패로 정했다. |
| 10 | inset shorthand와 cascade override를 layout adapter에서 재파싱하는가 | Stylo가 computed typed values와 shorthand reset을 이미 소유한다. 별도 원문 parser는 cascade 차이를 만들 수 있다. | Stylo computed side만 입력으로 받고 shorthand/longhand/important/var cascade 비교를 요구했다. |
| 11 | inset `calc()`가 0으로 조용히 치환되는가 | 현재 CSS math property registry는 position inset을 새 property로 등록하지 않았다. | C06 typed-math property와 axis basis를 추가하도록 적었다. |
| 12 | abs auto size를 0/viewport width로 성공 처리하는가 | auto size는 inset, available space, intrinsic/text measure와 상호작용한다. | definite sizing/auto inset/margin 조합을 분리하고 C14/C15 intrinsic 부재 때 ancestor까지 실패하도록 했다. |
| 13 | abs child가 Block/Flex parent의 크기와 line을 바꾸는가 | absolute child는 out-of-flow이며 Flex sizing/line collection에서 빠진다. | normal-flow sizing 불변과 out-of-flow projection을 별도 조건으로 넣었다. |
| 14 | static-position owner를 실제 containing block owner와 동일시하거나 Block/Flex 규칙을 섞는가 | 고정 Position 3은 Block의 static-position rectangle을 static-position containing block의 inline 축 양끝과 hypothetical block-start 위치로 정의하고, Flex 자식은 Flex formatting context의 별도 규칙을 쓴다. 둘 다 실제 CB와 다를 수 있다. | Block은 zero-thickness static-position rectangle과 hypothetical normal-flow 위치를 C12.2에서, Flex 규칙은 C10.3.5에서 따로 계산하고 owner·좌표 변환 fixture를 두도록 보강했다. |
| 15 | Flex abs child를 원래 `order`로 paint하는가 | CSS Display/Flex paint 규칙은 absolute Flex/Grid child를 paint rank 계산 때 `order:0`으로 취급한다. | layout child list와 paint rank를 분리하고 C10.3.5에서 교차 검증하도록 했다. |
| 16 | source/DOM traversal을 layout reparent나 order sort로 바꾸는가 | 시각 순서는 HostDocument child order·DOM traversal을 다시 쓰면 안 된다. | HostDocument 자식 벡터와 NodeId identity 보존을 구조 결정으로 명시했다. |
| 17 | `z-index`를 global sort key 하나로 계산하는가 | stacking context는 중첩·격리를 가진다. positioned `z-index` 외에 position static Flex item 예외가 있다. | stacking context tree, auto/음수/양수 level, static Flex item을 함께 C12.4에서 시험한다. |
| 18 | fixed가 viewport에만 붙는다고 고정해 transformed ancestor를 놓치는가 | 미지원 transform/contain ancestor가 fixed containing block을 바꿀 수 있다. | 전체 ancestor chain을 검사해 미지원 owner 효과를 fail-closed한다. |
| 19 | sticky를 매 프레임 viewport offset으로 흉내 내는가 | sticky는 nearest scrollport·scroll offset·containing block 한계·clip을 요구한다. | C13의 실제 scroll/clip 정보 전까지 구현·완료 주장을 차단했다. |
| 20 | viewport geometry를 Surface pixel/dp로 계산하거나 ancestor 변경 뒤 stale frame을 재사용하는가 | CSS viewport와 GPU texture는 서로 다른 단위이며 ancestor position/style 변화는 positioned descendants 결과를 바꾼다. | CSS px viewport를 EnvironmentRevision에 묶고 position/CB 변경 descendant cache invalidation을 요구했다. |

## 확인한 기준

- [CSS Positioned Layout Module Level 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/) — `position`, containing block, inset, absolute/fixed sizing, sticky scrollport.
- [CSS Flexbox · 2025-10-14 CRD](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/) · [CSS Display 3 · 2026-06-05 CRD](https://www.w3.org/TR/2026/CRD-css-display-3-20260605/) — Flex absolute child와 paint 순서.
- Cargo.lock의 Taffy `0.14.0` 및 설치된 `taffy-0.14.0/src/style/mod.rs`의 `Position` 정의.
- `crates/spinon-layout/src/style.rs`, `crates/spinon-layout/src/calc_tree.rs`, `crates/spinon-style-to-layout/src/` — 현재 layout DTO에 position/inset이 없고 Taffy child relation이 입력 tree에서 생성됨을 확인했다.

검토 반영 뒤 계획은 구현 전 상태다. Chromium fixture·mobile runtime·기능 적합성이 검증된 것은 아니며 C12 공식 체크는 미완료로 유지한다.
