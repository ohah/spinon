# C11.4 five-fit repeated-text row-span `min-height:auto` 사전 비교

## 기준과 범위

- 규범 기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. Automatic minimum eligibility, `fit-content()`의 automatic minimum track sizing function, spanning contribution을 구분한다.
- 독립 기준 구현은 Agent Browser 0.31.1이 실행한 HeadlessChrome `157.0.0.0`이다. `MacIntel`, DPR 1, macOS 26.5.1 arm64에서 실행했다. 이 결과는 Chrome CSS 동작이며 Spinon 구현 결과가 아니다. Terminal Browser는 사용하지 않았다.
- 고정 입력은 definite `40×220px` Root HTML Grid, explicit column `40px`, explicit row `10px`, gap 0, LTR sparse row flow, `overflow:visible`이다. Text payload·font·line-height는 four-fit row-span 기준 fixture와 같다.
- 여섯 implicit row list는 다섯 `fit-content()` track과 `min-content`, `max-content`, `auto` 중 하나인 singleton tail로 구성한다. Fit 위치 quintuple 6개 × 다섯 cap (`0px`, `10px`, `20%`, `30px`, `40px`) 순서 120개 × tail 3개로 2,160 track list를 만든다. 각 list에 `align-content:start|stretch`, 두 row-span topology의 forward/reverse DOM order, `min-height:0|auto`를 교차해 34,560 scene과 17,280 baseline/candidate pair를 관측했다.

## 관측과 독립 검증

독립 verifier는 scene index·브라우저 고정값·track/cap 순서·used row serialization·grid/item geometry·payload line count·source order·alignment·minimum mode를 검사했다. geometry tolerance는 `0.01 CSS px`다.

| 측정 | 결과 |
|---|---:|
| Chrome scene / baseline-candidate pair | 34,560 / 17,280 |
| 고유 track list / fit 위치 quintuple / cap 순서 / tail 값 | 2,160 / 6 / 120 / 3 |
| candidate automatic-minimum eligible item occurrence | 77,760 |
| baseline payload-floor violation | 13,248 |
| candidate payload-floor violation | 0 |
| span item frame과 covered row 합의 최대 차이 | 0.00025 CSS px |
| DOM source-order 반전의 의미 geometry 차이 | 0 CSS px |
| stretch의 non-auto track 최대 변화 | 0 CSS px |
| auto tail의 stretch 배분 최대값 | 130.0002 CSS px |
| cap 순열 검증 그룹 | 288개, 각 120개 순열 |

`align-content:stretch`는 tail 종류별로 판정했다. `auto` tail만 남은 공간을 받았고 `min-content`/`max-content` tail은 used row를 바꾸지 않았다. Automatic-minimum 집계는 같은 payload를 쓴 `min-height:0` 기준선과 `auto` 후보를 짝지었다. 이 수치는 이 고정 fixture에서만 관찰한 결과다.

## 재현과 산출물

- [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-five-fit-auto-minimum-precomparison.html)
- [Agent Browser collector](./capture-chrome.mjs) · [capture manifest](./capture-manifest.json) · [independent verifier](./verify-chrome-matrix.mjs)
- [압축된 전체 Chrome observation](./chrome157-observation.ndjson.gz) · [검증 요약](./verification.json) · [대표 장면](./chrome157-five-fit-auto-minimum-sample.png)
- [Agent Browser lifecycle](./agent-browser-lifecycle.json) · [SHA-256 manifest](./SHA256SUMS)

전용 Agent Browser session `spinon-c114-five-fit-row-span-auto-minimum-20261111`에서 캡처하고 종료했다. Collector는 observation을 gzip으로 저장하며 raw JSONL과 압축 파일의 byte hash를 manifest에 기록한다. `opened=true`, `closed=true`를 기록했고, 확인 시 session 목록은 비어 있었다. 같은 session 이름을 가진 Browser process도 남아 있지 않았다.

압축 observation은 1,151,212 bytes이고 해제된 JSONL은 67,378,320 bytes다. 기존 raw capture와 압축본을 해제한 byte stream은 `cmp`로 같음을 확인했다. Independent verifier는 압축 hash와 해제 후 hash 모두 확인한다.

## 범위 한계

이 자료는 Chrome 사전 비교다. Spinon RuntimeSession, Android/iOS 실행, 표면 readback, compositor 뒤의 pixel parity, 실기기·hardware GPU, WPT 적합성, 임의 CSS Grid admission 또는 C11.4/C11 완료를 증명하지 않는다. 계획과 제품 구현은 별도 작업이며 공식 구현 상태 `78/214`와 내부 숫자 계약 `0.1.0`을 바꾸지 않는다.
