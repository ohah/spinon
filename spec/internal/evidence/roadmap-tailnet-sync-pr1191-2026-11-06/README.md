# PR #1191 병합 후 Tailnet 로드맵 확인 · pass01

## 게시 원본과 파일 검증

PR [#1191](https://github.com/ohah/spinon/pull/1191)은 2026-11-05T19:35:44Z Rebase and merge됐고 merge SHA는 `41a20a698d783944979ee01ea7df7c337ba54c81`이다. Tailscale backend는 `Running`, 기존 preview server PID `28040`은 `127.0.0.1:8768`에서 수신 중이었다. 서버를 재시작하거나 종료하지 않았다.

저장소 원본과 Tailnet의 `roadmap.html`만 atomic rename으로 교체했다. source, local HTTP, remote HTTPS는 모두 581,932 bytes / SHA-256 `be9d936112884d0da18193ecbc9c8cb0c0ba4619f2a1799c4de42baa8bc647a9`로 일치했다. 이전 파일의 SHA-256 `41e2fed488313dea51676247b941ab93eb13244b1a2156468eadaefcafaac24b`는 다음 위치에 백업했다: `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1191-2026-11-06/pass01/roadmap.html`. 삭제는 없었다. [preflight](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/preflight-pass01.json), [atomic copy 및 backup](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/copy-and-backup-pass01.json), [served hash 확인](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/served-verification-pass01.json).

## Agent Browser 확인

Tailnet URL은 [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)이다. #1190 카드의 모든 내부 설명·화면 링크는 GitHub `main` 또는 PR #1190 merge SHA를 가리켰다.

- Desktop 1440×1000: 문서 너비 1440px, 카드 너비 928px. [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-desktop.png), [DOM·링크·geometry](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-agent-browser-desktop.json)
- Mobile 390×844: 문서/body 너비 390px, 카드 x=14px·width=362px·right=376px, 수평 overflow 없음. [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-mobile.png), [DOM·링크·geometry](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-agent-browser-mobile.json)
- 로드맵의 GitHub API-backed counter는 `78/214`였다. `spec/STATUS.md` checkbox를 같은 parser 규칙으로 독립 집계한 결과도 `78/214`다. [counter 화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-status-counter.png), [browser counter](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-agent-browser-status-counter.json), [local parser count](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/status-count-local.json).
- pass01 카드 본문에는 `72/213`이 남아 있어 live counter와 달랐다. 이 불일치는 보존했고 [PR #1192](https://github.com/ohah/spinon/pull/1192)에서 최신 #1187·#1190 카드, 계획, 상태 대장과 내부 계약을 `78/214`로 정정했다. 정정 merge 뒤 pass02를 다시 게시하고 아래에서 확인했다.

화면은 로드맵 문서의 표시와 반응형 layout만 확인한다. 구현 동작이나 전체 C11.4 적합성을 증명하지 않는다.


## 정정 후 재게시 · pass02

PR [#1192](https://github.com/ohah/spinon/pull/1192)은 2026-11-05T19:54:57Z Rebase and merge됐고 merge SHA는 `957f44ef7594098c146dcb814c95b7e91f4f18f6`이다. 수정된 source `spec/public/roadmap.html`은 581,932 bytes, SHA-256 `4cf96a01cf0ecfe0c71a1fd453410541f752abc432135fa66b6d3d10f2fe4f54`다. pass02는 이전 pass01 제공본 (`be9d9361…`)을 백업한 뒤 roadmap 파일 하나만 atomic rename으로 교체했다. 대상 file handle은 열려 있지 않았고 preview server PID `28040`은 계속 실행됐다. 삭제·server restart는 없었다. [pass02 preflight](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/preflight-pass02.json), [copy와 backup](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/copy-and-backup-pass02.json), [로컬·Tailnet 제공 검증](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/served-verification-pass02.json).

- local HTTP와 remote HTTPS는 모두 200, 581,932 bytes이며 source와 SHA-256이 일치했다.
- Desktop 1440×1000: 문서 너비 1440px, #1190 카드 너비 928px, 카드 본문과 live counter 모두 `78/214`, `72/213` 표시 없음. [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-desktop-pass02.png), [DOM·link·geometry](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-agent-browser-desktop-pass02.json).
- Mobile 390×844: document/body width 390px, horizontal overflow 없음, #1190 카드 x=14px·width=362px·right=376px. Card/live counter 모두 `78/214`. [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-mobile-pass02.png), [DOM·link·geometry](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-agent-browser-mobile-pass02.json).
- Counter와 공식 상태 동기화 note는 [pass02 desktop 화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/tailnet-roadmap-agent-browser-status-counter-pass02.png) 및 desktop/mobile JSON에 기록했다.
- 작업 뒤 `agent-browser close`를 실행했고 `pgrep -alf agent-browser`는 빈 결과였다.


문서 변경 PR diff 검토 30개: [검토 기록](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/roadmap-tailnet-sync-pr1191-2026-11-06/postmerge-docs-pr-diff-review-30.md).
