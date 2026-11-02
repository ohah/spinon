# Root HTML `fr`·percentage + fixed implicit row 계획 실패 관점 검토 20개

검토 기준: [계획](../../../../plan/c11-root-html-grid-fr-percent-implicit-rows.md), [독립 기대값](expected.json), [Chrome 156 observation](README.md). 구현은 시작하지 않았다.

| # | 실패 관점 | 계획에서 고정한 방어·판정 | 결과 |
|---:|---|---|---|
| 1 | 기존 Root HTML CSS profile 대신 opt-in fixture가 성공을 내는가 | 기본 `new_runtime_gpu_with_root_html()` entry와 fixture profile 미사용을 요구한다. | PASS |
| 2 | projection만 열고 layout validation은 계속 px-only라 실제 scene이 거부되는가 | projection과 `RootHtmlImplicitRowsV1` layout contract를 둘 다 수정·검증 대상으로 둔다. | PASS |
| 3 | percentage가 indefinite axis에서 임의 basis를 얻는가 | matching root content-box axis definite 조건과 negative control을 요구한다. | PASS |
| 4 | `fr` share 계산에서 fixed implicit row 또는 gap을 빼지 않는가 | row `fr` scene의 60−15=45px 분배와 gap composition을 exact 기대값으로 둔다. | PASS |
| 5 | implicit auto-row breadth가 percentage/fr로 잘못 확장되는가 | implicit row는 단일 fixed non-negative px로 고정하고 list·percentage·fr는 거부한다. | PASS |
| 6 | direct fraction을 허용하면서 negative·NaN·infinity가 통과하는가 | finite non-negative breadth 검증과 negative matrix를 요구한다. | PASS |
| 7 | direct percentage 허용이 `minmax()`·`fit-content()` intrinsic breadth까지 연다 | direct typed variant만 열고 intrinsic/minmax/auto-repeat을 계속 reject한다. | PASS |
| 8 | line-name metadata나 named area placement가 우연히 함께 허용되는가 | no-area, empty line-name contract와 named line/area negative controls를 고정한다. | PASS |
| 9 | row `fr` size가 implicit row를 포함한 definite container space를 초과 소비하는가 | 독립 rows `[15,30,15]` 및 Chrome used rows를 비교한다. | PASS |
| 10 | percentage rows가 implicit row 또는 free space로부터 basis를 받는가 | definite 80px root의 25% basis를 각각 20px로 계산하고 25px remaining space를 기록한다. | PASS |
| 11 | percentage + gap이 root content-box 밖 track extent를 만드는 것이 잘못 거부/정규화되는가 | `[30,90]` tracks + 5px gap을 독립 판정하고 일반 clamp를 금한다. | PASS |
| 12 | root 바깥 item frame이 120px 경계로 clamp되는가 | 100px item x=35, right edge 135px의 15px overflow를 명시한다. | PASS |
| 13 | geometry overflow를 paint proof로 과장하는가 | frame·surface sample·screenshot 근거를 분리하고 pixel claim은 readback 때만 허용한다. | PASS |
| 14 | DOM reorder, tag/class/id 변경으로 auto-placement 성공이 바뀌는가 | ID/class independence와 source-order controls를 runtime matrix에 둔다. | PASS |
| 15 | `display:none` child가 sparse occupancy와 implicit row count를 바꾸는가 | hidden child를 placement에서 제외하는 positive control을 요구한다. | PASS |
| 16 | implicit track/resource cap 검사 뒤 부분 scene이 발행되는가 | track expansion 전 cap 및 partial frame 부재를 negative control로 요구한다. | PASS |
| 17 | CSS invalid 선언 fallback과 valid unsupported computed value가 같은 결과로 합쳐지는가 | cascade drop control과 projection diagnostic rejection을 따로 검사한다. | PASS |
| 18 | stale document/style/layout revision의 이전 frame이 신규 scene으로 보이는가 | revision equality 및 stale revision/partial-output rejection을 확인한다. | PASS |
| 19 | 이 조합이 fixture ABI·symbol·payload를 기본 app에 넣는가 | fixture API/feature 추가 금지 및 product artifact marker boundary 확인을 요구한다. | PASS |
| 20 | AVD screenshot만으로 surface, 실기기 또는 iOS compositor를 주장하는가 | 플랫폼 실행 종류·Android renderer·Metal readback 시점을 따로 기록하고 unrun gate를 명시한다. | PASS |

## 판정

계획은 분리된 integration gates, definite-axis 조건, overflow 관찰과 범위 밖 grammar를 구체화했다. 사전 비교는 Agent Browser HeadlessChrome 156 관측과 독립 계산이 일치한다. 구현 전에 수정해야 할 계획 결함은 발견하지 못했다. 이 검토는 구현 검토나 plan PR diff 30관점 검토를 대체하지 않는다.
