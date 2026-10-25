# C11.4 row-axis mixed text/image 사전 비교

## 범위와 환경

2026-10-25에 Chrome 156 Agent Browser 관측을 두 번 수집했다. 초기 HTML은 self-alignment를 명시한 입력이었으며 원본 파일·관측·화면을 보존했다. Runtime 실패 뒤, author `align-self`/`justify-self` 선언을 생략해 computed initial value `auto`를 사용하는 별도 HTML에서도 네 scene을 재관찰했다. 두 관측의 geometry와 화면 픽셀은 같았다. 최신 입력은 Chrome/Chromium 156, macOS ARM, `ko-KR`, viewport `1280×633` CSS px, DPR 1이다. Ahem font는 loaded이고 네 PNG는 decoded 상태다. Ahem font와 PNG는 HTML data URL에 포함되어 외부 asset 요청은 없다.

- [고정 HTML 입력](./chrome156-row-axis-fixture.html)
- [기존 Chrome 관측 원본](./chrome156-row-axis-observation-original.txt)
- [Agent Browser DOM·geometry 재관측](./chrome156-row-axis-agent-browser-dom-geometry.json)
- [Agent Browser 화면](./chrome156-row-axis-agent-browser.png)

## CSS 초기 정렬값 재관측 · 2026-10-25

초기 CSS fixture는 text에 `justify-self:stretch;align-self:start`, image에 `justify-self:start;align-self:start`를 선언했다. Runtime profile은 이 Grid item 정렬 author 값을 허용하지 않는 것이 양 플랫폼 실행에서 드러났다. 이를 fixture에서 제거하고 새 Agent Browser session으로 다시 관찰했다. 새 [HTML 입력](./chrome156-row-axis-default-alignment-fixture.html)에서 모든 item의 computed `align-self`·`justify-self`가 `auto`이며 Ahem loaded, PNG `complete=true`/natural `24×16px`을 확인했다. 네 scene 모두 columns `[50,24]px`, row `20px`, root `74×20px`, text `(0,0) 50×20px`, image `(50,0) 24×16px`이고 source order만 forward/reversed로 달랐다. 새 screenshot은 초기 화면과 SHA-256이 같아 geometry 보존을 시각적으로도 확인한다.

- [기본 정렬값 Agent Browser DOM·geometry 관측](./chrome156-row-axis-default-alignment-agent-browser-dom-geometry.json)
- [root와 item computed 정렬값 보충 관측](./chrome156-row-axis-default-alignment-computed-style-supplement.json)
- [기본 정렬값 Agent Browser 화면](./chrome156-row-axis-default-alignment-agent-browser.png)

| 파일 | SHA-256 |
| --- | --- |
| `chrome156-row-axis-default-alignment-fixture.html` | `ea2dad9dda66b4d46e889750672d30b0a79e529995cc4d13a2f7a604dadb5122` |
| `chrome156-row-axis-default-alignment-agent-browser-dom-geometry.json` | `a3242e368c459a965fe8dbfb5cf70a615de83c6f60f9e2c940cf628b8eda2ff2` |
| `chrome156-row-axis-default-alignment-computed-style-supplement.json` | `fcedd03b3a6cfa74f1a8649aab9faa7aba72fd65c87a1af87c51204f2c6be14d` |
| `chrome156-row-axis-default-alignment-agent-browser.png` | `e8a09fd96ec58d81039634b90db3b1bfd1b33cd6a357c2a9c4f9223f265ed923` |

두 Agent Browser 세션 `c114-row-axis-default-alignment-20261025`와 `c114-row-axis-root-alignment-supplement-20261025`은 각 캡처 뒤 닫았다. 첫 session의 PID `53616`, supplemental session의 browser/Chrome PID `68736`/`68754`와 전용 profile은 종료 후 남지 않았다.

기존 관측과 재관측은 네 장면 모두 `gridTemplateColumns`, `gridTemplateRows`, source order, root/track/text/image frame, image natural size가 일치했다. 재관측 JSON에는 사용자 에이전트, viewport, font 상태와 실제 DOM child order도 함께 있다.

## 고정 입력

네 Grid 모두 `width:74px`, `grid-template-columns:50px 24px`, `column-gap:row-gap:gap:0`, padding/border 0, `horizontal-tb`, `ltr`다. Grid row track 한 개만 `min-content` 또는 `max-content`로 바뀐다. 각 Grid에는 direct-leaf Ahem text `XX XXXXX` (`font:10px/10px Ahem`, normal whitespace wrapping)와 natural-size `24×16px` PNG `<img>`가 하나씩 있다. Text는 row 1 / column 1에, image는 row 1 / column 2에 명시 배치된다. `track-probe`는 absolute positioned라 track sizing 참여 item이 아니다. 정방향/역방향은 text와 image DOM child 순서만 바꾼다.

| Scene | Row track | DOM source order | Chrome used row | Root frame (x/y/w/h) | Text frame | Image frame (root-relative) |
| --- | --- | --- | ---: | --- | --- | --- |
| `min-forward` | `min-content` | text → image | 20px | 0 / 0 / 74 / 20 | 0 / 0 / 50 / 20 | 50 / 0 / 24 / 16 |
| `min-reversed` | `min-content` | image → text | 20px | 0 / 0 / 74 / 20 | 0 / 0 / 50 / 20 | 50 / 0 / 24 / 16 |
| `max-forward` | `max-content` | text → image | 20px | 0 / 0 / 74 / 20 | 0 / 0 / 50 / 20 | 50 / 0 / 24 / 16 |
| `max-reversed` | `max-content` | image → text | 20px | 0 / 0 / 74 / 20 | 0 / 0 / 50 / 20 | 50 / 0 / 24 / 16 |

Raw page coordinates place the four roots at y=`0`, `60`, `120`, and `180`; table frames are normalized to each root. All four observed `grid-template-rows` values are `20px`.

## 독립 기대값

기대값은 Chrome 결과에서 가져오지 않고 입력과 [CSS Grid Layout Level 1의 track sizing algorithm §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)를 적용해 계산했다. [CSS Sizing Level 3의 intrinsic contribution 정의 §§2.2, 5.2](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/)는 용어를 확인하는 데 사용했다. CSS Sizing 문서는 현재 Working Draft이므로 여기서는 contribution 용어를 설명하는 참고 자료로만 취급한다.

고정된 50px text column 안에서 Ahem 10px/10px의 `XX XXXXX`는 공백에서 두 줄로 감긴다. 첫 줄 `XX`는 20px, 둘째 줄 `XXXXX`는 50px이며 line-height 10px이므로 text의 block-axis 기여는 20px이다. Inline size가 50px로 고정되어 min-content/max-content row 계산에서 text의 block size가 모두 20px이다. Image의 natural block size는 16px이다. 두 direct item은 각각 같은 단일 row에 놓이므로 row가 수용해야 할 큰 값은 `max(20px, 16px) = 20px`이다. 따라서 두 row keyword 모두 20px이며 DOM 순서를 바꿔도 frame과 track 값은 같다.

## 한계

이 입력은 `min-content`와 `max-content`의 예상 결과가 모두 20px이므로 row-axis min/max 알고리즘을 서로 구별해 검증하지 못한다. 두 keyword가 같은 잘못된 경로를 타도 이 네 장면만으로는 드러나지 않는다. 이 slice는 고정 inline-size가 있는 한 row에 direct text와 replaced image가 함께 있을 때의 혼합 기여 및 source-order 불변성을 사전 비교하기 위한 자료다.

여기에는 Chrome layout observation만 있다. Spinon runtime, Android/iOS app 또는 simulator, 실기기, GPU/compositor pixels, border stroke, WPT, 일반 CSS 지원은 검증하지 않았으며 C11.4 완료 근거가 아니다.

## SHA-256

| 파일 | SHA-256 |
| --- | --- |
| `chrome156-row-axis-fixture.html` | `ab34be0125b128a1d47f0a1cc703b67e31435f3f16a48f293d9d198b94acd12a` |
| `chrome156-row-axis-observation-original.txt` | `6b1abd2f57d5960de3cbb579b784c358cf1a493fb0370573a6561ef4bb924305` |
| `chrome156-row-axis-agent-browser-dom-geometry.json` | `6c85869a6c1e0f53fb29154193b717913926c1050d54530a4b67efbeeea6bc3c` |
| `chrome156-row-axis-agent-browser.png` | `e8a09fd96ec58d81039634b90db3b1bfd1b33cd6a357c2a9c4f9223f265ed923` |

## Agent Browser 정리

재관측에 사용한 `c11-row-axis-mixed-20261025` session을 캡처 뒤 닫았다. 종료 후 Agent Browser는 `No active sessions`를 반환했고, 해당 session의 Agent Browser PID `28882`, Headless Chrome PID `28885`, Crashpad PID `28887` 및 고유 임시 profile 경로에 해당하는 프로세스가 남지 않았다. 실행 전부터 존재하던 일반 Google Chrome/Crashpad PID `440`/`864`는 건드리지 않았다.

## PR #619 본문 이미지 확인

2026-10-25에 새 Agent Browser session으로 [PR #619](https://github.com/ohah/spinon/pull/619)을 열었다. 본문에 포함된 `Chrome 156 Agent Browser row-axis fixture` 이미지는 `complete=true`, intrinsic size `1280×633px`로 로드됐다. PR 본문 폭에 맞춘 화면 표시 크기는 `782×387 CSS px`였고, viewport `1280×633`에서 `visibility:visible`, `opacity:1`, 화면 교차 확인 `true`였다. 이미지 DOM은 `.js-comment-body.markdown-body` 안에 있다.

- [PR 본문에서 보이는 Agent Browser 화면](./pr619-row-axis-image-agent-browser.png)
- [이미지 DOM·크기·표시 확인 JSON](./pr619-row-axis-image-verification.json)

확인에 쓴 `pr619-row-axis-image-20261025` session은 종료했다. 종료 뒤 `agent-browser session list`는 `No active sessions`를 반환했고, Agent Browser PID `47889`와 그 session의 Headless Chrome/임시 profile 프로세스는 남지 않았다. 기존에 실행 중이던 일반 Google Chrome PID `440` 및 Crashpad PID `864`는 그대로 두었다.

| PR 확인 파일 | SHA-256 |
| --- | --- |
| `pr619-row-axis-image-verification.json` | `8977ba0bd7a3aa9ae8febfbea1444388d9ebb34230d14f3c26817512eb2f1049` |
| `pr619-row-axis-image-agent-browser.png` | `f770eed5db65c25743981419237bc0a61539352ccada9007783b0da057796d65` |
