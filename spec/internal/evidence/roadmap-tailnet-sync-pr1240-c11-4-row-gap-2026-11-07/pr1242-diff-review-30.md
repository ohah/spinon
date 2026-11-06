# PR #1242 Tailnet 동기화·상태 기록 diff 실패 관점 검토 30개

**검토 대상:** PR [#1242](https://github.com/ohah/spinon/pull/1242), base `main` at `79b2200bbf3f0c3756bcddd48e6591d2aa128ff2`. 변경은 STATUS·plan·내부 계약과 PR #1240 Tailnet 동기화 evidence다. `spec/public/roadmap.html` source는 이전 PR #1241에서 병합·동기화된 SHA 그대로다.

## 검토 항목

1. **소스 식별자:** `spec/public/roadmap.html` SHA-256 `65b9828a...eabd0`를 현재 source와 대조했다. **통과.**
2. **구현 PR 식별자:** PR #1240 merge SHA `e3b32918e3893ce8b6bfc835d194c3abad12bc2a`가 evidence, local response, Tailnet HTTPS response에 있다. **통과.**
3. **문서 PR 순서:** roadmap source를 변경한 PR #1241은 PR #1240 implementation 뒤에 Rebase and merge됐다. **통과.**
4. **기존 제공본 보존:** 교체 전 bytes를 별도 pass01 backup 경로에 보존했다. **통과.**
5. **Backup 무결성:** backup size와 SHA가 변경 직전 제공본 609,758 bytes / `0c49befe...a41dcb`와 일치한다. **통과.**
6. **기존 HTTP response preflight:** 복사 전에 local HTTP가 이전 제공 파일과 같은 status 200, size, SHA를 반환했다. **통과.**
7. **기존 HTTPS response preflight:** 복사 전에 Tailscale HTTPS가 이전 제공 파일과 같은 status 200, size, SHA를 반환했다. **통과.**
8. **Tailnet 상태:** 복사 직전 Tailscale backend가 `Running`이었다. **통과.**
9. **Server identity:** PID 28040 command line은 Spinon `tailnet-preview-server.mjs`였고 `127.0.0.1:8768` listener와 일치했다. **통과.**
10. **Target open-handle 보호:** 교체 전 `lsof`에서 `roadmap.html` target open handle이 없었다. **통과.**
11. **Unique backup destination:** PR #1240 전용 backup directory가 비어 있음을 확인해 이전 backup을 덮어쓰지 않았다. **통과.**
12. **Atomic replacement:** 새 bytes는 target과 같은 디렉터리의 unique temp 파일에 write/fsync 후 rename했다. **통과.**
13. **Partial-write recovery:** target file hash mismatch면 원래 backup을 atomic restore하도록 script가 처리한다. **통과.**
14. **Served response recovery:** local/HTTPS served verification mismatch면 이전 backup으로 target을 restore하도록 script가 처리한다. **통과.**
15. **변경 경로 제한:** sync record의 `filesChanged`는 `/spinon/roadmap.html` 한 경로다. **통과.**
16. **Server lifecycle:** preview server PID 28040을 재시작·종료하지 않았고 복사 뒤에도 같은 PID가 listen한다. **통과.**
17. **Local final status:** local HTTP는 200이며 size 611,424, SHA `65b9828a...eabd0`을 반환한다. **통과.**
18. **Tailnet final status:** HTTPS는 200이며 local/source와 같은 size/SHA를 반환한다. **통과.**
19. **Content marker:** 두 served response가 PR #1240와 merge SHA를 포함한다. **통과.**
20. **Official completion marker:** 두 served response가 `78/214`를 포함한다. **통과.**
21. **미완료 marker:** 두 served response가 C11.4/C11 미완료 표시를 보존한다. **통과.**
22. **Desktop Agent Browser target:** Agent Browser에서 지정 HTTPS Tailnet URL을 열고 `스피논 구현 로드맵` title과 최신 #1240 카드가 첫 번째임을 확인했다. **통과.**
23. **Mobile Agent Browser target:** 같은 HTTPS URL에서 390×844 CSS px layout을 확인했다. **통과.**
24. **Desktop overflow/layout:** 1440×1000에서 document/body 1440px, horizontal overflow 없음, 카드 geometry x90/width928/right1018이다. **통과.**
25. **Mobile overflow/layout:** 390×844에서 document/body 390px, horizontal overflow 없음, 카드 geometry x14/width362/right376이다. **통과.**
26. **Mobile card readability:** merged PR, SHA, 공식 count, 미완료 문구가 모바일 폭에서 wrap되어 표시된다. **통과.**
27. **Screenshot provenance:** Desktop 1440×1000·mobile 390×844 screenshots의 pixel dimensions와 SHA를 DOM record와 함께 보존했다. **통과.**
28. **Agent Browser cleanup:** 전용 session close exit 0이며 이후 `pgrep -fl '^agent-browser'`는 비어 있었다. **통과.**
29. **Deployment scope:** 이 기록은 private Tailnet preview만 다루며 GitHub Pages/`gh-pages` deploy는 실행하지 않았다. **통과.**
30. **Official state consistency:** STATUS, plan, 0059는 bounded PR #1240을 merged로 기록하고 78/214·C11.4/C11 미완료·contract 0.1.0을 유지한다. **통과.**

## 결과

위 30개 관점에서 차단 결함을 찾지 못했다. Tailnet UI 확인은 Agent Browser로 했고 터미널 브라우저를 사용하지 않았다. Screenshot과 DOM/response records는 PR evidence에 포함한다.
