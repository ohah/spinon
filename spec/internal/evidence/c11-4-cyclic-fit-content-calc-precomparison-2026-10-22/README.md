# C11.4 cyclic `fit-content(calc())` 사전 비교

**상태:** 계획 단계의 고정 입력과 Chrome 156 관측이다. Spinon runtime·Android/iOS 앱 실행이나 CSS Grid 전체 적합성 근거가 아니다.

## 입력과 재현 환경

- Fixture: [`grid-cyclic-fit-content-calc-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html).
- Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.8078.48`, viewport `1280×633` CSS px, DPR 1.
- Ahem 10px/10px line-height font가 `loaded`이고 `document.fonts.check('10px Ahem')`가 true일 때 `getComputedStyle()` track list와 root/item `getBoundingClientRect()`를 수집했다.
- 원본 관측: [Chrome observation JSON](./chrome156-ahem-observation.json). [Chrome 화면](./chrome156-fit-content-calc.png).
- 입력 fixture·사용한 Ahem font·oracle observation·화면·기대값 JSON의 [SHA-256 manifest](./SHA256SUMS.txt)를 보관한다.
- 전용 Agent Browser session `spinon-c11-4-fit-content-calc-oracle`은 확인 후 닫았다. 종료 뒤 session 목록과 Agent Browser/연결 Chrome process scan은 비었다([종료 기록](./browser-close.json)).
- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing).
- 계획 PR의 30개 diff 검토는 [plan-pr-diff-review-30.md](./plan-pr-diff-review-30.md)에 있다.

## 독립 계산과 브라우저 관측

§7.2.1은 cyclic percentage를 intrinsic grid container size 계산 중 `auto`처럼 취급하고, 결과 container size를 얻은 뒤 layout에서 다시 해석하도록 한다. `fit-content(limit)`의 used size는 `max(auto minimum, min(limit, max-content))`로 계산한다. 기대값은 고정 Ahem contribution과 이 두 sizing 단계로 계산하고, Chrome 156은 독립 oracle 비교값으로 보관한다.

| Scene | 독립 기대값 | Chrome 156 관측 |
| --- | --- | --- |
| `cyclic-fit-content-calc-column` | limit `50% × 50 + 4 = 29`; Columns `[29]`, Rows `[20]`; root `50×20`; item `29×20` | Columns `29px`, Rows `20px`; root `50×20`; item `29×20` |
| `cyclic-fit-content-calc-row` | limit `50% × 40 + 2 = 22`; Columns `[20]`, Rows `[22]`; root `20×40`; item `20×22` | Columns `20px`, Rows `22px`; root `20×40`; item `20×22` |
| `definite-fit-content-calc-column` | definite limit `50% × 40 + 4 = 24`; Columns `[24]`, Rows `[20]`; root `40×20`; item `24×20` | Columns `24px`, Rows `20px`; root `40×20`; item `24×20` |
| `definite-fit-content-calc-small-limit` | definite limit `50% × 10 + 4 = 9`; explicit item `min-width:0` permits the 9px track; Columns `[9]`, Rows `[20]`; root `10×20`; item `9×20` | Columns `9px`, Rows `20px`; root `10×20`; item `9×20` |

열 column scene의 Ahem `aa aa`는 min-content 20px, max-content 50px이다. 50px intrinsic width에서 limit 29px가 두 값 사이에 놓인다. row scene의 `a a a a`는 20px inline width에서 40px의 intrinsic block size를 만들고, 최종 calc limit은 22px이다. definite controls는 percentage basis를 각각 40px와 10px로 고정해 24px·9px limit을 대조한다.

### Root 바깥 overflow 표본

small-limit scene에서 root/item은 너비 10px/9px이지만 Ahem `aa` 한 단어는 20px이다. viewport screenshot에서 root 기준 `(5, 5)`는 item 내부이고 `(15, 5)`는 root와 item 양쪽의 오른쪽 경계 바깥이며, `(25, 5)`는 글자 영역 밖이다. 픽셀 결과는 [`chrome156-surface-samples.json`](./chrome156-surface-samples.json)에 저장했다. 첫 두 점의 색은 `#e5484d`, 세 번째는 흰 페이지 배경이다. 이 검사는 화면 overflow sample이며 CSS border stroke 검사가 아니다.

## 한계

- Chrome 156 한 버전의 관측이다. 다른 UA 상호 운용성이나 Chrome 155 baseline을 증명하지 않는다.
- 이 사전 비교에는 Spinon parser/projection/layout, 앱 runtime, Android/iOS surface, fixture artifact 경계가 포함되지 않는다.
- glyph pixel equality, border stroke, WPT, Android 실기기, hardware GPU, iOS compositor 이후 표면 및 전체 C11.4는 범위 밖이다.
