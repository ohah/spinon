# C11.4 direct text multiline last-baseline 제한 slice

**상위 원본:** [C11 상태](../spec/STATUS.md#c11-css-grid) · [Grid track sizing 계획](./c11-4-grid-track-sizing.md) · [CSS Grid 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing) · [CSS Box Alignment 3 baseline rules](https://www.w3.org/TR/css-align-3/#baseline-rules) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 후보. 구현 전 규범 기대값과 Chrome fixture 관측을 분리해 고정했다. C11.4/C11 완료나 공식 집계 변경은 아니다. 기준 branch의 첫-baseline 제한 fixture PR #827은 병합됐지만, 해당 경로는 multiline 및 last baseline을 지원하지 않는다.

## 해결하려는 차이

Spinon의 fixture-only Grid text adapter는 현재 first baseline만 `LayoutOutput.baselines.first`에 제공하며 정확히 한 줄인 text만 허용한다. vendored Taffy의 Grid item은 baseline 참가 여부를 일반 `Baseline` keyword 하나로 보존하고 `baselines.first`만 선택한다. 소스에는 last-baseline 및 vertical baseline 미지원 TODO가 남아 있다. 따라서 CSS `first baseline`과 `last baseline`이 같은 Taffy keyword로 내려가고, 여러 줄 문단의 마지막 line baseline으로 정렬할 수 없다.

이 계획은 고정된 두 Ahem direct-text Grid item의 multiline last-baseline 그룹만 다룬다. 구현 API/계약 초안은 Grid 내부 baseline kind를 first/last로 보존하고 text snapshot의 첫·마지막 line baseline을 기존 `Baselines` 값에 채워 전달하는 것이다. Grid의 shim은 first-baseline일 때 start side, last-baseline일 때 end side에 적용하고 각각 start/end fallback edge에 정렬한다. Taffy의 현재 단일 top-margin `baseline_shim`은 last-baseline 계약을 표현하지 못하므로, first 결과의 불변을 유지하면서 Grid 전용 shim 방향을 전달할 private 구조를 추가해야 한다. Spinon public API와 내부 계약 numeric version `0.1.0`은 바꾸지 않는다. 구현 전 vendored Taffy의 fallback 및 shim 계산을 CSS Grid/Box Alignment 규범과 대조한다.

## 고정 동작 계약

- 경로는 기존 `c11-4-grid-track-sizing-fixture-test`와 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture다. 일반 author CSS admission, root HTML의 일반 동작, public API 또는 WPT completion을 추가하지 않는다.
- 각 Grid는 `horizontal-tb`, LTR, `width:73px`, 두 explicit columns `25px 40px`, `column-gap:8px`, 한 `auto` row를 쓴다. 두 direct leaf item은 같은 row에 span 없이 놓이며 margin·border·padding은 0이다.
- 작은 item은 고정 Ahem `XX XX`, 10px font와 `line-height:1`, 25px 열에서 두 줄이다. 큰 item은 `XX`, 20px font와 `line-height:1`, 40px 열에서 한 줄이다. system font나 platform fallback을 사용하지 않는다.
- Positive scene은 `align-items:last baseline`; 대응 scene은 부모 `start`와 양 item의 `align-self:last baseline`; 세 번째 scene은 DOM source order만 뒤집는다. CSS Grid 2025 CRD로부터 기대하는 row height는 22px이고 두 last-baseline 좌표는 row start +18px다. source-order가 바뀌어도 이 geometry는 같다.
- `first baseline` control은 row 28px, item y=8/0px, 공통 first baseline 16px다. `start` control은 row 20px이며 두 item y=0으로, 잘못된 baseline 정렬이 없는지 확인한다.
- eligible text leaf는 paragraph snapshot의 `lines.first()`와 `lines.last()` baseline을 보존한다. 두 줄 미만, 잘못된 width, non-finite/out-of-range baseline, 누락된 paragraph/font에서 partial frame이나 높이 fallback으로 성공하지 않는다.
- baseline kind와 shim edge는 Grid item 내부까지 명확히 전달되어야 한다. first/last를 보통 `baseline`으로 축약하거나 두 경우 모두 top-margin shim을 쓰지 않는다. first 그룹은 start shim/start fallback, last 그룹은 end shim/end fallback을 사용한다. `LayoutOutput.baselines.last`가 모든 Grid에 영향을 주도록 하지 않는다. baseline 미존재 fallback, scroll container clamping과 auto block-margin 제외 규칙도 해당 kind 별로 규범 근거와 회귀 검사를 갖춘다.

## 고정 oracle

[사전 비교 README](../spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/README.md), [독립 기대값 JSON](../spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/expected-values.json), [Chrome 관측](../spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/chrome/chrome156-observation.json), [화면](../spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/chrome/chrome156-grid-last-baseline.png)를 별도 산출물로 유지한다. 규범값은 저장소 Ahem metrics와 CSS 규칙에서 독립 계산했다. Chrome은 고정 fixture의 교차 확인이며 expected JSON의 입력이 아니다.

Geometry 비교는 root-relative 좌표에서 1/64 CSS px까지 한다. Ahem bytes의 SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. Agent Browser는 Ahem 10px/20px load를 확인했고, browser 세션과 Chrome helper는 capture 후 닫았다.

## 구현 및 검증 단계

1. Taffy Grid의 private baseline kind 및 start/end shim 전달 방식을 확정하고 CSS Grid §11.5와 CSS Box Alignment baseline/fallback 규칙을 대조한다. 변경 범위와 API 초안을 구현 전에 계획 PR review 20개로 검토한다.
2. first baseline은 기존 first/start-shim 경로를 유지하고, last baseline은 text leaf의 마지막 line baseline과 end-shim/end-fallback 경로를 선택하도록 Taffy/adapter 경계를 수정한다. Grid baseline participation, auto margins, scroll-container behavior, shim 방향을 함께 검토한다. 최소한 Taffy Grid 단위 검증 및 기존 C11.4 first-baseline 회귀를 실행한다.
3. 고정 다섯 장면의 typed Grid input, row track-sizing trace, item frames, text line baselines를 비교한다. 두 줄이 아닌 입력, source order 역전, `align-items` 상속과 `align-self` override, 잘못된 baseline 입력에서 실패가 부분 결과를 남기지 않는지 확인한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 다섯 scene의 frame/track/draw/present를 확인한다. PixelCopy와 Metal drawable을 실행하면 capture 경계를 각각 기록하고, glyph paint 또는 iOS compositor 이후 pixel로 확대 해석하지 않는다.
5. 기본/product 및 opt-in fixture-test archive를 비교해 fixture 격리를 확인한다. 관련 Rust tests, formatting, feature-enabled Clippy를 실행한다. 변경된 구현/runtime 동작에 대한 새로운 실패 관점 20개를 기록한다.
6. 실제 PR diff를 파일/계약/링크/fixture/검증근거별 서로 다른 30가지 실패 관점으로 검토한다. PR에는 Chrome과 Android/iOS simulator 이미지를 구분해 첨부한다.
7. `spec/STATUS.md`, internal contract와 roadmap은 실행 결과만 기록한다. 이 slice의 병합으로 C11.4/C11 상위 checkbox나 공식 `72/213`을 완료로 올리지 않는다.

## 명시적으로 제외

일반 CSS admission, WPT, 임의 font와 폭, rich inline descendants, anonymous grid text, image/replaced item, span/multiple rows, vertical/orthogonal writing mode, nonzero box edges, auto block margins, scroll-container special behavior의 새 지원, Flex last-baseline, glyph raster equality, hardware 실기기, Android hardware GPU, iOS compositor 이후 pixel, 전체 CSS Grid/C11.4 완료는 이 제한 slice에 포함하지 않는다. 구현 도중 이 경계가 보장되지 않으면 범위를 더 넓히기 전에 계약을 재검토하고 별도의 계획으로 나눈다.
