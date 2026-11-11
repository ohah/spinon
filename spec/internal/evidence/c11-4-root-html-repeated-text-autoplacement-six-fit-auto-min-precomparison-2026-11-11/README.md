# C11.4 six-fit repeated-text auto-placement `min-height:auto` 사전 비교

## 범위와 기준

이 자료는 Agent Browser 0.31.1의 HeadlessChrome `157.0.0.0`에서 고정 HTML을 실행한 CSS Grid 관측이다. `navigator.platform`은 `MacIntel`, DPR은 1이었다. 로컬 HTML의 브라우저 CSS 동작만 관측했으며 Spinon Root HTML parser, `RuntimeSession`, 앱 route 또는 제품 지원의 근거가 아니다.

규범 비교 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-terminology), [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)다. §11.2에서 `fit-content()` track의 minimum sizing function은 `auto`다. §6.6은 non-scrollable grid item의 축에서 span한 track 중 `auto` minimum이 있을 때 content-based automatic minimum을 사용하며, `fit-content()` cap은 그 content-based minimum을 fixed max track처럼 자르지 않는다고 설명한다. Chrome 구현을 확인할 때 참고할 고정 source 경로는 [Chromium 157.0.8103.40 Grid track sizing](https://chromium.googlesource.com/chromium/src/+/refs/tags/157.0.8103.40/third_party/blink/renderer/core/layout/grid/grid_track_sizing_algorithm.cc)다. 실제 기대값은 Chromium 코드 해석이 아니라 아래 고정 fixture를 Chrome에서 실행해 얻었다.

## 고정 입력과 matrix

- definite `40×220px` Grid, explicit `40px` column과 빈 `10px` row, sparse row auto-flow, LTR, gap 0, `overflow:visible`, `align-items:normal`을 사용했다.
- explicit-row empty control 다음에 direct text item 여덟 개를 배치했다. 아이템은 한 row에만 자동 배치하고, dense flow·명시 배치·span은 사용하지 않았다.
- text는 10px monospace, 10px line-height이며 item 순서별 payload line count는 `[2,2,2,3,2,2,3,1]`이다. 각 item은 computed `min-height:0` 또는 `auto`를 쓴다.
- six-member `grid-auto-rows`는 모두 `fit-content()`다. 고정 cap 값 `0px`, `10px`, `20%`, `30px`, `40px`, `50px`의 authored 순열 720개를 definite 220px percentage basis에서 비교했다. 이 basis에서 `20%`는 44px다.
- track list 720개 각각을 `align-content:start|stretch`, DOM forward/reverse, item `min-height:0|auto`와 교차해 총 5,760 scene 및 2,880 baseline/candidate pair를 만들었다. `%` cap의 basis는 definite 220px block size다.
- 독립 검산기는 fixture SHA-256, browser metadata, scene identity와 전체 matrix axis, computed list/cap order, row/item 위치, payload line-box geometry, floor·overflow 및 alignment geometry를 확인한다.
- fixture와 결과 수집은 [oracle fixture](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/oracle-fixture.html), [Agent Browser capture](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/capture-chrome.mjs)다. 원본 observation은 [NDJSON](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/chrome157-observation.ndjson), independent verifier는 [Python source](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/verify-chrome-matrix.py)다.

## 결과

[독립 검산 결과 JSON](https://raw.githubusercontent.com/ohah/spinon/fc22a202944c9ebd2026d9abe30d8dc5df1caf18/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/verification.json):

| 관측 | Chrome 157 결과 |
| --- | ---: |
| Scenes / paired keys | 5,760 / 2,880 |
| Unique all-fit track lists / ordered cap assignments | 720 / 720 |
| Candidate `min-height:auto` fit-track item occurrences | 23,040 |
| Baseline `min-height:0` items below payload floor | 7,200 |
| Candidate payload-floor violations | 0 |
| Baseline payload overflowing its own row | 7,200 |
| Candidate payload overflowing its own row | 0 |
| Baseline payloads beyond the 220px Grid end | 0 |
| Candidate payloads beyond the 220px Grid end | 0 |
| Baseline/candidate paired keys with different row sizes | 2,880 / 2,880 |
| `align-content:start` vs `stretch` row or item geometry differences | 0 |

Baseline의 payload-floor 침범 7,200건은 `fit-content(0px)`에서 3,840건, `fit-content(10px)`에서 3,360건이었다. 나머지 cap에서는 이 입력 matrix의 payload floor보다 cap이 낮지 않았다. Candidate `min-height:auto`에서는 여섯 fit track의 전체 23,040 item occurrence가 payload floor 이상이었다. 가장 아래 payload 끝은 baseline `150px`, candidate `180px`였고 두 값 모두 220px Grid 안에 있었다.

이는 주어진 text, cap 집합, 고정 grid와 자동 배치에서의 비교값이다. 모든 fit-content 조합, 일반 CSS Grid, arbitrary text, dynamic mutation, row-span, nested grid, WPT 또는 Spinon 구현의 정확성을 주장하지 않는다. 특히 2,880 pair는 이 관측 matrix에서만 `min-height:auto` 적용에 따라 row 크기가 바뀌었다는 뜻이다.

## 재현과 작업 자원 정리

```sh
node spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/capture-chrome.mjs
python3 spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/verify-chrome-matrix.py
```

수집은 Agent Browser 전용 session `spinon-c114-six-fit-autoplacement-precomparison-20261111`을 사용했고 스크립트 종료 때 session을 닫았다. 종료 뒤 `session info --json`은 `active:false`, `pid:null`을 반환했다. active 목록에 남은 unrelated `hwpjs-host-print-focus-r1-after-preview` 세션은 보존했다. Terminal Browser 및 iOS/Android simulator는 실행하지 않았고 새 simulator를 만들거나 부팅하지 않았다. 종료 확인은 `agent-browser-lifecycle.json`에 기록했다.

계획 PR의 실제 원격 diff에 대한 20관점 검토는 [`plan-pr-diff-review-20.md`](./plan-pr-diff-review-20.md)에 있다.

대표 장면: [Chrome 157 six-fit sample](https://raw.githubusercontent.com/ohah/spinon/867c09706cfeab9faf10edaa912c70039948de0a/spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/chrome157-six-fit-auto-min-sample.png). 캡처는 layout 입력을 바꾸지 않았다. 모든 근거 파일의 해시는 이 폴더의 `SHA256SUMS`에 있다.
