# C11.1 구현 전 Chromium 비교 기준

**상태:** 기준 고정 완료 · Spinon Grid 미구현 · WPT 적합성 결과 아님  
**계획:** [C11 Grid 구현 계획](../../../../plan/c11-css-grid.md) · [내부 계약 0059](../../0059-c11-grid-layout.md)

## 고정 기준

- 브라우저: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`.
- 입력: [`grid-layout.html`](../../../../tests/fixtures/css/c11/grid-layout.html), [`grid-layout-inventory.json`](../../../../tests/fixtures/css/c11/grid-layout-inventory.json). viewport 360×800 CSS px, `horizontal-tb`/LTR, DPR 1·2.
- 장면: 10개, 총 40개 고유 node. 고정·percentage·`fr`·혼합 트랙, gap, row auto-placement, 빈/단일 item, Block·Flex·Grid 중첩을 포함한다.
- 저장 결과: [고정 reference v2](../../../../tests/fixtures/css/references/c11-grid-precomparison-v2.json), 실행 ID `c11-chrome-20261011T141747906Z-c5bc3779`; scene별 PNG는 [실행 폴더](./c11-chrome-20261011T141747906Z-c5bc3779/)에 보관했다.
- WPT 후보 원문은 commit `c999c58338ee1d223df5ad62e48be1202ced0d37`에서 다시 받아 [경로별 SHA-256](./wpt-source-hashes.log)을 확인했다. 후보 고정만 했으며 이 작업에서 WPT를 Chrome이나 Spinon에서 실행하지 않았다.

## 관측된 기준값

- 고정 트랙: columns `80px 100px 120px`, rows `50px 70px`, gaps 8px/6px. 두 번째 행의 item은 y=56px에서 시작한다.
- definite percentage: 320px content width에서 `25% 75%`는 `80px 240px`이며 두 번째 item x=80px다.
- fractional: 320px width와 12px gap의 `1fr 2fr`는 Chrome computed track `102.656px 205.344px`; 두 번째 item x=114.65625px다.
- 혼합 트랙: `80px 25% 1fr`, 360px width, 12px gap은 `80px 90px 166px`이며 item x는 0, 92, 194px다.
- 기본 row auto-placement의 5개 item 위치는 `(0,0)`, `(110,0)`, `(0,50)`, `(110,50)`, `(0,100)` CSS px다.
- nested scene에서 Block 안 Grid, Flex 안 Grid, Grid 안 Flex의 parent relation과 각 node frame을 모두 고정했다.

각 DPR에서 computed property·parent relation·NodeId 순서와 frame이 정확히 같았다. 최대 허용 geometry 오차는 향후 Spinon 비교를 위해 `0.5 CSS px`로 고정했다. 이 허용치는 이번 Chrome 관측의 차이를 뜻하지 않는다.

## 검증 재현

- 기준 생성: `mise exec -- node tools/css-reference/capture-c11-grid-precomparison.mjs --output=tests/fixtures/css/references/c11-grid-precomparison-v3.json`
- 기준 무결성·geometry·PNG·negative control 검사: `mise exec -- bun run test:css-c11-grid-precomparison`
- 현재 reference v2는 기존 v1을 덮어쓰지 않고 별도 실행으로 저장했다. capture 도구는 versioned output만 허용하고 기존 파일이 있으면 실패한다.
- 자동 검사는 입력·도구·Chrome 실행 파일 hash, 10 scene·40 node tree, pinned computed tracks·placement·nested frame, DPR parity, PNG signature·크기·hash, 누락/중복 node·비유한 geometry·DPR 변조 거부를 확인한다.

## 해석 경계

이 결과는 Chrome 154에서 고정 fixture의 기준 관측값을 만든 것이다. Taffy 또는 Spinon의 결과, 공식 WPT pass, CSS 지원 선언, Android/iOS runtime 실행이나 GPU pixel 비교가 아니다. WPT 자동 후보 중 C11.2에 속한 implicit track 후보는 이후 단계의 입력 inventory로만 둔다.
