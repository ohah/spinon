# C11.1 · parsed Root HTML Grid 교차축 부분 자동 배치

**상위:** [C11 CSS Grid 계획](./c11-css-grid.md) · [parsed Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [한 축 자동 배치](./c11-root-html-grid-one-axis-auto-placement.md) · [mixed explicit/one-axis 배치](./c11-root-html-grid-mixed-explicit-one-axis.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 사전 비교와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/README.md) · [기대 frame](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/expected.json) · [Agent Browser capture](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/capture.json) · [Chrome 화면](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/chrome-156-agent-browser.png) · [SHA-256](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/SHA256SUMS.txt)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-mixed-cross-axis-plan-review-20.md) · **실제 계획 PR diff 적대 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/plan-pr-diff-review-20.md).

## 목표와 범위

기존 기본 Root HTML Grid profile은 한 Grid에서 fully definite numeric item과 한 방향의 one-axis item은 함께 둘 수 있지만, row-locked item과 column-locked item이 함께 있으면 거부한다. 이 slice는 같은 no-area, `grid-auto-flow: row` sparse Grid owner에서 두 종류의 one-axis item을 함께 배치한다. 새 `RootHtmlMixedCrossAxisAutoPlacementV1` mode로 기존 placement profile을 구분하고, 서로 다른 item이 row와 column에 각각 고정된 입력을 CSS Grid의 placement phase에 맞춰 정규화한다.

새 mode를 선택하려면 `display:none`을 제외한 visible direct-item 집합에 row-locked item과 column-locked item이 각각 하나 이상 있어야 한다. 집합의 나머지 item은 fully definite numeric placement 또는 fully auto placement를 사용할 수 있다. 같은 축만 있는 입력은 기존 route를 유지한다. `visibility:hidden` item은 placement와 occupancy에 포함된다. HostDocument/DOM child vector와 paint/source order는 재정렬하지 않는다.

## 구현 전 비교 모델

규범 근거는 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)다. Row-flow sparse algorithm은 fully definite items를 먼저 배치하고, row-locked items를 order-modified document order로 배치한 다음 남은 items를 처리한다. 이 후속 단계에서 definite column이 있는 item은 해당 column을 유지하고 row cursor를 진행하며, fully auto item은 sparse cursor를 사용한다. `visibility:hidden`은 grid item이므로 occupancy를 차지하고 `display:none`은 grid item을 만들지 않는다. W3C normative wording과 독립 셀 점유 기대값은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/README.md)에 분리했다.

고정 입력은 4×3 explicit grid, 각 track `40px`, `160×120px` definite content box, 0 gap·edge, horizontal-tb/LTR, `grid-auto-flow:row`, empty static direct block items다. Agent Browser HeadlessChrome 156, viewport `360×800`, DPR 1로 다섯 positive scenes의 22 visible frames와 implicit-row negative boundary를 관찰했다. 독립 기대 frame과 Chrome 값은 모두 0 CSS px 차이다. 이는 브라우저 사전 비교이며 Spinon 동작 결과가 아니다.

## 허용 입력과 거부 경계

- 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` route에서 Grid owner 하나씩 판정한다. no-area, non-empty finite non-negative fixed-px explicit row/column track list와 definite finite non-negative width/height를 요구한다.
- `grid-auto-flow: row`만 허용한다. `dense`, column flow, named line/span, named area, auto-repeat 및 미지원 track grammar는 기존 node/property 진단으로 거부한다.
- Item placement는 fully definite numeric placement, row-locked (definite numeric row / auto column), column-locked (auto row / definite numeric column), 또는 fully auto여야 한다. 양·음 numeric line은 explicit grid 내부로 정규화하고 numeric span은 양수만 허용한다. 단일 endpoint가 auto와 numeric/span으로 섞인 축, named endpoint, non-positive span은 거부한다.
- `display:none` subtree는 Grid item 분류·occupancy 전에 제외한다. `visibility:hidden`은 보이지만 않는 참여 item으로 유지한다. `order`는 0으로 고정하며 order-modified order의 별도 구현은 범위 밖이다.
- Root는 zero gap/edge, visible overflow, 기본 alignment, LTR, static positioning을 요구한다. Item은 empty, non-replaced, non-nested direct block이며 width/height는 auto 또는 finite non-negative fixed px다. Text, anonymous item, nested/replaced item, positioned item, margin/padding/border, aspect ratio, nonzero order는 거부한다.
- 계산 결과가 explicit grid 밖의 implicit row/column을 필요로 하면 전체 layout/render output을 원자적으로 거부한다. `auto` 또는 fixed-px `grid-auto-*` 입력 자체가 implicit extent를 허용한다는 뜻은 아니다.
- 같은 visible item 집합이 row-only 또는 column-only이면 새 selector로 보내지 않는다. 완전 auto-only grid도 새 selector 대상이 아니다. 기존 one-axis/auto route 결과를 회귀 대조한다.

## 구현 순서와 수용 기준

1. `GridPlacementMode`에 `RootHtmlMixedCrossAxisAutoPlacementV1`을 추가하고 validator/normalizer의 mode 분기를 분리한다. 기존 `RootHtmlOneAxisAutoPlacementV1`와 `RootHtmlMixedExplicitOneAxisAutoPlacementV1` 계약을 변경하지 않는다.
2. Selector는 각 parsed Root HTML Grid owner의 visible direct item을 `display:none` filtering 뒤 분류한다. row-locked와 column-locked 유형이 각각 있는 동일 owner만 새 mode를 선택한다. Grid owner 간 items나 revision/state를 공유하지 않는다.
3. Taffy sparse row placement 결과를 고정 oracle과 대조한다. Fully definite occupancy, row-locked phase 우선순위, column-locked cursor progression, fully auto cursor progression, row/column spans와 negative line resolution을 각각 검사한다. DOM child order는 바뀌지 않아야 한다.
4. Chrome fixture matrix에 positive `dom-cross-axis`, `mixed-spans`, `hidden-controls`, `display-none-control`, `reverse-order` 장면과 implicit-row rejection을 연결한다. 독립 기대값, Chrome capture, Root HTML parsed runtime frame, layout trace를 같은 source revision으로 비교한다. Positive frame tolerance는 `0.5 CSS px` 이하, 고정 fixture의 target 값은 0 CSS px 차이다.
5. Failure controls는 mixed item이 다른 Grid owner에 분산된 경우, row-only/column-only/auto-only route selector, auto와 numeric endpoint 혼합, invalid/zero span, named line/area, dense/column flow, implicit row/column extent, hidden subtree, nonzero order, positioned/nested/text/replaced item, stale revision, duplicate/missing NodeId, allocator 실패 후 partial output 부재를 포함한다.
6. Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 positive scene, root 안/track boundary/표시된 item/clear 영역 표본을 검증한다. Source revision과 installed artifact hash를 묶는다. Android software renderer와 iOS compositor 제출 전 drawable 한계를 분리해 기록하고 실기기/hardware GPU 결과로 확대하지 않는다.
7. Focused Rust layout/selector/Root HTML integration checks, 기존 C11.1 Grid regression, `cargo fmt --all -- --check`, 변경 crate scoped Clippy, Android/iOS Simulator build/runtime과 PR 본문 screenshot을 보존한다. 넓은 suite 실패는 변경 밖이라고 추정하지 말고 baseline 여부와 함께 별도 기록한다.
8. 구현 후 새 runtime/code 관점 20개와 실제 source diff 관점 20개를 각각 수행한다. 이 계획 검토를 구현 검토로 재사용하지 않는다. 프로젝트 규칙에 따라 구현 PR은 병합 전 별도 30개 실패 관점으로 검토한다.
9. 구현과 merge metadata를 `spec/STATUS.md`, 내부 계약 0059, C11 계획 및 public roadmap에 반영한다. C11.1/C11 체크와 공식 전체 완료 `72/213`은 유지한다. 내부 계약 버전 `0.1.0`을 변경하지 않는다.

## 제외 범위

Public CSS support 전체, WPT 적합성, arbitrary track sizing, named lines/areas, dense/column flow, order-modified item order, implicit track sizing, text/replaced/nested items, positioned Grid items, hit testing/accessibility order, Android 실기기, hardware GPU, iOS compositor 이후 pixel parity 및 C11.1/C11 완료는 주장하지 않는다.

## 2026-11-04 구현 후보 checkpoint

구현은 `2b7bb7bc2f70dcff6be782a82a881cb6dcfb5351` 기반 checkout의 `feat/c11-root-html-grid-mixed-cross-axis-20261104` branch에서 진행했다. 구현 source commit은 `fb4135c3a1c8052d30bef83bf773c50dc677c9c8`, 최종 PR head는 `335805d171a8a3079089fcef68a5e1e69bf59f37`이다. 구현 PR [#1152](https://github.com/ohah/spinon/pull/1152)는 2026-11-04T17:59:44+09:00 Rebase and merge됐고 merge SHA는 `6fe4980533f3da1e7250ff00fdb4d21e541a3a55`다. Runtime/code review 20개, source diff review 20개와 독립 PR diff review 30개를 각각 수행했다.

- Layout mode·placement validator·shared sparse normalizer 연결, owner-scoped selector 및 Root HTML projection을 구현했다. 기존 one-axis/mixed-explicit route는 별도 selector/mode로 유지했다.
- Chrome 156 precomparison의 5 positive scene·22 visible frames 및 implicit-row negative boundary에서 독립 기대값과 관측값은 최대 0 CSS px 차이다.
- Focused Rust 결과: mixed-cross-axis layout 4/4, one-axis placement layout 13/13, C11.1 Root HTML projection/integration 10/10. 변경된 `spinon-layout`/`spinon-style-to-layout` scoped Clippy와 rustfmt가 통과했다. iOS/Android source boundary test는 14/14다.
- Android API 37 ARM64 AVD/SwiftShader runtime은 `layout=ready boxes=27`; SurfaceView PixelCopy 26/26 exact. iPhone 17 Pro/iOS 26.2 Simulator runtime은 `layout=ready boxes=27`; Metal drawable 26/26 exact. Flag-off Android APK DEX와 iOS executable에서 새 test marker가 없고, opt-in artifact positive scans에서는 marker가 있다. iOS 표본은 compositor 제출 전이다. [화면, logs, build/install hashes](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-runtime-2026-11-04/README.md)
- Initial `border:0` declaration은 제한 computed-style admission에서 거부되어 fixture에서 제거했다. Agent Browser에서 geometry가 그대로임을 재비교한 뒤 두 앱 경로를 다시 빌드/실행했다. 기본 bootstrap asset이 들어간 첫 APK도 hash로 발견해 fixture asset이 포함된 candidate APK를 재빌드했다. iOS default binary scan에서 unguarded argument prefix를 찾아 compile flag 안으로 옮겼고 default/opt-in binary를 다시 빌드해 대조했다. iOS 첫 replay에서 entry asset 이름을 틀린 실행과 모든 pre-fix log는 final 성공 cohort와 구분해 보존한다.
- 현재 실행은 시뮬레이터/AVD뿐이다. PixelCopy AVD는 software renderer이고 iOS drawable readback은 compositor 전이다. 실기기/hardware GPU, 전체 CSS Grid/WPT, 일반 CSS admission 및 상위 C11.1/C11 완료는 주장하지 않는다.

상태: 제한 구현·focused Rust 검증·Android/iOS Simulator runtime·default/opt-in artifact 경계·구현 review 20개씩·PR diff review 30개·PR 본문 screenshot을 확인했다. PR #1152는 Rebase and merge됐다. PR check rollup은 비어 있고 main required status checks도 설정되지 않아 CI 통과로 기록하지 않는다. C11.1/C11 상위 완료 체크와 공식 `72/213`은 그대로 둔다.
