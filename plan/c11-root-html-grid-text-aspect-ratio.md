# C11 · 기본 parsed Root HTML Grid direct-text aspect-ratio sizing

**계획 실패 관점 검토:** [독립 20개](./c11-root-html-grid-text-aspect-ratio-plan-review-20.md) · **구현 전 oracle:** [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-precomparison-2026-11-04/README.md)

## 목적

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1` 경로에서 숫자형 preferred `aspect-ratio`가 direct-text Grid item의 자동 크기 계산에 전달되는지 검증하고, 고정된 좁은 subset만 구현한다. 기존 C11.4 aspect-ratio slice는 별도 opt-in fixture profile이다. 그 구현이나 결과를 이 기본 앱 경로의 지원 근거로 재사용하지 않는다.

Grid owner마다 ratio를 가진 in-flow direct HTML block text item을 최대 하나 허용한다. item은 같은 auto row에 놓이는 sibling direct block text item 하나와 함께 온다. Root는 한 explicit column track list `[10px, 60px]`, 한 `auto` row, `grid-auto-flow:row`, zero gap, finite definite `width:70px`, 기본 writing mode/direction, start alignment, visible overflow를 사용한다. Item은 Ahem 10px의 `A`와 `AAAA`이며 대상 item은 `aspect-ratio:2/1`이다. 폭 지정 장면과 높이 지정 장면을 각각 ratio 없는 대조군과 비교한다.

## 기준과 독립 기대값

- 기준 구현은 Agent Browser의 HeadlessChrome `156.0.0.0`이다. 고정 입력은 [precomparison fixture](../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-precomparison.html)이며, 화면·DOM/CSSOM 관찰·독립 기대값·해시는 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-precomparison-2026-11-04/README.md)에 둔다.
- [CSS Grid Layout 2025 CRD §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)는 `start`로 정렬되는 Grid item을 해당 축의 fit-content sizing으로 처리한다. [CSS Sizing Level 4 2026 WD §4.2](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#automatic-sizes)는 preferred ratio가 있는 box의 automatic size 계산에서 definite ratio-determining size를 비율로 전이한다고 기술한다. 후자는 Working Draft이며, 이 계획은 이를 안정된 상호운용성 판정으로 확대하지 않고 Chrome 관측과 함께 고정 입력의 비교 자료로 쓴다.
- Grid는 width `70px`, columns `[10px,60px]`, gap `0`, single auto row다. Ahem glyph advance와 line-height는 각각 `10px`이다. Ratio 장면에서 `AAAA`의 definite axis 60px 또는 30px를 2:1로 전이하면 frame은 `60×30px`이다. Width-ratio scene의 auto row는 `max(10px,30px)=30px`; no-ratio control은 `max(10px,10px)=10px`이다. Height-ratio scene의 used width는 `60px`; no-ratio control은 direct text의 40px content width다.

| 장면 | `AAAA` 입력 | 기대 root track / frame |
|---|---|---|
| `width-ratio` | `width:60px; height:auto; aspect-ratio:2/1` | columns `[10,60]`, row `[30]`; target `60×30` |
| `width-control` | `width:60px; height:auto`; authored ratio 없음(computed `auto`) | columns `[10,60]`, row `[10]`; target `60×10` |
| `height-ratio` | `width:auto; height:30px; aspect-ratio:2/1` | columns `[10,60]`, row `[30]`; target `60×30` |
| `height-control` | `width:auto; height:30px`; authored ratio 없음(computed `auto`) | columns `[10,60]`, row `[30]`; target `40×30` |

허용 오차는 root/item frame과 track 값마다 `0 CSS px`다. 이는 layout geometry oracle이며 glyph raster, Android/iOS surface pixel 또는 compositor 결과가 아니다.

## 입력 계약과 fail-closed 경계

- 기본 parsed Root HTML profile만 변경한다. C11.4의 `RuntimeBlockFlexGridTrackSizingV1` fixture profile, generic style projection 및 다른 Root HTML Grid mode는 바꾸지 않는다.
- `aspect-ratio` computed Longhand는 default route에서 cascading snapshot에 기록하고, 동일 NodeId·Grid owner·style revision의 typed ratio와 CSS computed serialization이 서로 일치해야 한다. CSSOM declaration string을 projection에서 다시 파싱하지 않는다.
- ratio는 Grid container가 아니라 그 Grid owner의 direct in-flow non-replaced HTML block text item에만 적용한다. 한 owner 안에서 ratio item이 둘 이상이면 전체 owner를 contextual error로 거부한다. 다른 owner의 ratio나 control 값이 섞여선 안 된다.
- owner, item NodeId, source order, computed-style revision, layout revision이 snapshot과 다르거나 누락·중복되면 partial scene을 내지 않고 거부한다. ratio projection 실패, text measure 실패 및 layout 실패도 같은 원자적 output 경계를 지킨다.
- CSS ratio는 정확히 finite positive `2/1`만 성공 입력으로 둔다. `auto <ratio>`, 다른 ratio 값, degenerate `0/1`, ratio shorthand/중복 cascade가 다른 computed 값으로 이어지는 경우는 이 계획의 성공 subset이 아니다. syntax-invalid 선언의 cascade fallback과 valid-but-unsupported computed value를 구분한다.
- Grid root와 item은 zero margin/padding/border를 사용한다. Root width는 definite `70px`이고 height는 auto다. Grid별 배경색만 screenshot에서 scene을 구분하며 sizing에 영향을 주지 않는다. Grid item은 `min-width/height:auto`, `max-width/height:none`, horizontal-tb/LTR, static position, `order:0`, visible overflow를 유지한다. 대상 item 외 크기, ratio, Grid placement는 바꾸지 않는다.
- nested/replaced/anonymous Grid item, wrapping text, inline child, hidden/detached/stale item, Grid-root ratio, block flow ratio, multiple ratio items per owner, percentage/math/keyword size, min/max clamp, box edges, gap, implicit track, non-row flow, named area/line, explicit placement, dense flow, non-default writing mode/alignment는 거부하거나 기존 경로로 남긴다. 본 구현이 이 경계를 넓히지 않는다.

## 구현 순서와 수용 기준

1. 이 계획을 독립된 실패 관점 20개로 검토하고 수정한다. 이 검토가 끝나기 전에는 제품 코드를 변경하지 않는다.
2. `RuntimeRootHtmlTextPaintV1` cascade allowlist/property vector와 projection의 owner-scoped gate를 조사한다. ratio typed carrier는 기존 모델을 재사용하고, default route가 아닌 profile에서 allowlist가 넓어지지 않는 회귀 검사를 추가한다.
3. Rust cascade/projection/layout matrix는 위 네 scene의 track·root/item frame·computed string·typed ratio·NodeId·revision을 확인한다. fail-closed controls는 root/non-Grid/nested/multiple-owner/stale-revision/unsupported ratio and sizing inputs를 포함한다.
4. 앱 runtime fixture는 같은 parsed HTML source hash를 사용하고 default app route에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 실행한다. `layout-ready`, draw/present 경계와 item/root geometry를 기록한다. PixelCopy/Metal readback은 실제 읽었을 때만 별도 판정하고 Android software renderer·iOS compositor 전 readback 한계를 표시한다.
5. 변경 crate의 focused tests, fixture archive boundary, scoped Clippy/rustfmt, Android/iOS simulator build/runtime을 각각 기록한다. 실행하지 않은 gate를 통과로 쓰지 않는다. 로컬 후보를 PR에 올리면 실제 변경 diff를 별도 관점 20개로 검토한다.
6. 내부 계약 0059와 `spec/STATUS.md`, C11 계획 및 공개 Tailnet roadmap에 제한 결과만 연결한다. 일반 CSS 지원, 전체 C11.4/C11, WPT 적합성은 표시하지 않으며 공식 집계 `72/213`을 이 제한 slice만으로 올리지 않는다.

## 제외 범위

일반 `aspect-ratio` CSS 지원, 임의 ratio matrix, `auto <ratio>`, min/max transfer, replaced/image elements, anonymous text nodes, wrapping/shaping, intrinsic track grammar 전반, arbitrary HTML, WPT suite, real-device/hardware GPU, iOS post-compositor pixels, C11.4 또는 C11 완료 판정.
