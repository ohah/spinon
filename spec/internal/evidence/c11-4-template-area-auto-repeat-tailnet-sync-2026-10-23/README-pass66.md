# C11.4 Tailnet roadmap anchor 보정 · pass66

PR #541 병합 SHA `986d596fdea4f9235671acc4f9bdd6c13dc3a06e`를 기준으로 pass65 이후 바뀐 정적 `roadmap.html` 한 파일만 Tailnet preview에 반영했다. pass65의 나머지 9,639개 경로와 문서 site build는 그대로 유지했다.

## 한 파일 갱신과 검증

- main source `spec/public/roadmap.html`은 263,334 bytes, SHA-256 `fb51cbb07f2cb27e36af04a5d5e1700bfb31cb61ba69b4c0f382bc3a2013ba8b`다. 기존 Tailnet 파일은 pass65 manifest와 일치하는 263,320 bytes / SHA-256 `f99f0a0d58c7e9da107dafedb789e3a8d24b3fe67dacaa940b454b4ebb387b0a`였다.
- 기존 파일을 `SpinonPreviewBackups/c11-4-pr540-tailnet-sync-2026-10-23/pass66/roadmap.html`에 same-volume hardlink로 보존한 뒤 main source를 hardlink·atomic rename했다. 삭제는 없었다. [preflight](./preflight-pass66.json) · [copy/backup](./copy-and-backup-pass66.json).
- local target과 HTTPS `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`은 모두 source와 byte/hash가 일치한다. HTTPS는 HTTP 200이다. [원격 검증](./remote-verification-pass66.json).

## Agent Browser와 종료

- 로드맵에서 PR #539의 merge SHA, C11.4 미완료와 공식 완료 수 `77/213`을 확인했다. 이전 fragment는 사라졌고 링크는 `/spinon/STATUS.html#css-구현-체크리스트`다. [roadmap screenshot](./tailnet-roadmap-agent-browser-pass66.png).
- 로드맵 안의 실제 “공식 상태” 링크를 클릭했다. 새 URL의 decoded fragment는 `css-구현-체크리스트`, DOM target은 존재하고 heading은 “CSS 구현 체크리스트”로 표시됐다. [status anchor screenshot](./tailnet-status-anchor-agent-browser-pass66.png) · [DOM 확인](./agent-browser-verification-pass66.json).
- Agent Browser의 `spinon-pass65`와 default 세션을 닫았고 최종 process scan은 Agent Browser와 Chrome child 모두 0개다. Tailnet preview server PID 1191과 tmux는 유지했다. [종료 확인](./browser-close-pass66.json).
- 이 pass는 문서 경로 확인이다. 앱·시뮬레이터·실기기 테스트는 하지 않았다. C11.4와 공식 완료 수 `77/213`은 그대로 미완료다.
