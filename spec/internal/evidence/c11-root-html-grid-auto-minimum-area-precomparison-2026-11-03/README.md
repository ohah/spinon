# C11.1 Root HTML Grid `minmax(auto, fr)` named-area 사전 비교

이 기록은 기존 Root HTML named-area profile과 제한 `minmax(auto, fr)`의 상호작용을 구현 전에 고정한다. 기대값은 CSS Grid Level 1 2025-03-26 CRD의 track sizing 규칙에서 먼저 계산했고, Chrome은 교차 확인으로만 사용했다.

## 고정 기준

- 규범: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1, §11.5, §11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions).
- 입력: [고정 HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-area-precomparison.html).
- 독립 계산: [expected.json](expected.json).
- Chrome 156 관측: [chrome-observation.json](chrome-observation.json).
- 화면: [Agent Browser screenshot](chrome/chrome156-agent-browser.png).
- Fixture SHA-256: `c485826e6d35bcfb4504b89fab9c0986c2ff36af38129070409bec466ab015e2`.
- Browser: Headless Chrome 156.0.0.0, Agent Browser 0.31.1, viewport 1280×633 CSS px, DPR 1.

## 계산 결과

고정 100px content axis에서 `minmax(auto, 1fr) 40px`는 80px item automatic minimum을 보존해 `[80px, 40px]`이 된다. Root border는 2px이므로 20px content-axis overflow 중 18px이 바깥으로 나온다. `minmax(0, 1fr) 40px` control은 남은 폭을 사용해 `[60px, 40px]`이 되고 root border 밖 overflow는 없다. `minmax(auto, 0fr)`는 같은 80px automatic minimum을 보존한다. 같은 입력을 columns와 rows 양축에 각각 적용했다.

Chrome의 used tracks, DOMRect, scroll extent가 독립 기대값과 일치했다. `expected.json`의 surface RGBA는 fixture에 선언한 CSS 색상에서 산출했고 screenshot은 시각 자료다. 관측은 browser oracle이며 Spinon 구현이나 WPT 적합성 결과가 아니다. 이미지를 캡처한 뒤 Agent Browser 세션을 닫았고 `pgrep` 후속 확인에서 브라우저 프로세스는 남지 않았다.
