# C11.4 cyclic percentage `minmax()` 사전 비교

**날짜:** 2026-10-22

**상태:** 계획 단계. 이 문서는 Chrome 관찰과 규범에서 독립 도출한 기대값만 담으며 Spinon runtime 동작이나 WPT 적합성 근거가 아니다.

## 입력과 재현 환경

- 입력: [`grid-cyclic-percent-minmax-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-percent-minmax-c114-precomparison.html)
- 브라우저: Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- 원본 관찰: [chrome156-observation.json](./chrome156-observation.json) · [화면](./chrome156-minmax-cyclic-tracks.png)
- 고정 입력은 root와 item 두 개, 단일 explicit track, 0 margin/padding/border/gap 및 item `24×16px`로 제한한다.

## 독립 기대와 관찰

CSS Grid 2025 CRD [§11 Grid Layout Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 두 sizing 단계에 따라 먼저 intrinsic container basis를 정하고, 이후 결과 size로 percentage track maximum을 푼다. `minmax()`의 fixed minimum `0px`은 유지하며 cyclic percentage maximum만 intrinsic pass에서 `auto`처럼 취급한다.

| Scene | 규범 기대 | Chrome 156 관찰 | Root/item frame |
| --- | --- | --- | --- |
| `cyclic-minmax-column` | Columns `[12px]`, Rows `[16px]` | Columns `12px`, Rows `16px` | root/item `24×16px` |
| `cyclic-minmax-row` | Columns `[24px]`, Rows `[8px]` | Columns `24px`, Rows `8px` | root/item `24×16px` |
| `definite-minmax-column` | Columns `[20px]`, Rows `[16px]` | Columns `20px`, Rows `16px` | root `40×16px`, item `24×16px` |

열 장면은 24px intrinsic width를 얻은 다음 50% maximum을 12px로 해석한다. 행은 16px intrinsic height에서 8px row를 만든다. definite control은 40px basis의 절반인 20px이다. item은 고정 크기라 cyclic 열에서 x=12..24 범위와 cyclic 행에서 y=8..16 범위로 track을 넘지만 root frame을 벗어나지 않는다.

## 해석 한계

- 기대값의 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)이다. CRD는 작업 중 문서이며 이 자료는 그 고정 버전의 좁은 입력을 다룬다.
- Chrome 156은 단일 관찰 버전이다. 기존 Chrome 155 C11.4 baseline을 대체하거나 서로 다른 브라우저의 적합성을 증명하지 않는다.
- Spinon parser/projection, Taffy trace, Android/iOS 앱, 화면 표본과 artifact는 아직 검증하지 않았다.
- border stroke, iOS compositor 이후 표면, 실기기, WPT와 전체 CSS Grid는 포함하지 않는다.
