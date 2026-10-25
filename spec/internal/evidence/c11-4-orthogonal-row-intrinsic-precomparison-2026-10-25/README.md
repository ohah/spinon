# C11.4 orthogonal text + natural image row intrinsic 사전 비교

## 입력과 실행

고정 HTML을 Agent Browser 0.31.1의 고유 namespace `c114oa25`, session `oa25a`에서 열었다. 이번 재관측은 HTML을 바꾸지 않고 DOM computed-style 기록과 화면을 갱신했다. Chromium은 `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1이다. `document.fonts.ready`와 네 이미지의 `decode()` 완료 뒤 같은 DOM 관측에서 computed style, track 값, source order와 root/item rect를 읽었다.

- <a href="/spinon/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/chrome156-orthogonal-row-fixture.html">Self-contained CSS 입력</a>
- <a href="/spinon/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/chrome156-orthogonal-row-agent-browser-dom-geometry.json">Agent Browser DOM·computed-style·geometry JSON</a>
- <a href="/spinon/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/chrome156-orthogonal-row-agent-browser.png">Agent Browser 화면</a>
- <a href="/spinon/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/agent-browser-lifecycle-alignment-recheck.json">재관측과 브라우저 종료 증거</a>

HTML은 저장소 Ahem과 PNG bytes를 data URL로 embed한다. Ahem font face 400이 loaded이고 `10px Ahem`이 사용됨을 확인했다. 네 이미지는 모두 `complete=true`, natural size `24×16px`이다. 이는 Chrome 브라우저 관측이며 Spinon runtime 증거가 아니다.

## 고정 입력과 computed style

각 Grid root는 `horizontal-tb`/`ltr`, content width `74px`, auto height, columns `50px 24px`, 단일 row `min-content` 또는 `max-content`, gap 및 edge 0이다. Root의 authored/computed alignment는 `align-items:normal`, `align-content:start`, `justify-items:normal`, `justify-content:start`다. Text와 image는 `align-self:auto`, `justify-self:auto`다. 이 값은 계획의 scene-specific matcher와 동일하며, 기존 #611/#620 common validator의 `align-items:start` 기대값을 변경하지 않는다.

Text는 column 1 / row 1의 direct `<div>`로 `writing-mode:vertical-rl`, `direction:ltr`, `text-orientation:mixed`다. DOM computed style은 `font-family:Ahem`, `font-size:10px`, `font-weight:400`, `font-style:normal`, `line-height:10px`, `white-space:normal`, `white-space-collapse:collapse`, `text-wrap-mode:wrap`, `word-break:normal`, `overflow-wrap:normal`, `line-break:auto`를 기록한다. 내용은 `XX XXXXX`다. Image는 column 2 / row 1이며 writing mode/direction/orientation을 재정의하지 않는다. 실제 computed style은 상속된 `horizontal-tb`/`ltr`/`mixed`다.

각 root는 페이지에서 scene을 분리하기 위해 `position:absolute`와 고정 `top`을 쓴다. 페이지 좌표는 JSON에 보존하고 아래 표는 `relativeRect` 기준이다. Root 외부 위치는 내부 track sizing에 영향을 주지 않는다.

## 독립 규범 계산

CSS Grid Layout Module Level 1 2025 CRD [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)은 columns를 먼저 크기화한 뒤 그 결과를 사용해 rows를 계산한다. [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)와 [§11.5 step 2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)는 한 intrinsic track의 non-spanning item에 대해 해당 track 함수의 같은 종류 contribution 최댓값으로 track base를 정한다.

부모 `horizontal-tb`의 block axis는 물리 세로 축이고, 자식 `vertical-rl`의 inline axis도 물리 세로 축이다. 이 논리 축 매핑은 W3C Recommendation인 [CSS Writing Modes Level 3 §6.1](https://www.w3.org/TR/css-writing-modes-3/#abstract-dimensions) 및 [§7.3](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows)의 orthogonal-flow sizing 설명을 따른다. 따라서 부모 row의 text 기여는 자식의 inline contribution이다.

CSS Box Sizing Module Level 3 [§2.1–2.3·§5.2](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/#intrinsic-contribution)는 2026-09-04 Working Draft이며 intrinsic sizing 용어를 확인하는 참고다. Grid track algorithm의 출처를 대체하지 않는다. 고정 Ahem 10px font에서 `X`와 공백의 advance는 각각 10px이다. 공백에서 soft-wrap 가능한 `XX XXXXX`의 min-content inline size는 가장 긴 단어 `XXXXX`의 `5×10=50px`, max-content inline size는 전체 여덟 advance의 `8×10=80px`이다. Font bytes에서 `unitsPerEm=1000`, `X`/space advance가 각각 1000임도 확인했다.

PNG replaced item의 natural block size는 `16px`이다. Text와 image는 각기 다른 column에 놓인 row-span-1 item이므로 같은 track에서 기여를 더하지 않고 최댓값을 취한다. 기대 row는 `min-content=max(50,16)=50px`, `max-content=max(80,16)=80px`이다. CSS Grid §6.2의 `normal` automatic item sizing 표에 따라 text는 Grid area를 채우고 replaced image는 natural size `24×16px`를 유지한다. Row가 하나이고 root height는 `auto`여서 root 높이는 각 row track과 같다.

| Scene | Root | Columns | Rows | Text item | Image item |
| --- | --- | --- | --- | --- | --- |
| `min-forward` | `(0,0) 74×50px` | `[50,24]px` | `[50]px` | `(0,0) 50×50px` | `(50,0) 24×16px` |
| `max-forward` | `(0,0) 74×80px` | `[50,24]px` | `[80]px` | `(0,0) 50×80px` | `(50,0) 24×16px` |
| `min-reversed` | `(0,0) 74×50px` | `[50,24]px` | `[50]px` | `(0,0) 50×50px` | `(50,0) 24×16px` |
| `max-reversed` | `(0,0) 74×80px` | `[50,24]px` | `[80]px` | `(0,0) 50×80px` | `(50,0) 24×16px` |

## Chrome 관측과 범위

Agent Browser의 네 scene 모두 계산값과 일치했다. JSON은 root/text/image 각각의 computed `writing-mode`, `direction`, `text-orientation`을 담는다. Root는 `align-items:normal`, `align-content:start`, `justify-items:normal`, `justify-content:start`; text는 Ahem font/line/whitespace/wrap/word-break/overflow-wrap 값을 모두 기록한다. 네 case에서 rows는 `[50]` 또는 `[80]px`, columns는 `[50,24]px`, text frame은 `50×50/80px`, image frame은 natural `24×16px`이다. Forward/reversed 순서의 geometry도 같다.

화면은 root/item box 배치를 확인하기 위한 참고다. Glyph raster나 글자 방향의 pixel equality, PNG texel/color equality, Spinon runtime, Android/iOS simulator, 실기기, hardware GPU, 또는 iOS compositor 이후 pixel을 검증하지 않는다.

기존 [row-axis 사전 비교](../c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)는 horizontal `XX XXXXX` block text의 두 row contribution이 `20px`/`20px`라 min/max 행 함수를 구별하지 못한다. 이번 입력은 text item만 orthogonal writing mode로 두어 부모 row가 text inline contribution `50px`/`80px`을 받게 한다. 기존 runtime adapter가 `vertical-rl`을 지원함을 증명하지 않으며, 해당 경계는 계획의 첫 runtime feasibility gate다.

## Source hashes

| File or asset | SHA-256 |
| --- | --- |
| `chrome156-orthogonal-row-fixture.html` | `87a30a71718e80db40ccb83653f43958866fd6dba47d37506b9f79c1c0a3c10a` |
| `chrome156-orthogonal-row-agent-browser-dom-geometry.json` | `14487410e9000fb78c067c8df80655c57d8396f36e80f6bd1d6e940ed72ed0b6` |
| `chrome156-orthogonal-row-agent-browser.png` | `8a7ff6e88e13c3ada5d5947b064ff5352e11098a5c9b60e7cff4854d358418c5` |
| Repository `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| Repository `tests/fixtures/resources/c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |

## Agent Browser 정리

재관측은 고유 namespace `c114oa25`, session `oa25a`에서 수행했다. <a href="/spinon/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/agent-browser-lifecycle-alignment-recheck.json">Lifecycle JSON</a>은 `close --all` 뒤 session list가 비었고 session inactive/PID null, controller PID `14890` 및 Chrome PID `14920`이 종료됐으며 profile 경로가 사라졌음을 기록한다. Process/profile 검색에는 확인용 shell만 남았다. 기존 `c114_orthogonal` 외 다른 namespace를 건드리지 않았다.
