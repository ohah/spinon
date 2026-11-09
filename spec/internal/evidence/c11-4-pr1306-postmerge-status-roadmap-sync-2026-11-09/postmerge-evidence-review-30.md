# Tailnet 동기화 병합 후 evidence 대조 검토 30개

검토 대상은 PR #1308 병합 후 `origin/main`·선택된 Tailnet 파일·serve 응답 간 hash binding 및 lifecycle 기록이다.

| # | 실패 관점 | 결과 |
|---|---|---|
| 1 | 병합 후 확인을 PR #1308 이전 시각으로 적는다. | PASS — merge 시각 `2026-11-09T10:07:59Z`, 검증 시각 `10:09:37Z`다. |
| 2 | 문서 PR merge SHA가 다른 커밋을 가리킨다. | PASS — PR info와 `origin/main` head가 `cdd00d6da193848fbc39dda9685a2759fdf554a4`다. |
| 3 | 파일 blob ID가 병합 후 candidate 내용과 다르다. | PASS — main blob은 `cb980ec9f8fe81613d7481b56d9a28a3fb59b2b8`이다. |
| 4 | content SHA-256이 candidate source와 다르다. | PASS — source와 모든 serve 응답이 `968ffa...`다. |
| 5 | source byte length가 mismatch인데 hash만 복사해 적는다. | PASS — main/source/served 모두 650,253 bytes다. |
| 6 | local checkout은 merge 전 파일을 보여준다. | PASS — local source와 `origin/main` bytes/hash를 각각 비교했다. |
| 7 | origin/main fetch를 생략한 채 로컬 commit만 확인한다. | PASS — `git fetch origin main` 뒤 `origin/main`에서 file blob을 확인했다. |
| 8 | loopback 응답이 404 또는 stale bytes다. | PASS — `127.0.0.1:8768`은 200이며 source와 hash가 같다. |
| 9 | Tailnet HTTPS 응답이 404 또는 stale bytes다. | PASS — Tailnet URL은 200이며 source와 hash가 같다. |
| 10 | source·local serve·remote serve 중 한 hash만 다르다. | PASS — 네 개 측정값이 모두 같은 SHA-256이다. |
| 11 | pass02 preflight를 post-merge evidence로 오인한다. | PASS — pass02는 candidate sync, 이 JSON은 PR #1308 merge 뒤 검증으로 분리한다. |
| 12 | pass02 repository HEAD와 edited worktree bytes의 차이를 숨긴다. | PASS — `candidate-source-binding.json`은 base HEAD와 후보 blob/hash를 별도로 묶는다. |
| 13 | 첫 pass의 broken link 제공본 backup을 덮어쓴다. | PASS — pass01과 pass02 backup은 서로 다른 경로에 보존된다. |
| 14 | backup hash가 교체 직전 제공본과 불일치한다. | PASS — 두 pass의 copy-and-backup 기록에서 이를 대조한다. |
| 15 | 범위를 넘어 다른 preview 파일을 변경한다. | PASS — sync 기록의 `filesChanged`는 `/spinon/roadmap.html`뿐이다. |
| 16 | 잘못된 listener process가 선택되었다. | PASS — listener PID 28040은 preview server 명령과 일치한다. |
| 17 | Tailscale가 내려간 상태를 숨긴다. | PASS — backend는 `Running`으로 기록됐다. |
| 18 | hash 갱신 뒤 preview server를 불필요하게 재시작한다. | PASS — PID 28040 유지, restart false다. |
| 19 | private preview를 공식 Pages 배포로 표현한다. | PASS — GitHub Pages deployment는 false로 기록한다. |
| 20 | 구현 수가 C11.4 bounded merge 때문에 증가한다. | PASS — `78/214`로 유지한다. |
| 21 | C11.4/C11 상태를 완료로 읽게 한다. | PASS — 미완료 상태를 유지한다. |
| 22 | 내부 숫자 계약을 바꿔 적는다. | PASS — `0.1.0`을 유지한다. |
| 23 | 후속 slice 후보를 이미 확정한 것처럼 표시한다. | PASS — 다음 bounded 후보 미선정 표기를 유지한다. |
| 24 | PR #1306 card link fix 없이 404를 통과로 덮는다. | PASS — 기존 review row 22에 발견/수정 사실을 남겼다. |
| 25 | Agent Browser가 확인한 화면과 현재 페이지 내용을 혼동한다. | PASS — 캡처와 DOM/폭 결과를 별도 파일로 보존한다. |
| 26 | Agent Browser session을 사용 후 열어둔다. | PASS — active session 0개와 close 결과를 기록했다. |
| 27 | 검증 끝에 iOS Simulator가 booted 상태다. | PASS — booted iOS simulator 0개다. |
| 28 | 검증 끝에 Android emulator가 ADB 연결 상태다. | PASS — ADB device 0개다. |
| 29 | Android/iOS screenshot 링크를 파일 존재만으로 통과 처리한다. | PASS — Agent Browser에서 각각 1080×2400·1206×2622로 로드했다. |
| 30 | 전체 Rspress build의 dead-link 오류를 무시한다. | PASS — 전체 build는 실패로 기록하고 성공/CI green 주장을 하지 않는다. |

이 검토는 병합된 한 HTML 파일의 content integrity, selected preview 제공과 lifecycle 기록만 다룬다.
