# C11.4 · repeated-text row-span mixed automatic minimum 사전 비교

## 목적과 경계

PR #1365의 six-fit repeated-text row-span profile은 모든 span text item이 한 scene 안에서 전부 `min-height:0` 또는 전부 `auto`인 경우만 다룬다. 이번 비교는 그 사이에 있는 per-item mixed assignment가 CSS Grid에서 어떻게 계산되는지 확인해 다음 구현 계획의 기준을 고정한다.

이 자료는 Agent Browser 0.31.1이 구동한 HeadlessChrome 157의 CSS observation이다. Spinon RuntimeSession, Android/iOS 앱, simulator, 실기기 또는 제품 지원을 입증하지 않는다. 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. `fit-content()` track의 minimum sizing function은 `auto`이며, 각 non-scrollable spanning item의 automatic minimum을 개별 판정한다.

## 고정 입력과 matrix

- definite `40×220px` Grid, `40px` column, 빈 explicit `10px` row, zero gap, LTR sparse row flow, visible overflow, `align-content:start|stretch`를 고정했다.
- 여섯 implicit `fit-content()` track cap multiset은 `[0px, 10px, 20%, 30px, 40px, 50px]`이고 720개 authored order를 모두 검사했다. 220px block-size basis에서 `20%=44px`다.
- span2-six topology에는 A–E 다섯 span item이 있고 각 item은 두 implicit row를 점유한다. span2/span3-six topology에는 A/B span 2, C/D span 3이 있다. 각 topology의 DOM source order는 forward/reverse다.
- 모든 item별 `min-height:0|auto` bitmask를 검사했다: 32개(A–E), 16개(A–D). 각 mask는 모든 cap order·alignment·DOM order와 교차한다.
- 총 138,240 scene / 34,560 비교 group이다. 각 scene은 seven used rows, computed minimum, item/payload frames와 text line boxes를 보존한다.

## 결과

| 관측 | 결과 |
| --- | ---: |
| scene / comparison group | 138,240 / 34,560 |
| six-cap order / per-item masks | 720/720 / A 32/32, B 16/16 |
| 기존 uniform `0px`·`auto` controls와 일치 | 11,520/11,520 |
| `auto` item payload-floor violations | 0 |
| `0px` item payload-floor shortfalls | A 34,944, B 12,672 |
| Grid-end overflow | 두 topology 모두 0 |
| source order·alignment 최대 geometry 차이 | 0 CSS px |
| span frame 대 covered-row 합 최대 차이 | 0.0001 CSS px |
| mixed mask가 all-zero와 달랐던 cap orders | mask별 540–720 / 720 |

검증기는 각 scene의 mask-to-ID mapping, authored track list, computed style, row count, explicit control, placement와 covered-row 합을 독립 검사한다. Auto item의 payload floor 위반은 모든 mask에서 0이다. Zero minimum의 payload-floor shortfall은 자동 최소값 없이 visible text overflow가 발생하는 expected control observation으로 별도 보존한다.

## 재현과 산출물

```sh
mise exec -- node spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12/capture-chrome.mjs
mise exec -- python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-mixed-auto-minimum-precomparison-2026-11-12/verify-chrome-matrix.py
```

- Browser input: [fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-mixed-auto-minimum-precomparison.html)
- Agent Browser capture: [capture-chrome.mjs](./capture-chrome.mjs)
- Raw observations: [compressed NDJSON](./chrome157-mixed-minimum-observation.ndjson.gz)
- Independent verifier: [verify-chrome-matrix.py](./verify-chrome-matrix.py)
- Browser screenshot: [mixed A/C automatic minimum sample](./chrome157-mixed-minimum-sample.png)
- Machine summary: [verification.json](./verification.json)
- File hashes: [SHA256SUMS](./SHA256SUMS)

Capture used `agent-browser 0.31.1`, HeadlessChrome `157.0.0.0`, `MacIntel`, DPR 1 and viewport `1280×900`. The screenshot was captured after measuring and then visually scaling the Grid by 2.5. The dedicated Agent Browser session closed when capture completed; no Terminal Browser or app/simulator ran for this oracle.

## 제한

비교는 고정 CSS 입력과 browser 계산에 한정된다. Spinon 구현, Android/iOS layout 동등성, pixel 출력, 실기기, hardware GPU, WPT, 임의 Grid minimum 조합 또는 전체 C11.4/C11 지원을 증명하지 않는다. 공식 `78/214`, C11.4/C11 checkbox와 내부 계약 `0.1.0`은 그대로다.
