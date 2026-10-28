# C11.4 definite calc `fit-content()` implicit row WPT slice

**상태:** 구현 전 계획. Pinned WPT `fit-content-track-sizing.html`의 definite block-size `calc()` argument assertion 하나를 고정한다. Agent Browser HeadlessChrome 156 upstream harness 13/13과 target row `35px`를 관측했다. CSS Grid 2025 CRD에서 독립 계산한 기대값과 source hash는 [사전 비교](../spec/internal/evidence/c11-4-fit-content-calc-argument-precomparison-2026-10-29/README.md)에 있다. 계획 실패 관점 20개와 [계획 PR diff 30관점 검토](../spec/internal/evidence/c11-4-fit-content-calc-argument-precomparison-2026-10-29/plan-pr-diff-review-30.md)를 분리 기록했다. 구현은 계획 PR 병합 뒤 별도 PR에서 진행한다. C11.4와 공식 `72/213`은 그대로 미완료다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 기존 작업과의 차이

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 `calculatedArgument` assertion을 기존 opt-in C11.4 app fixture 경로에서 실행한다. definite 300px block-size basis에 대해 `fit-content(calc(10% + 5px))`의 argument가 35px로 resolve되고, 400px child의 max-content contribution보다 작은 row를 35px에서 제한하는지 검증한다.

기존 calc slice는 direct-leaf text의 definite column, 또는 intrinsic container의 cyclic percentage track을 다룬다. 기존 percentage WPT slice는 definite row의 `fit-content(50%)`를 다룬다. 여기서는 definite row에서 percentage와 absolute length를 함께 가진 calc argument 및 nested 400px child를 연결한다. 이 계획은 CSS calc 일반 지원, cyclic resolution, WPT 파일 전체 실행 또는 임의 DOM 지원을 추가하지 않는다.

## 고정 출처와 upstream 관측

- Pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, source SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- Target markup은 `calculatedArgument`; assertion은 35px row를 요구한다. Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, viewport 1280×633 CSS px, DPR 1에서 upstream harness `OK`, 13 tests / 13 Pass였고 computed rows는 35px이었다. Target item height는 35px, min-height는 0px, child height는 400px이다.
- 원본 root width는 viewport에서 1264px로 계산된다. Runtime fixture는 root를 60×300px로 normalize하고 fixed 60px column을 둔다. Row percentage basis와 child height를 보존하면서 viewport의 inline-size 의존성을 제거한다.
- 관측과 독립 기대값은 [기계 판독 기준](../spec/internal/evidence/c11-4-fit-content-calc-argument-precomparison-2026-10-29/expected.json)에 있다. 보존 화면은 [Agent Browser WPT 캡처](../spec/internal/evidence/c11-4-fit-content-percentage-argument-precomparison-2026-10-29/wpt-fit-content-percentage-argument.png)다. Chrome 결과는 upstream 기준이며 Spinon 결과가 아니다.

## 독립 기대값

CSS Grid 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 row percentage를 grid container inner block-size 기준으로 resolve한다. Height 300px이 definite이므로 calc의 percentage 항은 30px이고, 5px length를 합친 argument는 35px이다. 이 장면은 track이 container intrinsic size에 의존하지 않으므로 cyclic percentage fallback은 적용되지 않는다.

CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 fit-content formula는 `max(minimum, min(limit, max-content))`다. WPT item의 `min-height:0` 때문에 minimum은 0px이고, nested 400px child의 block contribution은 400px이다. 따라서 expected used row는 `max(0, min(35, 400)) = 35px`이다. Runtime stage trace는 base size, raw growth limit 및 final used size를 구분한다. CSSOM의 35px 결과만으로 calc typed value 또는 trace correctness를 대신하지 않는다.

## 제한 runtime 입력과 구현 경계

한 scene ID `fit-content-calc-row-wpt`를 추가한다. Root는 HTML div, 60×300px, `grid-template-columns:60px`, `grid-template-rows:none`, row auto-flow, 0 gap 및 `grid-auto-rows:fit-content(calc(10% + 5px))`다. Direct grid item 한 개는 `min-height:0`이며 그 안의 direct child div 하나는 30×400px다. WPT의 computed initial overflow `visible`을 보존하고 author `overflow:visible` declaration은 추가하지 않는다.

기존 definite percentage row adapter 및 report 경계를 우선 검토한다. 새 scene이 기존 scene의 typed track validation과 trace 의미를 손상시키면 같은 broad profile로 합치지 말고 fixture-only calc route를 분리한다. Product CSS admission이나 public API는 넓히지 않는다. Exact route는 root/item/child identity, namespace/tag, parent, node 수와 순서, computed style, typed `FitContent(calc())`, calc registry/revision, 하나의 implicit row, 60px column 및 환경/layout revision을 모두 확인한다. 등록되지 않은 scene/tree/style/track은 결과 일부를 발행하지 않고 거부한다.

## 수용 기준

1. Rust layout/runtime에서 root `[0,0,60,300]`, Columns `[60]px`, Rows `[35]px`, item `[0,0,60,35]`, child `[0,0,30,400]`을 검사한다.
2. Computed track은 exact `fit-content(calc(10% + 5px))` typed structure로 판별하고 percentage 10%, length 5px, definite block basis 300px의 identity/revision을 서로 연결한다. 10% only, `calc(10% + 4px)`, 다른 basis, stale calc handle은 negative control로 거부한다.
3. Rows trace는 기존 six-stage 순서를 보존한다. Initialize부터 IntrinsicSizingComplete까지 base는 0px, MaximizeTracks 이후 final base는 35px이다. Item contributions는 Minimum 0px (`SpecifiedMinimum`), MinContent 400px, MaxContent 400px이며 raw growth limit을 final used row로 오독하지 않는다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 scene의 typed track, frame, revision, trace, draw/present를 확인한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable의 네 고정 surface sample을 4/4 exact 비교한다.
5. Surface viewport는 320×400 CSS px이며 root의 percentage basis와 독립이다. Row 안 child `(15.5,34.5)`, row 안 item `(45.5,34.5)`, root 안 row 아래 배경 `(45.5,200.5)`, root 밖 child `(15.5,350.5)` 색을 각각 검사한다.
6. Default/product FFI archive에는 새 scene ID·marker·fixture symbols가 없어야 하며 opt-in fixture-test artifact에는 positive control이 있어야 한다. 정적 archive 경계는 최종 APK/IPA 전체의 제외 증거로 확대하지 않는다.
7. 변경된 Rust/CSS reference tests, Android/iOS simulator builds and runs, `cargo fmt`, 영향받은 Clippy와 fixture-module boundary checks를 기록한다. 실제 실행하지 않은 GitHub checks는 성공으로 표시하지 않는다.
8. 구현 뒤 새 runtime 실패 관점 20개와 PR diff의 별도 30개 관점을 기록한다. PR 병합 뒤 C11.4 상위 checkbox와 `72/213`은 그대로 둔다.

## 실패 기준 및 제외 범위

35px가 아닌 row, 300px이 아닌 percentage basis, calc term 손실, item automatic minimum이 0이 아닌 경우, expected typed track과 layout trace의 revision 불일치, unlisted input의 부분 결과, surface sample mismatch, 기본 artifact fixture 노출은 실패다. WPT source drift는 pinned revision/hash를 먼저 확인하기 전까지 baseline으로 사용하지 않는다.

일반 CSS admission, arbitrary nested Grid, 다른 calc grammar, cyclic percentage sizing, explicit rows, 여러 implicit row, 다른 font/writing-mode, 전체 WPT suite, Android 실기기/hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4 적합성은 제외한다.
