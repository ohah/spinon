# C11.4 `align-content:stretch` 계획 PR diff 실패 관점 검토 30개

범위: 새 bounded plan, parent C11.4 plan entry, prior two-auto-row plan cross-link, fixed precomparison HTML/JSON/README/screenshot, lifecycle record, plan-review 20, this PR-diff review. 각 항목은 staged plan PR diff에서 별도로 확인했다.

| # | 독립 실패 경로 | 결과 |
|---:|---|---|
| 1 | 계획 PR에 구현 코드가 섞임 | PASS — plan/precomparison/review만 포함 |
| 2 | 상태 대장 체크박스가 계획 PR에서 바뀜 | PASS — `spec/STATUS.md`는 변경하지 않음 |
| 3 | 공식 완료 수를 78/214에서 바꿈 | PASS — 계획과 parent roadmap entry 모두 78/214 유지 |
| 4 | internal version 0.1.0을 변경 | PASS — 버전 고정 명시 |
| 5 | 이전 PR #1231 SHA를 candidate commit SHA로 씀 | PASS — merged SHA `5c79d19619617e7424aaeabbe884579ac383d130`은 existing base context로만 사용 |
| 6 | norm source에서 다른 draft/version을 기준으로 삼음 | PASS — pinned W3C 2025-03-26 CRD link와 section numbers 고정 |
| 7 | §11.8에 content distribution 값 `normal`을 빠뜨림 | PASS — `normal`/`stretch` 두 값을 모두 기술 |
| 8 | §11.8을 모든 alignment property에 적용 | PASS — Grid container content-distribution value로 한정 |
| 9 | fixture Grid size를 40×40에서 변경 | PASS — width/height 40px 그대로 |
| 10 | explicit column/row list가 변함 | PASS — `[40]px` columns, `[10]px` explicit rows |
| 11 | `grid-auto-rows`가 `auto`가 아님 | PASS — exact computed value와 source fixture를 확인 |
| 12 | gap, box edges, overflow input이 섞임 | PASS — zero gap/edges, visible overflow 유지 |
| 13 | source-order direct child shape가 바뀜 | PASS — fixed empty, `A`, `B` exact order |
| 14 | child count가 2 또는 4로 변함 | PASS — visible direct children 정확히 3개 |
| 15 | text font/line metrics가 달라짐 | PASS — 10px/10px monospace, one-line `A`/`B` |
| 16 | minimum contribution을 10px 대신 15px로 기록 | PASS — each implicit row minimum 10px |
| 17 | track base input 산술이 달라짐 | PASS — `[10,10,10]px` |
| 18 | free-space remainder가 달라짐 | PASS — 10px remainder |
| 19 | share를 explicit fixed track에도 배분 | PASS — 두 implicit auto rows에 +5px씩 |
| 20 | final tracks 또는 child frames가 달라짐 | PASS — `[10,15,15]px`와 세 exact frames |
| 21 | line-count control 누락 | PASS — Chrome observation 1/1 text lines |
| 22 | normal control에서 same result를 가정만 함 | PASS — same DOM CSSOM observation 기록 |
| 23 | start control을 stretch positive로 오인 | PASS — `[10,10,10]px` tracks로 확인 |
| 24 | computed value와 author shorthand 지원을 동일시 | PASS — shorthand behavior claim을 분리하고 computed longhand gate 명시 |
| 25 | HeadlessChrome version을 잘못 기록 | PASS — UA는 HeadlessChrome/157.0.0.0 |
| 26 | fixture source hash가 브라우저가 본 file과 다름 | PASS — fixture SHA `44a3423c…a3e7308` 기록 |
| 27 | screenshot hash/file 누락 | PASS — PNG path와 SHA `f99b5916…14f5e5c` 기록 |
| 28 | Chrome screenshot을 Spinon/app pixel evidence로 주장 | PASS — browser oracle image로 한정 |
| 29 | Agent Browser process가 남음 | PASS — session list `No active sessions`, process match 0 |
| 30 | 다음 구현 PR 전에 계획 review/merge 문턱을 생략 | PASS — 계획 PR 20/30 review 후 병합을 구현 선행조건으로 둠 |

최종 staged diff는 구현 시작 전 plan-only scope를 유지한다. CSS Grid 기대값은 규범 산술이 기준이며 Chrome은 cross-check로만 사용한다.
