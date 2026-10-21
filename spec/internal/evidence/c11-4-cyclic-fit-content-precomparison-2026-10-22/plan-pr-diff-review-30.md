# C11.4 cyclic percentage `fit-content()` 계획 PR diff 검토 30개

검토 대상: 계획 PR의 최종 staged diff. 계획 실패 관점 20개와 별도로 파일 연결·수치·근거·상태 변경을 다시 확인했다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | definite `fit-content(50%)` 기존 case를 새 기능으로 중복 주장 | 새 범위를 cyclic max-content axes와 구분했다. |
| 2 | `minmax(0px,50%)` slice와 같은 sizing behavior로 축약 | fit-content intrinsic minimum과 max-content ceiling을 계획에 포함했다. |
| 3 | columns·rows의 cyclicness가 서로 섞임 | column과 row는 별도 scene이며 scene당 cyclic axis 하나다. |
| 4 | definite control이 cyclic 기대값으로 채워짐 | width `40px` definite control의 used column `20px`을 따로 둔다. |
| 5 | column root width의 max-content basis가 track과 무관해짐 | text max-content `50px`과 결과 track `25px`을 분리했다. |
| 6 | column min-content가 limit보다 큰 입력이라 clamp를 못 봄 | Ahem `aa aa` min-content `20px`, final limit `25px`다. |
| 7 | fixed-size item이 fit-content cap을 드러내지 못함 | 24×16 fixed probe를 명시적으로 거부하고 해당 관찰을 보존했다. |
| 8 | row root 높이가 명시값으로 고정되어 cyclic sizing을 우회 | row scene은 width만 definite이고 height는 `max-content`다. |
| 9 | row text wrapping이 viewport/fallback font에 의존 | width `20px`, Ahem 10px advance/line-height와 네 text fragment를 고정했다. |
| 10 | Ahem asset 상대 경로가 다른 fixture 위치로 해석 | Browser가 실제 파일을 열고 `document.fonts` loaded/check를 확인했다. |
| 11 | saved oracle이 font load 이전의 측정값 | observation JSON은 loaded status와 true font check를 보존한다. |
| 12 | 브라우저 판본과 viewport를 사후 추정 | HeadlessChrome `156.0.8078.48`, viewport와 DPR을 JSON에 기록했다. |
| 13 | item automatic minimum이 percentage cap을 가림 | fixture에서 `min-width:0;min-height:0`을 고정했다. |
| 14 | UA body margin 또는 insets가 intrinsic basis에 추가 | html/body와 fixture boxes의 margin·padding·border·gap을 0으로 둔다. |
| 15 | content box와 border box percentage basis를 혼동 | insets가 0이어서 width/height basis가 content size와 같다. |
| 16 | root size와 used track을 같은 값으로 검사 | 각 scene 표와 observation에서 tracks, root frame, item frame을 분리했다. |
| 17 | page-flow 누적 y를 장면의 intrinsic geometry로 오해 | acceptance는 size와 root-relative origin을 사용하고 page y를 제외한다. |
| 18 | fit-content formula의 인수 순서나 clamp 방향 오류 | 현재 CSS Grid 2025 CRD의 `min(max-content,max(auto,limit))` 순서를 확인했다. |
| 19 | cyclic percentage를 최종 container size로 intrinsic pass에서 선해결 | §7.2.1/§11.1의 intrinsic auto 처리와 final resolution을 분리했다. |
| 20 | 변경 이력의 UA 문구를 현재 normative allowance로 오독 | 변경 이력과 현재 normative text를 구분해 이력 문구를 기대 근거에서 제외했다. |
| 21 | spec 링크가 잘못된 section fragment를 가리킴 | Agent Browser DOM에서 `#track-sizes`, `#algo-grid-sizing` heading을 확인했다. |
| 22 | Chrome screenshot과 current fixture JSON이 다른 입력을 나타냄 | screenshot과 saved observation은 Ahem text fixture를 사용한다. |
| 23 | rejected screenshot이 current oracle로 보임 | 이전 fixed-item evidence에 rejected 이름을 붙이고 README에서 제외 이유를 적었다. |
| 24 | saved output에서 row line fragmentation을 검증할 수 없음 | current observation JSON에 각 text fragment rect를 기록했다. |
| 25 | PR/문서에서 local fixture path가 존재하지 않는 main-branch URL | Markdown 문서에는 경로를 code text로 남겨 merge 전 stale link를 만들지 않는다. |
| 26 | evidence image가 문서에서 download-route dead link가 됨 | PNG는 Markdown image로 포함했고 build log에서 새 파일 오류가 없었다. |
| 27 | 계획 검토 횟수를 PR diff 검토 횟수와 혼동 | 계획 20개와 이번 PR delta 30개를 별도 파일에 둔다. |
| 28 | plan-only PR인데 runtime pass를 암시 | plan/status/roadmap은 runtime과 simulator 검증이 아직 없다고 명시한다. |
| 29 | C11.4 checkbox나 완료 count가 계획 PR에서 승격 | `spec/STATUS.md`는 건드리지 않고 `72/213`을 유지했다. |
| 30 | docs build의 기존 dead links를 이번 PR 성공으로 보고 | `docs:build`는 기존 여러 파일의 dead links로 실패했고 이번 신규 plan/evidence 경로는 log 오류에 없다고 구분했다. |

판정: 계획 PR의 fixture, oracle 자료, normative anchors, roadmap note와 구현 전 검토 범위를 대조했다. 발견한 규범/변경 이력 혼동은 수정했다. 남은 build 실패는 이번 diff 밖의 기존 dead links이며 성공으로 보고하지 않는다. 이 검토는 implementation/runtime 20개와 이후 구현 PR diff 30개를 대신하지 않는다.
