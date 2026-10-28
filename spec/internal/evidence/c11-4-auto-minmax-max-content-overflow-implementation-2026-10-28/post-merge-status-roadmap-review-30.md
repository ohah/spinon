# C11.4 `minmax(auto, max-content)` 병합 후 문서 동기화 검토 · 실패 관점 30개

| # | 문서 동기화에서 확인한 독립 실패 경로 | 판정 |
|---:|---|---|
| 1 | PR #752의 상태가 열린 상태인데 병합으로 기록됨 | 통과 — GitHub 조회에서 `MERGED`를 확인했다. |
| 2 | merge timestamp가 다른 PR의 시각으로 복사됨 | 통과 — PR #752 `mergedAt`은 `2026-10-28T10:39:21Z`다. |
| 3 | Rebase and merge 결과 SHA가 PR head SHA로 대체됨 | 통과 — 상태·계획·로드맵 모두 main merge SHA `5ed428701832d539bd97cf6c2a1dca7253f7e966`를 사용한다. |
| 4 | 계획 PR #751과 구현 PR #752 역할이 뒤바뀜 | 통과 — 계획과 실제 test-only 구현을 따로 설명한다. |
| 5 | 빈 status-check rollup을 CI 통과로 기록함 | 통과 — PR check rollup이 비어 있음을 명시하고 통과 주장을 하지 않는다. |
| 6 | base branch required checks가 실제로 설정되어 있었는데 누락을 무시함 | 통과 — `main` branch protection에 required status checks가 없음을 재확인했다. |
| 7 | focused test 2/2를 전체 package suite 결과처럼 씀 | 통과 — focused `2/2`와 전체 `297/297`을 분리한다. |
| 8 | 전체 package count가 기존 295 tests와 혼동됨 | 통과 — merge된 최종 test count는 `297/297`로 기록한다. |
| 9 | formatter 결과를 검사하지 않은 상태에서 통과로 씀 | 통과 — `cargo fmt --all -- --check` 실제 결과를 인용한다. |
| 10 | workspace Clippy와 crate-targeted Clippy를 구분하지 않음 | 통과 — `spinon-layout --all-targets` targeted Clippy로 정확히 적었다. |
| 11 | 초기 E0425 compile failure를 숨김 | 통과 — 미가져온 `node` helper를 명시 import로 고친 뒤 최종 gate를 재실행했다고 기록한다. |
| 12 | 자동 최소 source만 확인하고 MaxContent trace를 누락함 | 통과 — docs는 Minimum source/value와 MaxContent contribution을 모두 적는다. |
| 13 | scrollable values에서 MaxContent contribution도 0이라고 기록함 | 통과 — Columns 100px, Rows 10px contribution을 유지한다고 적는다. |
| 14 | Chrome baseline을 Spinon 구현 결과로 소개함 | 통과 — 기준 화면이 browser baseline임을 각 공개 문서에 표시한다. |
| 15 | app route가 바뀌지 않았는데 Android/iOS Simulator 결과를 주장함 | 통과 — simulator 실행 없음과 pure unit-test 범위를 명시한다. |
| 16 | CSS parser overflow admission이 구현됐다고 확대함 | 통과 — 일반 CSS admission은 미완료라고 유지한다. |
| 17 | unit test를 clipping 또는 scroll offset 동작으로 표현함 | 통과 — clipping/scrolling 동작은 범위 밖이라고 분리한다. |
| 18 | C11.4 상위 항목을 완료 처리함 | 통과 — `spec/STATUS.md` checkbox는 미완료다. |
| 19 | 공식 완료 수를 72/213에서 올림 | 통과 — status, plan, public roadmap 모두 `72/213`을 유지한다. |
| 20 | C11.4 세부 계획에서 구현 예정 상태가 계속 표시됨 | 통과 — top-level heading/paragraph를 PR #752 병합 결과로 바꿨다. |
| 21 | 계획 문서에 실제 구현·검증 결과가 빠짐 | 통과 — 세부 계획에 gate 결과와 merge SHA 섹션을 추가했다. |
| 22 | evidence README가 pre-rebase source와 post-rebase merge SHA를 같은 값으로 혼동함 | 통과 — source test revision과 main merge commit을 별도 줄로 기록한다. |
| 23 | 최근 병합 목록에서 PR #752가 빠짐 | 통과 — 상위 계획의 최신 merged slices에 #752를 추가했다. |
| 24 | #749/#750의 이전 결과가 삭제되거나 #752와 합쳐짐 | 통과 — 기존 plan/test-only 기록을 보존하고 #752 result를 별도 section으로 추가했다. |
| 25 | public roadmap 링크가 내부 파일의 잘못된 prefix를 사용함 | 통과 — `/spinon/plan/...` 및 `/spinon/internal/evidence/...` 경로를 실제 사이트 구조대로 사용한다. |
| 26 | roadmap에 다음 계획 문구가 남아 merged 결과와 충돌함 | 통과 — 해당 paragraph를 실제 구현/merge 설명으로 교체했다. |
| 27 | 공개 설명에서 10개 case의 값이나 두 축이 빠짐 | 통과 — Columns/Rows, 100/60px 및 10/5px tracks와 두 contribution을 요약한다. |
| 28 | review artifact 링크가 README/plan에 연결되지 않음 | 통과 — 구현/runtime 20개, PR diff 30개, post-merge docs review 링크를 연결한다. |
| 29 | unrelated status/roadmap entries가 변경됨 | 통과 — 변경은 C11.4 항목과 해당 최신 병합 목록에 한정된다. |
| 30 | 문서 수정 때문에 전체 docs build가 통과했다고 잘못 보고함 | 통과 — 마지막 전체 build는 기존 dead-link 오류를 남겼으며 task 경로 오류가 없었다는 범위로만 기록한다. |

이 기록은 PR #752 병합 후 상태 대장, C11.4 상위/세부 계획 및 공개 로드맵의 사실 일치만 검토한다. C11.4 지원 완료나 공식 집계 증가를 승인하지 않는다.
