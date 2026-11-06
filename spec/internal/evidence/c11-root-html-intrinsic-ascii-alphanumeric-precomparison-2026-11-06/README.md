# C11.4 Root HTML ASCII 영숫자 단어 intrinsic sizing 사전 비교

## 목적과 경계

PR [#1208](https://github.com/ohah/spinon/pull/1208)은 기본 parsed Root HTML의 `white-space: normal` direct-text intrinsic implicit-row 경로에서 ASCII alphabetic token을 허용한다. 숫자와 영문·숫자 혼합 token은 아직 거부한다. 이 자료는 제한 후보 `[A-Za-z0-9]+`의 Grid track·row·줄 수를 Chrome observation과 독립 계산으로 고정한다. 구현이나 C11.4 완료를 증명하지 않는다.

## 입력과 실행

- Fixture: [`root-html-grid-intrinsic-ascii-alphanumeric-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-ascii-alphanumeric-precomparison.html)
- Browser: Agent Browser 0.31.1, HeadlessChrome 157, viewport `1280×633` CSS px, DPR 1.
- 캡처: [`capture.js`](./capture.js), [Chrome observation](./chrome-observation.json), [Agent Browser 화면](./chrome-agent-browser.png).
- 독립 계산: [`derive_expected.py`](./derive_expected.py)는 source token과 10px generic monospace Canvas advance를 1/64 CSS px로 올림 처리하고, CSS Grid track-sizing 모델과 normal-whitespace token 경계로 예상 track·row·line 수를 계산한다. Chrome의 used track/row는 계산 입력이 아니라 결과 비교에만 쓴다.
- 결과: [`expected.json`](./expected.json)의 10개 장면 모두 계산한 used track·row·line 수가 Chrome observation과 일치한다.
- 계획 PR의 실제 diff는 [30개 별도 관점](./plan-pr-diff-review-30.md)으로 확인했다.

## 독립 기대값

| 장면 | column | 예상 track | 예상 rows | 긴 text 줄 수 |
| --- | --- | ---: | --- | ---: |
| 기존 영문 control `A A A A A A` | `min-content` | 6.03125px | `[10,60,10,10]px` | 6 |
| 숫자 token `12 3 4` | `min-content` | 12.046875px | `[10,30,10,10]px` | 3 |
| 혼합 token `A1 BB2 C33` | `min-content` | 18.0625px | `[10,30,10,10]px` | 3 |
| 숫자 시작 token `1A 22B 33C` | `min-content` | 18.0625px | `[10,30,10,10]px` | 3 |
| `A1 BB2 C33`, reverse item order | `min-content` | 18.0625px | `[10,10,30,10]px` | 3 |
| unbreakable 숫자 `12345678` | `auto` | 48.171875px | `[10,10,10,10]px` | 1 |
| unbreakable 혼합 `A1B2C3D4` | `auto` | 48.171875px | `[10,10,10,10]px` | 1 |
| 혼합 token `A1 BB2 C33` | `auto` | 40px | `[10,20,10,10]px` | 2 |
| 숫자 시작 token `1A 22B 33C` | `auto` | 40px | `[10,20,10,10]px` | 2 |
| `A1 BB2 C33`, reverse item order | `auto` | 40px | `[10,10,20,10]px` | 2 |

`min-content`는 collapsible whitespace로 분리된 token 중 가장 넓은 token advance다. definite 40px container의 `auto` column은 `max(min-content, min(40px, max-content))`로 계산한다. CSS Text의 `normal` wrapping은 이 제한 token profile에서 token 사이에만 줄바꿈 기회를 둔다.

## 제한과 미확인 범위

- 후보 token grammar는 비어 있지 않은 ASCII 영문·숫자 조합 `[A-Za-z0-9]+`이며, token 사이에는 SPACE/TAB/LF/CR만 허용한다. 입력 source 및 Text node identity는 그대로 보존해야 한다.
- Unicode, punctuation, NBSP, vertical tab, form feed, empty/whitespace-only text, 다른 font/style/owner, 일반 CSS author admission, WPT 및 전체 C11.4는 이 비교 범위 밖이다.
- 이 결과는 Chrome 157 기준이지 Android/iOS runtime, glyph paint, 실제 기기, WPT 적합성 또는 전체 C11 완료 근거가 아니다.

## 입력 SHA-256

저장소 루트에서 `sha256sum -c spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/SHA256SUMS`로 fixture와 캡처·계산 자료를 확인한다.
