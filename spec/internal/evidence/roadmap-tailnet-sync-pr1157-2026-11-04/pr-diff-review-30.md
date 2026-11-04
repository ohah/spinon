# PR #1158 Tailnet 로드맵 동기화 근거 검토 30회

검토 범위는 PR #1157이 main에 병합한 `/spinon/roadmap.html` source와 source revision `f2c1448afe0d8f4322d9aa6b392cfa5544c22deb`, 동기화 script·report·Agent Browser captures·상태 링크다. preview server 설정이나 런타임 코드는 수정하지 않는다.

1. **Tailscale backend가 종료됐는데 동기화 성공으로 기록하는가?** 통과 — preflight와 copy/verify 응답이 backend `Running`을 기록한다.
2. **다른 호스트나 preview server를 대상으로 게시하는가?** 통과 — `macstudio.tailed42f2.ts.net`과 예상 `127.0.0.1:8768` listener만 허용한다.
3. **잘못된 listener process를 유지한 채 route를 교체하는가?** 통과 — PID `28040`의 args가 `tailnet-preview-server.mjs`와 일치함을 확인한다.
4. **동기화 source가 main의 merged source가 아닌가?** 통과 — source bytes를 `origin/main` revision의 `spec/public/roadmap.html`과 byte-for-byte 대조한다.
5. **merge SHA나 새 PR card가 누락된 source를 게시하는가?** 통과 — PR #1156 card 정확히 하나와 merge SHA `b667a80e…`가 source precondition이다.
6. **official count가 다르게 적히거나 누락되는가?** 통과 — `72/213` 존재 여부를 source precondition으로 검사한다.
7. **다른 route까지 덮어쓰는가?** 통과 — 대상은 preview root의 `roadmap.html` 하나다.
8. **`STATUS.html`이 바뀌거나 삭제되는가?** 통과 — 해당 route는 읽거나 교체하지 않고 파일 삭제도 없다.
9. **preview root가 symlink 또는 다른 경로 alias인가?** 통과 — real directory와 canonical path를 요구한다.
10. **기존 target이 symlink나 비정규 파일인데 덮어쓰는가?** 통과 — `lstat`로 regular non-symlink file만 허용한다.
11. **기존 파일이 바뀐 뒤 오래된 backup을 사용하는가?** 통과 — copy는 preflight의 target size·hash를 다시 비교한다.
12. **사용 중인 파일을 무백업 교체하는가?** 통과 — 기존 target이 없거나 정확히 보존 가능한 regular file일 때만 계속한다.
13. **다른 process의 열린 파일을 무시하는가?** 통과 — target handle owner·inode·read-only 상태를 확인하며 preflight에서는 열린 handle이 없었다.
14. **이전 route를 잃는가?** 통과 — 기존 559,228-byte file을 전용 backup에 복사하고 원본 hash를 기록한다.
15. **백업 경로를 재사용해 이전 evidence를 덮어쓰는가?** 통과 — 고유 PR/date 경로 존재 시 중단하고 새 backup만 만든다.
16. **부분 쓰기 상태가 route로 노출되는가?** 통과 — 임시 파일을 같은 directory에 쓴 뒤 atomic rename한다.
17. **실패 정리에서 기존 route나 다른 파일을 삭제하는가?** 통과 — 오류 시 제거 대상은 이번 run의 고유 temp file뿐이고 이전 route는 보존한다.
18. **복사 결과가 source hash와 다른데 통과하는가?** 통과 — byte length와 SHA-256을 source와 비교한다.
19. **server restart가 사용자 preview를 끊는가?** 통과 — 동일 PID `28040`과 listener를 유지하고 재시작하지 않았다.
20. **local server가 HTTP error나 stale content를 돌려주는가?** 통과 — copy 전후 local route를 HTTP 200·size·hash로 확인한다.
21. **Tailnet HTTPS route가 error이거나 stale cache를 돌려주는가?** 통과 — cache-busting query와 `Cache-Control: no-cache`를 적용해 remote hash를 확인한다.
22. **사용자 지정 query 없는 URL은 다른 body를 주는가?** 통과 — exact URL을 no-cache 요청과 Agent Browser에서 별도로 확인하고 동일 source hash·PR marker를 봤다.
23. **remote bytes 또는 served hash가 merged source와 다른가?** 통과 — source, local preview, cache-busted remote, queryless remote의 size/hash가 일치한다.
24. **페이지 title이나 새 merge card가 browser에 보이지 않는가?** 통과 — page title, PR #1156 marker 1개, merge SHA를 DOM에서 확인한다.
25. **공식 count나 C11 미완료 상태가 browser-visible page에서 사라지는가?** 통과 — `72/213`과 C11 미완료 문구가 desktop/mobile DOM에 있다.
26. **desktop layout에 수평 overflow가 생기는가?** 통과 — 1280 CSS px viewport에서 document width가 1280/1280이다.
27. **mobile layout에 수평 overflow가 생기는가?** 통과 — 390 CSS px viewport에서 document width가 390/390이다.
28. **capture 파일이 저장된 route 또는 viewport와 맞지 않는가?** 통과 — desktop/mobile screenshot은 확인된 queryless route와 각 viewport에서 캡처했다.
29. **Browser나 preview process cleanup을 누락하는가?** 통과 — Agent Browser session 종료 뒤 active session은 없고 process 확인도 비었다. 사용자 preview server는 의도대로 살아 있다.
30. **선택 route 확인을 전체 문서 build, link graph, C11/WPT 완료로 과장하는가?** 통과 — README는 route·hash·화면 검증으로 한정하고 전체 build/link/WPT 및 C11 완료를 주장하지 않는다.

## 판정

30개 관점에서 동기화 또는 보고 불일치를 찾지 못했다. raw source, preflight·copy·verify report, queryless URL 결과, desktop/mobile Agent Browser captures를 같은 evidence folder에 보존한다.
