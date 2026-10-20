# C11.4 auto-repeat과 typed implicit track 사전 비교

## 조사 대상

PR #385는 제한된 auto-repeat과 signed numeric placement를 연결했다. 그러나 auto-repeat이 있는 `RuntimeBlockFlexGridTrackSizingV1`에서 `grid-auto-rows`와 `grid-auto-columns`의 목록은 fixed `px`만 Taffy로 전달한다. 같은 C11.4 profile은 auto-repeat이 없는 positive implicit tracks에서 이미 `GridTrackSize` typed 목록을 계산한다. 이 사전 비교는 두 경로 사이의 차이를 다음 별도 구현 후보로 삼을 수 있는지 조사한다.

## 독립 기대값

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.5–§7.6, §8.3, §11.3–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. §7.6은 `grid-auto-rows`와 `grid-auto-columns`를 `<track-size>+` 목록으로 정의한다. 목록은 explicit grid 뒤에서 첫 값부터 앞으로 순환하고, explicit grid 앞에서는 explicit grid에 가장 가까운 track에 마지막 값을 주고 역방향으로 순환한다. §11은 각 track의 base size와 growth limit, intrinsic contribution, definite free space, flex sizing을 계산한다.

[규범에서 도출한 고정 기대값](spec-derived-expectations.json)은 browser 관찰값으로부터 역산하지 않았다. 두 축의 각 110px definite content size와 `repeat(auto-fill, 20px)`은 5개 explicit track을 만든다. 기본 `-8 / -7`은 두 leading implicit track을 만들고, cycle case의 `-9 / -8`은 세 leading track을 만든다. `8 / 9`는 세 trailing track을 만든다. fixed `[24px, 36px]`과 mixed typed 목록은 leading 쪽에서 뒤로, trailing 쪽에서 앞으로 순환한다.

## Chrome 관찰

고정 fixture [typed-implicit-track-observation.html](typed-implicit-track-observation.html)을 Agent Browser의 HeadlessChrome 155에서 열고, [원 관찰 JSON](chrome155-observation.json)과 [화면](chrome155-cases-agent-browser.png)을 저장했다. 14개 case가 모두 보이는 viewport는 1280×1800 CSS px, DPR 1이다. Browser console error는 없었다.

Chrome은 fixed cycle case에서 leading `[24, 36]`과 explicit `[20 × 5]`를 반환했고, 세 leading·세 trailing case에서 leading `[36, 24, 36]`, explicit `[20 × 5]`, trailing `[24, 36, 24]`를 반환했다. Mixed typed cycle은 leading columns `[27.5, 0, 27.5]` 및 rows `[16, 0, 16]`, trailing columns `[0, 27.5, 0]` 및 rows `[0, 16, 0]`를 반환했다. 이 순서는 §7.6의 역방향/정방향 규칙과 맞는다. 별도 auto-fit scene에서는 repeat count 5를 유지하면서 occupied control이 있는 경우 빈 4 tracks, 없는 경우 빈 5 tracks를 collapse했다. `auto`, `min-content`, `max-content`, `minmax()`, `fit-content()`, flex, percentage와 `calc()` 각각의 computed track과 item frame도 JSON에 보존했다.

추가 `auto-fit-typed-gutter` scene은 definite content size 132px과 양 축 8px gap을 사용한다. Chrome CSSOM은 축마다 10개 track(leading 2, repeat 5, trailing 3)을 반환했고, control이 차지한 repeat track 하나를 제외한 네 repeat tracks는 0px로 collapse했다. 관찰된 columns는 `[38.5312, 33, 20, 0, 0, 0, 0, 0, 33, 0]px`, rows는 `[60, 18.1875, 20, 0, 0, 0, 0, 0, 18.1875, 0]px`였다. 8px gutter가 있는 이 한 장면의 세부 track 값과 item 좌표는 브라우저 관찰 JSON에만 둔다. 규범 기대 파일은 repeat count, occupied/collapsed 개수와 typed-list 순서만 기술하며 text metric이나 Chrome의 fractional rounding을 독립 기대값으로 삼지 않는다. 해당 값은 이 고정 환경의 관찰이며 Chrome 자체를 규범 기대의 대체물로 삼지 않는다.

## 판정과 한계

이 fixture는 한정된 positive/negative implicit placement에서 양 축 auto-repeat을 동시에 사용하며, grid-auto 목록의 타입별 sizing과 leading/trailing cycle 방향을 관찰한다. 기본 13개 case는 definite 110px 기준이며, gutter case 하나는 definite 132px 기준이다. 앱 작성자용 API나 전체 CSS Grid 지원, WPT 통과, 실기기·hardware GPU 결과를 선언하지 않는다. 모든 case에서 indefinite/cyclic percentage와 line names, template areas, unsupported `grid-auto-*` grammar는 제외한다. 실제 implementation route에는 기존 C11.4 profile·C11.2 profile 회귀와 Android/iOS Simulator 및 surface 표본 검증이 별도로 필요하다.

## 파일

- `typed-implicit-track-observation.html`: 고정 Chrome 입력과 두 방향의 list-cycle 화면
- `spec-derived-expectations.json`: 14개 case의 독립 규범 기대값과 비교 허용오차
- `chrome155-observation.json`: HeadlessChrome 155 user agent·viewport·used track·item frame
- `chrome155-cases-agent-browser.png`: Agent Browser 캡처
- `SHA256SUMS`: 사전 비교 원본과 화면의 SHA-256

Agent Browser session `spinon-c114-typed-leading-20261020`는 캡처 후 닫았다. 종료 확인에서 active session과 Agent Browser/Chrome process가 남아 있지 않았다.
