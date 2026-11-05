# C11.4 공식 집계 정정 PR diff 검토 30개

- 대상: PR #1192, `spec/STATUS.md`, 내부 계약 0059, C11.4 direct-text 계획, 최신 #1187·#1190 로드맵 카드 및 집계 정정 evidence
- 기준 commit: #1191 merge `41a20a698d783944979ee01ea7df7c337ba54c81`
- 독립 기준: 공식 대장의 실제 checkbox parse와 Tailnet 페이지의 GitHub API-backed status note

## 실패 관점

1. **대장 원본 대체** — 공식 상태 원본은 `spec/STATUS.md` 그대로다. 로드맵은 read-only 집계만 한다. **PASS**
2. **중복 ID parse** — stable-ID task 행 212개를 센다. **PASS**
3. **완료 stable-ID 오산** — `[x]` stable-ID task 78개를 확인했다. **PASS**
4. **ID 없는 하위 task 누락** — parser가 잡는 하위 checkbox 2개를 포함한다. **PASS**
5. **하위 task 완료 오산** — 두 하위 checkbox가 모두 unchecked이므로 완료 수에 더하지 않는다. **PASS**
6. **분모 오산** — 212 stable ID와 2 하위 checkbox의 총합 214가 live counter와 일치한다. **PASS**
7. **합계 혼동** — 대장 parser와 Tailnet `#official-status-note`가 모두 `78/214`를 냈다. **PASS**
8. **기존 기록을 현재로 오인** — 이전 evidence의 당시 `72/213` 표기는 보존하고 최신 상태 표현만 수정한다. **PASS**
9. **새 완료 승격** — 어떤 checkbox도 바꾸지 않으며 집계 분모·분자만 새로 선언하지 않는다. **PASS**
10. **C11 조기 완료** — C11 parent checkbox는 unchecked로 남는다. **PASS**
11. **C11.4 조기 완료** — C11.4 parent checkbox는 unchecked로 남는다. **PASS**
12. **#1190 상태 문구** — 새 status entry를 live aggregate `78/214`와 맞춘다. **PASS**
13. **계획 수치 불일치** — C11.4 text plan의 현재 count도 `78/214`로 맞춘다. **PASS**
14. **계약 수치 불일치** — contract 0059의 해당 merged slice도 `78/214`를 말한다. **PASS**
15. **#1187 최신 카드 오염** — 이전 static card를 live aggregate와 같게 한다. **PASS**
16. **#1190 최신 카드 오염** — direct-text card를 live aggregate와 같게 한다. **PASS**
17. **다른 이력 일괄 치환** — 시점이 다른 이전 카드·evidence의 기록 수를 전역 replace하지 않는다. **PASS**
18. **정정 이력 숨김** — 이전 문서 검토 기록의 사후 정정 note를 남겨 `72/213` 오인의 원인을 드러낸다. **PASS**
19. **Tailnet mismatch artifact 덮어쓰기** — pass01 JSON/PNG를 유지하고 correction 뒤 결과는 별도 pass02 파일로 기록한다. **PASS**
20. **browser aggregate를 못 본 척함** — Agent Browser status counter가 GitHub API source를 성공적으로 읽어 `78/214`를 낸 결과를 연결한다. **PASS**
21. **local parse 생략** — 독립 local parser 결과 212/78 + 2/0을 JSON으로 남긴다. **PASS**
22. **live API와 source commit 오인** — API-backed value는 #1191 merge 뒤 `main` 문서에서 읽은 현재 표시로 기록한다. **PASS**
23. **pass01을 corrected page라 부름** — 첫 pass의 정적 #1190 card mismatch를 명시하며 correction merge 후 재게시를 pending으로 둔다. **PASS**
24. **PR #1191 본문 오정보 방치** — 병합 PR 본문도 #1192 수정 후 실제 집계로 정정한다. **PASS**
25. **C11.4 범위 과장** — 기존 single direct-text fixture 및 platform boundary는 변경하지 않는다. **PASS**
26. **PR #1190 screenshot 변조** — Android/iOS images는 원래 merge SHA `950c8581…`를 계속 가리킨다. **PASS**
27. **GitHub 문서 링크 경로 회귀** — plan/evidence links는 main blob 경로를 유지한다. **PASS**
28. **Rspress build 실패 은폐** — 기존 repository-wide dead-link failure를 기록하며 green으로 부르지 않는다. **PASS**
29. **CI 상태 오인** — #1192의 GitHub checks는 merge 전 확인하고 보고된 결과만 기록한다. **PASS**
30. **제품 코드 부작용** — 변경은 count text·review/evidence 문서에 한정되고 구현 코드나 fixture는 손대지 않는다. **PASS**

## 결과

공식 집계는 `78/214`다. `72/213`은 해당 historical records의 작성 시점에만 해당한다. Tailnet pass01은 잘못된 static text를 그대로 보여 주었고, correction PR 병합 뒤 pass02를 게시해 desktop/mobile Agent Browser 결과를 재확인한다.
