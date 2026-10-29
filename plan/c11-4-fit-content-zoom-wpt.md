# C11.4 · zoomed `fit-content()` WPT assertion runtime 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 범위:** 같은 pinned WPT의 automatic-minimum, spanning, percentage/calc argument, implicit-column, auto-row 사례는 각각 선행 제한 slice에 기록되어 있다. 이 계획은 그중 zoomed grid assertion 하나만 다룬다.

**사전 비교:** [고정 source, Chrome 156 관측, 독립 기대값](../spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29/README.md) · [machine-readable expected](../spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29/expected.json)

**상태:** 계획·사전 비교 단계다. 구현, app runtime, Android/iOS Simulator는 아직 실행하지 않았다. C11.4 checkbox와 공식 완료 `72/213`을 유지한다.

## 목적과 범위

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 `zoomedGrid` assertion 하나를 기존 opt-in C11.4 style→layout→runtime fixture 경로에 연결한다. 고정 원문은 root `zoom:2`, `height:300px`, `grid-auto-rows:fit-content(40px)`, `min-height:0` item 및 400px child를 사용하고, `getComputedStyle(zoomedGrid).gridTemplateRows`가 `40px`인지 확인한다.

Runtime fixture는 width를 60px로 고정한다. WPT의 original root와 width-normalized clone은 Agent Browser에서 모두 `gridTemplateRows:40px`, item visual height 80px, child visual height 800px이었다. 따라서 width 정규화는 이 block-axis assertion을 바꾸지 않는다. Runtime은 root-local zoom 2를 computed style과 같은 revision으로 보존하고, fixture의 normalized/unzoomed track result 40px과 zoomed visual row 80px을 서로 다른 값으로 검사한다.

이 계획은 일반 CSS `zoom` admission, 임의 ancestor/descendant zoom composition, zoom·percentage/calc/auto-repeat의 조합, zoom 아래 text wrapping, viewport/page scale 통합, WPT harness 전체 실행 또는 전체 C11.4 완료를 목표로 하지 않는다. 고정 입력이 기존 projection으로 이 assertion을 검증하지 못하면 기대값을 바꾸거나 zoom을 무시한 채 통과시키지 않는다. 이 계획의 실현 가능성이나 계층 간 값이 달라지면 범위를 다시 좁히고 별도 근거로 설명한다.

## 독립 기대값

CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 `fit-content(limit)` 공식은 `max(minimum, min(limit, max-content))`이다. Direct item의 `min-height:0`은 normalized fixture의 minimum contribution 0px을 만들고 child height 400px은 normalized max-content contribution 400px을 제공한다. Unzoomed fixture coordinates에서 `fit-content(40px)` 결과는 `max(0, min(40, 400)) = 40px`이다. CSS Viewport Editor's Draft [§4](https://drafts.csswg.org/css-viewport/#zoom-property)는 zoom이 used length를 배율만큼 바꾸며, DOM/CSSOM의 computed style length는 unscaled로 반환한다고 설명한다. 따라서 이 slice의 layout contract는 normalized track result 40px과 zoomed visual row 80px을 분리한다. WPT가 직접 assert하는 값은 `gridTemplateRows:40px`; visual geometry는 app integration assertion이며 Chrome 관측과 별도로 대조한다.

Chrome observation과 규범 계산은 [사전 비교 자료](../spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29/README.md)에서 구분한다. Chrome observation을 normative expectation으로 승격하지 않는다.

## 제한된 runtime input

Exact scene ID는 `fit-content-zoomed-grid-wpt`로 둔다. Adapter는 root/item/child의 HTML namespace·tag, parent/order, node 수, computed styles, root zoom, track value, same document/style/environment/layout revision을 모두 확인한다.

- Root: `display:grid; width:60px; height:300px; grid-template-columns:60px; grid-template-rows:none; grid-auto-rows:fit-content(40px); gap:0; zoom:2`.
- Direct item: `min-height:0`, auto row placement, child 한 개.
- Child: `height:400px`.
- App diagnostic paint는 background-color만 사용하며 sizing input을 바꾸지 않는다.
- Root authored/computed zoom은 exact finite `2`; 다른 값, 다른 node의 zoom, class/style/tree mutation, extra node, 다른 scene/profile은 layout·trace·draw 전 거부한다.
- Fixture adapter는 Taffy의 unzoomed CSS coordinate 결과와 root-local visual scale을 명시적으로 분리한다. Unzoomed rows는 `[40]px`, visual root/item/child height는 각각 600/80/800px이어야 한다. Transform origin은 root top-left이며 이 고정 장면의 root origin은 `(0,0)`으로 둔다. Frame·surface 경로가 이 contract를 보존하지 못하면 CSSOM rows만으로 runtime scene을 PASS 처리하지 않는다.

Zoom은 `RuntimeBlockFlexGridTrackSizingV1` 전체의 author allowlist로 승격하지 않는다. 필요하면 fixture-test feature 안에 zoom용 cascade/route를 추가하고, Stylo가 계산한 zoom을 실제 snapshot·matcher에 전달한다. Inline source 문자열을 읽어 layout output만 40px로 고정하는 구현은 허용하지 않는다.

## 수용 기준

1. Pinned WPT revision/hash와 Agent Browser WPT harness 13/13, target assertion 및 width-normalization probe를 evidence에 고정한다. Chrome WPT pass는 Spinon pass와 구분한다.
2. Stylo cascade가 같은 revision에서 root computed zoom `2`와 expected Grid input을 보존하는지 Rust regression으로 확인한다. `zoom:1`, `1.5`, `3`, 다른 zoom owner, missing/extra node, changed track/min-height/child size 및 stale revision은 fail-closed한다.
3. `fit-content(40px)`의 unzoomed used row `[40]px`, min contribution 0px, max-content contribution 400px, trace and layout revision을 exact 비교한다. WPT assertion과 app runtime 내부 stage trace의 역할을 섞지 않는다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS Simulator에서 root/item/child visual frames를 각각 확인한다. 정규화 입력에서 visual heights는 root 600px, item 80px, child 800px이어야 한다. Android는 software AVD, iOS Metal은 compositor 제출 전 readback으로 기록한다.
5. Android PixelCopy와 iOS Simulator Metal drawable에서 root, item, row 바깥 child, child 끝 뒤 surface의 고정 좌표를 확인한다. Screenshot만으로 zoom/overflow를 판정하지 않는다.
6. Default/product FFI archive에는 신규 fixture scene ID, source payload, symbol이 없어야 하며 fixture-test archive에는 positive control로 존재해야 한다. 정적 archive 결과를 APK/IPA 전체 제외로 확대하지 않는다.
7. Focused Rust/style/runtime/FFI tests, fixture boundary, `cargo fmt --all -- --check`, 변경 영향받은 Clippy 및 Android/iOS Simulator builds를 기록한다. 미실행 검사는 통과로 보고하지 않는다.
8. 구현/runtime은 계획 리뷰와 다른 새로운 실패 관점 20개로 검토하고, implementation PR diff는 별도의 20관점으로 검토한다. 결함을 수정하면 영향을 받은 검토 관점을 재실행한다.
9. Implementation PR은 Android/iOS screenshot과 범위를 설명한다. Agent Browser로 PR image 링크를 확인한 뒤 해당 session을 닫는다. Merge 뒤 내부 계약·상위 계획·STATUS·공개 roadmap·Tailnet preview를 제한 결과에 맞춰 갱신하며 C11.4 unchecked와 `72/213`을 유지한다.

## 제외 범위

일반 CSS zoom 지원, multiple zoom ancestors, zoomed auto-repeat, zoomed percentage/calc arguments, zoomed text/intrinsic wrapping, transforms, viewport page scale, CSSOM 일반 구현, full WPT runner, physical device/hardware GPU, iOS compositor-after pixels, C11.4 및 C11 전체 완료는 이 slice로 증명하지 않는다.

## 계획 실패 관점 검토

별도의 [계획 리뷰 20개](./c11-4-fit-content-zoom-wpt-plan-review-20.md)는 사전 비교와 구현 전 경계를 대상으로 한다. 이 표를 implementation/runtime 검토나 PR diff 검토에 재사용하지 않는다.
