# C11.4 익명 Grid 텍스트 `hyphens` 사전 비교

## 기준 구현과 규범

- 기준 구현: Agent Browser로 연 Headless Chrome `156.0.0.0` (UA: `HeadlessChrome/156.0.0.0`), 2026-11-01. CSS Text Module Level 3 §5.3은 [공식 W3C 문서](https://www.w3.org/TR/css-text-3/#hyphenation)의 2026-08-14 Candidate Recommendation Draft를 읽었다.
- 규범상 `hyphens`는 `none | manual | auto`, 초기 `manual`, inherited, computed value는 지정 keyword다. hyphenation 기회는 min-content intrinsic size에 포함된다. `manual`은 U+00AD SOFT HYPHEN 같은 명시적 기회에서만 단어를 끊고, `none`은 그 기회를 막는다. 눈에 보이는 U+002D HYPHEN-MINUS의 기존 soft-wrap 기회는 `none`에도 남는다. `auto`는 language-specific resource가 필요하다.
- 고정 문서: [anonymous-grid-text-hyphens.html](./anonymous-grid-text-hyphens.html). Grid 자식은 anonymous direct text다. 외부 label은 block 흐름으로 두어 parent Grid stretch가 측정 대상 row에 개입하지 않게 했다. [화면 캡처](./chrome-agent-browser.png)는 장면 위치를 보여주는 보조 자료며 숫자는 DOM geometry observation이다.

## 독립 Ahem 계산

`tests/fixtures/resources/c15/Ahem.ttf` SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. fontTools 4.60.2가 읽은 `unitsPerEm=1000`; `A`, `B`, `C`, `D`, `E`, `F`, `-` (U+002D), U+00AD와 U+2010의 horizontal advance는 각 1000 units다. Ahem 10px에서 visible ASCII 및 삽입 하이픈 advance는 10px이다. U+00AD 자체는 비분기 상태에서 표시 폭을 더하지 않는다.

따라서 `AB&shy;CD`는 `none`에서 4글자 40px이고, `manual`은 `AB` 뒤에 10px U+2010을 표시하는 첫 segment 30px과 `CD` 20px 중 큰 값인 30px이 min-content다. max-content는 양쪽 모두 40px이다. 고정 30px에서는 none이 한 줄 40px로 넘치고, manual은 30px/20px 두 줄이다. `AB-CD`는 visible hyphen 이후 break가 남으므로 두 mode 모두 min-content 30px다. `ABCD`에는 분기점이 없어 양쪽 모두 40px다.

## Chrome observation

각 장면의 `grid-template-columns`, `grid-template-rows`, width/height, scrollWidth와 text Range line count를 Agent Browser DOM에서 읽었다. Ahem font face는 `loaded`, `document.fonts.check('10px Ahem')`은 true였다.

| 고정 입력 | `none` | `manual` |
| --- | --- | --- |
| `AB&shy;CD`, min-content | column 40px, row 10px, 1 line | column 30px, row 20px, 2 lines |
| `AB&shy;CD`, max-content | column 40px, row 10px, 1 line | column 40px, row 10px, 1 line |
| `AB&shy;CD`, fixed 30px | track 30px, scrollWidth 40px, row 10px, 1 line | track 30px, scrollWidth 30px, row 20px, 2 lines |
| declaration 없음 (initial `manual`), fixed 30px | — | track 30px, scrollWidth 30px, row 20px, 2 lines |
| `AB-CD`, min-content 및 fixed 30px | column 30px / row 20px / 2 lines | 같은 값 |
| `ABCD`, min-content | column 40px / row 10px / 1 line | 같은 값 |
| `AB&shy;CD&shy;EF`, min-content | column 60px / row 10px / 1 line | column 30px / row 30px / 3 lines |
| `AB&shy;CD&shy;EF`, fixed 30px | scrollWidth 60px / row 10px / 1 line | track 30px / row 30px / 3 lines |
| 조상 선언 상속, `AB&shy;CD`, fixed 30px | scrollWidth 40px / row 10px / 1 line | track 30px / row 20px / 2 lines |

## 해석 한계

- 앞서 사용한 임시 DOM의 첫 측정은 부모 Grid label stretch 때문에 단일 줄 row가 16px이 되어 폐기했다. 최종 고정 HTML은 label을 block 흐름으로 분리했고, Chrome을 다시 열어 19개 장면과 화면 캡처를 갱신했다.
- 이 비교는 fixed Ahem/ASCII/U+00AD 예제에 대한 geometry 기준이다. 자동 사전, 임의 언어·font·script, glyph pixel painting, 모바일 runtime, 공개 CSS 지원을 검증하지 않았다. 제품 구현은 시작하지 않았다.

## 고정 자원

- fixture HTML SHA-256: `6f08960a394a13278f5d5a05647cbbb7fea728c2fd614b4daa6fee2b693d979d`
- Agent Browser PNG SHA-256: `3740eef02a70a5deae8263258b7807b927a7bd58ea1b40f2d35c57e3ce103e75`
- pinned Ahem TTF SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
