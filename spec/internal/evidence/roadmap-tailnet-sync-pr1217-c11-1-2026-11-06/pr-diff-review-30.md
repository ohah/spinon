# PR diff 적대 검토 30개

검토 대상은 `6d5220ab47e79cbc131e84c4e0932b828cd86a12` 이후의 실제 staged diff다. 서로 다른 실패 경로를 확인했고, 아래 범위 밖의 전체 RSPress 문서 build 성공이나 Tailnet 전체 route 동기화를 주장하지 않는다.

1. **원본 파일 선택:** `/spinon/roadmap.html`의 source가 별도 생성본이 아닌 `spec/public/roadmap.html`인지 확인했다. 일치한다.
2. **source revision:** preflight가 main의 `6d5220ab47e79cbc131e84c4e0932b828cd86a12`와 tree `141b8f2a434f0386a01f15f022cd35cfec948fa7`를 기록한다.
3. **새 merge identity:** source와 local/remote served response에 PR #1216 merge SHA `ec97003197569ca646563fc36afb386708d210a9`가 있다.
4. **공식 counter:** source와 브라우저 화면에서 `78/214`를 확인했다. Tailnet 복사 단계는 상태 checkbox나 집계를 수정하지 않는다.
5. **부모 범위:** 화면에 C11.1/C11 미완료가 표시되고 README도 구현 전체 완료로 확대하지 않는다.
6. **stale 제공본 감지:** preflight가 기존 595,374-byte SHA를 기록하고 이를 새 source SHA와 구분한다.
7. **실제 serve root:** preview server의 cwd와 canonical root가 `/Users/yoonhb/Library/Application Support/SpinonPreview`로 일치한다.
8. **서버 정체성:** port 8768 listener가 하나이고 PID 28040의 command가 전용 `tailnet-preview-server.mjs`다.
9. **Tailscale 상태:** 파일 변경 전 backend `Running`과 remote HTTPS 200을 확인했다.
10. **symlink 경계:** preview root와 source/target 파일 검사가 symlink를 일반 파일처럼 따라가지 않는다.
11. **열린 파일 보호:** 변경 전 및 copy 직전에 target의 열린 PID가 비어야 진행한다. 두 검사 모두 비었다.
12. **사전 산출물 보존:** preflight/copy/verify JSON과 backup 경로가 이미 있으면 덮어쓰지 않고 중단한다.
13. **백업 충돌 보호:** 지정된 PR #1217 pass01 backup 경로가 존재하지 않는 것을 확인한 뒤 생성했다.
14. **이전 제공본 보존:** backup `roadmap.html`의 SHA가 교체 전 target SHA와 같고 595,374 bytes다.
15. **copy 시 race 방지:** preflight 뒤 source 및 target hash를 다시 검사해 변동이 있으면 중단한다.
16. **copy 시 open-handle 재검사:** 교체 직전에 `lsof`를 재실행하고 열린 target을 거부한다.
17. **원자 교체:** 임시 파일을 target과 같은 디렉터리에 만든 뒤 rename해 부분 기록이 노출되지 않게 한다.
18. **권한 보존:** 교체 전 target의 mode bits를 임시 파일에 보존한다.
19. **바이트 무결성:** atomic rename 후 target 크기와 SHA가 source와 일치한다.
20. **삭제 부작용:** sync 모드는 `roadmap.html` 하나만 교체하고 기존 파일을 삭제하지 않는다.
21. **서버 수명:** copy 전후 PID 28040을 유지하고 서버 재시작은 하지 않았다.
22. **로컬 endpoint:** local route가 HTTP 200이며 served bytes가 source와 같다.
23. **Tailnet endpoint:** 사용자가 지정한 HTTPS route가 HTTP 200이며 served bytes가 source와 같다.
24. **검증 범위:** local과 remote에서 모두 source byte count `596819`와 SHA `817f462e737cc797a341959dbf1aeb86fbc8c3010efb931aaf0a7fa3a1d5bdfa`를 대조했다.
25. **cache 경로:** remote fetch에 `cache: no-store`를 지정해 브라우저 cache 응답을 서버의 최신 제공본으로 오인하지 않는다.
26. **브라우저 origin:** Agent Browser가 localhost가 아닌 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 열었다.
27. **화면 identity:** desktop/mobile DOM에서 제목 `스피논 구현 로드맵`, PR #1216, merge SHA 및 `78/214`를 확인했다.
28. **desktop geometry:** 1440 CSS px viewport에서 document/body 폭이 1440px이고 대상 카드가 표시되며 가로 overflow가 없다.
29. **mobile geometry:** 390 CSS px viewport에서 document/body 폭이 390px이고 대상 카드가 표시되며 가로 overflow가 없다.
30. **screenshot/process 증거:** desktop 1440×1000 및 mobile 390×844 PNG hash를 기록했고, 종료 후 세션 inactive·`pgrep -x agent-browser` 빈 결과를 기록했다.

추가 경계: full-site RSPress build 실패는 별도 JSON에 남겼고 성공으로 세지 않았다. 이에 따라 이 diff는 `roadmap.html` 외 다른 Tailnet 문서 route를 갱신했다고 주장하지 않는다. 공식 상태 checkbox 수에는 변화가 없다.
