# Roadmap five-fit 설명 표시 PR #1348 diff 검토 30개

**대상:** GitHub PR [#1348](https://github.com/ohah/spinon/pull/1348)의 원격 diff, rendered preview 값과 PR 본문 이미지. 검토 관점은 DOM 소유 경계, roadmap 계약, responsive overflow, 재현/정리와 게시물 링크를 분리한다.

| # | 실패 관점 | 관찰과 판정 |
|---:|---|---|
| 1 | 계획 설명이 `#kanban-board` 내부에서 JS 재렌더로 지워짐 | board container를 loading placeholder 뒤에서 닫고 plan paragraph를 sibling으로 이동했다. 통과. |
| 2 | JavaScript가 참조하는 board `id`가 바뀜 | `id="kanban-board"`를 유지했다. 통과. |
| 3 | board live announcement 의미가 사라짐 | `aria-live="polite"`는 기존 동적 container에 남아 있다. 통과. |
| 4 | JS 실행 전 loading placeholder가 빠짐 | placeholder는 계속 board 내부에 있다. 통과. |
| 5 | 정적 note selector가 과거 history note를 선택 | capture는 인접 selector `#kanban-board + .hero-progress`를 써 현재 설명을 정확히 읽는다. 통과. |
| 6 | note가 `section` 밖으로 빠져 heading context가 분리 | note는 같은 `kanban-section`의 board 바로 뒤 sibling이다. 통과. |
| 7 | div 닫기 이동으로 heading/nav DOM이 깨짐 | desktop과 mobile에서 제목·nav·칸반 column이 렌더링됐다. 통과. |
| 8 | dynamic kanban item이 `NEXT`가 아니게 됨 | live board text는 `C11.4 five-fit candidate · NEXT · 계획 완료`다. 통과. |
| 9 | 계획 단계가 구현 진행/완료로 과장 | card와 note에 RuntimeSession 구현 전, C11.4/C11 미완료가 있다. 통과. |
| 10 | five-fit 사전 비교 장면 수가 틀림 | note는 17,280 scenes와 8,640 paired keys로 일치한다. 통과. |
| 11 | track/tail/cap 요약에서 지원 범위가 잘못 표시 | note는 5 fit tracks, 세 tail 종류, 120 cap permutations로 고정한다. 통과. |
| 12 | 공식 체크 수가 올라감 | `spec/STATUS.md` 미변경, browser status `78/214`다. 통과. |
| 13 | C11.4 상위 상태를 완료로 승격 | 표시문은 C11.4/C11 미완료로 유지한다. 통과. |
| 14 | 내부 계약 version/API 범위가 표시 변경에 묶임 | contract 0059와 공개 API 문서는 diff 밖이다. 통과. |
| 15 | 계획 링크가 다른 C11.4 slice로 이동 | note의 plan href는 five-fit 계획 파일이다. 통과. |
| 16 | precomparison link가 네 fit 또는 다른 날짜를 가리킴 | href는 five-fit 2026-11-11 evidence README에 연결한다. 통과. |
| 17 | diff-review link가 계획 단계 증거에서 누락 | href는 five-fit plan PR diff 30 검토 경로다. 통과. |
| 18 | 계획 설명 추가로 desktop page overflow | viewport/document width는 1440/1440 CSS px다. 통과. |
| 19 | mobile page 바깥 가로 overflow 발생 | viewport/document width는 390/390 CSS px다. 통과. |
| 20 | mobile에서 칸반 열을 좌우로 볼 수 없음 | board client/scroll width는 362/1120이고 `overflow-x:auto`다. 통과. |
| 21 | collector가 JS board 내용 생성 전 screenshot 저장 | `wait --fn`은 five-fit card가 board에 도착할 때까지 기다린다. 통과. |
| 22 | collector가 잘못된 hero paragraph를 확인 | sibling selector, scene count, plan stage와 note visibility를 assert한다. 통과. |
| 23 | official status 불일치를 재현기가 놓침 | desktop assertion은 official status text에 `78/214`를 요구한다. 통과. |
| 24 | desktop layout은 맞지만 screenshot에서 설명이 안 보임 | Agent Browser 1440×1000 이미지에서 board 아래 note가 보임을 육안 확인했다. 통과. |
| 25 | mobile evidence가 pixel size와 viewport를 혼동 | image는 390×844이며 report의 CSS viewport/document width를 별도로 남긴다. 통과. |
| 26 | screenshot이 사용자가 지정하지 않은 브라우저 도구로 생성 | 재현기는 Agent Browser 0.31.1을 사용한다. Terminal Browser는 사용하지 않았다. 통과. |
| 27 | 실패 시 Agent Browser가 켜진 채 남음 | collector `finally`는 close를 호출하며 두 전용 session post-check 모두 inactive/pid null이다. 통과. |
| 28 | browser viewport 결과를 Android/iOS app 또는 simulator 결과로 표시 | README와 PR 본문은 desktop/mobile browser viewport라고 한정하고 simulator 미사용을 적는다. 통과. |
| 29 | PR body screenshot URL이 잘못됐거나 branch 이동에 취약 | commit-pinned raw image 두 장이 실제 PR body에서 `complete=true`, 1440×1000 및 390×844로 로드됐다. 통과. |
| 30 | check-run 없는 PR을 CI green으로 표시 | GitHub `statusCheckRollup`은 빈 목록이다. PR 본문에서 0 checks로 밝히며 CI green으로 부르지 않는다. 통과. |

## 판정

30개 관점은 five-fit 계획 PR #1347 검토와 별도다. PR #1348은 roadmap 문단의 DOM 위치와 그 표시 증거만 바꾼다. 구현 완료, API 지원, simulator 검증 또는 C11.4/C11 완료를 의미하지 않는다.
