# PR #1067 후속 Tailnet 문서·배포 근거 변경 검토 30개

검토 범위는 선택 게시 빌드 설정과 스크립트, route/hash manifest, backup/copy/served reports, Agent Browser captures 및 실행 요약이다. 각 관점은 별도 유실·잘못된 공개·범위 과장 경로를 점검했다.

1. **source revision 고정** — selected root와 publication manifest가 PR #1067 병합 뒤 `main` HEAD 및 tree SHA에 고정된다: PASS.
2. **tracked-only input** — source selection은 `git ls-files`로 가져와 미추적 artifact를 배포물에 섞지 않는다: PASS.
3. **source symlink 차단** — selected-root script는 각 경로 구성요소의 symlink를 거부한다: PASS.
4. **route traversal 차단** — staging 경로는 절대 경로, backslash, `.` 및 `..` component를 거부한다: PASS.
5. **고유 build root** — task별 `build/spinon/roadmap-tailnet-sync-pr1067-2026-11-03/pass01`이 비어 있을 때만 생성한다: PASS.
6. **기존 artifact 보존** — build, root, publication, manifest를 덮어쓰지 않고 이미 있으면 중단한다: PASS.
7. **RSPress 버전** — 저장소의 고정 `@rspress/core` 2.0.22 config를 재사용한다: PASS.
8. **build isolation** — RSPress root/outDir를 고유 selected-root 경로에 둔다: PASS.
9. **unrelated public directory** — `publicDir:false`로 전체 `spec/public` 자동 복사를 막는다: PASS.
10. **전체 사이트 검증 주장** — dead-link/dead-image 검사를 끈 선택 build이며 full-site 링크 검증을 하지 않았다고 기록한다: PASS.
11. **source hash integrity** — selected-root manifest와 실제 staged bytes의 크기·SHA-256을 대조한다: PASS.
12. **route collision** — 같은 route가 중복 선택되면 SHA-256이 같을 때만 허용한다: PASS.
13. **publication symlink 차단** — output walk와 target inspection에서 symlink를 거부한다: PASS.
14. **manifest completeness** — publication file 수와 manifest route 수가 일치하는지 검사한다: PASS.
15. **roadmap anchor** — 새 PR #1066 카드와 merge SHA 및 `72/213` 표식을 staging 전 검사한다: PASS.
16. **card local links** — 카드의 일곱 Tailnet 내부 링크가 publication route에 전부 포함된다: PASS.
17. **route aliases** — `plan/` 및 `internal/evidence/` 경로를 public URL과 일치하게 매핑한다: PASS.
18. **open spec alias** — 열린 `spec/plan/c11-css-grid.html`과 paired raw Markdown alias를 제외하고 이를 manifest에 기록한다: PASS.
19. **live broad plan route** — 사용자 접근 경로 `/plan/c11-css-grid.html`은 별도로 포함하고 HTTP/hash 검증한다: PASS.
20. **Tailnet preflight** — copy 전에 Tailscale backend `Running`, 예상 PID/명령, `127.0.0.1:8768`, local HTTP 200을 확인한다: PASS.
21. **lsof correction** — 첫 잘못된 CLI invocation은 수정된 preflight 전에 폐기했고 route 변경은 없었다: PASS.
22. **open-reader inventory** — corrected `lsof` scan은 `roadmap.html` 및 `STATUS.html`의 기존 reader PID를 기록한다: PASS.
23. **backup completeness** — 바뀔 기존 route를 모두 unique backup root에 복사하고 hash를 다시 확인한다: PASS.
24. **atomic replacement** — 같은 대상 디렉터리의 고유 temporary file을 완성한 뒤 rename해 partial content를 노출하지 않는다: PASS.
25. **reader/process preservation** — 서버를 재시작하지 않고 열린 reader는 이전 inode를 계속 읽게 둔다: PASS.
26. **no route deletion** — selected operation은 create, replace, same만 수행하고 preview route를 삭제하지 않는다: PASS.
27. **copy source revalidation** — copy 직전 manifest source hash와 preflight 당시 target hash를 다시 검사한다: PASS.
28. **served route hashes** — 11개 핵심 route를 local과 Tailnet에서 HTTP 200으로 받아 source SHA-256과 비교한다: PASS.
29. **visual responsive check** — Agent Browser desktop 1440px 및 mobile 390px에서 카드·count·C11 unchecked 상태와 문서 폭을 확인한다: PASS.
30. **browser/server lifecycle** — 고유 Agent Browser session만 닫고 `active=false`, `pid=null`을 확인했으며 preview PID 28040은 남겼다: PASS.

실패는 발견하지 못했다. 검토는 선택 게시 대상 및 C11 PR #1066 카드에 한정되며 전체 사이트 링크 적합성이나 제품 동작을 재판정하지 않는다.
