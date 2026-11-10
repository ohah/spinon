# C11.4 three-fit auto-placement 계획 PR diff 적대 검토 30개

검토 대상은 계획 PR 실제 diff(계획, 별도 계획 검토 20개, Chrome fixture/capture/관측/독립 verifier, screenshot, README, public roadmap 메모)다. 구현 코드는 이 PR에 포함하지 않는다.

| # | 서로 다른 실패 관점 | 실제 diff 확인 및 방어 | 판정 |
|---:|---|---|---|
| 1 | 계획 PR이 제품 구현을 미리 지원으로 게시하는가 | 계획은 `구현 시작 전`이며 Chrome 관측을 제품 RuntimeSession 근거와 분리한다. | 통과 |
| 2 | 직전 three-fit span 경계를 없애는가 | 직전 row-span plan/PR #1338을 선행으로 연결하고 explicit placement profile을 유지한다. | 통과 |
| 3 | one-/two-fit auto-placement 회귀 기준이 누락되는가 | 기존 #1322/#1318 계획을 직접 연결하고 separate regression을 완료 관문에 둔다. | 통과 |
| 4 | official count가 bounded plan 때문에 증가하는가 | roadmap/plan에 C11.4/C11 미완료와 공식 78/214 유지가 명시됐다. | 통과 |
| 5 | internal interface contract version이 불필요하게 바뀌는가 | API/FFI 및 contract 숫자 버전은 추가하지 않는다고 명시했다. | 통과 |
| 6 | `grid-auto-rows`의 six-member 제한이 흐려지는가 | exact 3 fit + min/max/auto 각 1개만 계획 범위로 고정했다. | 통과 |
| 7 | 모든 fit position triple이 corpus에 있는가 | generator/verifier는 6개 track 중 3개 선택 20조합을 요구한다. | 통과 |
| 8 | tail order 누락이 허용 list를 달리 만드는가 | 세 tail keyword의 여섯 순열을 확인하고 completeness를 assert한다. | 통과 |
| 9 | fit cap이 authored track position에서 이동하는가 | six cap permutations와 track index 매핑을 scene마다 검증한다. | 통과 |
| 10 | auto cap 중복 또는 cap permutation 편향이 생기는가 | 각 scene이 0px·10px·20%를 각각 한 번 쓰고 6 assignment 모두 확인한다. | 통과 |
| 11 | percentage cap의 basis가 불명확한가 | definite 220px Grid height를 고정하고 indefinite basis는 rejection boundary에 넣었다. | 통과 |
| 12 | 1,440/2,880/5,760 개수가 잘못 짝지어지는가 | 720 list × 2 alignment × 2 DOM order × 2 min-height = 5,760이며 paired key 2,880을 검증한다. | 통과 |
| 13 | baseline/candidate에서 다른 authored scene을 비교하는가 | verifier는 track list, alignment, source order로 pair를 찾아 min-height만 다르게 한다. | 통과 |
| 14 | reverse DOM이 단순 동일 결과를 강요하는가 | 각 방향의 item ID/payload를 별도 확인하고 두 방향의 geometry 동일성을 요구하지 않는다. | 통과 |
| 15 | 첫 explicit row control이 자동 배치 row로 오인되는가 | 첫 control은 0px min-height와 10px explicit row로 고정된다. | 통과 |
| 16 | line count 대신 payload 문자열만 확인하는가 | 여덟 payload의 기대 line count, frame과 text Range fragment 개수를 각각 검증한다. | 통과 |
| 17 | Range rect y offset을 box origin과 혼동하는가 | 1px first-line Range offset과 10px inter-line spacing을 separate assertion으로 둔다. | 통과 |
| 18 | `fit-content()` cap이 automatic minimum floor를 낮춘다고 해석하는가 | baseline/auto paired floor 관계를 실제 관측에서 독립 계산한다. | 통과 |
| 19 | min-content/max-content tracks가 auto-minimum eligible로 분류되는가 | verifier는 fit-content 및 auto member만 eligible로 분류한다. | 통과 |
| 20 | visible item overflow를 Grid frame overflow와 혼합하는가 | undersized baseline row payload와 마지막 item의 220px Grid 경계를 분리해 셈한다. | 통과 |
| 21 | stretch로 fit/min/max row도 바뀌는가 | candidate start/stretch의 각 row를 비교해 40px가 auto row occurrence에만 분배되는지 검사한다. | 통과 |
| 22 | `min-height:auto` candidate가 payload floor 아래에 남는가 | 모든 candidate row, eligible row와 baseline에서 회복한 수를 각각 assert한다. | 통과 |
| 23 | `min-height:0` baseline 자체에 예상된 overflow가 없는가 | baseline floor 아래 count를 positive control로 보존해 candidate 효과를 구분한다. | 통과 |
| 24 | 전체 used track 합을 root box 높이와 혼동하는가 | explicit 10px row 포함 start 180px와 stretch 220px를 별도로 검사한다. | 통과 |
| 25 | Chrome result가 다른 browser/platform 결과로 부풀려지는가 | metadata는 HeadlessChrome 157, MacIntel, DPR1, viewport를 기록한다. | 통과 |
| 26 | page가 제품 code나 Taffy를 oracle로 사용해 순환 검증하는가 | 고정 독립 HTML fixture는 browser CSS Grid에서 측정하며 제품 구현/RuntimeSession 호출이 없다. | 통과 |
| 27 | screenshot이 matrix 전체 또는 product screen인 것처럼 표현되는가 | README와 plan에서 representative single Chrome candidate scene이며 app screenshot이 아니라고 표시한다. | 통과 |
| 28 | README의 evidence links가 RSPress dead-link 검사에 새 실패를 보태는가 | raw json/script/image는 inline code 또는 Markdown image로 표현했다. 최신 build log에서 새 plan/evidence dead-link entry가 없었다. | 통과 |
| 29 | Agent Browser session 또는 Chrome process가 검증 뒤 남는가 | lifecycle JSON은 close success, session list는 empty, post-close process check는 browser process 없음이다. | 통과 |
| 30 | 저장소 전체 docs build 실패를 이 diff의 통과로 과장하는가 | README는 1,533개의 repo-wide dead-link group 때문에 docs build exit 1임을 기록하고 새 경로에는 항목이 없다고 한정한다. | 통과 |

## 결론

계획 PR diff에서 계획과 구현을 혼합하거나, 공식 완료 수를 변경하거나, 새 문서 경로의 dead link를 만드는 차단 결함을 찾지 못했다. screenshot은 raw commit URL로 PR 본문에 첨부하고 Agent Browser에서 실제 이미지 load를 확인한 뒤 병합한다. 이 검토는 계획 PR diff의 30개 관점이며 계획 20개 검토와 기능 구현/runtime 검토를 대체하지 않는다.
