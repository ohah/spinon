# C11.4 nested Grid intrinsic contribution 사전 비교

**상태:** Agent Browser에서 `HeadlessChrome/156.0.0.0` 기준 장면을 관찰했다. 이는 구현 비교를 위한 browser 자료다. Spinon layout이나 앱, Android/iOS 시뮬레이터 실행 결과는 아직 없다. 계획은 [C11.4 nested Grid item intrinsic contribution](../../../../plan/c11-4-nested-grid-intrinsic.md)에 기록한다.

## 입력과 환경

`chrome-nested-grid-oracle.html`은 바깥 Grid의 direct item 하나를 일반 nested Grid로 두고, 내부 Grid의 direct item 두 개에는 저장소 Ahem 글꼴을 사용한다. Subgrid는 선언하지 않는다. 글꼴 준비 후 `document.fonts.check('10px AhemFixture') === true`를 확인했다. Agent Browser의 실제 User-Agent는 JSON에 저장했다. CSSOM track 값, 각 node의 `getBoundingClientRect()`, scrollWidth를 같은 실행에서 캡처했다.

| 장면 | Chrome 156 바깥 tracks | 바깥 Grid | nested Grid와 내부 tracks | 판정 용도 |
| --- | --- | --- | --- | --- |
| `span-two` | `65px 65px` | `134×20px` | `134×20px`, `70px 60px` | 두 min-content track을 span하는 nested item |
| `one-track` | `134px 0px` | `138×20px` | `134×20px`, `70px 60px` | 한 parent track에 놓인 nested item과 빈 track |
| `definite-flex-control` | `58px 58px` | `120×20px` | `120×20px`, `70px 60px` | definite flex-track 폭과 intrinsic contribution을 구분; 내부 `b` 배경 영역은 바깥 Grid 오른쪽으로 14px 넘는다 |
| `inner-span-diagnostic` | `33px 33px` | `70×34px` | `70×34px`, `60px 6px`, rows `20px 10px` | 내부 item span/추가 행이 결과에 섞이는 것을 보여주는 진단 전용; acceptance에서 제외 |

fixture의 `XX XXXXXXX`는 Ahem 10px 기준 가장 긴 단어 폭 70px이며 max-content 폭은 100px이다. `XXXXXX`의 고정 max-content 폭은 60px이다. 내부 Grid의 `min-content max-content` tracks와 4px gap은 관찰된 134px border box에 맞는다. 바깥 두 min-content track의 span 분배는 gap 4px을 제외한 130px을 두 track에 나눠 각각 65px을 만든다. 한 track 장면에서는 같은 134px contribution이 첫 track에 들어가며, 빈 두 번째 track과 4px gap 때문에 바깥 intrinsic width는 138px이다. 이 계산은 계획의 규범 기대값이며 Chrome CSSOM 문자열을 규범 기준으로 승격하지 않는다.

## 규범 교차 확인

[CSS Grid Layout Module Level 2, 2025 CRD §12.5](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/#algo-content)는 intrinsic track sizing에서 단일-track item을 먼저 다룬 뒤 span item의 contribution을 span 길이 그룹별로 분배한다. §12.5.1은 계획된 증가량을 track별로 모아 item 순서에 따른 결과 차이를 피하도록 한다. §6.6은 Grid item의 automatic minimum size에 대한 예외를 정의한다. 이 slice는 일반 nested grid의 parent item contribution만 대상으로 하며 subgrid의 부모 track 공유를 구현하거나 검증하지 않는다.

## 산출물과 SHA-256

- 입력 fixture: [HTML](./chrome-nested-grid-oracle.html)
- 화면: [HeadlessChrome 156 Agent Browser capture](./headless-chrome156-nested-grid-oracle.png) — 1280×633, SHA-256 `484de00d54258c72f0f2cf82d9bb82e7ae2508c6edb77638fab6bcd26e7db14f`
- Ahem: `tests/fixtures/resources/c15/Ahem.ttf` — SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- 수치 관찰: [JSON](./chrome156-observation.json)
- 계획 자체의 실패 관점 검토: [20개 항목](./plan-review-20.md)

브라우저를 닫은 뒤 Agent Browser/HeadlessChrome 프로세스가 남지 않는 것을 확인했다. 이 capture는 text glyph paint가 아닌 브라우저 화면이며 native simulator surface, border stroke, 물리 디스플레이 표본을 대체하지 않는다.
