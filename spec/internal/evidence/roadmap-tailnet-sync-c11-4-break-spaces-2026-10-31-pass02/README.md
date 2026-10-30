# C11.4 break-spaces Tailnet 로드맵 동기화

## 결과

PR [#873](https://github.com/ohah/spinon/pull/873)은 2026-10-30T20:02:37Z에 Rebase and merge됐고 merge SHA는 `32ba37b18dc5a8fc553667f52cc47d46dae3952f`다. 로드맵과 공식 상태에 병합 정보를 반영했으며 C11.4/C11 완료 표시는 그대로 미완료, 공식 완료 수는 `72/213`으로 유지했다. main required status checks는 설정되지 않았고 PR check rollup도 비어 있어 CI 통과라고 기록하지 않았다.

Rspress 2.0.22로 선택한 문서 104개 경로(10,786,433 bytes)를 빌드하고 Tailnet 미리보기와 대조했다. Tailscale backend는 `Running`, 기존 preview server PID `28040`은 `127.0.0.1:8768`에서 응답했다. 서버를 재시작하지 않았다. 사전 검사에서 신규 75개, 교체 11개, 동일 18개, 변경 대상 open handle 0개를 확인했다. 기존 교체 파일 11개를 백업했다. 삭제는 없었다. 마지막 검증에서 HTTPS 104/104 경로가 HTTP 200을 반환했고 모든 응답 크기·SHA-256이 publication manifest와 일치했으며 backup hash도 11/11 일치했다.

Agent Browser로 [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html), [공식 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html), [C11.4 runtime evidence](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-anonymous-grid-text-break-spaces-runtime-2026-10-31/README.html)를 확인했다. 로드맵에는 `white-space:break-spaces`, 26/26 Android/iOS matrix, PR #873과 merge SHA, `72/213`이 보였다. 공식 상태의 C11.4 checkbox는 꺼져 있다. runtime evidence의 Android 1080×2400 및 iOS 1206×2622 PNG가 모두 완료 상태로 로드됐다. Agent Browser `close --all` 뒤 session은 `active=false`, PID 없음이었다. terminal-browser는 사용하지 않았다.

첫 Rspress 시도에서는 runtime README의 이미지 상대 경로가 `screenshots/...`라 module import로 해석되어 실패했다. 링크를 `./screenshots/...`로 바로잡은 뒤 pass02 빌드가 통과했다. [첫 실패 로그](rspress-build-first-pass01.log)와 [통과한 빌드 로그](rspress-build.log)에 둘 다 남겼다.

로그의 오른쪽 정렬용 trailing spaces는 Git whitespace 검사가 통과하도록 제거했다. 실행 원문은 별도 생성 build 경로에 남아 있다.

## 재현·원본 자료

- 선택 source HEAD `32ba37b18dc5a8fc553667f52cc47d46dae3952f`, tree `8eefb38009fafea47b82053550519a34576b3970`, tracked diff SHA-256 `64a3dd0c3d9ccc3649544903b4d91a4e1ed2e2515dee681d5310a56af60d031a`: [selected-root.json](selected-root.json)
- Rspress 2.0.22: [build log](rspress-build.log) · [publication manifest](selected-publication.json)
- 대상 사전 상태와 open-handle 검사: [preflight](preflight.json)
- 복사·백업 결과: [copy-and-backup](copy-and-backup.json)
- 로컬 파일, backup, HTTPS 응답 hash 검증: [served verification](served-verification.json)
- Agent Browser 페이지 결과와 화면: [검증 JSON](agent-browser-verification.json) · [로드맵 viewport](tailnet-roadmap-agent-browser-viewport.png) · [상태 viewport](tailnet-status-c114-agent-browser-viewport.png) · [runtime viewport](tailnet-runtime-agent-browser-viewport.png)
- Browser 종료: [lifecycle](agent-browser-lifecycle.json) · [close 출력](agent-browser-close.log) · [종료 후 session 조회](agent-browser-session-info.json)
- PR 문서 diff의 [실패 관점 검토 30개](pr-diff-review-30.md) · [최종 변경·publication 검증](pre-commit-validation.json)
- 재현 스크립트: [selected root](prepare-selected-root.mjs) · [Rspress config](rspress.config.ts) · [publication staging](stage-publication.mjs) · [preflight/copy/verify](sync-tailnet.mjs)

이 검증은 선택한 104개 route와 세 페이지의 브라우저 확인으로 한정한다. 전체 문서 dead link, GitHub Pages 배포, 전체 C11 적합성, 실기기 렌더링이나 glyph pixel 정합성을 증명하지 않는다.
