# C11.4 PreWrap 병합 후 상태·Tailnet sync 변경 실패 관점 30개

기준은 PR #870 병합 뒤 main에서 만든 선택 문서 PR diff다. 범위는 official status 및 public roadmap 상태 문구, pass02 Tailnet sync 스크립트와 route/hash/browser evidence다. 기능 코드나 CSS 지원 범위를 확장하지 않는다.

1. **계획 PR과 구현 PR 혼동** — #869는 계획, #870은 제한 fixture 구현으로 분리해 적었다. 통과.
2. **PR 상태 오기** — `gh pr view`의 state `MERGED`와 URL을 확인했다. 통과.
3. **merge 시각 오기** — GitHub `mergedAt` `2026-10-30T18:51:40Z`를 기록했다. 통과.
4. **merge commit 오기** — `mergeCommit.oid`와 main HEAD `5265793d71a02a6206a09b9bbbc9658696dfd992`가 일치한다. 통과.
5. **main 반영 전 성공 주장** — merge commit이 `origin/main`의 ancestor인지 확인했다. 통과.
6. **PR head diff와 검토본 불일치** — GitHub PR diff와 로컬 `origin/main...bf817aff25c5d39dcec45a744102cac9a0d9d0d7` patch의 변경 내용이 일치했다. hunk context 함수명만 달랐다. 통과.
7. **C11 parent 미완료 상태 승격** — 공식 checkbox는 미완료로 남았다. 통과.
8. **C11.4 부분 fixture를 상위 완료로 승격** — C11.4 checkbox도 미완료다. 통과.
9. **공식 완료 수 임의 증가** — STATUS와 두 페이지에서 `72/213`을 유지한다. 통과.
10. **CI 결과 과장** — PR rollup은 비었고 required check도 설정되지 않았다. 문서에 CI 통과를 주장하지 않는다. 통과.
11. **workspace test 실패 은폐** — C07.3 assertion의 expected `1.77778 / 1`, actual `1.7777778 / 1`을 유지한다. 통과.
12. **실패를 변경이 유발했다고 단정** — 실패 crate가 후보 diff 바깥이며 base에서도 실패하는지는 증명되지 않았다고 둔다. 통과.
13. **Android AVD를 실기기로 확대** — Android 결과를 API 37 ARM64 AVD로 한정한다. 통과.
14. **iOS Simulator를 실기기로 확대** — iPhone 17 Pro/iOS 26.2 Simulator로 명시한다. 통과.
15. **hardware GPU 주장** — hardware GPU는 검증 범위 밖으로 유지한다. 통과.
16. **glyph pixel 주장** — harness 캡처는 glyph ink/pixel oracle이 아니라고 둔다. 통과.
17. **일반 author CSS admission 주장** — fixture opt-in 범위만 기록한다. 통과.
18. **root HTML parity 혼동** — root-HTML white-space mapping은 완료 범위에 포함되지 않는다. 통과.
19. **일반 authored `tab-size` 과장** — 고정 unit-test profile 외 지원을 주장하지 않는다. 통과.
20. **Tailnet server 재시작 주장** — PID 28040 유지와 no-restart를 reports로 확인했다. 통과.
21. **Tailnet 경로 오배포** — `127.0.0.1:8768` preview root 및 `/spinon/` base를 확인했다. 통과.
22. **route manifest와 build 불일치** — publication manifest의 340 route를 source file과 대조했다. 통과.
23. **served route 누락** — HTTPS route 340/340이 응답했다. 통과.
24. **HTTP 상태 오류** — 340/340 route가 HTTP 200이다. 통과.
25. **served bytes/hash 손상** — 모든 HTTPS response bytes와 SHA-256이 publication manifest에 일치한다. 통과.
26. **기존 파일 무백업 덮어쓰기** — 교체 31개에 대한 backup hash를 확인했다. 통과.
27. **변경 중 열린 파일 덮어쓰기** — 사전 `lsof` 결과 open handle 0이다. 통과.
28. **의도치 않은 삭제** — copy report와 verify가 noDeletes를 기록하며 삭제를 수행하지 않았다. 통과.
29. **첫 copy 오류 후 불명확한 상태** — `EEXIST`가 파일 복사 전 발생한 것을 확인했고, symlink 검증을 유지한 pass02 parent handling으로 재실행해 통과했다. 통과.
30. **Agent Browser 종료 누락 또는 터미널 브라우저 사용** — Agent Browser에서 두 페이지를 확인한 뒤 session와 all-session close를 성공시켰고, process search에서 browser/Chrome이 남지 않았다. terminal-browser는 사용하지 않았다. 통과.

**판정:** 상태·배포 증거에서 추가 blocker를 찾지 못했다. 선택 route 결과는 전체 문서 사이트나 C11 적합성으로 확대하지 않는다.
