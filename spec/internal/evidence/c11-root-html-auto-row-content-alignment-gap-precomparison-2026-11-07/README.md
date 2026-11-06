# C11.4 implicit auto-row content alignment + row-gap 사전 비교

기존 Root HTML auto-row alignment shape에서 computed `row-gap`을 고정 0px만 받는 제한을 넓히기 위한 Chrome 기준을 고정한다. 이번 후보의 허용 범위는 finite non-negative px `row-gap` 0–7.5px, `column-gap:0`, definite 40px block size다. 이 범위에서 base tracks와 gutters가 차지하는 높이는 30–45px이며, positive·zero·negative free space를 모두 포함한다. 이는 구현 결과나 WPT 적합성 근거가 아니다.

## 입력과 환경

- Fixture: [`root-html-auto-row-content-alignment-row-gap-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-row-gap-precomparison.html)
- Fixture SHA-256: `12f0cdd179845e44b3e445e74ee9fde040361c048123768a6b5c5fed979eeb99`
- Agent Browser 0.31.1 / HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1
- Grid content box 40×40px, one 40px column, one 10px explicit row, two direct-text implicit `auto` rows, `column-gap:0`
- Browser observation: [`chrome-observation.json`](chrome-observation.json)
- Independent expected values: [`expected-values.json`](expected-values.json)
- Reproducible expected-vs-Chrome comparison: [`verify-precomparison.mjs`](verify-precomparison.mjs), output [`precomparison-verification.json`](precomparison-verification.json)
- Browser lifecycle: [`agent-browser-lifecycle.json`](agent-browser-lifecycle.json). All five sessions were closed; PR #1239's three Chrome reference images loaded at 1280×633; the final `pgrep -fl '^agent-browser'` result was empty.

## 독립 계산과 결과

세 row base는 `[10,10,10]px`다. Row gap을 `g`로 둘 때 alignment 전 남는 공간은 `40 - 30 - 2g`다. `g=0`, `2.5`, `5`, `7.5px`에서 각각 10, 5, 0, -5px다. `normal|stretch`에서는 양수 공간을 두 implicit auto rows에 균등 배분한다. 다른 값은 track base를 유지한 채 남는 공간으로 track group을 배치한다. Negative free space에서 `end`는 첫 row를 y=-5px, `center`는 y=-2.5px에 두어 start edge 밖 overflow를 만든다. `start`와 distributed values는 start 위치 `[0,17.5,35]px`로 fallback한다. Direct text item의 final block-size는 `align-items:normal` stretch에 따라 해당 auto row 크기와 같아진다. 네 gap 값 × 열 alignment 값 40개 scene의 track size와 item frame은 독립 계산과 Chrome 관측에서 좌표별 0.03125 CSS px 이내로 맞았다.

`space-around`의 분수 위치는 Chrome의 1/64 CSS px layout quantum에 맞춰 비교한다. Expected formula, raw browser output, viewport와 DPR을 따로 보존했다.

## 캡처

- `row-gap:2.5px`, `align-content:start`: [`chrome/align-content-start.png`](chrome/align-content-start.png), SHA-256 `31e2d78b240726faf0aa12874e1ff0358451b34f3c2804d359c7daf14122c6ec`, 1280×633
- `row-gap:2.5px`, `align-content:space-between`: [`chrome/align-content-space-between.png`](chrome/align-content-space-between.png), SHA-256 `d7ec3a99780e6cbb648b3bcdb0b3f917aef9249c0e03c37592908cf3d3fd32f1`, 1280×633
- `row-gap:7.5px`, `align-content:end`: [`chrome/align-content-end-gap-7_5.png`](chrome/align-content-end-gap-7_5.png), SHA-256 `4de8a8f2ca8feb9e821986abc4847c7ca40dc53e05285d3af7a461ae20c028ec`, 1280×633

Chrome은 비교 기준이다. 이 자료는 0–7.5px gap 범위에서 네 고정 지점만 다룬다. 7.5px에서는 `end`·`center`가 start edge 밖으로 overflow하지만 computed `safe|unsafe` modifier는 범위 밖이다. 더 큰 gap, non-zero column gap, collapsed gutters, 다른 item/topology shape, arbitrary CSS admission, Spinon runtime, WPT, 실기기·hardware GPU 결과는 주장하지 않는다.
