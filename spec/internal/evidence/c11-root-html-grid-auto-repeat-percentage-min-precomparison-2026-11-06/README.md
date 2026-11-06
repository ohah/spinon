# C11.1 Root HTML auto-repeat percentage minimum 사전 비교

## 범위

이 사전 비교는 기본 parsed Root HTML의 `RootHtmlAutoRepeatV1`에 `minmax(<percentage>,1fr)` minimum을 추가하기 전 기준을 고정한다. Chrome 157 Agent Browser는 CSS syntax와 실제 used tracks/frame을 관찰하는 구현 oracle다. 독립 기대값은 CSS Grid 2025 CRD의 repeat-count·definite-percentage·flex-track 절차와 아래 입력에서 별도로 계산했다.

기존 범위는 한 축의 auto-fill/auto-fit, 1~2개의 unnamed `minmax(<px>,1fr)` track, 다른 축의 단일 fixed-px track이다. 이번 계획은 fixed-px와 C11.4 opt-in fixture의 percentage tests를 재사용해 완료로 보지 않고, default Root HTML 경로를 별도로 연결한다.

## 고정 입력과 관측

- [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-percentage-min-precomparison.html) — 6개 scene, Ahem/font dependency 없음.
- [독립 기대값](./expected.json) — authored input의 fixed axis·percentage·gap·pattern size를 산식에 넣어 계산했다.
- [Chrome 157 observation](./chrome157-observation.json) — Agent Browser HeadlessChrome 157.0.0.0, DPR 1, `CSS.supports(...)=true`.
- [Agent Browser screenshot](./chrome157-agent-browser.png) — CSS layout 결과의 시각 자료.
- [검증기](./verify-observation.mjs) — scene ID, computed track 수와 ideal used-size/frame 허용 오차를 대조한다.
- [SHA-256](./SHA256SUMS.txt) — 입력·expected·observation·screenshot의 digest.

| Scene | 독립 count / tracks | Chrome 157 observation |
|---|---|---|
| columns auto-fill, 10% minimum | 8 / 8 × 20.625px | 8 × 20.625px; item origins `[0,25.625,51.25,76.875]` |
| columns auto-fit, 10% minimum | 8 / `[200,0×7]px` after occupancy | `[200,0×7]px` |
| columns auto-fill, 0% minimum | 34 / ideal 34 × 1.029412px; count uses a 1px minimum | 34 tracks; item origins `[0,6.015625,12.046875]` |
| columns auto-fill, `[10%,5%]` pair | 5 / ideal `[20,16.4]px × 5` | 10 tracks; per-track 1/64px rounding; frames within 0.02px |
| rows auto-fill, 20% minimum | 4 / 4 × 37px | 4 × 37px; item y `[0,41,82,123]` |
| columns auto-fill, 120% minimum | 1 / 240px | 240px track; 40px visible root overflow |

2-track flex sizing은 20px minimum track을 freeze하고 10px minimum track들이 남은 공간을 나눈다. Chrome은 ideal 16.4px share를 1/64px increments로 직렬화한다. Browser precision은 비교 허용 오차에만 반영하고 repeat count, track count, authored order는 exact하게 비교한다.

## 판정과 한계

0% 장면은 track sizing에 쓰이는 0%와 반복 횟수 계산의 1px minimum을 구분한다. 반복 횟수 규칙에 따라 34 tracks가 만들어지지만 percentage minimum의 resolved value는 0px다. 나머지 장면은 양수 percentage minimum, `auto-fit`, 두-track pattern, row-axis basis, 100% 초과 overflow를 확인한다.

6개 장면 모두 독립 count·geometry model과 일치한다. 이 결과는 Spinon 동작, WPT, 일반 CSS 입력 허용, 임의 auto-repeat 문법, indefinite percentage, hardware GPU 또는 실기기 출력을 검증하지 않는다. expected JSON은 산술 모델이며 browser 값을 복사하지 않았다.
