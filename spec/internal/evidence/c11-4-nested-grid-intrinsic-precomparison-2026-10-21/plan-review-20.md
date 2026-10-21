# C11.4 nested Grid item intrinsic contribution 계획 적대 검토 20개

대상은 [집중 계획](../../../../plan/c11-4-nested-grid-intrinsic.md), [Chrome 156 fixture](./chrome-nested-grid-oracle.html), [관찰 JSON](./chrome156-observation.json), [사전 비교 요약](./README.md), CSS Grid 2025 CRD §6.6·§12.5·§12.5.1이다. 구현 전에 서로 다른 실패 경로 20개를 검토했다.

| # | 실패 경로 | 계획의 차단 기준 |
| ---: | --- | --- |
| 1 | 직전 PR에 쓴 Chrome 155를 현재 기준으로 잘못 표기 | 이 실행의 실제 User-Agent는 `HeadlessChrome/156.0.0.0`이며 JSON과 PR 근거에 이 버전을 쓴다. |
| 2 | Ahem 지연 로드 후 fallback으로 geometry 확보 | `document.fonts.check('10px AhemFixture')` true를 기다린 실행만 관찰로 저장했다. |
| 3 | Chrome에 맞게 규범 기대값을 사후 조정 | 두-track 기대 65/65 및 one-track 134/0은 §12.5.1 계산으로 먼저 설명하고 CSSOM 관찰과 별도 기록한다. |
| 4 | Ahem text의 min-content와 max-content 폭을 혼동 | `XX XXXXXXX`의 70/100px과 `XXXXXX`의 60px 값을 fixture별로 분리했다. |
| 5 | 내부 Grid의 gap을 outer track 값에 두 번 더함 | nested width 134px은 내부 track 70+4+60으로 계산하고, outer 4px gap은 parent grid width에 한 번만 더한다. |
| 6 | outer span contribution에서 gap을 빼지 않아 67/67 기대를 작성 | 134px item contribution 중 4px outer gutter를 제외해 130px을 2개 parent track에 나눈다. |
| 7 | 한 track만 차지한 nested item이 빈 두 번째 track을 키움 | 기대값 `134/0px`, outer total `138px`로 별도 control한다. |
| 8 | `width:max-content`가 item의 definite width라고 오해 | 바깥 root intrinsic width와 nested item frame을 개별 비교하고 max-content sizing constraint를 plan에 명시한다. |
| 9 | 일반 nested Grid를 subgrid로 취급 | HTML에는 `subgrid`가 없으며 계획은 nested track을 parent로 공유하지 않는 일반 Grid에 한정한다. |
| 10 | inner child span이 outer item intrinsic path에 섞임 | `inner-span-diagnostic`은 화면 자료에만 남기고 acceptance와 app matrix에서 제외한다. |
| 11 | `inner-span-diagnostic`의 두 번째 implicit row를 놓침 | 관찰 JSON에 20/10px row와 34px height를 보존하고 이를 성공 기준으로 세지 않는다. |
| 12 | direct leaf input skip을 곧바로 버그로 판정 | non-leaf item의 input skip과 Taffy recursive sizing을 구별하고 실제 output/trace divergence를 재현한 뒤 수정 여부를 정한다. |
| 13 | parent contribution을 text source input으로 위장 | trace의 item ID는 nested Grid node로, leaf text input의 `grid_container`는 nested Grid ID로 각각 확인한다. |
| 14 | outer item query와 inner text query를 섞음 | 두 container의 Columns/Rows pass 및 item node ID를 별도 assertion한다. |
| 15 | track size만 맞고 child frame이 틀림 | root, nested Grid, 두 inner items 각각의 x/y/width/height를 확인한다. |
| 16 | definite flex control을 intrinsic min-content case로 계산 | width 120px·58/58px control은 별도 scene이며 definite flex behavior만 검사한다. |
| 17 | definite scene의 14px visual overflow를 contribution으로 더함 | overflow frame/sample 검사는 outer track sizing expectation과 분리한다. |
| 18 | background sample을 border stroke 검증으로 주장 | 계획은 해당 surface의 특정 배경 표본만 허용하고 모든 border stroke/compositor 결과는 제외한다. |
| 19 | 앱 fixture가 임의 DOM topology를 허용 | exact scene ID, parent relation, style/source revision allowlist와 unknown-case fail-closed를 요구한다. |
| 20 | fixture 결과를 일반 CSS 지원이나 C11.4 완료로 표현 | 전용 fixture-test 경계와 기본-artifact negative check를 유지하고 status checkbox/완료 수/계약 버전을 바꾸지 않는다. |

차단 결함은 발견하지 못했다. 단, 이 검토는 아직 Spinon의 해당 입력 수치가 Chrome과 일치함을 증명하지 않는다. 첫 구현 단계에서 Rust layout과 Taffy trace로 이를 확인해야 한다.
