# C11.1 Root HTML Grid `minmax()` 사전 비교 · 2026-11-03

## 자료와 기준

- 고정 입력: [`root-html-grid-minmax-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-minmax-precomparison.html)
- 입력 SHA-256: `74171bb89ebfcb193c50e0cdb0f0f5c115d2e9311f54e51eb3b871576cdab88d`
- 표준: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.6 Maximize Tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grow-tracks), [§11.7.1 Find the Size of an fr](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-find-fr-size)
- 기대값: [`expected.json`](./expected.json), CSS Grid 규범을 적용한 독립 산술
- 관측값: [`chrome156-observation.json`](./chrome156-observation.json), Agent Browser HeadlessChrome 156.0.0.0
- 화면: [`chrome156-precomparison.png`](./chrome156-precomparison.png), SHA-256 `8041900696c7049d2b4c89e60422a486fb9f1ff17d659ae8be5862f24205f9be`

`minmax()`는 min보다 작지 않고 max보다 크지 않은 track 크기 범위를 정의한다. max가 min보다 작으면 max를 min까지 올리고, `<flex>`는 최대 함수로 허용되지만 최소 함수로는 허용되지 않는다. 사전 비교에서는 track 크기 함수와 Chrome의 used track 값을 독립 산술과 비교했다. Chrome 값은 교차 관측이지 규범 기대값을 정하는 oracle이 아니다.

## 양성 비교

16개 장면에서 fixed px 경계, 남은 공간 안의 최대화, 최소 크기 overflow, max<min 바닥 처리, 두 bounded track과 gap, definite-axis percentage, flex maximum, zero-fr, zero percentage basis, row percentage, 정수 repeat 확장을 관측했다. 각 Chrome used track은 `expected.json`과 일치했다. 최대 오차는 0 CSS px다.

`minimum-overflow`는 root 폭 30px에 columns `minmax(40px, 80px) 10px`을 둔다. Chrome의 used columns는 `40px 10px`; 두 번째 8px marker의 시작은 x=40px이고 끝은 x=48px다. marker는 root의 오른쪽 끝 x=30px보다 18px 바깥까지 보인다. overflow를 root bounds 안으로 자르지 않는 비교 기준이다.

## 음성·경계 대조군

`minmax(min-content, 80px)`와 `minmax(20px, auto)`는 유효 CSS이며 Chrome은 각각의 content/auto sizing을 수행한다. 이번 계획은 intrinsic/auto sizing까지 기본 Root HTML path로 확장하지 않고 별도 후속 범위로 둔다. `minmax(20px, -1px)`은 유효하지 않아 선언이 cascade에서 빠진다. Chrome의 8px used column은 item으로 만들어진 implicit auto track이므로 minmax 동작 결과로 오인하지 않는다.

## 제품 경로의 시작 상태

현행 `RuntimeRootHtmlTextPaintV1`의 Grid validator는 explicit direct `px`, `%`, `fr`를 허용하지만 모든 `ComputedCssGridTrackSize::MinMax`를 거부한다. 일반 typed projection은 `MinMax`를 `GridTrackSize::MinMax`로 옮기는 converter를 이미 갖고 있다. 따라서 제품 변경은 기본 Root HTML 검증·axis constraints·수치 범위 검증과 기존 projection/layout 연결에 한정하며 CSS parser나 새 FFI route를 만들지 않는다.

제품 runtime과 Android/iOS Simulator 결과는 아직 없다. 이 문서는 구현 완료나 CSS 지원 선언이 아니다.
