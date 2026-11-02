# PR #1075 병합 후 Tailnet 상태 대장 동기화 — 2026-11-03

문서 정정 PR [#1075](https://github.com/ohah/spinon/pull/1075)은 2026-11-02T18:05:20Z Rebase and merge됐고 merge SHA는 `42d591f3f2509a0b5c9795815eb45ea206c0197d`다. 공개 roadmap의 PR #1052/#1072 기록은 이미 최신이었다. Tailnet `/STATUS.html`에는 #1070의 오래된 `implementation candidate` 표기가 남아 있어, 공식 상태 대장 route를 갱신했다.

## 선택 빌드와 발행

RSPress `2.0.22`로 Git 추적 source만 선택해 81개 파일, 120개 route, 총 12,858,919 bytes의 publication을 만들었다. 선택 root의 symlink와 manifest 밖 route는 거부한다. 선택 build는 전체 저장소 dead-link 검증을 수행하지 않는다. [빌드 로그](./rspress-build.log)는 터미널 표의 후행 공백만 제거해 보존하며, 입력 목록과 해시는 [selected-root.json](./selected-root.json) 및 [selected-publication.json](./selected-publication.json)에 있다.

전체 `bun run docs:build`는 실행하지 않았다. 따라서 전체 사이트 build 또는 전역 link 검증 성공을 주장하지 않는다. 선택 RSPress build는 성공했다.

## Tailnet 갱신과 hash 확인

갱신 전 Tailscale backend는 `Running`, 기존 preview server PID `28040`은 `127.0.0.1:8768`에서 수신 중이었고 local roadmap는 HTTP 200이었다. 갱신 전 live `/STATUS.html`에는 stale label이 실제로 남아 있었다.

사전 점검은 120개 route 중 17개 생성, 16개 교체, 87개 동일을 예상했다. 유일한 기존 파일 reader는 교체 대상 `STATUS.html`을 열고 있던 preview server PID였다. 16개 기존 파일을 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1075-2026-11-03/pass01`에 보존하고 같은 디렉터리 임시 파일에서 atomic rename으로 발행했다. route는 삭제하지 않았고 server를 재시작하지 않았다. 자세한 내역은 [preflight](./preflight.json), [copy and backup](./copy-and-backup.json), [served verification](./served-verification.json)에 있다.

Roadmap, STATUS, C11 plan, 계약 0059 및 실행 근거 route를 포함한 주요 10개 route가 local HTTP와 Tailnet HTTPS에서 모두 200을 반환했고, 각 응답 byte 수와 SHA-256이 publication manifest와 일치했다. `/STATUS.html`은 SHA-256 `97e62b8efeabca866ce1cfac84b149e86277ecb2e99faa3f4bb406362fa3c7ae`, 기존 public `/roadmap.html`은 `b8fa0013fb6bde01ea63a5fd0080f71e94a5e51617278b8ed66ea40872c53033`이다. 공식 count는 계속 `72/213`, C11은 미완료다.

## Agent Browser 확인과 종료

Agent Browser로 갱신된 Tailnet `/STATUS.html`을 열어 공식 count, C11 제목, `grid-auto-rows`의 제한 runtime slice 표기를 확인했다. 화면 너비는 1280px viewport에서 1280px이었다. [Agent Browser 화면](./status-agent-browser-desktop.png)과 [측정 결과](./agent-browser-status-desktop.json)를 보존했다.

확인 뒤 `agent-browser close --all`을 실행했다. 최종 session 목록은 빈 배열이고 Agent Browser process는 남지 않았다. Tailnet preview PID `28040`은 계속 실행 중이다. [종료 확인](./browser-close.json).

이 작업은 문서 미리보기 갱신이며 simulator, 앱 동작, pixel parity 또는 GitHub Pages 배포를 검증하지 않는다. C11 체크와 공식 `72/213`은 변경하지 않았다. [PR 변경 실패 경로 검토 30개](./roadmap-pr-diff-review-30.md).
