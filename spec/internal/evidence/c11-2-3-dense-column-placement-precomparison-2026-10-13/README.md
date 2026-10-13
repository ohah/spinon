# C11.2.3 dense·column Grid 사전 비교

## 고정 입력과 실행 환경

- Inventory v6: [`grid-auto-placement-c1123-v6.json`](../../../../tests/fixtures/css/c11/grid-auto-placement-c1123-v6.json) · SHA-256 `47c845ca6c5af40d3cd80c98904e5cdb639bba61e88161ea5ca0df82d09c22c7`
- HTML v6: [`grid-auto-placement-c1123-v6.html`](../../../../tests/fixtures/css/c11/grid-auto-placement-c1123-v6.html) · SHA-256 `3e252f64565d1b9819f6afd964e981117359b6f59ecf2c67fad8b9c537f9dfdd`
- Chromium reference v7: [`c11-2-3-grid-auto-placement-precomparison-v7.json`](../../../../tests/fixtures/css/references/c11-2-3-grid-auto-placement-precomparison-v7.json)
- 캡처 도구: [`capture-c11-2-3-grid-auto-placement-precomparison.mjs`](../../../../tools/css-reference/capture-c11-2-3-grid-auto-placement-precomparison.mjs) · SHA-256 `0b43fa6b37329c0adc7f49935b9914218d85e7e8941bc2b0daf66184cd15a1d9`
- 생성기: [`build-c11-2-3-grid-auto-placement-fixture.mjs`](../../../../tools/css-reference/build-c11-2-3-grid-auto-placement-fixture.mjs) · SHA-256 `a46b25ad8435ce3a3227a0f49167bf9583f89311572a384d3dd3c23e13173393`
- 실행 ID: `c1123-chrome-20261012T232221546Z-fe06e683`
- Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- macOS `26.5.1` arm64, Node `v26.7.0`, viewport `360 × 800` CSS px, DPR 1·2, `en-US`, UTC, light, LTR, `horizontal-tb`, coarse pointer

25개 scene·107개 node를 DPR 1·2에서 수집했다. 구현 후보는 21개, oracle-only 제외 입력은 4개다. scene마다 DOM preorder·parent NodeId, computed Grid property, explicit/computed/implicit track 수, CSS px rect를 저장하고 DPR 1 screenshot 25개를 보관했다. 두 DPR의 CSS computed 값·track 수·rect는 모든 scene에서 같았다.

## 주요 관측

| 관측 | Chrome 기준 |
| --- | --- |
| `row dense` hole fill | `small-c` 위치 `(108, 20)` |
| 같은 source order의 `row sparse` | `small-c` 위치 `(108, 44)` |
| `column dense` hole fill | `small-c` 위치 `(20, 76)` |
| 같은 source order의 `column sparse` | `small-c` 위치 `(68, 76)` |
| row dense positive implicit rows | 2개, `grid-auto-rows: 25px` |
| column sparse/dense positive implicit columns | 각 2개, `grid-auto-columns: 28px` |
| nested Grid | 바깥 row sparse와 안쪽 row dense 독립 계산; 안쪽 implicit rows 2개, 18px |
| 최대 track 후보 | `span 10000`은 computed columns 10,000개(implicit 9,999개) |
| 상한 초과 후보 | `span 10001`은 Chrome에서 computed columns 10,001개(implicit 10,000개) |

마지막 두 행은 Chrome 동작을 보존한다. Spinon profile 상한은 10,000이며 초과 입력은 Taffy 호출 전에 거부한다. Chrome이 10,001개를 계산했다는 사실은 Spinon이 이를 허용해야 한다는 뜻이 아니다.

![row dense hole fill](c1123-chrome-20261012T232221546Z-fe06e683/row-dense-hole-fill-dpr1.png)

![row sparse 대조군](c1123-chrome-20261012T232221546Z-fe06e683/row-sparse-hole-control-dpr1.png)

![column dense hole fill](c1123-chrome-20261012T232221546Z-fe06e683/column-dense-hole-fill-dpr1.png)

![column sparse 대조군](c1123-chrome-20261012T232221546Z-fe06e683/column-sparse-hole-control-dpr1.png)

## 고정 WPT 후보

WPT checkout revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`다. sparse checkout에서 공용 CSS helper가 빠진 첫 실행은 결과로 채택하지 않았다. `css/support/grid.css`를 checkout 범위에 보충한 뒤 재실행했다. 최종 Chrome WPT runner 결과는 3 tests, 36 subtests, 39 checks 모두 예상 결과, skip 0이다.

| 후보 | SHA-256 | Chrome runner 관측 |
| --- | --- | --- |
| [`grid-auto-flow-sparse-001.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/placement/grid-auto-flow-sparse-001.html) | `9d30cb69227ab5bf80fda3f557b03eb89be7c9d5171de73ef7db8ecc5f80c0cf` | PASS · column sparse 포함 |
| [`grid-auto-placement-fixed-row-column-001.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/placement/grid-auto-placement-fixed-row-column-001.html) | `14c4a5c29bfd093473c7a5794637ea8d5aea44f4298c7a4f17df360d373762a2` | PASS · 고정 축과 sparse/dense span 배치 |
| [`grid-auto-placement-implicit-tracks-001.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/placement/grid-auto-placement-implicit-tracks-001.html) | `785012f37a838995bdd6491bd9d27034d21e2441152a067f050bc82fd993ad53` | PASS · row/column dense와 implicit track |

실행 로그: [첫 helper 누락 실행](wpt-chrome-154-run.log), [최종 재실행 요약](wpt-chrome-154-rerun.log), [상세 runner 로그](wpt-chrome-154-mach.log), [구조화 결과](wpt-chrome-154-report.json). 공용 helper `css/support/grid.css` SHA-256은 `b2f35862e7b8fa9dcae8f4f8d25add78d93f3e7993e08a4337030a4dd2d4425e`다.

## 판정 경계

이 결과는 Chrome 154의 사전 기준과 WPT 원문 실행 결과다. Spinon layout·Stylo·FFI·runtime, Android 실기기, iOS Simulator, WGPU 표면 및 Spinon WPT 적합성은 실행하지 않았다. oracle-only scene은 Chrome의 제외 입력 동작을 보존할 뿐 Spinon 지원을 뜻하지 않는다. C11.2.3 런타임 구현·검증이나 전체 Grid/CSS 지원 완료의 근거가 아니다.

기준 검사: `bun run css:check:c11-2-3-grid-fixture`, `bun run test:css-c11-2-3-auto-placement` — fixture 재현 검사와 5개 reference test가 통과했다.
