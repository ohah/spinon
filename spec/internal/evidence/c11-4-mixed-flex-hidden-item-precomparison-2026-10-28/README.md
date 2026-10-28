# C11.4 mixed flex/fixed track · `display:none` intrinsic child 사전 비교

이 폴더는 다음 제한 fixture 계획을 위한 구현 전 oracle이다. Spinon runtime이나 앱의 성공 결과가 아니다. Chrome 관측과 CSS Grid 규칙에서 독립 계산한 기대값을 구분한다.

## 고정된 입력과 환경

- Fixture: [`grid-mixed-flex-hidden-item-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-mixed-flex-hidden-item-c114-precomparison.html)
- 기준 브라우저: Agent Browser로 관찰한 Chrome 156 (`HeadlessChrome/156.0.0.0`), macOS ARM64, viewport `1280×633` CSS px, DPR 1
- Ahem 10px font가 준비된 상태에서 관찰했고, 네 PNG가 모두 decode되어 natural size `24×16` CSS px임을 확인했다.
- 원본 관측: [`chrome156-observation.json`](./chrome156-observation.json) · 화면: [`mixed-flex-hidden-item.png`](./chrome/mixed-flex-hidden-item.png)
- 재계산: `python3 spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/derive_expected.py`; 결과는 [`expected.json`](./expected.json)이다.

## 규범 규칙과 독립 계산

기대값은 [CSS Grid Layout Module Level 1, 2025 CRD §11.5 및 §11.7–§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)에서 직접 계산했다. `display:none`은 해당 요소의 subtree를 box tree에서 제외한다는 [CSS Display Level 3의 box-generation 규칙](https://www.w3.org/TR/css-display-3/#box-generation)도 적용한다. `visibility:hidden`은 box와 layout 참여를 유지하므로 같은 입력으로 간주하지 않는다.

각 Grid의 sizing 축은 definite `300px`이고 반대 축은 `120px`이다. Track list는 `minmax(min-content, 1fr) 40px minmax(0px, 2fr)`, target-axis gap은 각 `10px`이다. Fixed track과 두 gap을 빼면 flex tracks에 `240px`이 남아 첫 hypothetical fraction은 `80px`이다.

| Scene | Chrome used tracks | 독립 계산 |
| --- | --- | --- |
| `column-hidden` | `[100, 40, 140]px` | 큰 숨김 text의 box가 없으므로 첫 track base는 보이는 Ahem text의 `100px`; 1fr을 freeze하고 나머지 `140px`을 2fr에 준다. |
| `column-visible-control` | `[150, 40, 90]px` | 같은 150px sibling을 보이게 한 대조군은 첫 base `150px`; freeze 뒤 남은 `90px`을 2fr에 준다. |
| `row-hidden` | `[100, 40, 140]px` | `vertical-rl` text가 row sizing 축에 만드는 `100px` 기여만 남는다. |
| `row-visible-control` | `[150, 40, 90]px` | visible 대조군의 row-axis `150px` 기여가 첫 base를 높인다. |

네 scene 모두 `track sum + 2 gaps = 300px`, remaining free space는 `0px`이다. 독립 산출물은 `Fraction`을 사용하고 root-relative item frame, hidden-box 상태, freeze 입력을 [`expected.json`](./expected.json)에 함께 기록한다. Chrome CSSOM의 final track/frame 값은 browser control이지 규범 산술의 입력이 아니다.

## SHA-256

| Artifact | SHA-256 |
| --- | --- |
| Fixture | `916b0fcd1c35b9d863ff31d1e7e4dae137ff726309e6cb330666d6aeb9b4d193` |
| `derive_expected.py` | `683faf1bd47b29fd9d53bf1a8f72e43c915967741be0f4f7494e47bc93e41635` |
| `expected.json` | `6ea70d8ff4a2d3a0c1e724e32f923a048be8ec8e2aaebddb3a36140331e6b9c8` |
| Chrome observation | `903619e93afcb3486d92be03ccba8e2046d95b612743a4d98bdf508b7dca9d71` |
| Chrome screenshot | `db6e46b6963955f8eeeb05fb104d88b0a54ed8559d5aec28d53925683d2948eb` |

계획 구현에서 필요한 경우 기존 fixture를 바꾸면 새 oracle revision으로 다시 관찰하고 hash를 갱신한다. 이 자료만으로 Spinon layout, CSS admission, 앱 draw/present, Android/iOS 동작 또는 전체 C11.4 완료를 주장하지 않는다.
