# C11.4 Root HTML ASCII 단어 intrinsic sizing 사전 비교

## 목적

PR #1205는 `white-space: normal` 직행 text 입력에서 단독 ASCII `A` token만 허용한다. 이 비교는 같은 root HTML Grid에서 ASCII 알파벳 단어를 허용하면 `min-content` track과 text 줄 수가 어떻게 달라지는지 고정한다. Spinon 구현 결과가 아니라 브라우저 기준과 독립적인 CSS Grid 계산이다.

## 고정 입력과 실행

- Fixture: [`root-html-grid-intrinsic-letter-words-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-precomparison.html)
- Browser: Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, viewport `1280×633` CSS px, DPR 1, macOS ARM.
- Capture source: [`capture.js`](./capture.js). Root, item, used-track, used-row, text-line rect와 10px `monospace` canvas advance를 기록한다.
- Browser observation: [`chrome-observation.json`](./chrome-observation.json). 전체 page screenshot은 [`chrome-156-agent-browser.png`](./chrome-156-agent-browser.png)이다.
- Independent calculation: [`derive_expected.py`](./derive_expected.py)가 source token, monospace word/space advance, CSS Grid 2025 CRD §11.4–11.7의 min-/max-content contribution, CSS Text 4 §4.1·§4.3의 normal whitespace line break를 사용해 track·row·line 기대값을 산출한다. Browser의 final used track/row 값을 계산 입력으로 사용하지 않는다.
- 결과: [`expected.json`](./expected.json). 독립 기대값과 Chrome의 used track/row·line count가 10/10 장면에서 일치한다.

## 독립 기대값

Chrome 156의 generic monospace 10px glyph advance는 CSS layout quantization 뒤 `6.03125px`다. `min-content` column은 단어 중 가장 긴 unbreakable ASCII letter run의 advance로 계산한다. 각 `auto` scene의 max-content contribution은 40px 이상이다. 그 definite 40px container에서 `auto` track은 `max(min-content, min(40px, max-content))`이며, 긴 단어의 automatic minimum이 더 크면 content width 바깥으로 확장한다.

| 장면 | 열 | 기대 Columns | 기대 Rows | 긴 text lines |
| --- | --- | ---: | --- | ---: |
| single-letter control | min-content | 6.03125px | `[10,60,10,10]px` | 6 |
| `AA A A A` | min-content | 12.046875px | `[10,40,10,10]px` | 4 |
| `AB BBB A` | min-content | 18.0625px | `[10,30,10,10]px` | 3 |
| `aa bb ccc` | min-content | 18.0625px | `[10,30,10,10]px` | 3 |
| `AB BBB A`, reverse item order | min-content | 18.0625px | `[10,10,30,10]px` | 3 |
| `AA A A A` | auto | 40px | `[10,20,10,10]px` | 2 |
| `AAAAAAAA` | auto | 48.171875px | `[10,10,10,10]px` | 1 |
| `AB BBB A` | auto | 40px | `[10,20,10,10]px` | 2 |
| `aa bb ccc` | auto | 40px | `[10,20,10,10]px` | 2 |
| `AB BBB A`, reverse item order | auto | 40px | `[10,10,20,10]px` | 2 |

## 범위와 한계

이 비교는 `lang="en"`, generic monospace 10px, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`인 ASCII alphabetic words만 다룬다. Unicode, 숫자, punctuation, NBSP 및 다른 font family는 이 fixture의 판정 범위가 아니다. Chrome 관측은 WPT pass, Spinon runtime, Android/iOS font parity 또는 전체 C11.4 적합성을 증명하지 않는다.

## 입력 hash

```text
213531a071f797eb19f5a63c465b05c61ee8d9589ba6f38974290eb56eb88db6  tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-precomparison.html
c4cd13e32141530d60fcc85a1a745dfeb2c232dc018a96adb7ff07313dcf0f29  capture.js
e4f395ad9ec352722f83eba24f3a3b8c1e362b48ff92b32ba2e280f4e9125047  derive_expected.py
9ebd819e4492652ec88e6ebd67aa15fe38773105636f16fa4271cf7bc081409e  chrome-observation.json
d4d6d42e5d128fdbbcfcfaacbc13621b51f0601ac1000be3abc89f80a442c2d1  expected.json
3b37dc06c0b0e031a2d0fc1e6522c5e98765781531d817e9ec1bfe653441bd06  chrome-156-agent-browser.png
```
