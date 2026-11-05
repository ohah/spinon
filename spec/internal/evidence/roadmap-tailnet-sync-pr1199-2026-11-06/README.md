# PR #1199 병합 후 Tailnet roadmap 동기화 · 2026-11-06

PR [#1199](https://github.com/ohah/spinon/pull/1199)은 2026-11-05T23:18:52Z Rebase and merge로 병합됐고 merge SHA는 `8bf916bc2ff8b922846a38e3c52c0ed298b44616`이다. 새 C11.4 bounded slice를 공식 완료로 승격하지 않은 채 `spec/STATUS.md`와 공개 roadmap source를 동기화해 사용자가 정한 Tailnet 미리보기 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에 제공했다. GitHub Pages를 배포하지 않았다.

## Roadmap 변경

- C11.4 상위 checkbox는 미완료이며 공식 완료 수 `78/214`를 유지한다.
- PR #1199의 범위와 merge 사실, Chrome 156 rows `[10,20,10,34]px` 및 source-order reverse `[10,10,20,34]px`, 10,000/10,001 track 경계, simulator·CI 제한을 기록했다.
- 새 카드의 계획, 내부 계약, precomparison, 실행 evidence, PR diff review 링크는 GitHub의 병합된 `main`을 가리킨다. Android/iOS screenshots는 merge commit SHA의 raw asset을 가리킨다. Tailnet에서 상태 page와 두 image URL은 HTTP 200을 반환했다.

## Tailnet 제공본

- Tailscale backend는 `Running`; 기존 preview server PID `28040`이 `127.0.0.1:8768`에서 요청을 받고 있었다. 서버를 재시작하거나 종료하지 않았다.
- 선택한 파일은 `spec/public/roadmap.html` 하나다. Preflight에서 이전 대상 파일의 열린 handle이 없음을 확인하고 기존 파일을 backup한 후 atomic rename으로 바꿨다. 파일을 삭제하지 않았다.
- source HEAD는 `8bf916bc2ff8b922846a38e3c52c0ed298b44616`, source tree는 `97424556e859fc1cfb338f18ffda15ae7f081ecb`다. Source bytes는 586,947, SHA-256은 `4c56c7e6e0f23c9a2282882a720d67d09d717604f6404ccc640f85089dd97e54`다.
- 기존 대상 584,210 bytes / SHA-256 `42369b6d07a4aad15b523e5e636de470f7126cac788361e1cb690300228c1255`는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1199-2026-11-06/pass01/roadmap.html`에 보존했다.
- Local HTTP 및 Tailnet HTTPS는 모두 HTTP 200, 586,947 bytes이며 source·served hashes가 일치한다. 세부 결과는 [`preflight.json`](./preflight.json), [`copy-and-backup.json`](./copy-and-backup.json), [`served-verification.json`](./served-verification.json)이다.
- Preview hash 검증 후 `spec/STATUS.md`에 이 sync evidence를 가리키는 줄만 추가했다. 제공 파일 `spec/public/roadmap.html`은 수정하지 않아 기록된 source/local/remote hash는 그대로 일치한다.

## Agent Browser 확인

Agent Browser에서 실제 Tailnet URL을 열어 병합 카드와 target links를 확인했다.

- Desktop 1440×1000: document/body width 1440px, PR #1199 카드 x=90px, width=928px, y=120px. [`desktop screenshot`](./tailnet-roadmap-agent-browser-desktop.png) 및 [`desktop DOM/geometry`](./tailnet-agent-browser-desktop.json).
- Mobile 390×844: document/body width 390px, horizontal overflow 없음, 카드 x=14px, width=362px, y=90px. [`mobile screenshot`](./tailnet-roadmap-agent-browser-mobile.png) 및 [`mobile DOM/geometry`](./tailnet-agent-browser-mobile.json).
- 두 화면의 C11.4 카드는 merge SHA와 `78/214`를 표시한다. 이는 상위 C11.4/C11 완료나 counter 상승을 뜻하지 않는다.
- GitHub 계획/evidence 링크, Tailnet `STATUS.html`, Android/iOS raw screenshot assets는 각각 HTTP 200이었다.
- 초기 페이지 위치를 보존한 [`desktop top capture`](./tailnet-roadmap-agent-browser-desktop-top-before-scroll.png)는 target card 검증 캡처가 아니다. 대표 결과는 카드가 보이도록 위치한 desktop/mobile 캡처다.

화면은 Tailnet 제공본의 문구·링크·responsive geometry 확인이다. 전체 문서 build/link validation, WPT, 공식 완료 판정 또는 GitHub Pages 배포를 증명하지 않는다.
