# C11.4 anonymous Grid text `white-space:nowrap` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 제한 구현과 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 마쳤다. 구현/runtime 실패 관점 20개 검토까지 끝냈고, PR diff 30개 검토와 PR은 다음 단계다. 이번 계획은 opt-in C11.4 anonymous Grid text fixture 경로에서 inherited `white-space:nowrap` 하나를 다룬다. C11.4/C11 checkbox와 공식 완료 `72/213`은 바꾸지 않는다. [Chrome 관측·입력 hash·화면](../spec/internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/README.md) · [구현/runtime 근거·한계](../spec/internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/README.md)

## 목적과 경계

CSS Grid Level 1 §6에 따라 Grid container의 in-flow direct text sequence는 anonymous block Grid item이 된다. 현재 제한 fixture는 `ParagraphStyle.white_space == Normal`만 허용한다. text intrinsic engine은 이미 `NoWrap`을 처리하지만 anonymous Grid text 경로는 유효한 `white-space:nowrap` 입력을 거부한다. 이번 slice는 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture 경로에서 이 한 값을 anonymous item에 보존하고 intrinsic contribution 및 track 결과를 고정한다.

이 작업은 일반 CSS style-to-text mapping, arbitrary font/script, inline spans, `white-space:pre`·`pre-line`, `word-break` 변경, Grid placement·span, 공개 API admission을 추가하지 않는다. `ParagraphStyle`이 fixture에서 고정 입력으로 제공되는 현재 구조를 유지하며 computed CSS 전체를 자동으로 text shaping에 연결했다고 주장하지 않는다.

## 사전 기준과 고정 입력

[Chrome 관측](../spec/internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/chrome156-observation.json)은 CSS Grid Level 1 §6·§11.5.1의 anonymous item/min-content 기여 기대값과 대조한다. CSS Text Level 3의 `white-space:nowrap` 동작에 따라 soft wrap opportunity를 막는다. 고정 fixture는 Ahem `10px/10px`, text `XX XX`, 한 `min-content` column, `width:20px`, 0px border/padding/gap이다.

| 장면 | CSS 값 | Chrome 156 | 판정 |
| --- | --- | --- | --- |
| `anonymous-normal-min-content` | `white-space:normal` | column `20px`, row `20px`, root `20×20px`, 두 text fragment 각 `20×10px` | 회귀 control: 공백에서 두 줄로 wrap |
| `anonymous-nowrap-min-content` | `white-space:nowrap` | column `50px`, row `10px`, root `20×10px`, 한 text fragment `50×10px` | nowrap min-content contribution은 공백 포함 max-content와 같고 root content box 바깥으로 30px overflow |

Browser input SHA-256, Ahem asset SHA-256, Chrome user agent, viewport·DPR 및 screenshot SHA-256은 같은 evidence README에 기록한다. Browser observation의 page-level x/y는 section 위치에 따른 값이므로 acceptance는 grid-local track/frame 크기와 fragment count에 둔다.

## 계획된 변경

1. `collect()`의 anonymous Grid text allowlist에서 `Normal`과 `NoWrap`만 허용하고 `PreLine`은 기존처럼 거부한다. `order:0`, direct text run grouping, `display:none` continuation, comment 및 block separator 의미는 바꾸지 않는다.
2. 두 개의 고정 scene을 existing C11.4 opt-in runtime route에 연결한다. Root style은 각각 `white-space-collapse:collapse;text-wrap-mode:wrap`과 `white-space-collapse:collapse;text-wrap-mode:nowrap`를 사용하고 `ParagraphStyle`과 일치하는지 scene validator에서 확인한다.
3. Layout test는 normal/nowrap의 `ParagraphStyle`, min/max-content width, row/column `TrackSizing` trace, text anchor/provenance, root/item frame을 검사한다. nowrap은 min-content width 50px·height 10px, root width 20px, single line이어야 한다. normal control은 min-content width 20px·height 20px로 유지한다.
4. `white-space:pre-line` 또는 `order != 0` negative control은 명시 오류와 부분 LayoutOutput 부재를 확인한다. 기존 5개 anonymous text scene을 함께 실행해 회귀를 막는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 5개와 신규 2개를 실행한다. 두 새 scene의 root frame, track/contribution trace, node frame, draw/present를 비교한다. Surface 표본은 화면 기반 scene 영역에 고정 sample을 둘 수 있을 때만 추가하고 glyph pixel 동일성으로 확대 해석하지 않는다.
6. 기본·fixture-test FFI artifact 경계를 확인한다. 새 scene id와 payload는 fixture-test opt-in artifact에만 있어야 한다.

## 실패 기준과 남은 제한

- `NoWrap`을 받아도 `min-content` track이 `20px`이거나 `XX XX`가 두 줄이면 실패다.
- Text style이 CSS scene의 `text-wrap-mode`와 다르거나 intrinsic contribution이 다른 revision/source에 연결되면 실패다.
- `Normal` control이 20px/두 줄에서 바뀌거나 `PreLine`·비영 `order` negative control이 partial output을 내면 실패다.
- 두 simulator에서 frame, track trace, intrinsic value, node/source order, draw/present 중 하나가 Chrome 및 독립 기대값과 다르면 실패다.
- 이 opt-in slice는 일반 CSS white-space cascade, 모든 CSS Text wrapping, arbitrary fonts/scripts, WPT 적합성, 실기기, Android hardware GPU, iOS compositor 이후 pixels 또는 C11.4/C11 완료를 증명하지 않는다.

## 선행 계획 검토

구현 전에 이 계획을 서로 다른 실패 관점 20개로 검토한다([검토 기록](./c11-4-anonymous-grid-text-nowrap-plan-review-20.md)). 구현 뒤에는 새 구현/runtime 20개 검토를 수행하고 PR diff는 [필수 프로젝트 규칙](../docs/project-rules.md)에 따라 30개 독립 관점으로 별도 검토한다. 구현 결과와 한계는 `spec/STATUS.md` 및 이 계획의 실행 기록에 연결하되 상위 완료 수는 변경하지 않는다.

## 구현 결과

`Normal | NoWrap`만 anonymous Grid text paragraph allowlist에 둔다. normal control은 min-content 20px·두 줄을 유지하고 nowrap은 min-content 50px·한 줄로 계산한다. `PreLine`과 비영 `order`의 오류 경계를 유지한다. Android·iOS trace에서 root 20×10px와 text item 50×10px를 분리해 확인했고, 화면 캡처는 draw/present 경로만 증명하며 glyph pixel 판정에는 쓰지 않았다. artifact scan도 새 두 scene marker와 host FFI export가 product feature에 없는 것, fixture-test positive control에 포함되는 것만 확인한다. [구현/runtime 근거·검사 결과·검증 한계](../spec/internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/README.md) · [구현/runtime 적대 검토 20개](../spec/internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/implementation-runtime-review-20.md) · [static archive 경계](../spec/internal/evidence/c11-4-anonymous-text-nowrap-implementation-2026-10-30-2026-10-30/artifact-boundary/README.md)
