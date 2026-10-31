# C11.4 anonymous Grid text `text-indent` 사전 비교

## 범위

기존 `RuntimeBlockFlexGridTrackSizingV1`의 anonymous direct-text item에서 computed `text-indent`가 CSS Grid min-content/max-content track contribution에 미치는 영향만 비교했다. Chrome 156 Agent Browser에 저장소의 고정 Ahem font를 로드했다. 이 문서는 Spinon 구현·적합성 결과가 아니다.

## 입력과 환경

- Chrome: `HeadlessChrome/156.0.0.0`, macOS arm64 host, Agent Browser
- font: `tests/fixtures/resources/c15/Ahem.ttf`, SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- `document.fonts.check('10px Ahem')`: `true`
- text: `XXX X`; 10px/10px Ahem, `white-space:normal`, one anonymous direct text item, `horizontal-tb`, LTR
- grid probe: `display:inline-grid; grid-template-columns:min-content`; measured used grid track and box width
- block probe: separate `width:min-content`/`width:max-content` blocks, same text/font and computed `text-indent`

## 규범과 독립 기대값

[CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property)는 `text-indent`를 block container의 first formatted line에 적용하고 `each-line`·`hanging` 키워드를 정의한다. 이 계획은 키워드가 없는 고정 nonnegative CSS-px 값만 고정 fixture allowlist에 연결한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)와 [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 intrinsic contribution 경계를 적용한다.

고정 Ahem font의 U+0058과 U+0020 advance는 각각 1000/1000 units per em이다. 10px font의 `XXX X`에서 단어 폭은 30px·10px, collapsed separator 폭은 10px다. nonnegative indent `d`에 대해 min-content 기대값은 `max(30px + d, 10px)`이고 max-content 기대값은 `50px + d`다. 실제 runtime matrix는 이 식을 `XX`, `XX XX`, `XX   XX XX  `에 적용하며 normal whitespace collapse 이후 남은 separator와 min-content break를 따로 확인한다.

## 관측값

Block probe와 Grid min-content probe에서 모두 Ahem이 loaded된 것을 확인했다.

| text-indent | block min-content | block max-content | Grid min-content track |
| ---: | ---: | ---: | ---: |
| `0px` | 30px | 50px | 30px |
| `5px` | 35px | 55px | 35px |
| `10px` | 40px | 60px | 40px |

추가 경계 probe에서 Chrome은 `-5px`의 min-content track을 25px로 계산했다. `5%`는 computed `5%`로 남았고 이 고정 min-content probe에서 track은 30px였다. `5px each-line`은 35px, `5px hanging`은 30px이었다. 이 값들은 유효한 CSS 입력의 oracle 관측이며 candidate fixture 지원으로 간주하지 않는다. 음수는 첫 줄이 negative advance 영역에 걸칠 때 min-content break 계산이 달라지고, percentage는 intrinsic basis에 의존하며, 두 키워드는 첫 줄 이후의 indent 규칙을 바꾸므로 계획 범위에서 제외한다.

세부 관측 JSON은 [`chrome156-observation.json`](./chrome156-observation.json)에 보존했다. viewport나 색 픽셀은 이 크기 측정에 사용하지 않았다. WPT, Spinon runtime, Android/iOS 앱, glyph paint, surface pixel, 실기기 또는 GPU는 검사하지 않았다.
