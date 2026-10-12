# C11.2.2 행 우선 자동 배치 Chromium 사전 비교

## 기준 식별자

- 기준: `C11-2-2-grid-auto-placement-precomparison-v2`
- Chrome: `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 실행 파일 SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- 실행 환경: macOS 25.5.0, arm64, Node `v26.7.0`
- viewport: `360 × 800 CSS px`, DPR `1`, `2`; `en-US`, UTC, light, LTR, `horizontal-tb`, coarse pointer
- capture run: `c1122-chrome-20261012T063150043Z-c60e5138`
- reference: [`c11-grid-auto-placement-precomparison-v3.json`](../../../../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json)
- fixture: [`grid-auto-placement.html`](../../../../tests/fixtures/css/c11/grid-auto-placement.html) · [`grid-auto-placement-inventory.json`](../../../../tests/fixtures/css/c11/grid-auto-placement-inventory.json)
- capture 도구: [`capture-c11-grid-auto-placement-precomparison.mjs`](../../../../tools/css-reference/capture-c11-grid-auto-placement-precomparison.mjs) · [`chromium-session.mjs`](../../../../tools/css-reference/chromium-session.mjs)
- 지원 profile 계획: [`C11.2.2 계획`](../../../../plan/c11-2-2-sparse-row-placement.md)

## 실행 결과

15개 scene·66개 node를 Chrome에서 DPR 1과 2로 각각 수집했다. scene마다 DOM preorder, NodeId parent, authored style, Grid computed property, explicit/computed/implicit track 수와 CSS pixel frame을 저장했다. DPR 1 screenshot 15개를 함께 보존하고 SHA-256 및 `360 × 800` PNG 크기를 검사했다. 두 DPR의 CSS computed 값·track 수·CSS px frame은 전 scene에서 같았다.

지원 후보 10개는 definite-first mixed placement, all-auto 채우기, 양의 implicit row, row-locked·column-locked item, span occupancy, sparse hole, gap·implicit track sizing, hidden child, nested container를 각각 관찰한다. 나머지 5개는 implicit column, implicit `auto` row, 반복 `grid-auto-rows`, dense flow, column flow가 Chrome에서 어떻게 보이는지 기록한 oracle-only 입력이다. oracle-only 결과는 Spinon 구현이 해당 입력을 거부하는지 확인한 결과가 아니다.

| 확인 장면 | Chrome에서 기록한 결과 |
| --- | --- |
| definite item이 auto item보다 DOM 뒤에 있는 경우 | definite item이 첫 cell, 앞선 auto item이 두 번째 cell에 놓임 |
| 양의 implicit row | `grid-template-rows: 40px 25px`, implicit row 1개 |
| definite row/auto column | 두 item이 같은 explicit row의 첫·두 번째 column을 사용 |
| definite column/auto row | 두 item이 같은 explicit column의 첫·두 번째 row를 사용 |
| sparse cursor | row 1의 남은 cell을 건너뛰고 span item 및 다음 item을 row 2에 둠 |
| implicit column negative | 명시 2개 외 computed column track 2개가 나타남 |
| column-flow negative | 두 explicit column 뒤에 computed implicit column 1개가 나타남 |
| hidden item | `display:none` item의 frame이 배치에 참여하지 않고 보이는 item이 첫 두 cell 사용 |

Chrome CSSOM은 authored `grid-auto-flow: row dense`의 computed property를 `dense`로 직렬화했다. 자동 기본값 `row`를 생략한 CSSOM 표현이다. typed cascade에서는 문자열을 그대로 비교하기보다 `row + dense` 의미를 판별해야 한다. 테스트는 이 관측값과 dense 장면의 hole backfill 결과를 함께 고정한다.

## WPT 후보 원문과 Chrome 기준 실행

고정 revision [`c999c58338ee1d223df5ad62e48be1202ced0d37`](https://github.com/web-platform-tests/wpt/tree/c999c58338ee1d223df5ad62e48be1202ced0d37)의 원문 SHA-256을 확인하고 WPT runner에서 Chrome `154.0.8037.98`로 각각 실행했다. ChromeDriver는 `154.0.8037.92`, 환경은 macOS `26.5.1` arm64다. 두 결과는 Chrome 기준 동작을 확인한 것이며 Spinon WPT 적합성 결과가 아니다.

| 파일 | SHA-256 | 단계 |
| --- | --- | --- |
| [`grid-layout-grid-span.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/placement/grid-layout-grid-span.html) | `1be809e54f1b7398112923acd60ef423735934b528ff42c1a8f3b326adbebef7` | Chrome WPT runner PASS · [결과 JSON](grid-span-chrome-rerun-wptreport.json) · [실행 로그](grid-span-chrome-rerun.log) |
| [`grid-negative-line-placement-001.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/placement/grid-negative-line-placement-001.html) | `e5d17120d87170282d2522ec483d751c6f7db9ac23ec09d2a540cce305d81d31` | Chrome WPT runner PASS · 7/7 subtests · [결과 JSON](grid-negative-lines-chrome-resources-rerun-wptreport.json) · [실행 로그](grid-negative-lines-chrome-resources-rerun.log) |

첫 실행에서 sparse checkout에 필요한 파일이 빠져 있었다. `grid-layout-grid-span.html`은 linked reference 누락으로 처음 실패했고, negative-line test는 `resources/testharness.js`, `testharnessreport.js`, `check-layout-th.js`가 없어 timeout 났다. [첫 실행 로그](wpt-chrome-154-run-2026-10-12.log)에서 시작 오류를 확인할 수 있다. 필요한 `css/css-grid/reference`와 `resources`를 checkout한 뒤 두 테스트를 다시 실행해 각각 PASS와 7/7 PASS를 확인했다. 초기 실패는 CSS 결과가 아니라 WPT checkout 구성 오류다.

## 완료 범위와 남은 검증

이 산출물은 pinned Chrome의 oracle·fixture 고정과 고정 WPT 후보 2개의 Chrome 기준 실행이다. Spinon runtime, Android/iOS, WGPU surface 또는 Spinon WPT 적합성을 실행하지 않았다. C11.2.1 코드는 별도 변경으로 통합되고 Rust·Chrome fixture 검사가 통과하기 전까지 C11.2.2 runtime 구현을 시작하지 않는다. C11.2.1의 Android 실기기·전체 10-scene placement matrix·Spinon WPT 관문도 별도로 미완료다.

검증 명령: `node --test tools/css-reference/c11-grid-auto-placement-precomparison.test.mjs` — 4개 test 통과.

## 장면 캡처

![definite item 선배치](c1122-chrome-20261012T063150043Z-c60e5138/definite-first-mixed-dpr1.png)

![sparse 배치가 앞선 hole을 다시 채우지 않음](c1122-chrome-20261012T063150043Z-c60e5138/sparse-hole-not-backfilled-dpr1.png)

![dense control은 hole을 다시 채움](c1122-chrome-20261012T063150043Z-c60e5138/reject-dense-flow-dpr1.png)
