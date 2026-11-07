# C11.4 fit-content/min-content/auto 남은 순서 계획 검토 20개

계획 및 사전 비교를 구현 전에 서로 다른 실패 관점 20개로 검토했다. 검토 대상은 `plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.md`, Chrome 157 capture, 독립 used-row 계산이다.

| # | 실패 관점 | 계획의 방어·판정 |
| --- | --- | --- |
| 1 | 기존 두 순서를 신규 목록 gate 때문에 거부하는가? | 기존 두 패턴을 계속 포함하고 회귀 fixture에 둔다. 통과. |
| 2 | 여섯 개 순열 중 신규 패턴 하나를 빠뜨리는가? | 세 member의 순열 6개를 열거하고 기존 두 개와 신규 네 개를 대조했다. 통과. |
| 3 | explicit row가 `grid-auto-rows` 반복 인덱스를 소비하는가? | explicit 10px row는 제외하고 첫 implicit item부터 순서가 시작되는 frame을 비교한다. 통과. |
| 4 | 네 번째 implicit row에서 반복 index가 초기화되는가? | 네 번째 implicit row의 first-member sizing을 네 item frame에서 확인한다. 통과. |
| 5 | definite height 대신 viewport 또는 border-box 외부값을 percentage basis로 쓰는가? | content-box height 100px와 px cap 동등 control을 계획에 고정했다. 통과. |
| 6 | px cap과 percentage cap이 다른 경로에서 계산되는가? | 신규 네 순서 모두 px/%를 Chrome 157에서 대조해 used rows와 frames가 같음을 확인했다. 통과. |
| 7 | fit-content가 cap보다 작은 payload에서도 잘못 고정되는가? | 두 auto-fit 순서에서 payload 10px, cap 20px scene을 포함한다. 통과. |
| 8 | fit-content cap이 34px nested payload가 있는 item의 track size와 overflow frame에 반영되는가? | cap을 20px, payload를 34px로 둔 장면에서 row는 20px, payload는 34px로 분리해 확인한다. 통과. |
| 9 | min-content track을 auto stretch member로 잘못 키우는가? | 각 ordering에서 intrinsic row와 auto row의 used sizes 및 item heights를 별도 비교한다. 통과. |
| 10 | 하나의 auto track에 free space를 잘못 분배하는가? | 첫 두 신규 패턴의 base 합 80/94px과 free space 20/6px을 따로 계산한다. 통과. |
| 11 | 두 auto tracks가 전체 공간을 각각 중복 수신하는가? | auto가 두 개인 두 패턴에서 total free 20/6px을 각각 10/3px으로 분배한다. 통과. |
| 12 | 소수 free space가 없는 이번 기대값을 일반 분배 규칙으로 과장하는가? | 계획은 정수 장면의 네 값만 주장하며 임의 fractional remainder 보장은 하지 않는다. 통과. |
| 13 | CSS Grid computed declaration을 used size로 착각하는가? | computed `grid-auto-rows`와 computed `grid-template-rows` used px를 별도 capture했다. 통과. |
| 14 | Grid container origin이 다른 이전 section의 y-offset에 오염되는가? | capture frame은 각 Grid의 local origin에서 뺀 좌표이며 Grid 자체는 40×100px다. 통과. |
| 15 | item frame만 맞고 nested payload overflow가 틀리는가? | 모든 scene에서 네 item과 네 payload frame을 저장하고 비교한다. 통과. |
| 16 | 허용 순서에 `max-content`, duplicate intrinsic member, extra track이 섞이는가? | exact-length/one-of-each gate와 negative projection/runtime controls를 요구한다. 통과. |
| 17 | 음수·NaN·무한 cap 또는 calc/minmax가 새 경로로 통과하는가? | finite non-negative px/% 이외는 계속 fail-closed로 명시했다. 통과. |
| 18 | 다른 placement mode나 named-line/area profile까지 열리는가? | 기존 `RootHtmlFitContentAutoRowsV1`의 고정 tree/container shape를 유지한다. 통과. |
| 19 | 실패 revision이 이전/부분 render를 새 결과처럼 게시하는가? | atomic failure와 같은 session의 유효 revision recovery를 구현 gate로 둔다. 통과. |
| 20 | 네 fixture 장면을 일반 Grid/CSS/WPT/기기 parity로 확대 주장하는가? | 범위·검증 증거와 제외 주장을 명시적으로 제한했다. 통과. |

이 검토는 구현/runtime failure review 20개 또는 PR diff review 30개를 대신하지 않는다.
