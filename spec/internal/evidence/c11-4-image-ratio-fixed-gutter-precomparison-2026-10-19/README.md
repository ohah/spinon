# C11.4 fixed max track span과 gutter의 사전 비교

2026-10-19, direct replaced Grid item의 §6.6 size suggestion과 automatic minimum을 위해 Chrome 155 Agent Browser 관찰과 CSS Grid Level 1 표준 기대값을 분리해 기록했다. 이 실행은 계획·oracle 자료이며 Spinon 구현, Android/iOS Simulator, WPT, 제품 기능 검증이 아니다.

## 기준과 재현

- 표준: [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto). fixed max track만 span하는 경우 suggestion과 transferred-size source input을 max track functions의 합과 내부 fixed gutters의 합으로 제한한다.
- Browser: Google Chrome `155.0.8059.85`, Agent Browser `0.31.1`, macOS `26.5.1` ARM64, DPR 1, `innerWidth×innerHeight` `1280×633`.
- Fixture: [grid-track-sizing-c114-image-ratio-fixed-gutter.html](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html). 모든 image는 decode 완료 후 수집했다. Natural dimensions는 `24×16`; fixed preferred size는 `36px`; authored ratio는 `3/2`이다.
- 주요 기록: [관찰 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/chrome155-agent-browser-observation.json), [Agent Browser 원출력](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/chrome155-agent-browser-observation.raw.json), [SHA-256](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/SHA256SUMS).

## 관찰과 표준 기대값

Chrome은 두/세 track, gutter 0/3px, row/column 및 외부 unspanned track 유무와 관계없이 row-transfer 장면의 item row `24px`, column-transfer 장면의 item column `54px`를 관찰했다. Item frame은 내부 transferred-size suggestion을 직접 보여주지 않으므로 이 값만으로 clamp 불일치를 주장하지 않는다. 다만 preferred width가 `36px`인 첫 장면에서 used columns `29px 4px`가 관찰됐고, §6.6 기준 specified size contribution은 fixed grid-area limit `10px` 이하여야 한다. Browser observation은 구현 비교 자료이며 규범적 정답은 아니다.

| 장면 | fixed max tracks | 내부 gutter | §6.6 limit | suggestion contribution 기대값 (item frame 아님) | Chrome item/track 관찰 |
| --- | --- | ---: | ---: | ---: | --- |
| column two tracks | `3px + 4px` | `3px` | `10px` | row `6.666667px` | columns `29px 4px`, item `36×24px`, row `24px` |
| column two tracks | `3px + 4px` | `0px` | `7px` | row `4.666667px` | columns `32px 4px`, item `36×24px`, row `24px` |
| column three tracks | `3px + 4px + 5px` | `3px × 2` | `18px` | row `12px` | columns `21px 4px 5px`, item `36×24px`, row `24px` |
| middle column span | `3px + 4px` | `3px` inside span | `10px` | row `6.666667px` | columns `80px 29px 4px 90px`, item `36×24px`, row `24px` |
| row two tracks | `3px + 4px` | `3px` | `10px` | column `15px` | rows `29px 4px`, item `54×36px`, column `54px` |
| row three tracks | `3px + 4px + 5px` | `3px × 2` | `18px` | column `27px` | rows `21px 4px 5px`, item `54×36px`, column `54px` |
| middle row span | `3px + 4px` | `3px` inside span | `10px` | column `15px` | rows `80px 29px 4px 90px`, item `54×36px`, column `54px` |
| content-box image | border-box max `10px`, content max `6px` | `3px` | `6px` transfer input | content height `4px` + edges `4px` = row `8px` | columns `33px 4px`, outer item `40×28px`, row `28px` |
| flexible / fit-content controls | not all fixed max functions | `3px` | no all-fixed limit | row `24px` | columns `29px 4px`, row `24px` |

표는 Chrome의 computed track 문자열과 `getBoundingClientRect()` 결과를 사용한다. 이미지 contribution으로 일부 used track이 authored fixed max보다 커졌다. 이는 관찰된 used size이며 §6.6 기대 limit의 입력은 아니다. 표준 기대 limit은 item span 안의 fixed max sizing function과 gutter에서 계산했다. 기대 열의 row/column 값은 중간 contribution 입력값이며 item frame 기대값이 아니다. 수집 전에 `window.c114ImageRatioFixedGutterReady`로 모든 이미지 decode 완료를 기다렸으며 JSON은 각 장면의 natural size와 source URL을 기록한다.

![Chrome 155 Agent Browser fixture screenshot](./chrome155-fixed-gutter.png)

## Hashes

The plan revision has a separate [20-point review](./plan-revision-review-20.md). The [SHA-256 manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19/SHA256SUMS) covers the fixture, normalized JSON, raw Agent Browser output, screenshot, plan reviews and separate PR diff review. The screenshot shows a visible rendered page; it does not prove Spinon behavior or pixel output from either mobile simulator.
