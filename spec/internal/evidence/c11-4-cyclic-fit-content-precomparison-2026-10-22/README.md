# C11.4 cyclic percentage `fit-content()` 사전 비교

**날짜:** 2026-10-22

**상태:** 계획 단계. Agent Browser Chrome observation은 단일 oracle 참고 자료다. Spinon runtime이나 CSS Grid 전체 적합성 근거가 아니다.

## 입력과 재현 환경

- 입력 fixture: `tests/fixtures/css/c11/grid-cyclic-fit-content-c114-precomparison.html`.
- 브라우저: Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.8078.48`, viewport `1280×633` CSS px, DPR 1.
- Ahem font: `document.fonts.status=loaded`, `document.fonts.check('10px Ahem')=true`.
- 원본 관찰 JSON: `chrome156-ahem-observation.json`.
- 제외한 초기 입력 JSON: `chrome156-rejected-fixed-item-observation.json`. Column root/track이 모두 `24px`이라 50% fit-content cap을 구분하지 못해 acceptance에서 제외했다. 해당 Chrome 화면은 아래에 보인다.
- ![Chrome 156 Ahem fixture](./chrome156-fit-content.png)
- ![Chrome 156 rejected fixed-item probe](./chrome156-rejected-fixed-item.png)
- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1 Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§11.1 Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing).

## 독립 기대값과 Chrome 관찰

CSS Grid의 intrinsic sizing pass에서는 track size의 cyclic percentage를 `auto`처럼 취급하고, 그 pass가 정한 container size를 기준으로 final track sizing에서 percentage를 다시 푼다. `fit-content()`는 `min(max-content, max(auto, limit))` 형태의 clamp다. 이 규칙과 고정 Ahem metrics로 expected tracks와 frames를 Chrome 관찰과 독립적으로 계산했다. 문서 뒤쪽 `Major Changes` 이력의 UA 조정 언급은 현재 §7.2.1 규범 텍스트가 아니므로 사용하지 않았다.

| Scene | 규범에서 독립 계산한 결과 | Chrome 156 observation |
| --- | --- | --- |
| `cyclic-fit-content-column` | Columns `[25px]`, Rows `[20px]`; root `50×20px`; item `25×20px` | Columns `25px`, Rows `20px`; root `50×20px`; item `25×20px` |
| `cyclic-fit-content-row` | Columns `[20px]`, Rows `[20px]`; root `20×40px`; item `20×20px` | Columns `20px`, Rows `20px`; root `20×40px`; item `20×20px` |
| `definite-fit-content-column` | Columns `[20px]`, Rows `[20px]`; root `40×20px`; item `20×20px` | Columns `20px`, Rows `20px`; root `40×20px`; item `20×20px` |

Column의 `aa aa`는 Ahem 10px advance에서 min-content `20px`, max-content `50px`다. cyclic intrinsic width `50px`의 절반인 fit-content limit `25px`가 두 값 사이에 놓인다. Row는 width `20px`에서 네 line, intrinsic height `40px`를 만든 뒤 percentage limit `20px`을 쓴다. Definite control은 width `40px`의 절반인 `20px`을 직접 쓴다. Root/item size와 used tracks를 별도로 비교하며 scene들의 누적 page y 위치는 acceptance에서 제외한다.

## 한계

- Chrome 156 한 버전만 관찰했으며 Chrome 155 baseline을 대체하거나 상호 운용성을 증명하지 않는다. 다른 UA 사이의 parity는 검증하지 않았다.
- 이 단계에서는 Spinon parser/projection/layout, Android/iOS 앱, surface pixels와 fixture artifact 경계를 검증하지 않았다.
- Ahem glyph pixel equality, CSS border stroke, Android 실기기, hardware GPU, iOS compositor-after surface, WPT 및 전체 C11.4는 범위 밖이다.
