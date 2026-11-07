# C11.4 fit-content/min-content/auto list 계획 검토 · 실패 관점 20개

**결과:** 구현 전 계획 검토를 마쳤다. 계획은 세 개 Chrome 157 scene과 독립 기대값을 고정하고, typed allowlist·stretch 분배·overflow·실패 복구를 구현 관문으로 둔다. 검토상 구현 착수 blocker는 없다. 구현/runtime 검토에는 아래 입력을 재사용하지 않고 새 실패 관점을 만든다.

| # | 독립 실패 경로 | 계획이 요구하는 판정·검증 |
| --- | --- | --- |
| 1 | 사전 비교 fixture가 변경되거나 Chrome capture가 다른 source hash에서 생성됨 | fixture hash와 Agent Browser capture metadata가 일치하지 않으면 기준 결과를 거부하고 재캡처한다. |
| 2 | computed `grid-auto-rows`가 지정한 세 track과 다름 | 두 지원 order의 typed value와 원본 순서를 검사하고, computed value가 다르면 projection 단계에서 거부한다. |
| 3 | 첫 implicit row가 list의 두 번째/세 번째 member로 시작함 | explicit 10px row 뒤 첫 item이 fit-content entry를 사용하고 이후 list 순서대로 돈다는 frame을 검사한다. |
| 4 | 네 번째 implicit row에서 pattern이 순환하지 않거나 잘못된 index로 돌아감 | fourth item의 6px payload가 네 번째 implicit track의 fit-content cap에 맞는지 확인한다. |
| 5 | `min-content`가 row item의 contribution 대신 max-content 값을 사용함 | 34px 및 10px payload 두 사례에서 min-content row의 used size가 각각 contribution과 같은지 비교한다. |
| 6 | `align-content:stretch`가 min-content track까지 늘림 | 두 order의 min-content row가 각각 34px/10px로 유지되고 auto row만 extra space를 받아야 한다. |
| 7 | `auto` track의 max-content contribution 누락/과소 산정 | 10px와 34px payload의 auto base를 사용한 뒤 정확히 남은 definite free space를 배분한다. |
| 8 | positive free space를 분배하기 전에 fit-content track을 stretch함 | 20% cap이 첫 track을 20px로 제한하며 auto row의 30px/54px와 다른 결과를 유지한다. |
| 9 | `fit-content(20%)`의 percentage basis가 width 또는 border box가 됨 | definite 100px block-size content basis로 20px cap을 계산한다. |
| 10 | `fit-content(120%)` overflow scene이 남은 공간 제한을 무시함 | first implicit track은 120px cap이 아니라 남은 40px에 맞고 전체 row 합은 100px이어야 한다. |
| 11 | 50px 반복 payload가 40px fit-content item/track에 잘리거나 root에서 clip됨 | 네 번째 implicit row가 fit-content member로 돌아오고 payload가 visible overflow로 Grid bottom보다 정확히 10px 아래까지 남아야 한다. |
| 12 | overflow visible 조건을 clip/hidden으로 대체하거나 overflow axis 하나만 검사함 | 두 축의 visible overflow와 payload bounds를 확인하고 hidden/clip 변형은 거부한다. |
| 13 | fit-content member가 없는 `[min-content, auto, 8px]` list까지 새 profile로 통과함 | existing fit-content anchor가 빠진 list는 property 진단과 함께 fail-closed한다. |
| 14 | `min-content` 누락 또는 중복을 허용해 list pattern shape가 모호해짐 | 두 exact order 외에 missing/duplicate min-content/auto와 다른 list length를 projection 및 layout에서 거부한다. |
| 15 | `max-content`, `minmax()`, `fr`, `calc()`가 generic intrinsic flag로 함께 열림 | typed allowlist가 위 두 pattern만 허용하는지 확인하고 모든 unmodeled track member를 거부한다. |
| 16 | 음수·NaN·무한 `fit-content()` cap이나 잘못된 percentage 계산이 통과함 | finite non-negative guard와 percentage resolution overflow 검사에서 게시 전에 거부한다. |
| 17 | positive free space가 없거나 definite height가 없어도 stretch 값을 기대값으로 강제함 | indefinite height와 zero/negative free-space controls를 별도로 거부/비교하고 positive-scene 결과를 재사용하지 않는다. |
| 18 | child가 text/image/nested Grid/span/positioned로 바뀌어도 payload fixture가 성공함 | exact 5-child DOM and nested fixed-px topology, static placement, auto placement만 허용한다. |
| 19 | 변경이 일반 Runtime profile, 다른 Root HTML profile 또는 explicit track semantics를 넓힘 | style projection과 layout unit test에서 untouched profile negative controls를 유지하고 exact owner/mode gating을 검사한다. |
| 20 | stale revision 또는 invalid style 뒤 partial frame/draw가 남고 다음 revision도 회복하지 못함 | invalid revision output 부재와 같은 RuntimeSession의 다음 valid revision 완전 복구를 확인한다. |

구현 관문에 Chrome/independent 비교, 두 track 순서, pattern cycle, error atomicity/recovery와 다른 profile negative controls가 각각 명시되어 있어 계획 검토를 통과한다. 실제 구현 결함 유무는 별도 구현/runtime failure review로 판단한다.
