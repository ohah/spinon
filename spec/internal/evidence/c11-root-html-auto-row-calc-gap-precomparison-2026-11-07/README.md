# C11.4 auto-row `calc()` `row-gap` 사전 비교

기존 `RootHtmlAutoRowStretchV1`은 definite 40px block-size 기준 direct percentage gap을 받지만 CSS math `row-gap`은 거부한다. 이 후보는 finite percentage 항 하나와 finite px 항 하나를 더하거나 빼는 제한 `calc()`만 동일한 profile에 연결한다.

## 입력과 기준

- 규범: [CSS Gaps Level 1 §2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties)은 definite Grid contents layout에서 gap percentage를 해당 축의 content-box 크기 기준으로 해석한다. [CSS Grid Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters)은 row gutter가 연속한 행 사이에 놓인다고 정의한다.
- 고정 입력: 40×40px content-box Grid, 40px column, 10px explicit row, auto implicit rows, auto-placed direct child 2개 또는 3개, `column-gap:0`, `align-items:normal`, `align-content` 10개 값.
- Fixture: [`root-html-auto-row-content-alignment-calc-row-gap-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-calc-row-gap-precomparison.html), SHA-256 `575ec2f9fb06e8ccd9657d937bbd7dc4c41860600ca1fbf70870bff5e0998a5e`.
- Agent Browser 0.31.1 CLI, HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1. 환경은 [`browser-environment.json`](./browser-environment.json)에 기록했다. Chrome은 동작을 관찰하는 기준이며 기대값을 만들 때 사용하지 않았다.
- 다음 6개 식을 child 수 2/3 × alignment 10개와 교차해 120개 scene을 관찰했다: `calc(1px - 10%)`, `calc(6.25% + 2px)`, `calc(2px + 6.25%)`, `calc(25% - 2px)`, `calc(10% - 1px)`, `calc(120% - 2px)`.

## 독립 기대값과 결과

각 식은 40px × percentage + signed px 항으로 독립 계산한 뒤 `row-gap`의 비음수 범위에 맞게 clamp한다. used gap은 각각 0, 4.5, 4.5, 8, 3, 46px다. 각 item 수의 기본 row 크기와 gutter를 빼 free space를 구하고, `normal`/`stretch`의 auto-row 증가, 위치 정렬과 양의 free-space 분산 정렬을 별도로 계산한다. 기대값 계산은 browser 관측을 입력으로 사용하지 않는다.

[`derive_expected.py`](./derive_expected.py)는 120/120 scene과 1,500/1,500 row/item coordinate가 일치하는지 검증했다. 최대 차이는 `0.0208333334` CSS px로 `1/32` CSS px 허용치 이내다. 생성 기대값은 [`expected-values.json`](./expected-values.json), 검증 결과는 [`precomparison-verification.json`](./precomparison-verification.json), Chrome 관측 원본은 [`chrome-observation.json`](./chrome-observation.json)에 있다. Chrome은 음수 계산값 `calc(1px - 10%)`을 computed math로 보존하지만 used gap은 0px로 제한했다. `calc(120% - 2px)` / `center`는 첫 item y=-41px, 마지막 item y=71px로 Grid 바깥 overflow를 보였다. [`calc-row-gap-120pct-center.png`](./calc-row-gap-120pct-center.png)은 해당 screenshot이며 OS 표면 readback 근거가 아니다.

## 범위와 브라우저 종료

이 사전 비교는 임의 `calc()/min()/max()/clamp()` 문법, 순환·indefinite percentage basis, 다른 Grid shape, 일반 CSS admission, WPT 적합성, 앱 runtime, Android/iOS 동작이나 OS surface pixel parity를 검증하지 않는다. Agent Browser 세션은 exit 0으로 종료했고, 종료 뒤 `pgrep -fl '^agent-browser'`에 프로세스가 없음을 [`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에 기록했다. Terminal Browser는 쓰지 않았다.
