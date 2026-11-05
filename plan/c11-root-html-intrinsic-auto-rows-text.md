# C11.4 Root HTML direct text가 있는 `min-content` implicit row

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [parsed Root HTML intrinsic explicit tracks](./c11-root-html-intrinsic-grid-tracks.md) · [기본 implicit `min-content` row slice](./c11-root-html-intrinsic-auto-rows.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 156 Agent Browser 결과](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-text-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/expected.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-auto-rows-text-plan-review-20.md). **계획 PR 변경 검토:** 실제 diff를 대상으로 별도 기록한다. **구현/runtime 검토:** 계획 검토와 분리해 20개를 기록한다. **구현 PR 변경 검토:** PR마다 독립된 관점으로 검토한다.

## 결손과 목표

기본 parsed Root HTML 경로의 [기존 `grid-auto-rows:min-content` 제한 구현](./c11-root-html-intrinsic-auto-rows.md)은 비어 있는 direct `<div>` 항목만 허용한다. 같은 Grid에 text가 있는 direct block `<div>` 항목이 오면 `grid-auto-rows` 후보 전체를 거부한다. 따라서 이 경로의 intrinsic implicit row는 text line layout에서 나온 min-content block contribution을 반영하지 않는다.

이번 계획은 기본 `RuntimeRootHtmlTextPaintV1` 경로에서 단일 intrinsic row pattern `grid-auto-rows:min-content`를 유지하면서, direct Grid item 하나에 속한 단일 text child가 min-content row 높이에 기여하도록 고정된 하위 집합을 연결한다. 기존 text paint·measurement 경로를 재사용하며 CSS parser, 새 runtime profile, FFI selector, public API 또는 테스트 전용 성공 경로는 추가하지 않는다.

## 지원 경계

- Grid owner는 하나이며, 보이는 direct HTML `<div>` item, `grid-auto-flow:row`, sparse auto-placement, LTR `horizontal-tb`, visible overflow만 허용한다.
- Explicit column과 row는 각각 유한한 양의 고정 CSS px track 하나다. 첫 번째 빈 item은 explicit row에 배치한다. Implicit row에는 반복되는 단일 `min-content` track만 허용한다.
- Grid에는 양수의 definite 고정 CSS px width/height, row·column gap 0, margin·padding·border 0, start content alignment, static position을 사용한다.
- 보이는 direct item은 크기가 고정된 빈 `<div>` 또는 text child 하나만 가진 block `<div>`다. Anonymous raw Grid text, 중첩/inline element, 여러 text run, replaced item, 숨김/분리된 item은 제외한다.
- Text item에는 `width:auto`, `height:auto`, `min-height:auto`, `max-height:none`, `font-family:monospace`, `font-size:10px`, `line-height:10px`, `white-space:normal`, static positioning을 사용한다. Source text는 ASCII `A A A A A A`로 고정하며 text item은 하나만 허용한다.
- Positive fixture는 40×40px Grid, explicit 40px column 하나, explicit 10px row 하나, gap 0, 높이 10px인 첫 item, 두 줄 text item, 높이 34px인 마지막 item이다. Browser 기준 row는 `[10,20,34]px`이고 visible overflow는 y=64px까지 이어진다.
- Placement longhand는 모두 `auto`여야 한다. Named line/area, span, repeat, implicit column, dense/column flow, `start` 이외의 content alignment, non-zero gap, min/max constraint, `aspect-ratio`, writing-mode 변경과 복수 owner는 원자적으로 거부한다.
- Text layout 값은 기존 runtime 글꼴 경로에서 측정한다. 일반 `monospace` advance는 플랫폼마다 다를 수 있으므로 Chrome과 simulator의 글꼴 지표를 따로 기록한다. CSS `monospace`에 고정 advance가 있다고 웹 공통 상수로 추정하지 않는다.
- 기존 10,000-track 및 checked-count 상한을 유지한다. 지원하지 않는 text topology 또는 sizing 값은 frame이나 paint scene을 게시하기 전에 property와 node를 식별하는 오류를 낸다.

## 기준과 독립 기대값

기준은 [CSS Grid Layout 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)와 고정된 [Chrome 156 Agent Browser 관측](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/chrome156-observation.json)이다. Chrome의 40px column과 10px monospace inline metrics에서 text는 두 줄로 나뉘며 각 line-height는 10px다. 독립 sizing 기대값은 min-content block contribution 20px, explicit row 10px, 고정된 마지막 item 행 34px다. 기대 frame은 `[0,0,40,10]`, `[0,10,40,20]`, `[0,30,40,34]`; Grid content의 visible bottom은 y=64px다.

같은 fixture에 `grid-auto-rows:10px`를 임시 대조값으로 적용하면 computed row는 `[10,10,10]px`, text item frame 높이는 10px, 마지막 item y는 20px다. 이는 intrinsic contribution 결과와 다른 대조이며 glyph overflow가 잘리는 픽셀 결과라고 주장하지 않는다. 고정 item 크기와 authored track은 모든 target에서 정확히 비교한다. Text에서 계산한 높이는 플랫폼별 font line count와 layout trace로 기록하고 각 플랫폼의 실제 default-route 측정값에 대조한다. Chrome screenshot은 reference geometry이지 Spinon, WPT, glyph raster 또는 native surface 증거가 아니다.

## 구현 요구

1. 기존 owner-scoped Root HTML intrinsic auto-row classifier가 `min-content` row와 text-bearing direct `<div>` item을 다른 Grid mode의 우선순위를 바꾸지 않고 선택한다.
2. Computed `white-space`, font size/family, line-height, item `NodeId`, text-child ownership, style revision과 layout revision이 같은 snapshot에 속해야 한다. CSSOM 문자열을 재파싱해 typed value를 만들지 않는다.
3. Implicit-row preflight가 text contribution을 고정 추정값으로 대체하지 않는다. Row sizing은 기존 text measure와 Taffy min-content algorithm을 사용하며 track trace에서 유한한 base, row line, contribution identity를 확인한다.
4. Text item이 없거나 숨겨진 경우, CSS-valid 미지원 값, 여러 text child, nested inline markup, anonymous Grid text, stale revision, 잘못된 owner, 미지원 track list 또는 resource-limit 실패가 partial frame/paint publication을 만들지 않는지 검증한다.
5. Rust layout/projection/runtime tests는 implicit row 한 개와 두 개, 두 줄 text, item frame, paint order, visible overflow와 explicit dimension의 정확한 값을 확인한다. Browser와 native text metric이 같은 font advance라고 단정하지 않는다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 HTML source hash로 기본 app route를 실행해 `layout=ready`, 관측 가능한 root/item frame, draw/present와 screenshot을 남긴다. Harness가 text NodeId frame이나 track trace를 내보내지 않으면 앱에서 직접 확인했다고 쓰지 않는다. PixelCopy/Metal readback은 실제 수집한 경우에만 별도로 기록한다.
7. Focused Rust tests, 변경 crate의 scoped Clippy, `cargo fmt --all -- --check`, default-artifact/fixture-boundary 검사 및 두 simulator build/runtime 결과를 각각 기록한다. 알려진 broad failure는 baseline과 나란히 두고 green으로 보고하지 않는다.
8. 계획 자체를 구현 전에 새로운 실패 관점 20개로 검토한다. 구현/runtime도 별개의 20개 관점으로 검토하며 구현 PR diff는 또 다른 관점으로 적대 검토한다. 각 PR에는 사용자가 요청한 20개 검토를 포함한다.
9. Merge 뒤 내부 contract 0059, C11 plan, `spec/STATUS.md`와 Tailnet roadmap preview를 실제 merge SHA에 맞춘다. 이 slice만으로 C11.4/C11 checkbox 또는 공식 `78/214`를 올리지 않는다.

## 제외와 완료 판정

Anonymous direct Grid text item, inline descendant, shaping/ligature, 임의 Unicode/script, `normal` 이외의 `white-space`, 임의 font property, text transform, replaced image, `aspect-ratio`, min/max transfer, explicit intrinsic track과 implicit min-content row의 조합, auto-sized container axis, intrinsic `grid-auto-columns`, 다른 implicit row function, gap/box edge, WPT, 일반 CSS admission, C11.4 및 C11 전체 완료는 범위 밖이다. 고정 추정값만으로 box를 키우거나, 실패 시 text를 숨기거나, sibling frame 일부만 게시하는 구현은 이 계획을 충족하지 않는다.

구현은 현재 checkout에서 순차 진행하며 worktree를 만들지 않는다. 계획 PR과 구현 PR은 따로 검토·병합한다. 검증 실행 뒤 Agent Browser process를 종료한다.
