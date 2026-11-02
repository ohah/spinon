# C11 Root HTML fractional implicit row list 사전 비교

- 기준 구현은 Agent Browser로 제어한 headless Chrome `156.0.0.0`, DPR 1이다. 관찰 뒤 브라우저 세션을 닫았고 `session info`에서 `active=false`, `pid=null`을 확인했다.
- Fixture [`root-html-grid-auto-row-fr-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-fr-precomparison.html)의 SHA-256은 `062f22d9e04236d9b6723717139f01211792fabccefe21d30d58bcd819479b88`다. 독립 계산과 사전 판정값은 [`expected.json`](./expected.json)에 브라우저 관찰 전에 기록했다.
- 계산은 definite root block size에서 고정 explicit/px/percentage tracks와 row gaps를 빼고 남은 공간을 direct fractional implicit tracks에 flex factor 비율로 나눈다. 목록은 첫 implicit row부터 순환하며 percentage basis에서 row gap을 차감하지 않는다. 120% case에서는 percentage overflow를 clamp하지 않고, 남은 free space가 없는 `1fr`은 item minimum이 0이므로 0px로 계산한다.
- Chrome은 일곱 scene의 computed `grid-auto-rows`, used rows와 모든 root-relative item frame을 0.02 CSS px 이내에서 사전 기대값과 일치하게 관찰했다. `0fr` automatic minimum, subunit factor의 여유 공간 보존과 고정 item minimum overflow도 일치했다. Raw data는 [`chrome156-observation.json`](./chrome156-observation.json), screenshot은 [`chrome156.png`](./chrome156.png)이며 캡처 SHA-256은 `b05dc8f41c5bf8f566a23940feb10dafe624939af9a94144fb54f0da48660198`다.
- 이 자료는 Chrome의 독립 관찰과 고정 입력 계산만 기록한다. Spinon 구현, Android/iOS, 표면 pixel readback, WPT 적합성은 검증하지 않는다.
