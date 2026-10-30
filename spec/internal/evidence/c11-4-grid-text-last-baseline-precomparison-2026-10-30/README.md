# C11.4 multiline direct-text last-baseline 사전 비교

**판정:** 고정 Ahem fixture의 multiline last-baseline 그룹을 CSS Grid 2025 CRD의 baseline-shim 규칙과 CSS Box Alignment의 last-baseline 정렬 규칙으로 계산했다. 다섯 장면의 독립 기대값은 Agent Browser가 연 HeadlessChrome 156 관측과 일치한다. 이는 browser oracle이며 Spinon 구현·앱 검증이 아니다.

## 고정 입력과 환경

- Fixture: [grid-last-baseline-multiline-direct-text.html](../../../../tests/fixtures/css/c11/grid-last-baseline-multiline-direct-text.html), SHA-256 `86d937790e2dd5a6336c86635972b7387bd36f7b78c99156d155965d762c0272`.
- Font: 저장소 [Ahem.ttf](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`; UPEM 1000, ascent 800, descent −200.
- Browser: Agent Browser HeadlessChrome 156, macOS ARM64, viewport 1280×633, DPR 1. `document.fonts.check('10px Ahem')`와 20px 모두 true였다.
- Raw observation: [Chrome JSON](./chrome/chrome156-observation.json) · [화면](./chrome/chrome156-grid-last-baseline.png) · [독립 기대값](./expected-values.json).
- Reproducible comparison: `node spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/verify-precomparison.mjs` checks all five scenes, root-relative geometry, wrapping, Ahem line baselines and browser font readiness.
- Plan PR hostile diff review: [30 distinct checks](./plan-pr-diff-review-30.md).

## 독립 규범 계산

[CSS Grid 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 baseline shim 계산에서 선택된 baseline 좌표를 맞추고 shim을 track contribution에 포함한다. [CSS Box Alignment 3의 baseline 규칙](https://www.w3.org/TR/css-align-3/#baseline-rules)은 last-baseline 그룹과 해당 정렬을 정의한다.

작은 item은 25px 열에서 `XX XX`가 두 줄로 감긴다. Ahem ascent 비율 `800/1000`에 따라 10px line box의 각 baseline은 line top에서 8px이며 마지막 baseline은 item top에서 18px다. 큰 item은 20px 한 줄이고 baseline은 item top에서 16px다. 따라서 last-baseline 그룹의 공통 좌표는 18px이다. 마지막 baseline에서 block-end까지의 거리는 작은 item 2px·큰 item 4px이므로 end shim은 작은 item 2px, 큰 item 0px이다. end fallback으로 배치하면 item y는 0px·2px이고 실제 last baseline은 둘 다 y=18px다. auto row는 `max(20+2, 20+0) = 22px`이다.

first-baseline control의 공통 좌표는 16px이며 start shim은 작은 item 8px, 큰 item 0px로 row 높이는 28px다. start control은 둘 다 y=0이고 row 높이는 20px다. 열 25px + 8px gap + 40px로 모든 Grid 폭은 73px다. align-self, source-order 역전 장면도 last-baseline geometry와 row size를 보존해야 한다.

| 장면 | 열 / 행 | 작은 item y | 큰 item y | 비교할 baseline |
| --- | --- | ---: | ---: | --- |
| `align-items:last baseline` | `25px 40px` / `22px` | 0px | 2px | 마지막 line baseline 모두 18px |
| 두 `align-self:last baseline` | `25px 40px` / `22px` | 0px | 2px | 마지막 line baseline 모두 18px |
| source-order 역전 | `25px 40px` / `22px` | 0px | 2px | 마지막 line baseline 모두 18px |
| `align-items:first baseline` control | `25px 40px` / `28px` | 8px | 0px | 첫 baseline 모두 16px |
| `align-items:start` control | `25px 40px` / `20px` | 0px | 0px | baseline은 8px와 16px로 다름 |

## 관측과 한계

Browser observation의 computed rows, Grid/item rect와 각 line rect가 위 독립 계산과 일치한다. Reversed scene은 각 item에 row와 column을 명시해 자동 배치가 우연히 두 번째 row를 만드는 것을 막았다. Grid wrapper는 block formatting context를 유지하여 바깥쪽 중첩 Grid의 intrinsic sizing이 결과를 오염시키지 않도록 했다. 이 두 fixture 조건은 앞선 잘못된 관측을 수정하며 plan review에서 회귀 조건으로 유지한다.

Screenshot의 파란 outline과 빨간 baseline guide는 배치 확인용이다. 이 자료는 Chrome의 고정 입력·geometry만 검증한다. Spinon layout, Android/iOS runtime, glyph raster equality, 일반 CSS admission, WPT 또는 C11.4 전체는 검증하지 않는다.

## SHA-256

```text
86d937790e2dd5a6336c86635972b7387bd36f7b78c99156d155965d762c0272  tests/fixtures/css/c11/grid-last-baseline-multiline-direct-text.html
b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448  tests/fixtures/resources/c15/Ahem.ttf
726f2bd2c689dbd8675b4578ca25eb2e900d54659ea16ab4333541997cc91d9e  spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/expected-values.json
44f9897e78f92b3fc6fc36b508ca062a8a4a7b1156cb685f8943ee5a4f500230  spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/verify-precomparison.mjs
7556a46061f07c67d115676e2c800a7c78b694f2dc5a828d778956c09e3a76d1  spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/chrome/chrome156-observation.json
f1c8e1bc83bda967f87a2e743da9d3877d4a91bd224081301544f11d01a5e565  spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/chrome/chrome156-grid-last-baseline.png
```
