# C11.4 anonymous Grid negative `text-indent` 사전 비교

## 범위와 환경

이 비교는 anonymous direct text item의 negative computed `text-indent`가 Grid min-content/max-content column과 item frame에 미치는 영향을 고정한다. Spinon 구현 검증이나 CSS 적합성 주장이 아니다.

- Browser: Agent Browser의 `HeadlessChrome/156.0.0.0`, macOS ARM64 host
- Fixture: [`c11-4-anonymous-grid-text-negative-indent-precomparison.html`](../../../../tests/fixtures/css/c11/c11-4-anonymous-grid-text-negative-indent-precomparison.html)
- Font: pinned Ahem 10px/10px, weight 400; `document.fonts.check("10px Ahem")`가 `true`
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Input: `XXX X`, `X XXX`, `XX`; `white-space:normal`; `horizontal-tb`; LTR; anonymous direct text; no inline padding, border, gap, or child element
- Observation: computed `text-indent`, computed Grid track, grid box width/height. Glyph raster와 표면 pixel은 판정에 사용하지 않았다.

원본 관측은 [`chrome156-observation.json`](./chrome156-observation.json), 화면은 [Agent Browser 캡처](./chrome156-agent-browser.png)다. JSON에는 candidate `-5px`·`-10px`와 `-30px` boundary probe 총 18개가 들어 있다.

**구현 전 계획:** [C11.4 negative `text-indent` 제한 계획](../../../../plan/c11-4-anonymous-grid-text-negative-indent.md) · [계획 실패 관점 검토 20개](../../../../plan/c11-4-anonymous-grid-text-negative-indent-plan-review-20.md) · [계획 PR diff 검토 30개](../../../../plan/c11-4-anonymous-grid-text-negative-indent-plan-pr-diff-review-30.md)

## 규범과 독립 기대값

[CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property)은 `text-indent`의 `<length-percentage>`에서 길이값을 정의하며, `hanging`·`each-line`은 별도 행 동작으로 정의한다. 이 후보는 유효한 음수 `<length>` 중 fixed CSS-px `-5px`와 `-10px`만 allowlist한다. Percentage와 키워드는 계속 거부한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes) 및 [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 intrinsic contribution/track sizing과 Ahem의 X·space advance 1000 units per em(10px at 10px font)을 사용한다.

`XXX X`와 `X XXX`는 각각 `30px + 10px separator + 10px = 50px` max-content advance를 갖는다. `d`는 첫 formatted line에만 더한다. 단어 경계에서 줄을 나눌 수 있으므로 candidate의 독립 min-content 값은 `XXX X: max(30 + d, 10)` 및 `X XXX: max(10 + d, 30)`이고, max-content는 `50 + d`다. `XX`는 나눌 지점이 없어 min/max 모두 `20 + d`다. 각 결과는 0 이상이다.

| `text-indent` | text | min-content track | max-content track | min-content row | max-content row |
|---:|---|---:|---:|---:|---:|
| -5px | `XXX X` | 25px | 45px | 20px | 10px |
| -10px | `XXX X` | 20px | 40px | 20px | 10px |
| -5px | `X XXX` | 30px | 45px | 20px | 10px |
| -10px | `X XXX` | 30px | 40px | 20px | 10px |
| -5px | `XX` | 15px | 15px | 10px | 10px |
| -10px | `XX` | 10px | 10px | 10px | 10px |

Chrome의 12 candidate track widths와 row heights가 위 산식과 일치했다. 전체 expected JSON은 [`expected.json`](./expected.json)이다.

## 제외 경계

`-30px`은 candidate가 아니라 oracle-only boundary다. Chrome은 `X XXX`의 min-content/max-content Grid track을 둘 다 30px로, `XX`는 둘 다 0px로 관측했다. 해당 경계는 첫-line negative indent가 raw max-content advance를 min-content보다 작게 만들 수 있어 text intrinsic min/max 불변 조건과 Grid track floor 상호작용을 별도로 요구한다. 따라서 이번 candidate는 -30px, 임의 음수 범위, general CSS admission을 지원하지 않으며 이 edge를 구현 결과로 주장하지 않는다. 이 경계를 포함한 여섯 Chrome scene은 추후 독립 slice 검토를 위해 관측만 고정한다.

Chrome 관측으로 명세 의미를 대체하지 않는다. 이번 비교는 WPT, Spinon runtime, 앱·시뮬레이터, glyph paint, surface pixel, 실기기 또는 GPU를 검증하지 않았다.
