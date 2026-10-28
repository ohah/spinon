# C11.4 mixed-track `visibility:hidden` 사전 비교

이 자료는 fixed fixture의 Chrome 관측과 CSS 규범에서 따로 계산한 기대값이다. Spinon runtime/app 결과가 아니다.

## 입력 및 관측

- Fixture: [`grid-mixed-flex-visibility-hidden-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-mixed-flex-visibility-hidden-c114-precomparison.html)
- 기준: Agent Browser 0.31.1 / HeadlessChrome 156, macOS ARM64, viewport `1280×633` CSS px, DPR 1
- 고정 Ahem 10px와 PNG natural size `24×16px`가 준비된 뒤 관측했다.
- Chrome 결과: [관측 JSON](./chrome156-observation.json), [화면](./chrome156-visibility-hidden.png), [화면 표본](./chrome156-surface-samples.json)
- Agent Browser session/process 종료 확인: [수명주기 기록](./agent-browser-lifecycle.json)
- PR #737 본문에서 full-SHA raw image link를 Agent Browser로 열어 실제 로드와 `1280×633` dimensions를 확인했다: [검증 JSON](./plan-pr-image-agent-browser-pass01.json)
- 독립 산술: [기대값 JSON](./expected.json), 재현 명령 `python3 spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/derive_expected.py`
- 입력·기대값·화면 해시는 [SHA-256 manifest](./SHA256SUMS.txt)에 있다.

## 규범과 기대값

CSS 2.1 §11.2는 `visibility:hidden` 상자가 생성되고 레이아웃에는 영향을 주지만 그려지지 않는다고 정의한다([W3C 규범](https://www.w3.org/TR/CSS21/visufx.html#visibility)). Grid item의 track contribution과 flexible track sizing은 [CSS Grid Layout Module Level 1, 2025 CRD §11.5·§11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)에 따라 계산했다. 이전 `display:none` 비교는 box tree에서 빠져 hidden text contribution을 만들지 않는다([CSS Display Level 3](https://www.w3.org/TR/css-display-3/#box-generation)).

첫 track은 `minmax(min-content,1fr)`, 가운데 track은 fixed `40px`, 세 번째는 `minmax(0px,2fr)`다. definite target-axis size `300px`에서 두 `10px` gap과 fixed track을 제외하면 flexible tracks에 `240px`이 남고 최초 fraction은 `240/3 = 80px`이다. visible Ahem item의 min-content는 `100px`; 겹쳐 놓은 hidden Ahem sibling의 min-content는 `150px`이다. Hidden item은 같은 첫 track의 largest min-content contribution을 내므로 1fr track이 `150px`에서 freeze되고 남은 `90px`은 2fr track에 간다. 기대 tracks는 hidden scene과 visible control 양쪽 모두 `[150,40,90]px`이다.

Columns root는 `300×120px`; Rows root는 `120×300px`이고 row scene은 `vertical-rl` text를 사용한다. 각 hidden item은 `display:block`, `visibility:hidden`, client rect 1개를 가진다. Root-relative 모든 item frame과 image coordinates는 `expected.json`에 있다. Visible controls는 같은 DOM, placement, contribution, 색을 사용하고 visibility만 `visible`로 바꿔 track 결과가 변하지 않는지, 표면 표본이 자주색으로 바뀌는지 확인한다.

Screenshot 표본은 column root-local `(120,60)`과 row root-local `(60,125)`다. Hidden scene에서는 앞서 그려진 visible A item의 파랑 `#2864dc`가 남고, visible control에서는 그 위의 hidden-sibling magenta `#8e44ad`가 나타난다. Pixel coordinates와 RGB는 [표본 JSON](./chrome156-surface-samples.json)에 둔다. 이는 고정 Chrome screenshot의 두 색 대조이며 앱 Simulator surface나 모든 renderer의 보이지 않음을 보증하지 않는다.

## 경계

이 oracle는 지정한 네 fixed scene과 Chrome 156만 다룬다. 일반 CSS admission, 동적 visibility mutation, visibility inheritance/visible descendants, `visibility:collapse`, 임의 Grid item tree, 다른 font/script, WPT 실행, Spinon runtime 또는 플랫폼 동작을 증명하지 않는다.
