# C11.4 zero-length percentage `calc()` auto-repeat 사전 비교

`repeat(auto-fill | auto-fit, minmax(calc(0px + 10%), 1fr))`를 definite 120px 축에서 12px 고정 minimum 대조군과 비교했다. 이 자료는 Chrome 156 관측과 독립 규범 계산이며 Spinon runtime 결과나 WPT pass가 아니다.

## 기준과 입력

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax) 및 [repeat-to-fill](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat).
- 비교 입력: [고정 HTML fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-zero-percent-calc-c114-precomparison.html). columns/rows, auto-fill/auto-fit 각각에서 `calc(0px + 10%)`와 `12px` control을 비교했다. grid의 content box는 양 축에서 120px이고 padding/border/gap은 0이다.
- 브라우저: Agent Browser가 구동한 Chrome `156.0.8078.48`, DPR 1, viewport `650×280`.
- 원본: [Chrome 관측 JSON](./chrome-observation.json), [규범 독립 계산](./spec-derived-expectations.json), [Agent Browser 캡처](./chrome156-agent-browser.png), [SHA-256](./sha256.txt).

## 독립 기대값과 관측

§7.2.1의 track minimum은 0 이상의 length-percentage를 허용한다. 지정된 축이 definite 120px이므로 10%는 12px이다. §7.2.3.2의 auto-repeat 계산에서 각 12px minimum으로 `floor(120/12)=10`개가 들어간다. auto-fill은 `[12]×10`이고 item 폭/높이는 12px이다. auto-fit도 먼저 10개를 만들고 배치 후 빈 9개를 0px로 접으며, 남은 공간을 차지하는 첫 1fr track과 단일 item은 120px이다.

Chrome 관측은 네 calc scene에서 각 축별 12px 10개 및 `[120,0,0,0,0,0,0,0,0,0]`을 반환했다. 각 결과는 대응하는 numeric control과 track list 및 item frame이 일치했다. `CSS.supports()`는 calc auto-repeat 문법을 지원한다고 보고했다.

zero-size basis, indefinite/cyclic basis, nonzero gap, implicit placement, general CSS admission과 Spinon 실행은 이 사전 비교에 포함하지 않았다.
