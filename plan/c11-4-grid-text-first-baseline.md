# C11.4 direct text first-baseline 제한 slice

**상위 원본:** [C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing 계획](./c11-4-grid-track-sizing.md) · [CSS Grid 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 검토를 마치고 제한 구현 후보를 두 Simulator에서 실행했다. 구현 PR [#827](https://github.com/ohah/spinon/pull/827)이 열려 있으며 PR diff 검토·병합 전이다. `C11.4` 및 `C11` 완료를 뜻하지 않는다. 기존 Grid baseline-shim 알고리즘은 텍스트 leaf의 first baseline을 받지 못해 item 높이로 대체했다. 이 계획은 고정 Ahem 텍스트 두 개의 first-baseline 그룹에 실제 line baseline을 전달하는 제한 profile을 검증한다.

## 고정 동작 계약

- 범위는 opt-in `c11-4-grid-track-sizing-fixture-test`의 `RuntimeBlockFlexGridTrackSizingV1` 경로다. public author CSS admission, 일반 root HTML 지원 또는 WPT 적합성은 추가하지 않는다.
- Grid container는 `horizontal-tb`, `direction:ltr`, 단일 `auto` row, 두 `auto` columns, `column-gap:8px`, `width:max-content`, `height:auto`로 고정한다. 고정 Ahem 문자열로 사용 폭은 68px, 행 높이는 20px가 된다. 두 direct Grid item은 같은 row에 span 없이 놓이며, 각각 한 줄 Ahem text leaf다.
- Positive scene은 10px 및 20px Ahem text, `line-height:1`, `align-items:first baseline`로 구성한다. 각 item에 개별 지정한 `align-self:first baseline` scene과 source-order 역전 scene도 같은 기대값을 가져야 한다.
- Differential control은 같은 입력에서 `align-items:start`를 사용한다. 이 control에서는 두 item의 block-start가 같고 glyph baseline은 서로 달라야 한다.
- item의 margin·border·padding은 모두 0이다. 해당 profile의 eligible text leaf만 `TextLayoutSnapshot` 첫 줄의 `baseline`을 Taffy `LayoutOutput.baselines.first`로 제공한다. Grid item margin은 Taffy의 기존 baseline shim 단계에서 별도로 더한다.
- fixture는 system font를 사용하지 않고 고정 SHA의 저장소 Ahem bytes만 등록한다. Runtime evidence에는 font hash와 `font_registry_revision`을 함께 남긴다.
- 계산 실패, 누락된 paragraph/font/line, non-finite baseline은 부분 결과나 높이 fallback을 성공으로 내보내지 않고 이 opt-in scene을 거부한다. 이미 지원하는 non-baseline Grid 입력은 결과를 바꾸지 않는다.

## 규범 기대값과 고정 oracle

[사전 비교 fixture](../tests/fixtures/css/c11/grid-first-baseline-direct-text.html), [독립 기대값과 Agent Browser 결과](../spec/internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/README.md), [JSON](../spec/internal/evidence/c11-4-grid-text-first-baseline-precomparison-2026-10-30/expected-values.json)를 구현 전에 고정했다. 기준은 [CSS Grid 2025 CRD §11.5 track-sizing baseline shim](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)이며 browser observation은 별도 교차 확인이다.

저장소 Ahem font는 1000 UPEM에서 ascent 800, descent −200이다. `line-height:1`인 단일 line은 10px/20px text에서 각각 10px/20px 높이와 line-top 기준 8px/16px baseline을 만든다. §11.5 baseline shim에서 row group baseline은 16px, item shim은 작은 text 8px·큰 text 0px이다. 따라서 auto row는 20px이고 item의 row-start 상대 y는 각각 8px·0px이다. 두 열은 각각 20px·40px이고 gap 8px이므로 container는 68×20px다.

## 구현 및 검증 경계

1. Text layout snapshot과 Taffy child layout 사이에 C11.4 opt-in Grid text first-baseline 전달을 구현한다. 내부 계약 0059에 제한 입력·출력·오류 경계를 문서화하고 고정 버전 `0.1.0`을 유지한다. first baseline 이외의 baseline set, Flex baseline, `last baseline`, vertical writing mode는 범위 밖이다.
2. `align-items`, `align-self`, source-order 역전 및 `align-items:start` control을 같은 fixed input/hash matrix에 포함한다. 각 scene에서 typed Grid input, Rows track-sizing trace, item frame과 text snapshot baseline을 비교한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 고정 matrix의 frames, draw/present와 root/item fill·gap 색 표본을 확인한다. 현재 경로는 text glyph paint를 검증하지 않으므로 baseline 계산은 paragraph metrics와 frame/track trace로 판정한다. PixelCopy와 Metal은 각각의 capture 경계로 기록하고 iOS compositor 이후 표본이라고 표현하지 않는다.
4. 기본/fixture-test FFI archive 경계, 관련 Rust 회귀, formatting 및 변경 범위에 맞는 Clippy를 확인한다. 실패 주입·잘못된 baseline과 unsupported input이 부분 frame을 남기지 않는지 대조한다.
5. 계획 실패 검토 20개, 구현/runtime 실패 검토 20개, PR diff 검토 20개를 서로 독립된 항목으로 기록한다. PR 본문에는 Chrome baseline과 Android/iOS Simulator 화면을 구별해 첨부한다.
6. `spec/STATUS.md`와 공개 roadmap 서술을 실제 제한 slice에 맞춰 갱신하되 C11.4/C11 체크와 전체 공식 완료 수는 승격하지 않는다.

## 명시적으로 포함하지 않는 항목

복수 line/wrapping, rich inline descendants, anonymous text grouping, empty text baseline synthesis, image/replaced-item baseline, span이 여러 row를 가로지르는 baseline group, scroll-container clamping, nonzero margin/border/padding, orthogonal/vertical writing mode, last baseline, Flex baseline, WGPU glyph paint, 임의 author CSS와 전체 C11.4/CSS Grid/WPT 지원은 이 slice로 주장하지 않는다. C11.4 stage D의 나머지 회귀 조합은 계속 추적한다.
