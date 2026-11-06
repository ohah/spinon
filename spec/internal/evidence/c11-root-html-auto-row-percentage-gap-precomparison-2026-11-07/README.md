# C11.4 auto-row alignment percentage `row-gap` 사전 비교

기존 고정 40×40px `RootHtmlAutoRowStretchV1`에서 computed percentage `row-gap`을 추가하는 후보의 사전 oracle을 기록한다. `row-gap`은 definite 40px content-box block size를 기준으로 0%, 6.25%, 12.5%, 18.75%, 20%, 100%, 120%를 사용한다. 따라서 used gutter는 각각 0, 2.5, 5, 7.5, 8, 40, 48px다. `column-gap:0`, explicit 40px column과 10px row, direct children 3개, `align-items:normal`, 기존 10개 `align-content` allowlist는 유지한다.

## 기준과 독립 기대값

- 규범: [CSS Gaps Level 1 §2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties)는 gap percentage를 요소 content box의 해당 축 크기에 대해 해석한다. definite size에서 contents layout은 그 content box를 기준으로 한다. [CSS Grid Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters)에서 Grid row-gap은 행 사이 gutter다.
- 입력 fixture: [`root-html-auto-row-content-alignment-percentage-row-gap-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-auto-row-content-alignment-percentage-row-gap-precomparison.html), SHA-256 `70f8aaca5c24751e959827b576a3a1f62b7c1c92fcd47a72d9f5b052f3870c41`.
- Browser: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, viewport 400×300 CSS px, DPR 1. UA/viewport는 [`browser-environment.json`](./browser-environment.json)에 있다.
- 7 percentage values × 10 alignment values = 70 cases를 관찰했다. 원본 browser JSON은 [`chrome-observation.json`](./chrome-observation.json), 독립 기대값 생성식은 [`derive_expected.py`](./derive_expected.py), 생성된 기대값은 [`expected-values.json`](./expected-values.json)에 있다.
- Independent formula: base row heights `[10,10,10]px`; `g = 40px × percentage`; pre-alignment free space는 `40 - 30 - 2g`. `normal|stretch`는 양의 free space만 두 implicit auto rows에 균등 배분한다. 위치/분배 정렬은 CSS Grid content alignment를 계산하고, 음수 free space의 distributed alignment는 start로 fallback한다.
- 검증기 [`derive_expected.py`](./derive_expected.py)는 모든 used row와 item frame을 Chrome observation과 대조했다. 70/70 통과, 최대 좌표 오차 `0.0260416667 CSS px`, 허용 오차 `1/32 CSS px`다. [`precomparison-verification.json`](./precomparison-verification.json)에 결과가 있다. Chrome은 독립 계산을 대신하지 않는 관측 기준이다.

## Overflow observation

`row-gap:120%`는 48px gutter를 만든다. `align-content:center`에서 세 row는 `[10,10,10]px`이며 첫 item은 Grid start보다 43px 위, 마지막 item은 Grid bottom보다 43px 아래에 놓인다. [`chrome-120pct-center.png`](./chrome-120pct-center.png), 400×300px, SHA-256 `e3e19c6934d3f65d4b9ac8a2df69585c9856b7b9436b43c2be20265b0bc55a92`.

## Browser lifecycle

[`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에 Agent Browser 세션 close exit 0과 종료 뒤 `pgrep -fl '^agent-browser'` 결과가 비어 있음을 기록했다. Terminal Browser는 사용하지 않았다.
