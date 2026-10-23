# C11.4 auto-repeat `calc()` 최소값 사전 비교

이 문서는 `repeat(auto-fill/fit, minmax(calc(절대 길이), 1fr))`의 구현 전 기준을 고정한다. 아직 Spinon 동작 결과가 아니다.

## 재현 입력과 환경

- 브라우저: Agent Browser 0.31.1에서 실행한 `HeadlessChrome/156.0.0.0` (macOS ARM64)
- 시각 자료: [oracle 시각화 HTML](./oracle.html)
- 정확한 geometry 입력: [고정 CSS fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-calc-minimum-c114.html)
- 관찰 JSON: [chrome-observation.json](./chrome-observation.json)
- 독립 기대값: [spec-derived-expectations.json](./spec-derived-expectations.json)
- columns 화면: [chrome156-agent-browser.png](./chrome156-agent-browser.png)
- rows 화면: [chrome156-agent-browser-rows.png](./chrome156-agent-browser-rows.png)
- 원본 파일 hash: [sha256.txt](./sha256.txt)
- 관찰 JSON은 고정 CSS fixture를 대상으로 하며 viewport 1280×633 CSS px, DPR 1과 두 축 여섯 장면을 기록했다. PNG는 같은 scene 결과를 설명하는 oracle 시각화 페이지에서 캡처했다.

## 규범 기대

CSS Grid 2025 CRD §7.2.1과 §7.2.3.2의 `<fixed-size>` 문법은 고정 최소값과 flexible 최대값 조합을 허용한다. 184px definite content box에서 `minmax(calc(20px + 20px), 1fr)`의 반복 최소 폭은 40px이다. 반복 수 계산은 네 track을 고르고, 남은 24px을 네 `1fr` track에 나눠 used width `[46,46,46,46]px`를 만든다. `auto-fit`에서는 배치 뒤 빈 track 세 개와 인접 gutter가 collapse되고, gap 0 조건에서 유일한 occupied track이 184px을 차지한다.

## Chrome 관찰

columns와 rows의 `calc()` 장면은 각 고정값 control과 동일하게 `[46,46,46,46]px`을 계산했다. 각 `auto-fit` 장면은 `[184,0,0,0]px`을 계산했다. 고정 fixture에서 root와 모든 child의 Chrome frame도 JSON에 보존했다. 이는 oracle 결과이며 Spinon 결과로 해석하지 않는다. 이 비교는 CSS box geometry와 used track만 관찰한다. 실제 앱, 표면 픽셀, WPT 적합성은 검증하지 않았다.

`chrome-observation.json`의 root/item 좌표는 viewport 기준이고, `spec-derived-expectations.json`과 계획 표의 item 좌표는 각 root 원점 기준이다.

## 범위

다음 구현 slice는 pure absolute-length `calc()` 최소값만 다룬다. 퍼센트가 포함된 calc, intrinsic/auto 최대값, 음수·비유한 계산 결과, 여러 track 반복 pattern, 이름 있는 line, 양축 동시 auto-repeat + template areas의 교차 조합은 별도 범위다.
