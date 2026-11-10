# C11.4 five-fit repeated-text auto-placement `min-height:auto` 사전 비교

## 범위와 기준

Agent Browser 0.31.1의 Chrome `157.0.8103.40`에서 고정 HTML fixture를 실행한 CSS Grid 관측이다. `navigator.userAgent`는 HeadlessChrome `157.0.0.0`, platform은 `MacIntel`, DPR은 1이었다. 이 자료는 Spinon RuntimeSession, 앱 route 또는 제품 지원의 증거가 아니다. Terminal Browser와 iOS/Android simulator는 실행하지 않았다.

규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.3–11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-algorithm)다. `fit-content()`의 minimum track sizing function이 `auto`라는 점과 grid item automatic minimum eligibility를 기준으로 baseline/candidate를 짝지었다. `min-content`, `max-content`, `auto` tail은 fit track과 별도 축으로 관측했다.

Chrome 쪽 구현 참고는 [Chromium 157.0.8103.40 Grid track sizing source](https://chromium.googlesource.com/chromium/src/+/refs/tags/157.0.8103.40/third_party/blink/renderer/core/layout/grid/grid_track_sizing_algorithm.cc)다. 여기서는 해당 버전의 track sizing 구현을 확인하는 데만 쓴다. 측정값은 아래 fixture를 Chrome에서 직접 실행한 결과이며 소스 코드 해석으로 대체하지 않는다.

## 고정 입력과 matrix

- Grid: definite `40×220px`, explicit `40px` column, 빈 explicit `10px` row, gap 0, LTR sparse row auto-flow, `overflow:visible`.
- Direct child: 빈 explicit-row control 하나와 반복 text item 여덟 개. Text는 10px monospace, 10px line-height, line count `[2,2,2,3,2,2,3,1]`.
- 여섯-member `grid-auto-rows`: 정확히 `fit-content()` 다섯 개와 tail 하나(`min-content`, `max-content`, `auto` 중 하나).
- Fit position quintuple: 6가지. Tail 종류: 3가지. 다섯 cap (`0px`, `10px`, `20%`, `30px`, `40px`)을 authored fit 순서에 배정하는 순열: 120가지. 따라서 2,160개 unique authored track list.
- 각 list를 `align-content:start|stretch`, DOM forward/reverse, item `min-height:0|auto`와 교차했다. 총 17,280 scenes, 8,640 baseline/candidate paired keys.
- `%` cap은 definite 220px block-size basis에 대해 계산했다. 모든 scene에서 computed `grid-auto-rows`, used row sequence, item/payload frame과 line placement를 보존했다.

## 결과

| 관측 | Chrome 157 결과 |
| --- | ---: |
| Scenes / paired keys | 17,280 / 8,640 |
| Unique track list / fit-position quintuple / ordered cap assignment | 2,160 / 6 / 120 |
| Candidate `min-height:auto` fit-track item occurrences | 57,600 |
| Baseline `min-height:0` fit-track occurrences below payload floor | 21,600 |
| Candidate fit-track floor violations | 0 |
| Tail payload-floor violations | 0 |
| Own-row text payload overflow, baseline `min-height:0` | tail별 7,200 |
| Own-row text payload overflow, candidate `min-height:auto` | 0 |
| `align-content:stretch` row result differs from `start` | 2,880 paired comparisons |
| Stretch difference by tail/mode | `auto/0px`: 1,440; `auto/auto`: 1,440 |
| Item frame beyond 220px Grid end | 0 |
| Text payload beyond 220px Grid end | 600, 모두 `auto` tail + baseline `min-height:0` |
| Maximum text payload end, baseline / candidate | 240px / 220px |

따라서 bounded candidate profile은 세 tail 종류 모두의 고정 matrix에서 fit-row payload floor를 지켰다. 그러나 `auto` tail의 stretch 결과는 다른 두 tail과 같지 않고, baseline `min-height:0`의 `auto` tail에는 Grid 끝을 넘는 payload 600건이 있다. 계획과 구현 검토에서 tail·minimum·alignment별 값을 분리한다. 관측은 이 고정 list/cap/text 조합에만 적용되며 일반 CSS Grid 결과로 확대하지 않는다.

## 재현과 task 자원 정리

- 입력: [`oracle-fixture.html`](./oracle-fixture.html)
- Collector: [`capture-chrome.mjs`](./capture-chrome.mjs). 고유 session `spinon-c114-five-fit-autoplacement-precomparison-20261111`을 열고 완료 뒤 닫는다.
- PR 화면: [`capture-pr-sample.mjs`](./capture-pr-sample.mjs)가 같은 oracle fixture의 대표 scene을 CSS `transform`으로 확대해 캡처한다. 값이나 layout input은 바꾸지 않는다. [`chrome157-five-fit-auto-min-pr-sample.png`](./chrome157-five-fit-auto-min-pr-sample.png)
- 원본: [`chrome157-observation.ndjson`](./chrome157-observation.ndjson)
- 독립 검산기: [`verify-chrome-matrix.py`](./verify-chrome-matrix.py)
- 결과 요약: [`verification.json`](./verification.json)
- 대표 장면: [`chrome157-five-fit-auto-min-sample.png`](./chrome157-five-fit-auto-min-sample.png)
- 생성 뒤 Agent Browser session은 비활성(`active:false`, `pid:null`)임을 확인했다. 별도 기존 Agent Browser session과 공유 프로세스는 건드리지 않았다. Simulator는 만들거나 부팅하지 않았다.
- [`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에는 session 종료, ADB/iOS booted-device가 없었던 cleanup 확인을 기록했다.
- 파일 무결성: [`SHA256SUMS`](./SHA256SUMS).

재현 명령:

```sh
node spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/capture-chrome.mjs
python3 spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11/verify-chrome-matrix.py
```

이 evidence는 Android/iOS app route, RuntimeSession, PixelCopy/Metal surface readback, compositor 이후 pixel, 실기기/hardware GPU, WPT runner 또는 일반 CSS admission을 주장하지 않는다.
