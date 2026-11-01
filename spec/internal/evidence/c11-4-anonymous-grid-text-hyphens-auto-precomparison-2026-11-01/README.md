# C11.4 `hyphens:auto` 사전 비교

## 질문과 범위

CSS Text 3 §5.3은 `hyphens:auto`가 언어에 맞는 hyphenation resource를 사용하도록 하고, 자동 기회를 min-content 계산에 반영한다. 정확한 분할 지점은 CSS Text가 정하지 않는다. 따라서 Chrome 관측은 현재 UA 동작의 참고값이고, 고정 영어 패턴의 구현 기대값과 같은 계약으로 취급하지 않는다.

이번 계획 후보는 기존 C11.4 익명 direct-text Grid fixture를 확장해, `lang=en`과 Ahem 10px 아래의 세 고정 영단어에만 `hyphens:auto`를 계산하는 것이다. 일반 CSS admission·임의 단어·다른 언어·전체 WPT는 포함하지 않는다.

## 재현 환경

- Chrome `156.0.8078.80` / Agent Browser CLI `0.31.1` / macOS.
- 로컬 HTML: [`anonymous-grid-text-hyphens-auto.html`](anonymous-grid-text-hyphens-auto.html).
- 글꼴: 저장소 fixture [`Ahem.ttf`](../../../../tests/fixtures/resources/c15/Ahem.ttf), 10px/10px, `lang=en`, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`.
- Agent Browser에서 `document.fonts.check("10px Ahem")`는 `true`; 자동 계산값은 computed `hyphens:auto`인 DOM 요소에서 관찰했다.
- 전체 화면: [`chrome156-agent-browser.png`](chrome156-agent-browser.png).
- 구조화된 브라우저·dictionary 관측값: [`chrome-observations.json`](chrome-observations.json).

## 관측값

표의 min-content 폭은 CSS `width:min-content` 요소의 bounding box다. 줄 목록은 DOM 원문 문자를 각 UTF-16 위치별 Range rectangle의 y 좌표로 묶은 값이다. 브라우저가 그리는 자동 하이픈 문자는 DOM 원문에 없으므로, 목록은 원문 분절만 나타낸다.

| 단어 | `none` min-content | Chrome `auto` min-content | `auto` 50px의 원문 줄 분절 |
| --- | ---: | ---: | --- |
| `extraordinary` | 130px | 110px | `ext` / `rao` / `rdin` / `ary` |
| `hyphenation` | 110px | 50px | `hyp` / `hena` / `tion` |
| `internationalization` | 200px | 50px | `int` / `ern` / `at` / `iona` / `li` / `zat` / `ion` |
| `representation` | 140px | 40px | `repr` / `es` / `ent` / `ation` |

Chrome 156에서 고정 폭 줄바꿈과 intrinsic 폭 감소를 관찰했다. 이 값은 동일한 단어의 하이픈 위치나 다른 플랫폼 UA의 사전 결과를 보장하지 않는다.

## 고정 영어 패턴 후보

임시 Rust probe는 `hypher = 0.1.8`, `default-features = false`, `features = ["english"]`를 사용했다. registry lock checksum은 `45f2b4f6b5da34d400d4c112eadcc14512cc5d04307a8eee182586d9bebd7ce6`이다.

| 단어 | Hypher 분절 | Ahem 10px에서 분절 뒤 하이픈을 포함한 최대 폭 |
| --- | --- | ---: |
| `hyphenation` | `hy-phen-ation` | 50px |
| `internationalization` | `in-ter-na-tion-al-iza-tion` | 50px |
| `representation` | `rep-re-sen-ta-tion` | 40px |
| `extraordinary` | `ex-tra-or-di-nary` | 50px |

Chrome과 Hypher의 정확한 분할 위치는 서로 다르다. 예를 들어 `hyphenation`은 Chrome에서 `hyp` / `hena` / `tion`, Hypher에서는 `hy` / `phen` / `ation`이다. CSS Text 3이 정확한 지점을 규정하지 않으므로, 구현 계획은 고정 Hypher 버전의 breakpoints와 Ahem 측정을 독립 기대값으로 삼고 Chrome은 브라우저 관측으로만 비교한다.

## 구현 전 확인할 경계

- `auto`는 HTML 콘텐츠 언어가 정확히 `en`으로 고정된 direct-text fixture에만 허용한다. 언어가 없거나 다른 언어면 추측하지 않고 거부한다.
- 자동 분절 대상은 `hyphenation`, `internationalization`, `representation`으로 고정한다. `extraordinary`는 Chrome/Hypher 차이 관측용이며 첫 구현 matrix에서는 제외한다.
- 허용 단어는 모두 ASCII 45바이트 이하이므로 Hypher 0.1.8의 no-alloc 영어 feature에서 긴 단어 panic 경계를 넘지 않는다. 구현은 이 길이 상한도 호출 전에 확인한다.
- dictionary marker는 layout용 normalized text에만 추가한다. DOM text와 UTF-16 source offset은 바꾸지 않고, 원문 source mapping·revision을 보존해야 한다.
- 기본 제품 artifact에는 optional English patterns가 없어야 한다. opt-in C11.4 fixture-test artifact에는 positive control로 포함되는지 검사한다.

## 해시

- HTML: `761ab28dbceacc62be2487cab155437bcfd4c9d355180950e6b75d765f61b2dc`
- screenshot: `819d3c652b2d0b7a44cec1d8ea49a1ce9e69cbd24d86f9271701b576f939b6ac`
- Ahem fixture: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- checkout source: `c5c4084fadb8a973d001bc345d65332b4330b7c6`

## 출처

- [CSS Text Module Level 3 §5.3](https://www.w3.org/TR/css-text-3/#hyphenation): `auto` 언어 사전, min-content, UA별 breakpoints.
- [Hypher 0.1.8 문서](https://docs.rs/hypher/0.1.8/hypher/): 내장 pattern, 언어별 feature, no-alloc 45바이트 한계.
- [Hypher Cargo manifest](https://github.com/typst/hypher/blob/main/Cargo.toml): `MIT OR Apache-2.0`, `english` feature, `alloc` feature.

이 사전 비교는 기능 구현이나 Spinon의 browser compatibility pass를 뜻하지 않는다.
