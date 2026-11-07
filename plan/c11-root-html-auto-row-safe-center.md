# C11.4 parsed Root HTML `align-content:safe|unsafe center`

**상태:** 제한 구현 PR [#1249](https://github.com/ohah/spinon/pull/1249)이 `2026-11-07T02:49:40Z` Rebase and merge됐고 merge SHA는 `a69030722d82a97e6ce40e276c0aae2b8447550d`다. Agent Browser HeadlessChrome 157과 독립 계산의 6/6 scene 비교, Layout 2/2, style projection allowlist 1/1, RuntimeSession 2/2, 관련 C11.4 Layout 14/14·projection 4/4·calc RuntimeSession 2/2, changed-crate Clippy와 rustfmt가 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 positive·safe overflow·unsafe overflow 각 장면을 기본 앱 경로로 실행했고 화면 색 경계는 기대 좌표와 1 CSS px 이내였다. 앱·bootstrap·build restoration도 해시로 확인했다. [Simulator·화면·hash·복원 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-center-runtime-2026-11-07/README.md). 이 제한 slice는 C11.4/C11 완료나 공식 `78/214` 집계를 바꾸지 않는다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 content alignment](./c11-root-html-auto-row-content-alignment.md) · [bounded `calc()` row-gap](./c11-root-html-auto-row-calc-gap.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적

기존 `RootHtmlAutoRowStretchV1` profile은 plain content-position keyword를 받지만 `safe|unsafe` modifier를 거부한다. 이 작업은 fixed 40×40px Root HTML Grid shape에서 `align-content:safe center`와 `unsafe center`를 제한 허용해 overflow 시 동작 차이를 구현하고 검증한다. 최신 `calc()` row-gap 경로와 결합해 안전 정렬이 필요한 실제 overflow 조건을 만든다.

규범 기대값은 [CSS Box Alignment Level 3 §4.4](https://www.w3.org/TR/css-align-3/#overflow-values) 및 [§5.1.4](https://www.w3.org/TR/css-align-3/#align-content-property)에서 가져온다. §4.4는 overflow가 생길 때 `safe`를 flex-start처럼 시작점에 놓고 `unsafe`는 지정된 alignment를 유지하도록 정의한다. Grid에서 flex-start는 start와 같다. §5.1.4는 Grid content-alignment subject에 tracks와 gutters를 포함한다. W3C 문서는 2026-10-08 Working Draft이고 `<overflow-position>`을 at-risk로 표시하므로 이 제한 구현을 전체 CSS 지원이나 안정된 공개 계약으로 확대하지 않는다.

Chrome 157 관측과 독립 계산의 원본은 [precomparison evidence](../spec/internal/evidence/c11-root-html-auto-row-safe-center-precomparison-2026-11-07/README.md)에 있다. fixture SHA, viewport, DPR, computed serialization, raw item bounds와 screenshot을 함께 보존했다.

## 고정 입력과 기대 동작

기존 fixed profile을 유지한다: definite 40×40px content box, 40px column, explicit 10px row, 두 implicit `auto` rows, column-gap 0, row auto-flow, `align-items:normal`, 하나의 40×10px empty direct item과 10px line-height direct-text `A`/`B`. Item 수·문서 revision·root ownership·track sizing gate는 바꾸지 않는다.

| Row gap | Alignment | Expected rows | Item y offsets (px) |
|---|---|---|---|
| `0px` | `center`, `safe center`, `unsafe center` | `[10,10,10]` | `[5,15,25]` |
| `calc(120% - 2px)` | `safe center` | `[10,10,10]` | `[0,56,112]` |
| `calc(120% - 2px)` | `unsafe center`, `center` | `[10,10,10]` | `[-41,15,71]` |

백분율 기준은 definite 40px block size이므로 overflow 장면의 각 row-gap은 46px이다. 정렬 subject는 `10 + 46 + 10 + 46 + 10 = 122px`, free space는 `-82px`이다. Safe center는 start로 fallback되어 offset 0을 쓰고 unsafe/plain center는 중앙 정렬을 유지해 offset `-41px`을 쓴다. Positive-space 대조는 safe/unsafe가 비-overflow geometry를 바꾸지 않음을 확인한다. Chrome 관측은 독립 좌표와 정확히 일치했으며, 향후 runtime 좌표 허용 오차는 최대 `1/64 CSS px`로 둔다.

## 구현 경계와 수용 기준

- 기존 default parsed Root HTML runtime route와 `RootHtmlAutoRowStretchV1`만 확장한다. 새 profile, public CSS admission, API, FFI, source input shape, text behavior 또는 contract version은 추가하지 않는다.
- 정확한 computed value `safe center`와 `unsafe center`만 기존 profile allowlist에 추가한다. 단일 token keyword, 다른 position, distributed safe modifier, baseline, CSS math와 알 수 없는 값은 계속 거부한다.
- typed `ContentAlignmentPosition::Center`와 `AlignmentSafety::{Safe, Unsafe}`가 projection에서 layout adapter/Taffy로 손실 없이 전달되는지 증명한다. Taffy가 고정 overflow에서 oracle 결과를 내는지 별도 focused layout test로 확인하고, 그렇지 않으면 implementation 전에 계획을 보완한다.
- Track sizes와 row-gap 계산을 변경하지 않는다. `safe` fallback은 정렬 offset에만 적용하고 rows/gutters에 stretch나 clamp를 추가하지 않는다. Plain `center`, 기존 10 alignment values, unsupported rejection의 회귀를 확인한다.
- Style-to-layout, layout-core, RuntimeSession의 focused positive/overflow/rejection/recovery gates를 둔다. Chrome 157와 동일한 6-scene matrix를 독립 계산에 비교한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 app route에서 positive/overflow 장면 및 복원을 확인한다. OS screenshot bounds는 visual evidence로만 쓰고 PixelCopy/Metal readback 결과처럼 쓰지 않는다.
- PR에는 명세, 고정 oracle/expected, runtime evidence, 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개를 기록한다. 공식 C11.4/C11 체크와 `78/214`는 바꾸지 않는다.

## 제외 범위

Safe/unsafe distributed values, 다른 content-position, baseline, scroll-container safety/default behavior, 다른 writing mode, 가변 크기·tracks·item topology, CSS parser 일반 확장, WPT conformance, real-device/hardware GPU, iOS compositor 이후 pixel equality, 전체 C11.4/C11 완료는 제외한다.
