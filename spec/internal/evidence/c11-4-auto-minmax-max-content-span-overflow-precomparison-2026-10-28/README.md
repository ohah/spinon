# C11.4 `minmax(auto, max-content)` two-track span × overflow 사전 비교

## 목적

이 비교는 기존 PR #752의 단일 `minmax(auto, max-content)` track overflow matrix와 달리, 하나의 item이 두 `minmax(auto, max-content)` columns를 함께 span할 때 automatic minimum과 span distribution이 결합하는 경계를 고정한다. Chrome 기준 관측과 독립 CSS Grid 계산이며 Spinon layout 또는 앱 runtime 결과가 아니다.

## 재현 환경

- Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, macOS ARM64 host
- viewport `1280×633` CSS px, DPR `1`
- 저장소 Ahem font를 load했고 `document.fonts.check("10px Ahem") === true`였다.
- 고정 입력: [Chrome fixture at the merged source commit](https://github.com/ohah/spinon/blob/8b90a68892aab0980d5f526d7ed47d14bb55d8dd/tests/fixtures/css/c11/grid-auto-minmax-max-content-two-track-span-overflow-c114-precomparison.html)
- Chrome 관측: [observation JSON](./chrome/chrome156-observation.json)
- 독립 기대값: [expected.json](./expected.json)
- 축별 overflow 계산 규칙: [CSS Overflow Module Level 3 §3.1](https://www.w3.org/TR/css-overflow-3/#overflow-properties)
- 화면: [Chrome 156 five-case baseline](./chrome/two-track-span-overflow.png). **Spinon runtime 화면이 아니다.**

## 독립 규범 계산

[CSS Grid 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)은 computed overflow가 scrollable overflow value가 아니며, 해당 축의 `auto` min track을 span하고, multi-track span에 flexible track이 없을 때 content-based automatic minimum을 사용하도록 한다. 따라서 `visible`과 `clip`은 100px min-content minimum을, `hidden`, `auto`, `scroll`은 0px minimum을 쓴다.

입력은 definite `80×20px` Grid, 두 `minmax(auto, max-content)` columns, fixed `20px` row, 두 columns를 정확히 span하는 Ahem 10px `XXXXXXXXXX` item이다. Item의 min-content와 max-content width는 각각 `100px`이다. §11.5는 span-two item을 non-spanning items 뒤에서 처리하고, §11.5.1은 남은 공간을 가능한 한 해당 tracks에 균등 분배한다. §11.6은 남는 definite free space를 track base sizes에 균등 분배하되 growth limits에서 멈춘다.

- `visible`/`clip`: span의 100px minimum contribution을 두 auto-minimum track에 나눠 base sizes `[50,50]px`로 만든다. Item frame은 `100×20px`이며 root width 80px을 넘는다.
- `hidden`/`auto`/`scroll`: item automatic minimum은 0px이다. Max-content contribution 100px은 두 growth limits에 `[50,50]px`를 허용한다. Definite 80px free space를 tracks에 균등 배분하면 used tracks는 `[40,40]px`; item frame은 `80×20px`, scroll width는 100px이다.

세로 overflow의 authored value는 `visible`로 고정했다. CSS overflow axis computation에 따라 X가 `hidden`/`auto`/`scroll`이면 computed Y는 `auto`가 되며 관측 JSON에 그 값을 보존했다. Item은 row track을 span하지 않으므로 이 secondary-axis value는 대상 columns automatic minimum을 바꾸지 않는다.

Chrome 156 관측의 다섯 track/frame 결과는 위의 독립 기대값과 모두 정확히 일치했다. Expected values는 Chrome observation에서 생성하지 않았다.

## Hashes and limits

SHA-256:

| File | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c11/grid-auto-minmax-max-content-two-track-span-overflow-c114-precomparison.html` | `a288f6f55a1e7fec1feac48fee0c0dcabfe10dabcf9eb63c7485836fe8c6f0fa` |
| `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| `chrome/chrome156-observation.json` | `cafacf0100bd7e98ddccdd1e26ac7278ddca547f63d50996705ed0d2a828ba16` |
| `chrome/two-track-span-overflow.png` | `2e3b70030ca123c2f715b66d831c061c34ea693474221bbc1c1cdc0684cf711f` |

The Agent Browser session was closed after capture. No `agent-browser` or `HeadlessChrome` process remained. This comparison does not exercise Spinon, app CSS admission, WPT, simulator rendering, clipping pixels, scroll offsets, scrollbars, or general CSS behavior.
