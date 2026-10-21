# C11.4 · cyclic percentage `fit-content()` track sizing 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 세 고정 장면 구현과 Android/iOS Simulator 실행을 마치고 PR #474를 2026-10-21 21:23:35 UTC Rebase and merge로 병합했다(SHA df0c607f83732081a47ceb8435f3a174289acee7). 구현 실패 관점 20개와 PR diff 30개 검토가 끝났다. 이 제한 fixture는 C11.4 일반 지원이 아니므로 공식 checkbox와 완료 수 72/213은 유지한다. [계획 실패 관점 검토 20개](./c11-4-cyclic-percent-fit-content-plan-review-20.md) · [사전 비교](../spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/README.md) · [구현·Simulator 근거](../spec/internal/evidence/c11-4-cyclic-fit-content-runtime-2026-10-22/README.md) · [구현 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-fit-content-runtime-2026-10-22/implementation-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-cyclic-fit-content-runtime-2026-10-22/pr-diff-review-30.md)

## 목적과 제한 범위

직전 cyclic percentage `minmax()` slice는 fixed zero minimum과 percent maximum을 분리했다. 이번 계획은 `fit-content(50%)`의 intrinsic minimum과 percentage limit이 한 sizing function 안에서 함께 작동하는 경우를 기존 opt-in C11.4 runtime fixture에서 확인한다. 기존 oracle의 definite-width `fit-content(50%)` case와 다르게, 새 두 장면은 `width` 또는 `height:max-content`로 해당 축의 grid container size가 track에 의존한다. 별도 definite column control은 percentage basis가 이미 결정된 경로와 결과를 대조한다.

허용 범위는 HTML `div` root/item 두 개, Ahem direct text item 하나, 단일 explicit track, 한 축 cyclic percentage, 기존 C11.4 grid track sizing fixture-test profile이다. CSS author support를 넓히거나 새 공개 API를 추가하지 않는다. 다른 형태와 미등록 scene은 fail-closed한다.

## 입력과 독립 기대값

고정 Chrome 입력은 `tests/fixtures/css/c11/grid-cyclic-fit-content-c114-precomparison.html`이다. Ahem은 10px 글꼴과 10px line-height로 고정한다. 모든 box의 margin·padding·border·gap은 0이고 `.item`은 `min-width:0;min-height:0`으로 자동 최소 크기가 결과를 가리지 않게 한다. CSS Grid 2025 CRD의 sizing 단계와 `fit-content()` clamp에서 intrinsic basis와 percentage limit을 독립 계산하고, Chrome 156은 별도 비교값으로 사용한다.

| Scene | Root sizing | Explicit tracks | 독립 used tracks | Root frame | Item frame |
| --- | --- | --- | --- | --- | --- |
| `cyclic-fit-content-column` | width `max-content` | Columns `fit-content(50%)`; Rows `auto` | Columns `[25px]`; Rows `[20px]` | `50×20px` | `25×20px` |
| `cyclic-fit-content-row` | width `20px`; height `max-content` | Columns `20px`; Rows `fit-content(50%)` | Columns `[20px]`; Rows `[20px]` | `20×40px` | `20×20px` |
| `definite-fit-content-column` | width `40px` | Columns `fit-content(50%)`; Rows `auto` | Columns `[20px]`; Rows `[20px]` | `40×20px` | `20×20px` |

### 규범 기반 계산

[CSS Grid Layout Module Level 1, 2025 CRD §7.2.1 Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 `fit-content(limit)`의 clamp를 `min(max-content, max(auto, limit))`로 정의한다. 같은 문서의 [§11.1 Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)은 grid container 크기를 구하는 동안 track size의 cyclic percentage를 `auto`처럼 취급한 뒤, 결과 grid container 크기를 기준으로 track percentage를 다시 해석한다. 현재 §7.2.1 규범 문장에 맞춰 intrinsic basis, final limit과 used tracks를 독립 계산했다. 문서 뒤쪽 `Major Changes` 이력의 UA 조정 언급은 현재 §7.2.1 규범 텍스트가 아니므로 기대값 근거로 사용하지 않는다.

- Column item의 Ahem `aa aa`는 min-content 폭 `20px`, max-content 폭 `50px`이다. intrinsic grid sizing으로 root 폭 `50px`를 얻은 뒤 `50%` limit은 `25px`가 된다. 자동 최소 floor는 `25px` 이하여서 final column track은 `25px`다. 폭 `25px`에서 text는 두 line, root/item 높이는 `20px`이다.
- Row root의 content width는 `20px`로 고정한다. Ahem `a a a a`는 이 폭에서 네 개의 10px line으로 block-axis intrinsic height `40px`를 만든다. intrinsic sizing 단계의 cyclic row percentage를 auto처럼 취급해 root 높이를 먼저 `40px`로 정한 뒤, 최종 `50%` limit `20px`을 적용한다. `min-height:0`인 item frame과 row track은 `20px`이고 text line은 남은 root 내부 높이로 계속 그려질 수 있지만 glyph paint는 이 계획의 acceptance가 아니다.
- Definite control의 content width `40px`는 intrinsic basis가 아니므로 `50%` limit을 바로 `20px`로 계산한다. `aa aa`의 min-content 폭 `20px`이 이 limit과 같아 used column은 `20px`이고 두 줄 item의 height는 `20px`이다.

Intrinsic basis와 percentage limit은 Chrome 관찰값을 역산하지 않고 Ahem advance, inline constraint, CSS Grid sizing 단계에서 계산했다. Chrome 156 observation은 독립 기대값과 대조하는 단일 브라우저 reference이며 다른 UA 사이의 parity는 이 계획에서 검증하지 않는다.

## 사전 비교와 재현 조건

- 브라우저는 Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.8078.48`; viewport `1280×633` CSS px, DPR 1이다. `document.fonts.check('10px Ahem')`가 true이고 `document.fonts.status`가 `loaded`인 상태에서 DOMRect와 computed track list를 관찰했다.
- 고정 관찰 JSON은 `spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/chrome156-ahem-observation.json`에 저장했다. Chrome screenshot은 아래에 보인다.
- ![Chrome 156 Ahem fixture](../spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/chrome156-fit-content.png)
- 시작 입력의 fixed-size item은 24px intrinsic width라 `fit-content(50%)`의 limit clamp를 드러내지 못했다. 이를 계획 acceptance에 넣지 않고, min-content와 max-content 사이에 limit이 놓이도록 Ahem text 입력으로 수정했다. Ahem 적용 후에는 column/row의 fit-content limit이 각각 25/20px이며 definite control은 20px이다.
- Root/page y 좌표는 세 block root의 순서에 따라 달라지므로 acceptance에서는 각 root/item의 size와 root 기준 상대 위치를 비교한다. computed track list와 frame을 함께 기록한다.

## 구현 및 판정 결과

- Rust projection은 `FitContent(50%)` typed 값과 독립 기대 Columns·Rows, root/item frame을 확인한다. text runtime fixture는 정확한 세 scene ID·tree·Ahem 입력·typed track과 크기만 허용하고 미등록 입력을 거부한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 세 scene 모두 두 축 track trace, ordered root/item frame, draw/present가 통과했다. Android PixelCopy 경계 표본 6/6, iOS Metal drawable 표본 6/6이 exact이며 iOS 표본은 compositor 제출 전이다.
- 기본 FFI release `.rlib`에는 새 module marker와 scene ID가 없고 `c04-runtime-gpu-fixture-test` positive control에만 포함된다. 결과와 hash는 [artifact 경계](../spec/internal/evidence/c11-4-cyclic-fit-content-runtime-2026-10-22/artifact-boundary/README.md)에 기록했다.
- 구현 전 계획 검토와 구분되는 [구현 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-fit-content-runtime-2026-10-22/implementation-review-20.md)를 마쳤다. 최종 PR diff 검토 30개, live required check·merge 상태 확인, Rebase and merge와 Tailnet preview 갱신은 남아 있다.
- 고정 capture metadata의 source commit은 `9af5ee8fa29269c589a02d6f1aafe3f72d53d1d0`이다. simulator 결과는 fixture 범위에만 적용하며 일반 CSS 지원·WPT·실기기·iOS compositor 이후 pixels·C11.4 완료를 뜻하지 않는다.

## 범위 밖

`fit-content(calc(...))`, indefinite percentage가 포함된 다른 track grammar, percentage `minmax()`와의 결합, 한 scene의 양축 cyclic percentage, 복수 track/복수 item, nested Grid, writing mode 변경, CSS author admission, WPT, Android 실기기, hardware GPU, iOS compositor-after pixels, glyph pixel equality, border stroke, 다른 UA 실행 및 C11.4 전체 완료는 검증하지 않는다. 이번 제한 fixture 결과만으로 `fit-content()`의 일반 지원을 선언하지 않는다.
