# C11.4 six-fit repeated-text row-span `min-height:auto` 사전 비교

## 목적과 경계

이 비교는 기본 parsed Root HTML repeated-text row-span에서 여섯 `fit-content()` implicit track에 놓인 spanning text item의 automatic minimum 차이를 관찰한다. Agent Browser 0.31.1이 구동한 HeadlessChrome 157의 CSS 결과이며 Spinon runtime, 앱, simulator, 기기 화면의 근거는 아니다.

C11.4의 직전 제한 구현은 five-fit row-span [PR #1360](https://github.com/ohah/spinon/pull/1360)으로 병합됐다. 별도 six-fit auto-placement [PR #1354](https://github.com/ohah/spinon/pull/1354)는 아홉 direct child와 non-spanning auto-placement shape만 다룬다. 이 비교는 row-span이라는 별도 input shape를 고정하며 두 profile을 합치지 않는다.

규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. Agent Browser에서 확인한 §6.6은 non-scrollable item이 해당 축에서 `auto` minimum sizing function인 track을 span할 때 automatic minimum의 조건을 정의하고, 여러 track을 span할 경우 flexible track이 없어야 한다고 규정한다. §11.2에서 `fit-content()`의 minimum sizing function은 `auto`다. 따라서 이 고정 profile의 모든 spanning item은 eligible하며, `fit-content()` cap은 content-based minimum을 fixed maximum track처럼 자르지 않는다.

## 입력 및 matrix

- Grid content box는 definite `40×220px`, column은 `40px`, explicit row는 빈 `10px`, 여섯 implicit `grid-auto-rows`는 모두 `fit-content()`다. Row/column gap은 0, direction은 LTR, auto-flow는 sparse row, overflow는 visible이다.
- Cap multiset은 `0px`, `10px`, `20%`, `30px`, `40px`, `50px`이며 720개 authored order를 모두 사용했다. Definite `220px` block basis에서 `20%`는 `44px`다.
- 두 개의 fixed row-span 가족을 비교했다: span-2 item 다섯 개가 implicit 여섯 행을 겹쳐 덮는 topology, 그리고 span-2 두 개와 span-3 두 개가 같은 여섯 행을 덮는 topology. 각 family의 DOM order는 forward/reverse다.
- 두 topology × DOM order 2개 × `align-content:start|stretch` × 모든 text item `min-height:0|auto`를 교차했다. 총 11,520 scenes와 5,760 baseline/candidate pair keys다. 각 scene은 explicit row 1개와 implicit row 6개를 관측했다.
- 고정 payload, 10px monospace/10px line-height, text item 및 payload frames, 각 span의 used row 합, line boxes, authored/computed cap order를 기록했다. `0.01 CSS px`를 geometry tolerance로 정했다.

## Chrome 157 결과

독립 검증은 [verification.json](./verification.json)에 있다.

| 관측 | 결과 |
| --- | ---: |
| scenes / paired keys | 11,520 / 5,760 |
| unique six-cap order | 720 / 720 |
| baseline `min-height:0` payload-floor shortfall | 4,800 occurrences |
| candidate `min-height:auto` payload-floor shortfall | 0 occurrences |
| baseline / candidate Grid-end overflow | 0 / 0 |
| baseline/candidate geometry가 달랐던 paired keys | 5,760 / 5,760 |
| span item frame과 covered-row 합의 최대 차이 | 0.00005 CSS px |
| DOM order 반전 geometry 차이 | 0 CSS px |
| `align-content:start`와 `stretch` geometry 차이 | 0 CSS px |

전체 capture는 25,920 baseline text-item occurrences와 25,920 candidate occurrences를 포함한다. 위 4,800건은 고정 Chrome 입력에서 `min-height:0` item frame이 자신의 text payload 높이보다 작았던 baseline occurrences다. `auto` 후보의 0건은 이 fixture matrix의 관측값이며 구현 결과나 전체 CSS 적합성 증거가 아니다.

## 재현 및 파일

```sh
mise exec -- node spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/capture-chrome.mjs
mise exec -- python3 spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/verify-chrome-matrix.py
```

- Oracle 입력: [fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html)
- Agent Browser capture: [capture-chrome.mjs](./capture-chrome.mjs)
- Compressed raw observations: [chrome157-observation.ndjson.gz](./chrome157-observation.ndjson.gz)
- Independent verifier: [verify-chrome-matrix.py](./verify-chrome-matrix.py)
- Representative Browser capture: [chrome157-six-fit-row-span-sample.png](./chrome157-six-fit-row-span-sample.png)
- SHA-256 manifest: [SHA256SUMS](./SHA256SUMS)

브라우저 CLI는 `agent-browser 0.31.1`, user agent는 HeadlessChrome `157.0.0.0`, `navigator.platform`은 `MacIntel`, DPR 1, viewport는 `1280×900`이다. 화면 캡처 직전에만 Grid를 2.5배 시각 확대했고, 관측 matrix는 확대 전에 수집했다. capture 스크립트는 전용 Agent Browser session을 종료한다. Terminal Browser와 앱/simulator를 실행하지 않았다.

## 제한

이 결과는 고정 text/cap/topology의 browser CSS comparison이다. Spinon RuntimeSession·Android/iOS geometry, pixel parity, 실기기, GPU hardware, WPT, 임의 track list와 전체 C11.4/C11 완료를 입증하지 않는다. 공식 완료 `78/214`, C11.4/C11 체크, 내부 계약 숫자 `0.1.0`은 바뀌지 않는다.
