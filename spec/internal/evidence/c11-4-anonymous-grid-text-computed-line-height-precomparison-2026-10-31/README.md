# C11.4 anonymous Grid text computed `line-height` 사전 비교

이 자료는 C11.4 anonymous Grid text의 computed `line-height` 제한 fixture 작업 전 기준을 고정한다. Chrome 결과는 브라우저 관찰이고 Spinon 실행 결과가 아니다.

## 입력과 환경

- 입력 fixture: [`c11-4-anonymous-grid-text-line-height.html`](../../../../tests/fixtures/css/c11/c11-4-anonymous-grid-text-line-height.html), SHA-256 `8dfa0356993153e5cd0a54134c6cb118d98ff5177d1edbca9ff5bd0728bab20d`
- Ahem: [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Browser: Agent Browser 0.31.1, HeadlessChrome/Google Chrome `156.0.8078.76`, macOS `26.5.1`, locale `ko-KR`, viewport `420×600`, DPR 1
- 폰트: `document.fonts.check("10px Ahem") == true`
- 재현: Agent Browser 전용 `spinon-c114-line-height` 세션에서 fixture를 열고 viewport를 `420×600`으로 설정한 뒤 `chrome/capture-observation.js`를 `eval --stdin`에 전달했다. JSON 관찰과 screenshot 저장 뒤 세션을 닫았다. terminal-browser는 사용하지 않았다.
- 화면: [`chrome156-line-height-matrix.png`](./chrome/chrome156-line-height-matrix.png), SHA-256 `73df185aff47ffbdf879b7b300a99b2e9f472113874155ecaab1cb8da82b7202`
- 구조화된 원본: [`chrome156-observation.json`](./chrome/chrome156-observation.json), SHA-256 `34b94a740f59cab64cac7fd6a3f33ca9eef69b68f483672cc4f7fecb890f9841`
- 관찰 스크립트: [`capture-observation.js`](./chrome/capture-observation.js), SHA-256 `00094845137a50f153aa1dad07bfcb9edd5cdb9ddab47a88b2d41387b7921938`

## 규범 규칙과 독립 기대값

[CSS Inline Layout Level 3 §5.1](https://www.w3.org/TR/css-inline-3/#line-height-property)은 `line-height`의 `normal`, 0 이상 `<number>`, 0 이상 `<length-percentage>`를 정의한다. 숫자 값의 preferred line height는 숫자와 computed `font-size`의 곱이고, percentage는 computed font-size의 백분율이며, `normal`은 font metrics로 정한다. `normal`의 정확한 숫자 결과는 브라우저와 글꼴 metrics에 의존하므로 아래 10px 값은 Chrome 156 관찰로만 고정한다.

[CSS Grid Layout Level 1 §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 track sizing 절차에 따라 한 개의 `auto` row가 direct-text anonymous item의 block-axis 기여를 받는다. Ahem `10px`과 두 개의 강제 줄을 쓰면 각각의 explicit number/length 기대값은 `line boxes × used line height`다. 단위 없는 숫자는 `number × 10px`; `125%`, `1.25em`, `calc(10px + 2.5px)`는 모두 `12.5px`로 계산된다. 이 산식은 Chrome 결과와 별도로 적용했다.

| 입력 | 줄 수 | 독립 기대 row | Chrome 156 row |
| --- | ---: | ---: | ---: |
| `normal` | 2 | Chrome Ahem metrics 기준 | 20px |
| `0`, `0px` | 2 | 0px | 0px |
| `0.5`, `5px` | 2 | 10px | 10px |
| `1`, `10px` | 2 | 20px | 20px |
| `1.5` | 2 | 30px | 30px |
| `2` | 2 | 40px | 40px |
| `12.5px`, `125%`, `1.25em`, `calc(10px + 2.5px)` | 2 | 25px | 25px |
| `normal` | 1 | Chrome Ahem metrics 기준 | 10px |
| `0.5` | 1 | 5px | 5px |
| `1.5` + 20px 폭의 soft wrap | 2 | 30px | 30px |
| `0` + 20px 폭의 soft wrap | 2 | 0px | 0px |

각 행은 [`expected-values.json`](./expected-values.json)에 장면별로 기록했다. `gridTemplateRows`가 track 판정값이며 root의 auto height는 별도 관찰값이다. Range rectangles는 inline text geometry이지 Grid track이나 glyph paint sample이 아니다. 특히 `line-height`가 글꼴 크기보다 작거나 0인 장면에서는 10px glyph rectangles가 겹치거나 track 밖에 있어도 행 크기는 line-height 산식을 따른다. 이 자료는 glyph 픽셀 정합성을 주장하지 않는다.

## 기존 코드와 구현 공백

`RuntimeBlockFlexGridTrackSizingV1` snapshot은 Stylo `computed_value_to_string`으로 `line-height`를 보존하지만 현재 C11.4 anonymous Grid text validator는 정확히 `10px`만 허용하고 `anonymous_paragraph_style`도 `line_height: 10.0`, `line_height_is_normal: false`를 고정한다. `ParagraphStyle`과 `InlineRunStyle` validation은 0을 거부한다. 이번 direct-text slice는 `ParagraphStyle` validation만 CSS 0 허용에 맞추고 span 전용 `InlineRunStyle` 경계는 그대로 둔다. `line-height` 전체 CSS parser나 일반 authored CSS admission은 추가하지 않는다.

## 범위와 남은 검증

기준은 Ahem 10px/400, `horizontal-tb`, LTR, zero spacing, anonymous direct text, 한 개 auto row의 기존 opt-in Grid profile이다. `normal`, unitless number, 계산된 absolute CSS px만 연결하고 unsupported/negative/non-finite serialization은 layout publication 전에 거부한다. 부모/자식 폰트 크기가 달라 number와 length inheritance가 달라지는 경우, font fallback, vertical writing, 임의 inline run, public CSS admission, 전체 WPT, glyph paint 및 전체 C11.4는 범위 밖이다.

구현 뒤에는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 opt-in scene matrix, 같은 revision의 computed style·text intrinsic result·Grid track/frame·draw/present, 그리고 default/fixture-test artifact 경계를 별도 확인한다. 현재 이 사전 비교만으로는 Spinon 구현, 시뮬레이터 동작 또는 C11 완료를 판정하지 않는다.
