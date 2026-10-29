# C11.3.3 area로 생성되는 implicit track · 사전 비교

## 기준

- 규범 출처: [CSS Grid Layout Module Level 1, 2025-03-26 CRD, §7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas-property), §7.3.2, §8.5. §7.3은 area 문자열마다 row 하나, 각 cell마다 column 하나를 정의한다. Grid에 explicit track이 없으면 필요한 행·열이 implicit하게 생성되고 `grid-auto-rows`/`grid-auto-columns`로 크기가 정해진다.
- 브라우저: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, DPR 1, viewport 1280×633.
- Fixture: [`grid-template-areas-auto-tracks-c1133-precomparison.html`](../../../../tests/fixtures/css/c11/grid-template-areas-auto-tracks-c1133-precomparison.html), SHA-256 `20ecc14e09aa3ae4c123c2ac6cdf1ff5c41aee8304fe14a60e7e908141a26f7e`.
- 브라우저 원시 관측: [`chrome-observed.json`](./chrome/chrome-observed.json), SHA-256 `0f924251911884a7456661ea92fd23e1e277672204bc71239c4d6ece629a1867`.
- Screenshot: [`chrome-156-c1133-area-auto-tracks.png`](./chrome/chrome-156-c1133-area-auto-tracks.png), SHA-256 `2307837d898b0912ea36b1d5d02098e1ad7104f7195ade5d80f8b8cdbee2da09`.

## 판정

Authored track list가 없을 때 2×2 area matrix는 row 두 개와 column 두 개로 계산된다. `grid-auto-rows:7px`, `grid-auto-columns:9px`에서는 used track이 rows `[7,7]px`, columns `[9,9]px`다. 첫 row만 `12px`로 authored한 경우 Chrome은 이를 유지하고 두 번째 row를 `7px`로 만든다. column 대조군도 authored `13px`와 생성된 `9px` column을 유지한다. `auto` generated track은 item 기여를 사용해 rows `[10,20]px`, columns `[20,30]px`가 된다. Area matrix가 2행·2열이어도 authored track이 세 개면 세 개 모두 유지된다.

기존 `area-track-count-mismatch` 거부는 Chrome 결과와 모순된다. Authored column `40px 120px` 두 개와 area column 세 개는 used columns `40px 120px 0px`를 만든다. 마지막 area는 폭 160px root의 x=160px에서 시작하고 최소 크기를 0으로 제한한 item은 폭 0px로 배치된다. 이는 유효한 area 확장이지 잘못된 declaration이 아니다.

## 고정 관측

Used track 목록과 root 기준 item 사각형은 `chrome-observed.json`에 있다. 이 자료는 CSSOM used track 직렬화와 item frame을 별도로 보존한다. Frame 비교 전 browser 페이지에서 각 장면을 쌓아 배치한 수직 offset을 제거했다. Screenshot은 화면 참고 자료이며 Spinon pixel 일치 근거가 아니다.

캡처 뒤 해당 Agent Browser session을 닫았고 일치하는 browser process가 남지 않았음을 확인했다.
