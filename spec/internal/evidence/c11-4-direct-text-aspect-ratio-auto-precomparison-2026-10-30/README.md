# C11.4 direct-text `auto <ratio>` 사전 비교

**기준 시점:** 2026-10-30, 구현 계획 PR 전

**기준 브라우저:** Agent Browser / HeadlessChrome 156.0.0.0, macOS, viewport 1280×633 CSS px, DPR 1

**기준 문서:** [CSS Box Sizing Level 4 §4.1](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio) Working Draft · [CSS Grid Layout 2025 CRD §6.2, §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)

이 관측은 `auto <ratio>`와 숫자 ratio의 box-sizing 차이를 padding·border가 있는 non-replaced direct-text Grid item에서 고정한다. W3C Level 4 문서는 2026-09-04 Working Draft이며 스스로 탐색 초안이라고 밝힌다. 아래 값은 Chrome 156 관측과 수식에 따른 독립 기대값이지, WPT 결과·Spinon 동작 또는 전체 CSS 지원 선언이 아니다.

## 입력과 브라우저 상태

[고정 fixture](../../../../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-auto-precomparison.html)는 Ahem 10px/10px direct text와 두 개의 max-content columns를 사용한다. `AAAA`의 Ahem content advance는 40px이고 `A`는 10px이다. 각 item은 `padding: 2px 4px`, 1px solid border, `box-sizing: border-box`를 갖는다. 비교 대상은 `aspect-ratio: auto 3 / 2`, `aspect-ratio: 3 / 2`, `aspect-ratio: auto`다. 추가 content-box control은 `auto 3 / 2`의 ratio basis가 `box-sizing`과 무관하게 content box인지 확인한다.

브라우저는 `HeadlessChrome/156.0.0.0`, viewport 1280×633, DPR 1이며 `document.fonts.ready` 이후 `document.fonts.check('10px Ahem')`가 true였다. Fixture와 Ahem asset은 HTTP 200을 반환했다. 결과 원장은 [`chrome-observation.json`](./chrome-observation.json), 전체 화면은 [`chrome-156-auto-ratio.png`](./chrome-156-auto-ratio.png)이다. 캡처 뒤 Agent Browser 세션과 임시 HTTP server를 종료했다.

## 독립 계산과 관측

수평 padding+border는 `10px`, 수직 padding+border는 `6px`이다. 따라서 auto-row의 `AAAA` content width 40px에 `3/2`를 적용하면 content height `40 / 1.5 = 26⅔px`, border-box height `32⅔px`다. 숫자 ratio는 `box-sizing:border-box`의 전체 box를 사용해 `50 / 1.5 = 33⅓px`를 예상한다. Chrome frame은 1/64 CSS px layout quantum에 맞춰 양자화돼 있다.

| 장면 | 독립 계산 | Chrome 156 관측 |
|---|---|---|
| auto row, `auto 3/2`, border-box | columns `[20, 50]`; content `40 × 26⅔`; item/root height `32⅔` | columns `20px 50px`; row `32.65625px`; item `50 × 32.65625px` |
| auto row, numeric `3/2`, border-box | columns `[20, 50]`; border-box height `50 / 1.5 = 33⅓` | columns `20px 50px`; row `33.328125px`; item `50 × 33.328125px` |
| auto row, `auto`, border-box | text line-height `10` + vertical edge `6` | row `16px`; item `50 × 16px` |
| auto row, `auto 3/2`, content-box | same content ratio calculation as first row | row `32.65625px`; item `50 × 32.65625px` |
| definite 60px row, `auto 3/2`, border-box | content height `60 − 6 = 54`; content width `54 × 1.5 = 81`; outer width `81 + 10 = 91` | columns `91px 20px`; ratio item `91 × 60px` |
| definite 60px row, numeric `3/2`, border-box | border-box width `60 × 1.5 = 90` | columns `90px 20px`; ratio item `90 × 60px` |
| definite 60px row, `auto`, border-box | no ratio transfer; `AAAA` max-content outer width `40 + 10 = 50` | columns `50px 20px`; item `50 × 60px` |

`auto <ratio>`는 non-replaced item에서도 content-box dimensions로 계산한다. 따라서 border-box를 지정해도 숫자 ratio처럼 padding/border를 ratio 식에 넣으면 안 된다. Definite row 장면은 `91px` 대 `90px`의 max-content track을 분리하며, auto row 장면은 `32⅔px` 대 `33⅓px`의 row contribution을 분리한다. Content-box control은 auto ratio 계산의 box basis가 `box-sizing`에 종속되지 않는다는 별도 대조군이다.

## 범위와 재현 자료

- 입력 fixture SHA-256: `e9da6167582959356f242d9048eb9cdd998fa45d0014c7f62f35d1ee89a958da`
- Ahem font SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Chrome observation SHA-256: `dda28a2321376c27c6b4e6e9ea5f63b0bf8bb8375e3b389a242314409b1deb36`
- Screenshot SHA-256: `a1a424a384cc77755bb8245cc307c2c75413847ca46ee1ba0f68c17f33cc3a22`

This is a fixed browser observation only. It does not verify Spinon, iOS/Android, text glyph paint, general CSS admission, full WPT, replaced items, min/max transfer, percentage edges, multiline text, spans or full C11.4. The local browser/server session was closed after capture.
