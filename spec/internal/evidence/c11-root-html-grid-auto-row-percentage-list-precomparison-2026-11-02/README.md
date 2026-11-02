# C11 Root HTML percentage implicit row list 사전 비교

- 기준 구현은 Agent Browser가 제어한 HeadlessChrome `156.0.0.0`, DPR 1이다. 캡처가 끝난 뒤 브라우저 세션을 닫았다.
- fixture [`root-html-grid-auto-row-percentage-list-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-percentage-list-precomparison.html)은 complete HTML 문서다. CSS input 및 source SHA-256은 `aaf6ed5775c9d5137299fe1845c327d3233a129c7e7a1e8dd32319772ff32a02`다.
- 독립 기대값은 definite root height를 percentage basis로 곱하고, 첫 implicit row부터 track 목록을 순환한 뒤 row gap을 더해 계산했다. 100%를 넘는 값을 clamp하지 않으며 row gap을 percentage basis에서 빼지 않는다. 수치와 item frame은 [`expected.json`](./expected.json)에 고정했다.
- Chrome은 cycle의 used rows `20px 10px 20px 10px 20px`, mixed의 `20px 15px 30px 15px 30px`, bounds의 `20px 0px 120px`, definite-zero의 `0px 0px`를 관찰했다. 네 root 및 모든 item frame은 독립 계산과 0.02 CSS px 이내로 일치했다. Raw observation은 [`chrome156-observation.json`](./chrome156-observation.json), 화면 캡처는 [`chrome156.png`](./chrome156.png)다. 캡처 SHA-256은 `5b6e62274a0f2ba5892af44dae56c806ee1d3c06988ba1129bc87670c6658d02`다.
- 이 자료는 Chrome의 관찰과 고정 입력에 대한 계산값만 기록한다. Spinon 구현, Android/iOS, 표면 pixel readback, WPT 적합성은 검증하지 않는다.
