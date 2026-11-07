# PR #1258 이후 Tailnet roadmap 동기화 · 2026-11-07

## 변경 내용

문서 PR [#1257](https://github.com/ohah/spinon/pull/1257)은 C11.4 `align-content:safe|unsafe flex-end` 계획과 진행 카드를 추가했다. [#1258](https://github.com/ohah/spinon/pull/1258)은 긴 작업 이력 아래에 묻혀 처음에는 보이지 않던 칸반을 페이지 제목 아래로 옮기고, 모바일 첫 화면에서 진행 열이 보이도록 수정했다. 병합된 source revision은 `bdcd7b7935091d12b9d526d43927ded424a4c60e`다.

## Tailnet 제공본과 보존

Tailscale backend는 `Running`이었다. 기존 `tailnet-preview-server.mjs` PID `28040`이 `127.0.0.1:8768`에서 요청을 계속 제공했다. 서버를 재시작하거나 종료하지 않았다. 이전 제공본 620,756 bytes·SHA-256 `d83e0a2944d24b54123a8251f48ce3f74cc0cd4dacf181b3ec704e2b24ba9ac1`을 byte-identical backup으로 보존했다: `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1258-c11-roadmap-active-kanban-2026-11-07/pass01/roadmap.html`.

병합된 roadmap source, preview target, localhost HTTP와 Tailnet HTTPS는 모두 621,472 bytes, SHA-256 `7052a3a027e5e2b77e8d482a2594bef0135b19f1ec1d800ec3d33646a8fe03d2`이며 두 URL 응답은 HTTP 200이다. 새 임시 파일에 기록·`fsync` 후 같은 디렉터리에서 atomic rename했다. [preflight](./preflight.json) · [copy/backup](./copy-and-backup.json) · [served verification](./served-verification.json) · [동기화 스크립트](./sync-tailnet-roadmap.mjs).

## Agent Browser 화면 확인

Agent Browser 0.31.1에서 [실제 Tailnet URL](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 desktop 1440×1000 및 mobile 390×844 viewport로 열었다. 두 화면의 첫 scroll에 `flex-end 계획` / `C11.4 · ACTIVE` 카드가 보이며, official status note는 `78/214` 완료·C11.4/C11 미완료를 표시한다. document width는 각각 viewport와 같은 1440px 및 390px이고 page-level 가로 overflow가 없다. 모바일은 내부 칸반을 `scrollLeft=272px`로 두어 진행 열을 먼저 보여준다.

- Desktop active card y=`455.02px`, board bottom y=`773.02px`. [DOM 관측](./tailnet-desktop.json) · [screenshot](./tailnet-roadmap-agent-browser-desktop.png)
- Mobile active card y=`438.53px`, progress column visible. [DOM 관측](./tailnet-mobile.json) · [screenshot](./tailnet-roadmap-agent-browser-mobile.png)
- Agent Browser session을 닫았고 이후 Agent Browser·HeadlessChrome process scan은 비었다. Tailnet preview server PID `28040`은 계속 실행 중이다. [lifecycle](./agent-browser-lifecycle.json)
- PR #1259 화면에서도 두 본문 이미지가 실제 로드됐고 natural size는 desktop 1440×1000, mobile 390×844였다. [PR image load 관측](./pr-images-agent-browser.json)

GitHub Pages/`gh-pages`는 배포하지 않았다. 공식 완료 `78/214`, C11.4/C11 상태는 변경하지 않았다.

[동기화 PR diff 검토 30개](./pr-diff-review-30.md)
