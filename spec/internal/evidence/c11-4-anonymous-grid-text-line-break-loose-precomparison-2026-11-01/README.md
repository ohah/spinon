# C11.4 anonymous Grid `line-break: loose` 사전 비교

## 입력과 환경

- 브라우저: Agent Browser `HeadlessChrome/156.0.0.0`.
- Fixture: [`anonymous-grid-line-break-loose-precomparison.html`](../../../../tests/fixtures/css/c11/anonymous-grid-line-break-loose-precomparison.html).
- Font: [`NotoSansKR-VF.ttf`](../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf), SHA-256 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`; 브라우저 `document.fonts.check()` 결과는 true다.
- Font table 독립 조회: U+6587 (`uni2F42`)과 U+FF04 (`uniFF04`) 모두 1000 UPEM 중 1000 advance(`head.unitsPerEm`, `hmtx`)여서 10px font에서 각각 정확히 10px이다.
- Source string: `文＄文` (U+6587, U+FF04, U+6587), glyph당 10px, max-content advance 합계 30px.
- Grid: content width 120px, border/padding/gap 0, direct text 하나, auto row, `font: 10px/10px`, `white-space: normal`, `word-break: normal`.
- [CSS Text 3 §5.2](https://www.w3.org/TR/css-text-3/#line-break-property)은 중국어·일본어 writing system의 `loose`에서 U+FF04 같은 East Asian fullwidth/ambiguous/wide prefix 뒤 줄바꿈을 허용한다. [Unicode UAX #14 LB25](https://www.unicode.org/reports/tr14/#LB25)는 기본 `PR × ID` prefix 뒤 줄바꿈을 금지한다. U+FF04 앞 경계는 허용되므로 `ja loose`의 segment는 `[文][＄][文]`이고 `en loose`, `ja normal` 대조군은 `[文][＄文]`이다.

## Results

| 장면 | used 열 | root row 높이 | `文＄文` 줄 위치 |
| --- | ---: | ---: | --- |
| `ja-loose-min` | 10px | 30px | x `[0,0,0]`, line `[0,1,2]` |
| `ja-loose-max` | 30px | 10px | x `[0,10,20]`, line `[0,0,0]` |
| `en-loose-min` | 20px | 20px | x `[0,0,10]`, line `[0,1,1]` |
| `ja-normal-min` | 20px | 20px | x `[0,0,10]`, line `[0,1,1]` |

Chrome 관찰 원본은 [chrome-observation.json](./chrome-observation.json), 별도 규범 계산과 font advance는 [spec-derived-expectations.json](./spec-derived-expectations.json)에 있다. 대표 화면은 [chrome156-ja-loose-min-content.png](./chrome156-ja-loose-min-content.png), SHA-256 `d6f20dd8e612f3594e5cd544b7ce47b0aaed0db355f79e2089c856fa9923dbc5`다.

기대 layout은 computed `line-break: loose`와 일본어 언어 설정을 영어 locale·`normal` 대조군과 구분한다. 이는 사전 비교이며 이 단계에서 Spinon 구현이나 runtime 동작은 검사하지 않았다.
