# C11.4 직교 Ahem text intrinsic 입력 일반화 사전 비교

## 실행과 원본

- Browser: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`
- Viewport: `1280×633` CSS px, DPR 1
- 입력: [self-contained fixture](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/tests/fixtures/css/c11/grid-orthogonal-text-generalization-c114-precomparison.html)
- DOM/computed-style/geometry: [observation JSON](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/chrome156-agent-browser-dom-geometry.json)
- 전체 scene 화면: [Agent Browser screenshot](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/chrome156-agent-browser.png)
- 독립 계산 재검증: [validate-precomparison.py](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/validate-precomparison.py)
- Browser cleanup: [lifecycle record](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/agent-browser-lifecycle.json)

`document.fonts.ready`와 24×16 PNG decode가 끝난 뒤 같은 페이지에서 열 target scene과 두 image 없는 whitespace control을 읽었다. `10px Ahem`은 loaded였고 target scene의 모든 image는 `complete=true`, `naturalWidth=24`, `naturalHeight=16`이었다. Observation은 user agent, viewport, computed writing mode/orientation/direction/font/ligatures/line-break/whitespace styles, row/column track 값과 root/text/image rect를 담는다. 화면은 glyph/PNG painting 확인용이 아니다.

## 고정 입력

열 target scene은 `horizontal-tb` Grid root와 `vertical-rl/mixed`, `direction:ltr` text item을 사용한다. Text는 Ahem 10px/400/normal, 명시 `line-height:10px`, `font-variant-ligatures:normal`, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`, `line-break:normal`이다. Root는 width `74px`, columns `50px 24px`, 단일 `min-content` 또는 `max-content` row, zero gap/border/padding이다. Target의 같은 row 두 번째 column에는 natural size 24×16px PNG가 있다. 마지막 두 spaces-only control은 같은 root/text style을 쓰되 PNG 없이 0px row를 관찰한다.

## 독립 기대값

저장소 Ahem font의 units-per-em은 1000이다. 사용한 ASCII 영숫자 `A`–`F`, `9`, `Z` 및 U+0020의 horizontal advance는 각각 1000 units이므로 10px font에서 advance는 10px다. 이는 별도 font table inspection으로 확인했다. CSS `white-space:normal`은 U+0020의 반복·양끝 공백을 collapse한다. 따라서 min-content inline size는 가장 긴 영숫자 단어의 advance이고 max-content inline size는 collapse 뒤 문자열 전체 advance다.

CSS Grid Level 1 2025 CRD [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)에 따라 columns 다음 rows를 계산하고, 같은 row의 비-spanning text와 image는 contribution을 더하지 않고 track 별 최대값을 취한다. CSS Writing Modes Level 3 [§6.1](https://www.w3.org/TR/css-writing-modes-3/#abstract-dimensions), [§7.3](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows)에 따라 horizontal parent의 block/row 축은 `vertical-rl` child의 inline 축에 대응한다. Image의 natural block size 16px는 여덟 개 nonempty-text target의 row를 바꾸지 않으며, 두 spaces-only target에서는 row를 16px floor로 만든다. Image 없는 두 control은 그 floor 없이 0px row를 관찰한다.

| Scene ID | Content after collapse | Track | `TextIntrinsicLogicalSizes` inline/block pair (px) | Expected row (px) |
| --- | --- | --- | ---: | ---: |
| `single-token-min-content` | `ABC` | min-content | `30/10` | 30 |
| `single-token-max-content` | `ABC` | max-content | `30/10` | 30 |
| `two-words-min-content` | `A BC` | min-content | `20/20` | 20 |
| `two-words-max-content` | `A BC` | max-content | `40/10` | 40 |
| `three-words-min-content` | `AB CDE F` | min-content | `30/30` | 30 |
| `three-words-max-content` | `AB CDE F` | max-content | `80/10` | 80 |
| `collapsed-whitespace-min-content` | `A 9Z` | min-content | `20/20` | 20 |
| `collapsed-whitespace-max-content` | `A 9Z` | max-content | `40/10` | 40 |
| `spaces-only-min-content` | empty | min-content | `0/0` | 16 |
| `spaces-only-max-content` | empty | max-content | `0/0` | 16 |
| `spaces-only-no-image-min-content` | empty | min-content | `0/0` | 0 |
| `spaces-only-no-image-max-content` | empty | max-content | `0/0` | 0 |

위 logical pair의 block 값은 현재 `TextIntrinsicLogicalSizes` API 회귀값이다. 현 구현은 min pair의 block 값을 `break_all_lines(Some(0px))`에서 nonempty line의 line-height 합으로, max pair를 unconstrained layout height로 계산한다. [CSS Box Sizing Level 3 §2.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)는 block-container min-content block size를 max-content block size와 동등하게 정의하므로, 표의 서로 다른 `min/block` 값은 CSS 규범의 block min-content 결과라는 주장이 아니다. 이 slice는 기존 API 값을 보존하고 Grid contribution에는 inline component만 사용한다. Min pair의 0px probe에서는 각 공백 분리 단어가 한 line이므로 block API 회귀값은 nonempty word 수 × 10px이고, max pair는 한 line이라 10px이다. 공백 전용 text는 CSS Text Level 3 [§3](https://www.w3.org/TR/css-text-3/#white-space-property)에 따라 collapse 뒤 빈 text가 돼 API pair `0/0`을 반환한다. Image가 있는 target rows는 natural image가 16px floor를 만들고, 별도 image 없는 두 control은 Chrome row `0px`로 빈 text의 기여를 직접 확인한다. Target scene의 root width/height는 `74px × row`, columns는 `[50,24]px`, text frame은 `50px × row`, image frame은 `24×16px`이다. Image는 grid 내부 x=50px, viewport 좌표 x=70px에 놓인다. [검증기](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/validate-precomparison.py)는 exact 12개 ID·문자열·허용 문자·좌표와 computed style을 확인하며 Chrome observation과 independent Ahem/grid 계산이 `12/12` 일치한다. Chrome은 기대값의 생성기가 아니라 독립 관측 oracle다.

## SHA-256

| File or asset | SHA-256 |
| --- | --- |
| [Fixture](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/tests/fixtures/css/c11/grid-orthogonal-text-generalization-c114-precomparison.html) | `7013790ca421401d4833ba9f3c50afbde5b9c7227780ddeda9e527337837c491` |
| [Chrome observation JSON](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/chrome156-agent-browser-dom-geometry.json) | `dae20ffebf02f953a1a90e7be2feb4c2ddb264f2a991880a1e439d89a4f3f475` |
| [Agent Browser screenshot](https://github.com/ohah/spinon/blob/plan/c11-4-orthogonal-intrinsic-generalization-20261025/spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/chrome156-agent-browser.png) | `671400c258ef786c9f0ccc0a3f35b63f8e6c00854ad30c690767d3191e434253` |
| `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| `tests/fixtures/resources/c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |

## 범위와 종료

이는 Chrome CSSOM/layout 관측과 고정 Ahem 수치의 사전 비교다. Spinon 측정, Android/iOS runtime, GPU surface, 실기기, glyph raster, PNG texel 및 OS compositor 이후 pixel을 검증하지 않는다. Agent Browser 0.31.1 session `c114og5`와 페이지가 없는 `default` session을 모두 닫았다. 직후 process scan에서 Agent Browser/Chromium/headless-shell 프로세스가 없었고 별도의 사용자 Chrome PID에는 접근하지 않았다.
