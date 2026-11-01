# C11.4 anonymous Grid text `text-transform:capitalize` 사전 비교

Chrome 156의 Agent Browser가 고정 local HTML fixture에서 Noto Sans KR variable font를 불러와 computed `text-transform:capitalize`와 intrinsic Grid track을 관찰했다. CSS Text Level 3 §2.1은 `capitalize`를 각 word의 첫 typographic letter unit이 소문자이면 titlecase로 바꾸고 나머지 문자는 그대로 두는 값으로 정의한다. Word boundary는 UA-dependent이며 UAX #29를 권고하지만 요구하지 않는다. [규범 출처](https://www.w3.org/TR/css-text-3/#text-transform-property).

구현 후보의 word profile은 direct `lang="en"`, ASCII letters와 U+0020만 허용하며 U+0020을 word separator로 정한다. ASCII punctuation, non-ASCII, tab/newline, inline run, language inheritance는 계획된 fixture에서 fail-closed할 경계다. 원문 DOM Text `iiii ii`와 `iIII ii`는 변하지 않고, shaped text 기대값 `Iiii Ii`와 `IIII Ii`만 layout 기준에 쓴다.

Font input은 Noto Sans KR `wght=400`, 10px, 1000 UPEM이다. 고정 advances는 `I=293`, `i=275`, space `=224` units. Min/max-content의 독립 합산과 Chrome CSSOM track 값은 [expected.json](./expected.json)에 따로 기록했다. Chrome track 값은 1/64px layout quantization을 포함하며 독립 합산과의 허용 오차는 0.03125px다. Mixed-case control은 capitalize가 첫 letter만 바꾸고 뒤의 uppercase `III`를 lowercase하지 않는지 확인한다. Triple-space control은 원문 공백 보존과 CSS normal whitespace collapse 뒤의 word boundary를 분리해 관찰한다.

Chrome 156.0.0.0 / macOS ARM64, DPR 1, viewport 1280×900에서 font-loaded와 여섯 장면을 확인했다. `getComputedStyle`은 authored mode를 반환하고 CSSOM Grid track sizes와 Range line rectangles를 기록했다. Range rectangles는 anonymous item box나 glyph ink의 대용이 아니다. 기계 판독값은 [Chrome observations](./chrome156-observations.json), 화면은 [Agent Browser screenshot](./chrome156-text-transform-capitalize.png), 재현 입력은 [`tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform-capitalize.html`](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-transform-capitalize.html)에 있다.

이 관찰은 CSS Text value 하나의 fixed English/ASCII profile을 계획하기 위한 Chrome oracle다. Spinon 구현, 일반 CSS 지원, WPT 적합성, Android/iOS 동작, glyph painting 또는 표면 pixel parity를 증명하지 않는다.

## SHA-256

| 입력 | SHA-256 |
| --- | --- |
| HTML fixture | `20d9318718919be403a07dbda25f620d85602c5cba7ea4212e0559858cde9b9c` |
| Noto Sans KR variable font | `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76` |
| Chrome observation JSON | `b679b16e86d14babf8722cd7adc95072b48991c00f8ee9aef9f32f1cde00869c` |
| expected JSON | `00ad1653f383787630c9ccab074edd2a38e47641188acaf4cf940409eb0df68f` |
| Chrome screenshot | `78fda42375182cbc99c6003667218c8d7c1605eb14a018d8e00aa0e00a8a21d0` |
