# Root HTML Grid intrinsic track 사전 비교 · 2026-11-05

## 목적과 범위

기본 parsed Root HTML runtime의 explicit Grid track에서 `auto`, `min-content`, `max-content`와 intrinsic `minmax()` breadth가 거부되는 경계를 확장하기 전에 고정한 비교 자료다. 기준 입력은 [fixture](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-tracks-precomparison.html), 독립 판정값은 [expected.json](./expected.json), Chrome 관찰은 [chrome156-observation.json](./chrome156-observation.json), 화면은 [Agent Browser 캡처](./chrome156-agent-browser.png)다.

fixture는 columns·rows, direct track, `minmax()`, integer `repeat()`, 2-track span, `auto` stretch, hidden child, wrapping text, visible overflow, definite percentage basis와 zero-basis `fr`/percentage cases를 포함한 19개 장면이다. Percentage minimum·maximum의 계산, percentage minimum과 `1fr`의 floor, definite zero basis를 비교한다. named area, named line, auto-repeat, implicit intrinsic track, column/dense placement, arbitrary author CSS 또는 WPT 적합성 기준을 추가하지 않는다. 구현 계획은 이들 기존 경계를 그대로 둔다.

19개 중 양의 definite container axis가 있는 17개 장면은 Root HTML runtime matrix에도 들어간다. `minmax-percent-zero-basis`와 `minmax-flex-zero-basis`는 Chrome-only oracle로 보존한다. Root HTML default render는 zero-area Grid text owner box를 publish할 수 없어 zero-width/height container를 runtime에서 지원 범위로 주장하지 않으며, adapter의 fail-closed rejection을 별도로 확인한다.

## 기준 환경

- Agent Browser가 연 `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- fixture는 `alpha beta`, `font-family: monospace`, `font-size: 10px`, `line-height: 10px`, `white-space: normal`을 쓴다. Chrome observation의 glyph advances는 `alpha` 30.1094px, 전체 문자열 60.2188px다. 독립값은 이 고정 입력의 glyph advance에 CSS Grid track sizing 규칙을 적용했다.
- [CSS Grid Layout Module Level 1 §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)에서 `auto`, `min-content`, `max-content`와 `minmax()`의 track sizing 의미를, §11.3–§11.8에서 intrinsic sizing, spanning contribution, flex expansion, auto-track stretching 순서를 따른다. Chrome 값은 normative 기대값과 대조하는 관찰값이며 명세를 대체하지 않는다.
- fixture SHA-256: `d671df7fbf92f9c9a349bae71f7fb6a8b9ad70f607380700ef347f9054bdedc5`.
- observation SHA-256: `32c54c7f85f8849db136d97c49b835e94ee0ebfe970a49fc973f61ad68686b62`.
- screenshot SHA-256: `037b92d362f406a1da5a7f214c48aa6f999145356d4a4b44aa622e2ce465df0b`.

## 독립 기대와 관찰

track 및 item frame 비교 허용치는 축별 `0.02 CSS px`다. `<section>`은 장면 사이 4px 간격을 만들기 때문에 Chrome observation의 절대 y좌표는 장면 순서를 검증할 때만 쓴다. Root HTML app runtime 비교는 각 Grid 내부의 track과 item frame을 사용한다.

| 장면 | 독립 기대 track | Chrome observation | 검증점 |
| --- | ---: | ---: | --- |
| `min-content` | columns `[30.1094]` | `[30.1094]` | break opportunity 사이의 가장 긴 unbreakable run `alpha` |
| `max-content` | columns `[60.2188]` | `[60.2188]` | `alpha beta` 한 줄 |
| `auto` | columns `[60.2188]` | `[60.2188]` | `justify-content:start`에서 auto 최대 기여 |
| `minmax-intrinsic` | columns `[60.2188]` | `[60.2188]` | min-content 최소와 max-content 최대 |
| `minmax-auto-max` | columns `[60.2188]` | `[60.2188]` | auto 최소와 max-content 최대 |
| `minmax-intrinsic-flex` | columns `[160]` | `[160]` | min-content 최소와 1fr 여유 공간 확장 |
| `minmax-floor` | columns `[80]` | `[80]` | 80px minimum이 max-content보다 큼 |
| `minmax-percent-min` | columns `[80]` | `[80]` | definite 160px 축에서 50% minimum floor |
| `minmax-percent-max` | columns `[80]` | `[80]` | definite 160px 축에서 50% maximum limit |
| `minmax-percent-flex-floor` | columns `[80, 100]` | `[80, 100]` | 50% minimum floor가 남은 `1fr` 공간보다 큼 |
| `minmax-percent-zero-basis` | columns `[0]` | `[0]` | Chrome-only: zero definite axis에서 percentage minimum이 0으로 resolve |
| `minmax-flex-zero-basis` | columns `[30.1094]` | `[30.1094]` | Chrome-only: zero definite axis에서도 min-content base 보존 |
| `repeat-intrinsic` | columns `[60.2188, 0]` | `[60.2188, 0]` | 정수 repeat expansion 안의 intrinsic track |
| `min-content-rows` / `max-content-rows` | rows `[10]` | `[10]` | 양 축 typed projection, 단일 text line-height |
| `span-two-intrinsic` | columns `[30.1094, 30.1094]` | `[30.1094, 30.1094]` | 2-track span의 min/max contribution 분배 |
| `auto-stretch` | columns `[160]` | `[160]` | `justify-content:normal`이 auto track에 주는 stretch |
| `hidden-item` | columns `[30.1094]` | `[30.1094]` | `display:none` item의 contribution 제외, hidden frame 0 |
| `max-content-overflow` | columns `[60.2188]` | `[60.2188]` | 20px Grid 밖으로 이어지는 60.2188px item |

## 재현과 판정

저장소 루트에서 Agent Browser로 fixture를 열어 각 장면의 `getComputedStyle(grid).gridTemplateColumns`, `gridTemplateRows`, Grid rect, 모든 item rect와 computed font를 읽었다. 캡처 브라우저 user agent, viewport, DPR, raw geometry와 screenshot을 보존했으며 사용한 Agent Browser session/process를 닫았다.

`expected.json`은 break opportunity, intrinsic contribution, span 분배, definite available space, gap·alignment와 hidden subtree 규칙을 사용해 독립 계산한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서는 같은 fixture source를 기본 Root HTML app route에서 실행하고 frame/layout 상태를 별도로 기록한다. AVD와 Metal drawable 표면 readback은 실제로 수집한 경우에만 별도 근거로 추가한다. Chrome과 플랫폼 글꼴 advance 차이는 분리 보고하고 허용치를 사후 완화하지 않는다.
