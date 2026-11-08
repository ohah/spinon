# C11.4 repeated intrinsic tail permutations 사전 비교

이 근거는 `fit-content()`로 시작하고 `min-content` 2개, `max-content` 2개, `auto` 1개를 뒤에 두는 정확한 여섯-member `grid-auto-rows` 목록의 후행 순서 30개를 다룬다. PR #1284의 기존 순서 하나도 포함해 120개 비교 scene을 만들었다.

## 규범과 독립 기대값

[CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)은 `grid-auto-rows`를 하나 이상의 `<track-size>` 목록으로 정의하고, 여러 track size가 있으면 필요한 만큼 패턴을 반복한다고 설명한다. [§12.8](https://www.w3.org/TR/css-grid-2/#algo-stretch)은 `normal|stretch`에서 남은 양의 definite free space를 auto max-track sizing function을 가진 track 사이에 균등 분배한다.

Fixture는 definite 40×100px border-box Grid, 40px column, 10px explicit row, 8 implicit items, zero gaps, row auto-flow, visible overflow와 fixed-px payload `[20,10,12,6,6,6,60,6]px`를 사용한다. 첫 track은 direct `fit-content(20px|20%)`이며 definite 100px height에서 두 cap은 20px이다. 후행 multiset의 고유 순서는 30개이고 px/% × start/stretch를 결합해 120 scenes를 관찰했다.

독립 계산은 각 implicit row의 반복 list member를 선택하고, fit-content row에는 `min(payload,20px)`, 나머지 fixed-px payload row에는 payload height를 사용했다. Track 합계는 `start`에서 96px, `stretch`에서 100px이다. Auto member가 후행 첫 위치인 순서에서만 여덟 implicit rows 안에 auto track이 두 번 나타나며, 그 외 순서는 한 번 나타난다. `stretch`는 남은 4px을 해당 auto occurrence에 균등하게 더한다. 이 고정-payload 입력에서는 min/max intrinsic contribution이 모두 payload height와 같으므로, 이 사전 비교는 임의 content의 min/max sizing 동등성을 주장하지 않는다.

독립 예상값은 [`expected.json`](./expected.json)에 저장했고, fixture를 생성한 뒤 Chrome 관측과 독립 비교했다. 예상값은 Chrome이나 Spinon에서 가져오지 않는다.

## Agent Browser Chrome 157 관측

- 브라우저: Agent Browser HeadlessChrome 157.0.0.0 / macOS arm64
- viewport: 1280×633 CSS px, DPR 1
- Fixture: [`root-html-grid-repeated-intrinsic-tail-permutations-precomparison.html`](/spinon/tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-tail-permutations-precomparison.html)
- 생성기 소스: `tools/css-reference/build-c11-4-repeated-tail-permutation-precomparison.mjs` (저장소 경로)
- raw observation: [`chrome-observation.json`](./chrome-observation.json)
- 독립 비교 보고서: [`chrome-verification.json`](./chrome-verification.json)
- 비교기 소스: `verify-observation.mjs` (같은 근거 디렉터리의 저장소 파일)
- 화면: [`chrome-agent-browser-157.png`](./chrome-agent-browser-157.png)

120/120 scene과 9,480 track·item·payload coordinate check가 모두 통과했고 최대 차이는 0 CSS px였다. computed `grid-auto-rows` member order, 여덟 implicit item의 두 번째 cycle, px/% cap, `start|stretch`, fit-content overflow를 확인했다. 이 결과는 계획된 고정 fixture의 Chrome 사전 비교이며 Spinon 구현 결과나 전체 CSS Grid 적합성이 아니다.

## 계획 단계 한계

계획 검토 20개를 별도로 기록한 뒤에만 구현을 시작한다. 일반 CSS admission, 다른 six-member multiplicity, fit-content 위치 변경, arbitrary text/content intrinsic contribution, WPT, full Grid sizing, C11.4/C11 완료는 범위 밖이다.
