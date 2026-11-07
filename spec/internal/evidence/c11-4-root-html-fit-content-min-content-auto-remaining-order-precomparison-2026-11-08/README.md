# C11.4 fit-content/min-content/auto 남은 순서 사전 비교

## 실행 조건

- 기준 구현: Agent Browser에서 실행한 HeadlessChrome 157.0.0.0 (Google Chrome `157.0.8103.39`).
- UA: `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/157.0.0.0 Safari/537.36`.
- viewport: `1280×633` CSS px.
- 입력: [`root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html), SHA-256 `e927dcbec971bba640d3cb7c90906850c6a4fe780226bf31b0f7dc854b8d0b89`.
- Capture: [`chrome-157-order-matrix.json`](./chrome-157-order-matrix.json); [전체 장면 screenshot](./chrome-157-order-matrix.png), SHA-256 `75e70e39daf26cbe5b04575c933e96ebf2f9a47a2a589b09053db6ba0a578eae`.

Capture는 authored `grid-auto-rows`, used `grid-template-rows`, 명시 row, 네 Grid item과 네 nested payload의 frame을 각각 기록한다. Frame 좌표는 각 40×100px Grid의 local origin 기준이다. 모든 scene의 CSS row size와 item/payload frame이 계획의 독립 계산과 exact 일치했다. 각 순서에서 `20%`와 `20px` cap도 같았다.

계획 PR 변경 diff는 별도 [30개 실패 관점 검토](./plan-pr-diff-review-30.md)를 마쳤다. 이는 계획 자체의 [20개 실패 관점 검토](../../../../plan/c11-4-root-html-fit-content-min-content-auto-remaining-order-plan-review-20.md)와 구현/runtime 검토를 대신하지 않는다.

## 독립 계산

모든 Grid는 definite 100px content-box height, explicit 10px row, zero gap/edge, `align-content:stretch`를 가진다. 네 payload는 DOM order대로 `[34,34,10,6]px`다. Percentage cap은 `20% × 100px = 20px`이다.

| 순서 | independent base 합 | free space | auto track 수 | 독립 used rows |
| --- | ---: | ---: | ---: | --- |
| `min-content fit-content(20px) auto` | 80px | 20px | 1 | `[10,34,20,30,6]px` |
| `min-content auto fit-content(20px)` | 94px | 6px | 1 | `[10,34,40,10,6]px` |
| `auto fit-content(20px) min-content` | 80px | 20px | 2 | `[10,44,20,10,16]px` |
| `auto min-content fit-content(20px)` | 94px | 6px | 2 | `[10,37,34,10,9]px` |

계산에서 explicit 10px row는 auto-list index에 포함하지 않는다. 각 listed track의 initial size는 payload contribution과 20px fit-content cap으로부터 계산하고, positive remaining space는 `auto` max sizing tracks에 같은 양씩 분배한다. 이번 fixed-size child fixture에서 min-content와 max-content contributions는 같은 높이이므로 이 matrix만으로 두 contribution 종류를 구분한다고 주장하지 않는다.

이 evidence는 구현 전 기준 비교만 담는다. 현재 product code가 신규 네 order를 지원한다는 근거가 아니다.
