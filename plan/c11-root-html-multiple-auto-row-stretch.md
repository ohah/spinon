# C11.4 parsed Root HTML 두 implicit `auto` 행 stretch

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [단일 implicit auto-row stretch](./c11-root-html-grid-auto-row-stretch.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §§7.6, 7.6.1, 8.5, 11.3, 11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). §11.8은 이 축의 content distribution이 `normal` 또는 `stretch`일 때 남은 양의 definite free space를 `auto` max track sizing function이 있는 행들에 균등 분배한다.

**구현 전 비교:** [HeadlessChrome 157 관측 및 독립 기대값](../spec/internal/evidence/c11-root-html-multiple-auto-row-stretch-precomparison-2026-11-07/README.md).

**계획 실패 관점 검토:** [20개](./c11-root-html-multiple-auto-row-stretch-plan-review-20.md). **계획 PR diff 검토:** 이 계획 PR의 실제 변경분에서 별도의 실패 경로 30개를 확인한다. 구현 뒤에는 새로운 runtime/code 20개와 구현 PR diff 30개를 따로 검토한다.

## 결손과 목표

PR #1228의 제한 profile은 기본 parsed Root HTML route에서 고정 explicit row 뒤의 implicit `auto` row 하나가 text minimum contribution을 받은 뒤 남은 definite free space를 채우는 장면을 연결한다. auto track이 하나뿐이면 남은 공간 전부가 그 track에 가므로 여러 `auto` track 사이의 균등 분배는 아직 검증되지 않았다.

이미 병합된 `RootHtmlAutoRowStretchV1` 경계를 정확히 세 direct item까지 넓혀 implicit `auto` 행 두 개의 남은 공간 균등 분배를 확인한다. 새 public API, CSS parser, runtime selector, profile ID 또는 fixture FFI는 추가하지 않는다. Legacy `RootHtmlIntrinsicTrackSizingV1`에서는 `grid-auto-rows:auto` 거부를 계속 유지한다.

## 고정 입력과 독립 기대값

사전 비교 fixture는 [root-html-multiple-auto-row-stretch-precomparison.html](../tests/fixtures/css/c11/root-html-multiple-auto-row-stretch-precomparison.html)이다. 표시되는 Grid owner는 하나이며 content box는 40×40 CSS px, explicit column 하나는 40px, explicit row 하나는 10px이다. `grid-auto-rows:auto`, row auto-flow, gap 0, `align-content:normal`, padding/border 없음, visible overflow를 사용한다. Direct child는 source order대로 40×10px fixed empty item 하나, `A` direct text item 하나, `B` direct text item 하나다. 두 text item은 10px/10px monospace 한 줄이다.

독립 기대값은 [Chrome 및 독립 JSON](../spec/internal/evidence/c11-root-html-multiple-auto-row-stretch-precomparison-2026-11-07/expected.json)에 고정했다. Explicit row base 10px과 각 implicit auto row의 10px text minimum contribution을 합하면 초기 track base는 `[10,10,10]px`다. Definite 40px content height에는 10px free space가 남는다. 두 auto max track이 각각 5px을 받아 used rows `[10,15,15]px`가 된다. Direct item frame은 `[0,0,40,10]`, `[0,10,40,15]`, `[0,25,40,15]`이며 각 text item은 한 줄이다.

Controls는 track base sizing과 stretch 단계를 분리한다. `align-content:start`, `grid-auto-rows:min-content`, fixed `10px` rows는 모두 `[10,10,10]px`를 유지한다. Implicit auto row가 하나인 control은 `A A A A A A` 두 줄 text를 사용하고, 기존 `[10,30]px` 장면을 유지한다. HeadlessChrome 157 관측은 각 control을 교차 확인하지만 기대값 권위는 규범에서 계산한 산술에 둔다.

## 구현 범위와 거부 경계

- 기본 `RuntimeRootHtmlTextPaintV1` route 및 표시되는 Grid owner 정확히 하나만 허용한다. LTR `horizontal-tb`, row auto-flow, static direct `<div>` items, definite 40×40px content box, explicit column 40px 하나와 explicit row 10px 하나, gap 0, padding/border 없음, visible overflow, `grid-auto-rows:auto`를 요구한다.
- Source order가 고정된 visible direct child 정확히 세 개만 허용한다. 첫 child는 40×10px fixed empty item, 두 번째는 source가 정확히 `A`인 direct text item, 세 번째는 source가 정확히 `B`인 direct text item이어야 한다. 두 text item은 auto sizing, 기본 alignment, 10px/10px monospace 한 줄 및 normal whitespace를 사용한다. 허용 text grammar를 기존보다 넓히지 않는다.
- 두 auto track은 Taffy 입력에서 implicit 상태를 유지해야 한다. Track trace는 explicit row 하나와 implicit auto rows 두 개의 경계를 구분하고 각각의 10px minimum contribution과 `StretchAutoTracks` 단계를 보존해야 한다. Used rows는 `[10,15,15]px`여야 한다. 독립 계산이 각 track에 +5px이 분배됨을 정한다. Text/item frame, source order, NodeId 및 style/layout revision도 서로 일치해야 한다.
- Positive acceptance는 기본 `align-content:normal` 장면뿐이다. `align-content:start`는 Chrome 비교 control이며 허용 profile 밖이다. 다른 정렬, item 추가/누락, 네 번째 visible child, 빈 text 또는 여러 줄 text, nested/inline text, source 변경, 다른 track size, gap, border/padding, placement declaration, 여러 Grid, 미지원 style, 방향 또는 writing mode는 새 layout을 게시하기 전에 원자적으로 거부한다.
- 기존 single implicit auto-row, min-content row 및 fixed-row 경로를 그대로 유지한다. 거부 입력은 partial frame, draw command, trace를 게시하지 않으며 같은 RuntimeSession의 다음 정상 요청은 성공해야 한다.
- 기존 opt-in runtime fixture-test 경계를 사용한다. 기본 release artifact에는 fixture 전용 source marker와 symbol이 계속 없어야 하며, FFI/API 변경은 계획하지 않는다.

## 구현·검증 순서

1. 계획과 사전 비교만 포함한 PR을 먼저 올리고 계획의 20개 검토와 실제 PR diff 30개 검토가 끝난 뒤 병합한다.
2. Projection gate는 세 child 고정 shape만 허용하도록 넓힌다. 기존 text measurement 및 Grid track sizing을 재사용하고 기대 10px contribution이나 5px stretch를 코드 상수로 넣지 않는다.
3. Style-to-layout과 RuntimeSession에서 positive/control/rejection/recovery를 집중 검증한다. Explicit/implicit row count, item별 contribution, stretch 단계, used tracks 및 frame을 대조한다.
4. 기존 single-auto-row와 intrinsic-row 회귀, placement/resource limits, same-session recovery, output revision consistency 및 default/fixture artifact 격리를 확인한다.
5. Focused Rust tests, 변경 crate Clippy 및 `cargo fmt --all -- --check`를 실행하고 기존 broad failure와 후보 결과를 분리한다.
6. 동일한 기본 Root HTML fixture를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. Fixture hash를 묶고 layout-ready frames, track trace, draw/present를 기록한다. Screenshot은 visual smoke이며 surface pixel parity가 아니다. Overflow/border claim은 별도 fixture와 readback 근거가 있어야 한다.
7. 구현/runtime 실패 관점을 새 20개로 검토하고 구현 PR diff를 별도 30개로 검토한다. 가능한 경우 앱 screenshot을 구현 PR 본문에 넣는다.
8. 구현 PR 병합 후 내부 계약 0059, `spec/STATUS.md`, public roadmap과 사용자가 지정한 Tailnet preview를 갱신한다. C11.4/C11 체크와 공식 완료 `78/214`는 그대로 둔다.

## 명시적 미포함

이 작업은 임의 개수/크기의 implicit tracks, 한 row 안 여러 content contribution, author input으로서의 `align-content:stretch`, 일반 auto-placement, 임의 text/font/language, nested/inline/replaced content, percentage/calc tracks, gap, border, padding, overflow policy, 일반 CSS Grid admission, WPT 적합성, Android 실기기 또는 hardware GPU parity, 전체 C11.4를 구현하지 않는다. 내부 interface version `0.1.0`을 유지한다.
