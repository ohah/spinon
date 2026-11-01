# C11.1 parsed Root HTML Grid template areas 사전 비교

이 자료는 다음 제한 구현 후보의 고정 입력과 독립 geometry 기대값을 구현 전에 기록한다. 기준 source는 `tests/fixtures/css/c11/root-html-grid-template-areas-precomparison.html`이고, Chrome 관측은 `chrome-observation.json`, 규범 기반 산술은 `expected.json`에 보존했다.

## 고정 입력

- Root: `display:grid`, `120×70px`, columns `[40,80]px`, rows `[20,50]px`, gap 0.
- Areas: `"header header" "side content"`.
- 세 직접 자식은 빈 block `<div>`이며 `grid-area: header|side|content`로 배치한다. Source order는 `content, header, side`다.
- viewport `1280×633 CSS px`, device scale factor 1.

## 독립 계산

Track 합으로 root 크기를 계산한다: `40 + 80 = 120px`, `20 + 50 = 70px`. 각 named area의 직사각형 cell 범위는 HTML source의 두 문자열에서 직접 세어 고정했다. 따라서 header는 `(0,0,120,20)`, side는 `(0,20,40,50)`, content는 `(40,20,80,50)`이다. 각 계산값은 Chrome 관측을 사용해 역산하지 않았다.

## Agent Browser 관측

Agent Browser HeadlessChrome 156에서 root computed tracks·areas와 각 item의 computed `grid-area`, 네 placement longhand, root-relative `DOMRect`, computed background color를 읽었다. Original DOM order를 역순으로 바꾸어도 각 semantic item's frame은 그대로였다. 아래 화면은 원래 DOM 순서의 배치 관측이다. `verify-precomparison.mjs`는 고정 expected JSON과 Chrome observation의 item/track/frame 값을 기계 대조했다. 결과는 `verification.json`에 보존했다. 이는 Chrome baseline이며 Spinon 구현이나 WPT 적합성 결과가 아니다.

![Agent Browser HeadlessChrome 156 named-area 배치](./chrome-156-template-areas.png)

Agent Browser는 관측 뒤 `agent-browser close --all`로 닫았다. 종료 직후 `ps` 검사에서 `agent-browser`와 `chrome-headless-shell` 프로세스가 없었다.

고정 입력, expected/observation JSON, Chrome screenshot과 verifier의 SHA-256은 `SHA256SUMS.txt`에 있다.

## 한계

이 baseline은 유효한 고정 2×2 area template의 geometry만 확인한다. 빈 cell, 중첩/중복 item, 누락·미등록 area, implicit track, auto placement와 일반 CSS 지원을 검증하지 않는다.
