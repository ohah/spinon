# parsed Root HTML intrinsic Grid + gap 사전 비교

## 범위

이 자료는 merged bounded candidate [PR #1167](https://github.com/ohah/spinon/pull/1167)의 `gap:0` admission을 따라, intrinsic explicit tracks와 기존 Root HTML의 **고정 양수 px gap** 조합을 검증하기 위한 구현 전 비교다. Chrome 관찰, 독립 산술 기대값, Spinon 구현 결과를 분리한다. 이 파일은 Spinon runtime 통과, 일반 CSS 지원 또는 C11 완료 근거가 아니다.

## 재현 입력과 환경

- Fixture: [`root-html-grid-intrinsic-gap-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-gap-precomparison.html)
- Browser: Agent Browser HeadlessChrome `156.0.0.0`, `1280×633` viewport, DPR 1
- Browser process는 비교 화면과 screenshot을 수집한 뒤 종료했고, 로컬 fixture server도 종료했다.
- 원본 브라우저 관찰: [`chrome156-observation.json`](./chrome156-observation.json)
- 독립 기대값: [`expected.json`](./expected.json)
- 화면: [`chrome-156-agent-browser.png`](./chrome-156-agent-browser.png)
- 입력·관찰·기대값·화면 SHA-256: [`SHA256SUMS.txt`](./SHA256SUMS.txt)
- 계획 PR diff 적대 검토: [`20개 관점`](./plan-pr-diff-review-20.md)

## 규범 기반 기대값

각 Grid는 content box `100×100px`, columns `min-content minmax(0, 1fr)`, rows `min-content minmax(0, 1fr)`다. profile이 허용하는 row-locked 배치를 사용한다. 첫째와 둘째 item은 row 1에, 셋째 item은 row 2에 고정하고 열은 자동 배치한다. fixed item 크기는 차례로 `40×10px`, `20×20px`, `10×10px`다. gap이 없는 control은 columns `[40,60]`, rows `[20,80]`이다. 고정 `10px` column gap은 남은 inline free space를 10px 줄여 columns `[40,50]`과 둘째 item의 x offset 50px을 만든다. 고정 `10px` row gap은 rows `[20,70]`과 셋째 item의 y offset 30px을 만든다. 두 gap을 함께 쓰면 두 축에서 각각 같은 변화가 난다. 입력과 독립 계산은 `expected.json`에 root-relative frame과 resolved track 크기로 보존한다.

Chrome의 네 fixed-gap positive scenes는 이 기대값과 정확히 일치했다. 다섯 번째 `10%` column-gap scene은 Chrome에서 유효하며 used gap 10px로 관측됐다. 이번 후보는 **fixed px만** 넓히므로 이 scene은 positive가 아니며, Spinon에서는 기존 fail-closed 경계가 유지되어야 한다.

## 계획된 수용 범위

- 기본 `RuntimeRootHtmlTextPaintV1`에 `row-gap`·`column-gap`의 finite non-negative fixed px 값만 허용한다.
- `gap` shorthand 및 row/column longhand에서 같은 typed px 값이 들어오는 네 fixed-gap scene을 비교한다.
- Intrinsic track sizing trace와 기존 fixed/percentage/`fr`/fit-content/placement/area/percent-gap 회귀를 보존한다.
- percentage, `calc()`, negative/non-finite typed gap, non-zero box edges, auto/percentage axes, implicit tracks와 모든 일반 CSS/WPT 범위는 미포함이다.

계획 자체의 구현 전 리뷰와 계획 PR diff review는 구현/runtime 검토와 별도로 수행한다.
