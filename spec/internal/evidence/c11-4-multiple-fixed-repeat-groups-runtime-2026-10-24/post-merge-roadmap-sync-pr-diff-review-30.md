# PR 병합 후 roadmap·상태 동기화 diff hostile review 30회 · 2026-10-24

대상은 PR #594 병합 후 계획, `spec/STATUS.md`, 내부 계약 0059, 공개 roadmap 및 Tailnet 검증 기록의 상태 동기화다. 기능 코드는 바꾸지 않는다. 각 관점은 서로 다른 상태·근거·배포 실패를 검사한다.

1. **다른 PR의 merge SHA 혼입** — GitHub PR #594 `mergeCommit.oid`와 `origin/main` HEAD를 대조했다. 둘 다 `1339874a04b2bc9866ce6eb6e0172e26dddf56ef`다. 통과.
2. **PR이 열린 상태로 잘못 기록됨** — PR API의 state와 본문 내 상태 서술을 대조했다. GitHub는 `MERGED`; 관련 네 문서는 병합 완료로 쓴다. 통과.
3. **병합 시각을 로컬 시간과 혼동** — PR API의 `mergedAt`과 문서 시각을 비교했다. `2026-10-24 10:14:36 UTC`로 일치한다. 통과.
4. **병합 방식이 규칙과 다름** — PR 상태 변경 기록과 문서의 방법을 대조했다. `Rebase and merge`이며 merge commit/squash로 표기하지 않는다. 통과.
5. **C11.4 parent checkbox를 slice 완료로 승격** — `spec/STATUS.md`에서 `C11.4 intrinsic track sizing` 상위 행을 확인했다. 미완료 표시를 유지한다. 통과.
6. **공식 카운터가 코드 진척에 맞춰 근거 없이 증가** — Agent Browser의 live DOM checkbox를 셌다. 총 213개, checked 72개로 `72/213`을 유지한다. 통과.
7. **72 유지 사유가 문서에 없음** — 상태·계약·roadmap 설명을 대조했다. 이 PR은 두-group 제한 slice이고 C11.4 전체 intrinsic sizing을 다루지 않는다고 명시한다. 통과.
8. **roadmap과 STATUS의 수치 불일치** — live roadmap DOM, plan, 상태 대장 본문을 대조했다. 모두 `72/213`이다. 통과.
9. **계획이 PR 검토 중으로 남음** — plan 첫 상태 문장과 병합 상태를 비교했다. #594와 merge SHA를 완료 상태로 쓴다. 통과.
10. **공식 상태 기록에 PR 누락** — C11.4 slice 기록의 PR 번호, 날짜, merge SHA를 GitHub와 대조했다. 일치한다. 통과.
11. **내부 계약에 이전 PR 상태가 잔존** — 계약 0059의 두-group section을 읽었다. 병합 정보를 반영했다. 통과.
12. **공개 roadmap에 stale open 상태 잔존** — 렌더링된 roadmap의 해당 C11.4 행을 Agent Browser로 검사했다. PR #594가 Rebase and merge로 표시된다. 통과.
13. **문서마다 서로 다른 SHA 사용** — plan·STATUS·0059·roadmap live page 텍스트를 같은 SHA와 비교했다. 일치한다. 통과.
14. **UTC 표기를 KST로 오인** — API가 반환한 `Z` 타임스탬프와 문서의 UTC label을 대조했다. 10:14:36 UTC로 일치한다. 통과.
15. **필수 검사가 없는데 CI 통과로 과장** — `gh pr checks 594 --required`와 check rollup을 확인했다. 보고된 검사는 없다고 쓰며 green CI라고 주장하지 않는다. 통과.
16. **branch protection이 요구하지 않는 검토를 완료로 주장** — main protection API 응답을 확인했다. required status checks와 required approval은 설정되지 않았다. 통과.
17. **시뮬레이터를 실기기 증거로 승격** — README와 상태 문서의 Android/iOS 한계를 확인했다. Android AVD software rendering 및 iOS compositor 이전 표본으로 구분한다. 통과.
18. **시뮬레이터 캡처를 WPT 적합성으로 확대** — 이번 범위 문구를 검사했다. WPT와 전체 CSS admission은 미완료로 남는다. 통과.
19. **내부 계약 숫자 버전 불필요 상승** — live 계약 page의 버전을 검사했다. `0.1.0`을 유지한다. 통과.
20. **성공 사례만 두고 fail-closed 범위를 삭제** — 계약의 세 번째 fixed-repeat, named lines, unsupported grammar 경계를 확인했다. 기존 제외 범위를 유지한다. 통과.
21. **전체 docs build 성공으로 오기** — README의 build 문구를 이전 전체 build 결과와 대조했다. 선택 build의 dead-link 검사 비활성화와 전체 저장소의 기존 dead-link 문제를 분리한다. 통과.
22. **Rspress 버전 드리프트** — 선택 build 로그의 banner와 프로젝트 pin을 비교했다. `2.0.22`다. 통과.
23. **selected route count와 checkbox count 혼동** — build manifest와 DOM 증거를 대조했다. 107개는 preview route 수이고 213은 roadmap checkbox 수라고 구분한다. 통과.
24. **기존 preview 백업 덮어쓰기** — pass04 backup root의 preflight/copy/verify 보고서를 확인했다. 교체 전 파일은 별도 `pass04` 경로에 보존한다. 통과.
25. **활성 파일을 안전성 점검 없이 덮어씀** — preflight의 22개 replace 경로를 `lsof` 결과와 교차 확인했다. 겹치는 open path가 없다. 통과.
26. **복사 중 부분 파일 노출** — sync script의 temp-file + atomic rename 경로를 검사하고 완료 report를 확인했다. 전체 복사 성공 및 hash 확인을 기록한다. 통과.
27. **preview HTTP는 200이지만 stale bytes** — verify report의 원격 body hash를 build hash와 비교했다. 107/107이 HTTP 200 및 SHA-256 일치다. 통과.
28. **백업 경로가 조용히 손상** — pass04 verify의 backup 검증 결과를 확인했다. 22/22 해시가 일치한다. 통과.
29. **sync cleanup이 unrelated route 삭제** — preflight/copy/verify report의 delete 상태를 검사했다. 세 보고서 모두 삭제 0개다. 통과.
30. **검증 후 Agent Browser 세션·프로세스 잔류** — Agent Browser close 완료 후 process 목록을 조회했다. 세션 종료 및 Agent Browser 프로세스 0개다. 통과.

## 판정

30개 관점에서 병합 상태와 완료 집계가 일치하며 문서·미리보기의 상태 표기가 서로 모순되지 않는다. C11.4 전체 항목은 미완료로 유지한다.
