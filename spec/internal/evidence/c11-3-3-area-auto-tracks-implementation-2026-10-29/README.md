# C11.3.3 implicit track 구현 비교 근거

## Chrome 156 보충 oracle

- 기준: Agent Browser 0.31.1, HeadlessChrome 156.0.0.0, DPR 1, viewport 1280×633.
- 입력: [`grid-template-areas-auto-tracks-c1133-runtime.html`](../../../../tests/fixtures/css/c11/grid-template-areas-auto-tracks-c1133-runtime.html), SHA-256 `45e5f78ac66ea632d24797a02bb4c8d267e9ae9bf3dcfbfc43e8365cb314c701`.
- 관측: [`chrome-observed.json`](./chrome/chrome-observed.json), 5 scene·25 node. CSSOM used `grid-template-rows`·`grid-template-columns`와 root 기준 모든 frame을 보존한다.
- Screenshot: [`chrome-156-c1133-area-auto-tracks-runtime.png`](./chrome/chrome-156-c1133-area-auto-tracks-runtime.png), SHA-256 `ff9301d0e151a3d3b7305fd77bfe1ecc5034c0eccddb6225a71892deaa0ef7a6`.
- 자동 크기 scene은 root 50×30 CSS px다. Chrome used rows는 `10px 20px`, columns는 `20px 30px`이며, `align-content:start` profile에 남은 공간이 track 크기에 섞이지 않도록 입력 크기를 track 합과 맞췄다.
- 나머지 scene은 root 120×80 CSS px다. 명시 track이 없는 경우 rows `7px 7px`·columns `9px 9px`; 한 축 authored list가 짧은 경우 기존 authored 값 뒤에 auto track을 추가하고, area보다 긴 authored list는 보존한다.
- 원시 Agent Browser 응답은 [`agent-browser-eval-envelope.json`](./chrome/agent-browser-eval-envelope.json)에 보존한다. 캡처 뒤 `agent-browser close`를 실행했고 `pgrep -fl 'agent-browser|HeadlessChrome'` 결과는 비어 있었다.
- 최종 후보 archive의 기본/opt-in fixture 경계는 [`ffi-artifact-boundary.json`](./ffi-artifact-boundary.json)에 기록했다. 기본 static archive에는 C11.3.3 fixture 심볼·scene payload가 없고 fixture-test opt-in archive에는 둘 다 있다. 이는 macOS ARM64 static archive 근거이며 APK/IPA 전체를 증명하지 않는다.

## 비교 경계

- 기존 26 scene 기준은 Chrome 154 고정 reference와 연결된 채 보존한다. 새 다섯 scene만 위 Chrome 156 보충 oracle을 사용한다.
- 이 자료는 browser CSSOM·geometry oracle이며 Android/iOS GPU pixel 일치 근거가 아니다. 시뮬레이터 행렬과 surface pixel 결과는 각각 별도 evidence에 기록한다.
- 기존 `area-track-count-mismatch` scene은 Chrome 154 pinned reference의 `40px 120px 0px` columns 및 마지막 cell `(160, 0, 0, 60)` 기준을 계속 사용한다.

## 검토

- [구현·런타임 실패 관점 20개](./implementation-runtime-review-20.md)
- [별도 변경 diff 실패 관점 20개](./pr-diff-review-20.md)
- [PR #807 적대적 변경 검토 30개](./pr-review-30.md)
