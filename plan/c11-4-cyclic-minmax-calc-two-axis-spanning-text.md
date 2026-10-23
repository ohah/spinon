# C11.4 · cyclic `minmax(calc())` 양축 span과 text wrapping 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [겹치는 span 선행 계획](./c11-4-cyclic-minmax-calc-overlapping-rows.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 사전 비교와 독립 기대값을 네 고정 장면으로 확정했다. 계획 검토 뒤 구현을 추가했고, commit `514a2ee73b807fdd2b19d57016adb976dae34d17` 기준 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 장면씩 통과했다. 기본/fixture static archive 경계와 새 구현 runtime 실패 관점 20개도 기록했다. 구현 변경은 [PR #506](https://github.com/ohah/spinon/pull/506), [실행 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-runtime-2026-10-22/README.md)에 연결한다. C11.4 체크박스와 `72/213` 집계는 유지한다.

## 목적과 경계

최근 C11.4 제한 fixture는 cyclic `minmax(calc())`의 column span 기여와 row span 기여를 별도 scene으로 검사했다. 이 계획은 하나의 direct text item이 두 columns와 두 rows를 동시에 span할 때, 먼저 sizing된 Columns가 item의 line wrapping과 Rows의 block contribution을 바꾸는 교차 축 의존성을 검증한다. CSS Grid 2025 CRD §11.1은 columns를 sizing한 뒤 row contribution이 필요한 item의 inline available space를 이 column sizes로 정한다.

범위는 explicit 2×2 track, 한 HTML `div` text item, 네 scene ID, Ahem 10px, 두 축 `minmax(calc(10% + 2px), max-content)`, 2px gap이다. Width는 `min-content`, `max-content`, definite 10px, definite 60px로 제한하며 height는 모두 `max-content`다. Item의 `min-width:auto`와 `min-height:auto`를 고정한다. 일반 CSS admission, 임의 item/track/span, implicit tracks, 다른 writing mode/font, 다른 calc 식, nested/anonymous/replaced item, WPT 또는 전체 C11.4 완료를 추가하지 않는다.

## 독립 기대값과 사전 관찰

CSS Grid Layout Module Level 1 2025-03-26 CRD의 §7.2.1 cyclic percentage 규칙, §11.1 sizing 순서와 §11.5.1 spanning contribution으로 기대값을 독립 계산했다. [사전 비교 README](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-precomparison-2026-10-22/README.md), [규범 기대값 JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-precomparison-2026-10-22/spec-derived-expectations.json), [Chrome 156 관측](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-precomparison-2026-10-22/chrome156-observation.json)를 분리해 보관한다.

| Scene | Columns | item inline width | text lines | Rows | content box |
| --- | --- | ---: | ---: | --- | --- |
| min-content width | `[4,4]px` | 10px | 6 | `[29,29]px` | `10×60px` |
| max-content width | `[29,29]px` | 60px | 1 | `[4,4]px` | `60×10px` |
| definite 10px width | `[4,4]px` | 10px | 6 | `[29,29]px` | `10×60px` |
| definite 60px width | `[29,29]px` | 60px | 1 | `[4,4]px` | `60×10px` |

Each scene's item frame is root-relative `(2,2)` with the content-box dimensions above; the 2px root border on every side makes border-box root size `14×64px` or `64×14px`. At intrinsic sizing, cyclic percentages are treated as auto and resolve afterward: each minimum is 3px or 8px and remains below the final used track size. Definite controls reproduce the corresponding intrinsic scene. Chrome line Range geometry is an oracle observation, not proof of Spinon glyph paint.

## 구현·판정 관문

1. 기존 opt-in C11.4 fixture-test path에 네 ID만 추가한다. Adapter는 exact root/item DOM shape, Ahem text, min-width/min-height, grid placement, typed rows/columns, gap, dimensions와 style profile을 검증하고 다른 input은 fail closed한다.
2. Rust layout에서 Columns `[4,4]` 또는 `[29,29]`, Rows `[29,29]` 또는 `[4,4]`, root/item frame, axis별 span contribution과 min/max-content stage trace를 확인한다. Text block contribution은 column sizing 후 계산된 inline size에 연결되어야 한다. Width 10px scene의 6줄/60px contribution과 width 60px scene의 1줄/10px contribution을 서로 바꾸거나 하나의 intrinsic size로 재사용하면 실패다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene 모두 실행한다. Track/frame/trace, source revision, draw/present 및 build/app hash를 기록한다. 실기기는 연결하지 않는다.
4. Android PixelCopy와 iOS drawable readback은 각 scene에서 item 내부의 `#e5484d` sample을 검사하고 scale·native viewport origin을 기록한다. iOS readback은 compositor 제출 전 결과다. Box screenshot은 text glyph raster equality나 CSS border stroke 증거가 아니다.
5. 기본 product FFI artifact에서 신규 fixture marker가 없는 것과 opt-in fixture-test artifact의 positive control을 검사한다. 변경한 Rust crate focused tests, formatter/lint, JS syntax와 Android/iOS Simulator app builds만 실행하고 무관한 전체 workspace 재검증은 근거 없이 확대하지 않는다.
6. 계획 검토와 겹치지 않는 구현/runtime 실패 관점 20개와 PR diff의 별도 20개 검토를 기록한다. 결함을 고치면 영향받은 Rust case, 두 Simulator scene, pixel sample을 재검증한다.
7. PR 본문에는 상태 ID C11.4, 입력·기대값, 실제 simulator/화면 근거와 미검증 범위를 연결한다. 일반 CSS 지원, WPT, 실기기, Android hardware GPU, iOS compositor 이후 pixels 또는 C11.4 전체 완료를 주장하지 않는다. C11.4 체크박스와 `72/213`은 유지한다.

## 범위 밖

이 slice는 한 text item, 2×2 explicit tracks와 단일 `calc()` expression의 고정 입력만 검증한다. CSS Grid의 모든 min/max-content stage, source-order span group, percentage 값의 일반 조합, intrinsic aspect ratio, re-layout 반복 조건, auto-repeat, implicit grid, WPT, 제품 CSS admission, 실기기와 전체 C11.4는 검증하지 않는다.

## 리뷰

- [계획 자체의 서로 다른 실패 관점 20개](./c11-4-cyclic-minmax-calc-two-axis-spanning-text-plan-review-20.md)
- [Chrome 사전 관찰과 규범 계산](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-precomparison-2026-10-22/README.md)
- [계획 PR 변경분 실패 경로 검토](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-precomparison-2026-10-22/plan-pr-diff-review-20.md)
- [구현/runtime 결과, artifact 경계와 구현 실패 관점 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-runtime-2026-10-22/README.md)
- [구현 PR #506 변경분 적대 검토 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-two-axis-spanning-text-runtime-2026-10-22/pr-diff-review-20.md)
