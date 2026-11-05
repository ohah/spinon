# C11.4 PR #1196 Tailnet evidence PR diff 검토 · 30개 관점

대상 PR은 PR #1196의 roadmap 변경 후 Tailnet 제공 파일·화면과 동기화 로그, 공식 문서 evidence pointer를 추가한다. Tailnet 파일은 PR #1196의 merged `main` SHA에서 생성했고, local HTTP와 remote HTTPS hash를 대조한 뒤 Agent Browser로 desktop/mobile 화면을 검사했다.

1. **Main source identity:** `sourceHead`는 PR #1196 merge SHA `b746dae9ba662da65b0ca2867934362ee9a40d09`다.
2. **PR lifecycle:** GitHub PR #1196 state는 `MERGED`이며 plan·README 날짜와 merge evidence가 맞는다.
3. **No local draft leak:** preflight의 tracked diff digest는 empty tree SHA이며 branch uncommitted code를 게시하지 않았다.
4. **Tailnet identity:** 상태 확인 대상은 `macstudio.tailed42f2.ts.net`의 사용자가 지정한 `/spinon/roadmap.html` route다.
5. **Backend state:** preflight에서 Tailscale backend `Running`을 기록했다.
6. **Server identity:** local listener PID 28040은 `tailnet-preview-server.mjs` process와 일치한다.
7. **Server address:** listener는 `127.0.0.1:8768`에서 확인됐다.
8. **No server replacement:** 기존 PID를 사용했고 새 preview server를 병행 실행하지 않았다.
9. **No restart:** copy/verify report에서 `serverRestarted`가 `false`다.
10. **Single route scope:** source·target 모두 `roadmap.html` 하나다.
11. **Source length:** main source는 584,210 bytes이며 preflight와 served report가 같은 값을 기록한다.
12. **Old content preservation:** 교체 전 대상 SHA는 `4cf96a01…e7f54`로 기록됐다.
13. **Backup isolation:** backup은 PR 번호와 날짜가 든 새 경로에 만들고 기존 backup overwrite를 차단한다.
14. **Backup identity:** `copy-and-backup.json`의 backup hash가 직전 target hash와 같다.
15. **Open-handle guard:** target file을 여는 PID가 없음을 copy 전 확인했다.
16. **Atomic replacement:** unique temporary file과 atomic rename을 사용한다.
17. **Deletion guard:** 보고서가 `noDeletes: true`이며 기존 route 제거는 없다.
18. **Source digest:** source SHA-256은 `42369b6d…c1255`다.
19. **Local response:** local route가 HTTP 200을 반환한다.
20. **Remote response:** Tailnet HTTPS route가 HTTP 200을 반환한다.
21. **Local payload identity:** local bytes와 SHA-256이 source와 일치한다.
22. **Remote payload identity:** remote bytes와 SHA-256이 source와 일치한다.
23. **Desktop viewport:** Agent Browser desktop JSON은 1440×1000을 기록한다.
24. **Desktop horizontal bounds:** document/body width는 1440px이고 viewport 밖으로 늘지 않았다.
25. **Desktop roadmap card:** PR #1195 card x=90px, width=928px, right=1018px다.
26. **Mobile viewport:** Agent Browser mobile JSON은 390×844를 기록한다.
27. **Mobile page overflow:** document/body width는 390px이어서 horizontal overflow가 없다.
28. **Mobile card bounds:** PR #1195 card x=14px, width=362px, right=376px다.
29. **Rendered status:** desktop/mobile 모두 live counter와 동기화 note에서 `78/214`를 표시하고 C11.4/C11 미완료를 보인다.
30. **Browser cleanup:** Agent Browser를 닫은 뒤 Agent Browser/Chrome process가 남지 않았고, Tailnet preview server는 의도대로 유지됐다.

## 검증 경계

전체 RSPress build의 dead-link scan은 exit 1이었고 변경 전 checkout 비교는 없다. 이 PR은 정적 roadmap 제공본과 선택된 viewport 화면만 기록하며 전체 docs build, CI, GitHub Pages 배포나 C11.4 전체 완료를 주장하지 않는다.
