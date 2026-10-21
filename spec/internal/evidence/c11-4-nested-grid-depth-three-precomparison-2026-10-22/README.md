# C11.4 depth-three nested Grid 사전 비교

## 입력과 재현

- 재현 HTML: tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-three-precomparison.html
- 입력 text: XX XXXXXXX
- font: 저장소 Ahem, 10px font size와 10px line-height
- browser: Agent Browser 0.31.1, HeadlessChrome 156, macOS ARM, viewport 1280×633, DPR 1
- document.fonts.check("10px Ahem")는 true였다.
- 결과: [Chrome observation JSON](./chrome156-observation.json) · [화면](./chrome156-depth-three-cases.png) · [CSS Grid 2025 CRD 독립 기대값](./spec-derived-expectations.json)

Agent Browser에서 저장소 HTML을 열고 DOM의 Grid별 computed columns/rows, used width/height, root-relative frame과 text frame을 읽었다. 페이지 offset은 fixture 배치 결과이므로 판정에 쓰지 않는다. CLI가 생성한 browser 창을 닫았으며 종료 뒤 Agent Browser helper와 Chrome 프로세스가 없는 것을 확인했다.

## 판정

| 장면 | Grid 개수 | 각 Grid columns / rows | 각 Grid 및 text |
| --- | ---: | --- | --- |
| nested-depth-three-min-content-two-axis | 4 | [70px] / [20px] | 70×20px |
| nested-depth-three-max-content-two-axis | 4 | [100px] / [10px] | 100×10px |

규범 기대값은 Chrome 관측을 베끼지 않고 Ahem advance와 CSS Grid 2025 CRD의 intrinsic sizing 순서에서 계산했다. 재현된 Chrome 결과가 이 고정 입력에서 그 기대값과 일치한다. 위치 비교 허용 오차는 1/64 CSS px이며 frame은 각 장면 root에 대해 상대화한다.

이 자료는 browser oracle과 fixture 검증만 포함한다. Spinon 구현, 모바일 앱, paint pixel, WPT 또는 일반 depth 지원을 증명하지 않는다.


## SHA-256

아래 manifest는 재현 HTML, 독립 기대값, Chrome 관측과 screenshot byte를 연결한다.

6f35f76fe9a33ee06f7a7f54b74a0245b42fdab63f587f058de1df256bcf0880  tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-three-precomparison.html
190f92b436ec24341f12ea8230bcbff669f3f2e126d2d7ad059361f1cd2df1b8  spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/spec-derived-expectations.json
a59bbbb4195935371453b11a5097f65ae6f2d27d4c47135a1be09ba931a8cf9b  spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/chrome156-observation.json
b5657ea7ba8a7028399e05bd758b2211c7ef9e1c179a6fdc17fbea767afbf8b3  spec/internal/evidence/c11-4-nested-grid-depth-three-precomparison-2026-10-22/chrome156-depth-three-cases.png
