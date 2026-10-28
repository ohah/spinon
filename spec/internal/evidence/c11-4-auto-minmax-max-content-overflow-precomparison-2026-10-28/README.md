# C11.4 `minmax(auto, max-content)` × overflow automatic minimum 사전 비교

## 목적

이 사전 비교는 automatic minimum 값이 `auto` minimum / `max-content` maximum track의 base size와 final used size에 미치는 제한 경계를 고정한다. 기존 #750의 test-only overflow matrix는 `minmax(auto, 1fr)`을 사용했다. 이 입력은 max-content growth limit과 scrollable-overflow minimum을 함께 확인하지 않는다.

이 자료는 Chrome 기준값과 독립 규범 계산이다. Spinon layout 또는 앱 runtime 결과가 아니다.

## 재현 환경

- Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, macOS ARM64 host
- viewport `1280×633` CSS px, DPR `1`
- 저장소의 Ahem TTF를 load하고 `document.fonts.check("10px Ahem") === true`를 확인했다.
- 다섯 PNG 입력을 모두 `decode()`한 뒤 결과를 수집했다. 원본은 `24×16`; CSS width `15px`로 transferred row size `10px`을 만든다.
- 고정 입력: [Chrome fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-auto-min-max-content-c114-precomparison.html)
- 원 관측 JSON: [Chrome 156 observation](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/chrome/chrome156-observation.json)
- 화면: 아래의 전체 10-scene Chrome baseline. **Spinon runtime 화면이 아니다.**
- 독립 규범 기대값: [expected.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/expected.json)

![Chrome 156 Agent Browser 10-scene baseline](./chrome/mixed-auto-min-max-content-overflow.png)

## 규범 계산

[CSS Grid 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)은 grid item의 automatic minimum을 콘텐츠 기반으로 정하는 조건을 computed overflow가 scrollable overflow value가 아닌 것, 해당 축에 `auto` minimum track을 span하는 것, 그리고 다중 span이면 flexible track을 포함하지 않는 것으로 정한다. 그 외에는 automatic minimum을 0으로 둔다. 이 입력은 한 `minmax(auto, max-content)` track만 span하므로 `visible`과 `clip`은 content-based minimum, `hidden`, `auto`, `scroll`은 0 minimum을 사용한다.

[CSS Grid 2025 CRD §§11.5–11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 `auto` minimum은 그 item의 minimum contribution으로 track base를 정한다. Max-content maximum의 growth limit은 item max-content contribution을 반영한다. Columns의 Ahem `XXXXXXXXXX`는 min-/max-content `100px`; definite container는 `60px`이다. Rows의 natural image는 natural size `24×16px`, CSS width `15px`, transferred row size `10px`; definite grid height는 `5px`이다.

따라서 content-based controls는 columns track `100px`, rows track `10px`로 container를 넘는다. Scrollable-overflow controls는 각각 0 minimum에서 시작하지만 definite available space로 maximize되어 columns `60px`, rows `5px`를 쓴다. Row image는 `align-self:start`라 item frame `15×10px`를 유지해 `5px` row track 밖으로 넘는다. 이 결과는 item clip pixels나 scroll behavior를 검증하지 않는다.

## 고정 결과

| Axis | Overflow | Automatic minimum | Used track | Item frame relative to grid |
| --- | --- | ---: | ---: | --- |
| Columns | `visible`, `clip` | 100px content-based | 100px | 100×20px |
| Columns | `hidden`, `auto`, `scroll` | 0px | 60px | 60×20px |
| Rows | `visible`, `clip` | 10px content-based | 10px | 15×10px |
| Rows | `hidden`, `auto`, `scroll` | 0px | 5px | 15×10px |

The complete ten case IDs, root frames, contribution values, and max-content contributions are in [expected.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-auto-minmax-max-content-overflow-precomparison-2026-10-28/expected.json). Chrome's used tracks and frames match these independent values exactly.

## Hashes and evidence limits

SHA-256:

| File | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c11/grid-auto-min-max-content-c114-precomparison.html` | `5ae90da3b67a358fabd1b2fa7a802f0094dadfb623e6264cfb4038d08888cd3e` |
| `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| `tests/fixtures/resources/c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| `chrome/chrome156-observation.json` | `c9241efd0ecef6da9f30b854d5cdf35dbb7d5a5c6aa8cd8aa9078b9f0df998ec` |
| `chrome/mixed-auto-min-max-content-overflow.png` | `683f47cad2fcd30f0f828a9b265f7f9ac0ecc4aee7a89bed3a326326dcb41652` |
| `expected.json` | `a5a38cc9ac64fdf0e9f166b39f901ffc35cb0f0cffbf69156360cd22be763e4c` |

The Agent Browser session was closed after capture. The final session list was empty and no agent-browser process remained. This comparison does not exercise Spinon, app CSS admission, WPT, simulator rendering, clipping, scroll offsets, or pixel equality.
