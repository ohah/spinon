# C11.4 four-fit repeated-text row-span `min-height:auto` 사전 비교

## 기준과 범위

- 규범 출처는 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. automatic minimum 적용 조건, fit-content 최소 sizing, spanning contribution을 각각 판정한다.
- 기준 구현은 Agent Browser 0.31.1의 HeadlessChrome `157.0.0.0`이다. `MacIntel`, DPR 1, macOS 26.5.1 arm64에서 실행했다. Chrome 바이너리 SHA-256은 `c9a0f292a25c3771f1895ec3397d2cbbee3ce07ce967acd35f34a38978b4aacd`다. Terminal Browser는 사용하지 않았다.
- 입력은 definite `40×220px` Root HTML Grid, explicit column `40px`, explicit row `10px`, gap 0, visible overflow, LTR sparse row flow다. 여섯 implicit row는 서로 다른 cap을 가진 `fit-content()` 네 개와 `min-content`·`auto` 한 개씩이다.
- Fixture는 fit 위치 quadruple 15개 × cap 순서 24개 × tail 순서 2개 × alignment 2개 × span topology/source order 4개 × minimum mode 2개 = **11,520 scene**을 관측한다. 고유 track list는 720개, paired key는 5,760개다.

## 실행 결과

| 측정 | 결과 |
|---|---:|
| Chrome 장면 / paired key | 11,520 / 5,760 |
| track list / fit 위치 / cap 순서 / tail 순서 | 720 / 15 / 24 / 2 |
| candidate automatic minimum 대상 item | 25,920 |
| 기준선 payload floor 위반 | 3,648 |
| candidate payload floor 위반 | 0 |
| item frame과 span row 합의 최대 차이 | 0.000225 CSS px |
| source-order 의미 geometry 차이 | 0 CSS px |
| stretch의 non-auto track 최대 변화 | 0 CSS px |
| auto track stretch 최대 배분 | 130 CSS px |
| cap permutation group | 480개, 각 24개 순열 포함 |

Candidate item은 scrollable하지 않다. Eligibility 집계는 span이 `fit-content()` 또는 `auto` minimum track을 포함하는 item만 대상으로 하며, verifier는 각 payload frame을 따로 검사한다. 기준선 위반 수와 candidate 0건은 이 고정 fixture에만 해당한다. 4배 확대 화면은 layout 관측 뒤 얻은 Chrome fixture의 한 장면이다.

## 재현·산출물

- [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-four-fit-auto-minimum-precomparison.html)
- [Agent Browser 캡처 스크립트](./capture-chrome.mjs) · [캡처 환경 manifest](./capture-manifest.json) · [독립 matrix verifier](./verify-chrome-matrix.mjs)
- [전체 관측 NDJSON](./chrome157-observation.ndjson) · [검증 요약](./verification.json) · [대표 화면](./chrome157-four-fit-auto-minimum-sample.png)
- [Agent Browser 수명 기록](./agent-browser-lifecycle.json) · [SHA-256 manifest](./SHA256SUMS)

캡처는 전용 session `spinon-c114-four-fit-row-span-auto-minimum-20261111`에서 실행했다. 캡처 script는 `opened=true`, `closed=true`를 기록했다. 종료 후 session 목록에 Spinon session은 없고 무관한 HWP session은 보존됐으며, 전용 session 이름으로 process를 검색했을 때 결과가 없었다. Terminal Browser는 사용하지 않았다.

## 한계

이 결과는 Chrome CSS 기준 관측이며 Spinon RuntimeSession 결과가 아니다. Android/iOS 앱 동작, PixelCopy/Metal 표면 출력, compositor 이후 픽셀 일치, 실기기 지원, WPT 적합성, 임의 CSS Grid admission 또는 C11.4/C11 완료를 입증하지 않는다. 계획과 구현 상태는 분리하며 공식 `78/214`를 유지한다.
