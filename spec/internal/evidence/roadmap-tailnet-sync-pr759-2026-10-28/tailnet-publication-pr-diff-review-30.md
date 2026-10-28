# Tailnet 공개 로드맵 publication PR diff 검토 30개

대상 diff는 PR #759 병합 후 `roadmap.html` 한 파일을 Tailnet preview로 동기화한 실행 기록과 재현 스크립트다. 아래 관점은 파일 경계, 쓰기 안전성, 제공 경로와 브라우저 관찰을 서로 다른 실패 경로로 점검한다.

1. **병합 source pin** — script는 `origin/main`이 공개 로드맵 source SHA를 포함하는 정확한 `3210face8b77dd921c403188fe73f2c6993c53c2`일 때만 실행한다.
2. **Git/worktree 일치** — `git show origin/main:spec/public/roadmap.html`와 체크아웃 source bytes가 다르면 중단한다.
3. **출처 내용 hash** — 374,155 bytes의 SHA-256을 기대값과 비교해 다른 build나 이전 PR 문서가 게시되는 경로를 막는다.
4. **source semantics** — PR #759 merge marker와 `72/213`, C11.4 미완료 표식이 출처 안에 있는지 확인한다.
5. **target root identity** — preview root의 실제 경로를 검사하고 symlink를 허용하지 않는다.
6. **target file type** — `roadmap.html`이 regular file이고 symlink가 아닐 때만 교체한다.
7. **기존 파일 pin** — 바뀌기 전 372,287-byte target SHA가 사전 고정값과 일치해야 한다.
8. **local route baseline** — 교체 전 loopback route가 HTTP 200과 기존 target hash를 반환한 기록이 있다.
9. **Tailnet route baseline** — 교체 전 공개 Tailnet URL도 같은 기존 bytes/hash를 제공해야 진행한다.
10. **Tailscale 상태** — backend `Running`이 아니면 외부 publication 단계가 중단된다.
11. **listener 유일성** — `8768`의 listener가 PID 28040 하나인지 검사한다.
12. **서버 소유권** — PID command에 정해진 `tailnet-preview-server.mjs`가 없으면 임의 서버에 쓰지 않는다.
13. **동시 connection** — 사후 preflight에서 HTTP 요청 때문에 남은 loopback 연결이 listener-only로 끝날 때까지 제한 시간 안에 기다린다.
14. **대상 open handle** — 교체 직전 roadmap에 열린 handle이 있으면 교체하지 않도록 검증한다.
15. **backup parent identity** — backup parent가 실제 directory인지 확인하고 symlink 경로를 거부한다.
16. **backup 재사용 방지** — 날짜별 작업 backup root가 이미 있으면 덮어쓰지 않고 중단한다.
17. **backup byte equality** — source를 바꾸기 전에 이전 target 전체 bytes를 exclusive create로 보존하고 hash를 대조한다.
18. **backup mode** — 백업본에 이전 파일 권한 비트를 적용해 증거 보존 중의 접근 mode 차이를 제한한다.
19. **selected route scope** — 고정 대상은 `/spinon/roadmap.html` 한 개이며 사용자 입력으로 임의 경로를 조합하지 않는다.
20. **same-directory temporary** — 교체 임시 파일은 같은 preview directory에 exclusive create해 다른 경로의 rename 문제를 피한다.
21. **partial-write durability** — 임시 파일을 fsync한 다음 replace한다.
22. **atomic visibility** — `os.replace`로 독자가 반쪽 HTML을 읽는 경로를 피한다.
23. **directory durability** — rename 후 preview directory fd를 fsync한다.
24. **permission preservation** — 새 target에 기존 roadmap mode를 적용한다.
25. **temporary cleanup** — replace 전에 실패해도 script가 만든 임시 파일만 정리한다. 기존 page나 backup은 삭제하지 않는다.
26. **installed-byte check** — 바뀐 target의 bytes와 SHA를 merged source와 정확히 대조한다.
27. **local after check** — local route가 HTTP 200, source와 같은 byte 수 및 SHA인지 확인한다.
28. **Tailnet after check** — Tailnet route도 HTTP 200과 같은 content hash를 반환해야 성공으로 기록한다.
29. **runtime ownership** — 동기화 전후 listener PID가 같아 기존 preview server를 중단하거나 재시작하지 않는다.
30. **render·lifecycle evidence** — Agent Browser DOM에서 PR #759, 5/5 두 플랫폼, `72/213`, C11.4 미완료를 확인하고 캡처를 보존하며 close 뒤 session `active:false`, `pid:null`을 기록한다.

최종 결과는 단일 route의 hash 일치와 렌더된 페이지 확인이다. 전체 문서 사이트 deploy, C11.4 완료, CI 통과 또는 새 앱 runtime 검증으로 확대 해석하지 않는다.
