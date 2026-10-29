# C11.4 fit-content 성장 한계 계획 PR diff 적대 검토 20개

2026-10-29 · 이 PR은 사전 oracle, 새 C11.4 하위 계획, 계획 실패 관점 20개 및 상위 계획 연결만 변경한다. 구현 diff나 기존 WIP는 포함하지 않는다.

| # | PR diff 실패 경로 | 확인 및 결과 | 판정 |
|---:|---|---|---|
| 1 | 계획 변경이 구현 완료로 읽히는가 | 상태를 사전 비교 완료·runtime 미구현이라고 구분한다. | 통과 |
| 2 | 세 target WPT ID의 대소문자가 잘못되는가 | `contentLargerThanArgument`, `smallArgument`, `largeArgument`를 pinned source와 대조했다. | 통과 |
| 3 | 원문 해시와 revision 연결이 끊기는가 | `expected.json`의 revision/path/5,060 bytes/hash가 Agent Browser fetch와 일치한다. | 통과 |
| 4 | Chrome 13/13을 Spinon 결과로 부르는가 | README와 plan은 upstream oracle로 한정한다. | 통과 |
| 5 | 기대 JSON이 계획표와 다른 값을 담는가 | 세 case formula/rows/item/overflow가 같은 값이다. | 통과 |
| 6 | 30px 입력이 40px 계획으로 잘못 기록되는가 | `smallArgument`는 30px limit와 30px expected row다. | 통과 |
| 7 | 200px limit에서 child height를 row size로 오인하는가 | row/item 100px과 child 100px을 구분한다. | 통과 |
| 8 | normalization 이전/이후 결과가 달라지는가 | raw observation에서 width1264/60 결과가 동일하다. | 통과 |
| 9 | screenshot이 다른 revision/run일 수 있는가 | WPT screenshot은 plan evidence 폴더에 고정하고 hash/size를 기록했다. | 통과 |
| 10 | source pin이 없는 raw screenshot만 추가되는가 | expected/Chrome observation/normalized probe와 README가 함께 추가됐다. | 통과 |
| 11 | 상위 plan의 zoom status가 아직 미실행으로 남는가 | PR #783 runtime 결과와 남은 일반 zoom 한계를 반영했다. | 통과 |
| 12 | fit-content WPT table에 새 plan link가 빠지는가 | `fit-content-track-sizing.html` 행과 새 하위 계획을 연결한다. | 통과 |
| 13 | plan review를 PR diff review로 재사용하는가 | 계획 20개와 이 PR diff 20개를 별도 파일에 기록했다. | 통과 |
| 14 | 상위 C11.4나 전역 `72/213`가 올라가는가 | 계획은 완료 checkbox나 count를 바꾸지 않는다. | 통과 |
| 15 | fixture가 product CSS allowlist를 넓히는가 | 범위를 정확한 opt-in fixture로 제한한다. | 통과 |
| 16 | iOS capture가 compositor 이후로 확대되는가 | plan은 제출 전 Metal drawable로 제한한다. | 통과 |
| 17 | Android AVD가 hardware GPU 증거가 되는가 | software AVD와 물리기기 한계를 분리한다. | 통과 |
| 18 | 구현 전에 지정한 gates가 빠지는가 | typed value, contribution, frames, revisions, draw/present와 surface를 지정했다. | 통과 |
| 19 | 계획 파일 외에 코드나 unrelated untracked가 stage되는가 | staged path 목록을 명시적으로 점검한다. | 통과 |
| 20 | PR 뒤 문서/Tailnet 작업이 완료된 것으로 착각하는가 | 계획 PR은 runtime·post-merge status·public/Tailnet sync 전 단계임을 적었다. | 통과 |

**판정:** diff에서 계획 범위, oracle와 status link의 불일치는 발견하지 못했다. 구현 전 계획 검토 20개와는 별도다.
