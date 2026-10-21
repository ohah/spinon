# C11.4 cyclic `calc()` track sizing — 사전 비교

CSS Grid track의 cyclic percentage를 포함한 `calc()` 값이 intrinsic container sizing과 최종 track sizing에서 분리되는지 네 고정 장면으로 확인한다. 이 자료는 구현 전 oracle이며 Spinon 동작 결과가 아니다.

## 입력과 재현

- HTML: [`grid-cyclic-calc-tracks-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html)
- Agent Browser: HeadlessChrome 156.0.0.0, macOS; 페이지의 `window.cyclicCalcResults`를 읽고 1280×633 화면을 저장했다.
- CSS: 단일 item `24×16px`, gap/border/padding 0. Column track은 `calc(50% + 4px)`, row track은 `calc(50% + 2px)`다. 세 cyclic case는 `max-content` 축을 사용하고 control은 inner size를 `40×20px`로 고정한다.
- 관측: [`chrome156-observation.json`](./chrome156-observation.json) · [Chrome 화면](./chrome156-calc-tracks.png) · 화면 SHA-256 `5e6ecabef5995aff77e397af113f98afc763af2f435580bf28bc08231d62a4fd`

## 규범과 독립 계산

[CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)와 [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)은 Grid 크기가 해당 track에 의존할 때 intrinsic container size를 구하는 동안 track percentage를 `auto`처럼 취급하고, 산출된 inner size를 기준으로 최종 grid layout에서 다시 resolve하도록 규정한다. 이 fixture는 item의 max-content size로 cyclic basis를 정한 뒤 percentage와 고정 px 항을 계산한다.

| Scene | Root | Columns | Rows | Item | Chrome |
| --- | ---: | ---: | ---: | ---: | --- |
| `cyclic-calc-column` | 24×16 | 16 | 16 | 24×16 | 일치 |
| `cyclic-calc-row` | 24×16 | 24 | 10 | 24×16 | 일치 |
| `cyclic-calc-two-axis` | 24×16 | 16 | 10 | 24×16 | 일치 |
| `definite-calc-control` | 40×20 | 24 | 12 | 24×16 | 일치 |

행·열의 16/10px track과 24×16px item 차이는 각 cyclic case에서 실제 item overflow를 만든다. definite control은 cyclic percentage 분류와 산술 평가를 구분한다. 독립 입력·계산 전체는 [`spec-derived-expectations.json`](./spec-derived-expectations.json)에 고정했다.

## 경계

Chrome 관측은 한 버전의 독립 비교다. 여기서는 `minmax()`·`fit-content()` 안의 `calc()`, 복수 calc track, gap/border, WPT, Spinon runtime, Android/iOS surface 또는 일반 CSS 지원을 판정하지 않는다. Agent Browser 세션을 닫았고 세션에 연결된 Chrome 프로세스가 남지 않음을 확인했다.
