# C11.4 repeated intrinsic text content precomparison

**상태:** 독립 Chrome 관측 완료 · 제품 구현 전 · 2026-11-09

## 입력과 환경

- Fixture: [HTML source](../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-text-content-precomparison.html)
- Fixture SHA-256: `803a8ee4b1be9e0c8469f828df0bb3ac8c7be1f1f2f7da1495ce2b80fc982d95`
- Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, DPR 1
- 40px column, 10px explicit row, 여덟 implicit-row text item, 0 gap, `align-content:start|stretch`
- payload style: `monospace` 10px, 10px line-height, weight 400, normal style, zero spacing, `white-space:normal`

Chrome oracle는 Agent Browser의 `getComputedStyle`, `getBoundingClientRect`, Range client rect를 사용했다. [캡처 코드](./capture-matrix.agent-browser.js), [jq 축약 필터](./reduce-matrix.jq), [한 장면당 한 줄로 쓰는 formatter](./format-expected.py)는 이 fixture와 세 콘텐츠 corpus를 대상으로 한다.

## 장면과 관측

`[min-content ×2, max-content ×2, auto ×1]`의 30개 고유 tail 순서를 생성했다. 각 corpus에서 220px Grid의 네 cap (`20px`, `20%`, `0%`, `120%`) × 두 alignment (`start`, `stretch`) 240장면과 100px Grid의 `20px/start` overflow 30장면을 측정했다. 세 corpus 합계는 810장면이며 각 장면은 grid rows, 9 item frames, 최대 8 payload frames를 기록한다.

| Corpus | Chrome Range line counts | 전체 | 100px overflow |
|---|---|---:|---:|
| `legacy-1290` | `[2,2,2,1,2,1,3,1]` | 270 | 30/30 |
| `varied-ascii` | `[3,2,2,2,2,2,2,2]` | 270 | 30/30 |
| `length-boundary-32` | `[5,6,5,4,4,1,3,4]` | 270 | 30/30 |

32 ASCII byte payload는 5줄, 31 byte payload는 6줄로 관측됐다. 경계 corpus의 100px overflow cohort에서 가장 아래 payload bottom은 240px로 Grid 바닥을 140px 넘었다. 기존 #1290 corpus는 회귀 기준으로 유지한다.

각 frame과 track 크기는 CSS px로 저장했다. 실제 구현 비교 허용치는 수치별 0.01 CSS px, track order·item/payload 수·문자열 길이·line count는 exact다. 이 파일은 Chrome 기준을 기록하며 Spinon 제품 적합성이나 Android/iOS 폰트 동등성을 주장하지 않는다.

## 산출물

- [Chrome 157 기대 geometry 810장면](./chrome157-expected.json)
- [Agent Browser capture script](./capture-matrix.agent-browser.js)
- [JSON reduction filter](./reduce-matrix.jq)
- [Per-scene JSON formatter](./format-expected.py)
- [Chrome 157 varied ASCII 장면 화면](./chrome157-varied-ascii-scaled.png). 가독성용 2.5배 CSS transform을 적용한 화면이며 수치 oracle은 JSON이다.
