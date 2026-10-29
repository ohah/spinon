# C11.4 fit-content 성장 한계 경계 계획 실패 관점 검토 20개

이 검토는 구현 전에 고정 source, 규범 기대값, normalization과 fixture 경계를 서로 다른 실패 경로로 확인한다. 구현/runtime 리뷰나 PR diff 검토를 대체하지 않는다.

| # | 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | 다른 WPT revision의 assertion을 pin하는가 | revision `c999c58338ee1d223df5ad62e48be1202ced0d37`와 파일 hash를 고정했다. | 통과 |
| 2 | 파일 원문이 live upstream과 다를 수 있는가 | Agent Browser fetch 5,060 bytes, SHA-256 `828160…90736`이 기존 pinned hash와 일치한다. | 통과 |
| 3 | 목표 test 제목이 다른 assertion일 수 있는가 | `contentLargerThanArgument`, `smallArgument`, `largeArgument`의 정확한 test title과 CSSOM rows를 관찰했다. | 통과 |
| 4 | 세 scene 중 일부가 기존 runtime과 겹치는가 | 기존 fixture·계획에서 exact IDs를 검색했으며 세 WPT ID를 opt-in runtime scene으로 노출하는 구현은 없다. | 통과 |
| 5 | minimum이 자동 최소 크기로 다시 커질 수 있는가 | 원본 target item 모두 computed `min-height:0px`이며, 구현은 이를 세 scene에 보존해야 한다. | 통과 |
| 6 | `overflow:visible`이 아닌 scrollable item으로 baseline이 바뀌는가 | 원본 computed overflow는 세 경우 모두 `visible`이다. | 통과 |
| 7 | child max-content input이 사례마다 달라지는가 | 각 WPT child의 used block size와 max-content contribution은 100px이다. | 통과 |
| 8 | argument 40px을 content 100px과 혼동하는가 | `max(0,min(40,100))=40px`을 별도 기대값으로 고정했다. | 통과 |
| 9 | 30px smaller argument에서 40px 값이 복사되는가 | 30px case의 독립 계산과 CSSOM row는 `30px`이다. | 통과 |
| 10 | 200px larger argument를 무조건 used size로 쓰는가 | max-content가 100px이므로 `max(0,min(200,100))=100px`이다. | 통과 |
| 11 | root definite height 300px이 각 fit-content row를 stretch시키는가 | WPT source는 세 root 모두 300px이며 fit-content max sizing과 item/row 결과를 별도로 기록한다. | 통과 |
| 12 | width normalization이 row assertion을 바꾸는가 | Chrome 원본 width 1264px와 normalized width 60px에서 rows `40/30/100px`가 모두 동일했다. | 통과 |
| 13 | auto width가 runtime에 그대로 들어가 비결정성을 만드는가 | Runtime root와 explicit column을 60px로 고정하고 row invariance의 Chrome probe를 보존한다. | 통과 |
| 14 | implicit row를 explicit row로 대체하는가 | `grid-template-rows:none`과 `grid-auto-rows:fit-content(...)`를 입력 계약에 둔다. | 통과 |
| 15 | root/item/child frame을 서로 다른 revision에서 합칠 수 있는가 | 구현 관문에서 document/style/environment/layout revision과 draw/present identity를 결합한다. | 통과 |
| 16 | Taffy typed input과 CSSOM serialization이 뒤섞이는가 | projected argument, contribution, used row와 frame을 각각 관찰한다. | 통과 |
| 17 | child overflow를 item이나 row size로 잘못 기록하는가 | 세 case에서 child 100px, item/row 40/30/100px과 overflow 60/70/0px를 분리한다. | 통과 |
| 18 | 화면 표본이 overflow 위치를 놓칠 수 있는가 | 40/30 rows 바깥·100px child 안의 sample과 child 이후 root background를 요구한다. | 통과 |
| 19 | fixture가 일반 CSS 지원으로 확장될 수 있는가 | exact scene/root/feature allowlist와 unsupported author CSS 경계를 고정한다. | 통과 |
| 20 | 세 assertion slice를 full WPT/C11.4 완료로 오해할 수 있는가 | WPT harness-in-Spinon, 일반 author CSS admission, 전체 matrix, C11.4/C11 completion은 명시적으로 남긴다. | 통과 |

**판정:** 고정 WPT와 규범 기대, 세 limit 경계, width normalization 및 범위의 구현 전 누락은 발견하지 못했다. 계획 PR diff 검토는 별도 20관점으로 수행한다.
