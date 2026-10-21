# C11.4 두 단계 nested Grid intrinsic contribution 사전 비교

## 입력과 환경

Fixture [grid-track-sizing-c114-nested-grid-depth-two-precomparison.html](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-two-precomparison.html)은 root 아래 nested Grid 한 단계인 control과 두 단계인 test를 각각 min-content/max-content constraint에서 실행한다. 모든 Grid는 한 intrinsic column, 0 gap이며 leaf에는 저장소 Ahem `10px/10px` text `XX XXXXXXX`가 있다. Box margin/padding/border는 0이다.

Agent Browser 0.31.1이 연 Chrome `HeadlessChrome/156.0.0.0`, macOS ARM, DPR 1, viewport `1280×633`에서 `document.fonts.ready`와 `document.fonts.check('10px Ahem')`를 기다린 뒤 computed used tracks와 box frames를 기록했다. Ahem load는 `true`였다.

## 독립 계산

[CSS Grid Layout Module Level 1, 2025 CRD §5.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-container-sizing)는 Grid container의 min/max-content size를 그 constraint 아래 계산한 track sizes와 gutters의 합으로 정의한다. §11.1의 Grid sizing order에 따라 각 child Grid는 자기 track을 계산하고 그 결과가 parent grid item의 intrinsic contribution으로 사용된다. 여기서는 한 track과 0 gap만 있어 nested 단계마다 수치가 보존된다.

Ahem에서 `X` advance는 10px이고 text에는 두 `X` 단어와 하나의 collapsible 10px space가 있다. 따라서 min-content는 longest word `XXXXXXX`의 70px이고 max-content는 `XX` 20px + space 10px + `XXXXXXX` 70px = 100px이다. Min-content line wrapping은 2줄×10px=20px block size이며 max-content는 한 줄 10px다. Root→middle→inner로 Grid를 하나 더 넣어도 각 단계의 intrinsic contribution은 같은 one-track size다.

| Scene | 기대 root/middle/inner Columns | 기대 Grid widths | 기대 leaf text frame size |
| --- | --- | --- | --- |
| `nested-depth-one-min-content` | `[70px]` | `70px` | `70×20px` |
| `nested-depth-two-min-content` | `[70px]` | `70px` | `70×20px` |
| `nested-depth-one-max-content` | `[100px]` | `100px` | `100×10px` |
| `nested-depth-two-max-content` | `[100px]` | `100px` | `100×10px` |

Expected values live in [spec-derived-expectations.json](./spec-derived-expectations.json), separate from [Chrome observation](./chrome156-observation.json). Chrome returned exact integer tracks/frames: 70px with `70×20px` text for min-content, 100px with `100×10px` for max-content, at both nesting depths.

## 근거와 한계

- [Chrome screenshot](./chrome156-nested-grid-depth-two.png)
- [fixture source](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-two-precomparison.html)
- Ahem source: `tests/fixtures/resources/c15/Ahem.ttf`
- [SHA-256 manifest](./SHA256SUMS.txt)
- [계획](../../../../plan/c11-4-nested-grid-depth-two.md)

이 사전 비교는 Chrome의 한 build 및 고정 Ahem 입력이다. Spinon runtime/layout, Android/iOS Simulator, WPT, glyph paint, nested depth 3+, arbitrary CSS, Android 실기기 또는 compositor-after pixels를 검증하지 않는다. C11.4 checkbox와 완료 집계는 변경하지 않는다.
