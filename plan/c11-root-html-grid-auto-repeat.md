# C11.1 · parsed Root HTML Grid auto-repeat

**상태:** 구현 후보 검증 중. 공개 CSS 지원이나 C11.1/C11 완료로 표시하지 않는다. 후보 코드는 단일 체크아웃의 `implementation/c11-root-html-grid-auto-repeat-20261105` 브랜치에 있으며 아직 PR이 아니다. 공식 구현 상태는 [STATUS.md](../spec/STATUS.md)만 기준으로 한다.

**선행:** [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md), [Root HTML integer repeat](./c11-root-html-grid-integer-repeat.md), [Root HTML intrinsic tracks](./c11-root-html-intrinsic-grid-tracks.md).

**사전 비교:** [Chrome 156 Agent Browser 비교와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/README.md) · [입력 fixture](../tests/fixtures/css/c11/root-html-grid-auto-repeat-precomparison.html) · [13 scene 기대값](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/normative-expected.json) · [Chrome 관측](../spec/internal/evidence/c11-root-html-grid-auto-repeat-precomparison-2026-11-05/chromium-observation.json).

## 목표

Stylo가 파싱·계산한 CSS 값을 기존 기본 `RuntimeRootHtmlTextPaintV1` parsed Root HTML 경로에서 사용한다. 제한 auto-repeat만 전용 `RootHtmlAutoRepeatV1` layout mode로 전달한다. C11.4 일반 `TrackSizing` 입력 허용 범위, C11.2.3 fixture ABI, 공개 CSS 지원 범위는 바꾸지 않는다.

반복 축은 columns 또는 rows 중 하나만 허용한다. 반복 track list 전체가 하나의 `repeat(auto-fill|auto-fit, …)`이어야 하며 pattern은 이름 없는 `minmax(positive finite px, 1fr)` 한 개 또는 두 개다. 다른 축은 이름 없는 fixed px track 하나다. 반복 축에는 finite positive definite px 크기와 `min-size:auto`·`max-size:none`이 필요하다. 양 축 gap은 normal 또는 finite non-negative px만 허용하고 margin·padding·border는 0이어야 한다. `grid-auto-flow:row`, `grid-auto-rows:auto`, `grid-auto-columns:auto`만 허용한다.

visible direct item은 기본 auto placement, source order, static positioning, auto self-alignment 및 auto aspect ratio여야 한다. 반복에서 만들어지는 explicit cells보다 item이 많으면 implicit track을 만들지 않고 거부한다. `display:none` item은 occupancy에서 제외한다. line names, 이웃 track, integer repeat 결합, 두 축 동시 repeat, areas, dense/column flow, 비율 gap, intrinsic track, 비기본 alignment, overflow, positioned item과 기타 제한 밖 값은 contextual error로 layout publication 전에 닫는다.

Auto-repeat 개수는 CSS Grid §7.2.3.2 규칙대로 계산한다. count 계산에만 UA 1px floor를 적용하고 sizing에는 작성한 positive px minimum을 유지한다. 모든 gutter를 count 산식에 포함하고 expanded track 수는 Taffy `GRID_MAX_TRACKS`에서 제한한다. 산술 overflow와 cell capacity 초과는 partial frame 없이 실패해야 한다.

## 판정 기준

1. 13개 독립 기대 장면과 Chrome 156 관측이 반복 횟수와 item geometry에서 일치한다. 169/170px 반복 경계, 2-track pattern, auto-fit empty/collapsed item, subpixel minimum의 count floor와 visible overflow를 포함한다.
2. Rust focused tests에서 positive grammar, rows/columns, unsupported syntax/style, item capacity와 publication atomicity를 확인한다. 기존 C11.4 auto-repeat regression을 유지한다.
3. 새 native Root HTML fixture를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 앱 route로 실행한다. fixture는 columns 1-track, columns 2-track pattern, rows repeat를 각각 다른 색 item과 고정 geometry로 포함한다. 같은 source hash·layout/draw 결과를 확인하고 원본 bootstrap HTML을 복구한다.
4. Simulator screenshots는 visual smoke로만 분류한다. PixelCopy, Metal drawable readback, hardware GPU, Android 실기기, iOS compositor 이후 pixels 또는 WPT를 실행하지 않으면 완료 근거로 주장하지 않는다.
5. `cargo fmt --all -- --check`, 변경 crate focused tests 및 Clippy를 실행한다. 코드·fixture·evidence를 적대적으로 검토하고 발견을 수정한 뒤 PR을 만든다.

## 제외 범위

임의 auto-repeat grammar, line-name/placement, 여러 repeat group과 이웃 track, intrinsic/percentage sizing, general implicit placement, nested-grid 조합, 일반 CSS Grid 또는 WPT 적합성, 공개 CSS 지원, Android hardware GPU, 실기기 및 iOS compositor-after pixel 동등성. 이 후보의 성공은 C11.1이나 C11 parent를 완료 처리하지 않는다.
