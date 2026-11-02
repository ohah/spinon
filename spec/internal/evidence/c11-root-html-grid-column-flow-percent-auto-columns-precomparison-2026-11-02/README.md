# C11.1 percentage implicit columns 사전 비교

이 evidence는 `RootHtmlColumnFlowV1`의 fixed-px implicit column 경로에 definite-width percentage track list를 추가하기 위한 구현 전 기준이다. 화면 관측, 독립 산술 기대값, 고정 source를 분리한다.

## 입력과 관측

- Fixture: [`root-html-grid-column-flow-percent-auto-columns-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-column-flow-percent-auto-columns-precomparison.html)
- Browser: Agent Browser가 실행한 `HeadlessChrome/156.0.0.0`, viewport `1280×633`, DPR 1
- Grid: content width 240px, height 63px, explicit columns `[30,40]px`, rows `[20,40]px`, gaps 5px/3px, 12 fixed 10×10px items, `grid-auto-flow:column`
- Raw `getComputedStyle`, used tracks, root 및 각 item `DOMRect`: [Chrome observation](./chrome156-observation.json)
- 독립 산술 기대값: [expected.json](./expected.json)
- source SHA-256: `3c449465c7b3425ad42ad7ea9a4466493f4453c49eac86fcc627ca0b81f59262`
- observation SHA-256: `e4533f8d560e9fe927e1212f2de512259d23044a5eb395cbe1d28ced7e810769`
- Chrome screenshot SHA-256: `3e41efa6393a058ee9401eabf00ed35feb8cb08f3e029b9f3db14bc9c3ba84b7`

![Chrome 156 percentage implicit-column fixture](./chrome156-default.png)

## 독립 계산

CSS Grid §7.6은 `grid-auto-columns`의 여러 track-size 목록이 필요한 implicit track에 반복 적용된다고 정의한다. definite 240px content width에서 10%와 15%는 각각 24px와 36px다. explicit 2 columns 뒤 네 implicit columns에 목록을 두 번 적용하면 used tracks는 `[30,40,24,36,24,36]px`다. column gap은 percentage basis에서 빼지 않고, track 사이 시작점 계산에 한 번씩 더한다. 따라서 starts는 `[0,35,80,109,150,179]px`, row starts는 `[0,23]px`다. item별 기대 frame과 browser observation을 별도 파일에 둔다.

보조 관측은 scalar `10%`, mixed `10px 15%`, `0%`, `120%`, definite zero-width basis를 포함한다. 120%는 288px used implicit column으로 남아 root 밖으로 보이며 clamp하지 않는다. 이 관측은 Chrome 동작만 확인한다. Spinon 구현이나 native renderer 동등성 근거가 아니다.

## 수명

관측 후 `agent-browser close --all`을 실행했다. 프로세스 검사에서 Agent Browser 및 Chrome 프로세스는 0개였다.
