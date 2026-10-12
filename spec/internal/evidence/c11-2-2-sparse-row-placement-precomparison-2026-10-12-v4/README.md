# C11.2.2 Chrome 154 사전 비교 v4

**범위:** C11.2.2 구현 전 Chromium 기준을 보강했다. 이번 추가 scene은 auto-placement cursor의 column이 이전 cursor column보다 작을 때 row를 증가시키는 sparse 동작을 고정한다. Spinon runtime, Android/iOS, GPU surface, 성능 또는 WPT 적합성 실행은 아니다.

## 재현 정보

- 고정 reference: [`c11-grid-auto-placement-precomparison-v4.json`](../../../../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json)
- versioned fixture: [`grid-auto-placement-v3.html`](../../../../tests/fixtures/css/c11/grid-auto-placement-v3.html) · [`grid-auto-placement-inventory-v3.json`](../../../../tests/fixtures/css/c11/grid-auto-placement-inventory-v3.json)
- capture tool: [`capture-c11-grid-auto-placement-precomparison-v4.mjs`](../../../../tools/css-reference/capture-c11-grid-auto-placement-precomparison-v4.mjs)
- 실행: `mise exec -- node tools/css-reference/capture-c11-grid-auto-placement-precomparison-v4.mjs --output=tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json`
- 테스트: `mise exec -- bun run test:css-c11-2-2-auto-placement-v4` — 4개 검사 통과
- run ID: `c1122-chrome-20261012T114657567Z-1957d602`
- Chrome `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- DPR 1·2에서 16 scene·69 node를 수집했고 DPR 1 화면 16개를 저장했다. 모든 CSS computed 값·track 수·frame은 두 DPR에서 같았다.

## 새 cursor row-wrap scene

3개 explicit column(각 40 CSS px, column gap 4 CSS px), 1개 explicit row(20 CSS px), `grid-auto-rows:20px`를 둔다. DOM 순서대로 첫 item은 column 2에, 다음 item은 column 1에 고정한다. 첫 item을 놓은 뒤 cursor column은 두 번째 column보다 앞서 있으므로 두 번째 item은 앞선 row의 빈 첫 column을 다시 채우지 않고 다음 row의 첫 column으로 이동한다.

| Node | Chrome frame (scene-root 기준) | Grid 콘텐츠 기준 |
| --- | --- | --- |
| `c1122-column-wrap-col-two` | `(64, 20, 18, 18)` | `(44, 0, 18, 18)` |
| `c1122-column-wrap-col-one` | `(20, 44, 18, 18)` | `(0, 24, 18, 18)` |

computed `grid-template-rows`는 `20px 20px`이며, explicit row 1개 뒤에 positive implicit row 1개가 생긴다. 새 장면의 [화면](../c11-2-2-sparse-row-placement-precomparison-2026-10-12/c1122-chrome-20261012T114657567Z-1957d602/column-cursor-wrap-dpr1.png)을 확인할 수 있다.

이 기준은 계획의 cursor 문장을 구체화한다. definite-column/auto-row item은 cursor column을 해당 item의 definite column으로 바꾸고, 새 column이 기존 cursor column보다 작으면 row를 먼저 증가시킨 뒤 빈 cell을 찾는다. 이는 CSS Grid §8.5의 row-auto-placement 단계와 일치한다([W3C CSS Grid Layout Module Level 1 §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)).

## 기존 기준 보존과 해석 경계

이전 15-scene·66-node reference v3와 해당 입력 hash는 보존했다. v4는 별도 HTML, inventory, capture tool 및 reference 경로를 사용한다. 고정 WPT 후보 2개의 Chrome runner 결과는 이전 pinned WPT 실행 근거를 참조하며, 이번 신규 scene을 WPT로 실행한 결과가 아니다. Chrome oracle 통과는 Spinon의 동작이나 적합성 증거가 아니다.
