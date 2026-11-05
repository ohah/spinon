# C11.4 Root HTML intrinsic implicit rows with multiple direct text items

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [single direct-text implicit rows](./c11-root-html-intrinsic-auto-rows-text.md) · [combined intrinsic columns and text rows](./c11-root-html-intrinsic-combined-tracks.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-multiple-text-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/expected.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-auto-rows-multiple-text-plan-review-20.md). **계획 PR 실제 diff 검토:** [30개 관점](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/plan-pr-diff-review-30.md). **구현/runtime 검토:** 구현 후 계획 검토와 겹치지 않는 새 20개 관점으로 기록한다. **구현 PR 실제 diff 검토:** 별도 30개 관점으로 검토한다.

## 결손과 목표

기본 parsed Root HTML runtime은 visible direct text item 하나가 있는 `grid-auto-rows:min-content` scene을 처리한다. Fixed explicit single-track route의 `validate_root_html_intrinsic_auto_row_items`는 두 번째 text-bearing item을 명시적으로 거부한다. 같은 owner 안에서 서로 다른 text measure가 각각 다른 implicit row의 min-content contribution이 되는 장면을 기본 runtime 경로로 검증할 수 없다.

기존 text painting·paragraph measurement·Grid track sizing·Taffy min-content 계산을 재사용해 visible direct text item을 여러 개 허용한다. 새 runtime profile, FFI selector, Grid grammar, CSS parser, public API 또는 test-only success path는 추가하지 않는다. 이번 slice는 기존 고정 explicit track route의 text-item cardinality/content gate를 확장하고, 각 text item's source identity와 row contribution이 별도로 보존되는지 확인한다.

## 비교 기준과 독립 기대값

규범 기준은 [CSS Grid Layout Level 1, 2025-03-26 CRD §7.6, §11.1, §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)다. §7.6의 `grid-auto-rows`는 explicit row 다음 implicit rows의 sizing function을 정하고, §11.1/§11.5의 track sizing은 각 auto-placed item의 min-content contribution을 해당 row의 base size에 반영한다.

Chrome 156 사전 비교에는 하나의 40×40px Grid, 40px column, 10px explicit row 및 반복 `min-content` implicit rows가 있다. 첫 direct item은 10px fixed-height control이다. 다음 두 direct `<div>`는 각 하나의 ASCII text child와 같은 monospace 10px / 10px line-height / `white-space:normal` style을 갖는다. `A A A A A A`는 40px resolved width에서 두 줄, `A`는 한 줄이다. 마지막 direct item은 fixed 34px다. 독립 계산한 행 크기는 `[10,20,10,34]px`다. Agent Browser에서 text item 순서만 뒤집은 대조도 확인했다. 행 크기가 `[10,10,20,34]px`로 바뀌고 각 frame은 text item을 따라가며 마지막 item y=40px는 유지된다.

| 항목 | 기대 frame (CSS px) | 행 contribution |
| --- | --- | ---: |
| 첫 고정 대조 item | `[0,0,40,10]` | explicit row 10px |
| 여섯 token text | `[0,10,40,20]` | min-content block 20px, 2줄 |
| 한 token text | `[0,30,40,10]` | min-content block 10px, 1줄 |
| 마지막 고정 대조 item | `[0,40,40,34]` | 고정 item 행 34px |

40px Grid의 visible content bottom은 74px로 Grid box보다 34px 아래다. 고정 `grid-auto-rows:10px` 대조군의 행은 `[10,10,10,10]px`이고 마지막 item은 y=30px에서 시작하며 content bottom은 64px다. 독립 계산, 고정 browser 관측, native runtime 측정은 각각 구분해 기록한다.

## 지원 경계

- Root HTML 기본 runtime의 visible Grid owner 하나와 visible direct HTML `<div>` item만 허용한다. `grid-auto-flow:row` sparse placement, LTR `horizontal-tb`, static position, start content alignment, 양수 고정 CSS px의 Grid width/height, margin/padding/border 0, `overflow:visible`을 사용한다. named area/line, explicit placement, auto-repeat, implicit columns, dense/column flow, non-zero gap, box edge 또는 min/max sizing은 지원하지 않는다.
- 양수 고정 px explicit column 하나와 explicit row 하나를 사용한다. 반복 `grid-auto-rows:min-content`만 이 route의 intrinsic implicit-row 함수다. 기존 explicit intrinsic-column 결합 동작은 그대로 두며 이 계획에서 범위를 늘리지 않는다.
- Direct item은 유한한 0 이상 크기의 빈 `<div>` 대조 item 또는 nested markup 없이 direct text child 하나만 가진 text `<div>`다. 숨김·분리 item은 기존 visibility와 ownership 처리를 따른다.
- Text item은 기존 computed style 계약을 유지한다: width/height auto, min-size auto, max-size 없음, `aspect-ratio` 없음, static position, 자동 placement, monospace 10px, line-height 10px, `white-space:normal`, `white-space-collapse:collapse`, `text-wrap-mode:wrap`, `word-break:normal`, `overflow-wrap:normal`, LTR `direction`, `horizontal-tb` writing mode, `text-transform:none`, `text-align:start`; text node, owner, style/layout revision이 같은 snapshot에 있어야 한다.
- Source text 범위는 현재 여섯 token 문자열에서 ASCII `A` token 하나 이상을 U+0020 SPACE 하나씩으로 구분한 문자열로 넓힌다. 시작/끝 공백은 허용하지 않는다. 이 범위 안에서 글꼴과 line-break model을 유지하면서 item별 measured line count를 다르게 한다. 다른 문자 집합, 임의 Unicode, markup, 명시적 줄바꿈, 공백 이외의 whitespace는 계속 거부한다.
- 기존 checked node/item/track/resource 제한 내에서 조건을 만족하는 visible text item을 하나 이상 허용한다. 새로 무제한 크기의 입력을 만들지 않으며, 10,000 explicit/implicit track 상한 초과는 frame 계산 전에 거부한다. item-style lookup은 한 번 만든 node index를 사용해 다중 item에서 반복 선형 탐색이 되지 않게 한다. 이름만 바꾼 단일-item 제한이나 고정 contribution 추정값을 추가하지 않는다.
- 미지원 item/source/style/topology와 resource limit 실패는 frame이나 paint를 게시하기 전에 layout 전체를 거부한다. 허용 item마다 `NodeId`, source order, text layout revision과 대응되는 row contribution identity를 보존한다.

## 구현과 검증 요구

1. 제한 ASCII text grammar 검사 helper를 추가하고 단일 text item 상한을 없앤다. checked count arithmetic, 현재 item/owner/style/revision 검증은 유지한다.
2. Rust projection tests는 서로 다른 줄 수를 가진 text item 두 개, source-order 역전, hidden item 처리, 두 번째 item의 잘못된 content/style, 다중 direct text child, nested inline markup 및 partial frame/paint 없는 원자 거부를 검사한다. 10,000-track 허용 경계와 10,001-track 원자 거부를 큰 입력에서 무거운 text layout을 호출하지 않고 확인한다.
3. Layout/style tests는 두 text item의 row-axis intrinsic contribution record 내 identity, min-content contribution, 최종 `[10,20,10,34]px` track, item frame, source order 및 visible overflow를 검사한다. 행 크기 계산에는 고정 fixture 값 대신 기존 text measurement engine을 사용한다.
4. 기본 `RuntimeSession` test는 고정 fixture hash를 사용하고 layout/cascade revision 일치, 두 text paragraph와 paint entry, 각 item의 정확한 frame/줄 수, track trace contribution identity, draw order 및 최종 overflow를 검사한다. 고정 px 대조군은 sizing 차이를 확인하는 control로 둔다.
5. Focused Rust tests, 변경 crate scoped Clippy, `cargo fmt --all -- --check`, 기본 artifact/fixture 경계 및 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 빌드·실행을 각각 기록한다. Simulator 결과는 같은 fixture SHA를 사용해 layout, 두 item의 frame/줄 수, draw/present 및 screenshot을 확인한다. 해당 검증이 없으면 실기기, hardware GPU, PixelCopy/Metal readback, compositor 이후 pixel, WPT 또는 전체 C11.4를 주장하지 않는다.
6. 서로 다른 실패 경로 20개를 새 implementation/runtime review로 검토하고, 실제 구현 PR diff도 별도 실패 경로 30개로 검토한다. 결함을 고친 뒤 영향을 받는 assertion과 evidence를 재실행한다.
7. Merge 후 내부 계약 0059, `spec/STATUS.md`, roadmap source와 Tailnet 제공본을 제한 동작·commit hash·검증 한계에 맞춰 갱신한다. C11.4/C11은 미완료로 두고 이 후보로 공식 완료 수를 변경하지 않는다.

## 제외와 완료 판정

이 slice는 임의 text/script, nested/anonymous Grid text, inline formatting, replaced element, 복수 Grid owner, variable font, 일반 CSS admission, WPT, public CSS 지원, 전체 Grid track sizing, C11.4/C11 완료 또는 공식 완료 수 증가를 뜻하지 않는다. 두 contribution을 한 row로 합치거나, item identity/order를 잃거나, 거부 item이 있는 상태에서 partial frame을 게시하면 통과가 아니다.

계획 검토·PR·구현은 현재 main checkout 하나에서 순차 진행한다. 별도 worktree와 병렬 implementation PR을 만들지 않는다. Agent Browser를 사용한 검증은 끝난 뒤 browser process를 종료한다.
