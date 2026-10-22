# C11.4 cyclic `minmax(calc())` overlapping row plan PR diff 실패 관점 20개

2026-10-22에 계획 PR의 문서·fixture·사전 비교 자료 diff를 별도로 확인했다. 이는 계획 내용 검토와 구분한 PR diff 검토이며 runtime 구현 검토가 아니다.

| # | diff에서 확인한 경계 | 결과와 반영 |
|---:|---|---|
| 1 | PR #500 선행 구현 링크가 이 계획의 원인이 되는지 | #500은 열 축 범위이고 row axis를 제외한 선행 slice로만 인용했다. |
| 2 | 새 집중 계획의 모든 상대 링크가 저장소 경로와 맞는지 | plan, plan review, evidence README 링크를 각각 실제 파일 경로에 대조했다. |
| 3 | README에서 fixture를 가리키는 상대 경로가 root로 복귀하는지 | evidence 폴더 기준 네 단계 상위 후 `tests/fixtures/...`에 도달한다. |
| 4 | HTML Ahem URL이 계획의 저장소 자원과 같은지 | fixture URL은 `tests/fixtures/resources/c15/Ahem.ttf`를 가리키며 해당 파일 SHA를 기록했다. |
| 5 | plan·fixture·expectation JSON의 여섯 scene ID가 같은지 | forward/reverse × cyclic/140px/100px 명칭을 모두 대조했다. |
| 6 | row axis 내용을 column axis 수치로 잘못 표현했는지 | 계획의 track list·gap·frame 표는 Rows, block contribution, 높이를 사용한다. |
| 7 | fixture 코드가 계획된 grammar보다 넓어졌는지 | 한 10px column, 세 explicit rows, 동일 calc track과 고정 3 item으로 제한했다. |
| 8 | source-order control이 placement까지 바꾸는지 | kind별 `grid-row`는 고정되고 DOM 순서만 달라지는 것을 markup에서 확인했다. |
| 9 | 독립 JSON이 Chrome observation을 normative source로 표시하는지 | `browserIsNotSourceOfExpectedValues`를 true로 두고 별도 계산을 수록했다. |
| 10 | Ahem의 7/8/14 line과 JSON contribution 수치가 일치하는지 | 10px line-height에서 70/80/140px을 각각 검산했다. |
| 11 | planned increase의 span gap arithmetic가 맞는지 | span-2 한 gap, span-3 두 gap, 결과 `[34,39,39]`와 `[42,47,47]`을 교차 확인했다. |
| 12 | definite 100px 근거에서 track min, free space, gap을 섞었는지 | minima 36px + gaps 4px 후 free space 60px, track당 20px이다. |
| 13 | root content size와 border-box size를 혼동했는지 | content height 140/100px과 border box 144/104px을 분리했다. |
| 14 | 100px overflow 문구가 content와 border 기준을 혼동했는지 | 40px content overflow와 text-end 좌표를 따로 쓰고 root border 바깥 거리로 과장하지 않았다. |
| 15 | observation이 line rect와 frame을 함께 보존하는지 | 원본 JSON에서 Ahem line rect, root/item frame, scrollHeight 및 computed Rows를 보존했다. |
| 16 | screenshot이 다른 fixture/session 실행에서 왔는지 | README가 동일 전용 Agent Browser session과 screenshot/observation artifact를 연결한다. |
| 17 | browser session 종료 사실이 사용자 Chrome 종료로 확대되었는지 | empty preflight, `close --all`, empty post-list, PID 종료, 보존된 Chrome/Tailnet PID를 명시했다. |
| 18 | browser 흰색 배경이나 screenshot을 native pixel/CSS border 근거로 기술했는지 | README는 이를 구분하고 native 색상은 이후 runtime에서 미측정이라고 기록한다. |
| 19 | 계획 PR diff가 구현 또는 완료 상태로 읽힐 수 있는지 | plan-only 문구와 runtime 미시작, C11.4 미완료 및 `77/213` 유지가 각 문서에 있다. |
| 20 | 광범위한 workspace 미추적 산출물이 diff에 섞일 위험 | PR 제출 시 변경 파일만 명시적으로 stage하고 관련 없는 근거/빌드 파일은 제외한다. |

## 판정

문서 링크, fixture 입력, 관찰/기대값 대응 및 상태 표시를 확인했다. PR diff는 계획과 사전 비교 자료에 한정한다.
