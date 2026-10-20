# C11.4 음수 방향 implicit track 사전 비교

## 목적과 판정 기준

구현 전에 CSS Grid 규범에서 독립적으로 계산한 결과와 Chrome 155 관찰값을 비교한다. 이 자료는 브라우저 oracle과 계획 근거이며 Spinon 구현이나 제품 지원을 증명하지 않는다.

CSS Grid Level 2 §7.1은 numeric negative line을 explicit grid 끝에서 해석하고, explicit track이 없어도 axis마다 line 하나를 둔다. §7.5는 definite placement와 auto-placement 양쪽에서 implicit grid가 explicit grid 앞과 뒤로 확장될 수 있다고 정의한다. §7.6은 `grid-auto-columns`·`grid-auto-rows`의 목록을 반복하며 explicit grid 앞에서는 explicit grid에 가장 가까운 implicit track에 목록의 마지막 크기를 주고 뒤 방향으로 순환한다고 정한다. §8.3은 numeric line 0을 invalid로 두며 §8.5는 definite position에 필요한 track을 implicit grid의 앞·뒤에 추가하고, auto-placement cursor를 implicit grid의 시작선에 놓는다. [W3C CSS Grid Layout](https://www.w3.org/TR/css-grid/#implicit-grids)

이 fixture처럼 left-to-right 행렬을 보면 negative implicit segment의 expected track list는 authored `grid-auto-*` 목록을 reverse cycle한 값이다. 목록 `[31, 47]`에서 explicit grid 앞 track이 1개면 `[47]`, 2개면 `[31, 47]`, 3개면 `[47, 31, 47]`이다. Explicit grid 뒤쪽은 `[31, 47, ...]` 순방향이다. 각 축은 독립적으로 계산한다.

## Chrome 관찰 환경

- Agent Browser의 격리 headless Chrome 155, UA `HeadlessChrome/155.0.0.0`, macOS arm64 호스트, DPR 1, viewport 1280×633.
- `tests/fixtures/resources/c15/Ahem.ttf`를 fixture에서 로드하고 `document.fonts.check('12px Ahem')`가 `true`인 상태에서 capture했다. 최초 작성에서 Ahem URL이 한 단계 잘못된 것을 발견해 수정한 뒤 전체 25개 case를 다시 관찰했다.
- 입력은 [`../../../../tests/fixtures/css/c11/grid-negative-implicit-track-sizing-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-negative-implicit-track-sizing-c114-precomparison.html)이고 관찰 DOMRect는 CSS pixel이다. track 목록은 computed `gridTemplateColumns/Rows`이며, `spec-derived-expectations.json`은 Chrome 결과와 별도로 규범/입력에서 수기로 도출했다.
- 수치 비교 허용치는 CSS px `1/64`이다. Ahem intrinsic size는 정수 결과여야 하고, 브라우저의 일반 geometry fractional rounding만 이 오차를 허용한다.

## 관찰 파일

- [Chrome 155 computed tracks와 각 item DOMRect](./chrome155-observation.json)
- [브라우저·DPR·글꼴 환경](./browser-environment.json)
- [W3C에서 별도로 도출한 기대값](./spec-derived-expectations.json)
- [고정 기대값 비교 validator](./validate-precomparison.mjs)
- [Agent Browser 전체 화면](./chrome155-agent-browser-full.png)
- [계획 적대 검토 20개](../c11-4-negative-implicit-track-sizing-plan-review-20-2026-10-20.md)

33개 case는 음수 implicit 1–4개, 양·음수 cycle, 단일·0px·intrinsic track 목록, explicit 경계 span, reversed numeric line, span, explicit track이 없는 grid, 양 축 독립 패턴, display:none, definite column과 auto-row placement, leading negative grid에서 row/column sparse·dense auto-flow, sparse/dense cursor 차이를 드러내는 paired hole case, 양 축의 root-boundary overflow를 포함한다.

`columns-negative-fixed-root-overflow`는 100px grid root 우측 끝을 172px 넘고, `rows-negative-fixed-root-overflow`는 100px root 하단을 108px 넘는다. screenshot과 JSON의 item rect가 모두 grid border 바깥 좌표를 보존한다. 이는 이후 앱 surface gate의 경계 좌표를 정할 browser 기준이다. PixelCopy/Metal surface, Simulator 또는 실기기 결과는 아니다.

## 재현 및 결과

저장한 Agent Browser 결과에서 `node spec/internal/evidence/c11-4-negative-implicit-track-sizing-precomparison-2026-10-20/validate-precomparison.mjs`를 실행한다. 기대값/관찰값/환경의 SHA-256은 같은 디렉터리의 `sha256.txt`에 둔다.

## 한계

Chrome 155 한 구현의 관찰은 규범의 대체물이 아니다. 이 matrix는 orthogonal writing mode, subgrid, nested/anonymous/out-of-flow item, auto-repeat과 typed-list 상호작용, WPT 전체 적합성, Android/iOS renderer 또는 compositor surface를 다루지 않는다. 이 제한은 계획의 지원 경계를 넓히지 않는다.
