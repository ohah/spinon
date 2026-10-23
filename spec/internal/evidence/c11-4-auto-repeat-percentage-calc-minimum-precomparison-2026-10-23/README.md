# C11.4 definite auto-repeat percentage calc minimum 사전 비교

이 문서는 `repeat(auto-fill | auto-fit, minmax(calc(20px + 10%), 1fr))`가 definite 112px axis에서 만드는 고정 CSS box geometry를 Chrome 156과 독립 규범 계산으로 비교한다. Spinon runtime 결과는 포함하지 않는다.

## 입력과 환경

- 규범 기준: CSS Grid Layout Module Level 1, 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), CSS Values and Units Level 4 [§5.6, §10](https://www.w3.org/TR/css-values/). `<fixed-breadth>`는 `<length-percentage>`이고 CSS math function은 해당 type이 허용되는 곳에서 사용할 수 있다. Column percentages는 inner inline size, row percentages는 inner block size에 상대적이다. Auto-repeat은 fixed-size를 요구하며, definite preferred/max size에서는 fixed sizing function과 gap을 사용해 가장 큰 fitting repetition count를 고른다.
- 브라우저: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, macOS ARM64
- viewport 1280×633 CSS px, DPR 1
- 정확 입력: [fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-percentage-calc-minimum-c114.html)
- 실제 관찰: [chrome-observation.json](./chrome-observation.json)
- 독립 기대값: [spec-derived-expectations.json](./spec-derived-expectations.json)
- 화면: [chrome156-agent-browser.png](./chrome156-agent-browser.png)
- SHA-256 목록: [sha256.txt](./sha256.txt)

## 독립 기대값

각 fixed axis의 inner size는 112px이다. `calc(20px + 10%)`의 minimum은 `20 + 0.10 × 112 = 31.2px`이다. `floor(112 / 31.2) = 3`이므로 auto-fill은 3개 track을 사용한다. `1fr` maximum이 남은 공간을 채우므로 이상적인 used track size는 각각 112/3px이다. `auto-fit`도 먼저 세 번 반복하고 placement 뒤 비어 있는 두 track을 0px로 collapse한다. Numeric `31.2px` control의 결과는 calc case와 같아야 한다.

## Chrome 관찰

Columns와 rows 모두 calc와 fixed control의 auto-fill tracks는 `[37.328125, 37.328125, 37.328125]px`이고 item frame도 일치했다. 이상적 `112/3`과의 차이는 각 0.005208… CSS px로 1/64 이내다. 두 auto-fit scene은 `[112,0,0]px` track과 하나의 112px item을 가졌다. `chrome-observation.json`에는 전체 root/item viewport frame이 있다.

이것은 Chrome 156 oracle이지 Spinon 결과나 WPT pass가 아니다. 사용자 작성 CSS의 일반 허용, indefinite/cyclic percentage, nonzero gap, 표면 픽셀, Android/iOS runtime은 비교하지 않았다.
