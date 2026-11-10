# C11.4 네 fit repeated-text auto-placement `min-height:auto` 사전 비교

## 범위

독립 oracle은 Agent Browser 0.31.1이 실행한 HeadlessChrome 157.0.0.0, `MacIntel`, DPR 1이다. Terminal Browser와 iOS/Android simulator는 사용하지 않았다. 이 비교는 Chrome CSS Grid 동작을 고정 입력에서 관찰하며 Spinon RuntimeSession 구현을 검증하지 않는다.

CSS 근거는 [CSS Grid 2025 CRD §6.6 자동 최소 크기](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.3–11.5 track sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-algorithm), [§7.2 track sizing functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions)다. `fit-content()` track의 min sizing function은 `auto`이고, `overflow`가 scrollable이 아니며 flexible track을 가로지르지 않는 grid item의 automatic minimum 조건과 맞는다. `min-content`·`max-content` tail은 별도의 대조 축으로 유지한다.

## 고정 입력과 관측량

- 40×220 CSS px Grid, 40px explicit column, 빈 10px explicit row, gap 0, LTR·sparse row auto-flow, `overflow:visible`.
- direct child는 빈 explicit-row control 한 개와 repeated text item 여덟 개다. text는 10px monospace/10px line-height이며 예상 line count는 `[2,2,2,3,2,2,3,1]`이다.
- 여섯-member `grid-auto-rows` list에는 정확히 `fit-content()` 네 개, `min-content` 한 개, `max-content` 한 개를 둔다. Fit 위치 quadruple 15개와 두 tail 순서를 모두 관측한다.
- 네 cap은 `0px`, `10px`, `20%`, `30px`이며 네 위치에 배정하는 24개 순열을 모두 관측한다. Percentage basis는 definite 220px이다.
- 각 authored list에 `align-content:start|stretch`, DOM order forward/reverse, item `min-height:0|auto`를 적용한다. 총 720 track list, 5,760 scenes, 2,880 paired baseline/candidate key다.

## 결과

| 관측 | 결과 |
| --- | ---: |
| Chrome scene / paired key | 5,760 / 2,880 |
| Unique track list / fit-position quadruple / cap assignment | 720 / 15 / 24 |
| `min-height:auto`에서 fit track에 놓인 item-frame occurrence | 15,360 |
| paired `min-height:0`에서 payload floor보다 작은 fit-row occurrence | 7,200 |
| `min-height:auto` candidate의 fit-row floor 위반 | 0 |
| text payload가 own auto-row를 넘는 occurrence (`0px` / `auto`) | 7,200 / 0 |
| min/max tail row의 payload-floor 위반 | 0 |
| item frame 또는 text payload가 220px Grid 끝을 넘음 | 0 |
| `align-content:stretch`와 `start` 간 track-row 차이 | 0 |

Representative scene의 authored list는 `fit-content(0px) fit-content(10px) fit-content(20%) fit-content(30px) min-content max-content`다. Chrome 사용 row는 baseline `[10,0,10,20,30,20,20,0,10]`, candidate `[10,20,20,20,30,20,20,30,10]`이다. Baseline의 7,200 visible text payload under-row occurrence는 Grid block end 밖으로 나가지 않았다. 이는 고정 geometry·text·cap 조합의 관측이며 다른 list나 앱 runtime 전체를 일반화하지 않는다.

## 실행 및 정리

- `oracle-fixture.html`은 브라우저에서 실행한 입력, `capture-chrome.mjs`는 고유 Agent Browser session을 이용하는 재현 collector, `chrome157-observation.ndjson`은 원본 scene corpus, `verify-chrome-matrix.py`는 별도 matrix verifier다.
- 재현: `node spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/capture-chrome.mjs` 후 `python3 spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/verify-chrome-matrix.py`.
- session `spinon-c114-four-fit-autoplacement-precomparison-20261111`은 capture 직후 close했다. 종료 뒤 session list는 비었고 작업 session/daemon process selector는 일치 항목이 없었다. Terminal Browser와 simulator는 실행하지 않았다.
- 이 evidence는 Android/iOS app route, RuntimeSession, PixelCopy/Metal readback, compositor 이후 pixel, 실기기/hardware GPU, WPT runner 또는 일반 CSS admission을 주장하지 않는다.

## 파일

- [oracle fixture](./oracle-fixture.html)
- [Chrome 157 observation](./chrome157-observation.ndjson)
- [matrix verifier](./verify-chrome-matrix.py)
- [대표 Chrome scene](./chrome157-four-fit-auto-min-sample.png)
- [계획 PR diff 적대 검토 30개](./plan-pr-diff-review-30.md)
- [Agent Browser lifecycle](./agent-browser-lifecycle.json)
- [종료 뒤 owned process 확인](./agent-browser-processes-after-close.txt)
