# C11.4 · repeated-text row-span fit-content positions 4–6 · `min-height:auto`

**상태:** 계획 및 사전 비교 완료 · 제품 코드는 변경하지 않음 · C11.4/C11 미완료 · 공식 `78/214` 유지

**기준 구현:** [six-row single-fit row-span positions 4–6](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md) · 구현 PR [#1306](https://github.com/ohah/spinon/pull/1306)

**구현 전 비교:** [Chrome 157 paired matrix](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-positions-4-6-auto-minimum-precomparison.html) · [독립 검증 결과](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10/matrix-verification.json)

**계획 실패 관점 검토:** [서로 다른 20개](./c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-plan-review-20.md) · **계획 PR diff 검토:** [별도 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum-precomparison-2026-11-10/plan-pr-diff-review-20.md)

## 결손과 제한 목표

PR #1306의 기본 parsed Root HTML repeated-text row-span profile은 실제 여섯 positive implicit row를 만들고, 여섯-member `grid-auto-rows` list의 authored `fit-content()` 위치 4–6을 허용한다. 해당 bounded profile은 span text item에 `min-height:0`을 사용한다. 별도 PR #1326은 동일 family의 세-row topology와 위치 1–3에서만 text item의 `min-height:auto`를 허용했다.

이 계획은 기존 six-row topology에 `min-height:auto`를 연결하는 별도 제한 slice를 정한다. 기존 한 개 fit-content track, 나머지 `min-content ×2`, `max-content ×2`, `auto ×1` multiset, direct-text grammar, explicit placement, column 1, visible overflow, `align-content:start|stretch`는 유지한다. 모든 span text item은 같은 `min-height:auto`여야 하고 첫 explicit-row empty control은 계속 `min-height:0`이다. 이 변경은 C11.4 전체, 임의 row-span, 일반 CSS admission 또는 public API 지원을 뜻하지 않는다.

## 독립 기준과 고정 비교 입력

규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6에서 grid item의 해당 축 computed overflow가 scrollable하지 않고, span에 `auto` minimum track이 있으며, 여러 track을 span할 때 flexible track이 없으면 content-based automatic minimum을 쓴다. `fit-content()`의 minimum sizing function은 `auto`이고, fit-content argument는 이 content-based minimum을 같은 방식으로 clamp하지 않는다. 따라서 계획은 content floor를 fit cap보다 낮출 수 있다고 가정하지 않는다.

고정 CSS 입력은 definite `40×220px` Grid, `40px` explicit column 하나, `10px` explicit row 하나, gap 0, LTR, sparse row flow, visible overflow, monospace 10px/line-height 10px의 제한 ASCII direct text다. 뒤에 정확히 여섯 positive implicit row를 만들며 두 topology family와 정방향·역방향 DOM source order를 비교한다.

| 요소 | 고정 입력 |
| --- | --- |
| Track list | `fit-content()` 하나 + `min-content ×2` + `max-content ×2` + `auto ×1`, 여섯 member |
| Fit 위치 | authored position 4, 5, 6 |
| 나머지 tail | 5-member multiset의 30개 고유 순서 |
| Cap | `0px`, `0%`, `10px`, `20px`, `20%`, `120%`; percentage basis는 definite 220px |
| Alignment | `start`, `stretch` |
| Topology | five span-2 items 또는 span-2/span-3 혼합 네 item; 각각 forward/reverse source order |
| Minimum 비교 | 기존 baseline `min-height:0`과 candidate `min-height:auto`; explicit empty control은 양쪽 모두 `0` |

행렬은 `3 positions × 30 tail orders × 6 caps × 2 alignments × 4 topologies × 2 minimum modes = 8,640 scenes`다. Chrome 157 Agent Browser에서 같은 authored key의 baseline/candidate 4,320쌍을 고정하고, 각 scene의 computed track serialization, used row sizes, item/payload frame, text line rectangles, source order 및 content overflow를 보존한다. 허용 기하 오차는 `0.01 CSS px`다.

## 사전 관측

Chrome 157 사전 비교는 baseline 4,320개와 candidate 4,320개를 모두 실행했다. 독립 검증기에서 각 fit position, cap, tail 순서, alignment, topology, source order, computed minimum 값과 expected item frame을 다시 검사했다.

- 12,240개 automatic-minimum eligible item occurrence에서 baseline 및 candidate content-floor 위반은 각각 0, candidate visible content overflow도 0이었다.
- `min-height:auto` 적용으로 item frame 15,348개가 바뀌었다. 최대 used-track 차이는 70px, 최대 item-frame 차이는 55px였다. text line count는 paired mode 사이에서 바뀌지 않았다.
- source order를 뒤집은 semantic frame 최대 차이는 0px다. `stretch`에서 non-auto track의 최대 변화도 0px이며, auto track은 각 pair에서 줄지 않고 최대 90px 커졌다.
- Chrome 결과는 CSS 기준 입력의 cross-check일 뿐 Spinon 구현 결과가 아니다. 이 계획은 모바일 실행, Android/iOS 표면 픽셀, compositor, 실기기 또는 전체 WPT 결과를 주장하지 않는다.

## 구현 범위와 거부 경계

- 기본 parsed Root HTML `RuntimeSession` 경로에서 기존 six-row single-fit profile의 fit 위치 4–6만 확장한다. 공개 API·FFI를 추가하지 않고 내부 numeric contract version `0.1.0`을 유지한다.
- computed `min-height:auto`는 네/다섯 span text item 모두에 동일하게 적용한다. item별 mixed `auto`/`0`, explicit control의 변경, 새 content/font/Unicode, implicit columns, 추가 Grid owner, 다른 topology와 unsupported cap/basis/style은 거부한다.
- min-content floor, fit-content authored position, row owner/revision, track trace와 occupied-row frame sum을 검사한다. malformed scene은 결과를 publish하지 않고 같은 `RuntimeSession`에서 이후 valid revision으로 복구되는지 확인한다.
- 일반 CSS admission, row/column scrolling·clipping, WPT 적합성, 공개 CSS 지원과 C11.4/C11 완료 상태는 넓히지 않는다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 기본 Root HTML baseline/candidate sentinel의 light/dark layout-ready 및 draw/present를 확인한다. 이는 full native matrix 또는 surface pixel parity가 아니다. 사용한 임시 simulator/AVD는 작업 후 제거하고 shared profiles/data는 보존한다.

## 구현 판정 관문

구현 뒤에는 CSS/Chrome observation과 분리된 layout, style projection, RuntimeSession track trace, rejection/atomic recovery 검사를 추가한다. 기존 positions 4–6 `min-height:0` 4,320-scene matrix 및 one-fit positions 1–3 auto-minimum, two-fit row-span/auto-placement 회귀를 재실행한다. Android/iOS는 변경 소스 해시가 기록된 기본 Root HTML route를 실행한다. `cargo fmt`, 변경 crate의 focused Clippy, 기본/opt-in fixture artifact 경계도 변경에 맞게 확인한다.

구현/runtime 동작은 새 실패 관점 20개로 검토하고 최종 PR diff도 별도의 20개 관점으로 검토한다. 발견한 결함은 수정 후 영향을 받는 검증을 다시 수행한다. `spec/STATUS.md`의 C11.4/C11 checkbox와 공식 `78/214`는 이 제한 slice가 C11.4 전체 완료를 충족하지 않는 한 유지한다.
