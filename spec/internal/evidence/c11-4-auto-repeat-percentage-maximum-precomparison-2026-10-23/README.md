# C11.4 definite percentage auto-repeat maximum precomparison

## 목적

계획은 auto-repeat의 definite maximum percentage 경로가 현재 opt-in profile에서 거부되는 결손을 다룬다. 이 폴더는 구현 전 기준만 보존한다.

## 기준

- 규범: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1·§7.2.3.2·§11.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).
- 브라우저: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, macOS ARM64, viewport 1280×633 CSS px, DPR 1.
- 입력: 고정 CSS와 시각화는 [oracle.html](./oracle.html), 관찰은 [chrome-observation.json](./chrome-observation.json), 독립 규범 계산은 [spec-derived-expectations.json](./spec-derived-expectations.json)에 보존했다. PNG [chrome156-agent-browser.png](./chrome156-agent-browser.png)은 1280px 너비의 full-page 캡처다.

## 독립 계산

content axis는 184px이고 gap은 8px다. 반복 수 n은 n×effective track size +(n−1)×8 ≤184를 만족하는 가장 큰 양의 정수다. 둘 다 definite인 경우 max를 min 이상으로 floor한 뒤 계산한다.

- min 16px / max 25% (=46px): 세 track [46,46,46]; auto-fit은 placement 뒤 [46,0,0]이다.
- min 25% (=46px) / max 20% (=36.8px): effective max는 46px, 세 track이다.
- min 25% (=46px) / max 0%: effective max는 46px, 세 track이다.
- min 16px / max calc(20px + 10%) (=38.4px): 네 track이다. Chrome의 layout unit으로 38.390625px가 관측되므로 ideal 대비 1/64px 이내를 허용한다.
- 각 기준은 columns와 rows, auto-fill/auto-fit, numeric fixed control과 비교한다. 단일 축 20 scene에 columns+rows 동시 percentage/calc 및 numeric control 8 scene을 더해 총 28 scene이다.

최종 관찰 28/28은 independent expectations와 track/frame tolerance 1/64 CSS px 내에서 일치한다. 초기 oracle 초안에 display:grid가 빠져 있었고 그 실행 결과는 폐기했다. 설정을 고친 뒤 final observation과 screenshot을 새로 생성하고 비교 검증했다.

## 한계

이는 고정 Chrome 관찰이며 Spinon 구현 결과, WPT, 일반 CSS admission, 앱 화면, OS compositor 이후 pixels를 증명하지 않는다. 계획과 공식 상태는 미완료로 유지한다.
