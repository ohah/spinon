# C13.1 iOS hit identity grid 병합 상태 동기화 PR 검토 30개

검토일: 2026-10-15
대상: PR #166 병합 결과를 공식 상태·내부 인덱스·실행 evidence에 반영하는 문서 변경
판정: 서로 다른 30개 stale-state·링크·범위 실패 관점에서 확인했다.

1. **PR 상태 오판** — PR #166이 실제 병합됐는지 확인했다. GitHub PR metadata는 state MERGED를 반환한다.
2. **merge SHA 출처 오판** — 임의 로컬 commit을 merge SHA로 기록할 위험을 확인했다. GitHub PR metadata의 mergeCommit OID를 쓴다.
3. **main 반영 누락** — GitHub 상태만 MERGED이고 원격 main에는 포함되지 않은 상황을 확인했다. fetch한 origin/main의 HEAD가 894d01089b2f1c0f8a880855d1ce69b7f74912bf다.
4. **Rebase and merge를 merge commit으로 오표기** — 선형 이력 정책과 실제 main 이력을 대조했다. PR은 Rebase and merge였고 main에는 재작성된 두 commit이 선형으로 기록됐다.
5. **원래 PR head와 main commit 혼동** — 원래 head SHA 4b59b6932d8643e29600c24bd3fafde97ddb4be7을 main SHA로 쓰는지 확인했다. 원래 head는 source reference로만 유지하고 merge 결과 OID는 별도 기록한다.
6. **구현 commit 누락** — PR에 포함된 code change가 main history에 있는지 확인했다. main log의 f51610a는 앱 구현 commit이다.
7. **문서 commit 누락** — PR evidence/status 검토 commit이 main history에 있는지 확인했다. main log의 894d010은 PR evidence review status commit이다.
8. **기준 branch 불일치** — PR base가 main인지 확인했다. GitHub PR metadata의 baseRefName은 main이다.
9. **계획 PR과 구현 PR 번호 혼합** — #165 계획 PR과 #166 구현 PR의 책임을 혼합하는지 확인했다. 각 링크와 merge 기록을 따로 적는다.
10. **Android PR과 iOS PR 혼합** — #164 Android app result를 #166 iOS result로 귀속하는지 확인했다. 각 플랫폼·PR 링크를 분리한다.
11. **병합 날짜 timezone 오류** — UTC timestamp를 현지 merge date로 바꾸면서 하루 경계가 틀리는지 확인했다. 2026-10-14T15:53:34Z는 KST 2026-10-15다.
12. **merge 시각을 evidence 시각으로 오인** — app run과 PR merge가 같은 시각으로 기록되는지 확인했다. Simulator run ID와 merge date를 서로 다른 기록으로 유지한다.
13. **CI 부재를 통과로 표현** — “no checks reported”를 초록 CI pass로 바꾸는지 확인했다. 체크가 보고되지 않았다고 적고 통과 주장에 쓰지 않는다.
14. **필수 status check 설정 추측** — 저장소 정책을 추정하는지 확인했다. main branch protection 응답에 required status checks가 없음을 확인했다.
15. **필수 승인 수 추측** — review 필요 여부를 추정하는지 확인했다. branch protection은 required approving review count 0을 반환했고, 프로젝트의 30개 적대 검토는 별도 evidence로 충족한다.
16. **merge 방식 위반** — squash나 merge commit으로 바뀌었는지 확인했다. PR merge 기록의 방식은 Rebase and merge다.
17. **선형 이력 위반** — 결과 main commit의 parent 관계에 merge commit이 생기는지 확인했다. 재작성된 두 commit이 기존 main 뒤에 순서대로 놓였다.
18. **image URL 소실** — source branch가 삭제돼 PR screenshot이 끊기는지 확인했다. head branch는 merge 후에도 존재했고 raw screenshot URL이 계속 로드된다.
19. **post-merge image 표시 변경** — PR 본문에서 image가 계속 complete인지 확인했다. agent-browser 재탐색에서 complete=true, naturalWidth=1206, naturalHeight=2622를 확인했다.
20. **screenshot 해상도 불일치** — PR 화면 크기가 저장 PNG와 다를 위험을 확인했다. PR DOM natural dimensions와 PNG 원본 해상도가 1206×2622다.
21. **실행 screenshot과 요약 run 불일치** — PR 본문 이미지가 다른 run artifact인지 확인했다. image path는 summary가 가리키는 run-1 screenshot이다.
22. **image load와 기능 결과 혼합** — 이미지 표시만으로 기능 성공을 주장하는지 확인했다. PR image는 화면 증거이며 런타임 판정은 run logs/summary가 맡는다.
23. **runtime 수치 축약 오해** — 플랫폼별 41/41·52/52·30,000/30,000가 합산되는지 확인했다. iOS app 결과는 iPhone 17 Pro/iOS 26.2 Simulator 세 process 각각으로 기록한다.
24. **fixture screenshot을 제품 UI로 오인** — fixture-test 화면을 일반 사용 기능으로 표현하는지 확인했다. 상태는 fixture-test Simulator 검증이며 C13.1 미완료다.
25. **drawable readback 과장** — Metal surface 제출과 pixel readback을 섞는지 확인했다. drawable readback 미수행을 그대로 남긴다.
26. **DOM event/WPT 완료 과장** — identity grid를 event dispatch나 WPT 적합성으로 확장하는지 확인했다. 둘 다 미완료로 유지한다.
27. **문서 경로 끊김** — merge review와 final run README 상대 링크가 실제 tree에 있는지 확인했다. 둘 다 같은 evidence directory에서 존재한다.
28. **상태 대장만 바뀌고 내부 인덱스가 stale** — 공식 상태와 내부 README의 merge state가 다른지 확인했다. 두 문서에 같은 PR #166 및 merge OID를 기록한다.
29. **원래 검토 파일의 시간 관점 파괴** — PR delta review의 “검토 당시 open” 사실을 현재 상태로 오독하는지 확인했다. 그 상태는 검토 당시로 한정하고 이 후속 파일에 병합 결과를 덧붙인다.
30. **문서 동기화가 feature 상태를 승격** — merge 상태 기록을 C13.1 제품 완료로 오해하는지 확인했다. 후속 검증은 기록하되 제품 지원 상태는 미완료로 유지한다.

## 확인 근거

- PR #166 GitHub state: MERGED
- GitHub mergeCommit OID와 origin/main HEAD: 894d01089b2f1c0f8a880855d1ce69b7f74912bf
- GitHub mergedAt: 2026-10-14T15:53:34Z, Asia/Seoul 2026-10-15
- agent-browser post-merge image: loaded, 1206×2622
- 범위: 상태 문서 동기화. 앱 코드나 런타임 동작은 변경하지 않는다.
