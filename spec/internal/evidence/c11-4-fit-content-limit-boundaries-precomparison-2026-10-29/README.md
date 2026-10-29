# C11.4 `fit-content()` 성장 한계 경계 사전 비교

2026-10-29 · WPT `fit-content-track-sizing.html`의 세 limit 경계 assertion을 구현 전에 고정했다. Agent Browser HeadlessChrome 156이 live WPT에서 13/13 pass를 보였고, live source SHA-256은 pinned revision의 원본 hash와 일치한다. 이 결과는 Chrome upstream oracle이며 Spinon WPT 통과가 아니다.

## 고정 입력과 결과

| WPT id | limit | minimum | max-content | Chrome used row |
| --- | ---: | ---: | ---: | ---: |
| `contentLargerThanArgument` | 40px | 0px | 100px | 40px |
| `smallArgument` | 30px | 0px | 100px | 30px |
| `largeArgument` | 200px | 0px | 100px | 100px |

세 원본 grid root는 height 300px, auto width, 60px explicit column, `grid-auto-rows:fit-content(...)`다. item computed `min-height`는 `0px`, overflow는 `visible`, nested child height는 100px이다. Chrome 원본 root width 1264px를 60px로 임시 정규화해도 각 row는 각각 40/30/100px 그대로였다.

독립 기대는 CSS Grid Level 1 2025 CRD §§11.2, 11.5.1의 `max(minimum,min(limit,max-content))`와 item의 explicit `min-height:0`에서 계산했다. CSSOM row, item/child geometry 및 formula를 [expected.json](./expected.json), raw browser observations는 [chrome-observation.json](./chrome-observation.json), width normalization 대조는 [normalized-width-probe.json](./normalized-width-probe.json)에 둔다.

## 원본과 화면

- Pinned source: revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, `css/css-grid/grid-definition/fit-content-track-sizing.html`.
- Source SHA-256: `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`, 5,060 bytes.
- Agent Browser full-page screenshot: [13/13 WPT results](./fit-content-track-sizing-wpt-13-results.png), 1280×5266, SHA-256 `a7eaaf9fd1d68755272c044c519d277546ec6e070861c0cf663dfa3698c77849`.
- Browser run session was closed after capture. No Spinon runtime, Android/iOS app or product artifact was built for this precomparison.
