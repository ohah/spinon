# C11.4 반복 텍스트 행 span 계획 PR · 변경 diff 검토 30개

검토 범위는 다음 slice plan, 계획 실패 관점 20개, Chrome 사전 비교 fixture·관측·검증 파일, 그리고 공식 상태대장·공개 roadmap의 계획 상태 갱신이다. 구현 코드는 변경하지 않았다. 각 항목은 다른 artifact boundary 또는 회귀 경로를 검사한다.

| # | 공격 관점 | 확인 | 결과 |
|---:|---|---|---|
| 1 | plan PR을 구현 PR처럼 표시 | `STATUS`와 roadmap 모두 “계획/구현 시작 전”이라고 명시 | 통과 |
| 2 | #1292 merge SHA 오기 | 직전 계획, `STATUS`, GitHub 링크의 SHA·시각 교차 확인 | 통과 |
| 3 | 공식 완료 수 변조 | top summary, C11.4 parent checkbox, counts `78/214`가 유지됨 | 통과 |
| 4 | C11.4/C11 checkbox 상승 | 체크 변경이 없고 bounded slice를 parent completion으로 확대하지 않음 | 통과 |
| 5 | 현재 작업 카드가 이미 병합된 작업을 가리킴 | roadmap heading을 새 계획명으로 갱신하고 #1292 이력은 보존 | 통과 |
| 6 | 계획 PR merge 전 실행 중이라고 주장 | roadmap/status 문구가 plan 상태와 코드 미착수를 구분 | 통과 |
| 7 | 새 계획 링크의 RSPress 경로 | Markdown basename과 `/spinon/plan/...html` 링크가 일치 | 통과 |
| 8 | matrix evidence 링크 오탈자 | plan, README, status, roadmap 모두 같은 evidence slug 사용 | 통과 |
| 9 | plan review 파일 경로 오기 | plan 및 status가 `c11-4-root-html-repeated-text-row-span-plan-review-20.md`를 가리킴 | 통과 |
| 10 | 계획 diff 리뷰와 계획 review 혼합 | plan failure review 20개와 계획 PR diff review 30개를 분리 | 통과 |
| 11 | 관측용 fixture가 제품 route로 유입 | fixture는 `tests/fixtures/css/c11/`에만 추가되고 앱/core 변경 없음 | 통과 |
| 12 | auto column이 implicit column을 생성 | 모든 fixture item의 column placement가 `1 / 2` | 통과 |
| 13 | row line/span 문법 혼동 | 네 topology가 CSS line 2/3과 span 2/3을 분명히 표현 | 통과 |
| 14 | overlap control 실효성 | span-2 pair는 row 3을 공유하고 span-3은 두 span-2와 겹침 | 통과 |
| 15 | reversal control에서 의미 item 손실 | reverse scene의 A/B/C 의미 item 및 placement가 같음 | 통과 |
| 16 | 미지원 text grammar 추가 | 세 payload가 기존 #1292 ASCII grammar·font/line-height 경계를 사용 | 통과 |
| 17 | cap matrix 누락 | 20px, 20%, 0%, 120%의 네 입력이 포함됨 | 통과 |
| 18 | tail multiset permutation 누락 | verifier가 30 unique tail × cap × align × topology 960 key를 검사 | 통과 |
| 19 | alignment 모델 오류 | start/stretch 비교가 track sizing과 post-sizing auto stretch를 분리 | 통과 |
| 20 | percentage basis indefinite | Root height 220px가 definite로 고정 | 통과 |
| 21 | span height를 payload height와 동일시 | verifier는 item height를 spanned used row 합에 대조 | 통과 |
| 22 | DOM order가 track sizing을 바꿈 | source-order reversal에서 semantic item frame delta를 계산 | 통과 |
| 23 | line count를 CSS content로 하드코딩 | Chrome Range rect에서 A/B/C 2/3/4 line count를 측정 | 통과 |
| 24 | JSON 관측 누락/중복 scene | 960 NDJSON rows와 960 unique key를 검사 | 통과 |
| 25 | screenshot을 제품 evidence로 오인 | README는 Chrome geometry/text-wrap screenshot으로만 분류 | 통과 |
| 26 | Agent Browser 사용 규칙 위반 | fixture 및 W3C 기준은 Agent Browser를 사용했고 session은 닫음 | 통과 |
| 27 | W3C 규범 URL의 부정확한 anchor | §10.5는 Agent Browser에서 확인한 `#grid-align` 사용 | 통과 |
| 28 | fixture column bug 잔존 | 초반 자동 column 배치 문제를 기록하고 최종 해시/행렬에서 수정 확인 | 통과 |
| 29 | implementation test 범위가 계획과 다른 경로 | 계획은 default parsed Root HTML→RuntimeSession route, opt-in 우회 금지 | 통과 |
| 30 | 플랫폼·CI·surface claim 과대화 | 구현 후 별도 simulator/build gates와 CI 상태 구분을 요구 | 통과 |

**결과:** 서로 다른 diff 실패 관점 30개를 확인했다. 계획 PR 단계에서 미해결 변경 결함을 찾지 못했다. 구현/runtime 증거와 구현 PR diff 검토는 이후 PR에서 새 관점으로 별도 수행한다.
