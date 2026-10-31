# C11.2.2 · 행 우선 sparse Grid auto-placement의 `order` 순서

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.2.2 행 우선 sparse 자동 배치](./c11-2-2-sparse-row-placement.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 사전 비교와 독립 기대값을 고정했다. 계획 자체를 별도 실패 관점 20개로 검토한다. 계획 PR의 변경 검토는 구현/runtime 검토와 별도로 30개 관점으로 수행한다. 이 제한 후속은 C11.2.2 또는 C11 상위 checkbox를 완료로 승격하지 않는다.

## 목표와 지원 경계

기존 C11.2.2 runtime profile은 computed Grid item `order`가 0이 아닌 값을 거부한다. CSS Grid 1은 Grid auto-placement를 order-modified document order로 진행하므로, 이 후속은 기존 `RuntimeBlockFlexGridAutoPlacementV1` profile에서 행 우선 sparse auto-placement만 제한 확장한다.

order가 0이 아닌 경우 visible direct child가 모두 `auto/auto` placement여야 하며 각 item은 고정 track 안의 고정 크기 leaf여야 한다. 양수·음수 order를 오름차순으로 정렬하고 같은 order는 기존 DOM 순서를 유지한다. 정확히 두 고정 scene을 사용한다. 첫 scene은 세 열에 A=2, B=-1, C=0을 주어 B·C·A 순서로 놓고, 두 번째 scene은 네 item 중 B와 C가 -1로 같아 source-order tie를 확인한다. 두 scene의 item은 10×10px이고 margin·padding·border 0, transform 없음, definite fixed track을 사용해 겹치지 않는다.

변경은 계산용 auto-placement 순서만 바꾼다. HostDocument의 DOM child list, source/accessibility 순서 및 renderer의 겹치는 item paint order는 이 계획에서 재정렬하지 않는다. 두 scene은 auto-placement가 서로 겹치지 않는 10×10px cell에만 놓인다. 이 profile·scene은 일반 CSS 지원을 선언하지 않는다. Dense/column flow, order가 있는 definite 또는 mixed placement, spanning item, overlap/stacking, intrinsic sizing 및 C11.4 sizing 동작은 계속 거부·제외한다.

## 기준 모델과 사전 비교

- 규범 근거: [CSS Grid Layout Module Level 1, 2025 CRD §6.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#order-property)에서 `order`가 auto-placement와 painting order에 적용된다고 정하며, [§8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)에서 auto-placement가 order-modified document order를 사용한다고 정한다.
- 고정 3열 10px, 10px 행, 10px `grid-auto-rows`, 0 gap, 고정 10×10px leaf, row sparse 입력을 쓴다. 독립 계산은 computed `order` 오름차순이며 동률은 DOM 순서다.
- Chrome 156 Agent Browser 관측과 입력 hash는 [사전 비교 근거](../spec/internal/evidence/c11-2-2-grid-item-order-precomparison-2026-10-31/README.md)에 고정했다. `mixed`는 A=(20,0), B=(0,0), C=(10,0); `ties`는 A=(20,20), B=(0,20), C=(10,20), D=(0,30)이다.
- Chrome DOM·computed style·rect 관측은 oracle일 뿐 Spinon 구현 증거가 아니다. 두 scene 모두 item이 비겹치므로 이 관측은 CSS paint-order 변경을 검증하지 않는다.

## 예정 변경

1. `crates/spinon-layout/src/grid_auto_placement.rs`의 row sparse 정규화에서 visible item을 안정적으로 computed `LayoutStyle.order` 오름차순으로 처리한다. definite/mixed placement가 하나라도 있을 때 nonzero order를 허용하지 않도록 global gate를 둔다. 입력 `LayoutNode.children`는 바꾸지 않는다.
2. `crates/spinon-layout/src/grid.rs`에서 order 지원을 `GridPlacementMode::SparseRow` 및 모든 visible item이 완전 자동 배치인 경우로 한정한다. 판정은 normalized `auto_placed_items`가 아니라 원본 `LayoutInput.grid_items`에서 `None | GridItemInput::is_auto()`로 한다. normalized set에는 partially auto item도 들어갈 수 있다. Dense/template/track-sizing profile, definite·mixed placement, hidden-state 불일치와 지원 외 조합은 Taffy 계산 전에 `UnsupportedGridOrder`로 실패해야 한다.
3. `crates/spinon-layout/src/calc_tree.rs`의 Taffy 내부 Grid child 순서를 order-modified document order로 안정 정렬한다. 동률은 source order다. `LayoutStyle.order` 주석도 Flex/Grid 계산용 값임을 정확히 적는다. Flex 기존 경로, source HostDocument, display-none filtering과 synthetic text node의 기본 order 0을 회귀시키지 않는다.
4. `crates/spinon-style-to-layout/src/c11_2_2_grid_auto_placement_tests.rs`에 고정 HTML source를 추가한다. Stylo computed `order` 문자열과 `LayoutStyle.order` typed 값, node identity, DOM child order, auto placement frame을 비교한다. `parse_order`가 현재 `i32` 범위를 벗어난 computed serialization을 `UnsupportedComputedValue`로 거부하는 negative test를 고정한다. `RuntimeBlockFlexGridAutoPlacementDenseV1` 등 C11.2.3 이후 profile의 기존 reject 경계를 확인한다.
5. `crates/spinon-layout/src/grid_auto_placement_tests.rs`에 mixed signed order, stable tie, DOM child 순서 보존, order=0 회귀, hidden child, definite/mixed placement negative control을 추가한다. unsupported input은 부분 placement/frame 없이 실패해야 한다.
6. 기존 C11.2.2 FFI host route에서 별도 versioned order runtime matrix와 두 scene을 호출한다. 기존 16-scene matrix/reference의 내용을 바꾸거나 hash를 덮어쓰지 않는다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 scene의 computed/typed order, 10px track과 item frame, layout revision, draw/present를 확인한다. PixelCopy와 Metal drawable 내부 표본은 box 위치·색을 확인하며 glyph, iOS compositor 이후 pixels 또는 겹침 paint order의 증거가 아니다.
7. 제품 기본 artifact와 opt-in fixture-test artifact의 FFI/module 경계를 확인한다. 새 scene source/marker는 opt-in 경로에만 있어야 한다. 관련 focused Rust/style/FFI 검사, `cargo fmt --all -- --check`, 영향 crate Clippy, Android/iOS simulator builds를 실행한다.
8. 구현/runtime 실패 경로는 기존 계획 리뷰와 별개인 새로운 20개 관점으로 기록한다. 최종 PR diff는 별도 30개 관점으로 검토한다. 계획 PR도 계획 20개와 독립적인 PR diff 30개 검토 후 허용된 Rebase and merge로 병합한다.

## 계획 실패 관점 검토

[계획 실패 관점 20개](./c11-2-2-sparse-row-order-plan-review-20.md)를 구현 전에 별도로 기록한다. 기존 C11.2.2/C11.2.3 계획 검토는 이 계획의 검토 수에 포함하지 않는다.

## 실패 기준과 잔여 한계

- Chrome oracle 또는 독립 계산과 다른 순서·좌표, 동률의 unstable reorder, source child list 변경은 실패다.
- nonzero order가 Dense/column flow, definite 또는 mixed placement, span/implicit track-sizing profile로 통과하면 실패다.
- 잘못된 style/property, duplicate node, hidden child, stale revision, unsupported placement에서 partial frame/track/trace가 남으면 실패다.
- Android/iOS의 computed value, typed order, frame, revision, draw/present 또는 box interior 표본이 서로 다르면 실패다. 두 simulator는 실제 Android/iOS 기기 증거가 아니다.
- CSS `order`의 overlapping paint/stacking semantics, keyboard/speech/accessibility order, 전체 CSS Grid, WPT 전체 적합성, 실기기·hardware GPU 및 C11.2/C11 완료는 확인하지 않는다.
- C11.2.2/C11 checkbox와 공식 완료 집계 `72/213`은 유지한다.
