# C11.4 · auto-repeat 앞뒤 `calc()` 이웃 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [percentage neighbor 선행 slice](./c11-4-auto-repeat-percentage-neighbors.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태 · 2026-10-24:** 계획 및 Agent Browser 사전 비교 단계다. CSS/runtime 구현은 시작하지 않았다. C11.4 상위 체크는 미완료이며 공식 집계는 `72/213`이다.

## 코드 경계와 목적

PR [#580](https://github.com/ohah/spinon/pull/580)은 definite positive content axis에서 유한·0 이상 `LengthPx`/`Percentage` neighbor를 제한 auto-repeat profile에 연결했다. 현재 [Stylo gate](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)의 `is_auto_repeat_neighbor_track`와 [layout gate](../crates/spinon-layout/src/grid.rs)의 `auto_repeat_neighbor_tracks_are_supported`는 `Calc` neighbor를 거부한다. Layout은 percentage에 양의 definite content basis를 요구하고, basis 곱과 aggregate를 표현 범위에서 검사한다. 기존 auto-repeat track 내부의 일부 `calc()` minimum/maximum 지원과 바깥 neighbor 지원은 별도 경계다.

이 계획은 fragment 앞·뒤에 쓰는 제한 2-term `calc(length + percentage)` 하나 이상을 위 neighbor 입력에 추가한다. 일반적인 CSS math 평가기를 새로 만들지 않고, Stylo가 보존한 `ComputedCssMath::Sum`의 정확히 두 항인 `LengthPx`와 `Percentage` 조합만 쓴다. 합의 항 순서는 어느 쪽이든 허용한다. Length 항은 유한 signed 값이며 Percentage 항은 유한 0 이상 값으로 제한한다. 다만 지정 축 basis에서 최종 계산값은 CSS range 규칙에 맞춰 0 이상으로 clamp한다.

## 규범 기준, oracle와 재현

- [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1, §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat): auto-repeat의 fixed-size 조건, column inline/row block percentage basis, definite content box·non-repeated tracks·gutter를 포함한 반복 횟수, 최소 1회 fallback 및 auto-fit collapse.
- [CSS Values and Units Level 4 §10.12](https://www.w3.org/TR/css-values/#range-checking): calc 최종값을 허용 범위에 clamp한다. Grid `<length-percentage [0,∞]>` neighbor의 음수 최종 결과는 0px로 사용한다. CSS-valid calc 선언 자체를 invalid syntax로 취급하지 않는다.
- 기준 브라우저는 Agent Browser 0.31.1의 Chrome 156.0.8078.48, macOS ARM64, 1280×633, DPR 1이다. [14-scene fixture](../tests/fixtures/css/c11/grid-auto-repeat-calc-neighbors-c114-precomparison.html), [독립 계산과 Chrome 관측](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/README.md), [fixture observation](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/chrome156-observation.json), [validator](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/validate-precomparison.py)가 재현 입력과 관측값을 고정한다.
- 재현 명령은 `python3 spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/validate-precomparison.py`다. 14 scene의 규범 계산과 Chrome CSSOM은 1/64 CSS px 허용 범위에서 통과했다. 이 validator는 제품 runtime 검증이 아니다.

## 고정 장면과 기대값

각 positive scene의 반복 track minimum은 40px, gap은 4px이다. 일반 definite content axis는 240px이고 border-box scene의 content basis는 `136 - 16 padding - 4 border = 116px`이다. 독립 계산은 fixture에서 Chrome 출력과 분리해 생성한다.

| 장면 | normative track 기대값 | 경계 |
|---|---|---|
| `col-fill-both`, `row-fill-both` | count 4, `[22,47,47,47,47,10]` | 양쪽 calc neighbor, 두 axis |
| `col-fill-prefix` | count 4, `[22,50.5,50.5,50.5,50.5]` | prefix만 |
| `row-fill-suffix` | count 5, `[42,42,42,42,42,10]` | suffix만, row basis |
| `col-fit-first` | count 4, `[22,200,0,0,0,10]` | 점유 repeat 1개, 나머지 collapse |
| `col-fit-empty` | count 4, `[22,0,0,0,0,10]` | 모든 auto-fit repeat collapse |
| `col-fill-overflow` | count 1, `[196,40,74]`; 78px track/gap overflow | 최소 1회 규칙; item은 grid border-box 오른쪽 4px 밖에 놓음 |
| `col-fill-border-box` | basis 116, count 2, `[9.8,44.15,44.15,5.9]` | border/padding 제외 후 percentage basis |
| `col-fill-negative-term` | count 5, `[10,42,42,42,42,42]` | signed length term, 최종 10px |
| `col-fill-negative-final` | count 5, `[0,44,44,44,44,44]` | calc 최종값 음수, 0px clamp |
| `col-fill-negative-percentage-term` | count 5, `[8,42.4,42.4,42.4,42.4,42.4]` | CSS-valid negative control; negative percentage term은 AST whitelist 밖 |
| `col-fill-extra-term` | count 4, `[23,50.25,50.25,50.25,50.25]` | CSS-valid negative control; 3-term sum은 whitelist 밖 |
| `col-max-content` | repeat count 1 | indefinite/cyclic CSS-valid negative control |
| `col-fill-fixed-control` | count 4, `[22,47,47,47,47,10]` | PR #580 fixed-px neighbor 회귀 |

14 scene 중 11개는 구현 대상, `col-fill-negative-percentage-term`·`col-fill-extra-term`·`col-max-content`는 CSS-valid negative control이다. Negative control 3개는 값이 유효하더라도 이 profile의 typed AST/basis 경계 밖이라 runtime에서 정확한 fail-closed error와 partial output 부재를 검사한다. Chrome CSSOM observation은 control 선언도 CSS-valid임을 보존한다.

## 계획된 구현 경계

1. Cascade projection에서 top-level repeat neighbor 위치를 보존하고, 기존 px/percentage 값과 새 2-term calc subset을 같은 typed whitelist 경계로 검사한다. Sum 항 수가 2가 아니거나, 지원하지 않는 math variant/차원/식별자가 있으면 구체 CSS property와 authored value를 포함해 거부한다.
2. Layout에서 axis와 node/property가 일치하는 `LayoutCssMathValue`만 계산한다. Percentage는 definite positive content size에 resolve하고, `length + percentage × basis`는 `f64`로 계산한다. CSS 범위에 따라 음수 결과는 0px로 clamp한다. 결과가 finite하지 않거나 `f32`로 표현할 수 없으면 값을 바꾸지 않고 fail-closed하며, prefix·suffix aggregate가 `f32::MAX`를 넘을 때도 부분 결과를 만들지 않는다.
3. 계산된 neighbor는 auto-repeat count 산정에만 사용한다. 이미 처리한 repeat fragment 내부 track의 minimum/maximum resolution, auto-fit empty track collapse, actual track sizing 순서와 alignment는 바꾸지 않는다.
4. `spinon-style`의 typed AST gate, `spinon-layout`의 exact expectation matrix와 overflow/indefinite negative cases, opt-in FFI fixture path의 14 case를 연결한다. 11 positive scene은 geometry/frame/trace를 확인하고 3 negative control은 정확한 error와 partial output 부재를 확인한다. 기본 product FFI artifact에는 fixture symbol/string/payload가 계속 없어야 한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 위 11 positive scene의 frame·used tracks·collapse·layout trace·draw/present와 3 negative control의 fail-closed 결과를 확인한다. `col-fill-overflow`는 suffix track 안의 2×2 item이 root border-box 오른쪽으로 4px 나간 coordinate를 두 플랫폼에서 확인하고, PixelCopy/Metal drawable에서 item 중심(`root.right + 5 CSS px`)의 실제 paint 표본과 root 안·바깥 경계 표본을 각각 대조한다. Screenshot은 root outline, item 위치와 fixture ID가 보이게 남긴다. 표본을 측정하지 못하면 밖으로 그려진다고 주장하지 않는다.

## 오류·실패 조건과 제외 범위

- Indefinite/cyclic percentage basis, non-positive root content size, unresolved axis, missing/mismatched calc ID/node/property, non-finite terms/intermediate/sum, f32 representability overflow는 `LayoutError` 또는 기존 precise cascade error로 거부한다. partial `LayoutOutput`은 만들지 않는다.
- Percentage term이 negative거나 math tree가 nested/product/min/max/clamp/3-term 이상이면 현재 scope에서 거부한다. CSS 자체가 유효하더라도 이 opt-in Spinon profile은 unsupported 입력을 조용히 0 또는 fixed 값으로 바꾸지 않는다. 최종 계산값이 음수인 지원 shape만 CSS 규범에 따라 0으로 clamp한다.
- Intrinsic/flexible neighbor, arbitrary author CSS admission, 복수 auto-repeat, 일반 Taffy percentage resolver 변경, WPT 전체, 실기기/hardware GPU, iOS compositor 이후 픽셀, 전체 C11.4/CSS Grid 적합성은 범위 밖이다.
- C11.4 checkbox와 공식 `72/213`은 유지한다. Runtime 통과만으로 C11.4 상위 완료를 주장하지 않는다.

## 구현 전 review gate

계획 자체를 20개의 독립 실패 관점으로 검토하고 계획 문서에 별도 기록을 남긴 뒤에만 제품 코드 구현을 시작한다. 계획 구현/PR에는 별도의 30개 PR diff 적대 검토가 필요하다. 구현 후에는 이 검토를 재사용하지 않고 새 20개 implementation/runtime 관점과 새 30개 PR diff 관점을 작성한다.
