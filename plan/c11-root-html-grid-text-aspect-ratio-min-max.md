# C11.1 · parsed Root HTML Grid direct-text `aspect-ratio` min/max

**상태:** 제한 구현 병합. PR [#1182](https://github.com/ohah/spinon/pull/1182)은 2026-11-05T12:25:28Z Rebase and merge됐고 merge SHA는 `8dab763943215e7fc7b5250a8eb22dddba5e811f`다. 기반 [direct-text `aspect-ratio`](./c11-root-html-grid-text-aspect-ratio.md) 및 유한 비율 값 확장을 기본 `RuntimeRootHtmlTextPaintV1` 경로에서 재사용하되, 이 계획의 min/max 적용 범위를 별도로 검증한다. C11.4의 opt-in fixture 결과만으로 기본 경로 지원을 주장하지 않는다. PR 병합 뒤에도 C11.1/C11 공식 체크와 완료 집계는 상위 범위 충족 전까지 미완료로 유지한다.

**기준과 구현 후보 근거:** [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-min-max-precomparison-2026-11-05/README.md) · [Android/iOS Simulator runtime 증거](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-min-max-runtime-2026-11-05/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-min-max-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-min-max-precomparison-2026-11-05/expected-values.json)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.1 intrinsic track sizing](./c11-root-html-intrinsic-grid-tracks.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 문제와 목표

기본 parsed Root HTML Grid에서 direct text item의 finite-positive numeric `aspect-ratio`가 definite width/height와 함께 자동 크기에 반영된다. 현재 ratio collector는 owner와 target item의 `min-width`, `max-width`, `min-height`, `max-height`가 초기값일 때만 이 경로를 허용한다. 이 계획은 직접 px min/max가 ratio-derived item 크기와 auto row에 미치는 영향을 제한 입력으로 연결한다.

고정 Grid는 width `70px`, columns `[10px, 60px]`, rows `[auto]`, zero gap, row sparse flow다. 정확히 두 direct block-text item `A`와 `AAAA`를 두며 `AAAA` 하나만 ratio를 가진다. 기존 definite dimension 두 방향을 유지한다: `(width:60px,height:auto)`와 `(width:auto,height:30px)`. Ratio 제약 입력은 정확한 computed `2 / 1`이고 ratio sizing box는 기존 `box-sizing` typed 값과 일치해야 한다.

수용할 item min/max는 각 물리 축의 `auto`/`none` 또는 finite non-negative direct px `0..=70px`다. 명시한 min/max가 모두 있을 때 min이 max보다 크면 computed CSS sizing 결과대로 min이 우선한다. 제한은 ratio 대상 item에만 적용하며 Grid owner와 `A` 대조 item은 기존 제약을 유지한다. 상한 `70px`은 fixed 60px track과 70px Grid 밖으로의 visible overflow를 확인할 수 있게 한다.

## 사전 비교 기준

기준은 [CSS Grid Layout 2025 CRD §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)와 [CSS Sizing Level 4 2026 WD §4.2](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#automatic-sizes)다. 두 문서는 각각 CRD와 Working Draft다. 고정 browser 관측과 독립적인 ratio 전이 후 px min/max clamp 산술을 함께 사용하며 이 후보를 전체 CSS conformance로 확대하지 않는다.

Chrome 156 Agent Browser에서 10 scene을 실행했다. 10px system `monospace`는 `AAAA` intrinsic width `24.09375px`였다. 고정 Grid columns는 모든 장면에서 `[10,60]px`다.

| Scene | `AAAA` computed sizing | 기대 item frame | 판별점 |
|---|---|---:|---|
| `width-ratio-baseline` | `60px × auto`, `2/1` | `60×30` | baseline |
| `width-ratio-min-height` | 위와 `min-height:40px` | `60×40` | ratio height 뒤 minimum 적용 |
| `width-ratio-max-height` | 위와 `max-height:20px` | `60×20` | ratio height 뒤 maximum 적용 |
| `width-ratio-minmax-height` | 위와 `min-height:40px; max-height:20px` | `60×40` | min-over-max 우선 |
| `width-control-min-height` | `60px × auto`, ratio 없음, `min-height:40px` | `60×40` | non-ratio control |
| `height-ratio-baseline` | `auto × 30px`, `2/1` | `60×30` | baseline |
| `height-ratio-min-width` | 위와 `min-width:70px` | `70×30` | Grid 바깥 10px overflow |
| `height-ratio-max-width` | 위와 `max-width:50px` | `50×30` | ratio width 뒤 maximum 적용 |
| `height-ratio-minmax-width` | 위와 `min-width:70px; max-width:50px` | `70×30` | min-over-max 우선 |
| `height-control-max-width` | `auto × 30px`, ratio 없음, `max-width:50px` | Chrome intrinsic `24.09375×30` | non-ratio control |

전체 root/item rectangles, computed ratio/min/max, row/column track values와 관측을 비교한 JSON은 사전 비교 README에 고정했다. Geometry는 browser 기준과 `0 CSS px` 차이를 요구한다. System font control은 Android/iOS에서 OS별 intrinsic width가 다를 수 있으므로 ratio 없는 intrinsic width를 cross-platform equality gate로 쓰지 않는다.

## 제한 입력 계약과 구현 경계

- 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`의 기존 ratio owner-scoped 경로만 확장한다. C11.4 fixture profile, 일반 computed-style projection, 다른 Grid mode와 CSS parser admission은 넓히지 않는다.
- Ratio owner는 non-nested `div` Grid, width `70px`, auto height, columns `[10px,60px]`, auto row, zero gap, `grid-auto-flow:row`, `align/justify-items:start`, 기본 horizontal-tb/LTR 및 visible overflow여야 한다.
- Owner는 direct child 두 개만 갖는다. 첫 item은 `A`, 둘째는 `AAAA`인 HTML block text다. Ratio와 min/max 확장은 둘째 item에서만 허용한다. 두 item은 기존 10px `monospace`/`Ahem`, `line-height:10px`, nowrap, static positioning, zero edge, auto placement 조건을 지킨다.
- 제약이 있는 ratio는 computed serialization과 typed carrier가 모두 정확한 numeric `2/1`이어야 한다. Min/max typed values와 computed longhand, node ID, owner, style revision이 같은 snapshot을 가리켜야 한다. CSS 문자열을 새로 파싱해 typed value를 대신하지 않는다.
- 직접 px `0..=70` 범위 밖 typed percentage/math/intrinsic keyword/negative/non-finite 값, 초과 range, owner 또는 sibling의 min/max, ratio의 `auto <ratio>`, 다중 ratio item, unsupported track/placement/edge/gap, nested/replaced/anonymous content는 layout 시작 전 owner/property 진단으로 원자 거부한다. CSS 문법 오류의 cascade fallback과 valid-but-out-of-profile computed value를 구분한다.
- Unsupported 입력에서 stale frame/partial scene을 공개하지 않고, 다음 valid revision에서 계산이 회복되어야 한다. 값 축약이나 CSS-invalid로의 재해석으로 거부를 숨기지 않는다.

## 구현 및 검증 관문

1. Projection과 layout 경계를 추적해 min/max typed dimension이 어떻게 ratio suggestion에 적용되는지 보인다. 다른 Root HTML owner, 기존 ratio-only scene 및 비-RootHTML profile negative control을 유지한다.
2. Rust matrix는 10개 고정 scene, item/root frame, columns/rows, typed ratio/basis, min/max serialization, NodeId 및 style/layout revision을 확인한다. 축 교환, min/max 순서, min-over-max, zero, 70px 상한, percentage/math/wrong-node/wrong-owner/ratio-owner 중복, stale snapshot 및 overflow rejection controls를 추가한다.
3. 기본 Root HTML runtime에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에 같은 source hash를 사용해 10개 장면의 layout-ready와 draw/present를 확인한다. Android는 software renderer, iOS drawable은 compositor 제출 전임을 기록한다.
4. Android SurfaceView PixelCopy와 iOS Metal drawable에서 Grid 밖 `min-width:70px` item 배경을 포함한 표본을 읽어 보존된 overflow를 확인한다. 각 표본 좌표·revision은 target frame과 같은 scene에 묶고, readback하지 않은 범위는 주장하지 않는다.
5. Focused Rust tests, 변경 crate의 scoped Clippy, `cargo fmt --all -- --check`, iOS/Android build/runtime, fixture/default artifact boundary 및 `git diff --check`를 실행한다. 실패가 변경 전부터 있으면 baseline과 분리하고 broad green으로 보고하지 않는다.
6. 구현/runtime 변경에는 계획 검토와 다른 실패 관점 20개, 구현 PR diff에는 프로젝트 규칙에 따라 별도 30개를 기록한다. 각 발견을 수정한 뒤 관련 기준을 다시 실행한다.
7. [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md), [공식 STATUS](../spec/STATUS.md), 이 계획과 Tailnet roadmap preview를 병합된 실제 범위에 맞춘다. C11.1/C11 checkbox와 공식 `78/214`는 상위 관문 충족 전까지 유지한다.

## 범위 밖

비율이 있는 replaced image, arbitrary font/script, text wrapping/inline runs, item edge와 border-box 전이, percentage/calc/intrinsic min/max, multiple ratio owners, nested/area/named-line Grid, dense/column flow, public CSS admission, WPT, Android 실기기/hardware GPU 및 iOS compositor 이후 pixel equality는 포함하지 않는다.
